use anyhow::Result;
use async_trait::async_trait;
use confy::ConfyError;

/// Fixerio downloader
use crate::{model::{NewPrice, SecuritySymbol}, config::PriceDbConfig};

use super::Downloader;

pub struct Fixerio {
    api_key: String
}

impl Fixerio {
    pub fn new() -> Fixerio {
        let key = get_api_key();
        Fixerio {
            api_key: key
        }
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

        todo!("implement")
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
}
