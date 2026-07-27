#![expect(clippy::unwrap_used)]

use super::*;
use aidoku_test::aidoku_test;
use paste::paste;

macro_rules! change_charset_to {
	($Charset:ident, $expected_url:literal, $expected_lang:literal) => {
		paste! {
			#[aidoku_test]
			fn [<change_charset_to_ $Charset:lower>]() {
				let url = Url::ChangeCharset(Charset::$Charset);
				assert_eq!(url.to_string(), $expected_url);
				assert!(url.request().unwrap().send().unwrap().get_header("Set-Cookie").unwrap().contains(&format!("lang={}", $expected_lang)));
			}
		}
	};
}
change_charset_to!(Traditional, "https://boylove.cc/home/user/toT.html", "TW");
change_charset_to!(Simplified, "https://boylove.cc/home/user/toS.html", "CN");

#[aidoku_test]
fn filters_page() {
	let url = Url::FiltersPage;
	let expected_url = "https://boylove.cc/home/book/cate.html";
	assert_eq!(url.to_string(), expected_url);
	assert_eq!(
		url.request()
			.unwrap()
			.html()
			.unwrap()
			.select_first("ul.stui-header__menu > li.active > a")
			.unwrap()
			.attr("abs:href")
			.unwrap(),
		expected_url
	);
}

macro_rules! from_filters {
	($name:ident, ($page:literal$(, $filter:expr)*), $expected_url:literal, $code:literal) => {
		paste! {
			#[aidoku_test]
			fn [<from_filters_ $name>]() {
				let filters = [$($filter,)*];
				let url = Url::from_query_or_filters(None, $page, &filters).unwrap();
				assert_eq!(url.to_string(), $expected_url);
				assert!(url.request().unwrap().string().unwrap().starts_with(&format!(r#"{{"code":{}"#, $code)));
			}
		}
	};
}
from_filters!(
	default,
	(1),
	"https://boylove.cc/home/api/cate/tp/1-0-2-1-1-0-1-2",
	1
);
from_filters!(
	basic_ongoing_safe_manga_2,
	(
		2,
		FilterValue::Select {
			id: "閱覽權限".into(),
			value: "0".into()
		},
		FilterValue::Select {
			id: "連載狀態".into(),
			value: "0".into()
		},
		FilterValue::Select {
			id: "內容分級".into(),
			value: "1".into()
		},
		FilterValue::MultiSelect {
			id: "標籤".into(),
			included: ["日漫".into()].into(),
			excluded: [].into()
		}
	),
	"https://boylove.cc/home/api/cate/tp/1-%E6%97%A5%E6%BC%AB-0-1-2-1-1-0",
	0
);
from_filters!(
	vip_completed_nsfw_manhwa_h_3,
	(
		3,
		FilterValue::Select {
			id: "閱覽權限".into(),
			value: "1".into()
		},
		FilterValue::Select {
			id: "連載狀態".into(),
			value: "1".into()
		},
		FilterValue::Select {
			id: "內容分級".into(),
			value: "2".into()
		},
		FilterValue::MultiSelect {
			id: "標籤".into(),
			included: ["韩漫".into(), "高H".into()].into(),
			excluded: [].into()
		}
	),
	"https://boylove.cc/home/api/cate/tp/1-%E9%9F%A9%E6%BC%AB+%E9%AB%98H-1-1-3-2-1-1",
	0
);
from_filters!(
	author,
	(
		1,
		FilterValue::Text {
			id: "author".into(),
			value: "소조금".into()
		}
	),
	"https://boylove.cc/home/api/searchk?keyword=%EC%86%8C%EC%A1%B0%EA%B8%88&type=1&pageNo=1",
	0
);
from_filters!(
	tag,
	(
		1,
		FilterValue::Select {
			id: "genre".into(),
			value: "韩漫".into()
		}
	),
	"https://boylove.cc/home/api/searchk?keyword=%E9%9F%A9%E6%BC%AB&type=1&pageNo=1",
	0
);

macro_rules! from_query {
	($name:ident, $keyword:literal, $page:literal, $expected_url:literal) => {
		paste! {
			#[aidoku_test]
			fn [<from_filters_ $name>]() {
				let url = Url::from_query_or_filters(Some($keyword), $page, &[]).unwrap();
				assert_eq!(url.to_string(), $expected_url);
				assert!(url.request().unwrap().string().unwrap().starts_with(r#"{"code":0"#));
			}
		}
	};
}
from_query!(
	red_1,
	"紅",
	1,
	"https://boylove.cc/home/api/searchk?keyword=%E7%B4%85&type=1&pageNo=1"
);
from_query!(
	snake_2,
	"蛇",
	2,
	"https://boylove.cc/home/api/searchk?keyword=%E8%9B%87&type=1&pageNo=2"
);

#[aidoku_test]
fn abs() {
	assert_eq!(
		Url::Abs("/bookimages/img/20240605/7d14a38ef25968d00999dcc1999a97dd.webp").to_string(),
		"https://boylove.cc/bookimages/img/20240605/7d14a38ef25968d00999dcc1999a97dd.webp"
	);
}

#[aidoku_test]
fn manga() {
	assert_eq!(
		Url::manga("16904").to_string(),
		"https://boylove.cc/home/book/index/id/16904"
	);
}

#[aidoku_test]
fn chapter_list() {
	assert_eq!(
		Url::chapter_list("2633991").to_string(),
		"https://boylove.cc/home/api/getChapterListInChapter/tp/2633991-0-1-1000"
	);
}

#[aidoku_test]
fn chapter() {
	assert_eq!(
		Url::chapter("2633991").to_string(),
		"https://boylove.cc/home/book/capter/id/2633991"
	);
}

macro_rules! daily_update {
	($name:ident, $day_of_week:literal, $page:literal, $expected_url:literal) => {
		paste! {
			#[aidoku_test]
			fn [<daily_update_ $name>]() {
				assert_eq!(Url::daily_update($day_of_week, $page).to_string(), $expected_url);
			}
		}
	};
}
daily_update!(
	last_updated,
	"11",
	1,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=11&limit=18&page=0&lastpage=0"
);
daily_update!(
	mon,
	"0",
	2,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=0&limit=18&page=1&lastpage=0"
);
daily_update!(
	tue,
	"1",
	3,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=1&limit=18&page=2&lastpage=0"
);
daily_update!(
	wed,
	"2",
	4,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=2&limit=18&page=3&lastpage=0"
);
daily_update!(
	thu,
	"3",
	5,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=3&limit=18&page=4&lastpage=0"
);
daily_update!(
	fri,
	"4",
	6,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=4&limit=18&page=5&lastpage=0"
);
daily_update!(
	sat,
	"5",
	7,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=5&limit=18&page=6&lastpage=0"
);
daily_update!(
	sun,
	"6",
	8,
	"https://boylove.cc/home/Api/getDailyUpdate.html?widx=6&limit=18&page=7&lastpage=0"
);

#[aidoku_test]
fn uncensored() {
	assert_eq!(
		Url::listing("recommend", 1).to_string(),
		"https://boylove.cc/home/api/getpage/tp/1-recommend-0"
	);
}

#[aidoku_test]
fn ranking() {
	assert_eq!(
		Url::listing("topestmh", 1).to_string(),
		"https://boylove.cc/home/api/getpage/tp/1-topestmh-0"
	);
}

#[aidoku_test]
fn random() {
	assert_eq!(
		Url::random().to_string(),
		"https://boylove.cc/home/Api/getCnxh.html?limit=5&type=1"
	);
}

#[aidoku_test]
fn home() {
	assert_eq!(Url::Home.to_string(), "https://boylove.cc/");
}
