#![no_std]

mod html;
mod json;
mod net;

use aidoku::{
	Chapter, DeepLinkHandler, DeepLinkResult, DynamicFilters, Filter, FilterValue, Manga,
	MangaPageResult, Page, Result, Source,
	alloc::{String, Vec},
	imports::std::send_partial_result,
	register_source,
};
use html::{ChapterPage as _, FiltersPage as _, GenresPage as _, KeyPage as _, MangaPage as _};
use json::{chapter_list, search};
use net::Url;

struct Relamanhua;

impl Source for Relamanhua {
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
		let request = url.request()?;
		let manga_page_result = if url.is_filters() {
			request.html()?.manga_page_result()?
		} else {
			request.json_owned::<search::Root>()?.into()
		};
		Ok(manga_page_result)
	}

	fn get_manga_update(
		&self,
		mut manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		let manga_page = Url::manga(&manga.key).request()?.html()?;
		if needs_details {
			manga_page.update_details(&mut manga)?;

			if needs_chapters {
				send_partial_result(&manga);
			} else {
				return Ok(manga);
			}
		}

		let key = manga_page.key()?;
		manga.chapters = Url::chapter_list(&manga.key)
			.request()?
			.json_owned::<chapter_list::Root>()?
			.chapters(&key)?;

		Ok(manga)
	}

	fn get_page_list(&self, manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		Url::chapter(&manga.key, &chapter.key)
			.request()?
			.html()?
			.pages()
	}
}

impl DeepLinkHandler for Relamanhua {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		let mut splits = url.split('/').skip(3);
		let deep_link_result = match splits.next() {
			Some("comic") => match (splits.next(), splits.next(), splits.next()) {
				(Some(key), None, None) => Some(DeepLinkResult::Manga { key: key.into() }),
				(Some(manga_key), Some("chapter"), Some(key)) => Some(DeepLinkResult::Chapter {
					manga_key: manga_key.into(),
					key: key.into(),
				}),
				_ => None,
			},

			Some("h5") => match (splits.next(), splits.next(), splits.next()) {
				(Some("details"), Some("comic"), Some(key)) => {
					Some(DeepLinkResult::Manga { key: key.into() })
				}
				(Some("comicContent"), Some(manga_key), Some(key)) => {
					Some(DeepLinkResult::Chapter {
						manga_key: manga_key.into(),
						key: key.into(),
					})
				}
				_ => None,
			},

			_ => None,
		};
		Ok(deep_link_result)
	}
}

impl DynamicFilters for Relamanhua {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
		let genre = Url::GenresPage.request()?.html()?.filter()?.into();
		Ok([genre].into())
	}
}

register_source!(Relamanhua, DeepLinkHandler, DynamicFilters);

#[cfg(test)]
type LiveAuditSource = Relamanhua;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
