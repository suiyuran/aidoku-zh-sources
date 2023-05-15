#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{net::Request, String, Vec},
	Chapter, Filter, FilterType, Manga, MangaPageResult, Page,
};
use alloc::string::ToString;

mod helper;
mod parser;

const FILTER_CATEGORY: [&str; 30] = [
	"0", "31", "26", "1", "3", "27", "5", "2", "6", "8", "9", "25", "10", "11", "12", "17", "33",
	"37", "14", "15", "29", "20", "21", "4", "7", "30", "34", "36", "40", "61",
];
const FILTER_STATUS: [&str; 3] = ["0", "1", "2"];
const FILTER_SORT: [&str; 3] = ["0", "1", "2"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::from("0");
	let mut status = String::from("0");
	let mut sort = String::from("0");

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
					"状态" => {
						status = FILTER_STATUS[index].to_string();
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
		helper::gen_explore_url(category, status, sort, page)
	} else {
		helper::gen_search_url(query.clone(), page)
	};

	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("response").as_object()?;
	let mangas;
	let has_more;

	if query.is_empty() {
		let list = data.get("mangas").as_array()?;
		mangas = parser::parse_manga_list(list);
		has_more = true;
	} else {
		let list = data.get("result").as_array()?;
		let total = data.get("total").as_int()? as i32;
		mangas = parser::parse_manga_list(list);
		has_more = page * 20 < total;
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = helper::gen_manga_details_url(id);
	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("response").as_object()?;

	Ok(parser::parse_manga(data))
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let url = helper::gen_manga_details_url(id);
	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("response").as_object()?;

	Ok(parser::parse_chapter_list(data))
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = helper::gen_page_list_url(manga_id, chapter_id);
	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("response").as_object()?;

	Ok(parser::parse_page_list(data))
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request
		.header("X-Yq-Yqci", r#"{"le": "zh"}"#)
		.header("User-Agent", "okhttp/3.11.0")
		.header("Referer", "http://www.dm5.com/dm5api/")
		.header("ClubReferer", "http://mangaapi.manhuaren.com/");
}
