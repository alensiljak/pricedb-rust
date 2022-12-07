/****
* DAL implemented with rusqlite
* Using SeaQuery to generate queries with variable parameters for filtering.
*
* Example for a query: https://stackoverflow.com/questions/67089430/how-do-we-use-select-query-with-an-external-where-parameter-in-rusqlite
*/
use rusqlite::{Connection, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder, QueryStatementWriter};

use crate::model::{Price, Security, SecurityFilter, SecurityIden, SecuritySymbol};

use super::Dal;

pub struct RuSqliteDal {
    pub(crate) conn_str: String,
}

impl Dal for RuSqliteDal {
    fn add_price(&self, new_price: &crate::model::NewPrice) {
        todo!()
    }

    fn delete_price(&self, id: i32) -> anyhow::Result<usize> {
        todo!()
    }

    fn get_ids_of_symbols_with_prices(&self) -> anyhow::Result<Vec<i32>> {
        let conn = open_connection(&self.conn_str);
        let sql = "select distinct security_id from price";
        let mut stmt = conn.prepare(sql).expect("Error");
        let ids = stmt
            .query_map([], |row| {
                let id = row.get::<usize, i32>(0).expect("error");
                //log::debug!("row: {:?}", id);
                return Ok(id);
            })
            .expect("Mapped rows");

        // let count = rows.count();
        // log::debug!("fetched {:?} rows", count);

        let mut result: Vec<i32> = vec![];

        for row in ids {
            let id = row.expect("Error reading row");
            // log::debug!("id: {:?}", id);
            result.push(id);
        }

        return Ok(result);
    }

    fn get_prices(&self, filter: Option<crate::model::PriceFilter>) -> Vec<Price> {
        todo!()
    }

    fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>> {
        let mut result: Vec<Price> = vec![];
        let conn = open_connection(&self.conn_str);
        let sql = "select * from price where security_id=? order by date desc, time desc;";
        let mut stmt = conn.prepare(sql).expect("Error");

        let rows = stmt
            .query_map([security_id], |row| {
                let price = map_price(row);
                // log::debug!("price read {:?}", price);

                return Ok(price);
            })
            .expect("Error");

        // let cursor: Vec<Result<Price, rusqlite::Error>> = rows.collect();
        // log::debug!("cursor: {:?}", cursor);

        for row in rows {
            //let record = map_price(&row);
            let record = row.expect("error extracting price");
            result.push(record);
            // log::debug!("row: {:?}", row);
        }
        return Ok(result);
    }

    /// Search for the securities with the given filter.
    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security> {
        let mut result: Vec<Security> = vec![];

        // assemble the sql statement
        // let sql = "select * from security";
        let sql = generate_query_with_filter(&filter);

        log::debug!("select statement = {:?}", sql);

        let conn = open_connection(&self.conn_str);
        let mut statement = conn.prepare(&sql).unwrap();

        let sec_iter = statement.query_map([], |row| {
            // map
            let sec = map_row_to_security(row);
            log::debug!("parsed: {:?}", sec);
            Ok(sec)
        }).expect("Filtered Securities");

        for item in sec_iter {
            match item {
                Ok(sec) => result.push(sec),
                Err(_) => todo!(),
            }
        }

        // log::debug!("securities: {:?}", result);

        return result;
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        log::trace!("fetching security by symbol {:?}", symbol);

        let conn = open_connection(&self.conn_str);
        let sql = "select * from security where symbol=?";
        let mut stmt = conn.prepare(sql).expect("Statement");
        let params = (1, symbol);
        //let result = stmt.execute(params);
        let security = stmt
            .query_row(params, |r| {
                // let result = Security::new();

                //let x: i64 = r.get(0).expect("error");
                let result = map_row_to_security(r);

                log::debug!("row fetched: {:?}", result);

                todo!("complete");

                return Ok(result);
            })
            .expect("Error fetching security");

        log::debug!("query result: {:?}", security);

        return security;
    }

    fn get_symbols(&self) -> Vec<SecuritySymbol> {
        todo!()
    }

    fn update_price(&self, id: i32, price: &Price) -> anyhow::Result<usize> {
        todo!()
    }
}

fn map_price(row: &Row) -> Price {
    let price = Price {
        id: row.get(0).expect("error reading field"),
        security_id: row.get(1).expect("error"),
        date: row.get(2).expect("error"),
        time: row.get(3).expect("error"),
        value: row.get(4).expect("error"),
        denom: row.get(5).expect("error"),
        currency: row.get(6).expect("error"),
    };
    price
}

/// rusqlite connection
fn open_connection(conn_str: &String) -> Connection {
    Connection::open(conn_str).expect("open sqlite connection")
}

fn map_row_to_security(row: &Row) -> Security {
    // let x: i32 = row.get(0).expect("field 0");
    // log::debug!("mapping row {:?}", x);

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

/// Generates SELECT statement with the given parameters/filters.
fn generate_query_with_filter(filter: &SecurityFilter) -> String {
    let query = Query::select()
        // Order of columns:
        .column(SecurityIden::Id)
        .column(SecurityIden::Namespace)
        .column(SecurityIden::Symbol)
        .column(SecurityIden::Updater)
        .column(SecurityIden::Currency)
        .column(SecurityIden::LedgerSymbol)
        .column(SecurityIden::Notes)
        //
        .from(SecurityIden::Table)
        .conditions(
            filter.currency.is_some(),
            |q| {
                if let Some(cur) = filter.currency.to_owned() {
                    let uppercase_cur = cur.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Currency).eq(uppercase_cur));
                }
            },
            |q| {},
        )
        .to_owned();

    // query.build(SqliteQueryBuilder)
    query.to_string(SqliteQueryBuilder)
}

