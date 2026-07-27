#![no_std]

mod html;
mod json;
mod net;
mod setting;
mod time;

use aidoku::{
	Chapter, DeepLinkHandler, DeepLinkResult, DynamicFilters, Filter, FilterValue, HashMap, Home,
	HomeLayout, ImageRequestProvider, ImageResponse, Listing, ListingProvider, Manga,
	MangaPageResult, NotificationHandler, Page, PageContext, PageImageProcessor, Result, Source,
	WebLoginHandler,
	alloc::{String, Vec},
	bail, error,
	imports::{
		canvas::{Canvas, ImageRef, Rect},
		net::Request,
		std::send_partial_result,
	},
	register_source,
};
use html::{
	ChapterPage as _, FiltersPage as _, HomePage as _, MangaPage as _, TryElement as _,
	TrySelector as _,
};
use json::{chapter_list, daily_update, manga_page_result, random};
use net::Url;
use setting::change_charset;
use time::DayOfWeek;

struct Boylove;

impl Source for Boylove {
	fn new() -> Self {
		Self
	}

	fn get_search_manga_list(
		&self,
		query: Option<String>,
		page: i32,
		filters: Vec<FilterValue>,
	) -> Result<MangaPageResult> {
		let manga_page_result = Url::from_query_or_filters(query.as_deref(), page, &filters)?
			.request()?
			.json_owned::<manga_page_result::Root>()?
			.into();
		Ok(manga_page_result)
	}

	fn get_manga_update(
		&self,
		mut manga: Manga,
		needs_details: bool,
		needs_chapters: bool,
	) -> Result<Manga> {
		if needs_details {
			let updated_details = Url::manga(&manga.key)
				.request()?
				.html()?
				.manga_details(&manga.key)?;

			manga = Manga {
				chapters: manga.chapters,
				..updated_details
			};

			if needs_chapters {
				send_partial_result(&manga);
			} else {
				return Ok(manga);
			}
		}

		let chapters = Url::chapter_list(&manga.key)
			.request()?
			.json_owned::<chapter_list::Root>()?
			.into();
		manga.chapters = Some(chapters);

		Ok(manga)
	}

	fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
		Url::chapter(&chapter.key).request()?.html()?.pages()
	}
}

impl ImageRequestProvider for Boylove {
	fn get_image_request(&self, url: String, _context: Option<PageContext>) -> Result<Request> {
		Ok(Request::get(url)?.header("Referer", "https://boylove.cc/"))
	}
}

impl PageImageProcessor for Boylove {
	fn process_page_image(
		&self,
		response: ImageResponse,
		context: Option<PageContext>,
	) -> Result<ImageRef> {
		let columns = context
			.as_ref()
			.and_then(|context| context.get("columns"))
			.and_then(|value| value.parse::<i32>().ok())
			.unwrap_or(1);

		let width = response.image.width();
		let height = response.image.height();
		if columns <= 1 || height >= 4000.0 {
			return Ok(response.image);
		}

		// The reader currently splits short images into `randomClass` vertical
		// strips. The first n-1 strips are stored in reverse order, while the
		// remainder strip is stored at x=0. Mirror the site's canvas routine.
		let unit_width = (width / columns as f32) as i32 as f32;
		let mut canvas = Canvas::new(width, height);
		for column in 1..=columns {
			let destination_x = unit_width * (column - 1) as f32;
			let (source_x, strip_width) = if column == columns {
				(0.0, width - unit_width * (columns - 1) as f32)
			} else {
				(width - unit_width * column as f32, unit_width)
			};
			let source = Rect::new(source_x, 0.0, strip_width, height);
			let destination = Rect::new(destination_x, 0.0, strip_width, height);
			canvas.copy_image(&response.image, source, destination);
		}
		Ok(canvas.get_image())
	}
}

