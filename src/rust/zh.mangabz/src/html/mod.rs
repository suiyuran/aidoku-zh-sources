use crate::{BASE_URL, USER_AGENT};
use aidoku::{
	Chapter, Manga, MangaPageResult, MangaStatus, Page, PageContent, Result,
	alloc::{String, Vec, string::ToString as _, vec},
	imports::{html::Document, net::Request},
	prelude::*,
};
use regex::Regex;

fn extract_chapter_number(title: &str) -> Option<f32> {
	let re =
		Regex::new(r"(?:第\s*)(\d+(?:\.\d+)?)|(\d+(?:\.\d+)?)\s*(?:话|話|章|回|卷|册|冊)").ok()?;
	if let Some(captures) = re.captures(title) {
		let num_match = captures.get(1).or_else(|| captures.get(2));
		if let Some(num_match) = num_match
			&& let Ok(num) = num_match.as_str().parse::<f32>()
		{
			return Some(num);
		}
	}
	None
}

pub trait MangaPage {
	fn manga_page_result(&self) -> Result<MangaPageResult>;
	fn update_details(&self, manga: &mut Manga) -> Result<()>;
}

pub trait ChapterPage {
	fn chapters(&self) -> Result<Vec<Chapter>>;
}

impl MangaPage for Document {
	fn manga_page_result(&self) -> Result<MangaPageResult> {
		let mut mangas: Vec<Manga> = Vec::new();

		let items = self
			.select("div.mh-item")
			.ok_or_else(|| error!("No manga items found"))?;
		for item in items {
			let title_node = item
				.select_first("h2.title > a")
				.ok_or_else(|| error!("No title link found"))?;

			let href = title_node
				.attr("href")
				.ok_or_else(|| error!("No href attribute"))?;
			let id = href.replace('/', "").replace("bz", "");
			let cover = item
				.select_first("img.mh-cover")
				.and_then(|img| img.attr("src"))
				.unwrap_or_default();
			let title = title_node.attr("title").unwrap_or_default();
			let url = format!("{}{}bz/", BASE_URL, id);

			let status_str = item
				.select_first("span")
				.and_then(|span| span.text())
				.unwrap_or_default();
			let status = match status_str.as_str() {
				"最新" => MangaStatus::Ongoing,
				"完結" => MangaStatus::Completed,
				_ => MangaStatus::Unknown,
			};

			mangas.push(Manga {
				key: id,
				cover: Some(cover),
				title,
				url: Some(url),
				status,
				..Default::default()
			});
		}

		let has_more = self
			.select("div.page-pagination a:contains(>)")
			.map(|elements| elements.count() > 0)
			.unwrap_or(false);

		Ok(MangaPageResult {
			entries: mangas,
			has_next_page: has_more,
		})
	}

	fn update_details(&self, manga: &mut Manga) -> Result<()> {
		manga.cover = self
			.select_first("img.detail-info-cover")
			.and_then(|img| img.attr("src"));
		manga.title = self
			.select("p.detail-info-title")
			.and_then(|elem| elem.text())
			.unwrap_or_default();

		let mut artists: Vec<String> = Vec::new();
		if let Some(manga_info_elem) = self.select("p.detail-info-tip")
			&& let Some(author_elements) = manga_info_elem.select("span:contains(作者) > a")
		{
			for item in author_elements {
				if let Some(artist_str) = item.text() {
					artists.push(artist_str);
				}
			}
		}
		manga.authors = Some(artists.clone());
		manga.artists = Some(artists);

		manga.description = self
			.select("p.detail-info-content")
			.and_then(|elem| elem.text());

		let status_str = self
			.select("p.detail-info-tip")
			.and_then(|elem| elem.select("span:contains(狀態) > span"))
			.and_then(|elem| elem.text())
			.unwrap_or_default();
		manga.status = match status_str.as_str() {
			"連載中" => MangaStatus::Ongoing,
			"已完結" => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};

		let mut categories: Vec<String> = Vec::new();
		if let Some(manga_info_elem) = self.select("p.detail-info-tip")
			&& let Some(item_elements) = manga_info_elem.select("span.item")
		{
			for item in item_elements {
				if let Some(genre) = item.text() {
					categories.push(genre);
				}
			}
		}
		manga.tags = Some(categories);

		Ok(())
	}
}

