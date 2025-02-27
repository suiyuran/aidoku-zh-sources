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
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://www.mkzhan.com";
const API_URL: &str = "https://comic.mkzcdn.com";

const FILTER_THEME: [&str; 24] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "12", "13", "14", "15", "16", "17",
	"18", "19", "20", "21", "23", "24", "26",
];
const FILTER_FINISH: [&str; 3] = ["", "1", "2"];
const FILTER_AUDIENCE: [&str; 5] = ["", "1", "2", "3", "4"];
const FILTER_COPYRIGHT: [&str; 3] = ["", "1", "2"];
const FILTER_FREE: [&str; 4] = ["", "is_free=1", "is_fee=1", "is_vip=1"];
const FILTER_ORDER: [&str; 3] = ["3", "1", "2"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut theme_id = String::from("0");
	let mut finish = String::new();
	let mut audience = String::new();
	let mut copyright = String::new();
	let mut free = String::new();
	let mut order = String::from("3");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"题材" => {
						theme_id = FILTER_THEME[index].to_string();
					}
					"进度" => {
						finish = FILTER_FINISH[index].to_string();
					}
					"受众" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"版权" => {
						copyright = FILTER_COPYRIGHT[index].to_string();
					}
					"资费" => {
						free = FILTER_FREE[index].to_string();
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

	let url = if query.is_empty() {
		let mut base = format!(
			"{}/search/filter/?theme_id={}&order={}&page_num={}&page_size=15",
			API_URL, theme_id, order, page
		);
		if !finish.is_empty() {
			base.push_str(format!("&finish={}", finish).as_str());
		}
		if !audience.is_empty() {
			base.push_str(format!("&audience={}", audience).as_str());
		}
		if !copyright.is_empty() {
			base.push_str(format!("&copyright={}", copyright).as_str());
		}
		if !free.is_empty() {
			base.push_str(format!("&{}", free).as_str());
		}
		base
	} else {
		format!(
			"{}/search/keyword/?keyword={}&page_num={}&page_size=20",
			API_URL,
			encode_uri(query),
			page
		)
	};

	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("list").as_array()?;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = item.as_object()?;
		let id = item.get("comic_id").as_string()?.read();
		let cover = item.get("cover").as_string()?.read();
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
		has_more: true,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut name = String::new();

	match listing.name.as_str() {
		"人气榜" => {
			name.push_str("popular");
		}
		"收藏榜" => {
			name.push_str("collection");
		}
		"新作榜" => {
			name.push_str("latest");
		}
		"上升榜" => {
			name.push_str("ascension");
		}
		"月票榜" => {
			name.push_str("ticket");
		}
		"打赏榜" => {
			name.push_str("gratuity");
		}
		"评分榜" => {
			name.push_str("score");
		}
		"付费榜" => {
			name.push_str("popular/pay");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!(
		"{}/top/{}/?type=1&page_num={}&page_size=10",
		API_URL, name, page
	);
	let json = Request::new(url, HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("list").as_array()?;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = item.as_object()?;
		let id = item.get("comic_id").as_string()?.read();
		let cover = item.get("cover").as_string()?.read();
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
		has_more: true,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/{}/", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get).html()?;
	let cover = html
		.select(".de-info__cover>img")
		.attr("data-src")
		.read()
		.replace("!cover-400", "");
	let title = html.select(".j-comic-title").text().read();
	let author = html
		.select(".comic-author>.name>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".intro-total").text().read();
	let categories = html
		.select(".comic-status>span:nth-child(1)>b")
		.text()
		.read()
		.split(" ")
		.map(|a| a.to_string())
		.collect::<Vec<String>>();
	let status = match html
		.select(".de-chapter__title>span:nth-child(1)")
		.text()
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
	let url = format!("{}/chapter/v1/?comic_id={}", API_URL, id.clone());
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let list = data.get("data").as_array()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = item.as_object()?;
		let chapter_id = item.get("chapter_id").as_string()?.read();
		let title = item.get("title").as_string()?.read();
		let chapter = (index + 1) as f32;
		let url = format!("{}/{}/{}.html", WWW_URL, id.clone(), chapter_id.clone());

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
	let uid = defaults_get("uid")?.as_string()?.read();
	let sign = defaults_get("sign")?.as_string()?.read();
	let url = format!(
		"{}/chapter/content/v1/?comic_id={}&chapter_id={}&format=1&quality=1&type=1&uid={}&sign={}",
		API_URL,
		manga_id.clone(),
		chapter_id.clone(),
		uid,
		sign
	);
	let json = Request::new(url.clone(), HttpMethod::Get).json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("page").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = item.as_object()?;
		let index = index as i32;
		let url = item.get("image").as_string()?.read();

		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	Ok(pages)
}
