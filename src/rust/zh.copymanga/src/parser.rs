use aidoku::{
	std::{ArrayRef, ObjectRef, String, Vec},
	Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page,
};
use alloc::string::ToString;

use crate::helper;

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

pub fn parse_chapter_list(manga: ObjectRef) -> Vec<Chapter> {
	let build = manga.get("build").as_object().unwrap();
	let manga_id = build.get("path_word").as_string().unwrap().read();
	let groups = manga.get("groups").as_object().unwrap();
	let group = groups.get("default").as_object().unwrap();
	let list = group.get("chapters").as_array().unwrap();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let chapter = item.as_object().unwrap();
		let id = chapter.get("id").as_string().unwrap().read();
		let title = chapter.get("name").as_string().unwrap().read();
		let chapter = (index + 1) as f32;
		let url = helper::gen_chapter_url(manga_id.clone(), id.clone());
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		})
	}

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
