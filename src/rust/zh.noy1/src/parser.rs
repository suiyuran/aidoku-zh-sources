use aidoku::{
	std::{ArrayRef, ObjectRef, String, Vec},
	Manga, MangaContentRating, MangaStatus, MangaViewer,
};
use alloc::string::ToString;

use crate::helper;

pub fn parse_manga_list(manga_list: ArrayRef) -> Vec<Manga> {
	manga_list
		.map(|manga| parse_manga(manga.as_object().unwrap()))
		.collect::<Vec<Manga>>()
}

pub fn parse_manga(manga: ObjectRef) -> Manga {
	let id = manga.get("Bid").as_int().unwrap().to_string();
	let cover = helper::gen_cover_url(id.clone());
	let title = manga.get("Bookname").as_string().unwrap().read();
	let author = manga.get("Author").as_string().unwrap().read();
	let artist = String::new();
	let description = String::new();
	let url = helper::gen_manga_url(id.clone());
	let categories = manga
		.get("Ptag")
		.as_string()
		.unwrap()
		.read()
		.split(" ")
		.map(|category| category.to_string())
		.collect::<Vec<String>>();
	let status = MangaStatus::Completed;
	let nsfw = MangaContentRating::Nsfw;
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
