/*!
 * Configuration definition
 */

use serde_derive::{Deserialize, Serialize};

/// The configuration file schema
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PriceDbConfig {
    /// The full path to the price database file.
    pub price_database_path: String,
    pub alphavantage_api_key: String,
    pub fixerio_api_key: String,
    /// The full path to the file where the prices will be exported.
    pub export_destination: String,
}
