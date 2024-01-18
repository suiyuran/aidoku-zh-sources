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
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://www.gufengmh.com";
const IMG_URL: &str = "https://res.xiaoqinre.com";

const FILTER_GENRE: [&str; 5] = ["", "shaonian", "shaonv", "qingnian", "zhenrenmanhua"];
const FILTER_REGION: [&str; 6] = [
	"",
	"ribenmanhua",
	"guochanmanhua",
	"gangtaimanhua",
	"oumeimanhua",
	"hanguomanhua",
];
const FILTER_STATUS: [&str; 3] = ["", "wanjie", "lianzai"];
const FILTER_SORT: [&str; 3] = ["post", "update", "click"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut genre = String::new();
	let mut region = String::new();
	let mut status = String::new();
	let mut sort = String::from("click");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类型" => {
						genre = FILTER_GENRE[index].to_string();
					}
					"地区" => {
						region = FILTER_REGION[index].to_string();
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
			"{}/list/{}-{}-{}/{}/{}/",
			WWW_URL, genre, region, status, sort, page
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

	for item in html.select(".book-list>li").array() {
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
		let title = item.select(".ell>a").text().read().trim().to_string();
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
	let mut name = String::new();

	match listing.name.as_str() {
		"总人气榜" => {
			name.push_str("popularity");
		}
		"日人气榜" => {
			name.push_str("popularity-daily");
		}
		"周人气榜" => {
			name.push_str("popularity-weekly");
		}
		"月人气榜" => {
			name.push_str("popularity-monthly");
		}
		"总点击榜" => {
			name.push_str("click");
		}
		"日点击榜" => {
			name.push_str("click-daily");
		}
		"周点击榜" => {
			name.push_str("click-weekly");
		}
		"月点击榜" => {
			name.push_str("click-monthly");
		}
		"总订阅榜" => {
			name.push_str("subscribe");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/rank/{}/", WWW_URL, name);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".rank-list>li").array() {
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
		let title = item.select(".ell>a").text().read().trim().to_string();
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
	let cover = html.select(".book-cover>.cover>img").attr("src").read();
	let title = html.select(".book-title>h1>span").text().read();
	let author = html.select("a[href*='author']").text().read();
	let artist = String::new();
	let description = html
		.select("#intro-cut>p")
		.text()
		.read()
		.replace("漫画简介：", "")
		.replace("介绍:", "")
		.trim()
		.to_string();
	let categories = html
		.select(".detail-list>li:nth-child(2)>span:nth-child(1)>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = match html
		.select(".detail-list>li:nth-child(1)>span:nth-child(1)>a")
		.text()
		.read()
		.as_str()
	{
		"已完结" => MangaStatus::Completed,
		"连载中" => MangaStatus::Ongoing,
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
	let url = format!("{}/manhua/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select("#chapter-list-1>li>a").array();
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
		let title = item.select("span").text().read();
		let chapter = (index + 1) as f32;
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
	chapters.reverse();

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
	let text = html.html().read();
	let list = text
		.substring_after("var chapterImages = ")
		.unwrap()
		.substring_before(";")
		.unwrap();
	let path = text
		.substring_after("var chapterPath = ")
		.unwrap()
		.substring_before(";")
		.unwrap()
		.replace("\"", "");
	let list = json::parse(list).unwrap().as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_string() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = format!("{}/{}{}", IMG_URL, path, item);
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
