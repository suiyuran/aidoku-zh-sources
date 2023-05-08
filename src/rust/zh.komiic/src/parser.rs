use crate::helper;
use aidoku::{
	std::{ArrayRef, ObjectRef, String, Vec},
	Chapter, Manga, MangaContentRating, MangaStatus, MangaViewer, Page,
};

pub fn parse_manga_list(manga_list: ArrayRef) -> Vec<Manga> {
	manga_list
		.map(|manga| parse_manga(manga.as_object().unwrap()))
		.collect::<Vec<Manga>>()
}

pub fn parse_manga(manga: ObjectRef) -> Manga {
	let id = manga.get("id").as_string().unwrap().read();
	let cover = manga.get("imageUrl").as_string().unwrap().read();
	let title = manga.get("title").as_string().unwrap().read();
	let author = manga
		.get("authors")
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
	let description = String::new();
	let url = helper::gen_manga_url(id.clone());
	let categories = manga
		.get("categories")
		.as_array()
		.unwrap()
		.map(|category| {
			category
				.as_object()
				.unwrap()
				.get("name")
				.as_string()
				.unwrap()
				.read()
		})
		.collect::<Vec<String>>();
	let status = match manga.get("status").as_string().unwrap().read().as_str() {
		"ONGOING" => MangaStatus::Ongoing,
		"END" => MangaStatus::Completed,
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

pub fn parse_chapter_list(manga_id: String, chapter_list: ArrayRef) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in chapter_list.enumerate() {
		let chapter = item.as_object().unwrap();
		let id = chapter.get("id").as_string().unwrap().read();
		let title = chapter.get("serial").as_string().unwrap().read();
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

pub fn parse_page_list(manga_id: String, chapter_id: String, page_list: ArrayRef) -> Vec<Page> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in page_list.enumerate() {
		let page = item.as_object().unwrap();
		let index = index as i32;
		let id = page.get("kid").as_string().unwrap().read();
		let url = helper::gen_page_url(manga_id.clone(), chapter_id.clone(), id);
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	pages
}
