/*
 * DAL with sqlite
 *
 * Pros:
 *   - does not use libsqlite3-sys (build issues on NTFS)
 *
 */

use sqlite::{Connection, Error, Row};

use crate::model::{Security, Price};

use super::Dal;

pub struct SqliteDal {
    pub(crate) conn_str: String,
}

impl Dal for SqliteDal {
    fn get_securities(
        &self,
        currency: Option<String>,
        agent: Option<String>,
        mnemonic: Option<String>,
        exchange: Option<String>,
    ) -> Vec<Security> {
        todo!()
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        log::trace!("getting security from symbol {:?}", symbol);
        
        let mut result: Security = Security::new();

        let conn = open_connection(&self.conn_str);
        let rows = conn
            .prepare("select * from security where symbol=?").unwrap()
            .into_iter()
            .bind((1, symbol)).unwrap()
            .map(|row| row.unwrap());
        for row in rows {
            log::debug!("row: {:?}", row);
            result = read_security(row);
        }
        return result;
    }

    fn get_symbols(&self) -> Vec<crate::model::SecuritySymbol> {
        todo!()
    }

    fn get_prices_for_security(
        &self,
        security_id: i64,
    ) -> anyhow::Result<Vec<Price>> {
        let mut result: Vec<Price> = vec![];
        let conn = open_connection(&self.conn_str);
        let sql = "select * from price where security_id=? order by date desc, time desc;";

        let cursor = conn.prepare(sql).unwrap()
            .into_iter()
            .bind((1, security_id)).unwrap()
            .map(|row| row.unwrap());

        for row in cursor {
            let record = map_price(&row);
            result.push(record);
        }
        return Ok(result);
    }

    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<(i64, String)>> {
        let mut result: Vec<(i64, String)> = vec![];
        let conn = open_connection(&self.conn_str);
        let rows = conn
            .prepare("select security_id, symbol from price").unwrap()
            .into_iter()
            .map(|row| row.unwrap());
        for row in rows {
            // log::debug!("row: {:?}", row);
            let id = row.read::<i64, _>(0);
            let symbol = row.read::<&str, _>(1);
            result.push((id, symbol.to_string()));
        }
        return Ok(result);
    }

    fn delete_price(&self, id: i64) -> Result<(), anyhow::Error> {
        let conn = open_connection(&self.conn_str);
        let sql = format!("delete from price where id={}", id);
        let result = conn.execute(sql).unwrap();
        return Ok(result);
    }
}

/// sqlite connection
fn open_connection(conn_str: &String) -> Connection {
    let connection = sqlite::open(conn_str).unwrap();
    return connection;
}

/// Read Security record from sqlite row
fn read_security(row: Row) -> Security {
    let mut security = Security::new();

    match row.try_read::<i64, _>("id") {
        Ok(id) => security.id = id,
        Err(e) => log::warn!("Could not read id field. {}", e),
    }

    match row.try_read::<&str, _>("namespace") {
        Ok(value) => security.namespace = Some(value.to_string()),
        Err(e) => log::warn!("Could not read namespace field. {}", e),
    }

    match row.try_read::<&str, _>("symbol") {
        Ok(value) => security.symbol = value.to_string(),
        Err(e) => log::warn!("Could not read symbol field. {}", e),
    }

    match row.try_read::<&str, _>("currency") {
        Ok(value) => security.currency = Some(value.to_string()),
        Err(e) => log::warn!("Could not read currency field. {}", e),
    }

    match row.try_read::<&str, _>("updater") {
        Ok(value) => security.updater = Some(value.to_string()),
        Err(e) => log::warn!("Could not read updater field. {}", e),
    }

    match row.try_read::<&str, _>("ledger_symbol") {
        Ok(value) => security.ledger_symbol = Some(value.to_string()),
        Err(e) => log::warn!("Could not read ledger_symbol field. {}", e),
    }

    return security;
}

fn map_price(row: &Row) -> Price {
    let price = Price {
        id: row.read(0),
        security_id: row.read(1),
        date: String::from(row.read::<&str, _>(2)),
        time: Some(String::from(row.try_read::<&str, _>(3).unwrap_or_default())),
        value: row.read::<i64, _>(4),
        denom: row.read::<i64, _>(4),
        currency: String::from(row.read::<&str, _>(6)),
    };

    return price;
}

/// Securities Repository
/// Table: security
pub(crate) struct SecurityRepository {}

impl SecurityRepository {
    /// Query the database.
    pub fn query(
        &self,
        currency_param: Option<String>,
        agent: Option<String>,
        mnemonic: Option<String>,
        exchange: Option<String>,
    ) -> Result<Vec<Security>, Error> {
        let mut query: String = "select * from security".to_string();

        // append parameters, if any
        if currency_param.is_some() || agent.is_some() || mnemonic.is_some() || exchange.is_some() {
            query += " where ";
        }

        let currency: &str;
        match &currency_param {
            Some(value) => {
                currency = value.as_str();
                query += "currency = :currency";
            }
            None => currency = "",
        }

        // let connection = open_connection();
        // let result = connection
        //     .prepare(query)
        //     .unwrap()
        //     .bind((":currency", currency))
        //     .iter();

        // full

        let conn = open_connection(&"".to_string());
        let query = "select * from security";

        // todo: implement the filter

        let cursor = conn.prepare(query).unwrap().into_iter();
        conn.prepare(query)?;
        let mut result: Vec<Security> = vec![];

        for row in cursor {
            let values = row.unwrap();
            let security = read_security(values);

            result.push(security);
        }
        return Ok(result);
    }

    /// Get all the records.
    pub(crate) fn all(&self) -> Result<Vec<Security>, Error> {
        return self.query(None, None, None, None);
    }
}
