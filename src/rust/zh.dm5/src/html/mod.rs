use crate::{BASE_URL, USER_AGENT};
use aidoku::{
	Chapter, Manga, MangaPageResult, MangaStatus, Page, PageContent, Result,
	alloc::{String, Vec, string::ToString as _, vec},
	imports::{html::Document, js::JsContext, net::Request},
	prelude::*,
};
use regex::Regex;

/// Extracts the leading number from a chapter/volume title and reports whether
/// the unit attached to that number marks it as a volume (卷/册/冊) rather than a
/// chapter (话/話/章/回). The unit is anchored to the matched number so that a 卷
/// appearing inside a word does not misclassify a chapter.
fn extract_chapter_number(title: &str) -> Option<(f32, bool)> {
	let re = Regex::new(
		r"第\s*(\d+(?:\.\d+)?)\s*([话話章回卷册冊])?|(\d+(?:\.\d+)?)\s*([话話章回卷册冊])",
	)
	.ok()?;
	let caps = re.captures(title)?;
	let (num, unit) = if let Some(n) = caps.get(1) {
		(n, caps.get(2))
	} else {
		(caps.get(3)?, caps.get(4))
	};
	let number = num.as_str().parse::<f32>().ok()?;
	let is_volume = matches!(
		unit.map(|m| m.as_str()),
		Some("卷") | Some("册") | Some("冊")
	);
	Some((number, is_volume))
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
		let mut entries: Vec<Manga> = Vec::new();

		// Handle banner_detail_form for search
		if let Some(banner) = self.select_first("div.banner_detail_form")
			&& let (Some(id), Some(title)) = (
				banner
					.select_first("p.title > a")
					.and_then(|e| e.attr("href"))
					.and_then(|s| s.split('/').rfind(|s| !s.is_empty()).map(Into::into)),
				banner.select_first("p.title > a").and_then(|e| e.text()),
			) {
			let cover = banner.select_first("img").and_then(|e| e.attr("abs:src"));
			entries.push(Manga {
				key: id,
				cover,
				title,
				..Default::default()
			});
		}

		// Handle list items
		if let Some(items) = self.select("ul.mh-list > li > div.mh-item") {
			for item in items {
				let Some(id) = item
					.select_first("h2.title > a")
					.and_then(|e| e.attr("href"))
					.and_then(|s| s.split('/').rfind(|s| !s.is_empty()).map(Into::into))
				else {
					continue;
				};
				let Some(title) = item.select_first("h2.title > a").and_then(|e| e.text()) else {
					continue;
				};
				let cover = item
					.select_first("p.mh-cover")
					.and_then(|e| e.attr("style"))
					.and_then(|s| {
						s.split("url(")
							.nth(1)
							.and_then(|s| s.split(')').next())
							.map(|s| s.to_string())
					});

				entries.push(Manga {
					key: id,
					cover,
					title,
					..Default::default()
				});
			}
		}

		let has_next_page = self
			.select_first("div.page-pagination a:contains(>)")
			.is_some();

		Ok(MangaPageResult {
			entries,
			has_next_page,
		})
	}

	fn update_details(&self, manga: &mut Manga) -> Result<()> {
		let title = self
			.select_first("div.banner_detail_form p.title")
			.and_then(|e| e.own_text())
			.unwrap_or_default();
		let cover = self
			.select_first("div.banner_detail_form img")
			.and_then(|e| e.attr("abs:src"));
		let author = self
			.select_first("div.banner_detail_form p.subtitle > a")
			.and_then(|e| e.text())
			.unwrap_or_default();
		let genres = self
			.select("div.banner_detail_form p.tip a")
			.map(|elements| elements.filter_map(|a| a.text()).collect::<Vec<String>>())
			.unwrap_or_default();
		let el = self
			.select_first("div.banner_detail_form p.content")
			.unwrap();
		let mut description = el.own_text().unwrap_or_default();
		if let Some(span) = el.select_first("span")
			&& let Some(span_text) = span.own_text()
		{
			description.push_str(&span_text);
		}
		let status_text = self
			.select_first("div.banner_detail_form p.tip > span > span")
			.and_then(|e| e.text())
			.unwrap_or_default();
		let status = match status_text.as_str() {
			"连载中" => MangaStatus::Ongoing,
			"已完结" => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};

		manga.title = title;
		manga.cover = cover;
		manga.authors = Some(vec![author]);
		manga.tags = if genres.is_empty() {
			None
		} else {
			Some(genres)
		};
		manga.description = Some(description);
		manga.status = status;
		manga.url = Some(format!("{}/{}", BASE_URL, manga.key));

		Ok(())
	}
}

