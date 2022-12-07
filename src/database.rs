/*
 * trying to encapsulate database-specific code
 */
// mod dal_diesel;            // requires schema; complex
// pub(crate) mod schema;
// mod dal_sqlx;              // async-only
// mod dal_sqlite;
mod dal_rusqlite;

use log::debug;

use crate::{
    config::PriceDbConfig,
    model::{NewPrice, Price, PriceFilter, Security, SecurityFilter, SecuritySymbol},
};

/// Initialize database connection.
pub(crate) fn init_dal() -> impl Dal {
    // "sqlite::memory:"
    let conn_str = load_db_path();

    // choose the dal implementation here.
    // let dal = dal_diesel::DieselDal { conn_str };
    // let dal = dal_sqlx::SqlxDal { conn_str };
    //let dal = dal_sqlite::SqliteDal {conn_str};
    let dal = dal_rusqlite::RuSqliteDal {conn_str};

    return dal;
}

fn load_config() -> Result<PriceDbConfig, anyhow::Error> {
    let config: PriceDbConfig = confy::load("pricedb", "config")?;

    Ok(config)
}

/// Loads database path from the configuration.
fn load_db_path() -> String {
    let config = load_config().expect("Error reading configuration");

    debug!("configuration: {:?}", config);

    let db_path = config.price_database_path;

    debug!("Db path: {:?}", db_path);

    return db_path;
}

pub(crate) trait Dal {
    /// Inserts a new price record.
    fn add_price(&self, new_price: &NewPrice);

    /// Deletes a price record.
    fn delete_price(&self, id: i32) -> anyhow::Result<usize>;

    /// Returns all the ids of the symbols that have prices in the database.
    /// Used for pruning.
    fn get_ids_of_symbols_with_prices(&self) -> anyhow::Result<Vec<i32>>;

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price>;

    fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>>;

    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security>;

    fn get_security_by_symbol(&self, symbol: &str) -> Security;

    fn get_symbols(&self) -> Vec<SecuritySymbol>;

    fn update_price(&self, id: i32, price: &Price) -> anyhow::Result<usize>;
}
