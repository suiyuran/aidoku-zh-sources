use crate::{USER_AGENT, html::GenresPage as _};
use aidoku::{
	FilterValue, Result,
	alloc::{String, Vec},
	helpers::uri::encode_uri,
	imports::net::Request,
	prelude::*,
};
use core::fmt::{Display, Formatter, Result as FmtResult};

const SORT: &[&str] = &["index", "update", "view", "rate"];

#[derive(Clone)]
pub enum Url {
	Search {
		query: String,
		page: i32,
	},
	Filter {
		region: String,
		genre: String,
		audience: String,
		progress: String,
		sort: String,
		page: i32,
	},
	GenresPage,
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

		let mut region = String::from("all");
		let mut genre = String::from("all");
		let mut audience = String::from("all");
		let mut progress = String::from("all");
		let mut sort_by = String::from(SORT[0]);

		for filter in filters {
			match filter {
				FilterValue::Text { value, .. } => {
					return Ok(Self::Search {
						query: encode_uri(value),
						page,
					});
				}
				FilterValue::Select { id, value } => match id.as_str() {
					"地区" => region = value.clone(),
					"受众" => audience = value.clone(),
					"进度" => progress = value.clone(),
					"类型" => genre = value.into(),
					"genre" => {
						let genres = Self::GenresPage.request()?.html()?.filter()?;
						let genre_id = genres
							.options
							.iter()
							.position(|option| option == value)
							.and_then(|index| genres.ids?.into_iter().nth(index))
							.ok_or_else(|| error!("Genre ID not found for option: `{value}`"))?;
						genre = genre_id.into();
					}
					_ => continue,
				},
				FilterValue::Sort { id, index, .. } => {
					if id == "排序" {
						sort_by = String::from(*SORT.get(*index as usize).unwrap_or(&SORT[0]));
					}
				}
				_ => continue,
			}
		}

		Ok(Self::Filter {
			region,
			genre,
			audience,
			progress,
			sort: sort_by,
			page,
		})
	}

	pub fn request(&self) -> Result<Request> {
		let url = format!("{}", self);
		Ok(Request::get(url)?
			.header("Referer", crate::settings::get_base_url())
			.header("User-Agent", USER_AGENT)
			.header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
			.header("Cookie", "device_view=pc; isAdult=1"))
	}
}

impl Display for Url {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Url::Search { query, page } => {
				write!(
					f,
					"{}s/{}_p{}.html",
					crate::settings::get_base_url(),
					query,
					page
				)
			}
			Url::Filter {
				region,
				genre,
				audience,
				progress,
				sort,
				page,
			} => {
				let mut filter_values: Vec<&str> = Vec::new();
				for val in [&region, &genre, &audience, &progress] {
					if *val != "all" {
						filter_values.push(val);
					}
				}

				let mut filter_str = filter_values.join("_");

				if !filter_str.is_empty() {
					filter_str = format!("/{}/", filter_str)
				}

				let page_str = format!("/{}_p{}.html", sort, page);

				write!(
					f,
					"{}list{}{}",
					crate::settings::get_base_url(),
					filter_str,
					page_str
				)
			}
			Url::GenresPage => {
				write!(f, "{}list/", crate::settings::get_base_url())
			}
		}
	}
}
