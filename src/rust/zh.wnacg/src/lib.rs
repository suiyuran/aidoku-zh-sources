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

const WWW_URL: &str = "https://www.wnacg01.cc";
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36";

const FILTER_CATEGORY: [&str; 4] = ["", "5", "6", "7"];
const FILTER_CATEGORY_5: [&str; 4] = ["5", "1", "12", "16"];
const FILTER_CATEGORY_6: [&str; 4] = ["6", "9", "13", "17"];
const FILTER_CATEGORY_7: [&str; 4] = ["7", "10", "14", "18"];

fn gen_request(url: String, method: HttpMethod) -> Request {
	Request::new(url, method).header("User-Agent", UA)
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类别" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					"语言" => match category.as_str() {
						"5" => category = FILTER_CATEGORY_5[index].to_string(),
						"6" => category = FILTER_CATEGORY_6[index].to_string(),
						"7" => category = FILTER_CATEGORY_7[index].to_string(),
						_ => continue,
					},
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!(
			"{}/albums-index-page-{}-cate-{}.html",
			WWW_URL, page, category
		)
	} else {
		format!(
			"{}/search/index.php?q={}&s=create_time_DESC&syn=yes&p={}",
			WWW_URL,
			encode_uri(query),
			page
		)
	};
	let html = gen_request(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".gallary_item").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".pic_box>a")
			.attr("href")
			.read()
			.split("-")
			.map(|a| a.replace(".html", ""))
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = format!("https:{}", item.select(".pic_box>a>img").attr("src").read());
		let title = item
			.select(".info>.title>a")
			.text()
			.read()
			.trim()
			.to_string();
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
	let mut category = String::new();

	match listing.name.as_str() {
		"CG画集" => {
			category.push_str("2");
		}
		"3D漫画" => {
			category.push_str("22");
		}
		"Cosplay" => {
			category.push_str("3");
		}
		"韩漫" => {
			category.push_str("19");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!(
		"{}/albums-index-page-{}-cate-{}.html",
		WWW_URL, page, category
	);
	let html = gen_request(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".gallary_item").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".pic_box>a")
			.attr("href")
			.read()
			.split("-")
			.map(|a| a.replace(".html", ""))
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = format!("https:{}", item.select(".pic_box>a>img").attr("src").read());
		let title = item
			.select(".info>.title>a")
			.text()
			.read()
			.trim()
			.to_string();
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
	let url = format!("{}/photos-index-aid-{}.html", WWW_URL, id.clone());
	let html = gen_request(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select("#bodywrap>div>.uwthumb>img")
		.attr("src")
		.read()
		.replace("//", "");
	let cover = format!("https://{}", cover);
	let title = html.select("#bodywrap>h2").text().read();
	let author = String::new();
	let artist = String::new();
	let description = String::new();
	let categories = html
		.select("#bodywrap>div>.uwconn>label:nth-child(1)")
		.text()
		.read()
		.replace("分類：", "")
		.split("／")
		.map(|a| a.split("&"))
		.flatten()
		.map(|a| a.trim().to_string())
		.collect::<Vec<String>>();
	let tags = html
		.select("#bodywrap>div>.uwconn>.addtags>.tagshow")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.collect::<Vec<String>>();
	let categories = [categories, tags].concat();
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
	let url = format!("{}/photos-index-aid-{}.html", WWW_URL, id.clone());
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
	let url = format!("{}/photos-gallery-aid-{}.html", WWW_URL, manga_id.clone());
	let text = gen_request(url.clone(), HttpMethod::Get).string()?;
	let urls = text
		.split("\\\"")
		.filter(|a| a.starts_with("//"))
		.map(|a| a.to_string());
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in urls.enumerate() {
		let index = index as i32;
		let url = format!("https:{}", item);
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
