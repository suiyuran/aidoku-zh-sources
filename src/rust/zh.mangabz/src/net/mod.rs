use crate::{BASE_URL, USER_AGENT};
use aidoku::{
	FilterValue, Result,
	alloc::{String, string::ToString as _},
	helpers::uri::encode_uri,
	imports::net::Request,
};
use core::fmt::{Display, Formatter, Result as FmtResult};

const GENRE_OPTIONS: &[&str] = &[
	"熱血", "戀愛", "校園", "冒險", "科幻", "生活", "懸疑", "魔法", "運動",
];

const GENRE_IDS: &[&str] = &["31", "26", "1", "2", "25", "11", "17", "15", "34"];

#[derive(Clone)]
pub enum Url {
	Search {
		query: String,
		page: i32,
	},
	Filter {
		genre: String,
		status: String,
		sort: String,
		page: i32,
	},
}

impl Url {
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

		let mut genre = String::from("0");
		let mut status = String::from("0");
		let mut sort = String::from("10");

		for filter in filters {
			match filter {
				FilterValue::Text { value, .. } => {
					return Ok(Self::Search {
						query: encode_uri(value.clone()),
						page,
					});
				}
				FilterValue::Select { id, value } => match id.as_str() {
					"題材" => genre = value.clone(),
					"狀態" => status = value.clone(),
					"genre" => {
						if let Some(index) = GENRE_OPTIONS
							.iter()
							.position(|&option| option == value.as_str())
							&& let Some(id) = GENRE_IDS.get(index)
						{
							genre = id.to_string();
						}
					}
					_ => continue,
				},
				FilterValue::Sort {
					id,
					index,
					ascending: _,
				} => {
					if id == "排序" {
						let sort_options = ["10", "2"];
						if let Some(s) = sort_options.get(*index as usize) {
							sort = s.to_string();
						}
					}
				}
				_ => continue,
			}
		}

		Ok(Self::Filter {
			genre,
			status,
			sort,
			page,
		})
	}

	pub fn request(&self) -> Result<Request> {
		let url = self.to_string();
		Ok(Request::get(url)?
			.header("Referer", BASE_URL)
			.header("User-Agent", USER_AGENT))
	}
}

impl Display for Url {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Url::Search { query, page } => {
				write!(f, "{}search?title={}&page={}", BASE_URL, query, page)
			}
			Url::Filter {
				genre,
				status,
				sort,
				page,
			} => {
				write!(
					f,
					"{}manga-list-{}-{}-{}-p{}/",
					BASE_URL, genre, status, sort, page
				)
			}
		}
	}
}
