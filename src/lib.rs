/*!
 * Price Database API
 *
 * Price Database downloads prices for given securities, stores it in an sqlite database,
 * and exports in Ledger-cli format.
 *
 * Project [Documentation](https://github.com/alensiljak/pricedb-rust).
 */

use config::PriceDbConfig;
use once_cell::unsync::OnceCell;
use rust_decimal::prelude::ToPrimitive;

pub mod config;
mod database;
mod ledger_formatter;
pub mod model;
mod quote;

use crate::{database::Dal, model::*, quote::Quote};

use std::{fs, vec};

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
            security_id: Some(new_price.security_id),
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

    pub async fn download_prices(&self, filter: SecurityFilter) {
        log::debug!("download filter: {:?}", filter);

        let dal = self.get_dal();
        let securities = dal.get_securities(Some(filter));

        // Debug
        {
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

        if securities.is_empty() {
            println!("No Securities found for the given parameters.");
            return;
        }

        let mut counter_updated = 0;
        let sec_count = securities.len().try_into().unwrap();
        let pb = indicatif::ProgressBar::new(sec_count);
        pb.set_style(indicatif::ProgressStyle::default_bar().progress_chars("=>-"));

        for sec in securities {
            let symbol = SecuritySymbol {
                namespace: sec.namespace.unwrap().to_owned(),
                mnemonic: sec.symbol.to_owned(),
            };

            let mut price = download_price(
                symbol,
                sec.currency.unwrap().as_str(),
                sec.updater.unwrap().as_str(),
            )
            .await
            .expect("Error fetching price");

            price.security_id = sec.id;

            log::debug!("the fetched price for {:?} is {:?}", sec.symbol, price);

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
        let mut pwss = self.load_all_prices_with_symbols();

        // sort
        pwss.sort_unstable_by_key(|p| (p.namespace.to_owned(), p.symbol.to_owned()));

        let mut result = String::new();
        for pws in pwss {
            let output = format!(
                "{}:{} {} {} {:?} {}",
                pws.namespace, pws.symbol, pws.date, pws.time, pws.value, pws.currency
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
            let security = dal.get_security_by_symbol(symb);
            security_ids.push(security.id);
        } else {
            // load all symbols
            security_ids = dal
                .get_securitiess_having_prices()
                .iter()
                .map(|item| item.id)
                .collect();
        }

        let mut count = 0;
        let mut count_deleted = 0;
        // Send the symbols to the individual prune.
        for security_id in security_ids {
            if let Result::Ok(i) = self.prune_for_sec(security_id) {
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
        let mut pwss = self.load_all_prices_with_symbols();

        pwss.sort_unstable_by_key(|p| {
            (
                p.date.to_owned(),
                p.time.to_owned(),
                p.namespace.to_owned(),
                p.symbol.to_owned(),
            )
        });

        // format in ledger format
        let output = ledger_formatter::format_prices_w_symbols(pwss);

        // get export destination from configuration
        let target = &self.config.export_destination;

        println!("Prices exported to {target}");

        save_text_file(&output, target);
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

    fn insert_price(&self, new_price: &Price) -> usize {
        // println!("\nInserting {new_price:?}");

        let dal = self.get_dal();

        dal.add_price(new_price)
    }

    fn load_all_prices_with_symbols(&self) -> Vec<PriceWSymbol> {
        let dal = self.get_dal();
        let prices = dal.get_prices(None);
        let securities = dal.get_securities(None);

        prices
            .iter()
            .map(|price| {
                let sec = securities
                    .iter()
                    .find(|sec| sec.id == price.security_id)
                    .expect("a related security");

                PriceWSymbol::from(price, sec)
            })
            .collect()
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i64) -> anyhow::Result<u16, Error> {
        log::trace!("pruning prices for security id: {:?}", security_id);

        let mut count = 0;
        // get prices for the given security
        let dal = self.get_dal();
        let prices = dal
            .get_prices_for_security(security_id)
            .expect("Error fetching prices for security");

        // log::debug!("prices for {:?} - {:?}", security_id, prices);
        //log::debug!("received {} prices", prices.len());

        let size = prices.len();
        if size <= 1 {
            // nothing to delete
            log::debug!("Nothing to prune for {:?}", security_id);
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

async fn download_price(symbol: SecuritySymbol, currency: &str, agent: &str) -> Option<Price> {
    // todo: there must be a symbol
    let mut dl = Quote::new();

    dl.set_source(agent);
    dl.set_currency(currency);

    let prices = dl.fetch(&symbol.namespace, vec![symbol.mnemonic]).await;

    if prices.is_empty() {
        println!("Did not receive any prices");
        return None;
    }

    let price = prices[0].to_owned();
    Some(price)
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
