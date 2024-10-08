use chrono::DateTime;
use pretty_assertions::assert_eq;
use std::{fs, path::PathBuf};

use crate::model::syndication::RawItem;
use crate::service::{feed, item};

fn fixture(path: &str) -> String {
    fs::canonicalize(PathBuf::from(format!("src/tests/fixtures/{}", path)))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn fetch_feed_title_rss() {
    let title = feed::fetch_title(&fixture("hnrss-org-frontpage.rss"), None)
        .await
        .unwrap();
    assert_eq!(title, "Hacker News: Front Page");
}

#[tokio::test]
async fn fetch_feed_title_atom() {
    let title = feed::fetch_title(&fixture("hnrss-org-frontpage.atom"), None)
        .await
        .unwrap();
    assert_eq!(title, "Hacker News: Front Page");
}

#[tokio::test]
async fn fetch_feed_items_rss() {
    let items = item::fetch(&fixture("hnrss-org-frontpage.rss"), None)
        .await
        .unwrap();
    assert_eq!(
        vec![
            RawItem {
                title: "Hacker Smacker: Friend/foe individual writers on Hacker News".to_string(),
                author: Some("swyx".to_string()),
                link: Some("https://github.com/samuelclay/hackersmacker".to_string()),
                content: Some("<p>Article URL: <a href=\"https://github.com/samuelclay/hackersmacker\">https://github.com/samuelclay/hackersmacker</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288627\">https://news.ycombinator.com/item?id=37288627</a></p>\n<p>Points: 36</p>\n<p># Comments: 14</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T01:33:24+00:00").unwrap()),
            },
            RawItem {
                title: "Writing Linux Modules in Ada – Part 1".to_string(),
                author: Some("slondr".to_string()),
                link: Some("http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1".to_string()),
                content: Some("<p>Article URL: <a href=\"http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1\">http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288446\">https://news.ycombinator.com/item?id=37288446</a></p>\n<p>Points: 27</p>\n<p># Comments: 5</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T01:05:24+00:00").unwrap()),
            },
            RawItem {
                title: "Federal study links testicular cancer to ‘forever chemicals’".to_string(),
                author: Some("EA-3167".to_string()),
                link: Some("https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/".to_string()),
                content: Some("<p>Article URL: <a href=\"https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/\">https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288208\">https://news.ycombinator.com/item?id=37288208</a></p>\n<p>Points: 62</p>\n<p># Comments: 15</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T00:32:34+00:00").unwrap()),
            },
        ],
        items,
    );
}

#[tokio::test]
async fn fetch_feed_items_atom() {
    let items = item::fetch(&fixture("hnrss-org-frontpage.atom"), None)
        .await
        .unwrap();
    assert_eq!(
        vec![
            RawItem {
                title: "Hacker Smacker: Friend/foe individual writers on Hacker News".to_string(),
                author: Some("swyx".to_string()),
                link: Some("https://github.com/samuelclay/hackersmacker".to_string()),
                content: Some("<p>Article URL: <a href=\"https://github.com/samuelclay/hackersmacker\">https://github.com/samuelclay/hackersmacker</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288627\">https://news.ycombinator.com/item?id=37288627</a></p>\n<p>Points: 36</p>\n<p># Comments: 14</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T01:33:24+00:00").unwrap()),
            },
            RawItem {
                title: "Writing Linux Modules in Ada – Part 1".to_string(),
                author: Some("slondr".to_string()),
                link: Some("http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1".to_string()),
                content: Some("<p>Article URL: <a href=\"http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1\">http://www.nihamkin.com/2016/10/23/writing-linux-modules-in-ada-part-1/#writing-linux-modules-in-ada-part-1</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288446\">https://news.ycombinator.com/item?id=37288446</a></p>\n<p>Points: 27</p>\n<p># Comments: 5</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T01:05:24+00:00").unwrap()),
            },
            RawItem {
                title: "Federal study links testicular cancer to ‘forever chemicals’".to_string(),
                author: Some("EA-3167".to_string()),
                link: Some("https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/".to_string()),
                content: Some("<p>Article URL: <a href=\"https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/\">https://undark.org/2023/08/22/federal-study-links-testicular-cancer-to-forever-chemicals/</a></p>\n<p>Comments URL: <a href=\"https://news.ycombinator.com/item?id=37288208\">https://news.ycombinator.com/item?id=37288208</a></p>\n<p>Points: 62</p>\n<p># Comments: 15</p>".to_string()),
                published_at: Some(DateTime::parse_from_rfc3339("2023-08-28T00:32:34+00:00").unwrap()),
            },
        ],
        items,
    );
}
