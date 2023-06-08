use aidoku::{
	prelude::*,
	std::{ArrayRef, ObjectRef, String, Vec},
	Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page,
};
use alloc::string::ToString;

use crate::helper;

pub fn has_more(data: ObjectRef) -> bool {
	let page = data.get("page").as_int().unwrap();
	let pages = data.get("pages").as_int().unwrap();
	pages > page
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
	let id = manga.get("_id").as_string().unwrap().read();
	let thumb = manga.get("thumb").as_object().unwrap();
	let host = thumb.get("fileServer").as_string().unwrap().read();
	let path = thumb.get("path").as_string().unwrap().read();
	let cover = format!("{}/static/{}", host, path);
	let title = manga.get("title").as_string().unwrap().read();
	let author = manga
		.get("author")
		.as_string()
		.unwrap()
		.read()
		.split("&")
		.map(|a| a.trim().to_string())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = manga
		.get("description")
		.as_string()
		.unwrap_or_default()
		.read();
	let url = helper::gen_manga_url(id.clone());
	let categories = manga
		.get("categories")
		.as_array()
		.unwrap()
		.map(|category| category.as_string().unwrap().read())
		.collect::<Vec<String>>();
	let status = if manga.get("finished").as_bool().unwrap_or_default() {
		MangaStatus::Completed
	} else {
		MangaStatus::Ongoing
	};
	let nsfw = MangaContentRating::Nsfw;
	let viewer = if categories.contains(&String::from("WEBTOON")) {
		MangaViewer::Scroll
	} else {
		MangaViewer::Rtl
	};
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

pub fn parse_chapter_list(manga_id: String, chapter_list: ArrayRef) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for item in chapter_list {
		let item = item.as_object().unwrap();
		let order = item.get("order").as_int().unwrap();
		let id = order.to_string();
		let title = item.get("title").as_string().unwrap().read();
		let chapter = order as f32;
		let url = helper::gen_chapter_url(manga_id.clone(), id.clone());
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		});
	}

	chapters
}

pub fn parse_page_list(page_list: ArrayRef, offset: i32) -> Vec<Page> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in page_list.enumerate() {
		let item = item.as_object().unwrap();
		let index = index as i32 + offset;
		let media = item.get("media").as_object().unwrap();
		let host = media.get("fileServer").as_string().unwrap().read();
		let path = media.get("path").as_string().unwrap().read();
		let url = format!("{}/static/{}", host, path);
		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	pages
}