impl ChapterPage for Document {
	fn chapters(&self) -> Result<Vec<Chapter>> {
		let mut chapters: Vec<Chapter> = Vec::new();
		let mut index = 1.0;

		let items = self
			.select("a.detail-list-form-item")
			.ok_or_else(|| error!("No chapter items found"))?;
		for item in items.rev() {
			let href = item.attr("href").ok_or_else(|| error!("No href found"))?;
			let id = href.replace(['/', 'm'], "");
			let title = item.text().ok_or_else(|| error!("No text found"))?;
			let clean_title = title.split('（').next().unwrap_or(&title).trim();
			let chapter_or_volume = extract_chapter_number(&title).unwrap_or(index);
			let (ch, vo) = if clean_title.ends_with('卷') {
				(-1.0, chapter_or_volume)
			} else {
				(chapter_or_volume, -1.0)
			};
			let url = format!("{}m{}/", BASE_URL, id);

			let scanlator = if vo > -1.0 {
				"单行本".to_string()
			} else {
				"默认".to_string()
			};

			chapters.push(Chapter {
				key: id,
				title: Some(title),
				volume_number: if vo >= 0.0 { Some(vo) } else { None },
				chapter_number: if ch >= 0.0 { Some(ch) } else { None },
				url: Some(url),
				scanlators: Some(vec![scanlator]),
				..Default::default()
			});
			index += 1.0;
		}

		chapters.reverse();

		Ok(chapters)
	}
}

pub fn get_page_list(url: String) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();
	let mut page = 1;
	let mut last_url = String::new();

	loop {
		let content = Request::get(format!("{}{}", url, page))?
			.header("Referer", BASE_URL)
			.header("User-Agent", USER_AGENT)
			.string()?;
		let urls = decode(content);
		for url in urls.clone() {
			if url == last_url {
				break;
			}
			last_url = url.clone();

			pages.push(Page {
				content: PageContent::Url(url, None),
				..Default::default()
			});
			page += 1;
		}
		if urls.len() == 1 {
			break;
		}
	}

	Ok(pages)
}

fn substring_after<'a>(s: &'a str, pat: &str) -> &'a str {
	s.find(pat).map(|pos| &s[pos + pat.len()..]).unwrap_or("")
}

fn substring_before<'a>(s: &'a str, pat: &str) -> &'a str {
	s.find(pat).map(|pos| &s[..pos]).unwrap_or("")
}

fn substring_before_last<'a>(s: &'a str, pat: &str) -> &'a str {
	s.rfind(pat).map(|pos| &s[..pos]).unwrap_or("")
}

fn substring_after_last<'a>(s: &'a str, pat: &str) -> &'a str {
	s.rfind(pat).map(|pos| &s[pos + pat.len()..]).unwrap_or("")
}

fn decode(encoded: String) -> Vec<String> {
	let packed = substring_after(&encoded, "return p;}").to_string();
	let k: Vec<&str> = substring_before(substring_after(&packed, ",\'"), "\'.")
		.split('|')
		.collect();

	let chapter = decoded_with_k(
		substring_before(substring_after(&packed, "=\""), "\";").to_string(),
		k.clone(),
	);
	let query = decoded_with_k(
		substring_after_last(substring_before_last(&packed, "\\\'"), "\\\'").to_string(),
		k.clone(),
	);

	substring_before(substring_after(&packed, "=["), "];")
		.split(',')
		.map(|item| decoded_with_k(item.replace('\"', "").to_string(), k.clone()))
		.map(|page| format!("{}{}{}", chapter, page, query))
		.collect()
}

fn decoded_with_k(encoded: String, k: Vec<&str>) -> String {
	let mut decoded = String::new();

	for char in encoded.chars() {
		let str = if char.is_digit(36) {
			let index = if char.is_ascii_uppercase() {
				(char as usize) - 29
			} else {
				char.to_digit(36).unwrap_or(0) as usize
			};
			if index < k.len() {
				let s = k[index];
				if s.is_empty() {
					char.to_string()
				} else {
					s.to_string()
				}
			} else {
				char.to_string()
			}
		} else {
			char.to_string()
		};
		decoded.push_str(str.as_str());
	}

	decoded
}
