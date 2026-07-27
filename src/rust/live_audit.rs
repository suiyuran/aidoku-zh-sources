//! Reusable opt-in live website audit.
//!
//! Each current-API source aliases its source type to `LiveAuditSource` before
//! including this module. Run a source with:
//! `cargo test full_reading_chain -- --ignored --nocapture`.

use super::*;
use aidoku::{PageContent, imports::net::Request};
use aidoku_test::aidoku_test;

#[aidoku_test]
#[ignore = "requires the live website"]
fn full_reading_chain() {
	let source = LiveAuditSource::new();
	let list = source
		.get_search_manga_list(None, 1, Vec::new())
		.expect("list");
	assert!(!list.entries.is_empty(), "empty list");

	let (manga, chapter, pages) = list
		.entries
		.into_iter()
		.take(10)
		.find_map(|entry| {
			let title = entry.title.clone();
			aidoku::println!("{title}: key={}", entry.key);
			let manga = match source.get_manga_update(entry, true, true) {
				Ok(manga) => manga,
				Err(error) => {
					aidoku::println!("{title}: details/chapters failed: {error:?}");
					return None;
				}
			};
			let (chapter, pages) = manga
				.chapters
				.as_ref()?
				.iter()
				.filter(|chapter| !chapter.locked)
				.take(12)
				.find_map(
					|chapter| match source.get_page_list(manga.clone(), chapter.clone()) {
						Ok(pages) if !pages.is_empty() => Some((chapter.clone(), pages)),
						Ok(_) => {
							aidoku::println!("{title}: chapter {} returned no pages", chapter.key);
							None
						}
						Err(error) => {
							aidoku::println!("{title}: chapter {} failed: {error:?}", chapter.key);
							None
						}
					},
				)?;
			Some((manga, chapter, pages))
		})
		.expect("no readable manga in the first ten list entries");

	let PageContent::Url(url, _) = &pages[0].content else {
		panic!("first page is not a URL")
	};
	let referer = chapter
		.url
		.as_ref()
		.or(manga.url.as_ref())
		.map(String::as_str)
		.unwrap_or_default();
	aidoku::println!("image url: {url}");
	let response = Request::get(url)
		.expect("image request")
		.header(
			"User-Agent",
			"Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Mobile/15E148 Safari/604.1",
		)
		.header("Referer", referer)
		.header("Accept-Language", "zh-TW,zh;q=0.9,en;q=0.8")
		.header("Cookie", "isAdult=1; usepc=0")
		.send()
		.expect("image response");
	assert_eq!(response.status_code(), 200);
	assert!(response.get_data().expect("image data").len() > 1000);
}
