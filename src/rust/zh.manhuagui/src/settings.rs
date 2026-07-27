use aidoku::imports::defaults::defaults_get;

pub fn get_base_url() -> &'static str {
	let is_traditional_chinese = defaults_get("isTraditionalChinese")
		.ok_or("https://www.manhuagui.com/")
		.unwrap_or(false);
	if is_traditional_chinese {
		"https://tw.manhuagui.com/"
	} else {
		"https://www.manhuagui.com/"
	}
}
