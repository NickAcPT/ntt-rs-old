use chrono::{DateTime, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::types::PgInterval;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    pub(crate) is_admin: bool,
    pub(crate) is_approved: bool,

    pub(crate) external_id: u64,
}

#[derive(FromRow)]
pub struct TimeTable {
    id: Uuid,
    owner_id: Uuid,
    name: String,
}

#[derive(FromRow)]
pub struct TimeTablePermissionEntry {
    time_table_id: Uuid,
    user_id: Uuid,
    can_edit: bool,
}

#[derive(FromRow)]
pub struct TimeTableEntry {
    time_table_id: Uuid,
    #[sqlx(rename = "type")]
    entry_type: TimeTableEntryType,
    author_id: Uuid,

    name: String,
    description: Option<String>,

    start_time: DateTime<Utc>,
    duration: PgInterval,

    repeating_data_id: Option<Uuid>,
}

#[derive(FromRow)]
pub struct TimeTableEntryHistoryEntry {
    time_table_entry_id: Uuid,
    time: DateTime<Utc>,
    author_id: Uuid,
    old_record: Value,
}

#[derive(FromRow)]
pub struct TimeTableEntryRepeatingData {
    id: Uuid,
    entry_start_date: NaiveDate,
    entry_end_date: NaiveDate,
    weekly_repeating_interval: Vec<WeekDay>,
}
/// The u8 represents the day of the week where 0 is Sunday. This is used in the [Javascript Date object](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay).
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
impl From<chrono::Weekday> for WeekDay{
    fn from(value: Weekday) -> Self {
        match value {
            Weekday::Sun => WeekDay::Sunday,
            Weekday::Mon => WeekDay::Monday,
            Weekday::Tue => WeekDay::Tuesday,
            Weekday::Wed => WeekDay::Wednesday,
            Weekday::Thu => WeekDay::Thursday,
            Weekday::Fri => WeekDay::Friday,
            Weekday::Sat => WeekDay::Saturday,
        }
    }
}
impl From<WeekDay> for chrono::Weekday{
    fn from(value: WeekDay) -> Self {
        match value {
            WeekDay::Sunday => Weekday::Sun,
            WeekDay::Monday => Weekday::Mon,
            WeekDay::Tuesday => Weekday::Tue,
            WeekDay::Wednesday => Weekday::Wed,
            WeekDay::Thursday => Weekday::Thu,
            WeekDay::Friday => Weekday::Fri,
            WeekDay::Saturday => Weekday::Sat,
        }
    }
}
#[derive(sqlx::Type)]
#[sqlx(type_name = "time_table_entry_type")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TimeTableEntryType {
    Recurring,
    OneTime,
}
