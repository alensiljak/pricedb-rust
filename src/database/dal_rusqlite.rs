use rusqlite::{Connection, Result, Row};

use crate::model::{Price, Security, SecuritySymbol};

use super::Dal;

pub struct RuSqliteDal {
    pub(crate) conn_str: String,
}

impl Dal for RuSqliteDal {
    fn delete_price(&self, id: i64) -> anyhow::Result<()> {
        todo!()
    }

    /// Search for the securities with the given filter.
    fn get_securities(
        &self,
        currency: &Option<String>,
        agent: &Option<String>,
        mnemonic: &Option<String>,
        exchange: &Option<String>,
    ) -> Vec<Security> {
        let result: Vec<Security> = vec![];

        // assemble the sql statement
        // let columns = get_query_parameters(currency, agent, mnemonic, exchange);
        // let sql = assemble_select_query(columns);
        let sql = "select * from security";
        log::debug!("select statement = {:?}", sql);

        // todo: implement filtering
        let conn = open_connection(&self.conn_str).expect("Error opening database.");
        let statement = conn.prepare(sql).unwrap();
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

        let conn = open_connection(&self.conn_str).expect("Error opening database.");
        let sql = "select * from security where symbol=?";
        let mut stmt = conn.prepare(sql).expect("Error");
        let params = (1, symbol);
        //let result = stmt.execute(params);
        let security = stmt
            .query_row(params, |r| {
                let result = Security::new();

                let x: i64 = r.get(0).expect("error");
                log::debug!("row fetched: {:?}", x);

                return Ok(result);
            })
            .expect("Error fetching security");

        log::debug!("query result: {:?}", security);

        return security;
    }

    fn get_symbols(&self) -> Vec<SecuritySymbol> {
        todo!()
    }

    fn get_prices_for_security(&self, security_id: i64) -> anyhow::Result<Vec<Price>> {
        let mut result: Vec<Price> = vec![];
        let conn = open_connection(&self.conn_str).expect("Error opening database.");
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

    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i64>> {
        let conn = open_connection(&self.conn_str).expect("Error opening database.");
        let sql = "select security_id from price";
        let mut stmt = conn.prepare(sql).expect("Error");
        let rows = stmt
            .query_map([], |row| {
                let id = row.get::<usize, i64>(0).expect("error");
                //log::debug!("row: {:?}", id);
                return Ok(id);
            })
            .expect("Error");

        // let count = rows.count();
        // log::debug!("fetched {:?} rows", count);

        let mut result: Vec<i64> = vec![];

        for row in rows {
            let id = row.expect("Error reading row");
            // log::debug!("id: {:?}", id);
            result.push(id);
        }

        return Ok(result);
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
fn open_connection(conn_str: &String) -> Result<Connection> {
    let connection = Connection::open(conn_str)?;
    return Ok(connection);
}