impl ChapterPage for Document {
	fn chapters(&self) -> Result<Vec<Chapter>> {
		let container = self
			.select_first("div#chapterlistload")
			.ok_or_else(|| error!("Chapter list not found"))?;

		let category_buttons = self
			.select("div.detail-list-title a.block")
			.map(|els| els.collect::<Vec<_>>())
			.unwrap_or_default();

		let is_ascending = self
			.select_first("div.detail-list-title a.order")
			.and_then(|e| e.text())
			.map(|t| t == "正序")
			.unwrap_or(false);

		let mut chapters: Vec<Chapter> = Vec::new();

		if let Some(ul_list) = container.select("ul[id^=detail-list-select-]") {
			for (ul_idx, ul) in ul_list.rev().enumerate() {
				let btn_idx = category_buttons.len().saturating_sub(ul_idx + 1);
				let scanlator = category_buttons.get(btn_idx).and_then(|btn| btn.own_text());

				if let Some(li) = ul.select("li > a") {
					let mut li_vec: Vec<_> = li.collect();
					if !is_ascending {
						li_vec.reverse();
					}
					for (index, item) in li_vec.into_iter().enumerate() {
						let href = item.attr("href").unwrap_or_default();
						let title = item
							.text()
							.map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
							.unwrap_or_default();
						let locked = item
							.select_first("span.detail-lock, span.view-lock")
							.is_some();

						let (num, title_is_volume) =
							extract_chapter_number(&title).unwrap_or(((index + 1) as f32, false));
						// A volume can be flagged by its category tab (卷), or by the
						// title itself, since some series file 单行本 entries under the
						// 连载 tab alongside regular chapters.
						let is_volume = scanlator.as_deref() == Some("卷")
							|| title_is_volume || title.contains("单行本")
							|| title.contains("單行本");
						let (chapter_number, volume_number) = if is_volume {
							(None, Some(num))
						} else {
							(Some(num), None)
						};

						chapters.push(Chapter {
							key: href.to_string(),
							title: Some(title),
							chapter_number,
							volume_number,
							url: Some(format!("{}{}", BASE_URL, href)),
							scanlators: scanlator.clone().map(|s| vec![s]),
							locked,
							..Default::default()
						});
					}
				}
			}
		}

		chapters.reverse();
		Ok(chapters)
	}
}

pub fn get_page_list(chapter_key: &str) -> Result<Vec<Page>> {
	let chapter_url = format!("{}{}", BASE_URL, chapter_key);
	let html = Request::get(&chapter_url)?
		.header("User-Agent", USER_AGENT)
		.header("Accept-Language", "zh-TW")
		.header(
			"Accept",
			"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
		)
		.header("Referer", BASE_URL)
		.header("DNT", "1")
		// Restricted series gate their reader pages behind the same consent
		// cookie the chapter list needs (see net::Url::request).
		.header("Cookie", "isAdult=1")
		.html()?;

	let mut pages: Vec<Page> = Vec::new();

	// Check for direct images first
	if let Some(img_list) = html.select("div#barChapter > img.load-src")
		&& !img_list.is_empty()
	{
		for img in img_list {
			let Some(data_src) = img.attr("data-src") else {
				continue;
			};
			pages.push(Page {
				content: PageContent::url(data_src),
				..Default::default()
			});
		}
		return Ok(pages);
	}

	// Find the script containing DM5 variables
	let script = html
		.select("script")
		.and_then(|mut scripts| scripts.find_map(|s| s.data().filter(|d| d.contains("DM5_MID"))))
		.ok_or_else(|| error!("Script not found"))?;

	if !script.contains("DM5_VIEWSIGN_DT") {
		if let Some(msg) = html
			.select_first("div.view-pay-form p.subtitle")
			.and_then(|e| e.text())
		{
			bail!("{}", msg);
		}
		bail!("Chapter not available");
	}

	let cid = extract_var(&script, "DM5_CID").ok_or_else(|| error!("CID not found"))?;
	let mid = extract_var(&script, "DM5_MID").ok_or_else(|| error!("MID not found"))?;
	let dt = extract_var(&script, "DM5_VIEWSIGN_DT").ok_or_else(|| error!("DT not found"))?;
	let sign = extract_var(&script, "DM5_VIEWSIGN").ok_or_else(|| error!("SIGN not found"))?;
	let image_count: usize = extract_var(&script, "DM5_IMAGE_COUNT")
		.and_then(|s| s.parse().ok())
		.ok_or_else(|| error!("Image count not found"))?;

	let base_url = chapter_url.split('?').next().unwrap_or(&chapter_url);

	// Each chapterfun.ashx request returns 2 URLs, so step by 2.
	let mut page_num = 1usize;
	while pages.len() < image_count {
		let api_url = format!(
			"{}/chapterfun.ashx?cid={}&page={}&key=&language=1&gtk=6&_cid={}&_mid={}&_dt={}&_sign={}",
			base_url, cid, page_num, cid, mid, dt, sign
		);
		let js_code = Request::get(&api_url)?
			.header("User-Agent", USER_AGENT)
			.header("Referer", base_url)
			.string()?;

		let result = JsContext::new().eval(&format!("{}\nJSON.stringify(d)", js_code))?;
		let trimmed = result.trim().trim_start_matches('[').trim_end_matches(']');
		for url_raw in trimmed.split(',') {
			let url = url_raw.trim().trim_matches('"');
			if url.starts_with("http") {
				pages.push(Page {
					content: PageContent::url(String::from(url)),
					..Default::default()
				});
				if pages.len() >= image_count {
					break;
				}
			}
		}
		page_num += 2;
	}

	Ok(pages)
}

fn extract_var(script: &str, var: &str) -> Option<String> {
	script
		.split(&format!("var {}=", var))
		.nth(1)
		.and_then(|s| s.split(';').next())
		.map(|s| s.trim().trim_matches('"').to_string())
}
