use aidoku::{
	prelude::*,
	std::{ArrayRef, ObjectRef, String, Vec},
	Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page,
};
use alloc::string::ToString;

use crate::helper;

pub fn parse_manga_list(manga_list: ArrayRef) -> Vec<Manga> {
	manga_list
		.map(|manga| parse_manga(manga.as_object().unwrap()))
		.collect::<Vec<Manga>>()
}

pub fn parse_manga(manga: ObjectRef) -> Manga {
	let id = manga.get("mangaId").as_int().unwrap().to_string();
	let cover = manga.get("mangaCoverimageUrl").as_string().unwrap();
	let cover = manga
		.get("mangaPicimageUrl")
		.as_string()
		.unwrap_or(cover)
		.read();
	let title = manga.get("mangaName").as_string().unwrap().read();
	let author = manga
		.get("mangaAuthor")
		.as_string()
		.unwrap()
		.read()
		.trim()
		.to_string();
	let artist = String::from("");
	let description = manga
		.get("mangaIntro")
		.as_string()
		.unwrap_or_default()
		.read();
	let url = manga.get("shareUrl").as_string().unwrap_or_default().read();
	let categories = manga
		.get("mangaTheme")
		.as_string()
		.unwrap()
		.read()
		.split(" ")
		.map(|category| category.to_string())
		.filter(|category| !category.is_empty())
		.collect::<Vec<String>>();
	let status = match manga.get("mangaIsOver").as_int().unwrap_or(-1) {
		0 => MangaStatus::Ongoing,
		1 => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Safe;
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
	let words = manga.get("mangaWords").as_array().unwrap();
	let rolls = manga.get("mangaRolls").as_array().unwrap();
	let episode = manga.get("mangaEpisode").as_array().unwrap();
	let mut chapters: Vec<Chapter> = Vec::new();
	chapters.append(&mut parse_chapters(words));
	chapters.append(&mut parse_chapters(rolls));
	chapters.append(&mut parse_chapters(episode));
	chapters.sort_by_key(|a| a.chapter.to_bits());
	chapters.reverse();
	chapters
}

pub fn parse_chapters(chapter_list: ArrayRef) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for item in chapter_list {
		let item = item.as_object().unwrap();
		let id = item.get("sectionId").as_int().unwrap().to_string();
		let section_title = item.get("sectionTitle").as_string().unwrap().read();
		let section_name = item.get("sectionName").as_string().unwrap().read();
		let is_must_pay = item.get("isMustPay").as_int().unwrap_or_default();
		let title = if section_title.is_empty() {
			section_name
		} else if is_must_pay == 0 {
			format!("{} {}", section_name, section_title)
		} else {
			format!("{} {} {}", "🔒", section_name, section_title)
		};
		let chapter = item.get("sectionSort").as_float().unwrap_or_default() as f32;
		let url = helper::gen_chapter_url(id.clone());
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		})
	}

	chapters
}

pub fn parse_page_list(chapter: ObjectRef) -> Vec<Page> {
	let list = chapter.get("mangaSectionImages").as_array().unwrap();
	let host_list = chapter.get("hostList").as_array().unwrap();
	let query = chapter.get("query").as_string().unwrap();
	let mut pages: Vec<Page> = Vec::new();

	if host_list.is_empty() {
		return pages;
	}

	let host = host_list.get(0).as_string().unwrap().read();

	for (index, item) in list.enumerate() {
		let item = item.as_string().unwrap().read();
		let index = index as i32;
		let url = format!("{}{}{}", host, item, query);
		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	pages
}
