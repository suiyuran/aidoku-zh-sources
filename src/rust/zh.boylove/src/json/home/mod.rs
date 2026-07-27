use super::*;
use aidoku::{HomeComponent, HomeComponentValue};

#[derive(Deserialize)]
pub struct Root {
	data: Vec<Data>,
}

impl From<Root> for HomeLayout {
	fn from(root: Root) -> Self {
		let components = root
			.data
			.into_iter()
			.filter_map(|data| (data.name != "article").then(|| data.into()))
			.collect();

		Self { components }
	}
}

#[expect(clippy::struct_field_names)]
#[derive(Deserialize)]
struct Data {
	data: Vec<MangaObj>,
	title: String,
	name: String,
}

impl From<Data> for HomeComponent {
	fn from(data: Data) -> Self {
		let title = data.title;

		let entries = data.data.into_iter().filter_map(Into::into).collect();

		let listing = match data.name.as_str() {
			"newest" => Some(Listing {
				id: "11".into(),
				name: "最新".into(),
				..Default::default()
			}),
			"recommend" => Some(Listing {
				id: "recommend".into(),
				name: "無碼專區".into(),
				..Default::default()
			}),
			"topestmh" => Some(Listing {
				id: "topestmh".into(),
				name: "排行榜".into(),
				..Default::default()
			}),
			"cnxh" => Some(Listing {
				name: "猜你喜歡".into(),
				..Default::default()
			}),
			_ => None,
		};

		let value = HomeComponentValue::MangaChapterList {
			page_size: Some(2),
			entries,
			listing,
		};

		Self {
			title: Some(title),
			value,
			..Default::default()
		}
	}
}

#[cfg(test)]
mod test;
