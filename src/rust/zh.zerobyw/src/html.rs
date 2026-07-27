use crate::net::Url;
use aidoku::{
	Chapter, Manga, MangaPageResult, MangaStatus, Page, PageContent, Result,
	alloc::{String, Vec},
	error,
	imports::html::{Document, Element, ElementList},
};
use alloc::format;
use alloc::string::ToString;
use core::default::Default;

pub trait FiltersPage {
	fn manga_page_result(&self) -> Result<MangaPageResult>;
}

impl FiltersPage for Document {
	fn manga_page_result(&self) -> Result<MangaPageResult> {
		let mut mangas: Vec<Manga> = Vec::new();
		let cards = self
			.select("a.group.block[href^='/pc/details/']")
			.ok_or_else(|| error!("No manga cards found"))?;

		for card in cards {
			// id
			let Some(href) = card.attr("href") else {
				continue;
			};
			let id = href
				.split("kuid=")
				.nth(1)
				.ok_or_else(|| error!("Invalid manga href found"))?
				.to_string();
			// cover
			let cover = card.select_first("img").and_then(|img| img.attr("src"));
			// title
			let title = card
				.select_first("h3.manga-card-title")
				.and_then(|title| title.text())
				.map(|text| text.trim().to_string())
				.unwrap_or_default();

			mangas.push(Manga {
				key: id,
				title,
				cover,
				..Default::default()
			});
		}

		let has_next_page = !mangas.is_empty() && self.select("a:contains('下一页')").is_some();

		Ok(MangaPageResult {
			entries: mangas,
			has_next_page,
		})
	}
}

pub trait MangaPage {
	fn update_details(&self, manga: &mut Manga) -> Result<()>;
}

impl MangaPage for Document {
	fn update_details(&self, manga: &mut Manga) -> Result<()> {
		manga.title = self
			.try_select_first("h1.text-2xl.font-medium")?
			.text()
			.ok_or_else(|| error!("Text not found"))?;
		manga.cover = self
			.try_select_first("img[src*='tupa.zerobyw33.com']")?
			.attr("src");

		let mut authors = Vec::new();
		let mut tags = Vec::new();
		let mut status = MangaStatus::Unknown;
		for span in self.try_select("span.px-3.py-1.bg-gray-100")? {
			let Some(text) = span.text() else {
				continue;
			};

			let text = text.trim();
			if text.is_empty() {
				continue;
			}

			if text.starts_with("作者:") {
				let raw = text.replace("作者:", "").trim().to_string();
				let parts: Vec<String> = raw
					.split('×')
					.flat_map(|s| s.split('x'))
					.map(|s| s.trim().to_string())
					.filter(|s| !s.is_empty())
					.collect();
				authors.extend(parts);
				continue;
			}

			match text {
				"连载中" => {
					status = MangaStatus::Ongoing;
					continue;
				}
				"已完结" => {
					status = MangaStatus::Completed;
					continue;
				}
				_ => {}
			}

			if text.starts_with("收藏:") || text.starts_with("人气:") {
				continue;
			}

			tags.push(text.to_string());
		}

		if !authors.is_empty() {
			manga.authors = Some(authors);
		}
		if !tags.is_empty() {
			manga.tags = Some(tags);
		}
		manga.status = status;

		manga.description = self.try_select_first("p[x-ref='summaryText']")?.text();

		manga.url = Url::manga(&manga.key).to_string().ok();

		Ok(())
	}
}

pub trait ChapterListPage {
	fn chapter_list(&self) -> Result<Vec<Chapter>>;
}

impl ChapterListPage for Document {
	fn chapter_list(&self) -> Result<Vec<Chapter>> {
		let mut chapters = Vec::new();
		let mut last_zjid = 0;

		for item in self.try_select(".grid > *")? {
			let title = item.text().unwrap_or_default().trim().to_string();
			let title = if title.is_empty() { None } else { Some(title) };

			if let Some(href) = item.attr("href")
				&& href.starts_with("/pc/view/index.php?zjid=")
			{
				// 可访问章节
				let zjid = href
					.split("zjid=")
					.nth(1)
					.ok_or_else(|| error!("Missing zjid in href: {}", href))?
					.to_string();
				let url = Url::chapter(&zjid).to_string().ok();
				if let Ok(num) = zjid.parse::<i64>() {
					last_zjid = num;
				}

				chapters.push(Chapter {
					key: zjid,
					title,
					url,
					locked: false,
					..Default::default()
				});
			} else {
				let new_zjid = last_zjid + 1;
				last_zjid = new_zjid;
				let zjid = new_zjid.to_string();
				let url = Url::chapter(&zjid).to_string().ok();
				chapters.push(Chapter {
					key: zjid,
					title,
					url,
					locked: true,
					..Default::default()
				});
			}
		}

		for (i, ch) in chapters.iter_mut().enumerate() {
			ch.chapter_number = Some((i + 1) as f32);
			if ch.title.is_none() || ch.title.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
				ch.title = Some(format!("{}", i + 1));
			}
		}

		chapters.reverse();
		Ok(chapters)
	}
}

pub trait ChapterPage {
	fn pages(&self) -> Result<Vec<Page>>;
}

impl ChapterPage for Document {
	fn pages(&self) -> Result<Vec<Page>> {
		let mut pages = Vec::new();

		let imgs = self.try_select("img.manga-image")?;

		for img in imgs {
			let Some(src) = img.attr("src") else {
				continue;
			};
			let url = if src.starts_with("//") {
				format!("https:{}", src)
			} else {
				src
			};
			pages.push(Page {
				content: PageContent::url(url),
				..Default::default()
			})
		}

		Ok(pages)
	}
}

trait TrySelect {
	fn try_select_first<S: AsRef<str>>(&self, css_query: S) -> Result<Element>;
	fn try_select<S: AsRef<str>>(&self, css_query: S) -> Result<ElementList>;
}

impl TrySelect for Document {
	fn try_select_first<S: AsRef<str>>(&self, css_query: S) -> Result<Element> {
		self.select_first(&css_query)
			.ok_or_else(|| error!("No element found for selector: `{}`", css_query.as_ref()))
	}
	fn try_select<S: AsRef<str>>(&self, css_query: S) -> Result<ElementList> {
		self.select(&css_query)
			.ok_or_else(|| error!("No element found for selector: `{}`", css_query.as_ref()))
	}
}
