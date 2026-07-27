use aidoku::{
	AidokuError, Page, PageContent, Result,
	alloc::{String, format},
	error,
	imports::defaults::defaults_get,
	serde::Deserialize,
};

#[derive(Deserialize)]
pub struct Item {
	url: String,
}

impl TryFrom<Item> for Page {
	type Error = AidokuError;

	fn try_from(page_item: Item) -> Result<Self> {
		let quality = defaults_get_string("image.quality").unwrap_or_else(|_| "h1500x".into());
		let format = defaults_get_string("image.format").unwrap_or_else(|_| "webp".into());

		// Check if URL already contains the user's preferred quality and format
		let url = if page_item.url.contains(&format!("{quality}.{format}")) {
			// URL already has the correct quality and format, use as-is
			page_item.url
		} else if let Some(last_dot) = page_item.url.rfind('.') {
			let before_ext = &page_item.url[..last_dot];
			let extension = &page_item.url[last_dot + 1..];

			// Check if extension looks like "h1234x.webp" (quality.format)
			if extension.starts_with('h') && extension.contains('x') && extension.contains('.') {
				// Replace existing quality.format with user's preference
				format!("{before_ext}.{quality}.{format}")
			} else {
				// Keep original URL if it doesn't match expected pattern
				page_item.url
			}
		} else {
			// No extension found, keep original
			page_item.url
		};

		let content = PageContent::Url(url, None);
		Ok(Self {
			content,
			..Default::default()
		})
	}
}

fn defaults_get_string(key: &str) -> Result<String> {
	defaults_get(key)
		.ok_or_else(|| error!("Default does not exist or is not a string or number value"))
}
