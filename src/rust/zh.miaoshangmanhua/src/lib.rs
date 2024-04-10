#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::{substring::Substring, uri::encode_uri},
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://www.miaoshangmanhua.com";

const FILTER_TAG: [&str; 80] = [
	"", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18",
	"19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34",
	"35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50",
	"51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "69", "70",
	"71", "73", "75", "76", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "95",
];
const FILTER_ORDER: [&str; 2] = ["hits", "addtime"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut tag = String::new();
	let mut order = String::from("hits");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
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
				order = FILTER_ORDER[index].to_string();
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		if tag.is_empty() {
			format!("{}/category/list/1/order/{}/page/{}", WWW_URL, order, page)
		} else {
			format!(
				"{}/category/list/1/order/{}/tags/{}/page/{}",
				WWW_URL, order, tag, page
			)
		}
	} else {
		format!("{}/search?key={}", WWW_URL, encode_uri(query))
	};

	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".cy_list_mh>ul").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select("li:nth-child(1)>a")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = item.select("li:nth-child(1)>a>img").attr("src").read();
		let title = item.select(".title>a").text().read();
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
		"人气总榜" => {
			list.push_str("hot");
		}
		"人气月榜" => {
			list.push_str("month");
		}
		"人气周榜" => {
			list.push_str("week");
		}
		"人气日榜" => {
			list.push_str("day");
		}
		"收藏总榜" => {
			list.push_str("fav");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/custom/{}", WWW_URL, list);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".cy_ph_list_mh>ul").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".pic>a")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = item.select(".pic>a>img").attr("src").read();
		let title = item.select(".title>a").text().read();
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
	let url = format!("{}/comic/{}.html", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html.select(".cy_info_cover>a>img").attr("src").read();
	let title = html.select(".cy_title>h1").text().read();
	let author = html
		.select(".cy_intro_l>.cy_xinxi:nth-child(4)>span:nth-child(1)>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select("#comic-description").text().read();
	let categories = html
		.select(".cy_intro_l>.cy_xinxi:nth-child(5)>span:nth-child(1)>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = match html
		.select(".cy_intro_l>.cy_xinxi:nth-child(4)>span:nth-child(2)>font")
		.text()
		.read()
		.trim()
		.to_string()
		.as_str()
	{
		"连载中" => MangaStatus::Ongoing,
		"已完结" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Scroll;

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
	let url = format!("{}/comic/{}.html", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select("#mh-chapter-list-ol-0>li>a").array();
	let len = list.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
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
		let title = item.select("p").text().read().trim().to_string();
		let chapter = (len - index) as f32;
		let url = format!(
			"{}/comic/{}/{}.html",
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
		"{}/comic/{}/{}.html",
		WWW_URL,
		manga_id.clone(),
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let text = html
		.select(".rd-article-wr")
		.html()
		.read()
		.as_str()
		.substring_before("<script>")
		.unwrap()
		.to_string();
	let list = text
		.split("\"")
		.filter(|a| a.starts_with("http") && !a.ends_with("/add.png"));
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let index = index as i32;
		let url = item.to_string();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
