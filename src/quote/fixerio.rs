use std::env::temp_dir;

use anyhow::Result;
use async_trait::async_trait;
use confy::ConfyError;
use tempfile::NamedTempFile;

/// Fixerio downloader
use crate::{model::{NewPrice, SecuritySymbol}, config::PriceDbConfig};

use super::Downloader;

pub struct Fixerio {
    api_key: String,
    cache_path: String
}

impl Fixerio {
    pub fn new() -> Fixerio {
        Fixerio {
            api_key: get_api_key(),
            cache_path: temp_dir().into_os_string().into_string().expect("Error")
        }
    }

    fn latest_rates_exist(&self) -> bool {
        let file_path = self.get_todays_file_path();
    
        todo!("complete");
    }
    
    fn get_todays_file_path(&self) -> String {
        let today = chrono::offset::Local::now();
        let today_str = today.date_naive().format("%Y-%m-%d").to_string();
    
        let result = self.get_rate_file_path(&today_str);

        result
    }
    
    /// Assemble the full file path for the given name (date).
    fn get_rate_file_path(&self, today_iso_str: &str) -> String {
        let cache_path = &self.cache_path;
        let filename = today_iso_str;
        format!("{cache_path}/fixerio_{filename}.json")
    }

}

#[async_trait]
impl Downloader for Fixerio {
    /// Download latest rates. Caches the (daily) prices into a temp directory.
    #[allow(unused_variables)]
    async fn download(&self, security_symbol: SecuritySymbol, currency: &str) -> Result<NewPrice> {
        //let namespace = security_symbol.namespace.to_uppercase();
        let currency = currency.to_uppercase();
        let mnemonic = security_symbol.mnemonic.to_uppercase();

        if mnemonic.contains(":") {
            panic!("Currency symbol should not contain namespace.");
        }

        // if latest_rates_exist() {

        // } else {

        // }

        todo!("complete the implementation")
    }    
}

/// Loads Fixerio API key from the config.
/// Panics if not found.
fn get_api_key() -> String {
    let config_result: Result<PriceDbConfig, ConfyError> = confy::load("pricedb", "config");
    match config_result {
        Ok(config) => config.fixerio_api_key,
        Err(e) => panic!("Fixerio API key not loaded: {}", e)
    } 
}

// Tests

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_read() {
        let key = get_api_key();

        assert_ne!(key, String::default());
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_cache_check() {
        let f = Fixerio::new();
        let result = f.latest_rates_exist();

        assert_eq!(result, false);
    }

    #[test]
    fn test_cache_location() {
        let f = Fixerio::new();
        let result = f.get_todays_file_path();

        assert_ne!(result, String::default());
        // on linux: /tmp/fixerio_2022-12-06.json
        assert_eq!(28, result.len());
    }
}
