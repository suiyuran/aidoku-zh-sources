use super::*;

#[derive(Deserialize)]
pub struct Root {
	result: Vec<MangaObj>,
	#[serde(rename = "pcPagi")]
	pc_pagi: PcPagi,
}

impl From<Root> for MangaPageResult {
	fn from(root: Root) -> Self {
		let entries = root.result.into_iter().filter_map(Into::into).collect();

		let has_next_page = root.pc_pagi.has_next_page();

		Self {
			entries,
			has_next_page,
		}
	}
}

#[derive(Deserialize)]
struct PcPagi {
	page_dump: u16,
	page_end: u16,
}

impl PcPagi {
	const fn has_next_page(&self) -> bool {
		self.page_dump < self.page_end
	}
}

#[cfg(test)]
mod test;
