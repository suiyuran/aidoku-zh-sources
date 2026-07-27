#![no_std]
extern crate alloc;

mod html;
mod net;

use crate::html::{ChapterListPage, ChapterPage, FiltersPage, MangaPage};
use crate::net::Url;
use aidoku::imports::net::HttpMethod;
use aidoku::{
	BasicLoginHandler, Chapter, DeepLinkHandler, DeepLinkResult, FilterValue, Manga,
	MangaPageResult, Page, Result, Source,
	alloc::{String, Vec},
	prelude::*,
};
use alloc::string::ToString;

struct Zerobyw;

impl Source for Zerobyw {
	fn new() -> Self {
		Self
	}

	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
		let url = Url::from_query_or_filters(query.as_deref(), page, &filters)?;
		let request = url.request(HttpMethod::Get)?;
		let manga_page_result = request.html()?.manga_page_result()?;
		Ok(manga_page_result)
	}

	fn get_manga_update(
		&self,
		mut manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		let manga_page = Url::manga(&manga.key).request(HttpMethod::Get)?.html()?;
		if needs_details {
			manga_page.update_details(&mut manga)?;
		}
		if needs_chapters {
			let chapters = manga_page.chapter_list()?;
			manga.chapters = Some(chapters);
		}
		Ok(manga)
	}

	fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		Url::chapter(&chapter.key)
			.request(HttpMethod::Get)?
			.html()?
			.pages()
	}
}

impl DeepLinkHandler for Zerobyw {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		let parts: Vec<&str> = url.split('?').collect();
		let path = parts[0];
		let query = parts.get(1).copied();
		if path.contains("/details/") {
			if let Some(q) = query
				&& let Some(kuid) = q
					.split('&')
					.find(|p| p.starts_with("kuid="))
					.and_then(|p| p.split('=').nth(1))
			{
				return Ok(Some(DeepLinkResult::Manga {
					key: kuid.to_string(),
				}));
			}
		} else if path.contains("/view/") {
			// Chapter link miss manga_key
			return Ok(None);
		}
		Ok(None)
	}
}

impl BasicLoginHandler for Zerobyw {
	fn handle_basic_login(&self, _key: String, username: String, password: String) -> Result<bool> {
		net::login(&username, &password)
	}
}

register_source!(Zerobyw, DeepLinkHandler, BasicLoginHandler);

#[cfg(test)]
type LiveAuditSource = Zerobyw;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
