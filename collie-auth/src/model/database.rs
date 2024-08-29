use sea_query::{ColumnDef, Iden, Table, TableCreateStatement};

#[derive(Iden)]
pub enum Keys {
    Table,
    Id,
    Access,
    Secret,
    Description,
    ExpiredAt,
}

pub fn keys_table() -> TableCreateStatement {
    Table::create()
        .table(Keys::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(Keys::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(Keys::Access).text().not_null().unique_key())
        .col(ColumnDef::new(Keys::Secret).text().not_null())
        .col(ColumnDef::new(Keys::Description).text())
        .col(ColumnDef::new(Keys::ExpiredAt).date_time())
        .to_owned()
}
