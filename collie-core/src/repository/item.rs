use sea_query::{Alias, Expr, Func, Order, Query, SqliteQueryBuilder, Values};
use sea_query_rusqlite::RusqliteBinder;

use crate::{
    error::Result,
    model::item::{
        Item, ItemOrder, ItemReadOption, ItemStatus, ItemToCreate, ItemToUpdate, ItemToUpdateAll,
    },
};

use super::database::{DbConnection, Feeds, Items};

pub fn create(conn: &DbConnection, arg: &ItemToCreate) -> Result<usize> {
    let (sql, values) = Query::insert()
        .into_table(Items::Table)
        .columns([
            Items::Fingerprint,
            Items::Author,
            Items::Title,
            Items::Description,
            Items::Link,
            Items::Status,
            Items::PublishedAt,
            Items::Feed,
        ])
        .values_panic([
            arg.fingerprint().into(),
            arg.author.clone().into(),
            arg.title.clone().into(),
            arg.description.clone().into(),
            arg.link.clone().into(),
            arg.status.to_string().into(),
            arg.published_at.into(),
            arg.feed.into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn read_all(conn: &DbConnection, opt: &ItemReadOption) -> Result<Vec<Item>> {
    let mut query = Query::select()
        .columns([
            (Items::Table, Items::Id),
            (Items::Table, Items::Fingerprint),
            (Items::Table, Items::Author),
            (Items::Table, Items::Title),
            (Items::Table, Items::Description),
            (Items::Table, Items::Link),
            (Items::Table, Items::Status),
            (Items::Table, Items::IsSaved),
            (Items::Table, Items::PublishedAt),
        ])
        .expr_as(Expr::col((Feeds::Table, Feeds::Id)), Alias::new("feed_id"))
        .expr_as(
            Expr::col((Feeds::Table, Feeds::Title)),
            Alias::new("feed_title"),
        )
        .expr_as(
            Expr::col((Feeds::Table, Feeds::Link)),
            Alias::new("feed_link"),
        )
        .from(Items::Table)
        .inner_join(
            Feeds::Table,
            Expr::col((Items::Table, Items::Feed)).equals((Feeds::Table, Feeds::Id)),
        )
        .clone();

    if let Some(ids) = &opt.ids {
        query.and_where(Expr::col(Items::Id).is_in(ids.clone()));
    }

    if let Some(feed) = &opt.feed {
        query.and_where(Expr::col(Items::Feed).eq(*feed));
    }

    if let Some(status) = &opt.status {
        query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
    }

    if let Some(is_saved) = &opt.is_saved {
        query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
    }

    if let Some(order_by) = &opt.order_by {
        match order_by {
            ItemOrder::ReceivedDateDesc => {
                query
                    .order_by((Items::Table, Items::Id), Order::Desc)
                    .order_by(Items::PublishedAt, Order::Desc);
            }
            ItemOrder::PublishedDateDesc => {
                query.order_by(Items::PublishedAt, Order::Desc);
            }
            ItemOrder::UnreadFirst => {
                query
                    .order_by(
                        (Items::Table, Items::Status),
                        Order::Field(Values(vec![ItemStatus::Unread.to_string().into()])),
                    )
                    .order_by(Items::PublishedAt, Order::Desc);
            }
        }
    }

    if let Some(limit) = &opt.limit {
        query.limit(*limit);
    }

    if let Some(offset) = &opt.offset {
        query.offset(if let Some(limit) = &opt.limit {
            offset * limit
        } else {
            *offset
        });
    }

    let db = conn.lock().unwrap();
    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Item::from(x)))?;

    Ok(rows.map(std::result::Result::unwrap).collect::<Vec<Item>>())
}

pub fn count_all(conn: &DbConnection, opt: &ItemReadOption) -> Result<i64> {
    let mut query = Query::select()
        .from(Items::Table)
        .expr(Func::count(Expr::col(Items::Id)))
        .clone();

    if let Some(feed) = &opt.feed {
        query.and_where(Expr::col(Items::Feed).eq(*feed));
    }

    if let Some(status) = &opt.status {
        query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
    }

    if let Some(is_saved) = &opt.is_saved {
        query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
    }

    let db = conn.lock().unwrap();
    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;

    Ok(if let Some(row) = rows.next()? {
        row.get_unwrap(0)
    } else {
        0
    })
}

pub fn update(conn: &DbConnection, arg: &ItemToUpdate) -> Result<usize> {
    let mut vals = vec![];

    if let Some(status) = &arg.status {
        vals.push((Items::Status, status.to_string().into()));
    }

    if let Some(is_saved) = &arg.is_saved {
        vals.push((Items::IsSaved, (*is_saved).into()));
    }

    let (sql, values) = Query::update()
        .table(Items::Table)
        .values(vals)
        .and_where(Expr::col(Items::Id).eq(arg.id))
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn update_all(conn: &DbConnection, arg: &ItemToUpdateAll) -> Result<usize> {
    let mut vals = vec![];

    if let Some(status) = &arg.status {
        vals.push((Items::Status, status.to_string().into()));
    }

    if let Some(is_saved) = &arg.is_saved {
        vals.push((Items::IsSaved, (*is_saved).into()));
    }

    let mut query = Query::update().table(Items::Table).values(vals).clone();

    if let Some(opt) = &arg.opt {
        if let Some(ids) = &opt.ids {
            query.and_where(Expr::col(Items::Id).is_in(ids.clone()));
        }

        if let Some(feed) = &opt.feed {
            query.and_where(Expr::col(Items::Feed).eq(*feed));
        }

        if let Some(status) = &opt.status {
            query.and_where(Expr::col((Items::Table, Items::Status)).eq(status.to_string()));
        }

        if let Some(is_saved) = &opt.is_saved {
            query.and_where(Expr::col(Items::IsSaved).eq(*is_saved));
        }
    }

    let db = conn.lock().unwrap();
    let (sql, values) = query.build_rusqlite(SqliteQueryBuilder);
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}
