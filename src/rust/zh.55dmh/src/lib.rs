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

const WWW_URL: &str = "https://www.55dmh.com";

const FILTER_CATEGORY: [&str; 11] = [
	"",
	"rexue",
	"wuxia",
	"kehuan",
	"tuili",
	"danmei",
	"kongbu",
	"shaonv",
	"lianai",
	"shenghuo",
	"zhanzheng",
];
const FILTER_AUDIENCE: [&str; 5] = ["", "ertong", "shaonian", "shaonv", "qingnian"];
const FILTER_STATUS: [&str; 3] = ["", "wanjie", "lianzai"];
const FILTER_SORT: [&str; 3] = ["click", "update", "post"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut audience = String::new();
	let mut status = String::new();
	let mut sort = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"题材" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					"读者" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"进度" => {
						status = FILTER_STATUS[index].to_string();
					}
					_ => continue,
				}
			}
			FilterType::Sort => {
				let value = match filter.value.as_object() {
					Ok(value) => value,
					Err(_) => continue,
				};
				let index = value.get("index").as_int()? as usize;
				let ascending = value.get("ascending").as_bool().unwrap_or(false);
				sort = FILTER_SORT[index].to_string();
				if ascending {
					sort = format!("-{}", sort)
				}
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!(
			"{}/list/{}-{}-{}/{}/?page={}",
			WWW_URL, category, audience, status, sort, page
		)
	} else {
		format!(
			"{}/search/?keywords={}&page={}",
			WWW_URL,
			encode_uri(query.clone()),
			page
		)
	};
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select("#dmList>ul>li").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".cover")
			.attr("href")
			.read()
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.select(".cover>img").attr("src").read();
		let title = item.select("dl>dt>a").text().read().trim().to_string();
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
	let mut key = String::new();

	match listing.name.as_str() {
		"最近更新" => {
			key.push_str("recent");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/main/{}/", WWW_URL, key);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".updateList>ul>li>a[i]").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.attr("i").read();
		let title = item.text().read().trim().to_string();
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
	let url = format!("{}/manhua/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html.select(".cover>img").attr("src").read();
	let title = html
		.select("h3[class]")
		.text()
		.read()
		.trim()
		.replace("简介：", "");
	let author = html
		.select(".info>p:nth-child(2)>a")
		.text()
		.read()
		.trim()
		.split(",")
		.map(|a| a.to_string())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html
		.select(".introduction>p:nth-child(1)")
		.text()
		.read()
		.replace("漫画简介：", "")
		.replace("介绍:", "")
		.trim()
		.to_string();
	let categories = html
		.select(".info>p:nth-child(5)>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = MangaStatus::Ongoing;
	let nsfw = MangaContentRating::Safe;
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
	let url = format!("{}/manhua/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select("#chapter-list-1>li>a").array();
	let len = list.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let title = item.text().read().trim().to_string();
		let chapter = (len - index) as f32;
		let url = format!(
			"{}/manhua/{}/{}.html",
			WWW_URL,
			id.clone(),
			chapter_id.clone()
		);
		chapters.push(Chapter {
			id: chapter_id,
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
	let url = format!(
		"{}/manhua/{}/{}.html",
		WWW_URL,
		manga_id.clone(),
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html.select("#imagesOld>img").array().enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.attr("data-original").read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
