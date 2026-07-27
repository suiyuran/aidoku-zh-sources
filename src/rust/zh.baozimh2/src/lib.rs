#![no_std]
extern crate alloc;

use aidoku::{
	error::{AidokuError, AidokuErrorKind, Result},
	helpers::uri::encode_uri,
	prelude::*,
	std::{
		defaults::defaults_get,
		json,
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://baozimh.org";
const API_URL: &str = "https://v2.apikk.top";
const USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) \
	AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Mobile/15E148 Safari/604.1";
const BASE64_STD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const BASE64_CUSTOM: &[u8] = b"_-9876543210abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const FILTER_CATEGORY: [&str; 35] = [
	"",
	"cn",
	"kr",
	"jp",
	"fuchou",
	"gufeng",
	"qihuan",
	"nixi",
	"lianai",
	"yineng",
	"zhaixiang",
	"chuanyue",
	"rexue",
	"chunai",
	"xitong",
	"chongsheng",
	"maoxian",
	"lingyi",
	"danvzhu",
	"juqing",
	"lianai",
	"xuanhuan",
	"nvshen",
	"kehuan",
	"mohuan",
	"tuili",
	"lieqi",
	"zhiyu",
	"dushi",
	"yixing",
	"qingchun",
	"mori",
	"xuanyi",
	"xiuxian",
	"zhandou",
];

fn handle_cover_url(url: String) -> String {
	let url = if let Some(value) = url.split("url=").nth(1) {
		value.split('&').next().unwrap_or(value).to_string()
	} else {
		url
	};
	let url = url
		.replace("%3A", ":")
		.replace("%3a", ":")
		.replace("%2F", "/")
		.replace("%2f", "/")
		.replace("%3F", "?")
		.replace("%3f", "?")
		.replace("%26", "&")
		.replace("%3D", "=")
		.replace("%3d", "=");

	if url.starts_with("//") {
		format!("https:{}", url)
	} else if url.starts_with('/') {
		format!("{}{}", WWW_URL, url)
	} else {
		url
	}
}

fn first_image_url(data_src: String, data_original: String, src: String) -> String {
	[data_src, data_original, src]
		.into_iter()
		.find(|value| !value.is_empty())
		.unwrap_or_default()
}

fn json_error<T>() -> Result<T> {
	Err(AidokuError {
		reason: AidokuErrorKind::JsonParseError,
	})
}

fn image_host(line: i64) -> String {
	let route = defaults_get("imageRoute")
		.ok()
		.and_then(|value| value.as_string().ok())
		.map(|value| value.read())
		.unwrap_or_else(|| String::from("c"));
	let line = if line == 2 { 2 } else { 3 };
	match route.as_str() {
		"t" => format!("https://t-nd{}-1.6wm.top", line),
		"g" if line == 2 => String::from("https://f40-1-4.g-mh.online"),
		"g" => String::from("https://t40-1-4.g-mh.online"),
		_ => format!("https://c-nd{}-1.6wm.top", line),
	}
}

