use aidoku::{
	error::{AidokuError, AidokuErrorKind},
	helpers::substring::Substring,
	std::{
		defaults::{defaults_get, defaults_set},
		net::{HttpMethod, Request},
		String, StringRef, ValueRef,
	},
};
use alloc::{format, string::ToString};

pub const WWW_URL: &str = "https://noy1.top";
const PIC_URL: &str = "https://img.noy.asia";

pub fn explore(tag: String, sort: String, page: i32) -> Result<ValueRef, AidokuError> {
	let url = if tag.is_empty() {
		format!("{}/api/booklist_v2", WWW_URL)
	} else {
		format!("{}/api/search_v2", WWW_URL)
	};
	let body = if tag.is_empty() {
		format!("page={}", page)
	} else {
		format!("info={}&type=tag&sort={}&page={}", tag, sort, page)
	};

	get_json(url, body)
}

pub fn search(keyword: String, page: i32) -> Result<ValueRef, AidokuError> {
	let url = format!("{}/api/search_v2", WWW_URL);
	let body = format!("info={}&type=de&sort=bid&page={}", keyword, page);

	get_json(url, body)
}

pub fn rank(name: String, level: String, page: i32) -> Result<ValueRef, AidokuError> {
	let url = format!("{}/api/{}", WWW_URL, name);
	let body = if !level.is_empty() {
		format!("page={}&type={}", page, level)
	} else {
		format!("page={}", page)
	};

	get_json(url, body)
}

pub fn details(manga_id: String) -> Result<ValueRef, AidokuError> {
	let url = format!("{}/api/getbookinfo", WWW_URL);
	let body = format!("bid={}", manga_id);

	get_json(url, body)
}

pub fn gen_request(url: String, method: HttpMethod) -> Request {
	let session = defaults_get("session").unwrap().as_string().unwrap().read();
	let session = if !url.contains("login") && session.is_empty() {
		login().unwrap()
	} else {
		session
	};
	Request::new(url, method)
		.header("Content-Type", "application/x-www-form-urlencoded")
		.header("Cookie", &format!("NOY_SESSION={}", session))
}

pub fn login() -> Result<String, AidokuError> {
	let url = format!("{}/api/login", WWW_URL);
	let request = gen_request(url, HttpMethod::Post);
	let username = defaults_get("username")?.as_string()?.read();
	let password = defaults_get("password")?.as_string()?.read();

	if username.is_empty() || password.is_empty() {
		return Err(AidokuError {
			reason: AidokuErrorKind::DefaultNotFound,
		});
	}

	let body = format!("user={}&pass={}", username, password);
	let request = request.body(body.as_bytes());

	request.send();

	if request.status_code() != 200 {
		return Err(AidokuError {
			reason: AidokuErrorKind::DefaultNotFound,
		});
	}

	let cookie_header = request.get_header("set-cookie").unwrap().read();
	let session = cookie_header
		.substring_after("NOY_SESSION=")
		.unwrap()
		.substring_before(";")
		.unwrap();

	defaults_set("session", StringRef::from(session).0);

	Ok(session.to_string())
}

pub fn get_json(url: String, body: String) -> Result<ValueRef, AidokuError> {
	let request = gen_request(url, HttpMethod::Post).body(body.as_bytes());

	request.send();

	if request.status_code() == 401 {
		request
			.header("Cookie", &format!("NOY_SESSION={}", &login()?))
			.json()
	} else {
		request.json()
	}
}

pub fn gen_cover_url(manga_id: String) -> String {
	format!("{}/{}/m1.webp", PIC_URL, manga_id)
}

pub fn gen_manga_url(manga_id: String) -> String {
	format!("{}/#/book/{}", WWW_URL, manga_id)
}

pub fn gen_chapter_url(manga_id: String) -> String {
	format!("{}/#/read/{}", WWW_URL, manga_id)
}

pub fn gen_page_url(manga_id: String, page: i32) -> String {
	format!("{}/{}/{}.webp", PIC_URL, manga_id, page)
}
