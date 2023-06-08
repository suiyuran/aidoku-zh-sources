use aidoku::{
	error::AidokuError,
	helpers::{substring::Substring, uri::encode_uri},
	prelude::*,
	std::{
		current_date,
		defaults::{defaults_get, defaults_set},
		net::{HttpMethod, Request},
		String, ValueRef,
	},
};
use alloc::string::ToString;
use md5::compute;

use crate::crypto;

const KEY: &[u8; 63] = br"~d}$Q7$eIni=V)9\RK/P.RM4;9[7|@/CA}b~OW!3?EV`:<>M7pddUBL5n|0/*Cn";
const API_KEY: &str = "C69BAF41DA5ABD1FFEDC6D2FEA56B";

const WWW_URL: &str = "https://manhuabika.com";
const API_URL: &str = "https://picaapi.picacomic.com";

pub fn gen_time() -> String {
	(current_date() as i64).to_string()
}

pub fn gen_nonce() -> String {
	format!("{:x}", compute(gen_time()))
}

pub fn gen_signature(url: &str, time: &str, nonce: &str, method: &str) -> String {
	let url = url.substring_after(&format!("{}/", API_URL)).unwrap();
	let text = format!("{}{}{}{}{}", url, time, nonce, method, API_KEY).to_ascii_lowercase();
	crypto::encrypt(text.as_bytes(), KEY)
}

pub fn gen_request(url: String, method: HttpMethod) -> Request {
	let time = gen_time();
	let nonce = gen_nonce();
	let signature = gen_signature(&url, &time, &nonce, &format!("{:?}", method));
	let token = defaults_get("token").unwrap().as_string().unwrap().read();
	let authorization = if token.is_empty() { login() } else { token };
	Request::new(url, method)
		.header("api-key", API_KEY)
		.header("app-build-version", "45")
		.header("app-channel", "1")
		.header("app-platform", "android")
		.header("app-uuid", "defaultUuid")
		.header("app-version", "2.2.1.3.3.4")
		.header("image-quality", "original")
		.header("time", &time)
		.header("nonce", &nonce)
		.header("signature", &signature)
		.header("Accept", "application/vnd.picacomic.com.v1+json")
		.header("Authorization", &authorization)
		.header("Content-Type", "application/json; charset=UTF-8")
		.header("User-Agent", "okhttp/3.8.1")
}

pub fn login() -> String {
	let request = gen_request(gen_login_url(), HttpMethod::Post).header("Authorization", "");
	let username = defaults_get("username")
		.unwrap()
		.as_string()
		.unwrap()
		.read();
	let password = defaults_get("password")
		.unwrap()
		.as_string()
		.unwrap()
		.read();
	let body = format!(
		r#"{{
			"email": "{}",
			"password": "{}"
		}}"#,
		username, password
	);
	let json = request.body(body.as_bytes()).json().unwrap();
	let data = json.as_object().unwrap();
	let data = data.get("data").as_object().unwrap();
	let token_ref = data.get("token");

	defaults_set("token", token_ref.clone());

	token_ref.as_string().unwrap().read()
}

pub fn search(keyword: String, page: i32) -> Result<ValueRef, AidokuError> {
	let url = gen_search_url(page);
	let body = format!(
		r#"{{
			"keyword": "{}",
			"sort": "dd"
		}}"#,
		keyword,
	);
	let request = gen_request(url, HttpMethod::Post).body(body.as_bytes());

	request.send();

	if request.status_code() == 401 {
		request.header("Authorization", &login()).json()
	} else {
		request.json()
	}
}

pub fn gen_login_url() -> String {
	format!("{}/{}", API_URL, "auth/sign-in")
}

pub fn gen_explore_url(category: String, sort: String, page: i32) -> String {
	if category.is_empty() {
		format!("{}/comics?page={}&s={}", API_URL, page, sort,)
	} else {
		format!(
			"{}/comics?page={}&c={}&s={}",
			API_URL,
			page,
			encode_uri(category),
			sort,
		)
	}
}

pub fn gen_rank_url(time: String) -> String {
	format!("{}/comics/leaderboard?tt={}&ct=VC", API_URL, time)
}

pub fn gen_random_url() -> String {
	format!("{}/comics/random", API_URL)
}

pub fn gen_search_url(page: i32) -> String {
	format!("{}/comics/advanced-search?page={}&s=dd", API_URL, page)
}

pub fn gen_manga_url(id: String) -> String {
	format!("{}/pcomicview/?cid={}", WWW_URL, id)
}

pub fn gen_manga_details_url(id: String) -> String {
	format!("{}/comics/{}", API_URL, id)
}

pub fn gen_chapter_list_url(id: String, page: i32) -> String {
	format!("{}/comics/{}/eps?page={}", API_URL, id, page)
}

pub fn gen_chapter_url(manga_id: String, chapter_id: String) -> String {
	format!(
		"{}/pchapter/?cid={}&chapter={}",
		WWW_URL, manga_id, chapter_id
	)
}

pub fn gen_page_list_url(manga_id: String, chapter_id: String, page: i32) -> String {
	format!(
		"{}/comics/{}/order/{}/pages?page={}",
		API_URL, manga_id, chapter_id, page
	)
}

pub fn get_json(url: String) -> Result<ValueRef, AidokuError> {
	let request = gen_request(url, HttpMethod::Get);

	request.send();

	if request.status_code() == 401 {
		request.header("Authorization", &login()).json()
	} else {
		request.json()
	}
}
