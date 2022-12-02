/**
 * Quote
 * Fetching prices
 */
mod fixerio;
mod yahoo_finance_downloader;

use anyhow::Result;
use async_trait::async_trait;

use crate::{
    model::{Price, SecuritySymbol, NewPrice},
    quote::{fixerio::Fixerio, yahoo_finance_downloader::YahooFinanceDownloader},
};

#[derive(Debug)]
pub struct Quote {
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub source: Option<String>,
    pub currency: Option<String>,
}

impl Quote {
    pub fn new() -> Quote {
        Quote {
            symbol: None,
            exchange: None,
            source: None,
            currency: None,
        }
    }

    pub async fn fetch(&self, exchange: &str, symbols: Vec<String>) -> Vec<NewPrice> {
        let mut result = vec![];

        for symbol in symbols {
            let price = self
                .download(exchange, &symbol)
                .await
                .expect("Did not receive price");
            result.push(price);
        }

        result
    }

    async fn download(&self, exchange: &str, symbol: &str) -> Option<NewPrice> {
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

        let actor: Box<dyn Downloader>;
        match self.source.as_ref().unwrap().as_str() {
            "yahoo_finance" => {
                println!("should use yahoo");
                actor = Box::new(YahooFinanceDownloader::new());
            }
            "fixerio" => {
                println!("use fixerio");
                actor = Box::new(Fixerio::new());
            }
            _ => {
                panic!("yo!");
            }
        }

        let price = actor
            .download(sec_symbol, self.currency.as_ref().unwrap().as_str())
            .await
            .expect("Huston?");

        Some(price)
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
    async fn download(&self, security_symbol: SecuritySymbol, currency: &str) -> Result<NewPrice>;
}
