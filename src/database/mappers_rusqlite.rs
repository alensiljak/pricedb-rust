/*
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
        id: row.get(0).expect("value"),
        security_id: row.get(1).expect("value"),
        date: row.get(2).expect("value"),
        time: row.get(3).expect("value"),
        value: row.get(4).expect("value"),
        denom: row.get(5).expect("value"),
        currency: row.get(6).expect("value"),
    }
}

pub(crate) fn map_row_to_security(row: &Row) -> Security {
    Security {
        id: row.get(0).expect("id"),
        namespace: row.get(1).expect("namespace"),
        symbol: row.get(2).expect("symbol"),
        updater: row.get(3).expect("updater"),
        currency: row.get(4).expect("currency"),
        ledger_symbol: row.get(5).expect("ledger symbol"),
        notes: row.get(6).expect("notes"),
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
            security.notes.to_owned().into()
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

    stmt.build_rusqlite(SqliteQueryBuilder)
}

pub(crate) fn generate_delete_price(id: i32) -> (String, RusqliteValues) {
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
        let actual_currency: String = params.0[5].0.to_owned().unwrap();
        assert_eq!(*actual_currency, price.currency);
    }

    #[test]
    fn test_gen_delete_price() {
        let price = create_dummy_price();

        let (sql, values) = generate_delete_price(price.id);

        assert_eq!(sql, r#"DELETE FROM "price" WHERE "id" = ?"#);

        assert!(values.0.len() == 1);

        let actual_id: i32 = values.0[0].0.to_owned().unwrap();
        // println!("actual id = {actual_id:?}");
        assert_eq!(actual_id, 13);
    }
}
