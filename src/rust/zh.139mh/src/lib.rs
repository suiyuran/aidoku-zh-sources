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

const WWW_URL: &str = "https://139mh.com";
const API_URL: &str = "https://api.139mh.com";
const IMG_URL: &str = "https://i.139img.com";

const FILTER_SUBJECT: [&str; 46] = [
	"all",
	"aiqing",
	"youmo",
	"maoxian",
	"xiaoyuan",
	"shaonyu",
	"shenghuo",
	"rexue",
	"kehuan",
	"jingji",
	"gedou",
	"kongbu",
	"shengcun",
	"xuanyi",
	"zhentan",
	"lishi",
	"zhanzheng",
	"lizhi",
	"zhichang",
	"meishi",
	"jizhan",
	"mohuan",
	"mofa",
	"qihuan",
	"shengui",
	"wuxia",
	"xianxia",
	"zhiyu",
	"mengxi",
	"zhaixi",
	"qingnian",
	"shaonian",
	"hougong",
	"baihe",
	"weiniang",
	"danmei",
	"tonghua",
	"dongfang",
	"sige",
	"huiben",
	"yishu",
	"zazhi",
	"lianhuanhua",
	"jiakong",
	"chuanyue",
	"tongren",
];
const FILTER_AREA: [&str; 7] = ["all", "china", "japan", "korea", "ea", "hktw", "other"];
const FILTER_PROGRESS: [&str; 4] = ["all", "updating", "complate", "stop"];
const FILTER_ORDER: [&str; 2] = ["hot", "last"];

fn handle_cover_url(url: String) -> String {
	format!("{}/cover/{}", IMG_URL, url)
}

fn handle_page_url(url: String) -> String {
	format!("{}/img/{}", IMG_URL, url)
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut subject = String::from("all");
	let mut area = String::from("all");
	let mut progress = String::from("all");
	let mut order = String::from("hot");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"分类" => {
						subject = FILTER_SUBJECT[index].to_string();
					}
					"地区" => {
						area = FILTER_AREA[index].to_string();
					}
					"状态" => {
						progress = FILTER_PROGRESS[index].to_string();
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
				order = FILTER_ORDER[index].to_string();
			}
			_ => continue,
		}
	}

	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	let url = if query.is_empty() {
		format!(
			"{}/api/comic/index?page={}&area={}&subject={}&progress={}&order={}",
			API_URL, page, area, subject, progress, order
		)
	} else {
		format!(
			"{}/api/comic/search?page={}&q={}",
			API_URL,
			page,
			encode_uri(query.clone())
		)
	};

	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;

	let list = if query.is_empty() {
		data.get("result").as_array()?
	} else {
		let data = data.get("result").as_object()?;
		data.get("list").as_array()?
	};

	for item in list {
		let item = match item.as_object() {
			Ok(object) => object,
			Err(_) => continue,
		};
		let id = item.get("id").as_int()?.to_string();
		let cover = handle_cover_url(item.get("cover").as_string()?.read());
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
	let url = format!("{}/api/comic/detail?id={}", API_URL, id.clone());
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("result").as_object()?;
	let info = data.get("info").as_object()?;
	let cover = handle_cover_url(info.get("cover").as_string()?.read());
	let title = info.get("title").as_string()?.read();
	let author = info
		.get("auth")
		.as_array()?
		.map(|a| {
			a.as_object()
				.unwrap()
				.get("auth_name")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = info.get("summary").as_string()?.read();
	let url = format!("{}/comic-{}", WWW_URL, id.clone());
	let categories = info
		.get("cata_list")
		.as_array()?
		.map(|a| {
			a.as_object()
				.unwrap()
				.get("title")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>();
	let status = match info.get("progress").as_int()?.to_string().as_str() {
		"1" => MangaStatus::Completed,
		"2" => MangaStatus::Hiatus,
		"3" => MangaStatus::Ongoing,
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
	let url = format!("{}/api/comic/detail?id={}", API_URL, id.clone());
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("result").as_object()?;
	let list = data.get("vol_list").as_array()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item.get("id").as_int()?.to_string();
		let title = item.get("title").as_string()?.read();
		let chapter = (index + 1) as f32;
		let url = format!("{}/comic/vol-{}", WWW_URL, chapter_id.clone());
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
fn get_page_list(_: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/api/vol/detail?id={}", API_URL, chapter_id.clone());
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("result").as_object()?;
	let list = data.get("img_list").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_string() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = handle_page_url(item.read());
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}
