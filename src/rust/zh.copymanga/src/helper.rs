use aidoku::{
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		ObjectRef, String,
	},
};

use crate::crypto;

const KEY: &[u8; 16] = b"xxxmanga.woo.key";

const WWW_URL: &str = "https://www.copymanga.tv";
const API_URL: &str = "https://api.copymanga.tv/api/v3";

pub fn decrypt(text: String) -> String {
	let text = text.as_bytes();
	let iv = &text[..16];
	let cipher = &text[16..];
	let cipher = hex::decode(cipher).unwrap();
	let pt = crypto::decrypt(&cipher, KEY, iv).unwrap();
	String::from_utf8_lossy(&pt).replace("", "")
}

pub fn get_json(url: String) -> ObjectRef {
	Request::new(url, HttpMethod::Get)
		.header("version", "2.0.7")
		.header("platform", "3")
		.header("region", "1")
		.json()
		.unwrap()
		.as_object()
		.unwrap()
}

pub fn gen_explore_url(theme: String, top: String, ordering: String, page: i32) -> String {
	format!(
		"{}/comics?theme={}&top={}&ordering={}&limit={}&offset={}",
		API_URL,
		theme,
		top,
		ordering,
		50,
		(page - 1) * 50,
	)
}

pub fn gen_search_url(query: String, page: i32) -> String {
	format!(
		"{}/search/comic?q={}&q_type={}&limit={}&offset={}",
		API_URL,
		encode_uri(query),
		"",
		20,
		(page - 1) * 20
	)
}

pub fn gen_rank_url(date_type: String, page: i32) -> String {
	format!(
		"{}/ranks?date_type={}&limit={}&offset={}",
		API_URL,
		date_type,
		30,
		(page - 1) * 30,
	)
}

pub fn gen_recs_url(page: i32) -> String {
	format!(
		"{}/recs?pos={}&limit={}&offset={}",
		API_URL,
		"3200102",
		30,
		(page - 1) * 30,
	)
}

pub fn gen_newest_url(page: i32) -> String {
	format!(
		"{}/update/newest?limit={}&offset={}",
		API_URL,
		30,
		(page - 1) * 30,
	)
}

pub fn gen_manga_url(id: String) -> String {
	format!("{}/comic/{}", WWW_URL, id)
}

pub fn gen_manga_details_url(id: String) -> String {
	format!("{}/comic2/{}", API_URL, id)
}

pub fn gen_chapter_list_url(id: String) -> String {
	format!("{}/comicdetail/{}/chapters", WWW_URL, id)
}

pub fn gen_chapter_url(manga_id: String, chapter_id: String) -> String {
	format!("{}/comic/{}/chapter/{}", WWW_URL, manga_id, chapter_id)
}

pub fn gen_page_list_url(manga_id: String, chapter_id: String) -> String {
	format!("{}/comic/{}/chapter/{}", API_URL, manga_id, chapter_id)
}
