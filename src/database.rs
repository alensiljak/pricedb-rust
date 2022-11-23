/*
 * trying to encapsulate database-specific code
 */
// mod dal_diesel;
// mod schema;
// mod dal_sqlx;
// mod dal_sqlite;
mod dal_rusqlite;

use confy::ConfyError;
use log::{debug, error};

use crate::{
    config::PriceDbConfig,
    model::{Price, Security, SecuritySymbol},
};

/// Initialize database connection.
pub fn init_db() -> impl Dal {
    // "sqlite::memory:"
    let conn_str = load_db_path();

    // choose the dal implementation here.
    //let dal = SqlxDal { conn_str };
    //let dal = dal_sqlite::SqliteDal {conn_str};
    let dal = dal_rusqlite::RuSqliteDal {conn_str};

    return dal;
}

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

pub trait Dal {
    fn delete_price(&self, id: i64) -> anyhow::Result<()>;

    fn get_securities(
        &self,
        currency: &Option<String>,
        agent: &Option<String>,
        mnemonic: &Option<String>,
        exchange: &Option<String>,
    ) -> Vec<Security>;

    fn get_security_by_symbol(&self, symbol: &str) -> Security;

    fn get_symbols(&self) -> Vec<SecuritySymbol>;

    fn get_prices_for_security(&self, security_id: i64) -> anyhow::Result<Vec<Price>>;

    /// Returns all the symbol ids that have prices in the database.
    /// Used for pruning.
    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i64>>;
}
