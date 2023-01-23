/*!
 * Schema definitions
 */

use sea_query::{SqliteQueryBuilder, Table, ColumnDef};

use crate::model::{PriceIden, SecurityIden};

pub(crate) fn get_drop_security() -> String {
    Table::drop()
        .table(SecurityIden::Table)
        .if_exists()
        .build(SqliteQueryBuilder)
}

pub(crate) fn get_drop_price() -> String {
    Table::drop()
            .table(PriceIden::Table)
            .if_exists()
            .build(SqliteQueryBuilder)
}

pub(crate) fn create_security() -> String {
    Table::create()
            .table(SecurityIden::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SecurityIden::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(SecurityIden::Namespace).string().null())
            .col(ColumnDef::new(SecurityIden::Symbol).string())
            .col(ColumnDef::new(SecurityIden::Updater).string().null())
            .col(ColumnDef::new(SecurityIden::Currency).string().null())
            .col(ColumnDef::new(SecurityIden::LedgerSymbol).string().null())
            .col(ColumnDef::new(SecurityIden::Notes).string().null())
            .build(SqliteQueryBuilder)
}

pub(crate) fn create_price() -> String {
    Table::create()
            .table(PriceIden::Table)
            .if_not_exists()
            .col(ColumnDef::new(PriceIden::Symbol).string())
            .col(
                ColumnDef::new(PriceIden::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(PriceIden::SecurityId).integer())
            .col(ColumnDef::new(PriceIden::Date).string())
            .col(ColumnDef::new(PriceIden::Time).string().null())
            .col(ColumnDef::new(PriceIden::Value).integer())
            .col(ColumnDef::new(PriceIden::Denom).integer())
            .col(ColumnDef::new(PriceIden::Currency).string())
            .build(SqliteQueryBuilder)
}

#[cfg(test)]
mod tests {
    use super::get_drop_price;

    // #[test]
    // fn test_drop_sec() {
    //     let actual = get_drop_security();

    //     assert_eq!(r#"DROP TABLE IF EXISTS "security""#, actual);
    // }

    #[test]
    fn test_drop_price() {
        let actual = get_drop_price();

        assert_eq!(r#"DROP TABLE IF EXISTS "price""#, actual);
    }
}
