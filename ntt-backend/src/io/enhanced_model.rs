use chrono::{DateTime, Duration, NaiveDate, Utc};
use ntt_core::io::db_model::TimeTableEntryType;
use ntt_core::weekday::WeekDay;
use serde::{Serialize, Serializer};
use typeshare::typeshare;
use uuid::Uuid;

// An "enhanced" database model contains additional information
// that is stored in other tables which then joined together.
// This enhanced model is used to serialize the data into JSON format and send it to the frontend.
#[inline(always)]
fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.num_seconds().serialize(serializer)
}

/// A user that is stored in the database.
#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct EnhancedUser {
    /// The user's ID.
    id: Uuid,
    /// The user's username.
    name: String,
    /// The user's time tables.
    timetables: Vec<EnhancedTimeTable>,
}

/// A time table that is stored in the database.
/// Permissions are not included in this struct since by this point they have already been checked.
#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct EnhancedTimeTable {
    /// The time table's ID.
    id: Uuid,
    /// The time table's name.
    name: String,
    /// The time table's owner.
    owner: Uuid,
    /// Entries in the time table.
    entries: Vec<EnhancedTimeTableEntry>,
}

/// A time table entry that is stored in the database.
#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct EnhancedTimeTableEntry {
    /// The time table entry's ID.
    id: Uuid,
    /// The time table entry's type.
    /// This is used to determine which fields are valid.
    entry_type: TimeTableEntryType,
    /// The user who authored this last change to this time table entry.
    author: Uuid,

    /// The time table entry's name.
    name: String,
    /// The time table entry's description.
    description: Option<String>,

    /// The time table entry's start time.
    /// Just the time component should be used when this entry is a repeating entry.
    /// Otherwise, both the date and time components should be used.
    start_time: DateTime<Utc>,
    /// The time table entry's duration.
    #[serde(serialize_with = "serialize_duration")]
    duration: Duration,

    /// The time table entry's recurrence interval.
    /// If this is `None`, then this is a one-time entry.
    recurring_event: Option<RecurringTimeTableEntry>,

    one_time_event: Option<OneTimeTimeTableEntry>,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecurringTimeTableEntry {
    /// The date of the first occurrence of this recurring event.
    pub entry_start_date: NaiveDate,
    /// The date of the last occurrence of this recurring event. (inclusive)
    pub entry_end_date: NaiveDate,
    /// The interval between each occurrence of this event.
    pub repeats_every: RepeatingInterval,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OneTimeTimeTableEntry {
    /// The date of this one-time event.
    pub date: NaiveDate,
    /// The time of this one-time event.
    pub time: DateTime<Utc>,
}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum RepeatingInterval {
    Weekly(Vec<WeekDay>),
}
