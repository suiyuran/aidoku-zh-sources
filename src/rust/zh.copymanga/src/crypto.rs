use aes::{
	cipher::{
		block_padding::{Pkcs7, UnpadError},
		BlockDecryptMut, KeyIvInit,
	},
	Aes128,
};
use aidoku::std::Vec;
use cbc::Decryptor;

type Aes128CbcDec = Decryptor<Aes128>;

pub fn decrypt(cipher: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, UnpadError> {
	Aes128CbcDec::new(key.into(), iv.into()).decrypt_padded_vec_mut::<Pkcs7>(&cipher)
}
