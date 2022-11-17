/*
 * DAL with sqlite
 */

use sqlite::{Connection, Row, Error};

/// sqlite connection
fn open_connection() -> Connection {
    let con_str = load_db_path();
    let connection = sqlite::open(con_str).unwrap();
    return connection;
}

/// Read Security record from sqlite row
fn read_security(row: Row) -> Security {
    let mut security = Security::new();

    match row.try_read::<i64, _>("id") {
        Ok(id) => security.id = id,
        Err(e) => warn!("Could not read id field. {}", e),
    }

    match row.try_read::<&str, _>("namespace") {
        Ok(value) => security.namespace = value.to_string(),
        Err(e) => warn!("Could not read namespace field. {}", e),
    }

    match row.try_read::<&str, _>("symbol") {
        Ok(value) => security.symbol = value.to_string(),
        Err(e) => warn!("Could not read symbol field. {}", e),
    }

    match row.try_read::<&str, _>("currency") {
        Ok(value) => security.currency = value.to_string(),
        Err(e) => warn!("Could not read currency field. {}", e),
    }

    match row.try_read::<&str, _>("updater") {
        Ok(value) => security.updater = value.to_string(),
        Err(e) => warn!("Could not read updater field. {}", e),
    }

    match row.try_read::<&str, _>("ledger_symbol") {
        Ok(value) => security.ledger_symbol = value.to_string(),
        Err(e) => warn!("Could not read ledger_symbol field. {}", e),
    }

    return security;
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

        let connection = open_connection();
        // let result = connection
        //     .prepare(query)
        //     .unwrap()
        //     .bind((":currency", currency))
        //     .iter();

        // full

        let conn = open_connection();
        let query = "select * from security";

        // todo: implement the filter

        let cursor = connection.prepare(query).unwrap().into_iter();
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

    // pub(crate) fn get(&self, id: i32) {
    //     // load from db
    // }
}
