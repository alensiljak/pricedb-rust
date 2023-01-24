/*!
Data mappers dal/models
 */

use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};

use crate::model::{Price, PriceIden};

pub fn get_price_columns() -> Vec<(PriceIden, PriceIden)> {
    vec![
        (PriceIden::Table, PriceIden::Symbol),
        (PriceIden::Table, PriceIden::Id),
        (PriceIden::Table, PriceIden::Date),
        (PriceIden::Table, PriceIden::Time),
        (PriceIden::Table, PriceIden::Value),
        (PriceIden::Table, PriceIden::Denom),
        (PriceIden::Table, PriceIden::Currency),
    ]
}

pub(crate) fn map_row_to_price(row: &Row) -> Price {
    Price {
        symbol: row.get(0).expect("symbol string"),
        id: row.get(1).expect("int id"),
        date: row.get_unwrap(2),
        time: row.get_unwrap(3),
        value: row.get(4).expect("value"),
        denom: row.get(5).expect("value"),
        currency: row.get(6).expect("value"),
    }
}

pub(crate) fn generate_insert_price(price: &Price) -> (String, RusqliteValues) {
    let result = Query::insert()
        .into_table(PriceIden::Table)
        .columns([
            PriceIden::Symbol,
            PriceIden::Date,
            PriceIden::Time,
            PriceIden::Value,
            PriceIden::Denom,
            PriceIden::Currency,
        ])
        .values_panic([
            price.symbol.to_owned().into(),
            price.date.to_owned().into(),
            price.time.to_owned().into(),
            price.value.into(),
            price.denom.into(),
            price.currency.to_owned().into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);
    // .build(SqliteQueryBuilder);
    //.to_string(SqliteQueryBuilder);
    result
}

pub(crate) fn generate_update_price(price: &Price) -> (String, RusqliteValues) {
    let mut stmt = Query::update()
        .table(PriceIden::Table)
        // .values([
        //     (PriceIden::SecurityId, price.security_id.into()),
        //     (PriceIden::Date, price.date.to_owned().into()),
        //     //(PriceIden::Time, price.time.)
        //     (PriceIden::Value, price.value.into()),
        //     (PriceIden::Denom, price.denom.into()),
        //     (PriceIden::Currency, price.currency.to_owned().into()),
        // ])
        // Update only the record with the given id.
        .and_where(Expr::col(PriceIden::Id).eq(price.id))
        .to_owned();

    // Values

    if price.symbol != String::default() {
        stmt = stmt
            .value(PriceIden::Symbol, price.symbol.to_owned())
            .to_owned();
    }

    if price.date != String::default() {
        stmt = stmt
            .value(PriceIden::Date, price.date.to_owned())
            .to_owned();
    }

    if price.time != String::default() {
        stmt = stmt
            .value(PriceIden::Time, price.time.to_owned())
            .to_owned();
    }

    if price.value != i64::default() {
        stmt = stmt.value(PriceIden::Value, price.value).to_owned();
    }

    if price.denom != i64::default() {
        stmt = stmt.value(PriceIden::Denom, price.denom).to_owned();
    }

    if price.currency != String::default() {
        stmt = stmt
            .value(PriceIden::Currency, price.currency.to_owned())
            .to_owned();
    }

    stmt.build_rusqlite(SqliteQueryBuilder)
}

pub(crate) fn generate_delete_price(id: i64) -> (String, RusqliteValues) {
    Query::delete()
        .from_table(PriceIden::Table)
        .and_where(Expr::col(PriceIden::Id).eq(id))
        .build_rusqlite(SqliteQueryBuilder)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::Price;

    use test_log::test;

    fn create_dummy_price() -> Price {
        Price {
            symbol: "".to_owned(),
            id: 13,
            date: "2022-12-07".to_owned(),
            time: "12:00:01".to_owned(),
            value: 12345,
            denom: 100,
            currency: "EUR".to_owned(),
        }
    }

    #[test]
    fn test_price_insert_statement() {
        let new_price = Price {
            symbol: "TEST:TEST".to_owned(),
            id: i64::default(),
            date: "2022-12-01".to_string(),
            time: Price::default_time(),
            value: 100,
            denom: 10,
            currency: "AUD".to_string(),
        };

        // let sql = dal.add_price(&new_price);
        let (sql, values) = generate_insert_price(&new_price);

        // println!("sql: {:?}, values: {:?}", sql, values);

        let expected = r#"INSERT INTO "price" ("symbol", "date", "time", "value", "denom", "currency") VALUES (?, ?, ?, ?, ?, ?)"#;
        assert_eq!(expected, sql);

        // Assert parameters

        assert_eq!(values.0[0].0, sea_query::Value::String(Some(Box::new("TEST:TEST".to_string()))));
        assert_eq!(
            values.0[1].0,
            sea_query::Value::String(Some(Box::new("2022-12-01".to_string())))
        );

        assert_eq!(
            values.0[2].0,
            sea_query::Value::String(Some(Box::new("00:00:00".to_owned())))
        );
        assert_eq!(values.0[3].0, sea_query::Value::BigInt(Some(100)));
        assert_eq!(values.0[4].0, sea_query::Value::BigInt(Some(10)));
        assert_eq!(
            values.0[5].0,
            sea_query::Value::String(Some(Box::new("AUD".to_string())))
        );
    }

    #[test_log::test]
    fn test_gen_update_price() {
        let price = create_dummy_price();

        let (sql, params) = generate_update_price(&price);

        log::debug!("update: {sql:?}");
        log::debug!("{params:?}");

        assert_eq!(
            sql,
            r#"UPDATE "price" SET "date" = ?, "time" = ?, "value" = ?, "denom" = ?, "currency" = ? WHERE "id" = ?"#
        );

        // time
        let actual_time = &params.0[1].0;
        assert_eq!(
            *actual_time,
            sea_query::Value::String(Some(Box::new(price.time)))
        );

        // currency
        let actual_currency: String = params.0[4].0.to_owned().unwrap();
        assert_eq!(*actual_currency, price.currency);
    }

    #[test]
    fn test_gen_delete_price() {
        let price = create_dummy_price();

        let (sql, values) = generate_delete_price(price.id);

        assert_eq!(sql, r#"DELETE FROM "price" WHERE "id" = ?"#);

        assert!(values.0.len() == 1);

        let actual_id: i64 = values.0[0].0.to_owned().unwrap();
        // println!("actual id = {actual_id:?}");
        assert_eq!(actual_id, 13);
    }
}
