use chrono::NaiveDateTime;
use serde::Serialize;

use crate::schema::notes;

#[derive(Debug, Clone, Queryable, Serialize, QueryableByName)]
pub struct Note {
    pub id: i32,
    pub note_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_updated: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub note_id: i32,
    pub title: &'a str,
    pub content: &'a str,
    pub last_updated: NaiveDateTime
}