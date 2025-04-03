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

const WWW_URL: &str = "https://m.zaimanhua.com";
const API_URL: &str = "https://manhua.zaimanhua.com/api/v1";
const APP_URL: &str = "https://manhua.zaimanhua.com/app/v1";
const V4_APP_URL: &str = "https://v4api.zaimanhua.com/app/v1";

const FILTER_STATUS: [&str; 3] = ["0", "1", "2"];
const FILTER_AUDIENCE: [&str; 4] = ["0", "3262", "3263", "3264"];
const FILTER_THEME: [&str; 23] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "11", "13", "14", "15", "16", "17", "18",
	"19", "20", "21", "22", "23", "24",
];
const FILTER_CATE: [&str; 3] = ["0", "1", "2"];
const FILTER_FIRST_LETTER: [&str; 28] = [
	"", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
	"s", "t", "u", "v", "w", "x", "y", "z", "9",
];
const FILTER_SORT_TYPE: [&str; 1] = ["0"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut status = String::from("0");
	let mut audience = String::from("0");
	let mut theme = String::from("0");
	let mut cate = String::from("0");
	let mut first_letter = String::from("");
	let mut sort_type = String::from("0");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"状态" => {
						status = FILTER_STATUS[index].to_string();
					}
					"受众" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"题材" => {
						theme = FILTER_THEME[index].to_string();
					}
					"类别" => {
						cate = FILTER_CATE[index].to_string();
					}
					"字母" => {
						first_letter = FILTER_FIRST_LETTER[index].to_string();
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
				sort_type = FILTER_SORT_TYPE[index].to_string();
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!("{}/comic1/filter?sortType={}&page={}&size=18&status={}&audience={}&theme={}&cate={}&firstLetter={}",
			API_URL, sort_type, page, status, audience, theme, cate, first_letter)
	} else {
		format!(
			"{}/search/index?keyword={}&source=0&page={}&size=20",
			APP_URL,
			encode_uri(query.clone()),
			page
		)
	};

	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let mut mangas: Vec<Manga> = Vec::new();

	if query.is_empty() {
		let list = data.get("comicList").as_array()?;

		for item in list {
			let item = match item.as_object() {
				Ok(item) => item,
				Err(_) => continue,
			};
			let id = item.get("id").as_int()?.to_string();
			let cover = item.get("cover").as_string()?.read();
			let title = item.get("name").as_string()?.read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	} else {
		let list = data.get("list").as_array()?;

		for item in list {
			let item = match item.as_object() {
				Ok(item) => item,
				Err(_) => continue,
			};
			let id = item.get("id").as_int()?.to_string();
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
		has_more: true,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut cate = String::new();
	let mut duration = String::new();

	match listing.name.as_str() {
		"周人气排行" => {
			cate.push_str("1");
			duration.push_str("1");
		}
		"月人气排行" => {
			cate.push_str("1");
			duration.push_str("2");
		}
		"总人气排行" => {
			cate.push_str("1");
			duration.push_str("3");
		}
		"周点击排行" => {
			cate.push_str("2");
			duration.push_str("1");
		}
		"月点击排行" => {
			cate.push_str("2");
			duration.push_str("2");
		}
		"总点击排行" => {
			cate.push_str("2");
			duration.push_str("3");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/comic1/rank_list?channel=pc&app_name=zmh&version=1.0.0&page={}&size=10&duration={}&cate={}&tag=0&theme=0",
		API_URL, page, duration, cate);
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("list").as_array()?;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let id = item.get("comic_id").as_int()?.to_string();
		let cover = item.get("cover").as_string()?.read();
		let title = item.get("title").as_string()?.read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: true,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/comic/detail/{}", V4_APP_URL, id.clone());
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("data").as_object()?;
	let cover = data.get("cover").as_string()?.read();
	let title = data.get("title").as_string()?.read();
	let authors = data.get("authors").as_array()?;
	let author = authors
		.map(|a| {
			a.as_object()
				.unwrap()
				.get("tag_name")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = data.get("description").as_string()?.read();
	let url = format!("{}/pages/comic/detail?id={}", WWW_URL, id);
	let categories = data
		.get("types")
		.as_array()?
		.map(|a| {
			a.as_object()
				.unwrap()
				.get("tag_name")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>();
	let status = match data
		.get("status")
		.as_array()?
		.get(0)
		.as_object()?
		.get("tag_name")
		.as_string()?
		.read()
		.as_str()
	{
		"连载中" => MangaStatus::Ongoing,
		"已完结" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
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
	let url = format!("{}/comic/detail/{}", V4_APP_URL, id.clone());
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("data").as_object()?;
	let chapter_list = data.get("chapters").as_array()?;
	let chapter = chapter_list.get(0).as_object()?;
	let list = chapter.get("data").as_array()?;
	let len = list.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item.get("chapter_id").as_int()?.to_string();
		let title = item.get("chapter_title").as_string()?.read();
		let chapter = (len - index) as f32;
		let url = format!(
			"{}/pages/comic/page?comic_id={}&chapter_id={}",
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
	let url =
		format!(
		"{}/comic1/chapter/detail?channel=pc&app_name=zmh&version=1.0.0&comic_id={}&chapter_id={}",
		API_URL, manga_id.clone(), chapter_id.clone()
	);
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("chapterInfo").as_object()?;
	let list = data.get("page_url").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_string() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
