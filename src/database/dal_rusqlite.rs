/****
* DAL implemented with rusqlite
*
* Example for a query: https://stackoverflow.com/questions/67089430/how-do-we-use-select-query-with-an-external-where-parameter-in-rusqlite
   let mut stmt = conn.prepare("SELECT id, name, age, data FROM person WHERE age=:age;")?;
   let person_iter = stmt.query_map(&[(":age", &age.to_string())], |row| {
       Ok(Person {
           id: row.get(0)?,
           name: row.get(1)?,
           age: row.get(2)?,
           data: row.get(3)?,
       })
   })?;
*/
use rusqlite::{Connection, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder};

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
        let result: Vec<Security> = vec![];

        // assemble the sql statement
        // let columns = get_query_parameters(currency, agent, mnemonic, exchange);
        // let sql = assemble_select_query(columns);
        // let sql = "select * from security";
        let query = generate_query_with_filter(&filter);
        let sql = query.0;

        log::debug!("select statement = {:?}, params: {:?}", sql, query.1);

        todo!("filtering");

        // todo: implement filtering
        let conn = open_connection(&self.conn_str);
        let statement = conn.prepare(&sql).unwrap();
        // append parameters
        // let statement = append_param_values(&statement, currency, agent, mnemonic, exchange);
        //let cursor = statement.into_iter().map(|row| row.unwrap());
        // for row in cursor {
        //     log::debug!("row: {:?}", row);
        // }

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
    let sec = Security {
        id: row.get(0).expect("id"),
        namespace: todo!(),
        symbol: todo!(),
        updater: todo!(),
        currency: todo!(),
        ledger_symbol: todo!(),
        notes: todo!(),
    };

    sec
}

fn generate_query_with_filter(filter: &SecurityFilter) -> (String, sea_query::Values) {
    let query = Query::select()
        .column(SecurityIden::Symbol)
        .from(SecurityIden::Table)
        .conditions(
            filter.currency.is_some(),
            |q| {
                if let Some(cur) = filter.currency.to_owned() {
                    q.and_where(Expr::col(SecurityIden::Currency).eq(cur.to_owned()));
                }
            },
            |q| {},
        )
        .to_owned();

    query.build(SqliteQueryBuilder)
}

/// Don't use this.
/// It works but was written before finding .conditions()
///
fn generate_query_with_filter_manual(filter: &SecurityFilter) -> String {
    let mut query = Query::select()
        .column(SecurityIden::Id)
        .from(SecurityIden::Table)
        .to_owned();

    // Some conditions
    if let Some(agent) = &filter.agent {
        query = query
            .and_where(Expr::col(SecurityIden::Updater).eq(agent.to_owned()))
            .to_owned();
    }
    if let Some(cur) = &filter.currency {
        query = query
            .and_where(Expr::col(SecurityIden::Currency).eq(cur.to_owned()))
            .to_owned();
    }
    if let Some(exc) = &filter.exchange {
        query = query
            .and_where(Expr::col(SecurityIden::Namespace).eq(exc.to_owned()))
            .to_owned();
    }
    if let Some(sym) = &filter.symbol {
        query = query
            .and_where(Expr::col(SecurityIden::Symbol).eq(sym.to_owned()))
            .to_owned();
    }

    let x = query.build(SqliteQueryBuilder);
    log::debug!("generated SQL: {:?}", x);

    x.0
}
