use chrono::prelude::*;

// Define a distinct type
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(pub u64);

#[derive(Eq, PartialEq)]
pub struct DiaryEntry {
    id: EntryId,
    time: DateTime<Utc>,
    author: String,
    contents: String,
}

impl DiaryEntry {
    pub fn new(id: EntryId, time: DateTime<Utc>, author: String, contents: String) -> Self {
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

    pub fn get_id(self: &Self) -> EntryId {
        return self.id;
    }

    pub fn get_time(self: &Self) -> DateTime<Utc> {
        return self.time;
    }
}
