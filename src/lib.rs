//! Price Database
//! API / business logic

use config::PriceDbConfig;
use once_cell::unsync::OnceCell;

/*
 * Application
 * Exposing the main app functionality as a library, for testing purposes.
 */
pub mod config;
mod database;
mod ledger_formatter;
pub mod model;
mod quote;

use crate::{database::Dal, model::*, quote::Quote};

use std::{fs, vec};

use anyhow::Error;

pub const APP_NAME: &str = "pricedb";

pub struct App {
    config: PriceDbConfig,
    dal: OnceCell<Box<dyn Dal>>,
}

impl App {
    pub fn new(config: PriceDbConfig) -> Self {
        App {
            config,
            dal: OnceCell::new(),
        }
    }

    pub fn add_price(&self, new_price: Price) -> usize {
        log::debug!("Adding price {:?}", new_price);

        let dal = self.get_dal();

        // Is there already a price with the same id, date, and time?

        let filter = PriceFilter {
            security_id: Some(new_price.security_id),
            date: Some(new_price.date.to_owned()),
            time: new_price.time.to_owned(),
        };
        let existing_prices = dal.get_prices(Some(filter));

        // insert or update
        let result = if existing_prices.is_empty() {
            // insert
            self.insert_price(&new_price)
        } else {
            // update
            self.update_price(existing_prices, &new_price)
        };
        result
    }

    pub fn config_show(&self) {
        let path = confy::get_configuration_file_path(APP_NAME, None).expect("configuration path");
        // let cfg = load_config().expect("configuration");
        let cfg = &self.config;

        println!("Configuration file: {path:?}");
        println!("{cfg:?}");
    }

    pub async fn download_prices(&self, filter: SecurityFilter) {
        log::debug!("download filter: {:?}", filter);

        let dal = self.get_dal();
        let securities = dal.get_securities(filter);

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

        let mut counter = 0;
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

            self.add_price(price);

            counter += 1;
        }

        log::debug!("Downloaded {} prices", counter);
    }

    /// Load and display all prices.
    /// Also returns the list as a string, for testing.
    pub fn list_prices(
        &self,
        _date: &Option<String>,
        _currency: &Option<String>,
        _last: &Option<String>,
    ) -> String {
        let dal = self.get_dal();
        let prices = dal.get_prices(None);

        let mut result = String::new();

        for price in prices {
            let output = format!("{price:?}");
            println!("{output}");
            result += &output;
        }

        result
    }

    /// Prune historical prices for the given symbol, leaving only the latest.
    /// If no symbol is given, it prunes all existing symbols.
    /// Returns the number of items removed.
    pub fn prune(&self, symbol: &Option<String>) -> u16 {
        log::trace!("Pruning symbol: {:?}", symbol);

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
            // log::debug!("symbol ids with prices: {:?}", security_ids);
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
        // get prices
        let mut prices = self.get_prices();

        // sort by date
        //prices.sort_by(|a, b| b.date.cmp(&a.date) && b.time.cmp(&a.time));
        prices.sort_unstable_by_key(|price| {
            (
                price.date.to_owned(),
                match price.time {
                    Some(_) => price.time.to_owned().unwrap(),
                    None => "".to_owned(),
                },
            )
        });
        // log::debug!("sorted: {prices:?}");

        // get all symbols with prices
        let dal = self.get_dal();
        let securities = dal.get_securitiess_having_prices();
        // log::debug!("{securities:?}");
        // let mut sec_map: HashMap<i32, Security> = HashMap::new();
        // for sec in securities {
        //     sec_map.insert(sec.id, sec);
        // }

        // format in ledger format
        let output = ledger_formatter::format_prices(prices, &securities);

        // log::debug!("output: {output:?}");

        // get export destination from configuration
        let target = &self.config.export_destination;

        println!("Prices exported to {target:?}");

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
        println!("Inserting {new_price:?}");

        let dal = self.get_dal();

        dal.add_price(&new_price)
    }

    fn get_prices(&self) -> Vec<Price> {
        let dal = self.get_dal();
        let prices = dal.get_prices(None);

        // todo: sort by namespace/symbol?
        // prices.sort_by(compare);
        //prices.sort_by(|a, b| b.date.cmp(&a.date));

        prices
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i32) -> anyhow::Result<u16, Error> {
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
            println!(
                "Updating value from {:?} to {}",
                existing.value, new_price.value
            );
            for_update.value = new_price.value;
            should_update = true;
        }
        if existing.denom != new_price.denom {
            println!("Updating denom {} to {}", existing.denom, new_price.denom);
            for_update.denom = new_price.denom;
            should_update = true;
        }

        // Exit if there's nothing to update.
        if !should_update {
            log::debug!("Nothing to update");
            return 0;
        };

        //log::debug!("updating record {new_price:?}");
        println!("for {new_price:?}");

        let update_result = dal.update_price(&for_update).unwrap();
        update_result
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
