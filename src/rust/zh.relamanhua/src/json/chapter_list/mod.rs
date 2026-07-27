use super::EncryptedJson as _;
use crate::net::Url;
use aidoku::{
	AidokuError, Chapter, HashMap, Result,
	alloc::{String, Vec, borrow::ToOwned as _, string::ToString as _},
	serde::Deserialize,
};
use chinese_number::{ChineseCountMethod, ChineseToNumber as _};
use regex::Regex;
use spin::Lazy;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Root {
	results: String,
}

impl Root {
	pub fn chapters(self, key: &str) -> Result<Option<Vec<Chapter>>> {
		let plain_text = self.results.decrypt(key)?;
		let chapters = serde_json::from_slice::<Results>(&plain_text)
			.map_err(AidokuError::message)?
			.into();
		Ok(chapters)
	}
}

static RE: Lazy<Regex> = Lazy::new(|| {
	#[expect(clippy::unwrap_used)]
	Regex::new(
		r"^(?<volume>第?(?<volume_num>[\d零一二三四五六七八九十百千]+(\.\d)?)[卷部季冊册]完?)?(?<chapter>(第|连载|CH)?(?<chapter_num>[\d零一二三四五六七八九十百千]+(\.\d+)?)(?<more_chapters>-(\d+(\.\d+)?))?[話话回]?)?([ +]|$)",
	)
	.unwrap()
});

#[derive(Deserialize)]
struct Results {
	build: Build,
	groups: HashMap<String, Group>,
}

impl From<Results> for Option<Vec<Chapter>> {
	fn from(results: Results) -> Self {
		let mut groups: Vec<_> = results
			.groups
			.into_values()
			.map(|group| {
				let mut chaps = group.into_chapters(&results.build.path_word);
				chaps.reverse();
				chaps
			})
			.collect();

		groups.sort_by(|a, b| {
			let a_max = a
				.iter()
				.map(|c| c.date_uploaded.unwrap_or_default())
				.max()
				.unwrap_or_default();
			let b_max = b
				.iter()
				.map(|c| c.date_uploaded.unwrap_or_default())
				.max()
				.unwrap_or_default();
			b_max.cmp(&a_max)
		});

		let chapters: Vec<Chapter> = groups.into_iter().flatten().collect();

		Some(chapters)
	}
}

#[derive(Deserialize)]
struct Build {
	path_word: String,
}

#[derive(Deserialize)]
struct Group {
	name: String,
	chapters: Vec<ChapterItem>,
}

impl Group {
	fn into_chapters(self, manga_key: &str) -> Vec<Chapter> {
		self.chapters
			.into_iter()
			.map(|chapter_item| chapter_item.into_chapter(manga_key, &self.name))
			.collect()
	}
}

#[derive(Deserialize)]
struct ChapterItem {
	r#type: u8,
	name: String,
	id: Uuid,
}

impl ChapterItem {
	fn into_chapter(self, manga_key: &str, group: &str) -> Chapter {
		let key = self.id.to_string();

		let (volume_number, chapter_number, title) = parse(self.r#type, self.name.trim());

		let date_uploaded = self
			.id
			.get_timestamp()
			.and_then(|timestamp| timestamp.to_unix().0.try_into().ok());

		let scanlators = [group.into()].into();

		let url = Url::chapter(manga_key, &key).into();

		Chapter {
			key,
			title,
			chapter_number,
			volume_number,
			date_uploaded,
			scanlators: Some(scanlators),
			url: Some(url),
			..Default::default()
		}
	}
}

fn parse(r#type: u8, title: &str) -> (Option<f32>, Option<f32>, Option<String>) {
	if r#type == 3 {
		return (None, None, Some(title.into()));
	}

	let mut chars = title.chars();
	if chars.next() == Some('全') && matches!(chars.next(), Some('一' | '1')) {
		match chars.next() {
			Some('卷' | '冊' | '册') => return (Some(1.0), None, Some(title.into())),
			Some('話' | '话' | '回') => return (None, Some(1.0), Some(title.into())),
			_ => (),
		}
	}

	let Some(caps) = RE.captures(title) else {
		return (None, None, Some(title.into()));
	};

	let parse_number = |group| {
		let str = caps.name(group)?.as_str();
		if let Ok(num) = str.parse() {
			return Some(num);
		}

		str.to_number(ChineseCountMethod::TenThousand).ok()
	};
	let volume_num = parse_number("volume_num");
	let chapter_num = parse_number("chapter_num");

	let mut real_title = title.to_owned();
	let mut remove_group = |name| {
		if let Some(group) = caps.name(name) {
			real_title = real_title.replace(group.as_str(), "");
		}
	};
	remove_group("volume");
	if caps.name("more_chapters").is_none() {
		remove_group("chapter");
	}
	real_title = real_title.trim().into();

	(
		volume_num,
		chapter_num,
		(!real_title.is_empty()).then_some(real_title),
	)
}
