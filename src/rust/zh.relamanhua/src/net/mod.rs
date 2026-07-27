use crate::html::GenresPage as _;
use aidoku::{
	FilterValue, Result,
	alloc::{String, format, string::ToString as _},
	bail, error,
	helpers::uri::QueryParameters,
	imports::net::Request,
};
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum::{Display, EnumIs, FromRepr};

#[derive(Display, EnumIs)]
#[strum(prefix = "https://www.manga2026.com")]
pub enum Url<'a> {
	#[strum(to_string = "/filter")]
	GenresPage,
	#[strum(to_string = "/comics?{0}")]
	Filters(FiltersQuery),
	#[strum(to_string = "/search")]
	SearchPage,
	#[strum(to_string = "{api}?{query}")]
	Search { api: String, query: SearchQuery },
	#[strum(to_string = "/comic/{key}")]
	Manga { key: &'a str },
	#[strum(to_string = "/comicdetail/{manga_key}/chapters")]
	ChapterList { manga_key: &'a str },
	#[strum(to_string = "/comic/{manga_key}/chapter/{key}")]
	Chapter { manga_key: &'a str, key: &'a str },
}

impl Url<'_> {
	pub fn request(&self) -> Result<Request> {
		let mut request = Request::get(self.to_string())?.header(
			"User-Agent",
			"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15",
		);

		if self.is_chapter_list() {
			request.set_header("dnts", "1");
		}

		Ok(request)
	}

	pub fn from_query_or_filters(
		query: Option<&str>,
		page: i32,
		filters: &[FilterValue],
	) -> Result<Self> {
		if let Some(keyword) = query {
			let search_query = SearchQuery::new(page, keyword);
			let url = Self::search(search_query)?;
			return Ok(url);
		}

		let mut genre = String::new();
		let mut is_asc = false;
		let mut sort = Sort::LastUpdated;

		for filter in filters {
			#[expect(clippy::wildcard_enum_match_arm)]
			match filter {
				FilterValue::Text { id, value } => match id.as_str() {
					"author" => {
						let search_query = SearchQuery::new(page, value);
						let url = Self::search(search_query)?;
						return Ok(url);
					}
					_ => bail!("Invalid text filter ID: `{id}`"),
				},

				FilterValue::Sort {
					id,
					index,
					ascending,
				} => match id.as_str() {
					"排序" => {
						is_asc = *ascending;
						sort = Sort::from_repr(*index)
							.ok_or_else(|| error!("Invalid `排序` index: `{index}`"))?;
					}
					_ => bail!("Invalid sort filter ID: `{id}`"),
				},

				FilterValue::Select { id, value } => match id.as_str() {
					"題材" => genre = value.into(),
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
					_ => bail!("Invalid select filter ID: `{id}`"),
				},

				_ => bail!("Invalid filter: `{filter:?}`"),
			}
		}

		let filters_query = FiltersQuery::new(&genre, is_asc, sort, page);
		Ok(Self::Filters(filters_query))
	}

	fn search(query: SearchQuery) -> Result<Self> {
		// Use the API endpoint directly from the search.html JavaScript
		let api = "/api/v3/search/comic".into();
		Ok(Self::Search { api, query })
	}
}

impl<'a> Url<'a> {
	pub const fn manga(key: &'a str) -> Self {
		Self::Manga { key }
	}

	pub const fn chapter_list(manga_key: &'a str) -> Self {
		Self::ChapterList { manga_key }
	}

	pub const fn chapter(manga_key: &'a str, key: &'a str) -> Self {
		Self::Chapter { manga_key, key }
	}
}

impl From<Url<'_>> for String {
	fn from(url: Url<'_>) -> Self {
		url.to_string()
	}
}

pub struct FiltersQuery(QueryParameters);

impl FiltersQuery {
	fn new(genre: &str, is_asc: bool, sort: Sort, page: i32) -> Self {
		let mut query = QueryParameters::new();

		if !genre.is_empty() {
			query.push_encoded("theme", Some(genre));
		}

		let order = if is_asc { "" } else { "-" };
		let sort_by = format!("{order}{sort}");
		query.push_encoded("ordering", Some(&sort_by));

		let limit = 50;
		let offset = Offset::new(page, limit).to_string();
		query.push_encoded("offset", Some(&offset));
		query.push_encoded("limit", Some(&limit.to_string()));

		Self(query)
	}
}

impl Display for FiltersQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

pub struct SearchQuery(QueryParameters);

impl SearchQuery {
	fn new(page: i32, keyword: &str) -> Self {
		let mut query = QueryParameters::new();

		let limit = 12;
		let offset = Offset::new(page, limit).to_string();
		query.push_encoded("offset", Some(&offset));

		query.push_encoded("platform", Some("2"));
		query.push_encoded("limit", Some(&limit.to_string()));
		query.push("q", Some(keyword));
		query.push_encoded("free_type", Some("1")); // Always search for free comics

		Self(query)
	}
}

impl Display for SearchQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

#[derive(Display, Clone, Copy, FromRepr)]
#[repr(i32)]
enum Sort {
	#[strum(to_string = "datetime_updated")]
	LastUpdated,
	#[strum(to_string = "popular")]
	Popularity,
}

struct Offset(i32);

impl Offset {
	fn new(page: i32, limit: i32) -> Self {
		let offset = page
			.checked_sub(1)
			.filter(|index| *index >= 0)
			.unwrap_or(0)
			.saturating_mul(limit);
		Self(offset)
	}
}

impl Display for Offset {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}
