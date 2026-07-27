use super::*;

#[derive(Deserialize)]
pub struct Root {
	result: Result,
}

impl From<Root> for MangaPageResult {
	fn from(root: Root) -> Self {
		root.result.into()
	}
}

#[derive(Deserialize)]
struct Result {
	list: Vec<MangaObj>,
	#[serde(rename = "lastPage")]
	last_page: bool,
}

impl From<Result> for MangaPageResult {
	fn from(result: Result) -> Self {
		let entries = result.list.into_iter().filter_map(Into::into).collect();

		let has_next_page = !result.last_page;

		Self {
			entries,
			has_next_page,
		}
	}
}

#[cfg(test)]
mod test;
