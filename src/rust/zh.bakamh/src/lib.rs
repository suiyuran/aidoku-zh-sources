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

const WWW_URL: &str = "https://bakamh.com";

const FILTER_FINISH: [&str; 3] = ["", "on-going", "end"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut finish = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"进度" => {
						finish = FILTER_FINISH[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!("{}/{}/page/{}/", WWW_URL, finish, page)
	} else {
		format!(
			"{}/page/{}/?s={}&post_type=wp-manga",
			WWW_URL,
			page,
			encode_uri(query.clone())
		)
	};
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	if query.is_empty() {
		for item in html.select(".page-item-detail").array() {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let id = item
				.select(".item-thumb>a")
				.attr("href")
				.read()
				.split("/")
				.map(|a| a.to_string())
				.filter(|a| !a.is_empty())
				.collect::<Vec<String>>()
				.pop()
				.unwrap();
			let cover = item
				.select(".item-thumb>a>img")
				.attr("data-src")
				.read()
				.replace("-175x238", "");
			let title = item.select(".item-summary>.post-title>h3>a").text().read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	} else {
		for item in html.select(".c-tabs-item__content").array() {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let id = item
				.select(".col-4>.tab-thumb>a")
				.attr("href")
				.read()
				.split("/")
				.map(|a| a.to_string())
				.filter(|a| !a.is_empty())
				.collect::<Vec<String>>()
				.pop()
				.unwrap();
			let cover = item
				.select(".col-4>.tab-thumb>a>img")
				.attr("data-src")
				.read()
				.replace("-193x278", "");
			let title = item
				.select(".col-8>.tab-summary>.post-title>h3>a")
				.text()
				.read();
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
	let mut name = String::new();

	match listing.name.as_str() {
		"新作" => {
			name.push_str("newmanga");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/{}/page/{}/", WWW_URL, name, page);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".page-item-detail").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".item-thumb>a")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item
			.select(".item-thumb>a>img")
			.attr("data-src")
			.read()
			.replace("-175x238", "");
		let title = item.select(".item-summary>.post-title>h3>a").text().read();
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
	let url = format!("{}/manga/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select("meta[property='og:image']")
		.attr("content")
		.read();
	let title = html.select("meta[property='og:title']").text().read();
	let author = html
		.select(".author-content>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let len = html.select(".post-content>div").array().len();
	let description = html
		.select(format!(".post-content>div:nth-child({})>div>p", len))
		.text()
		.read();
	let categories = html
		.select(".tags-content>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = match html
		.select(format!(
			".post-content>div:nth-child({})>.summary-content",
			len - 2
		))
		.text()
		.read()
		.trim()
		.to_string()
		.as_str()
	{
		"OnGoing" => MangaStatus::Ongoing,
		"Completed" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Nsfw;
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
	let url = format!("{}/manga/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select(".wp-manga-chapter>a").array();
	let len = list.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
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
		let title = item.text().read().trim().to_string();
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
	let url = format!(
		"{}/manga/{}/{}",
		WWW_URL,
		manga_id.clone(),
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html.select("img[id]").array().enumerate() {
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
	request.header("Referer", WWW_URL);
}
