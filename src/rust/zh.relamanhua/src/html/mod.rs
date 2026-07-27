use crate::{
	json::{EncryptedJson as _, MangaItem, page_list},
	net::Url,
};
use aidoku::{
	AidokuError, Manga, MangaPageResult, MangaStatus, Page, Result, SelectFilter,
	alloc::{String, Vec, borrow::ToOwned as _, string::ToString as _, vec},
	error,
	imports::html::{Document, Element, ElementList},
};

pub trait GenresPage {
	fn filter(&self) -> Result<SelectFilter>;
}

impl GenresPage for Document {
	fn filter(&self) -> Result<SelectFilter> {
		let (mut options, mut ids) = self
			.try_select("div#all a:not([disabled])")?
			.filter_map(|element| {
				let option = element.own_text()?.into();
				let id = element.attr("href")?.rsplit_once('=')?.1.to_owned().into();
				Some((option, id))
			})
			.collect::<(Vec<_>, Vec<_>)>();

		options.insert(0, "全部".into());
		ids.insert(0, "".into());

		Ok(SelectFilter {
			id: "題材".into(),
			title: Some("題材".into()),
			is_genre: true,
			uses_tag_style: true,
			options,
			ids: Some(ids),
			..Default::default()
		})
	}
}

pub trait FiltersPage {
	fn manga_page_result(&self) -> Result<MangaPageResult>;
}

