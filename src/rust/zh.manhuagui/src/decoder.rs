use aidoku::{
	alloc::{String, Vec, vec},
	prelude::*,
};

pub struct Decoder {
	func: String,
	a: i32,
	c: i32,
	data: Vec<String>,
}

impl Decoder {
	pub fn new(document: String) -> Self {
		let script = get_script(document);

		let func = get_func(script.clone());
		let a = get_a(script.clone(), func.clone())
			.parse::<i32>()
			.unwrap_or(-1);
		let c = get_c(script.clone(), func.clone())
			.parse::<i32>()
			.unwrap_or(-1);
		let data: Vec<String> = get_data(script, func.clone());

		Decoder { func, a, c, data }
	}

	fn e(&self, c: i32) -> String {
		let prefix: String = if c >= self.a {
			self.e(c / self.a)
		} else {
			String::new()
		};

		let suffix_vec = [
			self.tr(c % self.a, 36),
			String::from_utf8(vec![(c % self.a + 29) as u8]).unwrap_or_else(|_| String::new()),
		];
		let suffix = suffix_vec[(c % self.a > 35) as usize].clone();

		format!("{}{}", prefix, suffix)
	}

	fn tr(&self, value: i32, num: i32) -> String {
		let tmp = Self::itr(value, num);
		if tmp.is_empty() {
			return String::from("0");
		}
		tmp
	}

