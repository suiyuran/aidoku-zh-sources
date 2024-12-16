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
	Chapter, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://yandanshe.com";
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

const FILTER_CATEGORY: [&str; 3] = ["", "bl", "bg"];
const FILTER_STATUS: [&str; 3] = ["", "lz", "wj"];
const FILTER_TAG: [&str; 12] = [
	"",
	"韓漫",
	"會員專區",
	"abo",
	"幻想",
	"職場",
	"校園",
	"娛樂圈",
	"異世界",
	"强制",
	"台漫",
	"百合",
];
const FILTER_SORT: [&str; 2] = ["time", "like"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut status = String::new();
	let mut tag = String::new();
	let mut sort = String::from("time");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"分类" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					"状态" => {
						status = FILTER_STATUS[index].to_string();
					}
					"标签" => {
						tag = FILTER_TAG[index].to_string();
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
				sort = FILTER_SORT[index].to_string();
			}
			_ => continue,
		}
	}

	if category.is_empty() || status.is_empty() {
		category = String::new();
		status = String::new();
	}

	let url = if query.is_empty() {
		format!(
			"{}/{}{}/page/{}/?tag={}&sort={}",
			WWW_URL,
			category,
			status,
			page,
			encode_uri(tag),
			sort
		)
	} else {
		format!("{}/page/{}/?s={}", WWW_URL, page, encode_uri(query.clone()))
	};
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select("article").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select("h3>a")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.select(".thumbnail>a>img").attr("src").read();
		let title = item.select("h3>a").text().read();
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
	let url = format!("{}/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let title = html.select("h1.article-title").text().read();
	let author = html
		.select(".article-meta>.item-author")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html
		.select(".article-content>blockquote>p:nth-child(2)")
		.text()
		.read()
		.replace("內容簡介：", "");
	let mut categories = html
		.select(".article-meta>.item-cat>a")
		.text()
		.read()
		.trim()
		.split("·")
		.map(|a| a.to_string())
		.collect::<Vec<String>>();
	let mut tags = html
		.select(".article-tags>.inner>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = match categories.pop().unwrap().to_string().as_str() {
		"連載" => MangaStatus::Ongoing,
		"完結" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Nsfw;
	let viewer = MangaViewer::Rtl;
	categories.append(&mut tags);

	Ok(Manga {
		id,
		cover: String::new(),
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
	let url = format!("{}/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select(".list>*").array();
	let mut chapters: Vec<Chapter> = Vec::new();

	if list.len() == 0 {
		let id = String::from("1");
		let title = format!("第 {} 话", id);
		let chapter = 1 as f32;
		let url = format!("{}/{}/", url, id);
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		});
	} else {
		for (index, item) in list.enumerate() {
			let item = match item.as_node() {
				Ok(item) => item,
				Err(_) => continue,
			};
			let id = item.text().read().trim().to_string();
			let title = format!("第 {} 话", id);
			let chapter = (index + 1) as f32;
			let url = format!("{}/{}/", url, id);
			chapters.push(Chapter {
				id,
				title,
				chapter,
				url,
				..Default::default()
			});
		}
		chapters.reverse();
	}

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/{}/{}/", WWW_URL, manga_id.clone(), chapter_id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html.select(".article-content>p>img").array().enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.attr("data-src").read().trim().to_string();
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
	request.header("Referer", WWW_URL).header("User-Agent", UA);
}
