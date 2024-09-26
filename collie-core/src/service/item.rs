use chrono::{DateTime, Utc};

use crate::{
    error::Result,
    model::{
        item::{Item, ItemReadOption, ItemToCreate, ItemToUpdate, ItemToUpdateAll},
        syndication::{Feed as SyndicationFeed, RawItem},
    },
    repository::{database::DbConnection, item},
    util::fetcher,
};

pub fn create(conn: &DbConnection, arg: &ItemToCreate) -> Result<usize> {
    item::create(conn, arg)
}

pub fn read_all(conn: &DbConnection, opt: &ItemReadOption) -> Result<Vec<Item>> {
    item::read_all(conn, opt)
}

pub fn count_all(conn: &DbConnection, opt: &ItemReadOption) -> Result<i64> {
    item::count_all(conn, opt)
}

pub fn update(conn: &DbConnection, arg: &ItemToUpdate) -> Result<usize> {
    item::update(conn, arg)
}

pub fn update_all(conn: &DbConnection, arg: &ItemToUpdateAll) -> Result<usize> {
    item::update_all(conn, arg)
}

pub async fn fetch(link: &str, proxy: Option<&str>) -> Result<Vec<RawItem>> {
    let content = fetcher::get(link, proxy).await?;
    match content.parse::<SyndicationFeed>()? {
        SyndicationFeed::Atom(atom) => Ok(atom
            .entries()
            .iter()
            .map(|x| RawItem {
                title: x.title().to_string(),
                author: Some(
                    x.authors()
                        .iter()
                        .map(|x| x.name().trim())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                link: x.links().first().map(|x| x.href().trim().to_string()),
                content: x
                    .content()
                    .map(atom_syndication::Content::value)
                    .filter(std::option::Option::is_some)
                    .map(|x| x.unwrap().trim().to_string()),
                published_at: x
                    .published()
                    .or(Some(x.updated()))
                    .map(|x| x.with_timezone(&Utc).fixed_offset()),
            })
            .collect()),
        SyndicationFeed::RSS(rss) => Ok(rss
            .items()
            .iter()
            .map(|x| RawItem {
                title: x.title().unwrap_or("Untitled").trim().to_string(),
                author: x
                    .author()
                    .map(|x| x.trim().to_string())
                    .or(x.dublin_core_ext().map(|x| x.creators().join(","))),
                link: x.link().map(std::string::ToString::to_string),
                content: x.description().map(std::string::ToString::to_string),
                published_at: x
                    .pub_date()
                    .map(|x| {
                        DateTime::parse_from_rfc2822(x)
                            .map(|x| x.with_timezone(&Utc).fixed_offset())
                    })
                    .filter(std::result::Result::is_ok)
                    .map(std::result::Result::unwrap),
            })
            .collect()),
    }
}
