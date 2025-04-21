#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		defaults::defaults_get,
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36";

const FILTER_TAG: [&str; 23] = [
	"全部", "青春", "性感", "长腿", "多人", "御姐", "巨乳", "新婚", "媳妇", "暧昧", "清纯", "调教",
	"少妇", "风骚", "同居", "淫乱", "好友", "女神", "诱惑", "偷情", "出轨", "正妹", "家教",
];
const FILTER_AREA: [&str; 4] = ["-1", "1", "2", "3"];
const FILTER_END: [&str; 3] = ["-1", "0", "1"];

fn get_url() -> String {
	defaults_get("url").unwrap().as_string().unwrap().read()
}

fn gen_request(url: String, method: HttpMethod) -> Request {
	Request::new(url, method).header("User-Agent", UA)
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut tag = String::new();
	let mut area = String::new();
	let mut end = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"题材" => {
						tag = FILTER_TAG[index].to_string();
					}
					"地区" => {
						area = FILTER_AREA[index].to_string();
					}
					"进度" => {
						end = FILTER_END[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!(
			"{}/booklist?tag={}&area={}&end={}&page={}",
			get_url(),
			encode_uri(tag),
			area,
			end,
			page
		)
	} else {
		format!("{}/search?keyword={}", get_url(), encode_uri(query.clone()))
	};
	let html = gen_request(url, HttpMethod::Get).html()?;
	let has_more = query.is_empty();
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".mh-item").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select("a")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item
			.select("a>p")
			.attr("style")
			.read()
			.replace("background-image: url(", "")
			.replace(")", "");
		let title = item
			.select(".mh-item-detali>h2>a")
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
	let url = format!("{}/book/{}", get_url(), id.clone());
	let html = gen_request(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select(".banner_detail_form>.cover>img")
		.attr("src")
		.read();
	let title = html
		.select(".banner_detail_form>.info>h1")
		.text()
		.read()
		.trim()
		.to_string();
	let author = html
		.select(".banner_detail_form>.info>p:nth-child(3)")
		.text()
		.read()
		.trim()
		.replace("作者：", "")
		.split("&")
		.map(|a| a.to_string())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html
		.select(".banner_detail_form>.info>.content")
		.text()
		.read()
		.trim()
		.to_string();
	let categories = html
		.select(".banner_detail_form>.info>p:nth-child(5)>span>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read().trim().to_string())
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = match html
		.select(".banner_detail_form>.info>p:nth-child(4)>span:nth-child(1)>span")
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
	let url = format!("{}/book/{}", get_url(), id.clone());
	let html = gen_request(url.clone(), HttpMethod::Get).html()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in html.select("#detail-list-select>li>a").array().enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
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
		let title = item.text().read().trim().to_string();
		let chapter = (index + 1) as f32;
		let url = format!("{}/chapter/{}", get_url(), id.clone());
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
	let url = format!("{}/chapter/{}", get_url(), chapter_id.clone());
	let html = gen_request(url.clone(), HttpMethod::Get).html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html
		.select(".comicpage>div>img,#cp_img>img")
		.array()
		.enumerate()
	{
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.attr("data-original").read().trim().to_string();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
