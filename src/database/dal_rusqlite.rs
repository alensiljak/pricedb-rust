use rusqlite::{Connection, Result};

use crate::model::{Security, SecuritySymbol, Price};

use super::Dal;

pub struct RuSqliteDal {
    pub(crate) conn_str: String,
}

impl Dal for RuSqliteDal {
    fn delete_price(&self, id: i64) -> anyhow::Result<()> {
        todo!()
    }

    fn get_securities(
        &self,
        currency: &Option<String>,
        agent: &Option<String>,
        mnemonic: &Option<String>,
        exchange: &Option<String>,
    ) -> Vec<Security> {
        todo!()
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        log::trace!("fetching security by symbol {:?}", symbol);

        let conn = open_connection(&self.conn_str).expect("Error opening database.");
        let sql = "select * from security where symbol=?";
        let mut stmt = conn.prepare(sql).expect("Error");
        let params = (1, symbol);
        //let result = stmt.execute(params);
        let security = stmt.query_row(params, |r| {
            let result = Security::new();

            let x: i64 = r.get(0).expect("error");
            log::debug!("row fetched: {:?}", x);
            
            return Ok(result);
        }).expect("Error fetching security");
        
        log::debug!("query result: {:?}", security);

        return security;
    }

    fn get_symbols(&self) -> Vec<SecuritySymbol> {
        todo!()
    }

    fn get_prices_for_security(
        &self,
        security_id: i64,
    ) -> anyhow::Result<Vec<Price>> {
        todo!()
    }

    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i64>> {
        let conn = open_connection(&self.conn_str).expect("Error opening database.");
        let sql = "select security_id from price";
        let mut stmt = conn.prepare(sql).expect("Error");
        let rows = stmt.query_map([], |row| {
            let id = row.get::<usize, i64>(0).expect("error");
            //log::debug!("row: {:?}", id);
            return Ok(id);
        }).expect("Error");
        
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

/// rusqlite connection
fn open_connection(conn_str: &String) -> Result<Connection> {
    let connection = Connection::open(conn_str)?;
    return Ok(connection);
}
