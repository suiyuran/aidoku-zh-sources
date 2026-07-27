use crate::{BASE_URL, USER_AGENT};
use aidoku::{
	FilterValue, Result,
	alloc::{String, Vec, string::ToString as _},
	helpers::uri::encode_uri,
	imports::net::Request,
};
use core::fmt::{Display, Formatter, Result as FmtResult};

const SORTS: &[&str] = &["s10", "s2", "s18"];

const GENRE_OPTIONS: &[&str] = &[
	"热血", "恋爱", "校园", "冒险", "后宫", "科幻", "战争", "悬疑", "推理", "搞笑", "奇幻", "魔法",
	"神鬼", "历史", "同人", "运动", "绅士", "机甲",
];
const GENRE_IDS: &[&str] = &[
	"tag31", "tag26", "tag1", "tag2", "tag8", "tag25", "tag12", "tag17", "tag33", "tag37", "tag14",
	"tag15", "tag20", "tag4", "tag30", "tag34", "tag36", "tag40",
];

#[derive(Clone)]
pub enum Url {
	Manga { id: String },
	Search { query: String, page: i32 },
	Filter { segments: String, page: i32 },
}

impl Url {
	pub fn manga(id: String) -> Self {
		Self::Manga { id }
	}

	pub fn from_query_or_filters(
		query: Option<&str>,
		page: i32,
		filters: &[FilterValue],
	) -> Result<Self> {
		if let Some(q) = query {
			return Ok(Self::Search {
				query: encode_uri(q),
				page,
			});
		}

		let mut sort: Option<&str> = None;
		let mut genre: Option<&str> = None;
		let mut area: Option<&str> = None;
		let mut status: Option<&str> = None;
		let mut audience: Option<&str> = None;
		let mut pay: Option<&str> = None;

		for filter in filters {
			match filter {
				FilterValue::Sort { id, index, .. } if id == "排序" => {
					sort = SORTS.get(*index as usize).copied();
				}
				FilterValue::Select { id, value } => match id.as_str() {
					"题材" => genre = Some(value),
					"地区" => area = Some(value),
					"进度" => status = Some(value),
					"受众" => audience = Some(value),
					"收费" => pay = Some(value),
					"genre" => {
						if let Some(idx) =
							GENRE_OPTIONS.iter().position(|&opt| opt == value.as_str())
						{
							genre = Some(GENRE_IDS[idx]);
						}
					}
					_ => {}
				},
				_ => {}
			}
		}

		let segments = [area, genre, audience, status, pay, sort]
			.into_iter()
			.flatten()
			.collect::<Vec<_>>()
			.join("-");

		Ok(Self::Filter { segments, page })
	}

	pub fn request(&self) -> Result<Request> {
		let url = self.to_string();
		Ok(Request::get(url)?
			.header("User-Agent", USER_AGENT)
			.header("Accept-Language", "zh-TW")
			.header(
				"Accept",
				"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
			)
			.header("Referer", BASE_URL)
			// Restricted (限制漫画) series replace their chapter list with an
			// age-gate warning page unless this consent cookie is present.
			.header("Cookie", "isAdult=1"))
	}
}

impl Display for Url {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Url::Manga { id } => write!(f, "{}/{}", BASE_URL, id),
			Url::Search { query, page } => {
				write!(
					f,
					"{}/search?title={}&language=1&page={}",
					BASE_URL, query, page
				)
			}
			Url::Filter { segments, page } => {
				if segments.is_empty() {
					write!(f, "{}/manhua-list-p{}/", BASE_URL, page)
				} else {
					write!(f, "{}/manhua-list-{}-p{}/", BASE_URL, segments, page)
				}
			}
		}
	}
}
