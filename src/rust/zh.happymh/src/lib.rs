#![no_std]
extern crate alloc;

use aidoku::{
	error::Result,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		ObjectRef, String, Vec,
	},
	Chapter, Filter, FilterType, Listing, Manga, MangaContentRating, MangaPageResult, MangaStatus,
	MangaViewer, Page,
};
use alloc::string::ToString;

const WWW_URL: &str = "https://m.happymh.com";
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

const FILTER_GENRE: [&str; 132] = [
	"",
	"rexue",
	"gedou",
	"wuxia",
	"mohuan",
	"mofa",
	"maoxian",
	"aiqing",
	"gaoxiao",
	"xiaoyuan",
	"kehuan",
	"hougong",
	"lizhi",
	"zhichang",
	"meishi",
	"shehui",
	"heidao",
	"zhanzheng",
	"lishi",
	"xuanyi",
	"jingji",
	"tiyu",
	"kongbu",
	"tuili",
	"shenghuo",
	"weiniang",
	"zhiyu",
	"shengui",
	"sige",
	"baihe",
	"danmei",
	"wudao",
	"zhentan",
	"zhainan",
	"yinyue",
	"mengxi",
	"gufeng",
	"lianai",
	"dushi",
	"xingzhuan",
	"chuanyue",
	"youxi",
	"qita",
	"aiqi",
	"richang",
	"fuhei",
	"guzhuang",
	"xianxia",
	"shenghua",
	"xiuxian",
	"qinggan",
	"gaibian",
	"chunai",
	"weimei",
	"qiangwei",
	"mingxing",
	"lieqi",
	"qingchun",
	"huanxiang",
	"jingqi",
	"caihong",
	"qiwen",
	"quanmou",
	"zhaidou",
	"xianzhiji",
	"zhuangbi",
	"langman",
	"ouxiang",
	"danvzhu",
	"fuchou",
	"nuexin",
	"egao",
	"lingyi",
	"jingxian",
	"chongai",
	"nixi",
	"yaoguai",
	"aimei",
	"tongren",
	"jiakong",
	"zhenren",
	"dongzuo",
	"juwei",
	"gongdou",
	"naodong",
	"mangai",
	"zhandou",
	"sangshi",
	"meishaonv",
	"guaiwu",
	"xitong",
	"zhidou",
	"jijia",
	"gaotian",
	"jiangshi",
	"zhiyu",
	"dianjing",
	"shenmo",
	"yineng",
	"mori",
	"yinv",
	"haokuai",
	"qihuan",
	"shenshi",
	"zhengnengliang",
	"gongting",
	"qinqing",
	"yangcheng",
	"juqing",
	"qingxiaoshuo",
	"anhei",
	"changtiao",
	"xuanhuan",
	"bazong",
	"ouhuang",
	"shengcun",
	"yishijie",
	"qita",
	"C99",
	"jiecao",
	"AA",
	"yingshihua",
	"oufeng",
	"nvshen",
	"shuanggan",
	"zhuansheng",
	"yixing",
	"fantaolu",
	"shuangnanzhu",
	"wudiliu",
	"xingzhuanhuan",
	"zhongsheng",
];
const FILTER_AREA: [&str; 7] = ["", "china", "japan", "hongkong", "europe", "korea", "other"];
const FILTER_AUDIENCE: [&str; 6] = ["", "shaonian", "shaonv", "qingnian", "BL", "GL"];
const FILTER_STATUS: [&str; 3] = ["-1", "0", "1"];
const FILTER_ORDER: [&str; 2] = ["last_date", "views"];

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	let mut genre = String::new();
	let mut area = String::new();
	let mut audience = String::new();
	let mut status = String::from("-1");
	let mut order = String::from("last_date");

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				query = filter.value.as_string()?.read();
			}
			FilterType::Select => {
				let index = filter.value.as_int()? as usize;
				match filter.name.as_str() {
					"类型" => {
						genre = FILTER_GENRE[index].to_string();
					}
					"地区" => {
						area = FILTER_AREA[index].to_string();
					}
					"读者" => {
						audience = FILTER_AUDIENCE[index].to_string();
					}
					"状态" => {
						status = FILTER_STATUS[index].to_string();
					}
					_ => continue,
				}
			}
			FilterType::Sort => {
				let value = match filter.value.as_object() {
					Ok(value) => value,
					Err(_) => continue,
				};
				let index = value.get("index").as_int()? as usize;
				order = FILTER_ORDER[index].to_string();
			}
			_ => continue,
		}
	}

	let request = if query.is_empty() {
		let url = format!(
			"{}/apis/c/index?genre={}&area={}&audience={}&series_status={}&order={}&pn={}",
			WWW_URL, genre, area, audience, status, order, page
		);
		Request::new(url, HttpMethod::Get).header("Referer", &format!("{}/latest", WWW_URL))
	} else {
		let url = format!("{}/v2.0/apis/manga/ssearch", WWW_URL);
		let body = format!("searchkey={}&v=v2.13", query);
		Request::new(url, HttpMethod::Post)
			.header("Content-Type", "application/x-www-form-urlencoded")
			.header("Referer", &format!("{}/sssearch", WWW_URL))
			.body(body.as_bytes())
	};
	let json = request
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL)
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("items").as_array()?;
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let id = item.get("manga_code").as_string()?.read();
		let cover = item.get("cover").as_string()?.read();
		let title = item.get("name").as_string()?.read();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: true,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut name = String::new();

	match listing.name.as_str() {
		"日阅读" => {
			name.push_str("day");
		}
		"日收藏" => {
			name.push_str("dayBookcases");
		}
		"周阅读" => {
			name.push_str("week");
		}
		"周收藏" => {
			name.push_str("weekBookcase");
		}
		"月阅读" => {
			name.push_str("month");
		}
		"月收藏" => {
			name.push_str("monthBookcases");
		}
		"总评分" => {
			name.push_str("voteRank");
		}
		"月投票" => {
			name.push_str("voteNumMonthRank");
		}
		_ => return get_manga_list(Vec::new(), page),
	}

	let url = format!("{}/rank/{}", WWW_URL, name);
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("Referer", &url)
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL)
		.html()?;

	let list = html.select(".manga-rank").array();
	let mut mangas: Vec<Manga> = Vec::new();

	for item in list {
		let item = match item.as_node() {
			Ok(node) => node,
			Err(_) => continue,
		};
		let id = item
			.select(".manga-rank-cover>a")
			.attr("href")
			.read()
			.split("/")
			.filter(|a| !a.is_empty())
			.map(|a| a.to_string())
			.collect::<Vec<String>>()
			.pop()
			.unwrap();
		let cover = item
			.select(".manga-rank-cover>a>mip-img")
			.attr("src")
			.read();
		let title = item.select(".manga-title").text().read().trim().to_string();
		mangas.push(Manga {
			id,
			cover,
			title,
			..Default::default()
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: false,
	})
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	let url = format!("{}/manga/{}", WWW_URL, id.clone());
	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("Referer", &format!("{}/latest", WWW_URL))
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL)
		.html()?;
	let cover = html.select(".mg-cover>mip-img").attr("src").read();
	let title = html.select("h2.mg-title").text().read();
	let author = html
		.select(".mg-sub-title>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>()
		.join(", ");
	let artist = String::new();
	let description = html.select("#showmore").text().read().trim().to_string();
	let categories = html
		.select(".mg-cate>a")
		.array()
		.map(|a| a.as_node().unwrap().text().read())
		.collect::<Vec<String>>();
	let status = MangaStatus::Unknown;
	let nsfw = MangaContentRating::Safe;
	let viewer = MangaViewer::Scroll;

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories,
		status,
		nsfw,
		viewer,
	})
}

