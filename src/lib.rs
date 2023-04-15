/*!
Price Database API

Price Database downloads prices for given securities, stores it in an sqlite database,
and exports in Ledger-cli format.

Project [Documentation](https://github.com/alensiljak/pricedb-rust).
*/

use as_symbols::SymbolMetadata;
use config::PriceDbConfig;
use once_cell::unsync::OnceCell;
use rust_decimal::prelude::ToPrimitive;

pub mod config;
mod database;
mod ledger_formatter;
pub mod model;
mod price_flat_file;
mod quote;

use crate::{database::Dal, model::*, quote::Quote, price_flat_file::{PriceFlatFile, PriceRecord}};

use std::{fs, path::PathBuf, vec};

use anyhow::Error;

pub const APP_NAME: &str = "pricedb";

/**
 * Application logic
 * Exposing the main app functionality as a library. This is useful for testing purposes,
 * as well as for utilization by external tools.
 */
pub struct App {
    config: PriceDbConfig,
    dal: OnceCell<Box<dyn Dal>>,
}

impl App {
    pub fn new(config: PriceDbConfig) -> Self {
        Self {
            config,
            dal: OnceCell::new(),
        }
    }

    pub fn add_price(&self, new_price: &Price) -> AdditionResult {
        log::debug!("Adding price {:?}", new_price);

        let dal = self.get_dal();

        // Is there already a price with the same id, date, and time?

        let filter = PriceFilter {
            // security_id: Some(new_price.security_id),
            symbol: Some(new_price.symbol.to_owned()),
            date: Some(new_price.date.to_owned()),
            time: Some(new_price.time.to_owned()),
        };
        let existing_prices = dal.get_prices(Some(filter));

        // insert or update
        let mut result = AdditionResult::default();
        if existing_prices.is_empty() {
            // insert
            result.inserted = self.insert_price(new_price).to_u16().unwrap();
        } else {
            // update
            result.updated = self
                .update_price(existing_prices, new_price)
                .to_u16()
                .unwrap();
        };
        result
    }

    pub fn config_show(&self) {
        let path =
            confy::get_configuration_file_path(APP_NAME, APP_NAME).expect("configuration path");
        let cfg = &self.config;

        println!("Configuration file: {}", path.display());
        println!("{cfg:?}");
    }

    pub async fn download(&self, filter: SecurityFilter) {
        log::debug!("download filter: {:?}", filter);

        let securities = self.get_securities(None, Some(filter));

        // Debug
        log_securities(&securities);

        if securities.is_empty() {
            println!("No Securities found for the given parameters.");
            return;
        }

        // download

        // progress bar init.
        let mut counter_updated = 0;
        let sec_count = securities.len().try_into().unwrap();
        let pb = indicatif::ProgressBar::new(sec_count);
        pb.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=>-"));

        for sec in securities {
            let symbol = SecuritySymbol {
                namespace: sec.namespace.unwrap().to_owned(),
                mnemonic: sec.symbol.to_owned(),
            };

            let price = download_price(
                &symbol,
                sec.currency.unwrap().as_str(),
                sec.updater.unwrap().as_str(),
            )
            .await
            .expect("Error fetching price");

            //price.security_id = sec.id;

            log::debug!("the fetched price for {:?} is {:?}", sec.symbol, price);

            // save to database

            let saved = self.add_price(&price);

            let symbol = match sec.ledger_symbol {
                Some(ledger_symbol) => ledger_symbol,
                None => sec.symbol,
            };
            if saved.inserted > 0 {
                let msg = format!(
                    "Added {} {} {} {} {}",
                    symbol,
                    price.date,
                    price.time,
                    price.to_decimal(),
                    price.currency
                );
                pb.println(msg);
                counter_updated += 1;
            }
            if saved.updated > 0 {
                let msg = format!(
                    "Updated {} {} {} {} {}",
                    symbol,
                    price.date,
                    price.time,
                    price.to_decimal(),
                    price.currency
                );
                pb.println(msg);
                counter_updated += 1;
            }

            // update progress bar
            pb.inc(1);
        }

        pb.finish();
        println!("Added/updated {counter_updated} prices.\n");
    }

