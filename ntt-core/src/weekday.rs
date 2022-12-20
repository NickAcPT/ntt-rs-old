use chrono::Weekday as ChronoWeekday;
use serde::{Deserialize, Serialize};

/// The u8 represents the day of the week where 0 is Sunday.
/// This is used in the [Javascript Date object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay).
#[derive(Debug, Clone, PartialEq, Eq, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "week_day")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeekDay {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}
impl From<ChronoWeekday> for WeekDay {
    fn from(value: ChronoWeekday) -> Self {
        match value {
            ChronoWeekday::Sun => WeekDay::Sunday,
            ChronoWeekday::Mon => WeekDay::Monday,
            ChronoWeekday::Tue => WeekDay::Tuesday,
            ChronoWeekday::Wed => WeekDay::Wednesday,
            ChronoWeekday::Thu => WeekDay::Thursday,
            ChronoWeekday::Fri => WeekDay::Friday,
            ChronoWeekday::Sat => WeekDay::Saturday,
        }
    }
}
impl From<WeekDay> for ChronoWeekday {
    fn from(value: WeekDay) -> Self {
        match value {
            WeekDay::Sunday => ChronoWeekday::Sun,
            WeekDay::Monday => ChronoWeekday::Mon,
            WeekDay::Tuesday => ChronoWeekday::Tue,
            WeekDay::Wednesday => ChronoWeekday::Wed,
            WeekDay::Thursday => ChronoWeekday::Thu,
            WeekDay::Friday => ChronoWeekday::Fri,
            WeekDay::Saturday => ChronoWeekday::Sat,
        }
    }
}
