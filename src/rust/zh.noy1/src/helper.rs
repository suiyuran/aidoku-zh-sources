use aidoku::{
	error::{AidokuError, AidokuErrorKind},
	prelude::*,
	std::{
		defaults::defaults_get,
		net::{HttpMethod, Request},
		String, ValueRef,
	},
};

const WWW_URL: &str = "https://noy1.top";
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

pub fn get_json(url: String, body: String) -> Result<ValueRef, AidokuError> {
	let session = defaults_get("session")?.as_string()?.read();

	if session.is_empty() {
		return Err(AidokuError {
			reason: AidokuErrorKind::DefaultNotFound,
		});
	}

	let request = Request::new(url, HttpMethod::Post)
		.header("Cookie", &format!("NOY_SESSION={}", session))
		.header("Content-Type", "application/x-www-form-urlencoded")
		.body(body.as_bytes());

	request.json()
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
