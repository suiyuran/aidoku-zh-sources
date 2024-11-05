use core::str::FromStr;

use aidoku::{
	prelude::*, std::{ArrayRef, ObjectRef, String, Vec}, Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page
};
use alloc::string::ToString;

use crate::helper;
use uuid::Uuid;

pub fn has_more(data: ObjectRef) -> bool {
	let total = data.get("total").as_int().unwrap();
	let limit = data.get("limit").as_int().unwrap();
	let offset = data.get("offset").as_int().unwrap();
	total > limit + offset
}

pub fn parse_manga_list(manga_list: ArrayRef) -> Vec<Manga> {
	manga_list
		.map(|manga| parse_manga(manga.as_object().unwrap()))
		.collect::<Vec<Manga>>()
}

pub fn parse_manga(manga: ObjectRef) -> Manga {
	let manga = match manga.get("comic").as_object() {
		Ok(value) => value,
		Err(_) => manga,
	};
	let id = manga.get("path_word").as_string().unwrap().read();
	let cover = manga.get("cover").as_string().unwrap().read();
	let title = manga.get("name").as_string().unwrap().read();
	let author = manga
		.get("author")
		.as_array()
		.unwrap()
		.map(|author| {
			author
				.as_object()
				.unwrap()
				.get("name")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = manga
		.get("brief")
		.as_string()
		.unwrap_or_default()
		.read()
		.trim()
		.to_string();
	let url = helper::gen_manga_url(id.clone());
	let categories = manga
		.get("theme")
		.as_array()
		.unwrap_or_default()
		.map(|theme| {
			theme
				.as_object()
				.unwrap_or_default()
				.get("name")
				.as_string()
				.unwrap_or_default()
				.read()
		})
		.collect::<Vec<String>>();
	let status = match manga
		.get("status")
		.as_object()
		.unwrap_or_default()
		.get("value")
		.as_int()
		.unwrap_or(-1)
	{
		0 => MangaStatus::Ongoing,
		1 => MangaStatus::Completed,
		2 => MangaStatus::Unknown,
		_ => MangaStatus::Unknown,
	};
	let nsfw = match manga
		.get("restrict")
		.as_object()
		.unwrap_or_default()
		.get("value")
		.as_int()
		.unwrap_or(-1)
	{
		0 => MangaContentRating::Safe,
		1 => MangaContentRating::Suggestive,
		2 => MangaContentRating::Nsfw,
		3 => MangaContentRating::Nsfw,
		4 => MangaContentRating::Nsfw,
		_ => MangaContentRating::Safe,
	};
	let viewer = MangaViewer::Rtl;
	Manga {
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
	}
}

pub fn parse_chapter_group(manga_id: String, group: ObjectRef, name: String, start: usize) -> Vec<Chapter> {
	let list = group.get("chapters").as_array();
	let mut chapters: Vec<Chapter> = Vec::new();

	if list.is_ok() {
		for (index, item) in list.unwrap().enumerate() {
			let chapter = item.as_object().unwrap();
			let id = chapter.get("id").as_string().unwrap().read();
			let title = format!("{} - {}", name, chapter.get("name").as_string().unwrap().read());
			let chapter = (index + start + 1) as f32;
			let (p1, p2) = Uuid::from_str(&id.clone()).unwrap().get_timestamp().unwrap().to_unix();
			let date_updated = (p1 as f64) + (p2 as f64 * 10e-10);
			let url = helper::gen_chapter_url(manga_id.clone(), id.clone());
			chapters.push(Chapter {
				id,
				title,
				chapter,
				date_updated,
				url,
				..Default::default()
			})
		}
	}

	chapters
}

pub fn parse_chapter_list(manga: ObjectRef) -> Vec<Chapter> {
	let build = manga.get("build").as_object().unwrap();
	let manga_id = build.get("path_word").as_string().unwrap().read();
	let groups = manga.get("groups").as_object().unwrap();
	let default_group = groups.get("default").as_object().unwrap_or_default();
	let tankobon_group = groups.get("tankobon").as_object().unwrap_or_default();
	let other_honyakuchimu_group = groups.get("other_honyakuchimu").as_object().unwrap_or_default();
	let karapeji_group = groups.get("karapeji").as_object().unwrap_or_default();
	let default = parse_chapter_group(manga_id.clone(), default_group, String::from("默认"), 0);
	let tankobon = parse_chapter_group(manga_id.clone(), tankobon_group, String::from("单行本"), default.len());
	let other_honyakuchimu = parse_chapter_group(manga_id.clone(), other_honyakuchimu_group, String::from("其它汉化版"), default.len() + tankobon.len());
	let karapeji = parse_chapter_group(manga_id.clone(), karapeji_group, String::from("全彩版"), default.len() + tankobon.len() + other_honyakuchimu.len());
	let mut chapters = [default, tankobon, other_honyakuchimu, karapeji].concat();

	chapters.reverse();
	chapters
}

pub fn parse_page_list(chapter: ObjectRef) -> Vec<Page> {
	let list = chapter.get("contents").as_array().unwrap();
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let page = item.as_object().unwrap();
		let index = index as i32;
		let url = page.get("url").as_string().unwrap().read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	pages
}
