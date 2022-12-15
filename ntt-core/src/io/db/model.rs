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
    name: String
}

#[derive(FromRow)]
pub(crate) struct TimeTablePermissionEntry {
    time_table_id: Uuid,
    user_id: Uuid,
    can_edit: bool
}