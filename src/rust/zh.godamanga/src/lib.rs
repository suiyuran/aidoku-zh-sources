#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::substring::Substring,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

mod helper;

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
		let url = helper::gen_explore_url(String::from("manga"), category, page);
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
			let cover = helper::gen_cover_url(item.select("div>img").attr("src").read());
			let title = item.select("div>h3").text().read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	} else {
		let json = helper::search(query, page)?;
		let data = json.as_object()?;
		let list = data.get("hits").as_array()?;

		for item in list {
			let item = item.as_object()?;
			let id = item.get("slug").as_string()?.read();
			let cover = item.get("cover").as_string()?.read();
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

	let url = helper::gen_explore_url(list, String::new(), page);
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
		let cover = helper::gen_cover_url(item.select("div>img").attr("src").read());
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
	let url = helper::gen_manga_url(id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select("meta[property='og:image']")
		.attr("content")
		.read();
	let title = html
		.select("meta[property='og:title']")
		.attr("content")
		.read()
		.replace("-G站漫畫", "");
	let author = html
		.select("a[href*=author]>span")
		.array()
		.map(|a| a.as_node().unwrap().text().read().replace(",", ""))
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".my-unit-md").text().read();
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
		})
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = MangaStatus::Ongoing;
	let nsfw = MangaContentRating::Nsfw;
	let viewer = MangaViewer::Rtl;

	Ok(Manga {
		id,
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
	let url = helper::gen_chapter_list_url(id);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let html = html.select(".grid>.rounded-lg>a");
	let len = html.array().len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in html.array().enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let url = item.attr("href").read();
		let id = url
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let title = item.select("div>span:first-child").text().read();
		let chapter = (len - index) as f32;
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		});
	}

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = helper::gen_page_list_url(manga_id.clone(), chapter_id.clone());
	let text = Request::new(url.clone(), HttpMethod::Get).string()?;
	let id = text
		.substring_after("\\\",{\\\"id\\\":")
		.unwrap()
		.substring_before(",\\\"isAd\\\"")
		.unwrap()
		.to_string();
	let page_url = helper::gen_page_url(id);
	let json = Request::new(page_url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("attributes").as_object()?;
	let list = data.get("chapter_img").as_array()?;

	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let page = item.as_object()?;
		let index = index as i32;
		let url = page.get("url").as_string()?.read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
