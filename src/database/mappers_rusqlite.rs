/*
Data mappers dal/models
 */

use rusqlite::Row;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteValues, RusqliteBinder};

use crate::model::{Price, Security, PriceIden};

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

pub(crate) fn generate_insert_price(new_price: &Price) -> (String, RusqliteValues) {
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
            new_price.security_id.into(),
            new_price.date.to_owned().into(),
            new_price.time.to_owned().into(),
            new_price.value.into(),
            new_price.denom.into(),
            new_price.currency.to_owned().into(),
        ])
        .build_rusqlite(SqliteQueryBuilder);
    // .build(SqliteQueryBuilder);
    //.to_string(SqliteQueryBuilder);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::Price;

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

        //let expected = "INSERT INTO \"price\" (\"security_id\", \"date\", \"time\", \"value\", \"denom\", \"currency\") VALUES (111, '2022-12-01', NULL, 100, 0, 'AUD')";
        let expected = "INSERT INTO \"price\" (\"security_id\", \"date\", \"time\", \"value\", \"denom\", \"currency\") VALUES (?, ?, ?, ?, ?, ?)";
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


}