use rusqlite::Connection as RusqliteConnection;
use sea_query::{
    ColumnDef, Expr, ForeignKey, ForeignKeyAction, Iden, Index, SqliteQueryBuilder, Table,
    TableStatement,
};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::error::Result;

pub type DbConnection = Arc<Mutex<RusqliteConnection>>;

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
    tables: Vec<Vec<TableStatement>>,
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

    pub fn table(mut self, stmts: Vec<TableStatement>) -> Self {
        self.tables.push(stmts);
        self
    }

    pub fn migrate(&self, db: &RusqliteConnection) -> Result<()> {
        let sql = self
            .tables
            .iter()
            .map(|stmts| {
                stmts
                    .iter()
                    .map(|stmt| stmt.build(SqliteQueryBuilder))
                    .collect::<Vec<_>>()
                    .join(";")
            })
            .collect::<Vec<_>>();

        for stmt in sql {
            let _ = db.execute_batch(&stmt);
        }

        Ok(())
    }
}

pub fn open_connection(path: &Path) -> Result<RusqliteConnection> {
    Ok(RusqliteConnection::open(path)?)
}

pub fn feeds_table() -> Vec<TableStatement> {
    let create_stmt = Table::create()
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
        .to_owned();

    let alter_stmt = Table::alter()
        .table(Feeds::Table)
        .add_column_if_not_exists(
            ColumnDef::new(Feeds::FetchOldItems)
                .boolean()
                .not_null()
                .default(true),
        )
        .to_owned();

    vec![
        TableStatement::Create(create_stmt),
        TableStatement::Alter(alter_stmt),
    ]
}

pub fn items_table() -> Vec<TableStatement> {
    let create_stmt = Table::create()
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
        .to_owned();

    vec![TableStatement::Create(create_stmt)]
}
