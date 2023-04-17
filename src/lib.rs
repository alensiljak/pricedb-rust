/*!
Price Database API

Price Database downloads prices for given securities, stores it in an sqlite database,
and exports in Ledger-cli format.

Project [Documentation](https://github.com/alensiljak/pricedb-rust).
*/

use as_symbols::SymbolMetadata;
use config::PriceDbConfig;

pub mod config;
pub mod model;
pub mod price_flat_file;
mod quote;

use crate::{
    model::*,
    price_flat_file::{PriceFlatFile, PriceRecord},
    quote::Quote,
};

use std::{path::PathBuf, vec};

use anyhow::Error;

pub const APP_NAME: &str = "pricedb";

/**
 * Application logic
 * Exposing the main app functionality as a library. This is useful for testing purposes,
 * as well as for utilization by external tools.
 */
pub struct App {
    config: PriceDbConfig,
}

impl App {
    pub fn new(config: PriceDbConfig) -> Self {
        Self {
            config,
        }
    }

    pub fn config_show(&self) {
        let path =
            confy::get_configuration_file_path(APP_NAME, APP_NAME).expect("configuration path");
        let cfg = &self.config;

        println!("Configuration file: {}", path.display());
        println!("{cfg:?}");
    }

    /// Download directly into the price file in ledger format.
    /// Maintains the latest prices in the price file by updating the prices for
    /// existing symbols and adding any new ones.
    pub async fn dl_quote(
        &self,
        symbols_path_param: &Option<String>,
        price_path_param: &Option<String>,
        filter: SecurityFilter,
    ) {
        let (symbols_path, price_path) =
            self.get_quote_params(symbols_path_param, price_path_param);

        // load the symbols table for mapping
        let securities = self.get_securities(Some(&symbols_path), Some(filter));

        // load existing prices from the file
        let mut prices_file = PriceFlatFile::load(&price_path);
        // log::debug!("prices: {:?}", prices);

        // progress bar init.
        let mut counter_updated = 0;
        let sec_count = securities.len().try_into().unwrap();
        let pb = indicatif::ProgressBar::new(sec_count);
        pb.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=>-"));

        // download prices, as per filters
        for sec in securities {
            let symbol = SecuritySymbol {
                namespace: sec.namespace.as_ref().unwrap().to_owned(),
                mnemonic: sec.symbol.to_owned(),
            };

            let price = download_price(
                &symbol,
                &sec.currency.as_ref().unwrap().to_owned(),
                match &sec.updater {
                    Some(ag) => ag,
                    None => "",
                },
            )
            .await
            .expect("Error fetching price");

            log::debug!("the fetched price for {:?} is {:?}", sec.symbol, price);

            // convert
            let mut price_record = PriceRecord::from(&price);
            // Use ledger symbol.
            price_record.symbol = sec.get_symbol();

            // Add the record. The symbol is used as the key.
            prices_file
                .prices
                .insert(price_record.symbol.to_owned(), price_record);

            // update progress bar
            // pb.println(msg);
            counter_updated += 1;
            pb.inc(1);
        }

        // save the file
        // log::debug!("current values: {:?}", self.prices);
        prices_file.save();

        pb.finish();
        println!("Added/updated {counter_updated} prices.\n");
    }

    // Private

    /// Gets the configuration parameters for quote dl.
    /// Reads from the configuration file if not provided on the command line.
    fn get_quote_params(
        &self,
        symbols_path_param: &Option<String>,
        price_path_param: &Option<String>,
    ) -> (String, String) {
        // symbols
        let symbol_path = match symbols_path_param {
            Some(path) => path.to_string(),
            None => self.config.symbols_path.to_owned(),
        };

        // prices
        let prices_path = match price_path_param {
            Some(path) => path.to_string(),
            None => self.config.prices_path.to_owned(),
        };

        (symbol_path, prices_path)
    }

    /// Load symbols list, applying the filters.
    fn get_securities(
        &self,
        symbols_path: Option<&str>,
        filter: Option<SecurityFilter>,
    ) -> Vec<SymbolMetadata> {
        let symbols_file_path = match symbols_path {
            Some(path) => path,
            None => &self.config.symbols_path,
        };
        let list = self
            .load_symbols(symbols_file_path)
            .expect("symbols loaded");

        if filter.is_none() {
            return list;
        }

        let filter_val = filter.unwrap();

        list.into_iter()
            .filter(|sym| match &filter_val.agent {
                Some(agent) => match &sym.updater {
                    Some(updater) => agent == updater,
                    None => true,
                },
                None => true,
            })
            .filter(|sym| match &filter_val.currency {
                Some(filter_currency) => match &sym.currency {
                    Some(sym_currency) => sym_currency == &filter_currency.to_uppercase(),
                    None => true,
                },
                None => true,
            })
            .filter(|sym| match &filter_val.exchange {
                Some(filter_exchange) => match &sym.namespace {
                    Some(sym_namespace) => sym_namespace == &filter_exchange.to_uppercase(),
                    None => true,
                },
                None => true,
            })
            .filter(|sym| match &filter_val.symbol {
                Some(filter_symbol) => &sym.symbol == &filter_symbol.to_uppercase(),
                None => true,
            })
            .collect()
    }

    fn load_symbols(&self, symbols_path: &str) -> Result<Vec<SymbolMetadata>, Error> {
        let path = PathBuf::from(symbols_path);
        as_symbols::read_symbols(&path)
    }

}

async fn download_price(symbol: &SecuritySymbol, currency: &str, agent: &str) -> Option<Price> {
    // todo: there must be a symbol
    let mut dl = Quote::new();

    dl.set_source(agent);
    dl.set_currency(currency);

    let prices = dl.fetch(&symbol.namespace, vec![&symbol.mnemonic]).await;

    if prices.is_empty() {
        println!("Did not receive any prices");
        return None;
    }

    let price = prices[0].to_owned();
    Some(price)
}

pub fn load_config() -> PriceDbConfig {
    let config_path = confy::get_configuration_file_path(APP_NAME, APP_NAME)
        .expect("config path retrieved");
    println!("Using config {:?}", config_path);

    let config: PriceDbConfig =
        confy::load(APP_NAME, APP_NAME).expect("valid config should be loaded");

    config
}

#[cfg(test)]
mod tests {
    use rstest::fixture;

    use crate::{config::PriceDbConfig, App, model::SecurityFilter};

    #[fixture]
    fn dbg_config() -> PriceDbConfig {
        let mut cfg = PriceDbConfig::default();
        cfg.symbols_path = "tests/symbols.csv".into();
        // cfg.prices_path = "tests/prices.txt".into();
        cfg
    }

    #[fixture]
    fn app_dbg(dbg_config: PriceDbConfig) -> App {
        App::new(dbg_config)
    }

    #[rstest::rstest]
    fn test_getting_securities(app_dbg: App) {
        let actual = app_dbg.get_securities(None, None);

        assert!(!actual.is_empty());
        assert_eq!(3, actual.len());
    }

    // debugging test
    #[tokio::test]
    async fn test_vanguard_datetime() {
        let cfg = dbg_config();
        let app = App::new(cfg);
        
        let mut filter = SecurityFilter::new();
        filter.symbol = Some("hy".into());

        app.dl_quote(&None, &Some("tests/prices.txt".into()), filter).await;

        
    }
}