	fn itr(value: i32, num: i32) -> String {
		let d = String::from("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
		if value <= 0 {
			return String::from("");
		}
		let first = Self::itr(value / num, num);
		let second = d.chars().nth((value % num) as usize).unwrap_or('\0');
		format!("{}{}", first, second)
	}

	pub fn decode(&self) -> (String, Vec<String>) {
		let mut c = self.c - 1;
		let mut d_key: Vec<String> = vec![];
		let mut d_value: Vec<String> = vec![];
		while c > -1 {
			let key = self.e(c);
			let value_index = self.data[c as usize].is_empty() as usize;
			let value = vec![self.data[c as usize].clone(), self.e(c)][value_index].clone();

			let index = d_key
				.clone()
				.iter()
				.position(|r| r.eq(key.as_str()))
				.unwrap_or(999);
			if index == 999 {
				d_key.push(key);
				d_value.push(value);
			} else {
				d_key[index] = key;
				d_value[index] = value;
			}
			c -= 1;
		}

		let func = self.func.clone();
		let mut result: Vec<String> = vec![];
		let mut splited: Vec<String> = vec![];
		let mut skip_next = false;
		for (a, b) in func.split("").zip(func.clone().split("").skip(1)) {
			if skip_next {
				skip_next = false;
				continue;
			}
			let key = format!("{}{}", a, b);
			if d_key.contains(&key) {
				splited.push(key);
				skip_next = true;
			} else {
				splited.push(String::from(a));
			}
		}
		if !skip_next {
			let last = func.split("").last().unwrap_or("");
			splited.push(String::from(last));
		}

		for ori in splited {
			if d_key.contains(&ori) {
				let index = d_key
					.clone()
					.iter()
					.position(|r| r.eq(ori.as_str()))
					.unwrap_or(0);
				result.push(d_value[index].clone());
			} else {
				result.push(ori);
			}
		}

		let js = result.join("");
		let mut json = String::new();

		for (i, s) in js.split(".imgData(").enumerate() {
			if i == 1 {
				for (j, ss) in String::from(s).clone().split(").preInit();").enumerate() {
					if j == 0 {
						json = String::from(ss);
					}
				}
			}
		}

		let mut pages: Vec<String> = vec![];
		let mut path: String = String::new();

		for (i, s) in json.split('[').enumerate() {
			if i == 1 {
				for (j, ss) in s.split(']').enumerate() {
					if j == 0 {
						// get files here
						for sss in ss.split(',') {
							pages.push(String::from(sss).replace('\"', ""));
						}
					} else if j == 1 {
						// get path here
						for (k, sss) in ss.split("\"path\":\"").enumerate() {
							if k == 1 {
								for (x, ssss) in sss.split("\",\"").enumerate() {
									if x == 0 {
										path = String::from(ssss);
									}
								}
							}
						}
					}
				}
			}
		}

		(path, pages)
	}
}

fn get_script(document: String) -> String {
	let mut script: String = String::new();
	for splited in document.split("window[\"\\x65\\x76\\x61\\x6c\"]") {
		if splited.starts_with("(function") {
			for s in splited.split(" </script>") {
				if s.starts_with("(function") {
					script.push_str(s);
				}
			}
		}
	}

	script
}

fn get_func(script: String) -> String {
	let mut func: String = String::new();
	for splited in script.split(";return p;}") {
		if splited.starts_with("('") {
			for s in splited.split("',") {
				if s.starts_with('(') {
					func.push_str(s.replacen("('", "", 1).as_str());
				}
			}
		}
	}

	func
}

fn get_a(script: String, func: String) -> String {
	let mut a: String = String::new();
	for splited in script.split(";return p;}") {
		if splited.starts_with("('") {
			let s = splited.replace(func.as_str(), "func");
			for (i, ss) in s.split(',').enumerate() {
				if i == 1 {
					a.push_str(ss);
					break;
				}
			}
		}
	}

	a
}

fn get_c(script: String, func: String) -> String {
	let mut c: String = String::new();
	for splited in script.split(";return p;}") {
		if splited.starts_with("('") {
			let s = splited.replace(func.as_str(), "func");
			for (index, ss) in s.split(',').enumerate() {
				if index == 2 {
					c.push_str(ss);
					break;
				}
			}
		}
	}

	c
}

fn get_data(script: String, func: String) -> Vec<String> {
	let mut data: Vec<String> = Vec::new();
	let mut data_str: String = String::new();
	for splited in script.split(";return p;}") {
		if splited.starts_with("('") {
			let s = splited.replace(func.as_str(), "func");
			for (i, ss) in s.split(',').enumerate() {
				if i == 3 {
					for (j, sss) in ss.split('\'').enumerate() {
						if j == 1 {
							data_str.push_str(sss);
						}
					}
					break;
				}
			}
		}
	}

	data_str = String::from_utf16(&decompress_from_base64(data_str.as_str()).unwrap_or_default())
		.unwrap_or_default();
	for str in data_str.split('|') {
		data.push(String::from(str));
	}
	data
}

// LZ String
// const URI_KEY: &[u8] =
// b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";
const BASE64_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const U8_CODE: u8 = 0;
const U16_CODE: u8 = 1;
const CLOSE_CODE: u8 = 2;
const START_CODE_BITS: u8 = 2;

#[derive(Debug)]
pub struct DecompressContext<I> {
	val: u16,
	compressed_data: I,
	position: u16,
	reset_val: u16,
}

impl<I> DecompressContext<I>
where
	I: Iterator<Item = u16>,
{
	#[inline]
	pub fn new(mut compressed_data: I, bits_per_char: u8) -> Option<Self> {
		let reset_val_pow = bits_per_char - 1;
		let reset_val: u16 = 1 << reset_val_pow;

		Some(DecompressContext {
			val: compressed_data.next()?,
			compressed_data,
			position: reset_val,
			reset_val,
		})
	}

	#[inline]
	pub fn read_bit(&mut self) -> Option<bool> {
		let res = self.val & self.position;
		self.position >>= 1;

		if self.position == 0 {
			self.position = self.reset_val;
			self.val = self.compressed_data.next()?;
		}

		Some(res != 0)
	}

	#[inline]
	pub fn read_bits(&mut self, n: u8) -> Option<u32> {
		let mut res = 0;
		let max_power: u32 = 1 << n;
		let mut power: u32 = 1;
		while power != max_power {
			res |= u32::from(self.read_bit()?) * power;
			power <<= 1;
		}

		Some(res)
	}
}

pub fn decompress_from_base64(compressed: &str) -> Option<Vec<u16>> {
	let compressed: Option<Vec<u16>> = compressed
		.encode_utf16()
		.flat_map(|c| {
			BASE64_KEY
				.iter()
				.position(|k| u8::try_from(c) == Ok(*k))
				.map(|n| u16::try_from(n).ok())
		})
		.collect();

	decompress_internal(compressed?.into_iter(), 6)
}

fn decompress_internal<I>(compressed: I, bits_per_char: u8) -> Option<Vec<u16>>
where
	I: Iterator<Item = u16>,
{
	let mut ctx = match DecompressContext::new(compressed, bits_per_char) {
		Some(ctx) => ctx,
		None => return Some(Vec::new()),
	};

	let mut dictionary: Vec<Vec<u16>> = Vec::with_capacity(16);
	for i in 0_u16..3_u16 {
		dictionary.push(vec![i]);
	}

	// u8::MAX > u2::MAX
	let code = u8::try_from(ctx.read_bits(START_CODE_BITS)?).unwrap_or_default();
	let first_entry = match code {
		U8_CODE | U16_CODE => {
			let bits_to_read = (code * 8) + 8;
			// bits_to_read == 8 or 16 <= 16
			u16::try_from(ctx.read_bits(bits_to_read)?).unwrap_or_default()
		}
		CLOSE_CODE => return Some(Vec::new()),
		_ => return None,
	};
	dictionary.push(vec![first_entry]);

	let mut w = vec![first_entry];
	let mut result = vec![first_entry];
	let mut num_bits: u8 = 3;
	let mut enlarge_in: u64 = 4;
	let mut entry;
	loop {
		let mut code = ctx.read_bits(num_bits)?;
		match u8::try_from(code) {
			Ok(code_u8 @ (U8_CODE | U16_CODE)) => {
				let bits_to_read = (code_u8 * 8) + 8;
				// if cc == 0 {
				// if (errorCount++ > 10000) return "Error"; // TODO: Error logic
				// }

				// bits_to_read == 8 or 16 <= 16
				let bits = u16::try_from(ctx.read_bits(bits_to_read)?).unwrap_or_default();
				dictionary.push(vec![bits]);
				code = u32::try_from(dictionary.len() - 1).ok()?;
				enlarge_in -= 1;
			}
			Ok(CLOSE_CODE) => return Some(result),
			_ => {}
		}

		if enlarge_in == 0 {
			enlarge_in = 1 << num_bits;
			num_bits += 1;
		}

		// Return error if code cannot be converted to dictionary index
		let code_usize = usize::try_from(code).ok()?;
		if let Some(entry_value) = dictionary.get(code_usize) {
			entry = entry_value.clone();
		} else if code_usize == dictionary.len() {
			entry = w.clone();
			entry.push(*w.first()?);
		} else {
			return None;
		}

		result.extend(&entry);

		// Add w+entry[0] to the dictionary.
		let mut to_be_inserted = w.clone();
		to_be_inserted.push(*entry.first()?);
		dictionary.push(to_be_inserted);
		enlarge_in -= 1;

		w = entry;

		if enlarge_in == 0 {
			enlarge_in = 1 << num_bits;
			num_bits += 1;
		}
	}
}
