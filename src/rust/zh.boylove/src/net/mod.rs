use super::*;
use aidoku::{
	alloc::{format, string::ToString as _},
	helpers::uri::{QueryParameters, encode_uri_component},
	imports::{defaults::defaults_get, net::Request},
};
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum::{Display, FromRepr};

#[derive(Display)]
#[strum(prefix = "https://boylove.cc")]
pub enum Url<'a> {
	#[strum(to_string = "/")]
	Home,
	#[strum(to_string = "{0}")]
	Abs(&'a str),
	#[strum(to_string = "/home/user/to{0}.html")]
	ChangeCharset(Charset),
	#[strum(to_string = "/home/book/cate.html")]
	FiltersPage,
	#[strum(
		to_string = "/home/api/cate/tp/1-{tags}-{status}-{sort_by}-{page}-{content_rating}-1-{view_permission}"
	)]
	Filters {
		tags: Tags<'a>,
		status: &'a str,
		sort_by: Sort,
		page: i32,
		content_rating: &'a str,
		view_permission: &'a str,
	},
	#[strum(to_string = "/home/api/searchk?{0}")]
	Search(SearchQuery),
	#[strum(to_string = "/home/book/index/id/{key}")]
	Manga { key: &'a str },
	#[strum(to_string = "/home/api/getChapterListInChapter/tp/{manga_key}-0-1-1000")]
	ChapterList { manga_key: &'a str },
	#[strum(to_string = "/home/book/capter/id/{key}")]
	Chapter { key: &'a str },
	#[strum(to_string = "/home/Api/getDailyUpdate.html?{0}")]
	DailyUpdate(DailyUpdateQuery),
	#[strum(to_string = "/home/api/getpage/tp/1-{0}-{1}")]
	Listing(&'a str, OffsetPage),
	#[strum(to_string = "/home/Api/getCnxh.html?{0}")]
	Random(RandomQuery),
}

impl Url<'_> {
	pub fn request(&self) -> Result<Request> {
		let request = Request::get(self.to_string())?
			.header(
				"User-Agent",
				"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
				 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15",
			)
			.header("Referer", "https://boylove.cc/")
			.header("Accept-Language", "zh-TW,zh;q=0.9,en;q=0.8")
			.header("Accept", "text/html,application/xhtml+xml,*/*;q=0.8");
		Ok(request)
	}

	pub fn daily_update(day_of_week: &str, page: i32) -> Self {
		let query = DailyUpdateQuery::new(day_of_week, page);
		Self::DailyUpdate(query)
	}

	pub fn random() -> Self {
		let query = RandomQuery::new();
		Self::Random(query)
	}
}

impl<'a> Url<'a> {
	pub const fn manga(key: &'a str) -> Self {
		Self::Manga { key }
	}

	pub const fn chapter_list(manga_key: &'a str) -> Self {
		Self::ChapterList { manga_key }
	}

	pub const fn chapter(key: &'a str) -> Self {
		Self::Chapter { key }
	}

	pub fn from_query_or_filters(
		query: Option<&str>,
		page: i32,
		filters: &'a [FilterValue],
	) -> Result<Self> {
		if let Some(keyword) = query {
			let search_query = SearchQuery::new(keyword, page);
			return Ok(Self::Search(search_query));
		}

		let mut tags = Tags::default();
		let mut status = "2";
		let mut sort_by = Sort::default();
		let mut content_rating = "0";
		let mut view_permission = "2";

		for filter in filters {
			#[expect(clippy::match_wildcard_for_single_variants)]
			match filter {
				FilterValue::Text { id, value } => match id.as_str() {
					"author" => {
						let search_query = SearchQuery::new(value, page);
						return Ok(Self::Search(search_query));
					}
					_ => bail!("Invalid text filter ID: `{id}`"),
				},

				FilterValue::Sort { id, index, .. } => match id.as_str() {
					"排序方式" => sort_by = Sort::from_repr(*index).unwrap_or_default(),
					_ => bail!("Invalid sort filter ID: `{id}`"),
				},

				FilterValue::Select { id, value } => match id.as_str() {
					"閱覽權限" => view_permission = value,
					"連載狀態" => status = value,
					"內容分級" => content_rating = value,
					"genre" => {
						let search_query = SearchQuery::new(value, page);
						return Ok(Self::Search(search_query));
					}
					_ => bail!("Invalid select filter ID: `{id}`"),
				},

				FilterValue::MultiSelect { id, included, .. } => match id.as_str() {
					"標籤" => tags.0 = included,
					_ => bail!("Invalid multi-select filter ID: `{id}`"),
				},

				_ => bail!("Invalid filter: `{filter:?}`"),
			}
		}

		Ok(Self::Filters {
			tags,
			status,
			sort_by,
			page,
			content_rating,
			view_permission,
		})
	}

	pub fn listing(listing: &'a str, page: i32) -> Self {
		let offset_page = OffsetPage::new(page);
		Self::Listing(listing, offset_page)
	}
}

impl From<Url<'_>> for String {
	fn from(url: Url<'_>) -> Self {
		url.to_string()
	}
}

#[derive(Display)]
pub enum Charset {
	#[strum(to_string = "S")]
	Simplified,
	#[strum(to_string = "T")]
	Traditional,
}

impl Charset {
	pub fn from_settings() -> Result<Self> {
		let is_traditional_chinese = defaults_get("isTraditionalChinese")
			.ok_or_else(|| error!("Default does not exist for key: `isTraditionalChinese`"))?;
		let charset = if is_traditional_chinese {
			Self::Traditional
		} else {
			Self::Simplified
		};
		Ok(charset)
	}
}

#[derive(Display, Default, FromRepr)]
#[repr(i32)]
pub enum Sort {
	#[strum(to_string = "0")]
	Popularity,
	#[default]
	#[strum(to_string = "1")]
	LastUpdated,
}

#[derive(Default)]
pub struct Tags<'a>(&'a [String]);

