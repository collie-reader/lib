use std::{path::Path, sync::Mutex};

use rusqlite::Connection as RusqliteConnection;
use sea_query::{
    ColumnDef, Expr, ForeignKey, ForeignKeyAction, Iden, Index, SqliteQueryBuilder, Table,
    TableCreateStatement,
};

use crate::error::Result;

pub struct Connection {
    pub db: Mutex<RusqliteConnection>,
}

#[derive(Iden)]
pub enum Feeds {
    Table,
    Id,
    Title,
    Link,
    Status,
    CheckedAt,
    FetchOldItems,
}

#[derive(Iden)]
pub enum Items {
    Table,
    Id,
    Fingerprint,
    Author,
    Title,
    Description,
    Link,
    Status,
    IsSaved,
    PublishedAt,
    Feed,
}

pub struct Migration {
    tables: Vec<TableCreateStatement>,
}

impl Default for Migration {
    fn default() -> Self {
        Self::new()
    }
}

impl Migration {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn add_table(mut self, tables: TableCreateStatement) -> Self {
        self.tables.push(tables);
        self
    }

    pub fn migrate(&self, db: &RusqliteConnection) -> Result<()> {
        let sql = self
            .tables
            .iter()
            .map(|t| t.build(SqliteQueryBuilder))
            .collect::<Vec<_>>()
            .join(";");
        db.execute_batch(&sql)?;
        Ok(())
    }
}

pub fn open_connection(path: &Path) -> Result<RusqliteConnection> {
    Ok(RusqliteConnection::open(path)?)
}

pub fn feed_table() -> TableCreateStatement {
    Table::create()
        .table(Feeds::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Feeds::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Feeds::Title).text().not_null())
        .col(ColumnDef::new(Feeds::Link).text().not_null())
        .col(
            ColumnDef::new(Feeds::Status)
                .text()
                .check(Expr::col(Feeds::Status).is_in(["subscribed", "unsubscribed"]))
                .not_null()
                .default("subscribed"),
        )
        .col(ColumnDef::new(Feeds::CheckedAt).date_time().not_null())
        .col(
            ColumnDef::new(Feeds::FetchOldItems)
                .boolean()
                .not_null()
                .default(true),
        )
        .index(
            Index::create()
                .unique()
                .name("uk_feeds_title_link")
                .col(Feeds::Title)
                .col(Feeds::Link),
        )
        .to_owned()
}

pub fn items_table() -> TableCreateStatement {
    Table::create()
        .table(Items::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Items::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(Items::Fingerprint)
                .text()
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(Items::Author).text())
        .col(ColumnDef::new(Items::Title).text().not_null())
        .col(ColumnDef::new(Items::Description).text().not_null())
        .col(ColumnDef::new(Items::Link).text().not_null())
        .col(
            ColumnDef::new(Items::Status)
                .text()
                .check(Expr::col(Items::Status).is_in(["unread", "read"]))
                .not_null()
                .default("unread"),
        )
        .col(
            ColumnDef::new(Items::IsSaved)
                .integer()
                .check(Expr::col(Items::IsSaved).is_in([0, 1]))
                .not_null()
                .default(0),
        )
        .col(ColumnDef::new(Items::PublishedAt).date_time().not_null())
        .col(ColumnDef::new(Items::Feed).integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("fk_items_feeds")
                .from(Items::Table, Items::Feed)
                .to(Feeds::Table, Feeds::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned()
}
