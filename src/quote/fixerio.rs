use std::{env::temp_dir, fs, path::Path, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use confy::ConfyError;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde_json::Value;

/// Fixerio downloader
use crate::{
    config::PriceDbConfig,
    model::{SecuritySymbol, Price}, APP_NAME,
};

use super::Downloader;

pub struct Fixerio {
    api_key: String,
}

impl Fixerio {
    pub fn new() -> Self {
        Self {
            api_key: get_fixerio_api_key(),
        }
    }

    /// Saves the retrieved rates into a cache file.
    fn cache_rates(&self, rates: &Value) {
        let file_date = rates["date"].as_str().unwrap();
        let file_path = get_rate_file_path(file_date);

        let content = rates.to_string();

        let path = Path::new(&file_path);
        match fs::write(path, content) {
            Ok(_) => (),
            Err(_) => panic!("Could not cache rates"),
        }
    }

    /// Downloads the latest rates. Requires base currency and a list of currencies to
    /// retrieve.
    /// # Returns
    /// json response object from Fixer.io.
    async fn download_rates(&self, base_currency: &str) -> Result<Value> {
        let base_url = "http://data.fixer.io/api/latest";
        let api_key = self.api_key.as_str();
        let url = format!("{base_url}?base={base_currency}&access_key={api_key}");

        let result: Value = reqwest::get(url)
            .await?
            .json()
            .await
            .expect("Error retrieving quotes");

        Ok(result)
    }

    fn latest_rates_exist(&self) -> bool {
        let file_path = get_todays_file_path();

        let exists = std::path::Path::new(&file_path).exists();
        exists
    }
}

#[async_trait]
impl Downloader for Fixerio {
    /// Download latest rates. Caches the (daily) prices into a temp directory.
    #[allow(unused_variables)]
    async fn download(&self, security_symbol: SecuritySymbol, currency: &str) -> Result<Price> {
        //let namespace = security_symbol.namespace.to_uppercase();
        let currency = currency.to_uppercase();
        let mnemonic = security_symbol.mnemonic.to_uppercase();

        if mnemonic.contains(':') {
            panic!("Currency symbol should not contain namespace.");
        }

        let rates_json: Value;
        if self.latest_rates_exist() {
            log::debug!("Reading cached rates");
            rates_json = read_rates_from_cache();

            // log::debug!("Read rates from the cache file: {:?}", rates_json);
        } else {
            rates_json = self
                .download_rates(&currency)
                .await
                .expect("Error downloading rates");

            self.cache_rates(&rates_json);
        }

        log::debug!("Mapping rates for {}", &mnemonic);
        let rate = map_rates_to_price(rates_json, &mnemonic);

        Ok(rate)
    }
}

/// Loads Fixerio API key from the config.
/// Panics if not found.
fn get_fixerio_api_key() -> String {
    let config_result: Result<PriceDbConfig, ConfyError> = confy::load(APP_NAME, None);
    match config_result {
        Ok(config) => config.fixerio_api_key,
        Err(e) => panic!("Fixerio API key not loaded: {}", e),
    }
}

fn get_cache_path() -> String {
    let cache_path = temp_dir().into_os_string().into_string().expect("Error");

    cache_path
}
/// Assemble the full file path for the given name (date).
fn get_rate_file_path(today_iso_str: &str) -> String {
    let cache_path = get_cache_path();
    let filename = today_iso_str;
    format!("{cache_path}/fixerio_{filename}.json")
}

fn get_todays_file_path() -> String {
    let today = chrono::offset::Local::now();
    let today_str = today.date_naive().format("%Y-%m-%d").to_string();

    let result = get_rate_file_path(&today_str);

    result
}

/// Read and map a single currency rate
/// symbol: The currency to fetch the rate for.
fn map_rates_to_price(rates: Value, symbol: &str) -> Price {
    let date_str = rates["date"].as_str().unwrap().to_string();

    // Get value

    let base = rates["base"].as_str().unwrap().to_string();
    let rates_dict = &rates["rates"];
    let rate_node = &rates_dict[symbol];
    
    log::debug!("Rate located: {:?}", rate_node);

    let value_f = rate_node.as_f64().unwrap();
    let value = Decimal::from_f64(value_f).expect("Error parsing value");
    // The rate is inverse value.
    let rate = Decimal::ONE / value;
    
    log::debug!("The inverse rate is {:?}", rate);

    // Round to 6 decimals max.
    let rounded_str = format!("{0:.6}", rate);
    let rounded = Decimal::from_str(&rounded_str).unwrap();
    
    log::debug!("Rounded rate: {rounded:?}");

    // result

    Price {
        id: i32::default(),
        security_id: i32::default(),
        date: date_str,
        time: None,
        value: rounded.mantissa().to_i32().unwrap(),
        denom: 10_i32.pow(rounded.scale()),         // in 10^3 = 1000, scale=3, denom=1000
        currency: base,
    }
}

fn read_rates_from_cache() -> Value {
    let file_path = get_todays_file_path();

    log::debug!("Loading rates from {}", file_path);

    let content = fs::read_to_string(file_path).expect("Error reading rates file");

    let result = serde_json::from_str(&content).expect("Error parsing rates JSON");
    // log::debug!("parsed rates: {:?}", result);

    result
}

// Tests

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    /// This test depends on having a value 
    #[test]
    fn test_config_read() {
        let key = get_fixerio_api_key();

        assert_ne!(key, String::default());
        assert_eq!(key.len(), 32);
    }

    /// Cached rates must exist after fetching.
    #[tokio::test]
    async fn test_cache_check() {
        let f = Fixerio::new();
        f.download_rates("EUR").await.expect("rates fetched");
        let result = f.latest_rates_exist();

        assert_eq!(true, result);
    }

    #[test]
    fn test_cache_location() {
        let result = get_todays_file_path();

        println!("Fixerio cache file: {result:?}");

        assert_ne!(result, String::default());
        // on linux: /tmp/fixerio_2022-12-06.json
        assert_eq!(28, result.len());
    }

    #[test_log::test(tokio::test)]
    async fn test_download() {
        let f = Fixerio::new();
        let symbol = SecuritySymbol::parse("AUD");
        let price = f
            .download(symbol, "EUR")
            .await
            .expect("Error downloading price");

        // let expected = NewPrice {
        //     currency: "EUR".to_string(),
        //     security_id: i32::default(),
        //     date: chrono::Local::now().date_naive().to_string(),
        //     time: None,
        //     value: 10,
        //     denom: 10,
        // };
        assert_eq!(price.currency, "EUR");
        // assert_eq!(expected, price);
    }

    // Frequent downloads consume the API quota. Use the cached version.
    // #[test_log::test(tokio::test)]
    // async fn test_dl_rates() {
    //     let f = Fixerio::new();
    //     let result = f.download_rates("EUR").await.expect("Error");
    //     assert_eq!(result["base"], "EUR");
    //     assert_ne!(result["rates"]["BAM"], String::default());
    // }

    #[test_log::test(tokio::test)]
    async fn test_price_parsing_AUD() {
        let symbol = SecuritySymbol::parse("CURRENCY:AUD");

        let f = Fixerio::new();
        let price = f.download(symbol, "EUR").await.expect("Error");

        let value = price.to_decimal();
        
        println!("Parsing AUDEUR rate...");
        println!("parsed price: {price:?}");
        println!("price value: {value:?}");

        assert!(price.value > 0);
        assert!(false);
    }

    #[test_log::test(tokio::test)]
    async fn test_price_parsing_GBP() {
        let symbol = SecuritySymbol::parse("CURRENCY:GBP");

        let f = Fixerio::new();
        let price = f.download(symbol, "EUR").await.expect("Error");

        let value = price.to_decimal();
        
        println!("Parsing AUDEUR rate...");
        println!("parsed price: {price:?}");
        println!("price value: {value:?}");

        assert!(price.value > 0);
        assert!(false);
    }
}
