use aidoku::{
	helpers::substring::Substring,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		ObjectRef, String,
	},
};
use alloc::string::ToString;

const WWW_URL: &str = "https://komiic.com";
const API_URL: &str = "https://komiic.com/api/query";

pub fn gen_manga_url(id: String) -> String {
	format!("{}/comic/{}", WWW_URL, id)
}

pub fn gen_chapter_url(manga_id: String, chapter_id: String) -> String {
	format!(
		"{}/comic/{}/chapter/{}/images/all",
		WWW_URL, manga_id, chapter_id
	)
}

pub fn gen_page_url(manga_id: String, chapter_id: String, page_id: String) -> String {
	format!(
		"{}/api/image/{}?mangaId={}&chapterId={}",
		WWW_URL, page_id, manga_id, chapter_id
	)
}

pub fn gen_referer(image_url: String) -> String {
	if image_url.starts_with(WWW_URL) {
		let query = image_url.substring_after("?").unwrap();
		let manga_id = query
			.substring_after("mangaId=")
			.unwrap()
			.substring_before("&")
			.unwrap()
			.to_string();
		let chapter_id = query.substring_after("chapterId=").unwrap().to_string();
		gen_chapter_url(manga_id, chapter_id)
	} else {
		WWW_URL.to_string()
	}
}

pub fn get_json(body: String) -> ObjectRef {
	Request::new(API_URL, HttpMethod::Post)
		.body(body.as_bytes())
		.header("Content-Type", "application/json")
		.json()
		.unwrap()
		.as_object()
		.unwrap()
}

pub fn gen_category_body_string(
	category: String,
	status: String,
	order_by: String,
	page: i32,
) -> String {
	let category_id = if category.is_empty() {
		"[]".to_string()
	} else {
		format!(r#"["{}"]"#, category)
	};
	format!(
		r#"{{
      "operationName": "comicByCategories",
      "query": "query comicByCategories($categoryId: [ID!]!, $pagination: Pagination!) {{\n  comicByCategories(categoryId: $categoryId, pagination: $pagination) {{\n    id\n    title\n    status\n    year\n    imageUrl\n    authors {{\n      id\n      name\n      __typename\n    }}\n    categories {{\n      id\n      name\n      __typename\n    }}\n    dateUpdated\n    monthViews\n    views\n    favoriteCount\n    lastBookUpdate\n    lastChapterUpdate\n    __typename\n  }}\n}}\n",
      "variables": {{
        "categoryId": {},
        "pagination": {{
            "asc": false,
            "limit": {},
            "offset": {},
            "orderBy": "{}",
						"status": "{}"
        }}
      }}
    }}"#,
		category_id,
		20,
		(page - 1) * 20,
		order_by,
		status,
	)
}

pub fn gen_recent_update_body_string(page: i32) -> String {
	format!(
		r#"{{
      "operationName": "recentUpdate",
    	"query": "query recentUpdate($pagination: Pagination!) {{\n  recentUpdate(pagination: $pagination) {{\n    id\n    title\n    status\n    year\n    imageUrl\n    authors {{\n      id\n      name\n      __typename\n    }}\n    categories {{\n      id\n      name\n      __typename\n    }}\n    dateUpdated\n    monthViews\n    views\n    favoriteCount\n    lastBookUpdate\n    lastChapterUpdate\n    __typename\n  }}\n}}\n",
    	"variables": {{
        "pagination": {{
            "asc": true,
            "limit": {},
            "offset": {},
            "orderBy": "DATE_UPDATED"
        }}
    	}}
    }}"#,
		20,
		(page - 1) * 20
	)
}

pub fn gen_hot_body_string(order_by: String, page: i32) -> String {
	format!(
		r#"{{
      "operationName": "hotComics",
    	"query": "query hotComics($pagination: Pagination!) {{\n  hotComics(pagination: $pagination) {{\n    id\n    title\n    status\n    year\n    imageUrl\n    authors {{\n      id\n      name\n      __typename\n    }}\n    categories {{\n      id\n      name\n      __typename\n    }}\n    dateUpdated\n    monthViews\n    views\n    favoriteCount\n    lastBookUpdate\n    lastChapterUpdate\n    __typename\n  }}\n}}\n",
    	"variables": {{
        "pagination": {{
            "asc": true,
            "limit": {},
            "offset": {},
            "orderBy": "{}",
            "status": ""
        }}
    	}}
    }}"#,
		20,
		(page - 1) * 20,
		order_by,
	)
}

pub fn gen_search_body_string(query: String) -> String {
	format!(
		r#"{{
      "operationName": "searchComicAndAuthorQuery",
      "query": "query searchComicAndAuthorQuery($keyword: String!) {{\n  searchComicsAndAuthors(keyword: $keyword) {{\n    comics {{\n      id\n      title\n      status\n      year\n      imageUrl\n      authors {{\n        id\n        name\n        __typename\n      }}\n      categories {{\n        id\n        name\n        __typename\n      }}\n      dateUpdated\n      monthViews\n      views\n      favoriteCount\n      lastBookUpdate\n      lastChapterUpdate\n      __typename\n    }}\n    authors {{\n      id\n      name\n      chName\n      enName\n      wikiLink\n      comicCount\n      views\n      __typename\n    }}\n    __typename\n  }}\n}}\n",
      "variables": {{
        "keyword": "{}"
      }}
    }}"#,
		query,
	)
}

pub fn gen_id_body_string(id: String) -> String {
	format!(
		r#"{{
      "operationName": "comicById",
      "query": "query comicById($comicId: ID!) {{\n  comicById(comicId: $comicId) {{\n    id\n    title\n    status\n    year\n    imageUrl\n    authors {{\n      id\n      name\n      __typename\n    }}\n    categories {{\n      id\n      name\n      __typename\n    }}\n    dateCreated\n    dateUpdated\n    views\n    favoriteCount\n    lastBookUpdate\n    lastChapterUpdate\n    __typename\n  }}\n}}\n",
      "variables": {{
        "comicId": "{}"
      }}
    }}"#,
		id
	)
}

pub fn gen_chapter_body_string(id: String) -> String {
	format!(
		r#"{{
      "operationName": "chapterByComicId",
      "query": "query chapterByComicId($comicId: ID!) {{\n  chaptersByComicId(comicId: $comicId) {{\n    id\n    serial\n    type\n    dateCreated\n    dateUpdated\n    size\n    __typename\n  }}\n}}\n",
      "variables": {{
        "comicId": "{}"
      }}
    }}"#,
		id
	)
}

pub fn gen_images_body_string(id: String) -> String {
	format!(
		r#"{{
      "operationName": "imagesByChapterId",
      "query": "query imagesByChapterId($chapterId: ID!) {{\n  imagesByChapterId(chapterId: $chapterId) {{\n    id\n    kid\n    height\n    width\n    __typename\n  }}\n}}\n",
      "variables": {{
        "chapterId": "{}"
      }}
    }}"#,
		id
	)
}
