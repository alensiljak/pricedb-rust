/*
 * Configuration reader
 */

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PriceDbConfig {
    pub(crate) price_database_path: String,
    pub(crate) alphavantage_api_key: String,
    pub(crate) fixerio_api_key: String,
    pub(crate) export_destination: String,
}

/// `PriceDbConfig` implements `Default`
impl ::std::default::Default for PriceDbConfig {
    fn default() -> Self {
        Self {
            price_database_path: "/home/user/pricedb/prices.db".to_string(),
            alphavantage_api_key: "".to_string(),
            fixerio_api_key: "".to_string(),
            export_destination: "/home/user/pricedb/export.txt".to_string(),
        }
    }
}
