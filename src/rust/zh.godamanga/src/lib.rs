#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://godamh.com";
const NEWS_URL: &str = "https://news.cocolamanhua.com";
const API_URL: &str = "https://api-get.mgsearcher.com";

const FILTER_CATEGORY: [&str; 35] = [
	"",
	"cn",
	"kr",
	"jp",
	"fuchou",
	"gufeng",
	"qihuan",
	"nixi",
	"lianai",
	"yineng",
	"zhaixiang",
	"chuanyue",
	"rexue",
	"chunai",
	"xitong",
	"chongsheng",
	"maoxian",
	"lingyi",
	"danvzhu",
	"juqing",
	"lianai",
	"xuanhuan",
	"nvshen",
	"kehuan",
	"mohuan",
	"tuili",
	"lieqi",
	"zhiyu",
	"dushi",
	"yixing",
	"qingchun",
	"mori",
	"xuanyi",
	"xiuxian",
	"zhandou",
];

fn handle_cover_url(url: String) -> String {
	if url.contains("url=") {
		url.split("url=")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace("%3A", ":")
			.replace("%2F", "/")
			.replace("&w=250&q=60", "")
	} else {
		url
	}
}

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

	if query.is_empty() {
		let caregory_str = if category.is_empty() {
			String::from("manga")
		} else if category.len() <= 2 {
			format!("manga-genre/{}", category)
		} else {
			format!("manga-tag/{}", category)
		};
		let url = format!("{}/{}/page/{}", WWW_URL, caregory_str, page);
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
			let cover = handle_cover_url(item.select("div>img").attr("src").read());
			let title = item.select("div>h3").text().read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	} else {
		let url = String::from("https://go.mgsearcher.com/indexes/mangaStrapiPro/search");
		let body = format!(
			r#"{{
			"hitsPerPage": 30,
			"page": {},
			"q": "{}"
		}}"#,
			page, query
		);
		let json = Request::new(url, HttpMethod::Post)
			.body(body.as_bytes())
			.header("Content-Type", "application/json")
			.header(
				"Authorization",
				"Bearer 9bdaaa44f0dd520da24298a02818944327b8280a79feb480302feda7c009264a",
			)
			.json()?;
		let data = json.as_object()?;
		let list = data.get("hits").as_array()?;

		for item in list {
			let item = item.as_object()?;
			let id = item.get("slug").as_string()?.read();
			let cover = handle_cover_url(item.get("cover").as_string()?.read());
			let title = item.get("title").as_string()?.read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
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
		let cover = handle_cover_url(item.select("div>img").attr("src").read());
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
	let url = format!("{}/manga/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mid = html.select("#mangachapters").attr("data-mid").read();
	let cover = handle_cover_url(
		html.select("meta[property='og:image']")
			.attr("content")
			.read(),
	);
	let title = html.select("title").text().read().replace("-G站漫畫", "");
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
				.replace("热门漫画", "")
				.replace("#", "")
				.replace("热门推荐", "")
				.trim()
				.to_string()
		})
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = MangaStatus::Ongoing;
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Scroll;

	Ok(Manga {
		id: format!("{}/{}", id, mid),
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
	let url = format!("{}/api/manga/get?mid={}&mode=all", API_URL, ids[1]);
	let json = Request::new(url.clone(), HttpMethod::Get)
		.header("Origin", &WWW_URL)
		.header("Referer", &WWW_URL)
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("chapters").as_array()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let attributes = item.get("attributes").as_object()?;
		let id = item.get("id").as_int()?.to_string();
		let title = attributes.get("title").as_string()?.read();
		let slug = attributes.get("slug").as_string()?.read();
		let url = format!("{}/manga/{}/{}", WWW_URL, ids[0], slug);
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
		"{}/chapter/getinfo?m={}&c={}",
		API_URL,
		ids[1],
		chapter_id.clone()
	);
	let json = Request::new(url.clone(), HttpMethod::Get)
		.header("Origin", &WWW_URL)
		.header("Referer", &WWW_URL)
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let info = data.get("info").as_object()?;
	let list = info.get("images").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.get("url").as_string()?.read();
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
	request.header("Referer", &NEWS_URL);
}
