#![no_std]

mod html;
mod net;

use aidoku::{
	Chapter, DeepLinkHandler, DeepLinkResult, FilterValue, ImageRequestProvider, Manga,
	MangaPageResult, Page, Result, Source,
	alloc::{String, Vec, string::ToString as _},
	imports::net::Request,
	prelude::*,
};
use html::{ChapterPage as _, MangaPage as _};
use net::Url;

pub const BASE_URL: &str = "https://mangabz.com/";
pub const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_3_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.0.0 Safari/537.36";

struct Mangabz;

impl Source for Mangabz {
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
		let html = url.request()?.html()?;
		html.manga_page_result()
	}

	fn get_manga_update(
		&self,
		mut manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		if needs_details {
			let url = format!("{}{}bz/", BASE_URL, manga.key);
			let html = Request::get(url)?
				.header("Referer", BASE_URL)
				.header("User-Agent", USER_AGENT)
				.html()?;
			html.update_details(&mut manga)?;
		}

		if needs_chapters {
			let url = format!("{}{}bz/", BASE_URL, manga.key);
			let html = Request::get(url)?
				.header("Referer", BASE_URL)
				.header("User-Agent", USER_AGENT)
				.html()?;
			manga.chapters = Some(html.chapters()?);
		}

		Ok(manga)
	}

	fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		let url = format!(
			"{}m{}/chapterimage.ashx?cid={1}&page=",
			BASE_URL, chapter.key
		);
		crate::html::get_page_list(url)
	}
}

impl ImageRequestProvider for Mangabz {
	fn get_image_request(
		&self,
		url: String,
		_context: Option<aidoku::PageContext>,
	) -> Result<Request> {
		Ok(Request::get(url)?
			.header("User-Agent", USER_AGENT)
			.header("Referer", BASE_URL))
	}
}

impl DeepLinkHandler for Mangabz {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		if url.contains("bz/") {
			let key = url
				.replace("bz/", "")
				.split('/')
				.next_back()
				.ok_or_else(|| error!("invalid url"))?
				.to_string();
			return Ok(Some(DeepLinkResult::Manga { key }));
		}

		if url.contains(".com/m") {
			let _id = url.split("/m").last().expect("chapter id").replace('/', "");
			// For chapter, we need manga key too, but we don't have it from URL
			// This is a limitation, but we'll just return None for now
			return Ok(None);
		}

		Ok(None)
	}
}

register_source!(Mangabz, ImageRequestProvider, DeepLinkHandler);

#[cfg(test)]
type LiveAuditSource = Mangabz;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
