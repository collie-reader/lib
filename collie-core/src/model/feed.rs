use chrono::{DateTime, FixedOffset};
use core::fmt;
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FeedStatus {
    Subscribed,
    Unsubscribed,
}

impl Display for FeedStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FeedStatus::Subscribed => write!(f, "subscribed"),
            FeedStatus::Unsubscribed => write!(f, "unsubscribed"),
        }
    }
}

impl FromStr for FeedStatus {
    type Err = Error;

    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        match x {
            "subscribed" => Ok(Self::Subscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            _ => Err(Error::InvalidEnumKey(
                x.to_string(),
                "FeedStatus".to_string(),
            )),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Feed {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub status: FeedStatus,
    pub checked_at: DateTime<FixedOffset>,
    pub fetch_old_items: bool,
}

impl From<&Row<'_>> for Feed {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get_unwrap("id"),
            title: row.get_unwrap("title"),
            link: row.get_unwrap("link"),
            status: FeedStatus::from_str(&row.get_unwrap::<&str, String>("status")).unwrap(),
            checked_at: row.get_unwrap("checked_at"),
            fetch_old_items: row.get_unwrap("fetch_old_items"),
        }
    }
}

#[derive(Deserialize)]
pub struct FeedToCreate {
    pub title: String,
    pub link: String,
    pub fetch_old_items: bool,
}

#[derive(Deserialize)]
pub struct FeedToUpdate {
    pub id: i32,
    pub title: Option<String>,
    pub link: Option<String>,
    pub status: Option<FeedStatus>,
    pub checked_at: Option<DateTime<FixedOffset>>,
    pub fetch_old_items: Option<bool>,
}
