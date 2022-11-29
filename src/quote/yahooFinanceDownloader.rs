use std::collections::HashMap;

use crate::model::{Price, SecuritySymbol};

use super::Downloader;

/// YahooFinanceDownloader
#[derive(Debug)]
pub struct YahooFinanceDownloader {
    url: String,
    namespaces: HashMap<&'static str, &'static str>
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
            ("XETRA", "DE")
        ]);

        YahooFinanceDownloader {
            url: "https://query1.finance.yahoo.com/v8/finance/chart/".to_string(),
            namespaces
        }
    }

    fn assemble_url(&self, symbol: SecuritySymbol) -> &String {
        if self.namespaces.contains_key(&symbol.namespace.as_str()) {
            log::debug!("found!");
        }
        todo!("assemble the url");

        return &self.url;
    }

}

impl Downloader for YahooFinanceDownloader {
    fn download(
        &self,
        security_symbol: crate::model::SecuritySymbol,
        currency: &str,
    ) -> crate::model::Price {
        let url = self.assemble_url(security_symbol);

        todo!("download price here");

        // todo!("parse the price");

        // todo!("replace")
        let result = Price::new();
        
        result
    }

}
