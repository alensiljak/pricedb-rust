/*
 * trying to encapsulate database-specific code
 */

pub struct Database {
    db: Rbatis
}

impl Database {
    pub fn initialize(&self) {
        let db_path: String = load_db_path();

        let db = repositories::initialize_database(db_path);
    }
}

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
