#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri_component,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Manga, MangaPageResult, Page,
};
use alloc::string::ToString;

mod parser;

const FILTER_CATEGORY: [&str; 45] = [
	"",
	"649c1841ab4619df894c5009",
	"649c1868c63d0ab0014c4ff2",
	"649c1874b907de3ef64c507f",
	"649c18861e8697616f4c4ffe",
	"649c1891ab9629e1214c4fda",
	"649c1897b907de3ef64c5080",
	"649c189e3ce2cd72e04c5012",
	"649c18a5e68837ef064c4fda",
	"649c18afc63d0ab0014c4ff3",
	"649c18c650c42baf004c5035",
	"649c18cdd3f6ea51324c5099",
	"649c18dd50c42baf004c5036",
	"649c18ef50c42baf004c5037",
	"649c18f750c42baf004c5038",
	"649c19009ef7fb37cb4c4fed",
	"649c1911b907de3ef64c5081",
	"649c191aab9629e1214c4fdb",
	"649c19249ef7fb37cb4c4fee",
	"649c192cc63d0ab0014c4ff4",
	"649c19339a673d04fb4c4ff3",
	"649c1942c63d0ab0014c4ff5",
	"649c1948e68837ef064c4fdb",
	"649c1952d3f6ea51324c509a",
	"649c1958b907de3ef64c5082",
	"649c1962d3f6ea51324c509b",
	"649c1968b907de3ef64c5083",
	"649c197157ecb260924c4fdc",
	"649c19793ce2cd72e04c5013",
	"649c1989c63d0ab0014c4ff6",
	"649c198fe68837ef064c4fdc",
	"649c1996ef3a5edbc44c5042",
	"649c19a2ef3a5edbc44c5043",
	"649c19acef3a5edbc44c5046",
	"649c19bab907de3ef64c5086",
	"649c19c23ce2cd72e04c5014",
	"649c19ca3ce2cd72e04c5015",
	"649c19d4ef3a5edbc44c5049",
	"649c19dfef3a5edbc44c504d",
	"649c19e8ef3a5edbc44c504f",
	"649c19f250c42baf004c503a",
	"649c19f9ef3a5edbc44c5052",
	"649c1a00d3f6ea51324c509d",
	"649c1a07d3f6ea51324c509f",
	"649c1a0e57ecb260924c4fde",
];
const FILTER_REGION: [&str; 6] = [
	"",
	"649c176950c42baf004c5031",
	"649c177050c42baf004c5032",
	"649c177750c42baf004c5033",
	"649c178050c42baf004c5034",
	"649c178a9a673d04fb4c4ff2",
];
const FILTER_AUDIENCE: [&str; 5] = [
	"",
	"649c16ffab4619df894c5008",
	"649c17068636d653444c4fe4",
	"649c170f9a673d04fb4c4ff0",
	"649c17319b4bb93a3f4c504b",
];
const FILTER_STATUS: [&str; 3] = ["", "649c16c1ab4619df894c5005", "649c16d0ab4619df894c5006"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();
	let mut region = String::new();
	let mut audience = String::new();
	let mut status = String::new();

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
					"受众" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"连载" => {
						status = FILTER_STATUS[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let url = format!("{}/api/search/", parser::WWW_URL);
	let query = if query.is_empty() {
		format!("board:漫画 {} {} {} {}", category, region, audience, status)
	} else {
		format!("board:漫画 {}", query)
	};
	let body = format!("query={}&page={}", encode_uri_component(query), page);
	let json = Request::new(url, HttpMethod::Post)
		.body(body)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.json()?;
	let data = json.as_object()?;
	let list = data.get("data").as_array()?;
	let page_data = data.get("page").as_object()?;
	let total_page = page_data.get("total_page").as_int()? as i32;
	let mangas = parser::parse_manga_list(list);

	Ok(MangaPageResult {
		manga: mangas,
		has_more: page < total_page,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/api/get/info/", parser::WWW_URL);
	let body = format!("category=comic&id={}", id);
	let json = Request::new(url, HttpMethod::Post)
		.body(body)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;

	Ok(parser::parse_manga(data))
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}/api/get/episode/page/", parser::WWW_URL);
	let mut page = 1;
	let mut chapters: Vec<Chapter> = Vec::new();

	loop {
		let body = format!("category=comic&id={}&page={}&limit=20", id.clone(), page);
		let json = Request::new(url.clone(), HttpMethod::Post)
			.body(body)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.json()?;
		let data = json.as_object()?;
		let list = data.get("data").as_array()?;
		let page_data = data.get("page").as_object()?;
		let total_page = page_data.get("total_page").as_int()? as i32;
		let chapter_list = parser::parse_chapter_list(list);
		chapters = [chapters, chapter_list].concat();

		if page == total_page {
			break;
		}

		page += 1;
	}
	chapters.reverse();

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/api/get/episode/", parser::WWW_URL);
	let body = format!("category=comic&post_id={}&id={}", manga_id, chapter_id);
	let json = Request::new(url, HttpMethod::Post)
		.body(body)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let data = data.get("param").as_object()?;
	let list = data.get("img").as_array()?;

	Ok(parser::parse_page_list(list))
}
