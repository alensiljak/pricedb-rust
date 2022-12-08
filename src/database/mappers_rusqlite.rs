/*
Data mappers dal/models
 */

use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};

use crate::model::{Price, PriceIden, Security};

pub(crate) fn map_row_to_price(row: &Row) -> Price {
    let result = Price {
        id: row.get(0).expect("value"),
        security_id: row.get(1).expect("value"),
        date: row.get(2).expect("value"),
        time: row.get(3).expect("value"),
        value: row.get(4).expect("value"),
        denom: row.get(5).expect("value"),
        currency: row.get(6).expect("value"),
    };

    result
}

pub(crate) fn map_row_to_security(row: &Row) -> Security {
    let sec = Security {
        id: row.get(0).expect("id"),
        namespace: row.get(1).expect("namespace"),
        symbol: row.get(2).expect("symbol"),
        updater: row.get(3).expect("updater"),
        currency: row.get(4).expect("currency"),
        ledger_symbol: row.get(5).expect("ledger symbol"),
        notes: row.get(6).expect("notes"),
    };

    sec
}

pub(crate) fn generate_insert_price(price: &Price) -> (String, RusqliteValues) {
    let result = Query::insert()
        .into_table(PriceIden::Table)
        .columns([
            PriceIden::SecurityId,
            PriceIden::Date,
            PriceIden::Time,
            PriceIden::Value,
            PriceIden::Denom,
            PriceIden::Currency,
        ])
        .values_panic([
            price.security_id.into(),
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

    if price.security_id != i32::default() {
        stmt = stmt
            .value(PriceIden::SecurityId, price.security_id)
            .to_owned();
    }

    if price.date != String::default() {
        stmt = stmt
            .value(PriceIden::Date, price.date.to_owned())
            .to_owned();
    }

    if price.time.is_some() {
        let value = price.time.to_owned().unwrap().as_str().to_owned();
        stmt = stmt.value(PriceIden::Time, value).to_owned();
    }

    if price.value != i32::default() {
        stmt = stmt.value(PriceIden::Value, price.value).to_owned();
    }

    if price.denom != i32::default() {
        stmt = stmt.value(PriceIden::Denom, price.denom).to_owned();
    }

    if price.currency != String::default() {
        stmt = stmt
            .value(PriceIden::Currency, price.currency.to_owned())
            .to_owned();
    }

    let result = stmt.build_rusqlite(SqliteQueryBuilder);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::Price;

    use test_log::test;

    fn create_dummy_price() -> Price {
        Price {
            id: i32::default(),
            security_id: 155,
            date: "2022-12-07".to_owned(),
            time: Some("12:00:01".to_owned()),
            value: 12345,
            denom: 100,
            currency: "EUR".to_owned(),
        }
    }

    #[test]
    fn test_price_insert_statement() {
        let new_price = Price {
            id: i32::default(),
            security_id: 111,
            date: "2022-12-01".to_string(),
            time: None,
            value: 100,
            denom: 10,
            currency: "AUD".to_string(),
        };

        // let sql = dal.add_price(&new_price);
        let (sql, values) = generate_insert_price(&new_price);

        println!("sql: {:?}, values: {:?}", sql, values);

        //let expected = "INSERT INTO "price" ("security_id", "date", "time", "value", "denom", "currency") VALUES (111, '2022-12-01', NULL, 100, 0, 'AUD')";
        let expected = r#"INSERT INTO "price" ("security_id", "date", "time", "value", "denom", "currency") VALUES (?, ?, ?, ?, ?, ?)"#;
        assert_eq!(expected, sql);

        assert_eq!(values.0[0].0, sea_query::Value::Int(Some(111)));
        assert_eq!(
            values.0[1].0,
            sea_query::Value::String(Some(Box::new("2022-12-01".to_string())))
        );

        assert_eq!(values.0[2].0, sea_query::Value::String(None));
        assert_eq!(values.0[3].0, sea_query::Value::Int(Some(100)));
        assert_eq!(values.0[4].0, sea_query::Value::Int(Some(10)));
        assert_eq!(
            values.0[5].0,
            sea_query::Value::String(Some(Box::new("AUD".to_string())))
        );
    }

    #[test]
    fn test_gen_update_price() {
        let price = create_dummy_price();

        let (sql, params) = generate_update_price(&price);

        // println!("update: {sql:?}");
        // println!("{params:?}");

        assert_eq!(
            sql,
            r#"UPDATE "price" SET "security_id" = ?, "date" = ?, "time" = ?, "value" = ?, "denom" = ?, "currency" = ? WHERE "id" = ?"#
        );

        // time
        let actual_time = &params.0[2].0;
        assert_eq!(
            *actual_time,
            sea_query::Value::String(Some(Box::new(price.time.unwrap())))
        );

        // currency
        let actual_currency = &params.0[5].0;
        assert_eq!(
            *actual_currency,
            sea_query::Value::String(Some(Box::new(price.currency)))
        );
    }
}
