use aidoku::{
	FilterValue, Result,
	alloc::{String, format, string::ToString},
	bail, error,
	helpers::uri::{QueryParameters, encode_uri_component},
	imports::net::{HttpMethod, Request},
};
use alloc::vec::Vec;

use crate::net::Url::SearchOrFilter;
use core::fmt::{Display, Formatter, Result as FmtResult};
use strum::{Display, EnumIs};

const API_URL: &str = "https://www.zerobyw33.com";

#[derive(Display, EnumIs)]
pub enum Url<'a> {
	#[strum(to_string = "/pc/pc/?{0}")]
	SearchOrFilter(SearchOrFilterQuery),
	#[strum(to_string = "/pc/details/?kuid={key}")]
	Manga { key: &'a str },
	#[strum(to_string = "/pc/view/index.php?zjid={key}")]
	Chapter { key: &'a str },
	#[strum(to_string = "/member.php?mod=logging&action=login")]
	Login,
	#[strum(to_string = "/pc/pc/")]
	Home,
	#[strum(to_string = "{key}")]
	Logout { key: &'a str },
}

impl Url<'_> {
	pub fn to_string(&self) -> Result<String> {
		let base_url = API_URL;
		Ok(format!("{base_url}{self}"))
	}

	pub fn request(&self, method: HttpMethod) -> Result<Request> {
		let url = self.to_string()?;
		let request = Request::new(url, method)?.header(
			"User-Agent",
			"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
			 AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0.1 Safari/605.1.15",
		);
		Ok(request)
	}

	pub fn from_query_or_filters(
		query: Option<&str>,
		page: i32,
		filters: &[FilterValue],
	) -> Result<Self> {
		let mut category_id = "";
		let mut jindu = "";
		let mut shuxing = "";
		let mut order = "addtime";
		let mut dir = "desc";

		for filter in filters {
			match filter {
				FilterValue::Select { id, value } => match id.as_str() {
					"分类" => category_id = value,
					"进度" => jindu = value,
					"语言" => shuxing = value,
					_ => (),
				},
				FilterValue::Sort {
					id,
					index,
					ascending,
				} => match id.as_str() {
					"排序" => {
						dir = if *ascending { "asc" } else { "desc" };
						match index {
							0 => order = "addtime",
							1 => order = "views",
							2 => order = "favores",
							_ => bail!("Invalid index"),
						}
					}
					_ => bail!("Invalid sort filter id:`{id}`"),
				},

				_ => bail!("Invalid filter:`{filter:?}`"),
			}
		}

		let query = SearchOrFilterQuery::new(query, category_id, jindu, shuxing, order, dir, page);
		Ok(SearchOrFilter(query))
	}
}

impl<'a> Url<'a> {
	pub const fn manga(key: &'a str) -> Self {
		Self::Manga { key }
	}
	pub const fn chapter(key: &'a str) -> Self {
		Self::Chapter { key }
	}
	pub const fn login() -> Self {
		Self::Login
	}
	pub const fn home() -> Self {
		Self::Home
	}
	pub const fn logout(key: &'a str) -> Self {
		Self::Logout { key }
	}
}

pub struct SearchOrFilterQuery(QueryParameters);

impl SearchOrFilterQuery {
	fn new(
		keyword: Option<&str>,
		category_id: &str,
		jindu: &str,
		shuxing: &str,
		order: &str,
		dir: &str,
		page: i32,
	) -> Self {
		let mut q = QueryParameters::new();
		if let Some(keyword) = keyword {
			q.push("keyword", Some(keyword));
		}
		if !category_id.is_empty() {
			q.push_encoded("category_id", Some(category_id));
		}
		if !jindu.is_empty() {
			q.push_encoded("jindu", Some(jindu));
		}
		if !shuxing.is_empty() {
			q.push("shuxing", Some(shuxing));
		}
		q.push_encoded("order", Some(order));
		q.push_encoded("dir", Some(dir));
		q.push_encoded("page", Some(&page.to_string()));
		Self(q)
	}
}

impl Display for SearchOrFilterQuery {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.0)
	}
}

pub fn login(username: &str, password: &str) -> Result<bool> {
	let home_doc = Url::home().request(HttpMethod::Get)?.html()?;
	if let Some(logout_elem) = home_doc.select_first("a.user-logout-btn")
		&& let Some(logout_href) = logout_elem.attr("href")
	{
		Url::logout(&logout_href).request(HttpMethod::Get)?.send()?;
	}

	let login_doc = Url::login().request(HttpMethod::Get)?.html()?;

	let formhash = login_doc
		.select_first("input[name='formhash']")
		.ok_or_else(|| error!("formhash not found in form"))?
		.attr("value")
		.ok_or_else(|| error!("No formhash found"))?
		.to_string();

	let form = login_doc
		.select("form[action*='logging&action=login']")
		.ok_or_else(|| error!("formaction not found in form"))?
		.first()
		.ok_or_else(|| error!("No form action found"))?;
	let action = form
		.attr("action")
		.ok_or_else(|| error!("Action not found"))?
		.to_string();

	let post_url = format!("{}/{}", API_URL, action);

	let params = [
		("formhash", formhash),
		("referer", format!("{}/./", API_URL)),
		("loginfield", "username".to_string()),
		("username", username.to_string()),
		("password", password.to_string()),
		("cookietime", "2592000".to_string()),
		("loginsubmit", "true".to_string()),
		("questionid", "0".to_string()),
		("answer", "".to_string()),
	];

	let body = params
		.iter()
		.map(|(k, v)| format!("{}={}", k, encode_uri_component(v)))
		.collect::<Vec<_>>()
		.join("&");

	let mut request = Request::new(post_url, HttpMethod::Post)?;
	request.set_header("Content-Type", "application/x-www-form-urlencoded");
	request.set_body(body.as_bytes());

	let response = request.send()?;

	let text = response.get_string()?;
	if text.contains("欢迎您回来") {
		return Ok(true);
	}
	if text.contains("登录失败") {
		return Ok(false);
	}
	Ok(false)
}
