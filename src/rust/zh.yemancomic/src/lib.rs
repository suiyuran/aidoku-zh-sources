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

const WWW_URL: &str = "https://yemancomic.com";

const FILTER_CATEGORY: [&str; 40] = [
	"全部",
	"长条",
	"大女主",
	"百合",
	"耽美",
	"纯爱",
	"後宫",
	"韩漫",
	"奇幻",
	"轻小说",
	"生活",
	"悬疑",
	"格斗",
	"搞笑",
	"伪娘",
	"竞技",
	"职场",
	"萌系",
	"冒险",
	"治愈",
	"都市",
	"霸总",
	"神鬼",
	"侦探",
	"爱情",
	"古风",
	"欢乐向",
	"科幻",
	"穿越",
	"性转换",
	"校园",
	"美食",
	"剧情",
	"热血",
	"节操",
	"励志",
	"异世界",
	"历史",
	"战争",
	"恐怖",
];
const FILTER_REGION: [&str; 7] = ["9", "1", "2", "3", "4", "5", "6"];
const FILTER_STATUS: [&str; 3] = ["3", "4", "1"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::from("全部");
	let mut region = String::from("9");
	let mut status = String::from("3");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"题材" => {
						category = FILTER_CATEGORY[index].to_string();
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
			_ => continue,
		}
	}

	let mut has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	if query.is_empty() {
		let url = format!(
			"{}/comiclists/{}/{}/{}/{}.html",
			WWW_URL,
			region,
			encode_uri(category),
			status,
			page
		);
		let html = Request::new(url, HttpMethod::Get).html()?;
		has_more = true;

		for item in html.select(".acgn-item").array() {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let id = item
				.select(".acgn-thumbnail")
				.attr("href")
				.read()
				.split("/")
				.filter(|a| !a.is_empty())
				.map(|a| a.to_string())
				.collect::<Vec<String>>()
				.pop()
				.unwrap();
			let cover = item.select(".acgn-thumbnail>img").attr("src").read();
			let title = item
				.select(".acgn-info>.acgn-title>a")
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
	} else {
		let url = format!("{}/api/front/index/search", WWW_URL);
		let body = format!("key={}", query);
		let json = Request::new(url, HttpMethod::Post)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(body.as_bytes())
			.json()?;
		let data = json.as_object()?;
		let list = data.get("data").as_array()?;

		for item in list {
			let item = match item.as_object() {
				Ok(object) => object,
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
		"总点击" => {
			name.push_str("alldj");
		}
		"月点击" => {
			name.push_str("ydj");
		}
		"周点击" => {
			name.push_str("zdj");
		}
		"日点击" => {
			name.push_str("rdj");
		}
		"总收藏" => {
			name.push_str("allfav");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/top/{}.html", WWW_URL, name);
	let html = Request::new(url, HttpMethod::Get).html()?;
	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".acgn-item").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".acgn-thumbnail")
			.attr("href")
			.read()
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item.select(".acgn-thumbnail>img").attr("src").read();
		let title = item
			.select(".acgn-info>.acgn-title>a")
			.attr("title")
			.read()
			.split(",")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
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
	let url = format!("{}/book/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select("meta[property='og:image']")
		.attr("content")
		.read();
	let title = html
		.select("meta[property='og:title']")
		.attr("content")
		.read();
	let author = html
		.select("meta[property='og:novel:author']")
		.attr("content")
		.read()
		.split("|")
		.map(|a| a.to_string())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html
		.select("meta[property='og:description']")
		.attr("content")
		.read();
	let categories = html
		.select("meta[property='og:novel:category']")
		.attr("content")
		.read()
		.split(",")
		.map(|a| a.to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = match html
		.select("meta[property='og:novel:status']")
		.attr("content")
		.read()
		.as_str()
	{
		"连载" => MangaStatus::Ongoing,
		"完结" => MangaStatus::Completed,
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
	let url = format!("{}/book/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select(".chapter-list>li").array();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item.attr("data-chapter").read();
		let title = item.select("a").attr("title").read();
		let chapter = (index + 1) as f32;
		let url = format!(
			"{}/chapter/{}/{}.html",
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
		"{}/chapter/{}/{}.html",
		WWW_URL,
		manga_id.clone(),
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html
		.select(".acgn-reader-chapter__item>div>img")
		.array()
		.enumerate()
	{
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = encode_uri(item.attr("src").read());
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
