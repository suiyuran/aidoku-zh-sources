use aidoku::{
	prelude::*,
	std::{ArrayRef, ObjectRef, String, Vec},
	Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page,
};

pub const WWW_URL: &str = "https://nicohub.cc";

pub fn parse_manga_list(manga_list: ArrayRef) -> Vec<Manga> {
	manga_list
		.map(|manga| parse_manga(manga.as_object().unwrap()))
		.collect::<Vec<Manga>>()
}

pub fn parse_manga(manga: ObjectRef) -> Manga {
	let id = manga.get("id").as_string().unwrap().read();
	let cover = manga.get("cover").as_string().unwrap().read();
	let title = manga.get("name").as_string().unwrap().read();
	let author = manga
		.get("author")
		.as_array()
		.unwrap()
		.map(|author| author.as_string().unwrap().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = manga.get("introduction").as_string().unwrap().read();
	let url = format!("{}/comic/info/{}", WWW_URL, id.clone());
	let filters = manga.get("type").as_object().unwrap();
	let categories = filters
		.get("题材")
		.as_object()
		.unwrap_or_default()
		.keys()
		.map(|category| category.as_string().unwrap().read())
		.collect::<Vec<String>>();
	let status = match filters
		.get("连载")
		.as_object()
		.unwrap_or_default()
		.keys()
		.map(|status| status.as_string().unwrap().read())
		.collect::<Vec<String>>()
		.pop()
		.unwrap_or_default()
		.as_str()
	{
		"连载中" => MangaStatus::Ongoing,
		"已完结" => MangaStatus::Completed,
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

pub fn parse_chapter_list(chapter_list: ArrayRef) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for item in chapter_list {
		let item = item.as_object().unwrap();
		let id = item.get("id").as_string().unwrap().read();
		let title = item.get("name").as_string().unwrap().read();
		let chapter = item.get("sort").as_int().unwrap() as f32;
		let url = String::new();
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

pub fn parse_page_list(page_list: ArrayRef) -> Vec<Page> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in page_list.enumerate() {
		let item = item.as_object().unwrap();
		let index = index as i32;
		let url = item.get("path").as_string().unwrap().read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	pages
}
