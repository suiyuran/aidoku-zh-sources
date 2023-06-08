#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{String, Vec},
	Chapter, Filter, FilterType, Listing, Manga, MangaPageResult, Page,
};
use alloc::string::ToString;

mod crypto;
mod helper;
mod parser;

const FILTER_CATEGORY: [&str; 37] = [
	"",
	"嗶咔漢化",
	"全彩",
	"長篇",
	"同人",
	"短篇",
	"圓神領域",
	"碧藍幻想",
	"CG雜圖",
	"英語 ENG",
	"生肉",
	"純愛",
	"百合花園",
	"耽美花園",
	"偽娘哲學",
	"後宮閃光",
	"扶他樂園",
	"單行本",
	"姐姐系",
	"妹妹系",
	"SM",
	"性轉換",
	"足の恋",
	"人妻",
	"NTR",
	"強暴",
	"非人類",
	"艦隊收藏",
	"Love Live",
	"SAO 刀劍神域",
	"Fate",
	"東方",
	"WEBTOON",
	"禁書目錄",
	"歐美",
	"Cosplay",
	"重口地帶",
];
const FILTER_SORT: [&str; 4] = ["dd", "da", "ld", "vd"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut sort = String::from("dd");

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

	let json = if query.is_empty() {
		helper::get_json(helper::gen_explore_url(category, sort, page))?
	} else {
		helper::search(query, page)?
	};

	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("comics").as_object()?;
	let list = data.get("docs").as_array()?;

	Ok(MangaPageResult {
		manga: parser::parse_manga_list(list),
		has_more: parser::has_more(data),
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut rank_time = String::new();
	let mut is_random = false;
	let mut category = String::new();
	let sort = String::from("dd");

	match listing.name.as_str() {
		"日榜" => rank_time = String::from("H24"),
		"周榜" => rank_time = String::from("D7"),
		"月榜" => rank_time = String::from("D30"),
		"随机本子" => is_random = true,
		"大湿推荐" => category = String::from("大濕推薦"),
		"那年今天" => category = String::from("那年今天"),
		"大家都在看" => category = String::from("大家都在看"),
		"官方都在看" => category = String::from("官方都在看"),
		_ => return get_manga_list(Vec::new(), page),
	};

	let url = if !rank_time.is_empty() {
		helper::gen_rank_url(rank_time.clone())
	} else if is_random {
		helper::gen_random_url()
	} else if !category.is_empty() {
		helper::gen_explore_url(category, sort, page)
	} else {
		String::new()
	};

	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;

	let mangas;
	let has_more;

	if !rank_time.is_empty() || is_random {
		let list = data.get("comics").as_array()?;
		mangas = parser::parse_manga_list(list);
		has_more = false;
	} else {
		let data = data.get("comics").as_object()?;
		let list = data.get("docs").as_array()?;
		mangas = parser::parse_manga_list(list);
		has_more = parser::has_more(data);
	};

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
	let data = data.get("data").as_object()?;

	Ok(parser::parse_manga(data))
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let mut page = 1;
	let url = helper::gen_chapter_list_url(id.clone(), page);
	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("eps").as_object()?;
	let list = data.get("docs").as_array()?;
	let pages = data.get("pages").as_int()? as i32;
	let mut chapters = parser::parse_chapter_list(id.clone(), list);

	while page < pages {
		page += 1;
		let next_page_chapters = get_chapter_list_by_page(id.clone(), page);
		chapters = [chapters, next_page_chapters].concat();
	}

	Ok(chapters)
}

fn get_chapter_list_by_page(id: String, page: i32) -> Vec<Chapter> {
	let url = helper::gen_chapter_list_url(id.clone(), page);
	let json = helper::get_json(url).unwrap();
	let data = json.as_object().unwrap();
	let data = data.get("data").as_object().unwrap();
	let data = data.get("eps").as_object().unwrap();
	let list = data.get("docs").as_array().unwrap();

	parser::parse_chapter_list(id, list)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let mut page = 1;
	let url = helper::gen_page_list_url(manga_id.clone(), chapter_id.clone(), page);
	let json = helper::get_json(url)?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("pages").as_object()?;
	let list = data.get("docs").as_array()?;
	let pages = data.get("pages").as_int()? as i32;
	let mut page_list = parser::parse_page_list(list, 0);

	while page < pages {
		page += 1;
		let next_page_page_list = get_page_list_by_page(manga_id.clone(), chapter_id.clone(), page);
		page_list = [page_list, next_page_page_list].concat();
	}

	Ok(page_list)
}

fn get_page_list_by_page(manga_id: String, chapter_id: String, page: i32) -> Vec<Page> {
	let url = helper::gen_page_list_url(manga_id.clone(), chapter_id.clone(), page);
	let json = helper::get_json(url).unwrap();
	let data = json.as_object().unwrap();
	let data = data.get("data").as_object().unwrap();
	let data = data.get("pages").as_object().unwrap();
	let list = data.get("docs").as_array().unwrap();
	let limit = data.get("limit").as_int().unwrap() as i32;

	parser::parse_page_list(list, (page - 1) * limit)
}
