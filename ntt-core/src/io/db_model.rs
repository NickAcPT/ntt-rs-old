use chrono::{DateTime, Utc, Weekday};
use serde::Serialize;
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
    onetime_data_id: Option<Uuid>,
}

#[derive(FromRow)]
pub struct TimeTableEntryHistoryEntry {
    time_table_entry_id: Uuid,
    time: DateTime<Utc>,
    author_id: Uuid,
    old_record: Value,
}

#[derive(FromRow)]
pub struct TimeTableEntryOneTimeData {
    id: Uuid,
    weekly_repeating_interval: Vec<WeekDay>,
}

#[derive(FromRow)]
pub struct TimeTableEntryRepeatingData {
    id: Uuid,
    entry_start_date: DateTime<Utc>,
    entry_end_date: DateTime<Utc>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "week_date")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<WeekDay> for Weekday {
    fn from(value: WeekDay) -> Self {
        match value {
            WeekDay::Monday => Weekday::Mon,
            WeekDay::Tuesday => Weekday::Tue,
            WeekDay::Wednesday => Weekday::Wed,
            WeekDay::Thursday => Weekday::Thu,
            WeekDay::Friday => Weekday::Fri,
            WeekDay::Saturday => Weekday::Sat,
            WeekDay::Sunday => Weekday::Sun,
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
