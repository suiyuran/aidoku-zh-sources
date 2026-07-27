use super::MangaItem;
use aidoku::{MangaPageResult, alloc::Vec, serde::Deserialize};

#[derive(Deserialize)]
pub struct Root {
	results: Results,
}

impl From<Root> for MangaPageResult {
	fn from(root: Root) -> Self {
		root.results.into()
	}
}

#[derive(Deserialize)]
struct Results {
	list: Vec<MangaItem>,
	total: u16,
	limit: u16,
	offset: u16,
}

impl From<Results> for MangaPageResult {
	fn from(results: Results) -> Self {
		let entries = results.list.into_iter().map(Into::into).collect();

		let has_next_page = results
			.offset
			.checked_add(results.limit)
			.is_some_and(|current_total| current_total < results.total);

		Self {
			entries,
			has_next_page,
		}
	}
}
