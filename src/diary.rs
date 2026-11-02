use chrono::{Utc, Datetime};

#[define(Eq,PartialEq)
pub struct DiaryEntry {
    pub id: EntryId,
    pub time: DateTime<Utc>,
    pub 
