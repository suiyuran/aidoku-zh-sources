use aidoku::{
	error::AidokuError,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, ValueRef,
	},
};

const WWW_URL: &str = "https://godamanga.com";
const ART_URL: &str = "https://gd.godamanga.art";
const API_URL: &str = "https://papi.mgsearcher.com/api";

pub fn gen_explore_url(list: String, category: String, page: i32) -> String {
	let mut url = String::new();

	if category.is_empty() {
		url.push_str(format!("{}/{}/", WWW_URL, list).as_str());
	} else {
		url.push_str(format!("{}/", WWW_URL).as_str());

		if category.len() > 2 {
			url.push_str(format!("manga-tag/{}", category).as_str());
		} else {
			url.push_str(format!("manga-genre/{}", category).as_str());
		};
	};

	if page > 1 {
		url.push_str(format!("page/{}", page).as_str());
	};

	url
}

pub fn search(query: String, page: i32) -> Result<ValueRef, AidokuError> {
	let url = String::from("https://go.mgsearcher.com/indexes/mangaStrapiPro/search");
	let body = format!(
		r#"{{
			"hitsPerPage": 30,
			"page": {},
			"q": "{}"
		}}"#,
		page, query
	);
	Request::new(url, HttpMethod::Post)
		.body(body.as_bytes())
		.header("Content-Type", "application/json")
		.header(
			"Authorization",
			"Bearer 9bdaaa44f0dd520da24298a02818944327b8280a79feb480302feda7c009264a",
		)
		.json()
}

pub fn gen_cover_url(url: String) -> String {
	format!("{}/{}", WWW_URL, url)
}

pub fn gen_manga_url(id: String) -> String {
	format!("{}/manga/{}", WWW_URL, id)
}

pub fn gen_chapter_list_url(id: String) -> String {
	format!("{}/chapterlist/{}", WWW_URL, id)
}

pub fn gen_page_list_url(manga_id: String, chapter_id: String) -> String {
	format!("{}/manga/{}/{}", ART_URL, manga_id, chapter_id)
}

pub fn gen_page_url(chapter_id: String) -> String {
	format!(
		"{}/chapters/{}?fields%5B0%5D=chapter_img&encodeValuesOnly=true",
		API_URL, chapter_id
	)
}
