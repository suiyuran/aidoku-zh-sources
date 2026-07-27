#![no_std]

mod decoder;
mod html;
mod net;
mod settings;

use aidoku::{
	Chapter, DeepLinkHandler, DeepLinkResult, DynamicFilters, FilterValue, ImageRequestProvider,
	Manga, MangaPageResult, Page, Result, Source,
	alloc::{String, Vec},
	imports::net::Request,
	prelude::*,
};
use html::{ChapterPage as _, GenresPage as _, MangaPage as _};
use net::Url;

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

struct Manhuagui;

impl Source for Manhuagui {
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
			let url = format!("{}/comic/{}", settings::get_base_url(), manga.key);
			let html = Request::get(url)?
				.header("Referer", settings::get_base_url())
				.header("User-Agent", USER_AGENT)
				.header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
				.header("Cookie", "device_view=pc; isAdult=1")
				.html()?;
			html.update_details(&mut manga)?;
		}

		if needs_chapters {
			let url = format!("{}/comic/{}", settings::get_base_url(), manga.key);
			let html = Request::get(url)?
				.header("Referer", settings::get_base_url())
				.header("User-Agent", USER_AGENT)
				.header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
				.header("Cookie", "device_view=pc; isAdult=1")
				.html()?;
			manga.chapters = Some(html.chapters()?);
		}

		Ok(manga)
	}

	fn get_page_list(&self, manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		let base_url = format!(
			"{}/comic/{}/{}.html",
			settings::get_base_url(),
			manga.key,
			chapter.key
		);
		crate::html::get_page_list(base_url)
	}
}

impl ImageRequestProvider for Manhuagui {
	fn get_image_request(
		&self,
		url: String,
		_context: Option<aidoku::PageContext>,
	) -> Result<Request> {
		Ok(Request::get(url)?
			.header("User-Agent", USER_AGENT)
			.header("Referer", settings::get_base_url())
			.header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
			.header("Cookie", "device_view=pc; isAdult=1"))
	}
}

impl DeepLinkHandler for Manhuagui {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		if url.contains("/comic/") {
			let id = String::from(url.replace("/comic/", "").split('/').next().unwrap_or(""));
			return Ok(Some(DeepLinkResult::Manga { key: id }));
		}

		Ok(None)
	}
}

impl DynamicFilters for Manhuagui {
	fn get_dynamic_filters(&self) -> Result<Vec<aidoku::Filter>> {
		let genre = Url::GenresPage.request()?.html()?.filter()?.into();
		Ok([genre].into())
	}
}

register_source!(
	Manhuagui,
	ImageRequestProvider,
	DeepLinkHandler,
	DynamicFilters
);

#[cfg(test)]
type LiveAuditSource = Manhuagui;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
