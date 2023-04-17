/*!
 * Configuration definition
 */

use serde_derive::{Deserialize, Serialize};

/// The configuration file schema
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDbConfig {
    /// The full path to the price database file.
    pub price_database_path: String,
    pub alphavantage_api_key: String,
    pub fixerio_api_key: String,
    /// The full path to the file where the prices will be exported.
    pub prices_path: String,
    pub symbols_path: String,
}

impl Default for PriceDbConfig {
    fn default() -> Self {
        Self {
            price_database_path: ":memory:".to_owned(),
            alphavantage_api_key: Default::default(),
            fixerio_api_key: Default::default(),
            prices_path: Default::default(),
            symbols_path: Default::default(),
        }
    }
}
