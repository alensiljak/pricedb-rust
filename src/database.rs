/*
 * trying to encapsulate database-specific code
 */
// mod dal_diesel;
// mod schema;
mod dal_sqlx;

use confy::ConfyError;
use log::{debug, error};

use crate::{config::PriceDbConfig, model::Security};

/// Loads database path from the configuration.
pub fn load_db_path() -> String {
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

trait Dal {
    fn get_securities(currency: Option<String>, agent: Option<String>, 
        mnemonic: Option<String>, exchange: Option<String>) -> Vec<Security>;
}

pub fn get_securities(
    currency: Option<String>,
    agent: Option<String>,
    mnemonic: Option<String>,
    exchange: Option<String>,
) -> Vec<Security> {
    todo!("proxy to the current dal")
}