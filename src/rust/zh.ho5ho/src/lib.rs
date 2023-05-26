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

const WWW_URL: &str = "https://www.ho5ho.com";
const MANGA_URL: &str = "https://www.ho5ho.com/%E4%B8%AD%E5%AD%97h%E6%BC%AB";

const FILTER_CATEGORY: [&str; 31] = [
	"",
	"m女",
	"m男",
	"ntr",
	"亂倫",
	"催眠",
	"全彩",
	"可愛",
	"同人",
	"多p",
	"女同",
	"女性向",
	"姐姐",
	"學生",
	"巨乳",
	"幻想",
	"強姦",
	"懷孕",
	"接吻",
	"正太",
	"母子",
	"母狗",
	"深喉",
	"熟女",
	"痴女",
	"聖水",
	"肉感",
	"肛交",
	"變態",
	"貧乳",
	"韓漫",
];
const FILTER_SORT: [&str; 3] = ["latest", "rating", "views"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut sort = String::from("latest");

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

	let url = if query.is_empty() {
		if category.is_empty() {
			format!("{}/page/{}/?m_orderby={}", WWW_URL, page, sort)
		} else {
			format!(
				"{}/manga-genre/{}/page/{}/?m_orderby={}",
				WWW_URL,
				encode_uri(category),
				page,
				sort
			)
		}
	} else {
		format!(
			"{}/page/{}/?s={}&post_type=wp-manga",
			WWW_URL,
			page,
			encode_uri(query)
		)
	};

	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select("div[class*='c-image-hover']>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let list = item
			.attr("href")
			.read()
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>();
		let id = (&list[3]).to_string();
		let cover = item.select("img").attr("data-src").read();
		let title = item.attr("title").read();
		mangas.push(Manga {
			id,
			cover: encode_uri(cover),
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
	let url = format!("{}/{}/", MANGA_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select(".summary_image>a>img")
		.attr("data-src")
		.read()
		.replace("193x278", "175x238");
	let title = html.select(".post-title>h1").text().read();
	let author = html
		.select(".author-content>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html
		.select(".description-summary>div>p")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join("\n");
	let categories = html
		.select(".genres-content>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = match html
		.select(".post-status>div:nth-child(2)>.summary-content")
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
	let viewer = MangaViewer::Rtl;

	Ok(Manga {
		id,
		cover: encode_uri(cover),
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
	let url = format!("{}/{}/", MANGA_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let html = html.select(".wp-manga-chapter>a");
	let len = html
		.html()
		.read()
		.match_indices("Server")
		.collect::<Vec<_>>()
		.len();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in html.array().enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let url = item.attr("href").read();
		let list = url
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>();
		let id = format!("{}/{}/{}", list[4], list[5], list[6]);
		let title = item
			.text()
			.read()
			.split("-")
			.last()
			.unwrap()
			.trim()
			.to_string();
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
	let url = format!("{}/{}/{}/", MANGA_URL, manga_id, chapter_id);
	let text = Request::new(url.clone(), HttpMethod::Get).string()?;
	let list = text
		.substring_after("var chapter_preloaded_images = ")
		.unwrap()
		.substring_before(", chapter_images_per_page =")
		.unwrap();
	let list = json::parse(list).unwrap().as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let index = index as i32;
		let url = item.as_string()?.read();
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
