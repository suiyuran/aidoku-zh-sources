#![no_std]
extern crate alloc;

use encoding_rs::BIG5;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		html::Node,
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://www.cartoonmad.com";

fn handle_img_url(url: String) -> String {
	if url.starts_with("http") {
		return url;
	}
	format!("https:{}", url)
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
		format!("{}/m/?page={}", WWW_URL, page)
	} else {
		format!("{}/m/?keyword={}", WWW_URL, encode_uri(query.clone()))
	};
	let has_more = query.is_empty();
	let mut mangas: Vec<Manga> = Vec::new();

	let html = Request::new(url, HttpMethod::Get).html()?;
	let list = html.select(".comic_prev").array();

	for item in list {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".a1")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = format!("{}{}", WWW_URL, item.select("img").attr("src").read());
		let title = item.select(".covertxt+a").attr("title").read();
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
	let mut act = String::new();

	match listing.name.as_str() {
		"最新上架" => {
			act.push_str("1");
		}
		"热门连载" => {
			act.push_str("2");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/m/?act={}&page={}", WWW_URL, act, page);
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	let html = Request::new(url, HttpMethod::Get).html()?;
	let list = html.select(".comic_prev").array();

	for item in list {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".a1")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = format!("{}{}", WWW_URL, item.select("img").attr("src").read());
		let title = item.select(".covertxt+a").attr("title").read();
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
	let url = format!("{}/m/comic/{}.html", WWW_URL, id.clone());
	let data = Request::new(url.clone(), HttpMethod::Get).data();
	let html = Node::new(BIG5.decode(&data).0.as_bytes())?;
	let cover = format!(
		"{}{}",
		WWW_URL,
		html.select("link[rel='image_src']").attr("href").read()
	);
	let title = html
		.select("meta[name='keywords']")
		.attr("content")
		.read()
		.split(",")
		.map(|a| a.trim().to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>()
		.first()
		.unwrap()
		.to_string();
	let author = html
		.select("td[height='24'")
		.array()
		.get(1)
		.as_node()
		.unwrap()
		.text()
		.read()
		.trim()
		.replace("作者：", "");
	let artist = String::new();
	let description = html
		.select("td[style='font-size:11pt;']")
		.array()
		.get(2)
		.as_node()
		.unwrap()
		.text()
		.read();
	let categories = html
		.select("a[href*='tkey']")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = MangaStatus::Unknown;
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
	let url = format!("{}/m/comic/{}.html", WWW_URL, id.clone());
	let data = Request::new(url.clone(), HttpMethod::Get).data();
	let html = Node::new(BIG5.decode(&data).0.as_bytes())?;
	let list = html
		.select("td[style='font-size:11pt;']")
		.array()
		.get(3)
		.as_node()
		.unwrap()
		.select("td a")
		.array();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let title = item.text().read().trim().to_string();
		let chapter = (index + 1) as f32;
		let url = format!("{}/m/comic/{}.html", WWW_URL, id.clone());
		chapters.push(Chapter {
			id,
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
fn get_page_list(_: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/m/comic/{}.html", WWW_URL, chapter_id.clone());
	let data = Request::new(url.clone(), HttpMethod::Get).data();
	let html = Node::new(BIG5.decode(&data).0.as_bytes())?;
	let img_url = handle_img_url(html.select("img[onload]").attr("src").read());
	let length = html
		.select(".pages:not(:has(img))")
		.last()
		.text()
		.read()
		.parse::<i32>()
		.unwrap_or_default();

	let mut pages: Vec<Page> = Vec::new();

	for index in 0..length {
		let index = index as i32;
		let url = img_url.replace("001.", &format!("{:03}.", index + 1));
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
