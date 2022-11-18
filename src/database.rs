/*
 * trying to encapsulate database-specific code
 */

use confy::ConfyError;
use log::{debug, error};
// use tracing::{debug, error};

use crate::{config::PriceDbConfig};

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
    fn get_securities();
}