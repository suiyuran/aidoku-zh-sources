#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{String, Vec},
	Chapter, Filter, FilterType, Listing, Manga, MangaPageResult, Page,
};
use alloc::string::ToString;

mod helper;
mod parser;

const FILTER_TAG: [&str; 14] = [
	"",
	"蘿莉",
	"全彩",
	"長筒襪",
	"原創",
	"女學生制服",
	"雙馬尾",
	"巨乳",
	"中出",
	"性玩具",
	"姐妹",
	"百合",
	"無修正",
	"自慰",
];
const FILTER_SORT: [&str; 3] = ["bid", "views", "favorites"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut tag = String::new();
	let mut sort = String::from("bid");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"标签" => {
						tag = FILTER_TAG[index].to_string();
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

	let json = if query.is_empty() {
		helper::explore(tag.clone(), sort, page)?
	} else {
		helper::search(query.clone(), page)?
	};

	let data = json.as_object()?;
	let key = if query.is_empty() && tag.is_empty() {
		"info"
	} else {
		"Info"
	};
	let list = data.get(key).as_array()?;
	let total = data.get("len").as_int()? as i32;
	let has_more = page * 20 < total;

	Ok(MangaPageResult {
		manga: parser::parse_manga_list(list),
		has_more,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut name = String::new();
	let mut level = String::new();

	match listing.name.as_str() {
		"日阅读榜" => {
			name.push_str("readLeaderboard");
			level.push_str("day");
		}
		"周阅读榜" => {
			name.push_str("readLeaderboard");
			level.push_str("week");
		}
		"月阅读榜" => {
			name.push_str("readLeaderboard");
			level.push_str("moon");
		}
		"日收藏榜" => {
			name.push_str("favLeaderboard");
			level.push_str("day");
		}
		"周收藏榜" => {
			name.push_str("favLeaderboard");
			level.push_str("week");
		}
		"月收藏榜" => {
			name.push_str("favLeaderboard");
			level.push_str("moon");
		}
		"高质量榜" => {
			name.push_str("proportion");
		}
		_ => return get_manga_list(Vec::new(), page),
	};

	let json = helper::rank(name, level, page)?;
	let data = json.as_object()?;
	let list = data.get("info").as_array()?;
	let total = data.get("len").as_int()? as i32;
	let has_more = page * 20 < total;

	Ok(MangaPageResult {
		manga: parser::parse_manga_list(list),
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let json = helper::details(id)?;
	let data = json.as_object()?;

	Ok(parser::parse_manga(data))
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let url = helper::gen_chapter_url(id.clone());
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
	let json = helper::details(manga_id.clone())?;
	let data = json.as_object()?;
	let len = data.get("Len").as_int()? as i32;
	let mut pages: Vec<Page> = Vec::new();
	let mut index = 0 as i32;

	while index < len {
		let url = helper::gen_page_url(manga_id.clone(), index + 1);
		pages.push(Page {
			index,
			url,
			..Default::default()
		});
		index += 1;
	}

	Ok(pages)
}
