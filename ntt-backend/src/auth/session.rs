use chrono::{DateTime, Local};
use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use uuid::Uuid;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub user_id: Uuid,
    pub expires_at: DateTime<Local>,
}
// A basic in memory session store.
#[derive(Debug, Default)]
pub struct SessionManager {
    pub(crate) sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_session(&mut self, user_id: Uuid) -> String {
        let session_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        self.sessions.insert(
            session_id.clone(),
            Session {
                user_id,
                expires_at: Local::now() + chrono::Duration::days(31),
            },
        );
        session_id
    }
    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }
    pub fn delete_session(&mut self, session_id: &str) {
        self.sessions.remove(session_id);
    }
}
