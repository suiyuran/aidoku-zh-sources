use aidoku::{prelude::*, std::String};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn encrypt(data: &[u8], key: &[u8]) -> String {
	let mut mac = HmacSha256::new_from_slice(key).expect("");
	mac.update(data);
	let result = mac.finalize();
	format!("{:x}", result.into_bytes())
}