impl Display for Tags<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let tags = if self.0.is_empty() {
			"0".into()
		} else {
			self.0
				.iter()
				.map(encode_uri_component)
				.collect::<Vec<_>>()
				.join("+")
		};
		write!(f, "{tags}")
	}
}

pub struct SearchQuery(QueryParameters);

impl SearchQuery {
	fn new(keyword: &str, page: i32) -> Self {
		let mut query = QueryParameters::new();
		query.push("keyword", Some(keyword));
		query.push_encoded("type", Some("1"));
		query.push_encoded("pageNo", Some(&page.to_string()));

		Self(query)
	}
}

impl Display for SearchQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

pub struct DailyUpdateQuery(QueryParameters);

impl DailyUpdateQuery {
	fn new(day_of_week: &str, page: i32) -> Self {
		let mut query = QueryParameters::new();
		query.push_encoded("widx", Some(day_of_week));
		query.push_encoded("limit", Some("18"));

		let offset_page = OffsetPage::new(page).to_string();
		query.push_encoded("page", Some(&offset_page));

		query.push_encoded("lastpage", Some("0"));

		Self(query)
	}
}

impl Display for DailyUpdateQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

pub struct OffsetPage(i32);

impl OffsetPage {
	fn new(page: i32) -> Self {
		let offset_page = page
			.checked_sub(1)
			.filter(|offset_page| *offset_page >= 0)
			.unwrap_or(0);
		Self(offset_page)
	}
}

impl Display for OffsetPage {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

pub struct RandomQuery(QueryParameters);

impl RandomQuery {
	fn new() -> Self {
		let mut query = QueryParameters::new();
		query.push_encoded("limit", Some("5"));
		query.push_encoded("type", Some("1"));

		Self(query)
	}
}

impl Display for RandomQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

#[cfg(test)]
mod test;