    /// Load and display all prices.
    /// Also returns the list as a string, for testing.
    pub fn list_prices(
        &self,
        _date: &Option<String>,
        _currency: &Option<String>,
        _last: &Option<String>,
    ) -> String {
        let mut prices = self.get_dal().get_prices(None);

        // sort by symbol
        prices.sort_unstable_by_key(|p| p.symbol.to_owned());

        let mut result = String::new();
        for pws in prices {
            let output = format!(
                "{} {} {} {:?} {}",
                pws.symbol,
                pws.date,
                pws.time,
                pws.to_decimal(),
                pws.currency
            );
            println!("{output}");
            result += &output;
        }

        result
    }

    /// Prune historical prices for the given symbol, leaving only the latest.
    /// If no symbol is given, it prunes all existing symbols.
    /// Returns the number of items removed.
    pub fn prune(&self, symbol: &Option<String>) -> u16 {
        println!("Pruning prices...");

        log::trace!("for symbol: {:?}", symbol);

        let dal = self.get_dal();
        let mut security_ids = vec![];

        if symbol.is_some() {
            // get id
            let symb = symbol.as_ref().unwrap().as_str();
            // let security = dal.get_security_by_symbol(symb);
            //security_ids.push(security.id);
            let full_symbol = SecuritySymbol::new(symb);
            security_ids.push(full_symbol.to_string());
        } else {
            // load all symbols
            security_ids = dal
                .get_securitiess_having_prices()
                .iter()
                .map(|item| item.to_string())
                .collect();
        }

        let mut count: u16 = 0;
        let mut count_deleted = 0;

        // init progress bar
        let pb = indicatif::ProgressBar::new(security_ids.len().to_u64().unwrap());
        // pb.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=>-"));

        // Send the symbols to the individual prune.
        for security_id in security_ids {
            // move progress bar
            pb.inc(1);

            if let Result::Ok(i) = self.prune_for_sec(&security_id) {
                // success. Log only if something was deleted.
                if i > 0 {
                    log::debug!("deleted {:?} records for {:?}", i, security_id);
                    count_deleted += 1;
                }
            } else {
                // error?
                log::warn!("Error pruning for {:?}", security_id);
            }

            count += 1;
        }

        println!("Processed {} records, deleted {}.", count, count_deleted);

        count
    }

    /// Export prices in ledger format
    pub fn export(&self) {
        let mut prices = self.get_dal().get_prices(None);

        prices
            .sort_unstable_by_key(|p| (p.date.to_owned(), p.time.to_owned(), p.symbol.to_owned()));

        let symbols = self
            .load_symbols(&self.config.symbols_path)
            .expect("symbols loaded");

        // format in ledger format
        let output = ledger_formatter::format_prices(prices, symbols);

        // get export destination from configuration
        let target = &self.config.export_destination;

        println!("Prices exported to {target}");

        save_text_file(&output, target);
    }

    /// Download directly into the price file in ledger format.
    /// Maintains the latest prices in the price file by updating the prices for
    /// existing symbols and adding any new ones.
    pub async fn dl_quote(&self, symbols_path: &str, price_path: &str, filter: SecurityFilter) {
        // load the symbols table for mapping
        let securities = self.get_securities(Some(symbols_path), Some(filter));
        // let symbols = self.load_symbols(symbols_path).expect("symbols loaded");
        // log::debug!("symbols: {:?}", symbols);

        // load existing prices from the file
        let mut prices_file = PriceFlatFile::load(price_path);
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
            prices_file.prices.insert(price_record.symbol.to_owned(), price_record);

            // update progress bar
            counter_updated += 1;
            pb.inc(1);
        }

        // save the file
        // log::debug!("current values: {:?}", self.prices);
        prices_file.save();