fn get_chapter_list_by_page(id: String, page: i32) -> Result<Vec<ObjectRef>> {
	let url = format!(
		"{}/v2.0/apis/manga/chapterByPage?code={}&page={}&lang=cn&order=asc",
		WWW_URL, id, page
	);
	let json = Request::new(url, HttpMethod::Get)
		.header("Referer", &format!("{}/manga/{}", WWW_URL, id))
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL)
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let is_end = data.get("isEnd").as_int()?;
	let items = data.get("items").as_array()?;
	let mut list = items
		.map(|a| a.as_object().unwrap())
		.collect::<Vec<ObjectRef>>();

	if is_end == 1 {
		return Ok(list);
	}

	let mut next_list = get_chapter_list_by_page(id, page + 1)?;
	list.append(&mut next_list);

	Ok(list)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	let list = get_chapter_list_by_page(id, 1)?;
	let mut chapters: Vec<Chapter> = Vec::new();

	for (index, item) in list.iter().enumerate() {
		let item = match item.0.clone().as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let id = item.get("codes").as_string()?.read();
		let title = item.get("chapterName").as_string()?.read();
		let chapter = (index + 1) as f32;
		let url = format!("{}/mangaread/{}", WWW_URL, id.clone());
		chapters.push(Chapter {
			id,
			title,
			chapter,
			url,
			..Default::default()
		});
	}
	chapters.reverse();

	Ok(chapters)
}

#[get_page_list]
fn get_page_list(_: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!(
		"{}/v2.0/apis/manga/reading?code={}&v=v3.1818134",
		WWW_URL,
		chapter_id.clone()
	);
	let json = Request::new(url, HttpMethod::Get)
		.header(
			"Referer",
			&format!("{}/mangaread/{}", WWW_URL, chapter_id.clone()),
		)
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL)
		.header("X-Requested-With", "XMLHttpRequest")
		.json()?;
	let data = json.as_object()?;
	let data = data.get("data").as_object()?;
	let list = data.get("scans").as_array()?;
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in list.enumerate() {
		let item = match item.as_object() {
			Ok(item) => item,
			Err(_) => continue,
		};
		let index = index as i32;
		let url = item.get("url").as_string()?.read();
		pages.push(Page {
			index,
			url,
			..Default::default()
		})
	}

	Ok(pages)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request
		.header("Referer", &WWW_URL)
		.header("User-Agent", &UA)
		.header("Origin", &WWW_URL);
}