// baozimh.org shares godamh.com's chapter backend and encoded image payload.
fn decode_chapter_images(encoded: String) -> Result<Vec<String>> {
	let input = encoded.as_bytes();
	if input.len() < 10 || !input.starts_with(b"J7r") || !input.ends_with(b"nQ") {
		return json_error();
	}

	let body = &input[3..input.len() - 2];
	let payload_len = match body.len().checked_sub(5) {
		Some(len) => len,
		None => return json_error(),
	};
	let part3_len = payload_len / 3;
	let part1_len = (payload_len - part3_len) / 2;
	let part2_len = payload_len - part3_len - part1_len;
	let marker1 = part1_len;
	let part2_start = marker1 + 2;
	let marker2 = part2_start + part2_len;
	let part3_start = marker2 + 3;

	if body.get(marker1..part2_start) != Some(b"kD")
		|| body.get(marker2..part3_start) != Some(b"W4s")
		|| body.len() - part3_start != part3_len
	{
		return json_error();
	}

	let mut normalized = Vec::with_capacity(payload_len);
	normalized.extend_from_slice(&body[part3_start..]);
	normalized.extend_from_slice(&body[..part1_len]);
	normalized.extend_from_slice(&body[part2_start..marker2]);

	for (index, chunk) in normalized.chunks_mut(7).enumerate() {
		if index % 2 == 1 {
			chunk.reverse();
		}
	}

	for byte in &mut normalized {
		let alphabet_index = match BASE64_CUSTOM.iter().position(|item| item == byte) {
			Some(index) => index,
			None => return json_error(),
		};
		*byte = BASE64_STD[alphabet_index];
	}

	let mut decoded = Vec::with_capacity(normalized.len() * 3 / 4);
	let mut buffer = 0u32;
	let mut bits = 0u8;
	for byte in normalized {
		let value = match byte {
			b'A'..=b'Z' => byte - b'A',
			b'a'..=b'z' => byte - b'a' + 26,
			b'0'..=b'9' => byte - b'0' + 52,
			b'-' => 62,
			b'_' => 63,
			_ => return json_error(),
		};
		buffer = (buffer << 6) | value as u32;
		bits += 6;
		if bits >= 8 {
			bits -= 8;
			decoded.push((buffer >> bits) as u8);
			buffer &= (1 << bits) - 1;
		}
	}

	let list = json::parse(decoded)?.as_array()?;
	let mut images = Vec::new();
	for item in list {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		if let Ok(url) = item.get("url").as_string() {
			images.push(url.read());
		}
	}
	if images.is_empty() {
		return json_error();
	}
	Ok(images)
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut category = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类型" => {
						category = FILTER_CATEGORY[index].to_string();
					}
					_ => continue,
				}
			}
			_ => continue,
		}
	}

	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	let url = if query.is_empty() {
		let caregory_str = if category.is_empty() {
			String::from("manga")
		} else if category.len() <= 2 {
			format!("manga-genre/{}", category)
		} else {
			format!("manga-tag/{}", category)
		};
		format!("{}/{}/page/{}", WWW_URL, caregory_str, page)
	} else {
		format!("{}/s/{}?page={}", WWW_URL, encode_uri(query), page)
	};
	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	for item in html.select(".pb-2>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let image = item.select("div>img");
		let cover = handle_cover_url(first_image_url(
			image.attr("data-src").read(),
			image.attr("data-original").read(),
			image.attr("src").read(),
		));
		let title = item.select("div>h3").text().read();
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

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut list = String::new();

	match listing.name.as_str() {
		"人气推荐" => {
			list.push_str("hots");
		}
		"热门更新" => {
			list.push_str("dayup");
		}
		"最新上架" => {
			list.push_str("newss");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/{}/page/{}", WWW_URL, list, page);
	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;
	let has_more = true;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in html.select(".pb-2>a").array() {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.attr("href")
			.read()
			.split("/")
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let image = item.select("div>img");
		let cover = handle_cover_url(first_image_url(
			image.attr("data-src").read(),
			image.attr("data-original").read(),
			image.attr("src").read(),
		));
		let title = item.select("div>h3").text().read();
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
	let ids = id.split("/").collect::<Vec<&str>>();
	let url = format!("{}/manga/{}", WWW_URL, ids[0]);
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;
	let mid = html.select("#mangachapters").attr("data-mid").read();
	let cover = handle_cover_url(
		html.select("meta[property='og:image']")
			.attr("content")
			.read(),
	);
	let title = html
		.select("title")
		.text()
		.read()
		.replace("-🌈️包子漫畫", "");
	let author = html
		.select("a[href*=author]>span")
		.array()
		.map(|a| a.as_node().unwrap().text().read().replace(",", ""))
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select(".text-medium.my-unit-md").text().read();
	let categories = html
		.select(".py-1>a:not([href*=author])>span")
		.array()
		.map(|a| {
			a.as_node()
				.unwrap()
				.text()
				.read()
				.replace(",", "")
				.replace("热门漫画", "")
				.replace("#", "")
				.replace("热门推荐", "")
				.trim()
				.to_string()
		})
		.filter(|a| !a.is_empty())
		.collect::<Vec<String>>();
	let status = MangaStatus::Ongoing;
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Scroll;

	Ok(Manga {
		id: format!("{}/{}", ids[0], mid),
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
	let ids = id.split("/").collect::<Vec<&str>>();
	let url = format!("{}/api/v2/manga/get?mid={}&mode=all", API_URL, ids[1]);
	let json = Request::new(url.clone(), HttpMethod::Get)
		.header("Origin", &WWW_URL)
		.header("Referer", &WWW_URL)
		.header("User-Agent", USER_AGENT)
		.header("Accept", "application/json")
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("chapters").as_array()?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let attributes = item.get("attributes").as_object()?;
		let id = item.get("id").as_int()?.to_string();
		let title = attributes.get("title").as_string()?.read();
		let slug = attributes.get("slug").as_string()?.read();
		let url = format!("{}/manga/{}/{}", WWW_URL, ids[0], slug);
		let chapter = (index + 1) as f32;
		chapters.push(Chapter {
			id,
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
	let ids = manga_id.split("/").collect::<Vec<&str>>();
	let url = format!(
		"{}/api/v2/chapter/getinfo?m={}&c={}",
		API_URL,
		ids[1],
		chapter_id.clone()
	);
	let json = Request::new(url.clone(), HttpMethod::Get)
		.header("Origin", &WWW_URL)
		.header("Referer", &WWW_URL)
		.header("User-Agent", USER_AGENT)
		.header("Accept", "application/json")
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let info = data.get("info").as_object()?;
	let images = info.get("images").as_object()?;
	let line = images.get("line").as_int().unwrap_or(3);
	let list = decode_chapter_images(images.get("images").as_string()?.read())?;
	let image_host = image_host(line);
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.into_iter().enumerate() {
		let index = index as i32;
		let url = if item.starts_with("http://") || item.starts_with("https://") {
			item
		} else {
			format!("{}{}", image_host, item)
		};
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
	request
		.header("Referer", &WWW_URL)
		.header("User-Agent", USER_AGENT)
		.header(
			"Accept",
			"image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8",
		)
		.header("Accept-Language", "zh-TW,zh;q=0.9,en;q=0.8");
}
