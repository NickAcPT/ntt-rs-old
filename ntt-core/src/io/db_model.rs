use chrono::{DateTime, Utc};
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

    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,

    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,

    duration: PgInterval,
    recurrence_interval: Option<PgInterval>,
}

#[derive(FromRow)]
pub struct TimeTableEntryHistoryEntry {
    time_table_entry_id: Uuid,
    time: DateTime<Utc>,
    author_id: Uuid,
    old_record: Value,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "time_table_entry_type")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TimeTableEntryType {
    Recurring,
    OneTime,
}
