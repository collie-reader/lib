use chrono::{DateTime, FixedOffset};
use core::fmt::{self, Display, Formatter};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use sha1_smol::Sha1;
use std::str::FromStr;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemStatus {
    Unread,
    Read,
}

impl Display for ItemStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unread => write!(f, "unread"),
            Self::Read => write!(f, "read"),
        }
    }
}

impl FromStr for ItemStatus {
    type Err = Error;

    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        match x {
            "unread" => Ok(Self::Unread),
            "read" => Ok(Self::Read),
            _ => Err(Error::InvalidEnumKey(
                x.to_string(),
                "ItemStatus".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ItemFeed {
    pub id: i32,
    pub title: String,
    pub link: String,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub id: i32,
    pub fingerprint: String,
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub link: String,
    pub status: ItemStatus,
    pub is_saved: bool,
    pub published_at: DateTime<FixedOffset>,
    pub feed: ItemFeed,
}

impl From<&Row<'_>> for Item {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            fingerprint: row.get_unwrap("fingerprint"),
            author: row.get_unwrap("author"),
            title: row.get_unwrap("title"),
            description: row.get_unwrap("description"),
            link: row.get_unwrap("link"),
            status: ItemStatus::from_str(&row.get_unwrap::<&str, String>("status")).unwrap(),
            is_saved: row.get_unwrap("is_saved"),
            published_at: row.get_unwrap("published_at"),
            feed: ItemFeed {
                id: row.get_unwrap("feed_id"),
                title: row.get_unwrap("feed_title"),
                link: row.get_unwrap("feed_link"),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ItemToCreate {
    pub author: Option<String>,
    pub title: String,
    pub description: String,
    pub link: String,
    pub status: ItemStatus,
    pub published_at: DateTime<FixedOffset>,
    pub feed: i32,
}

impl ItemToCreate {
    pub fn fingerprint(&self) -> String {
        Sha1::from(format!("{}:{}", &self.title, &self.link)).hexdigest()
    }
}

#[derive(Deserialize)]
pub struct ItemToUpdate {
    pub id: i32,
    pub status: Option<ItemStatus>,
    pub is_saved: Option<bool>,
}

#[derive(Deserialize)]
pub struct ItemToUpdateAll {
    pub status: Option<ItemStatus>,
    pub is_saved: Option<bool>,
    pub opt: Option<ItemReadOption>,
}

#[derive(Deserialize)]
pub enum ItemOrder {
    ReceivedDateDesc,
    PublishedDateDesc,
    UnreadFirst,
}

#[derive(Deserialize)]
pub struct ItemReadOption {
    pub ids: Option<Vec<i32>>,
    pub feed: Option<i32>,
    pub status: Option<ItemStatus>,
    pub is_saved: Option<bool>,
    pub order_by: Option<ItemOrder>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
