use crate::model::Price;

use super::Downloader;

/// YahooFinanceDownloader
#[derive(Debug)]
pub struct YahooFinanceDownloader {
    url: String,
}

impl YahooFinanceDownloader {
    pub fn new() -> YahooFinanceDownloader {
        YahooFinanceDownloader {
            url: "https://query1.finance.yahoo.com/v8/finance/chart/".to_string(),
        }
    }

    fn assemble_url(&self) -> &String {
        
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
        let url = self.assemble_url();

        todo!("download price here");

        // todo!("parse the price");

        // todo!("replace")
        let result = Price::new();
        
        result
    }

}
