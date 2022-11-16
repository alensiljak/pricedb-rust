/*
 * Operations on the database
 */

use confy::ConfyError;
use sqlite::{State, Connection};
use tracing::{debug, error};

use crate::config::PriceDbConfig;

pub(crate) fn test_db() {
    let connection = sqlite::open(":memory:").unwrap();

    let insert_query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";

    connection.execute(insert_query).unwrap();

    // query
    let query = "SELECT * FROM users;";
    let mut statement = connection.prepare(query).unwrap();

    //let result = connection.execute(query).unwrap();
    //let result = statement.iter();
    //debug!("sqlite: {:?}", result);

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}

/// Securities Repository
/// Table: security
pub(crate) struct SecurityRepository {}

impl SecurityRepository {
    /// Query the database.
    pub(crate) fn all(&self) -> Vec<String> {
        let connection = open_connection();
        let query = format!("select * from {}", "security");
        
        // todo: implement the filter

        let cursor = connection.prepare(query).unwrap().into_iter();
        let mut result: Vec<String> = vec![];

        for row in cursor {
            let values = row.unwrap();
            //let id = values.read::<i64, _>("id");
            let symbol = values.read::<&str, _>("symbol");

            //println!("security id: {}", id);
            result.push(symbol.to_string());
        }
        return result;
    }

    pub(crate) fn get(&self, id: i32) {
        // load from db
    }
}

/// Load connection string from the configuration.
fn load_connection_string() -> String {
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

fn open_connection() -> Connection {
    let con_str = load_connection_string();
    let connection = sqlite::open(con_str).unwrap();
    return connection;
}
