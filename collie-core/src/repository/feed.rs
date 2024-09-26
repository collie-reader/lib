use chrono::Utc;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

use crate::{
    error::Result,
    model::feed::{Feed, FeedToCreate, FeedToUpdate},
};

use super::database::{DbConnection, Feeds};

pub fn create(conn: &DbConnection, arg: &FeedToCreate) -> Result<usize> {
    let (sql, values) = Query::insert()
        .into_table(Feeds::Table)
        .columns([
            Feeds::Title,
            Feeds::Link,
            Feeds::CheckedAt,
            Feeds::FetchOldItems,
        ])
        .values_panic([
            (*arg.title).into(),
            (*arg.link).into(),
            Utc::now().into(),
            arg.fetch_old_items.into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn read_all(conn: &DbConnection) -> Result<Vec<Feed>> {
    let (sql, values) = Query::select()
        .columns([
            Feeds::Id,
            Feeds::Title,
            Feeds::Link,
            Feeds::Status,
            Feeds::CheckedAt,
            Feeds::FetchOldItems,
        ])
        .from(Feeds::Table)
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    let mut stmt = db.prepare(sql.as_str())?;
    let rows = stmt.query_map(&*values.as_params(), |x| Ok(Feed::from(x)))?;

    Ok(rows.map(std::result::Result::unwrap).collect::<Vec<Feed>>())
}

pub fn read(conn: &DbConnection, id: i32) -> Result<Option<Feed>> {
    let (sql, values) = Query::select()
        .columns([
            Feeds::Id,
            Feeds::Title,
            Feeds::Link,
            Feeds::Status,
            Feeds::CheckedAt,
            Feeds::FetchOldItems,
        ])
        .from(Feeds::Table)
        .and_where(Expr::col(Feeds::Id).eq(id))
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;

    Ok(rows.next()?.map(Feed::from))
}

pub fn update(conn: &DbConnection, arg: &FeedToUpdate) -> Result<usize> {
    let mut vals = vec![];

    if let Some(title) = &arg.title {
        vals.push((Feeds::Title, title.into()));
    }

    if let Some(link) = &arg.link {
        vals.push((Feeds::Link, link.into()));
    }

    if let Some(status) = &arg.status {
        vals.push((Feeds::Status, status.to_string().into()));
    }

    if let Some(checked_at) = arg.checked_at {
        vals.push((Feeds::CheckedAt, checked_at.into()));
    }

    if let Some(fetch_old_items) = arg.fetch_old_items {
        vals.push((Feeds::FetchOldItems, fetch_old_items.into()));
    }

    let (sql, values) = Query::update()
        .table(Feeds::Table)
        .values(vals)
        .and_where(Expr::col(Feeds::Id).eq(arg.id))
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn delete(conn: &DbConnection, id: i32) -> Result<usize> {
    let (sql, values) = Query::delete()
        .from_table(Feeds::Table)
        .and_where(Expr::col(Feeds::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}
