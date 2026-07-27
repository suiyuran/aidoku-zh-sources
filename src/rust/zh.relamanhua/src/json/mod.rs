pub mod chapter_list;
pub mod page_list;
pub mod search;

use crate::net::Url;
use aes::{
	Aes128,
	cipher::{BlockDecryptMut as _, KeyIvInit as _, block_padding::Pkcs7},
};
use aidoku::{
	AidokuError, Manga, MangaStatus, Result,
	alloc::{String, Vec},
	error,
	serde::Deserialize,
};

#[derive(Deserialize)]
pub struct MangaItem {
	pub path_word: String,
	pub name: String,
	pub cover: String,
	pub status: Option<u8>,
	pub author: Vec<Author>,
}

impl From<MangaItem> for Manga {
	fn from(item: MangaItem) -> Self {
		let url = Url::manga(&item.path_word).into();

		let key = item.path_word;

		let title = item.name;

		let cover = item.cover.replace(".328x422.jpg", "");

		let authors = item.author.into_iter().map(|author| author.name).collect();

		let status = match item.status {
			Some(0) => MangaStatus::Ongoing,
			Some(1 | 2) => MangaStatus::Completed,
			_ => MangaStatus::Unknown,
		};

		Self {
			key,
			title,
			cover: Some(cover),
			authors: Some(authors),
			url: Some(url),
			status,
			..Default::default()
		}
	}
}

#[derive(Deserialize)]
pub struct Author {
	pub name: String,
}

pub trait EncryptedJson {
	fn decrypt(&self, key: &str) -> Result<Vec<u8>>;
}

impl<S: AsRef<str>> EncryptedJson for S {
	fn decrypt(&self, key: &str) -> Result<Vec<u8>> {
		let data = self.as_ref();
		let iv = data
			.get(..16)
			.ok_or_else(|| error!("Expected 16 bytes for IV"))?
			.as_bytes()
			.into();

		let encoded_cipher_text = data
			.get(16..)
			.ok_or_else(|| error!("No data found after IV"))?;
		let mut cipher_text = hex::decode(encoded_cipher_text).map_err(AidokuError::message)?;

		let plain_text = cbc::Decryptor::<Aes128>::new(key.as_bytes().into(), iv)
			.decrypt_padded_mut::<Pkcs7>(&mut cipher_text)
			.map_err(AidokuError::message)?
			.into();
		Ok(plain_text)
	}
}
