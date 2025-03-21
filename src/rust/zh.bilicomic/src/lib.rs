#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://www.bilicomic.net";
const UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1";

const FILTER_TAGID: [&str; 66] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
	"17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
	"33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
	"49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
	"65",
];
const FILTER_SORTID: [&str; 13] = [
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12",
];
const FILTER_RGROUPID: [&str; 6] = ["0", "1", "2", "3", "4", "5"];
const FILTER_ORDER: [&str; 10] = [
	"weekvisit",
	"monthvisit",
	"weekvote",
	"monthvote",
	"weekflower",
	"monthflower",
	"words",
	"goodnum",
	"lastupdate",
	"postdate",
];
const FILTER_ANIME: [&str; 3] = ["0", "1", "2"];
const FILTER_QUALITY: [&str; 3] = ["0", "1", "2"];
const FILTER_ISFULL: [&str; 3] = ["0", "1", "2"];
const FILTER_UPDATE: [&str; 5] = ["0", "1", "2", "3", "4"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut tagid = String::from("0");
	let mut sortid = String::from("0");
	let mut rgroupid = String::from("0");
	let mut order = String::from("lastupdate");
	let mut anime = String::from("0");
	let mut quality = String::from("0");
	let mut isfull = String::from("0");
	let mut update = String::from("0");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"作品主题" => {
						tagid = FILTER_TAGID[index].to_string();
					}
					"作品分类" => {
						sortid = FILTER_SORTID[index].to_string();
					}
					"文库地区" => {
						rgroupid = FILTER_RGROUPID[index].to_string();
					}
					"是否动画" => {
						anime = FILTER_ANIME[index].to_string();
					}
					"是否轻改" => {
						quality = FILTER_QUALITY[index].to_string();
					}
					"连载状态" => {
						isfull = FILTER_ISFULL[index].to_string();
					}
					"更新时间" => {
						update = FILTER_UPDATE[index].to_string();
					}
					_ => continue,
				}
			}
			FilterType::Sort => {
				let value = match filter.value.as_object() {
					Ok(value) => value,
					Err(_) => continue,
				};
				let index = value.get("index").as_int()? as usize;
				order = FILTER_ORDER[index].to_string();
			}
			_ => continue,
		}
	}

	let url = if query.is_empty() {
		format!(
			"{}/filter/{}_{}_{}_{}_{}_{}_{}_{}_{}_0.html",
			WWW_URL, order, tagid, isfull, anime, rgroupid, sortid, update, quality, page
		)
	} else {
		format!(
			"{}/search/{}_{}.html",
			WWW_URL,
			encode_uri(query.clone()),
			page
		)
	};
	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", UA)
		.html()?;
	let link = html.select("#pagelink");
	let has_more = if query.is_empty() {
		link.select("strong").text().read() != link.select(".last").text().read()
	} else {
		link.select(".next").attr("href").read() != "#"
	};
	let mut mangas: Vec<Manga> = Vec::new();

	let alternate_url = html.select("link[rel='alternate']").attr("href").read();

	if alternate_url.contains("detail") {
		let id = alternate_url
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = html.select(".book-cover").attr("src").read();
		let title = html.select("h1.book-title").text().read();

		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	} else {
		for item in html.select(".book-li>a").array() {
			let item = match item.as_node() {
				Ok(node) => node,
				Err(_) => continue,
			};
			let id = item
				.attr("href")
				.read()
				.split("/")
				.map(|a| a.to_string())
				.filter(|a| !a.is_empty())
				.collect::<Vec<String>>()
				.pop()
				.unwrap()
				.replace(".html", "");
			let cover = item.select(".book-cover>img").attr("data-src").read();
			let title = item.select(".book-title").text().read();
			mangas.push(Manga {
				id,
				cover,
				title,
				..Default::default()
			});
		}
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut name = String::new();

	match listing.name.as_str() {
		"月点击榜" => {
			name.push_str("monthvisit");
		}
		"周点击榜" => {
			name.push_str("weekvisit");
		}
		"月推荐榜" => {
			name.push_str("monthvote");
		}
		"周推荐榜" => {
			name.push_str("weekvote");
		}
		"月鲜花榜" => {
			name.push_str("monthflower");
		}
		"周鲜花榜" => {
			name.push_str("weekflower");
		}
		"月鸡蛋榜" => {
			name.push_str("monthegg");
		}
		"周鸡蛋榜" => {
			name.push_str("weekegg");
		}
		"最近更新" => {
			name.push_str("lastupdate");
		}
		"最新入库" => {
			name.push_str("postdate");
		}
		"收藏榜" => {
			name.push_str("goodnum");
		}
		"新书榜" => {
			name.push_str("newhot");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/top/{}/1.html", WWW_URL, name);
	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", UA)
		.html()?;
	let has_more = false;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".book-li>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let cover = item.select(".book-cover>img").attr("data-src").read();
		let title = item.select(".book-title").text().read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/detail/{}.html", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("User-Agent", UA)
		.html()?;
	let cover = html.select(".book-cover").attr("src").read();
	let title = html.select("h1.book-title").text().read();
	let author = html
		.select(".authorname,.illname")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".book-summary>content").text().read();
	let categories = html
		.select(".tag-small-group>.tag-small>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = match html
		.select(".book-layout-inline")
		.text()
		.read()
		.trim()
		.split("|")
		.map(|a| a.trim().to_string())
		.collect::<Vec<String>>()
		.first()
		.unwrap()
		.as_str()
	{
		"连载" => MangaStatus::Ongoing,
		"完结" => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	};
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Rtl;

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories,
		status,
		nsfw,
		viewer,
	})
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}/read/{}/catalog", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("User-Agent", UA)
		.html()?;
	let list = html.select(".catalog-volume .chapter-li-a").array();
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_node() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let chapter_id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.filter(|a| !a.is_empty())
			.collect::<Vec<String>>()
			.pop()
			.unwrap()
			.replace(".html", "");
		let title = item.select("span").text().read();
		let chapter = (index + 1) as f32;
		let url = format!(
			"{}/read/{}/{}.html",
			WWW_URL,
			id.clone(),
			chapter_id.clone()
		);
		chapters.push(Chapter {
			id: chapter_id,
			title,
			chapter,
			url,
			..Default::default()
		});
	}
	chapters.reverse();

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!(
		"{}/read/{}/{}.html",
		WWW_URL,
		manga_id.clone(),
		chapter_id.clone()
	);
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("User-Agent", UA)
		.header("Cookie", "night=0")
		.header("Accept-Language", "zh-CN,zh;q=0.9")
		.html()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html.select("#acontentz>img").array().enumerate() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.attr("data-src").read().trim().to_string();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request.header("User-Agent", UA).header("Referer", WWW_URL);
}
