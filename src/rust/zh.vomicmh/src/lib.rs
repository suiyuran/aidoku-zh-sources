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

const WWW_URL: &str = "http://www.vomicmh.com";
const API_URL: &str = "http://api.vomicmh.com";

fn encode_img_url(url: String) -> String {
	if !url.contains("%") {
		encode_uri(url)
	} else {
		url
	}
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!("{}/api/v1/rank/rank-data?rank_id=1&page={}", API_URL, page)
	} else {
		format!(
			"{}/api/v1/search/search-comic-data?title={}&page={}",
			API_URL,
			encode_uri(query),
			page
		)
	};
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("result").as_array()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = match item.as_object() {
			Ok(object) => object,
			Err(_) => continue,
		};
		let id = item.get("mid").as_string()?.read();
		let cover = encode_img_url(item.get("cover_img_url").as_string()?.read());
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
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!(
		"{}/api/v1/detail/get-comic-detail-data?mid={}",
		API_URL,
		id.clone()
	);
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let cover = encode_img_url(data.get("cover_img_url").as_string()?.read());
	let title = data.get("title").as_string()?.read();
	let author = data
		.get("authors_name")
		.as_array()?
		.map(|a| a.as_string().unwrap().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = data.get("description").as_string()?.read();
	let url = format!("{}/#/detail?id={}", WWW_URL, id.clone());
	let categories = data
		.get("categories")
		.as_array()?
		.map(|a| a.as_string().unwrap().read())
		.collect::<Vec<String>>();
	let status = match data.get("status").as_string()?.read().as_str() {
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
	let url = format!(
		"{}/api/v1/detail/get-comic-detail-chapter-data?mid={}",
		API_URL,
		id.clone()
	);
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let list = data.get("data").as_array()?;
	let len = list.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item.get("cid").as_string()?.read();
		let title = item.get("title").as_string()?.read();
		let chapter = (len - index) as f32;
		let url = format!("{}/#/page/{}/{}", WWW_URL, id.clone(), chapter_id.clone(),);
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
		"{}/m_{}/chapterimage.ashx?mid={}",
		WWW_URL,
		chapter_id.clone(),
		manga_id.clone()
	);
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let list = data.get("img_url_list").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_string() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = encode_img_url(item.read());
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
