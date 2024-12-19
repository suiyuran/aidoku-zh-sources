use aidoku::{
	error::{AidokuError, AidokuErrorKind},
	prelude::format,
	std::{
		defaults::{defaults_get, defaults_set},
		html::Node,
		net::{HttpMethod, Request},
		StringRef,
	},
};
use alloc::{string::String, vec::Vec};

fn gen_request(url: String, method: HttpMethod) -> Request {
	Request::new(url, method)
}

fn handle_cookie_header(cookie_header: String) -> String {
	return cookie_header
		.replace(",", ";")
		.split(";")
		.filter(|a| a.contains("Ckng"))
		.map(|a| a.trim())
		.collect::<Vec<&str>>()
		.join(";");
}

fn get_default(key: &str) -> Result<String, AidokuError> {
	Ok(defaults_get(key)?.as_string()?.read())
}

pub fn get_url() -> String {
	get_default("url").unwrap()
}

pub fn get_html(url: String) -> Result<Node, AidokuError> {
	let default_cookie = get_default("cookie")?;
	let request = gen_request(url.clone(), HttpMethod::Get).header("Cookie", &default_cookie);

	request.send();

	let cookie_header = request.get_header("set-cookie").unwrap_or_default().read();
	let html = request.html().unwrap();

	if html
		.select("#main_message #messagetext>p")
		.text()
		.read()
		.contains("仅限用户观看，请先登录")
	{
		let username = get_default("username")?;
		let password = get_default("password")?;

		if username.is_empty() || password.is_empty() {
			return Err(AidokuError {
				reason: AidokuErrorKind::DefaultNotFound,
			});
		}

		let action = html.select("#lsform").attr("action").read();
		let formhash = html.select("input[name=formhash]").attr("value").read();
		let login_cookie = handle_cookie_header(cookie_header);
		let body = format!(
			"username={}&cookietime=2592000&password={}&formhash={}&quickforward=yes&handlekey=ls",
			username, password, formhash
		);
		let login_url = format!("{}/{}&inajax=1", get_url(), action);
		let login_request = gen_request(login_url, HttpMethod::Post)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.header("Cookie", &login_cookie)
			.body(body.as_bytes());

		login_request.send();

		let new_cookie_header = login_request
			.get_header("set-cookie")
			.unwrap_or_default()
			.read();

		if !new_cookie_header.contains("auth") {
			return Err(AidokuError {
				reason: AidokuErrorKind::DefaultNotFound,
			});
		}

		let new_cookie = handle_cookie_header(new_cookie_header);

		defaults_set("cookie", StringRef::from(new_cookie).0);

		return get_html(url);
	}

	return Ok(html);
}
