/*
 * trying to encapsulate database-specific code
 */
// mod dal_diesel;            // requires schema; complex
// pub(crate) mod schema;
// mod dal_sqlx;              // async-only
// mod dal_sqlite;
mod dal_rusqlite;
mod mappers_rusqlite;

use log::debug;

use crate::model::{Price, PriceFilter, Security, SecurityFilter};

/// Initialize database connection.
/// `db_path` is the path to the file.
/// 
/// "sqlite::memory:" or ":memory:"
pub(crate) fn init_dal(db_path: &String) -> impl Dal {
    validate_db_path(db_path);

    // choose the dal implementation here.
    // let dal = dal_diesel::DieselDal { db_path };
    // let dal = dal_sqlx::SqlxDal { db_path };
    //let dal = dal_sqlite::SqliteDal {db_path};
    dal_rusqlite::RuSqliteDal::new(db_path.to_owned())
}

fn validate_db_path(db_path: &String) {
    // let config = super::load_config().expect("Error reading configuration");
    // debug!("configuration: {:?}", config);
    // let db_path = config.price_database_path;

    debug!("Db path: {:?}", db_path);

    if db_path.eq("") {
        panic!(
            r#"The database path has not been configured. 
            Please edit the config file manually and add the database file path.
            Run `pricedb config show` to display the exact location of the config file."#
        )
    }
}

pub(crate) trait Dal {
    /// Inserts a new price record.
    fn add_price(&self, new_price: &Price) -> usize;

    /// Deletes a price record.
    fn delete_price(&self, id: i32) -> anyhow::Result<usize>;

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price>;

    fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>>;

    fn get_prices_with_symbols(&self) -> Vec<Price>;

    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security>;

    /// Returns all the ids of the symbols that have prices in the database.
    /// Used for pruning.
    fn get_securitiess_having_prices(&self) -> Vec<Security>;

    fn get_security_by_symbol(&self, symbol: &str) -> Security;

    // fn get_symbols(&self) -> Vec<SecuritySymbol>;

    fn update_price(&self, price: &Price) -> anyhow::Result<usize>;

    //// Schema

    fn create_tables(&self);
}
