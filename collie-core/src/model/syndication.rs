use chrono::{DateTime, FixedOffset};
use std::str::FromStr;

use crate::error::Error;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RawItem {
    pub title: String,
    pub author: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<FixedOffset>>,
}

// borrowed from https://github.com/rust-syndication/syndication

#[derive(Clone)]
pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for Feed {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match atom_syndication::Feed::from_str(s) {
            Ok(feed) => Ok(Self::Atom(feed)),
            Err(_) => match rss::Channel::from_str(s) {
                Ok(channel) => Ok(Self::RSS(channel)),
                Err(_) => Err(Error::SyndicationParsingFailure),
            },
        }
    }
}
