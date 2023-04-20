#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::{substring::Substring, uri::encode_uri},
	prelude::*,
	std::{
		json,
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;
use base64::{engine::general_purpose, Engine};

const WWW_URL: &str = "https://www.manhuadb.com";
const STATIC_URL: &str = "https://i2.manhuadb.com/static";

const FILTER_REGION: [&str; 7] = ["", "4", "5", "6", "7", "8", "9"];
const FILTER_AUDIENCE: [&str; 10] = ["", "3", "4", "5", "6", "7", "9", "10", "11", "12"];
const FILTER_STATUS: [&str; 3] = ["", "1", "2"];
const FILTER_CATEGORY: [&str; 66] = [
	"", "26", "66", "12", "64", "39", "41", "20", "40", "33", "48", "13", "46", "44", "71", "52",
	"43", "27", "18", "55", "72", "32", "59", "16", "53", "56", "80", "54", "60", "73", "47", "58",
	"30", "51", "21", "22", "9", "11", "45", "68", "67", "19", "70", "57", "29", "61", "78", "37",
	"76", "17", "23", "65", "28", "10", "49", "69", "62", "50", "42", "34", "77", "74", "63", "81",
	"82", "83",
];

fn handle_cover(mut cover: String) -> String {
	if !cover.starts_with("https") {
		cover = format!("{}{}", WWW_URL, cover)
	}
	cover
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut region = String::new();
	let mut audience = String::new();
	let mut status = String::new();
	let mut category = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"原作地区" => {
						region = FILTER_REGION[index].to_string();
					}
					"面向读者" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"连载状态" => {
						status = FILTER_STATUS[index].to_string();
					}
					"类型" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!(
			"{}/manhua/list-r-{}-a-{}-s-{}-c-{}-page-{}.html",
			WWW_URL, region, audience, status, category, page
		)
	} else {
		format!(
			"{}/search?q={}&p={}",
			WWW_URL,
			encode_uri(query.clone()),
			page
		)
	};
	let class = if query.is_empty() {
		"comic-book"
	} else {
		"comicbook"
	};
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(format!("div[class*='{}']", class)).array() {
		let element = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = element
			.select("a")
			.attr("href")
			.read()
			.split("/")
			.last()
			.unwrap()
			.to_string();
		let cover = handle_cover(element.select("a>img").attr("data-original").read());
		let title = element.select("div>h2>a").text().read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		})
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/manhua/{}", WWW_URL, id);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = handle_cover(html.select(".comic-cover>img").attr("src").read());
	let title = html.select(".comic-title").text().read();
	let author = html
		.select("meta[property='og:novel:author']")
		.attr("content")
		.read()
		.split(" ")
		.map(|a| a.trim().to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".comic_story").text().read();
	let categories = html
		.select("meta[property='og:novel:category']")
		.attr("content")
		.read()
		.split(" ")
		.map(|a| a.to_string())
		.collect::<Vec<String>>();
	let status = match html
		.select("meta[property='og:novel:status']")
		.attr("content")
		.read()
		.as_str()
	{
		"连载中" => MangaStatus::Ongoing,
		"已完结" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = match html.select(".comic_age").text().read().as_str() {
		"青年" => MangaContentRating::Suggestive,
		"女青" => MangaContentRating::Suggestive,
		_ => MangaContentRating::Safe,
	};
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
	let url = format!("{}/manhua/{}", WWW_URL, id);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in html.select(".links-of-books>li>a").array().enumerate() {
		let element = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let chapter_id = element
			.attr("href")
			.read()
			.split("/")
			.last()
			.unwrap()
			.replace(".html", "");
		let title = element.text().read();
		let chapter = (index + 1) as f32;
		let url = format!("{}/manhua/{}/{}.html", WWW_URL, id, chapter_id);
		chapters.push(Chapter {
			id: chapter_id,
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
	let url = format!("{}/manhua/{}/{}.html", WWW_URL, manga_id, chapter_id);
	let string = Request::new(url, HttpMethod::Get).string()?;
	let string = string
		.substring_after("var img_data = ")
		.unwrap()
		.substring_before(";")
		.unwrap()
		.replace("'", "");
	let data = general_purpose::STANDARD.decode(string).unwrap();
	let list = json::parse(data)?.as_array()?;
	let sub_path = chapter_id.replace("_", "/");
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = item.as_object()?;
		let index = index as i32;
		let img = item.get("img").as_string()?.read();
		let url = format!("{}/{}/{}", STATIC_URL, sub_path, img);
		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	Ok(pages)
}
