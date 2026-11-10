use chrono::prelude::*;
use crate::utils::Id;

#[derive(Eq, PartialEq)]
pub struct DiaryEntry {
    id: Id,
    time: DateTime<Utc>,
    author: String,
    contents: String,
}

impl DiaryEntry {
    pub fn new(id: Id, time: DateTime<Utc>, author: String, contents: String) -> Self {
        return DiaryEntry {
            id: id,
            time: time,
            author: author,
            contents: contents,
        };
    }

    pub fn get_author(self: &Self) -> &str {
        return self.author.as_str();
    }

    pub fn read_contents(self: &Self) -> &str {
        return self.contents.as_str();
    }

    pub fn get_id(self: &Self) -> Id {
        return self.id;
    }

    pub fn get_time(self: &Self) -> DateTime<Utc> {
        return self.time;
    }
}
