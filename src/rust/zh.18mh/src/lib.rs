#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://18mh.org";

const FILTER_GENRE: [&str; 4] = ["hanman", "zhenrenxiezhen", "riman", "aixiezhen"];
const FILTER_CATEGORY: [&str; 18] = [
	"",
	"hanman",
	"zhenrenxiezhen",
	"riman",
	"aixiezhen",
	"duoren",
	"yuwang",
	"zhengmei",
	"tongju",
	"nxuesheng",
	"juqing",
	"touqing",
	"xiaoyuan",
	"nixi",
	"bangongshi",
	"youhuo",
	"fanzhuan",
	"shun",
];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类型" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	let url = if query.is_empty() {
		let caregory_str = if category.is_empty() {
			String::from("manga")
		} else if FILTER_GENRE.contains(&category.as_str()) {
			format!("manga-genre/{}", category)
		} else {
			format!("manga-tag/{}", category)
		};
		format!("{}/{}/page/{}", WWW_URL, caregory_str, page)
	} else {
		format!("{}/s/{}?page={}", WWW_URL, encode_uri(query), page)
	};

	let html = Request::new(url, HttpMethod::Get).html()?;

	for item in html.select(".pb-2>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.select("div>img").attr("src").read();
		let title = item.select("div>h3").text().read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut list = String::new();

	match listing.name.as_str() {
		"人气推荐" => {
			list.push_str("hots");
		}
		"热门更新" => {
			list.push_str("dayup");
		}
		"最新上架" => {
			list.push_str("newss");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/{}/page/{}", WWW_URL, list, page);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".pb-2>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.select("div>img").attr("src").read();
		let title = item.select("div>h3").text().read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let ids = id.split("/").collect::<Vec<&str>>();
	let url = format!("{}/manga/{}", WWW_URL, ids[0]);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mid = html.select("#mangachapters").attr("data-mid").read();
	let cover = html
		.select("meta[property='og:image']")
		.attr("content")
		.read();
	let title = html.select("title").text().read().replace("-18漫畫", "");
	let author = html
		.select("a[href*=author]>span")
		.array()
		.map(|a| a.as_node().unwrap().text().read().replace(",", ""))
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".text-medium.my-unit-md").text().read();
	let categories = html
		.select(".py-1>a:not([href*=author])>span")
		.array()
		.map(|a| {
			a.as_node()
				.unwrap()
				.text()
				.read()
				.replace(",", "")
				.replace("熱門漫畫", "")
				.replace("#", "")
				.replace("推荐", "")
				.trim()
				.to_string()
		})
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = match html.select("h1.mb-2>span").text().read().trim() {
		"連載中" => MangaStatus::Ongoing,
		"完結" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Nsfw;
	let viewer = MangaViewer::Scroll;

	Ok(Manga {
		id: format!("{}/{}", ids[0], mid),
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories,
		status,
		nsfw,
		viewer,
	})
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let ids = id.split("/").collect::<Vec<&str>>();
	let url = format!("{}/manga/get?mid={}&mode=all", WWW_URL, ids[1]);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select("#allchapterlist>.chapteritem>a").array();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let id = item.attr("data-cs").read();
		let title = item
			.select("div>span:nth-child(1)")
			.text()
			.read()
			.trim()
			.to_string();
		let slug = item.attr("href").read();
		let url = format!("{}/{}", WWW_URL, slug);
		let chapter = (index + 1) as f32;
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		});
	}
	chapters.reverse();

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let ids = manga_id.split("/").collect::<Vec<&str>>();
	let url = format!(
		"{}/chapter/getcontent?m={}&c={}",
		WWW_URL,
		ids[1],
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("Referer", &WWW_URL)
		.html()?;
	let list = html.select("#chapcontent>div>img").array();
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = if item.has_attr("data-src") {
			item.attr("data-src").read()
		} else {
			item.attr("src").read()
		};
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request.header("Referer", &WWW_URL);
}
