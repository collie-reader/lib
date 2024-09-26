use scraper::{Html, Selector};

use crate::{
    error::{Error, Result},
    model::{
        feed::{Feed, FeedToCreate, FeedToUpdate},
        syndication::Feed as SyndicationFeed,
    },
    repository::{database::DbConnection, feed},
    util::fetcher,
};

pub async fn create(conn: &DbConnection, arg: &FeedToCreate, proxy: Option<&str>) -> Result<usize> {
    if arg.link.is_empty() {
        return Err(Error::BadArgument);
    }

    let html_content = fetcher::get(&arg.link, proxy).await.unwrap();
    let is_feed = html_content.parse::<SyndicationFeed>().is_ok();

    let link = if is_feed {
        arg.link.clone()
    } else if let Some(link) = parse_link(&html_content)? {
        link
    } else {
        return Err(Error::FeedNotFound);
    };

    let title = fetch_title(&link, proxy).await?;

    let arg = FeedToCreate {
        title,
        link,
        fetch_old_items: arg.fetch_old_items,
    };

    feed::create(conn, &arg)
}

pub fn read_all(conn: &DbConnection) -> Result<Vec<Feed>> {
    feed::read_all(conn)
}

pub fn read(conn: &DbConnection, id: i32) -> Result<Option<Feed>> {
    feed::read(conn, id)
}

pub fn update(conn: &DbConnection, arg: &FeedToUpdate) -> Result<usize> {
    feed::update(conn, arg)
}

pub fn delete(conn: &DbConnection, id: i32) -> Result<usize> {
    feed::delete(conn, id)
}

pub fn parse_link(html: &str) -> Result<Option<String>> {
    let document = Html::parse_document(html);
    let selector =
        Selector::parse("link[type='application/rss+xml'], link[type='application/atom+xml']")
            .unwrap();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            return Ok(Some(href.to_string()));
        }
    }

    Ok(None)
}

pub async fn fetch_title(link: &str, proxy: Option<&str>) -> Result<String> {
    let content = fetcher::get(link, proxy).await?;
    match content.parse::<SyndicationFeed>()? {
        SyndicationFeed::Atom(atom) => Ok(atom.title().to_string()),
        SyndicationFeed::RSS(rss) => Ok(rss.title().to_string()),
    }
}
