/*!
Data mappers dal/models
 */

use rusqlite::Row;
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};

use crate::model::{Price, PriceIden, Security, SecurityIden};

pub fn get_price_columns() -> Vec<(PriceIden, PriceIden)> {
    vec![
        (PriceIden::Table, PriceIden::Id),
        (PriceIden::Table, PriceIden::SecurityId),
        (PriceIden::Table, PriceIden::Date),
        (PriceIden::Table, PriceIden::Time),
        (PriceIden::Table, PriceIden::Value),
        (PriceIden::Table, PriceIden::Denom),
        (PriceIden::Table, PriceIden::Currency),
    ]
}

pub fn get_security_columns() -> Vec<(SecurityIden, SecurityIden)> {
    vec![
        (SecurityIden::Table, SecurityIden::Id),
        (SecurityIden::Table, SecurityIden::Namespace),
        (SecurityIden::Table, SecurityIden::Symbol),
        (SecurityIden::Table, SecurityIden::Updater),
        (SecurityIden::Table, SecurityIden::Currency),
        (SecurityIden::Table, SecurityIden::LedgerSymbol),
        (SecurityIden::Table, SecurityIden::Notes),
    ]
}

pub fn get_security_columns_wo_table() -> Vec<SecurityIden> {
    vec![
        SecurityIden::Id,
        SecurityIden::Namespace,
        SecurityIden::Symbol,
        SecurityIden::Updater,
        SecurityIden::Currency,
        SecurityIden::LedgerSymbol,
        SecurityIden::Notes,
    ]
}

pub(crate) fn map_row_to_price(row: &Row) -> Price {
    Price {
        id: row.get_unwrap(0),
        security_id: row.get_unwrap(1),
        date: row.get_unwrap(2),
        time: row.get(3).expect("value"),
        value: row.get(4).expect("value"),
        denom: row.get(5).expect("value"),
        currency: row.get(6).expect("value"),
    }
}

pub(crate) fn map_row_to_security(row: &Row) -> Security {
    Security {
        id: row.get_unwrap(0),
        namespace: row.get_unwrap(1),
        symbol: row.get_unwrap(2),
        updater: row.get_unwrap(3),
        currency: row.get_unwrap(4),
        ledger_symbol: row.get_unwrap(5),
        notes: row.get_unwrap(6),
    }
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

pub(crate) fn generate_insert_security(security: &Security) -> (String, RusqliteValues) {
    let columns = get_security_columns_wo_table();

    Query::insert()
        .into_table(SecurityIden::Table)
        .columns(columns)
        .values_panic([
            security.id.into(),
            security.namespace.to_owned().into(),
            security.symbol.to_owned().into(),
            security.updater.to_owned().into(),
            security.currency.to_owned().into(),
            security.ledger_symbol.to_owned().into(),
            security.notes.to_owned().into(),
        ])
        .build_rusqlite(SqliteQueryBuilder)
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

    if price.security_id != i64::default() {
        stmt = stmt
            .value(PriceIden::SecurityId, price.security_id)
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
            id: 13,
            security_id: 155,
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
            id: i64::default(),
            security_id: 111,
            date: "2022-12-01".to_string(),
            time: Price::default_time(),
            value: 100,
            denom: 10,
            currency: "AUD".to_string(),
        };

        // let sql = dal.add_price(&new_price);
        let (sql, values) = generate_insert_price(&new_price);

        // println!("sql: {:?}, values: {:?}", sql, values);

        let expected = r#"INSERT INTO "price" ("security_id", "date", "time", "value", "denom", "currency") VALUES (?, ?, ?, ?, ?, ?)"#;
        assert_eq!(expected, sql);

        // Assert parameters

        assert_eq!(values.0[0].0, sea_query::Value::BigInt(Some(111)));
        assert_eq!(
            values.0[1].0,
            sea_query::Value::String(Some(Box::new("2022-12-01".to_string())))
        );

        assert_eq!(values.0[2].0, sea_query::Value::String(Some(Box::new("00:00:00".to_owned()))));
        assert_eq!(values.0[3].0, sea_query::Value::Int(Some(100)));
        assert_eq!(values.0[4].0, sea_query::Value::Unsigned(Some(10)));
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
            sea_query::Value::String(Some(Box::new(price.time)))
        );

        // currency
        let actual_currency: String = params.0[5].0.to_owned().unwrap();
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
