use crate::{USER_AGENT, decoder::Decoder};
use aidoku::{
	Chapter, ContentRating, Manga, MangaPageResult, MangaStatus, Page, PageContent, Result,
	SelectFilter, Viewer,
	alloc::{String, Vec, borrow::Cow, vec},
	helpers::uri::encode_uri,
	imports::{
		html::{Document, Html},
		net::Request,
	},
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

pub trait GenresPage {
	fn filter(&self) -> Result<SelectFilter>;
}

impl MangaPage for Document {
	fn manga_page_result(&self) -> Result<MangaPageResult> {
		let mut mangas: Vec<Manga> = Vec::new();

		// Check if this is a search page or home page
		let (selector, pagination_selector) =
			if self.select_first(".cf > .book-cover > a").is_some() {
				// Search page
				(".cf > .book-cover > a", "#AspNetPagerResult > a")
			} else {
				// Home page
				("#contList > li", "#AspNetPager1 > a")
			};

		let elements = self
			.select(selector)
			.ok_or_else(|| error!("Failed to select manga list"))?;
		for element in elements {
			let link_element = if selector == ".cf > .book-cover > a" {
				element
			} else {
				element
					.select_first("a")
					.ok_or_else(|| error!("Failed to select link"))?
			};

			let href = link_element
				.attr("href")
				.ok_or_else(|| error!("Failed to get href"))?;
			let manga_id = href.replace("/comic/", "").replace('/', "");
			let title = link_element
				.attr("title")
				.ok_or_else(|| error!("Failed to get title"))?;
			let manga = Manga {
				key: manga_id.clone(),
				cover: Some(format!("https://cf.hamreus.com/cpic/b/{}.jpg", manga_id)),
				title,
				url: Some(format!(
					"{}/comic/{}",
					crate::settings::get_base_url(),
					manga_id
				)),
				..Default::default()
			};
			mangas.push(manga);
		}

		let mut has_next: bool = false;
		if let Some(pages) = self.select(pagination_selector) {
			for page in pages {
				if let Some(text) = page.text()
					&& (text == "尾页" || text == "尾頁")
				{
					has_next = true;
					break;
				}
			}
		}

		Ok(MangaPageResult {
			entries: mangas,
			has_next_page: has_next,
		})
	}

	fn update_details(&self, manga: &mut Manga) -> Result<()> {
		manga.cover = Some(format!("https://cf.hamreus.com/cpic/b/{}.jpg", manga.key));

		let title_element = self
			.select_first(".book-title > h1")
			.ok_or_else(|| error!("Failed to select title"))?;
		manga.title = title_element
			.text()
			.ok_or_else(|| error!("Failed to get title"))?;

		let mut authors = Vec::new();
		if let Some(author_elements) =
			self.select("ul.detail-list li:nth-child(2) span:nth-child(2) a")
		{
			for author_link in author_elements {
				if let Some(author_text) = author_link.text()
					&& !author_text.is_empty()
				{
					authors.push(author_text);
				}
			}
		}
		manga.authors = Some(authors.clone());
		manga.artists = Some(authors);

		let desc_element = self
			.select_first("#intro-cut")
			.ok_or_else(|| error!("Failed to select description"))?;
		manga.description = Some(
			desc_element
				.text()
				.ok_or_else(|| error!("Failed to get description"))?,
		);

		let status_element = self
			.select_first("li.status")
			.ok_or_else(|| error!("Failed to select status"))?;
		let status_text = status_element
			.text()
			.ok_or_else(|| error!("Failed to get status"))?;
		manga.status = if status_text.contains("已完结") || status_text.contains("已完結") {
			MangaStatus::Completed
		} else {
			MangaStatus::Ongoing
		};

		let mut categories = Vec::new();
		if let Some(category_elements) =
			self.select("ul.detail-list li:nth-child(2) span:nth-child(1) a")
		{
			for category in category_elements {
				if let Some(cat_text) = category.text()
					&& !cat_text.is_empty()
				{
					categories.push(cat_text);
				}
			}
		}

		let country = if let Some(country_element) =
			self.select_first("ul.detail-list li:nth-child(1) span:nth-child(2) a")
		{
			country_element.text()
		} else {
			None
		};

		let mut all_tags = Vec::new();
		if let Some(ref c) = country {
			all_tags.push(c.clone());
		}
		all_tags.extend(categories);
		manga.tags = Some(all_tags);

		manga.viewer = if let Some(ref c) = country {
			if c.contains("内地") || c.contains("韩国") || c.contains("韓國") {
				Viewer::Webtoon
			} else if c.contains("日本") {
				Viewer::RightToLeft
			} else {
				Viewer::LeftToRight
			}
		} else {
			Viewer::LeftToRight
		};

		// Check for NSFW content
		let has_check_adult = self.select_first("#checkAdult").is_some();
		let has_viewstate = self.select_first("#__VIEWSTATE").is_some();
		manga.content_rating = if has_check_adult || has_viewstate {
			ContentRating::NSFW
		} else {
			ContentRating::Safe
		};

		Ok(())
	}
}

impl ChapterPage for Document {
	fn chapters(&self) -> Result<Vec<Chapter>> {
		let mut chapters: Vec<Chapter> = Vec::new();
		let mut index = 1.0;

		let div_owned: Option<Document> = self
			.select_first("#__VIEWSTATE")
			.and_then(|el| el.attr("value"))
			.and_then(|compressed| {
				crate::decoder::decompress_from_base64(&compressed)
					.and_then(|data| String::from_utf16(&data).ok())
			})
			.filter(|s| !s.is_empty())
			.and_then(|decompressed| Html::parse(&decompressed).ok());

		let div: &Document = div_owned.as_ref().unwrap_or(self);

		// Parse scanlators from h4 tags
		let mut scanlators: Vec<String> = Vec::new();
		if let Some(h4_elements) = div.select("h4") {
			for h4 in h4_elements {
				if let Some(span) = h4.select_first("span")
					&& let Some(scanlator) = span.text()
				{
					scanlators.push(scanlator);
				}
			}
		}
		scanlators.reverse(); // Reverse to match the .rev() order of chapter-list

		let chapter_list_elements = div
			.select(".chapter-list")
			.ok_or_else(|| error!("Failed to select chapter list"))?;
		for (scanlator_index, element) in chapter_list_elements.rev().enumerate() {
			let chapt_list_div = element;

			let scanlator = if scanlator_index < scanlators.len() {
				scanlators[scanlator_index].clone()
			} else {
				String::new()
			};

			if let Some(ul_elements) = chapt_list_div.select("ul") {
				for ul_ref in ul_elements {
					let ul = ul_ref;

					if let Some(li_elements) = ul.select("li") {
						for li_ref in li_elements.rev() {
							let elem = li_ref;

							let a_element = elem
								.select_first("a")
								.ok_or_else(|| error!("Failed to select a"))?;
							let href = a_element
								.attr("href")
								.ok_or_else(|| error!("Failed to get href"))?;
							let url = format!("{}{}", crate::settings::get_base_url(), href);
							let id = href.replace("/comic/", "").replace(".html", "");
							let chapter_id = match id.split('/').next_back() {
								Some(id) => String::from(id),
								None => String::new(),
							};

							let title_a = elem
								.select_first("a")
								.ok_or_else(|| error!("Failed to select title"))?;
							let mut title = title_a
								.attr("title")
								.ok_or_else(|| error!("Failed to get title"))?;
							let chapter_or_volume = extract_chapter_number(&title).unwrap_or(index);
							let (ch, vo) = if scanlator == "单行本" || scanlator == "單行本" {
								(-1.0, chapter_or_volume)
							} else {
								(chapter_or_volume, -1.0)
							};

							// Add page count if available
							if let Some(i_element) = elem.select_first("i")
								&& let Some(page_text) = i_element.text()
								&& !page_text.is_empty()
							{
								title = format!("{} ({})", title, page_text);
							}

							let chapter = Chapter {
								key: chapter_id,
								title: Some(title),
								volume_number: if vo >= 0.0 { Some(vo) } else { None },
								chapter_number: if ch >= 0.0 { Some(ch) } else { None },
								url: Some(url),
								scanlators: if scanlator.is_empty() {
									None
								} else {
									Some(vec![scanlator.clone()])
								},
								language: Some(String::from("zh")),
								..Default::default()
							};

							chapters.push(chapter);
							index += 1.0;
						}
					}
				}
			}
		}
		chapters.reverse();

		Ok(chapters)
	}
}

impl GenresPage for Document {
	fn filter(&self) -> Result<SelectFilter> {
		let genre_links = self
			.select_first("div.filter.genre")
			.and_then(|div| div.select("ul li a"))
			.ok_or_else(|| error!("Failed to select genre filter links"))?;

		let (options, ids): (Vec<_>, Vec<_>) = genre_links
			.filter_map(|element| {
				let option = element.text()?;
				let href = element.attr("href")?;
				let id = href.trim_matches('/').split('/').next_back()?.into();
				Some((Cow::Owned(option), Cow::Owned(id)))
			})
			.unzip();

		Ok(SelectFilter {
			id: "类型".into(),
			title: Some("类型".into()),
			is_genre: true,
			uses_tag_style: true,
			options,
			ids: Some(ids),
			..Default::default()
		})
	}
}

pub fn get_page_list(base_url: String) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let html_content = Request::get(base_url)?
		.header("Referer", crate::settings::get_base_url())
		.header("User-Agent", USER_AGENT)
		.header("Accept-Language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
		.header("Cookie", "device_view=pc; isAdult=1")
		.string()?;

	let decoder = Decoder::new(html_content);
	let (path, pages_str) = decoder.decode();

	for str in pages_str.into_iter() {
		let encoded_path = encode_uri(&path);
		let url = format!("https://i.hamreus.com{}{}", encoded_path, str);
		pages.push(Page {
			content: PageContent::Url(url, None),
			..Default::default()
		});
	}

	Ok(pages)
}