impl DeepLinkHandler for Boylove {
	fn handle_deep_link(&self, url: String) -> Result<Option<DeepLinkResult>> {
		let mut splits = url.split('/').skip(3);
		let deep_link_result = match (
			splits.next(),
			splits.next(),
			splits.next(),
			splits.next(),
			splits.next(),
		) {
			(Some("home"), Some("book"), Some("index"), Some("id"), Some(key)) => {
				Some(DeepLinkResult::Manga { key: key.into() })
			}

			(Some("home"), Some("book"), Some("capter"), Some("id"), Some(key)) => {
				let path = Url::chapter(key)
					.request()?
					.html()?
					.try_select_first("a.back")?
					.try_attr("href")?;
				let manga_key = path
					.rsplit_once('/')
					.ok_or_else(|| error!("Character not found: `/`"))?
					.1;

				Some(DeepLinkResult::Chapter {
					manga_key: manga_key.into(),
					key: key.into(),
				})
			}

			(Some("home"), Some("index"), Some("dailyupdate1"), None, None) => {
				let day_of_week = DayOfWeek::today()?;
				let id = day_of_week.as_id().into();

				let name = day_of_week.as_name().into();

				Some(DeepLinkResult::Listing(Listing {
					id,
					name,
					..Default::default()
				}))
			}

			(
				Some("home"),
				Some("index"),
				Some("dailyupdate1"),
				Some("weekday"),
				Some(day_of_week),
			) => {
				let id = day_of_week.into();

				let name = match day_of_week {
					"11" => "最新",
					"0" => "週一",
					"1" => "週二",
					"2" => "週三",
					"3" => "週四",
					"4" => "週五",
					"5" => "週六",
					"6" => "週日",
					_ => return Ok(None),
				}
				.into();

				Some(DeepLinkResult::Listing(Listing {
					id,
					name,
					..Default::default()
				}))
			}

			(
				Some("home"),
				Some("index"),
				Some("pages"),
				Some("w"),
				Some("recommend.html" | "recommend"),
			) => Some(DeepLinkResult::Listing(Listing {
				id: "recommend".into(),
				name: "無碼專區".into(),
				..Default::default()
			})),

			(
				Some("home"),
				Some("index"),
				Some("pages"),
				Some("w"),
				Some("topestmh.html" | "topestmh"),
			) => Some(DeepLinkResult::Listing(Listing {
				id: "topestmh".into(),
				name: "排行榜".into(),
				..Default::default()
			})),

			_ => None,
		};
		Ok(deep_link_result)
	}
}

impl DynamicFilters for Boylove {
	fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
		let tags = Url::FiltersPage.request()?.html()?.tags_filter()?;

		let filters = [tags].into();
		Ok(filters)
	}
}

impl Home for Boylove {
	fn get_home(&self) -> Result<HomeLayout> {
		Url::Home.request()?.html()?.home_layout()
	}
}

impl ListingProvider for Boylove {
	fn get_manga_list(&self, listing: Listing, page: i32) -> Result<MangaPageResult> {
		let manga_page_result = match listing.name.as_str() {
			"最新" | "週一" | "週二" | "週三" | "週四" | "週五" | "週六" | "週日" => {
				Url::daily_update(&listing.id, page)
					.request()?
					.json_owned::<daily_update::Root>()?
					.into()
			}

			"無碼專區" | "排行榜" => Url::listing(&listing.id, page)
				.request()?
				.json_owned::<manga_page_result::Root>()?
				.into(),

			"猜你喜歡" => Url::random()
				.request()?
				.json_owned::<random::Root>()?
				.into(),

			name => bail!("Invalid listing name: `{name}`"),
		};
		Ok(manga_page_result)
	}
}

impl NotificationHandler for Boylove {
	fn handle_notification(&self, notification: String) {
		if notification == "updatedCharset" {
			_ = change_charset();
		}
	}
}

impl WebLoginHandler for Boylove {
	fn handle_web_login(&self, key: String, cookies: HashMap<String, String>) -> Result<bool> {
		if key != "login" {
			bail!("Invalid login key: `{key}`");
		}

		let is_logged_in = cookies.get("rfv").is_some();
		Ok(is_logged_in)
	}
}

register_source!(
	Boylove,
	DeepLinkHandler,
	DynamicFilters,
	Home,
	ListingProvider,
	NotificationHandler,
	ImageRequestProvider,
	PageImageProcessor,
	WebLoginHandler
);

#[cfg(test)]
type LiveAuditSource = Boylove;
#[cfg(test)]
#[path = "../../live_audit.rs"]
mod live_audit;
