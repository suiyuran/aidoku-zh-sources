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

pub const BASE_URL: &str = "https://www.dm5.com";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/139.0.0.0 Safari/537.36";

struct Dm5;

impl Source for Dm5 {
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
		let html = Url::manga(manga.key.clone()).request()?.html()?;
		if needs_details {
			html.update_details(&mut manga)?;
		}
		if needs_chapters {
			manga.chapters = Some(html.chapters()?);
		}
		Ok(manga)
	}

	fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		html::get_page_list(&chapter.key)
	}
}

impl ImageRequestProvider for Dm5 {
	fn get_image_request(
		&self,
		url: String,
		_context: Option<aidoku::PageContext>,
	) -> Result<Request> {
		let cid = url.split("cid=").nth(1).and_then(|s| s.split('&').next());

		let referer = if let Some(cid) = cid {
			format!("{}/m{}", BASE_URL, cid)
		} else {
			BASE_URL.to_string()
		};

		Ok(Request::get(url)?
			.header("User-Agent", USER_AGENT)
			.header("Referer", &referer))
	}
}

impl DeepLinkHandler for Dm5 {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		let path = url.trim_start_matches(BASE_URL);
		let mut parts = path.split('/').filter(|s| !s.is_empty());
		let result = match parts.next() {
			Some(key) if key.starts_with('m') => match parts.next() {
				None => Some(DeepLinkResult::Manga { key: key.into() }),
				Some(chapter_key) => Some(DeepLinkResult::Chapter {
					manga_key: key.into(),
					key: chapter_key.into(),
				}),
			},
			_ => None,
		};
		Ok(result)
	}
}

register_source!(Dm5, ImageRequestProvider, DeepLinkHandler);

#[cfg(test)]
type LiveAuditSource = Dm5;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
