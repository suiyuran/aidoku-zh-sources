use super::*;

#[derive(Deserialize)]
pub struct Root {
	data: Vec<MangaObj>,
}

impl From<Root> for MangaPageResult {
	fn from(root: Root) -> Self {
		let entries = root.data.into_iter().filter_map(Into::into).collect();

		let has_next_page = true;

		Self {
			entries,
			has_next_page,
		}
	}
}

#[cfg(test)]
mod test;
