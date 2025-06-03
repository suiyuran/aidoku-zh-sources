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
use alloc::string::ToString;

const WWW_URL: &str = "https://www.baozimh.com";
const IMG_URL: &str = "https://static-tw.baozimh.com";

const FILTER_CATEGORY: [&str; 26] = [
	"all",
	"lianai",
	"chunai",
	"gufeng",
	"yineng",
	"xuanyi",
	"juqing",
	"kehuan",
	"qihuan",
	"xuanhuan",
	"chuanyue",
	"maoxian",
	"tuili",
	"wuxia",
	"gedou",
	"zhanzheng",
	"rexie",
	"gaoxiao",
	"danuzhu",
	"dushi",
	"zongcai",
	"hougong",
	"richang",
	"hanman",
	"shaonian",
	"qita",
];
const FILTER_REGION: [&str; 5] = ["all", "cn", "jp", "kr", "en"];
const FILTER_STATUS: [&str; 3] = ["all", "serial", "pub"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut region = String::new();
	let mut status = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类型" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					"地区" => {
						region = FILTER_REGION[index].to_string();
					}
					"状态" => {
						status = FILTER_STATUS[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let has_more = query.is_empty();
	let mut mangas: Vec<Manga> = Vec::new();

	if query.is_empty() {
		let url = format!(
			"{}/api/bzmhq/amp_comic_list?type={}&region={}&state={}&page={}&language=tw",
			WWW_URL, category, region, status, page
		);
		let json = Request::new(url, HttpMethod::Get).json()?;
		let data = json.as_object()?;
		let list = data.get("items").as_array()?;

		for item in list {
			let item = match item.as_object() {
				Ok(item) => item,
				Err(_) => continue,
			};
			let id = item.get("comic_id").as_string()?.read();
			let cover = format!(
				"{}/cover/{}?w=285&h=375&q=100",
				IMG_URL,
				item.get("topic_img").as_string()?.read()
			);
			let title = item.get("name").as_string()?.read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	} else {
		let url = format!("{}/search/?q={}", WWW_URL, encode_uri(query.clone()));
		let html = Request::new(url, HttpMethod::Get).html()?;
		let list = html.select(".pure-g>.comics-card").array();

		for item in list {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let id = item
				.select(".comics-card__info")
				.attr("href")
				.read()
				.split("/")
				.filter(|a| !a.is_empty())
				.map(|a| a.to_string())
				.collect::<Vec<String>>()
				.pop()
				.unwrap();
			let cover = item
				.select(".comics-card__poster>amp-img")
				.attr("src")
				.read();
			let title = item
				.select(".comics-card__title")
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
	let id = html
		.select("meta[name='og:novel:read_url']")
		.attr("content")
		.read()
		.split("/")
		.filter(|a| !a.is_empty())
		.map(|a| a.to_string())
		.collect::<Vec<String>>()
		.pop()
		.unwrap();
	let cover = html.select("meta[name='og:image'").attr("content").read();
	let title = html
		.select("meta[name='og:novel:book_name']")
		.attr("content")
		.read();
	let author = html
		.select("meta[name='og:novel:author']")
		.attr("content")
		.read();
	let artist = String::new();
	let description = html
		.select("meta[name='og:description']")
		.attr("content")
		.read()
		.split(",")
		.skip(2)
		.map(|a| a.to_string())
		.collect::<Vec<String>>()
		.join(", ");
	let categories = html
		.select("meta[name='og:novel:category']")
		.attr("content")
		.read()
		.split(",")
		.map(|a| a.to_string())
		.filter(|a| !a.starts_with("types"))
		.collect::<Vec<String>>();
	let status = match html
		.select("meta[name='og:novel:status']")
		.attr("content")
		.read()
		.as_str()
	{
		"連載中" => MangaStatus::Ongoing,
		"已完結" => MangaStatus::Completed,
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
	let url = format!("{}/comic/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let list = html.select("div[id^='chapter']>div>a").array();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item
			.attr("href")
			.read()
			.split("&")
			.skip(1)
			.map(|a| a.split("=").nth(1).unwrap().to_string())
			.collect::<Vec<String>>()
			.join("_");
		let title = item.select("div>span").text().read();
		let chapter = (index + 1) as f32;
		let url = format!(
			"{}/comic/chapter/{}/{}.html",
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
	let mut pages: Vec<Page> = Vec::new();
	let mut page = 1;
	let mut current_chapter_id = chapter_id.clone();

	loop {
		let url = format!(
			"{}/comic/chapter/{}/{}.html",
			WWW_URL,
			manga_id.clone(),
			current_chapter_id.clone()
		);
		let html = Request::new(url.clone(), HttpMethod::Get).html()?;
		let list = html.select("amp-img[id^='chapter-img']").array();
		let next_chapter_id = html
			.select("#next-chapter")
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");

		for item in list {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let index = page as i32;
			let url = item.attr("src").read().replace("fcomic", "scomic");
			pages.push(Page {
				index,
				url,
				..Default::default()
			});
			page += 1;
		}

		if !next_chapter_id
			.starts_with(&current_chapter_id.split('_').collect::<Vec<&str>>()[..2].join("_"))
		{
			break;
		}

		current_chapter_id = next_chapter_id;
	}

	Ok(pages)
}
