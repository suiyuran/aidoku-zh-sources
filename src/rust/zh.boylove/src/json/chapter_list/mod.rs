use super::*;
use aidoku::alloc::borrow::ToOwned as _;
use chinese_number::{ChineseCountMethod, ChineseToNumber as _};
use regex::Regex;
use spin::Lazy;

#[derive(Deserialize)]
pub struct Root {
	result: Result,
}

impl From<Root> for Vec<Chapter> {
	fn from(root: Root) -> Self {
		root.result.into()
	}
}

#[derive(Deserialize)]
struct Result {
	list: Vec<ListItem>,
}

impl From<Result> for Vec<Chapter> {
	fn from(result: Result) -> Self {
		result.list.into_iter().map(Into::into).collect()
	}
}

#[derive(Deserialize)]
struct ListItem {
	id: u32,
	title: String,
	create_time: i64,
}

impl From<ListItem> for Chapter {
	fn from(list_item: ListItem) -> Self {
		let key = list_item.id.to_string();

		let (volume_number, chapter_number, title) = parse(list_item.title.trim());

		let date_uploaded = list_item.create_time;

		let url = Url::chapter(&key).into();

		Self {
			key,
			title,
			chapter_number,
			volume_number,
			date_uploaded: Some(date_uploaded),
			url: Some(url),
			..Default::default()
		}
	}
}

pub fn parse(title: &str) -> (Option<f32>, Option<f32>, Option<String>) {
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
	let volume_numb = parse_number("volume_num");
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
		volume_numb,
		chapter_num,
		(!real_title.is_empty()).then_some(real_title),
	)
}

static RE: Lazy<Regex> = Lazy::new(|| {
	#[expect(clippy::unwrap_used)]
	Regex::new(
		r"^(?<volume>第?(?<volume_num>[\d零一二三四五六七八九十百千]+(\.\d+)?)[卷部季冊册] ?)?(?<chapter>第?(?<chapter_num>[\d零一二三四五六七八九十百千]+(\.\d+)?)(?<more_chapters>-(\d+(\.\d+)?))?[话話回]?)?([ +]|$)",
	)
	.unwrap()
});

#[cfg(test)]
mod test;