        pb.finish();
        println!("Added/updated {counter_updated} prices.\n");
    }

    pub fn get_dal(&self) -> &Box<dyn Dal> {
        self.dal.get_or_init(|| {
            let dal = database::init_dal(&self.config.price_database_path);

            // Create tables if needed.
            let tables = dal.get_tables();
            if tables.is_empty() {
                log::debug!("Creating tables...");
                dal.create_tables();
            }

            Box::new(dal)
        })
    }

    // Private

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

    // fn get_security_by_symbol(&self, symbol: &str) -> Option<SecuritySymbol> {
    // use std::{fs, path::PathBuf, vec};
    //     // load all symbols
    //     let path = PathBuf::from(&self.config.symbols_path);
    //     let symbols = as_symbols::read_symbols(&path).expect("symbols parsed");

    //     symbols.iter().find_map(|sym| {
    //         let s = SecuritySymbol::new_separated(
    //             &sym.namespace.as_ref().unwrap(),
    //             &sym.symbol,
    //         );
    //         match s.to_string() == symbol || s.mnemonic == symbol {
    //             true => Some(s),
    //             false => None
    //         }
    //     })
    // }

    fn insert_price(&self, new_price: &Price) -> usize {
        // log::debug!("\nInserting {new_price:?}");

        let dal = self.get_dal();

        dal.add_price(new_price)
    }

    // fn load_all_prices_with_symbols(&self) -> Vec<PriceWSymbol> {
    //     let dal = self.get_dal();
    //     let prices = dal.get_prices(None);
    //     let securities = dal.get_securities(None);
    //     prices
    //         .iter()
    //         .map(|price| {
    //             let sec = securities
    //                 .iter()
    //                 .find(|sec| sec.id == price.security_id)
    //                 .expect("a related security");
    //             PriceWSymbol::from(price, sec)
    //         })
    //         .collect()
    // }

    fn load_symbols(&self, symbols_path: &str) -> Result<Vec<SymbolMetadata>, Error> {
        let path = PathBuf::from(symbols_path);
        as_symbols::read_symbols(&path)
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, symbol: &str) -> anyhow::Result<u16, Error> {
        log::trace!("pruning prices for security: {:?}", symbol);

        let mut count = 0;
        // get prices for the given security
        let dal = self.get_dal();
        let prices = dal
            .get_prices_for_security(symbol)
            .expect("Error fetching prices for security");

        // log::debug!("prices for {:?} - {:?}", security_id, prices);
        //log::debug!("received {} prices", prices.len());

        let size = prices.len();
        if size <= 1 {
            // nothing to delete
            log::debug!("Nothing to prune for {:?}", symbol);
            return Ok(0);
        }

        // skip the first
        let to_prune = &prices[1..];

        // delete
        for price in to_prune {
            log::debug!("deleting price: {:?}", price);

            dal.delete_price(price.id)?;
            count += 1;
        }

        Ok(count)
    }

    fn update_price(&self, existing_prices: Vec<Price>, new_price: &Price) -> usize {
        log::debug!("Updating");

        let dal = self.get_dal();

        // get an existing record
        let existing = existing_prices.first().expect("error fetching security");

        log::debug!("Existing price found: {existing:?}");

        if new_price.currency != existing.currency {
            log::error!(
                "The currencies are different {:?} vs {:?}",
                new_price.currency,
                existing.currency
            );
            panic!("The currencies differ!");
        }

        let mut should_update = false;
        let mut for_update = Price::new();

        for_update.id = existing.id;

        // log::debug!("clone of the price to update: {:?}", for_update);

        if existing.value != new_price.value {
            log::debug!(
                "Updating value from {:?} to {}",
                existing.value,
                new_price.value
            );

            for_update.value = new_price.value;
            should_update = true;
        }
        if existing.denom != new_price.denom {
            log::debug!("Updating denom {} to {}", existing.denom, new_price.denom);

            for_update.denom = new_price.denom;
            should_update = true;
        }

        // Exit if there's nothing to update.
        if !should_update {
            log::debug!("Nothing to update");
            return 0;
        };

        //log::debug!("updating record {new_price:?}");
        // println!("for {new_price:?}");

        dal.update_price(&for_update).unwrap()
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

fn log_securities(securities: &Vec<SymbolMetadata>) {
    let symbols: Vec<String> = securities
        .iter()
        .map(|sec| {
            format!(
                "{}:{}",
                sec.namespace.as_ref().unwrap().as_str(),
                sec.symbol.as_str()
            ) as String
        })
        .collect();
    log::debug!("Securities to fetch the prices for: {:?}", symbols);
}

fn save_text_file(contents: &String, location: &String) {
    fs::write(location, contents).expect("file saved");
}

#[derive(Default)]
/// The result of adding records (insert, update)
pub struct AdditionResult {
    inserted: u16,
    updated: u16,
}

pub fn load_config() -> PriceDbConfig {
    let config: PriceDbConfig =
        confy::load(APP_NAME, APP_NAME).expect("valid config should be loaded");

    config
}

#[cfg(test)]
mod tests {
    use rstest::fixture;

    use crate::{config::PriceDbConfig, App};

    #[fixture]
    fn dbg_config() -> PriceDbConfig {
        let mut cfg = PriceDbConfig::default();
        cfg.symbols_path = "tests/symbols.csv".into();
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
}
