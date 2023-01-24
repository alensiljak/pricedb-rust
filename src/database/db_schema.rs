/*!
 * Schema definitions
 */

use sea_query::{SqliteQueryBuilder, Table, ColumnDef};

use crate::model::PriceIden;

pub(crate) fn get_drop_price() -> String {
    Table::drop()
            .table(PriceIden::Table)
            .if_exists()
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

    #[test]
    fn test_drop_price() {
        let actual = get_drop_price();

        assert_eq!(r#"DROP TABLE IF EXISTS "price""#, actual);
    }
}
