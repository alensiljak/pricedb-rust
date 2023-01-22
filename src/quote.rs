/*!
 * Quote implementation in Rust.
 * Fetching prices.
 *
 * Based on [Price Database](https://gitlab.com/alensiljak/price-database),
 * Python library.
 */
mod fixerio;
mod vanguard_au;
mod yahoo_finance_downloader;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    model::{Price, SecuritySymbol},
    quote::{
        fixerio::Fixerio, vanguard_au::VanguardAuDownloader,
        yahoo_finance_downloader::YahooFinanceDownloader,
    },
};

#[derive(Debug)]
pub struct Quote {
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub source: Option<String>,
    pub currency: Option<String>,
}

impl Quote {
    pub fn new() -> Self {
        Self {
            symbol: None,
            exchange: None,
            source: None,
            currency: None,
        }
    }

    /// Fetch prices for the given symbols.
    pub async fn fetch(&self, exchange: &str, symbols: Vec<String>) -> Vec<Price> {
        let mut result = vec![];

        for symbol in symbols {
            // log::debug!("Downloading price for {:?}", symbol);

            let price = self
                .download(exchange, &symbol)
                .await
                .expect("Did not receive price");
            result.push(price);
        }

        result
    }

    async fn download(&self, exchange: &str, symbol: &str) -> Option<Price> {
        if exchange != exchange.to_uppercase() {
            panic!("handle this case!");
        }

        if self.currency.is_some() {
            let currency_val = self.currency.clone().unwrap();
            if currency_val != currency_val.to_uppercase() {
                panic!("currency must be uppercase!");
            }
        }

        let sec_symbol = SecuritySymbol {
            namespace: exchange.to_owned(),
            mnemonic: symbol.to_owned(),
        };
        // todo: parse symbol

        let actor = self.get_downloader();
        let currency = self.currency.as_ref().unwrap().as_str();

        log::debug!(
            "Calling download with symbol {} and currency {}",
            sec_symbol,
            currency
        );

        let mut price = actor
            .download(&sec_symbol, currency)
            .await
            .expect("downloaded price");

        // Set the symbol here.
        price.symbol = format!("{}:{}", sec_symbol.namespace, sec_symbol.mnemonic);

        Some(price)
    }

    fn get_downloader(&self) -> Box<dyn Downloader> {
        match self.source.as_ref().unwrap().as_str() {
            "yahoo_finance" => {
                log::trace!("using yahoo finance");
                Box::new(YahooFinanceDownloader::new())
            }
            "fixerio" => {
                log::trace!("using fixerio");
                Box::new(Fixerio::new())
            }
            "vanguard_au" => {
                log::trace!("using vanguard");
                Box::new(VanguardAuDownloader::new())
            }
            _ => {
                panic!("unknown downloader: {}", self.source.as_ref().unwrap());
            }
        }
    }

    // fn currency() {}

    pub fn set_currency(&mut self, currency: &str) {
        self.currency = Some(currency.to_string().to_uppercase());
    }

    pub fn set_source(&mut self, source: &str) {
        self.source = Some(source.to_string());
    }
}

#[async_trait]
trait Downloader {
    async fn download(&self, security_symbol: &SecuritySymbol, currency: &str) -> Result<Price>;
}
