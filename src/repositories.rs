/*
 * Operations on the database
 */

use confy::ConfyError;
// use sqlite::{Connection, Row, State};
use rusqlite::{Connection, Result};
use tracing::{debug, error};

use crate::{config::PriceDbConfig, model::Security};

pub(crate) fn test_db() {
    //let connection = sqlite::open(":memory:").unwrap();

    let insert_query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";

    // connection.execute(insert_query).unwrap();

    // query
    let query = "SELECT * FROM users;";
    // let mut statement = connection.prepare(query).unwrap();

    //let result = connection.execute(query).unwrap();
    //let result = statement.iter();
    //debug!("sqlite: {:?}", result);

    // while let Ok(State::Row) = statement.next() {
    //     println!("name = {}", statement.read::<String, _>("name").unwrap());
    //     println!("age = {}", statement.read::<i64, _>("age").unwrap());
    // }
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
    ) -> Result<Vec<Security>> {
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

        let conn = open_connection().unwrap();
        let query = "select * from security";

        // todo: implement the filter

        // let cursor = connection.prepare(query).unwrap().into_iter();
        conn.prepare(query)?;
        let mut result: Vec<Security> = vec![];

        // for row in cursor {
        //     let values = row.unwrap();
        //     // let security = read_security(values);

        //     // result.push(security);
        // }
        return Ok(result);
    }

    /// Get all the records.
    pub(crate) fn all(&self) -> Result<Vec<Security>> {
        return self.query(None, None, None, None);
    }

    // pub(crate) fn get(&self, id: i32) {
    //     // load from db
    // }
}

// fn read_security(row: Row) -> Security {
//     let mut security = Security::new();

//     match row.try_read::<i64, _>("id") {
//         Ok(id) => security.id = id,
//         Err(e) => warn!("Could not read id field. {}", e),
//     }

//     match row.try_read::<&str, _>("namespace") {
//         Ok(value) => security.namespace = value.to_string(),
//         Err(e) => warn!("Could not read namespace field. {}", e),
//     }

//     match row.try_read::<&str, _>("symbol") {
//         Ok(value) => security.symbol = value.to_string(),
//         Err(e) => warn!("Could not read symbol field. {}", e),
//     }

//     match row.try_read::<&str, _>("currency") {
//         Ok(value) => security.currency = value.to_string(),
//         Err(e) => warn!("Could not read currency field. {}", e),
//     }

//     match row.try_read::<&str, _>("updater") {
//         Ok(value) => security.updater = value.to_string(),
//         Err(e) => warn!("Could not read updater field. {}", e),
//     }

//     match row.try_read::<&str, _>("ledger_symbol") {
//         Ok(value) => security.ledger_symbol = value.to_string(),
//         Err(e) => warn!("Could not read ledger_symbol field. {}", e),
//     }

//     return security;
// }

/// Loads database path from the configuration.
fn load_db_path() -> String {
    let config_result: Result<PriceDbConfig, ConfyError> = confy::load("pricedb", "config");
    let db_path: String;

    debug!("configuration: {:?}", config_result);

    match config_result {
        Ok(config) => db_path = config.price_database_path,
        Err(e) => {
            error!("Error: {:?}", e);
            panic!("{}", e);
        }
    }

    debug!("Db path: {:?}", db_path);

    return db_path;
}

// fn open_connection() -> Connection {
//     let con_str = load_connection_string();
//     let connection = sqlite::open(con_str).unwrap();
//     return connection;
// }

fn open_connection() -> Result<Connection> {
    let con_str = load_db_path();
    let connection = Connection::open(con_str)?;
    return Ok(connection);
}
