use super::*;
use aidoku::imports::std::current_date;
use chrono::{Datelike as _, FixedOffset, TimeZone as _, Weekday};

pub struct DayOfWeek(Weekday);

impl DayOfWeek {
	pub const fn as_id(&self) -> &'static str {
		match self.0 {
			Weekday::Mon => "0",
			Weekday::Tue => "1",
			Weekday::Wed => "2",
			Weekday::Thu => "3",
			Weekday::Fri => "4",
			Weekday::Sat => "5",
			Weekday::Sun => "6",
		}
	}

	pub const fn as_name(&self) -> &'static str {
		match self.0 {
			Weekday::Mon => "週一",
			Weekday::Tue => "週二",
			Weekday::Wed => "週三",
			Weekday::Thu => "週四",
			Weekday::Fri => "週五",
			Weekday::Sat => "週六",
			Weekday::Sun => "週日",
		}
	}

	pub fn today() -> Result<Self> {
		let now = current_date();
		let day_of_week = FixedOffset::east_opt(8 * 60 * 60)
			.ok_or_else(|| error!("Failed to create UTC+8 offset"))?
			.timestamp_opt(now, 0)
			.single()
			.ok_or_else(|| error!("Invalid or ambiguous timestamp: `{now}`"))?
			.weekday();
		Ok(Self(day_of_week))
	}
}
