use std::collections::HashMap;

use async_trait::async_trait;

use crate::model::{Price, SecuritySymbol};

use anyhow::{Ok, Result};

use super::Downloader;

/// YahooFinanceDownloader
#[derive(Debug)]
pub struct YahooFinanceDownloader {
    url: String,
    namespaces: HashMap<&'static str, &'static str>,
}

impl YahooFinanceDownloader {
    pub fn new() -> YahooFinanceDownloader {
        let namespaces = HashMap::from([
            ("AMS", "AS"),
            ("ASX", "AX"),
            ("BATS", ""),
            ("BVME", "MI"),
            ("FWB", "F"),
            ("LSE", "L"),
            ("NASDAQ", ""),
            ("NYSE", ""),
            ("NYSEARCA", ""),
            ("XETRA", "DE"),
        ]);

        YahooFinanceDownloader {
            url: "https://query1.finance.yahoo.com/v8/finance/chart/".to_string(),
            namespaces,
        }
    }

    fn assemble_url(&self, symbol: &SecuritySymbol) -> String {
        let current_namespace = symbol.namespace.as_str();
        let mut local_namespace: &str = current_namespace;

        if self.namespaces.contains_key(current_namespace) {
            local_namespace = self.namespaces[current_namespace];
        }

        let mut url = format!("{}{}", self.url, symbol.mnemonic);

        if local_namespace != "" {
            url = format!("{}.{}", url, local_namespace);
        }

        url
    }
}

#[async_trait]
impl Downloader for YahooFinanceDownloader {
    async fn download(&self, security_symbol: SecuritySymbol, currency: &str) -> Result<Price> {
        let url = self.assemble_url(&security_symbol);

        let body = reqwest::get(url)
            .await?
            //.expect("Huston")
            .text()
            // .json()
            .await?;
            //.expect("Huston?");

        let something: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

        log::debug!("something downloaded: {:?}", something);

        // todo!("parse the price");

        // todo!("replace")
        let result = Price::new();

        Ok(result)
    }
}

/// # Tests
#[cfg(test)]
mod tests {
    use crate::quote::Downloader;
    #[allow(unused_imports)]
    use crate::{model::SecuritySymbol, quote::yahoo_finance_downloader::YahooFinanceDownloader};

    #[test]
    fn test_assemble_url_xetra() {
        let x = YahooFinanceDownloader::new();
        let s = SecuritySymbol {
            namespace: "XETRA".to_string(),
            mnemonic: "EL4X".to_string(),
        };

        let first = x.assemble_url(&s);
        assert_eq!(
            "https://query1.finance.yahoo.com/v8/finance/chart/EL4X.DE",
            first
        );
    }

    #[test]
    fn test_assemble_url_vhyl() {
        let x = YahooFinanceDownloader::new();
        let s = SecuritySymbol {
            namespace: "".to_string(),
            mnemonic: "VHYL".to_string(),
        };

        let first = x.assemble_url(&s);
        assert_eq!(
            "https://query1.finance.yahoo.com/v8/finance/chart/VHYL",
            first
        );
    }

    #[test_log::test(tokio::test)]
    //#[test(tokio::test)]
    //#[test_log::test]
    //#[tokio::test]
    async fn test_download() {
        let o = YahooFinanceDownloader::new();
        let symbol = SecuritySymbol {
            namespace: "XETRA".to_string(),
            mnemonic: "EL4X".to_string(),
        };
        let currency = "EUR";

        let result = o.download(symbol, currency).await.expect("Huston?");

        log::debug!("downloaded {:?}", result);

        assert_eq!(result.currency, "EUR");
    }
}
