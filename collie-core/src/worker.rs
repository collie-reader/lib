use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use std::collections::HashMap;

use crate::error::Result;
use crate::model::feed::FeedStatus;
use crate::model::feed::FeedToUpdate;
use crate::model::item::ItemOrder;
use crate::model::item::ItemReadOption;
use crate::model::item::ItemStatus;
use crate::model::item::ItemToCreate;
use crate::model::syndication::RawItem;
use crate::repository::database::DbConnection;
use crate::service::feed;
use crate::service::item;

pub struct Worker {
    conn: DbConnection,
    proxy: Option<String>,
}

impl Worker {
    pub fn new(conn: DbConnection, proxy: Option<String>) -> Self {
        Self { conn, proxy }
    }

    pub async fn execute(&self) -> Result<Vec<ItemToCreate>> {
        let pairs = self.get_links_to_check();

        let mut inserted = vec![];

        let feed_ids_to_check: Vec<i32> = pairs
            .iter()
            .filter_map(|(id, _, fetch_old_items)| if !fetch_old_items { Some(*id) } else { None })
            .collect();

        let most_recent_items = if !feed_ids_to_check.is_empty() {
            self.get_most_recent_items(&feed_ids_to_check)
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        for (feed, link, fetch_old_items) in pairs {
            let items = item::fetch(&link, self.proxy.as_deref()).await?;

            let mut filtered_items = if !fetch_old_items && !most_recent_items.contains_key(&feed) {
                items
                    .into_iter()
                    .max_by_key(|x| x.published_at)
                    .into_iter()
                    .collect()
            } else {
                items
                    .into_iter()
                    .filter(|item| {
                        most_recent_items.get(&feed).map_or(true, |most_recent| {
                            item.published_at
                                .map_or(false, |published_at| published_at > *most_recent)
                        }) || fetch_old_items
                    })
                    .collect::<Vec<_>>()
            };

            filtered_items.sort_by_key(|x| x.published_at);
            inserted.extend(self.insert_new_items(feed, &filtered_items));
        }

        Ok(inserted)
    }

    fn get_links_to_check(&self) -> Vec<(i32, String, bool)> {
        if let Ok(feeds) = feed::read_all(&self.conn) {
            let current = Utc::now().fixed_offset();
            let filtered = feeds.iter().filter(|x| x.status == FeedStatus::Subscribed);

            filtered
                .map(|x| {
                    let _ = feed::update(
                        &self.conn,
                        &(FeedToUpdate {
                            id: x.id,
                            title: None,
                            link: None,
                            status: None,
                            checked_at: Some(current),
                            fetch_old_items: None,
                        }),
                    );
                    (x.id, x.link.clone(), x.fetch_old_items)
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn insert_new_items(&self, feed: i32, items: &[RawItem]) -> Vec<ItemToCreate> {
        let current = Utc::now().fixed_offset();

        let args = items.iter().map(|x| ItemToCreate {
            author: x.author.clone().map(|x| x.trim().to_string()),
            title: x.title.trim().to_string(),
            link: x.link.clone().unwrap_or("#".to_string()).trim().to_string(),
            description: x.content.clone().unwrap_or_default().trim().to_string(),
            status: ItemStatus::Unread,
            published_at: x.published_at.unwrap_or(current),
            feed,
        });

        let mut inserted = vec![];
        for arg in args {
            if item::create(&self.conn, &arg).is_ok() {
                inserted.push(arg);
            }
        }

        inserted
    }

    fn get_most_recent_items(
        &self,
        feed_ids: &[i32],
    ) -> Result<HashMap<i32, DateTime<FixedOffset>>> {
        let mut most_recent_items = HashMap::new();

        for feed_id in feed_ids {
            let opt = ItemReadOption {
                ids: None,
                feed: Some(*feed_id),
                status: None,
                is_saved: None,
                order_by: Some(ItemOrder::PublishedDateDesc),
                limit: Some(1),
                offset: None,
            };

            if let Some(item) = item::read_all(&self.conn, &opt)?.first() {
                most_recent_items.insert(item.feed.id, item.published_at);
            }
        }

        Ok(most_recent_items)
    }
}
