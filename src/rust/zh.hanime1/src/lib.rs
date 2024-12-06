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

const WWW_URL: &str = "https://hanime1.me";

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

	let url = format!(
		"{}/comics/search?query={}&page={}",
		WWW_URL,
		encode_uri(query),
		page
	);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".comic-rows-videos-div>a").array() {
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
		let cover = item.select("img").attr("data-srcset").read();
		let title = item.select("div>.comic-rows-videos-title").text().read();
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
	let mut sort = String::new();

	match listing.name.as_str() {
		"日榜" => {
			sort.push_str("popular-today");
		}
		"周榜" => {
			sort.push_str("popular-week");
		}
		"总榜" => {
			sort.push_str("popular");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!(
		"{}/comics/search?sort={}&query=&page={}",
		WWW_URL, sort, page
	);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".comic-rows-videos-div>a").array() {
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
		let cover = item.select("img").attr("data-srcset").read();
		let title = item.select("div>.comic-rows-videos-title").text().read();
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
	let url = format!("{}/comic/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select("meta[property='og:image']")
		.attr("content")
		.read();
	let title = html
		.select("h3[class^=title]>span")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.collect::<Vec<String>>()
		.join(" ");
	let author = html.select("a[href*=artists]>div[style]").text().read();
	let artist = String::new();
	let description = html
		.select("meta[property='og:description']")
		.attr("content")
		.read()
		.trim()
		.to_string();
	let categories = html
		.select("a[href*=tags]>div[style]")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.collect::<Vec<String>>();
	let status = MangaStatus::Unknown;
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
	let url = format!("{}/comic/{}/1", WWW_URL, id.clone());
	let mut chapters: Vec<Chapter> = Vec::new();
	let title = String::from("第 1 话");
	let chapter = 1 as f32;
	chapters.push(Chapter {
		id,
		title,
		chapter,
		url,
		..Default::default()
	});

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, _: String) -> Result<Vec<Page>> {
	let url = format!("{}/comic/{}", WWW_URL, manga_id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html
		.select(".comics-panel-margin>a>img")
		.array()
		.enumerate()
	{
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item
			.attr("data-srcset")
			.read()
			.replace("t.nhentai.net", "i.nhentai.net")
			.replace("t.", ".");
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