impl FiltersPage for Document {
	fn manga_page_result(&self) -> Result<MangaPageResult> {
		let mut entries = Vec::new();

		if let Some(elements) = self.select("div.exemptComicItem") {
			// Debug: check if we found any elements
			let mut element_count = 0;
			let mut temp_i = 0;
			while elements.get(temp_i).is_some() {
				element_count += 1;
				temp_i += 1;
			}

			// Debug: if no elements found, return error with count info
			if element_count == 0 {
				return Err(error!(
					"No exemptComicItem elements found - selector returned empty list (element_count: {})",
					element_count
				));
			}

			let mut i = 0;
			while let Some(item) = elements.get(i) {
				// Debug: check what we find in each item
				let title_link =
					item.select_first("div.exemptComicItem-txt a")
						.ok_or_else(|| {
							error!("Item {i}: No title link found (div.exemptComicItem-txt a)")
						})?;
				let href = title_link
					.attr("href")
					.ok_or_else(|| error!("Item {i}: Title link has no href attribute"))?;
				let target_url_prefix = "/comic/";
				let path_word = href
					.find(target_url_prefix)
					.map(|pos| href[pos + target_url_prefix.len()..].to_string())
					.ok_or_else(|| error!("Item {i}: Invalid URL format: {href}"))?;

				// Get title from the p.twoLines element inside the link
				let title_element = title_link.select_first("p.twoLines").ok_or_else(|| {
					error!("Item {}: No p.twoLines element found in title link", i)
				})?;
				let title = title_element
					.own_text()
					.ok_or_else(|| error!("Item {}: p.twoLines element has no text content", i))?
					.trim()
					.to_string();
				if title.is_empty() {
					return Err(error!("Item {}: Title is empty", i));
				}

				// Get cover image - use the src attribute directly as it contains the full URL
				let cover = item
					.select_first("img")
					.ok_or_else(|| error!("Item {i}: No image found"))?
					.attr("src")
					.ok_or_else(|| error!("Item {i}: Image has no src attribute"))?
					.replace(".328x422.jpg", "");

				// Get author
				let author = item
					.select_first("span.exemptComicItem-txt-span")
					.and_then(|el| el.own_text())
					.map(|author_text| {
						author_text
							.strip_prefix("作者：")
							.unwrap_or(&author_text)
							.trim()
							.to_string()
					})
					.unwrap_or_default();

				let manga_item = MangaItem {
					path_word: path_word.to_string(),
					name: title,
					cover,
					status: Some(0), // Default to ongoing
					author: vec![crate::json::Author { name: author }],
				};

				entries.push(manga_item.into());
				i += 1;
			}
		} else {
			return Err(error!(
				"Selector 'div.exemptComicItem' failed to find any elements"
			));
		}

		// Debug: check if we processed any entries
		if entries.is_empty() {
			return Err(error!(
				"No manga entries were processed - all parsing failed"
			));
		}

		let has_next_page = self
			.select("li.page-all-item")
			.and_then(|mut elements| elements.next_back())
			.is_some_and(|element| !element.has_class("active"));

		Ok(MangaPageResult {
			entries,
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
			.try_select_first("h6")?
			.text()
			.ok_or_else(|| error!("Text not found"))?;

		manga.cover = self
			.try_select_first("img")?
			.attr("src")
			.map(|resized| resized.replace(".328x422.jpg", ""));

		let authors = self
			.select("a[href^='/author/']")
			.into_iter()
			.flatten()
			.filter_map(|element| element.text())
			.collect();
		manga.authors = Some(authors);

		manga.description = self.try_select_first("p.intro")?.text();

		let url = Url::manga(&manga.key).into();
		manga.url = Some(url);

		let tags = self
			.select("a[href*='theme=']")
			.into_iter()
			.flatten()
			.filter_map(|element| {
				let full_text = element.text()?;
				let tag = full_text.strip_prefix('#').unwrap_or(&full_text).into();
				Some(tag)
			})
			.collect();
		manga.tags = Some(tags);

		let status = self
			.select("span.comicParticulars-right-txt")
			.and_then(|mut elements| {
				elements.find_map(|element| {
					let text = element.text()?;
					matches!(text.trim(), "連載中" | "已完結" | "短篇").then_some(text)
				})
			});
		manga.status = match status.as_deref().map(str::trim) {
			Some("連載中") => MangaStatus::Ongoing,
			Some("已完結" | "短篇") => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};

		Ok(())
	}
}

pub trait KeyPage {
	fn key(&self) -> Result<String>;
}

impl KeyPage for Document {
	fn key(&self) -> Result<String> {
		if let Some(disposable_div) = self.select_first("div.disPass")
			&& let Some(disposable_attr) = disposable_div
				.attr("contentKey")
				.or_else(|| disposable_div.attr("contentkey"))
		{
			return Ok(disposable_attr);
		}

		if let Some(disposable_div) = self.select_first("div.disposablePass")
			&& let Some(disposable_attr) = disposable_div.attr("disposable")
		{
			return Ok(disposable_attr);
		}

		Err(error!("No key found"))
	}
}

pub trait ChapterPage {
	fn pages(&self) -> Result<Vec<Page>>;
}

impl ChapterPage for Document {
	fn pages(&self) -> Result<Vec<Page>> {
		let key = self.key()?;

		let json = self
			.select_first("div.disData")
			.and_then(|div| div.attr("contentKey").or_else(|| div.attr("contentkey")))
			.map(|attr| attr.to_string())
			.ok_or_else(|| error!("No disData div found"))?;

		let decrypted_json = json.decrypt(&key)?;
		serde_json::from_slice::<Vec<page_list::Item>>(&decrypted_json)
			.map_err(AidokuError::message)?
			.into_iter()
			.map(TryInto::try_into)
			.collect()
	}
}

trait TrySelect {
	fn try_select<S: AsRef<str>>(&self, css_query: S) -> Result<ElementList>;
	fn try_select_first<S: AsRef<str>>(&self, css_query: S) -> Result<Element>;
}

impl TrySelect for Document {
	fn try_select<S: AsRef<str>>(&self, css_query: S) -> Result<ElementList> {
		self.select(&css_query)
			.ok_or_else(|| error!("No element found for selector: `{}`", css_query.as_ref()))
	}

	fn try_select_first<S: AsRef<str>>(&self, css_query: S) -> Result<Element> {
		self.select_first(&css_query)
			.ok_or_else(|| error!("No element found for selector: `{}`", css_query.as_ref()))
	}
}
