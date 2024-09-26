use crate::{error::Result, model::key::KeyToCreate};
use collie_core::repository::database::DbConnection;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::RusqliteBinder;

use super::database::Keys;

pub fn create(conn: &DbConnection, arg: &KeyToCreate) -> Result<usize> {
    let (sql, values) = Query::insert()
        .into_table(Keys::Table)
        .columns([
            Keys::Access,
            Keys::Secret,
            Keys::Description,
            Keys::ExpiredAt,
        ])
        .values_panic([
            (*arg.access).into(),
            (*arg.secret).into(),
            arg.description.clone().into(),
            arg.expired_at.into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    Ok(db.execute(sql.as_str(), &*values.as_params())?)
}

pub fn exists(conn: &DbConnection, access: &str, secret: &str) -> Result<bool> {
    let (sql, values) = Query::select()
        .columns([Keys::Id])
        .from(Keys::Table)
        .and_where(Expr::col(Keys::Access).eq(access))
        .and_where(Expr::col(Keys::Secret).eq(secret))
        .and_where(
            Expr::col(Keys::ExpiredAt)
                .gt(chrono::Utc::now())
                .or(Expr::col(Keys::ExpiredAt).is_null()),
        )
        .limit(1)
        .build_rusqlite(SqliteQueryBuilder);

    let db = conn.lock().unwrap();
    let mut stmt = db.prepare(sql.as_str())?;
    let mut rows = stmt.query(&*values.as_params())?;

    Ok(rows.next()?.is_some())
}
