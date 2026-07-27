pub mod chapter_list;
pub mod daily_update;
pub mod home;
pub mod manga_page_result;
pub mod random;

use super::*;
use aidoku::{
	ContentRating, MangaStatus, MangaWithChapter, alloc::string::ToString as _, serde::Deserialize,
};
use chapter_list::parse;

#[derive(Deserialize)]
struct MangaObj {
	id: u32,
	title: String,
	lanmu_id: Option<u8>,
	image: Option<String>,
	auther: Option<String>,
	desc: Option<String>,
	mhstatus: Option<u8>,
	keyword: Option<String>,
	last_chapter_title: Option<String>,
}

impl From<MangaObj> for Option<Manga> {
	fn from(manga: MangaObj) -> Self {
		if manga.lanmu_id == Some(5) {
			return None;
		}

		let tags = manga.keyword.map(|keyword| {
			keyword
				.split(',')
				.filter(|tag| !tag.is_empty())
				.map(Into::into)
				.collect::<Vec<_>>()
		});

		if tags
			.as_deref()
			.map(|tags_slice| tags_slice.iter().any(|tag| tag == "香香公告"))
			== Some(true)
		{
			return None;
		}

		let key = manga.id.to_string();

		let title = manga.title;

		let cover = manga.image.map(|image| {
			if image.starts_with('/') {
				Url::Abs(&image).into()
			} else {
				image
			}
		});

		let authors = manga.auther.map(|auther| {
			auther
				.split([',', '&', '/'])
				.filter_map(|author| {
					let trimmed_author = author.trim();
					(!trimmed_author.is_empty()).then(|| trimmed_author.into())
				})
				.collect()
		});

		let description = manga
			.desc
			.map(|desc| desc.trim().replace("\r\n", "\n").replace('\n', "  \n"));

		let url = Url::manga(&key).into();

		let status = match manga.mhstatus {
			Some(0) => MangaStatus::Ongoing,
			Some(1) => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};

		let content_rating = tags
			.as_deref()
			.and_then(|tags_slice| {
				tags_slice
					.iter()
					.any(|tag| tag == "清水")
					.then_some(ContentRating::Safe)
			})
			.unwrap_or(ContentRating::NSFW);

		Some(Manga {
			key,
			title,
			cover,
			authors,
			description,
			url: Some(url),
			tags,
			status,
			content_rating,
			..Default::default()
		})
	}
}

impl From<MangaObj> for Option<MangaWithChapter> {
	fn from(manga_obj: MangaObj) -> Self {
		let (volume_number, chapter_number, title) = manga_obj
			.last_chapter_title
			.as_deref()
			.map(|last_chapter_title| parse(last_chapter_title.trim()))
			.unwrap_or_default();

		let chapter = Chapter {
			title,
			chapter_number,
			volume_number,
			..Default::default()
		};

		let manga = Option::from(manga_obj)?;

		Some(MangaWithChapter { manga, chapter })
	}
}
