/*
 * Application
 * Exposing the main app functionality as a library, for testing purposes.
 */
use model::Price;

use crate::database::Dal;

pub mod model;

mod config;
mod database;
mod ledger_formatter;
mod quote;

use crate::{
    model::{PriceFilter, SecurityFilter, SecuritySymbol},
    quote::Quote,
};

use std::vec;

use anyhow::Error;

pub struct App {
    dal: Box<dyn Dal>,
}

impl App {
    pub fn new() -> App {
        App {
            dal: Box::new(database::init_dal()),
        }
    }

    pub fn add_price(&self, new_price: Price) {
        log::debug!("Adding price {:?}", new_price);

        // Is there a price already?

        let filter = PriceFilter {
            security_id: Some(new_price.security_id),
            date: Some(new_price.date.to_owned()),
            time: new_price.time.to_owned(),
        };
        // security_id, date, time
        let prices = self.dal.get_prices(Some(filter));

        // insert or update
        if prices.len() == 0 {
            // insert
            log::debug!("Inserting");
            self.dal.add_price(&new_price);
        } else {
            // update
            log::debug!("Updating");
            // get an existing record
            let existing = prices.first().expect("error fetching security");

            log::info!("Existing price found: {:?}", existing);

            if new_price.currency != existing.currency {
                log::error!(
                    "The currencies are different {:?} vs {:?}",
                    new_price.currency,
                    existing.currency
                );
                panic!("The currencies differ!");
            }

            // let cur_val = Decimal::from_i32(price.value).unwrap();
            // let cur_denom = Decimal::from_i32(price.denom).unwrap();
            // let new_value = cur_val / cur_denom;

            let mut updated = existing.clone();

            log::debug!("clone of the price to update: {:?}", updated);

            if existing.value != new_price.value {
                log::info!(
                    "Updating value from {:?} to {}",
                    existing.value,
                    new_price.value
                );
                updated.value = new_price.value;
            }
            if existing.denom != new_price.denom {
                log::info!("Updating denom {} to {}", existing.denom, new_price.denom);
                updated.denom = new_price.denom;
            }

            let update_result = self.dal.update_price(existing.id, &updated);
            match update_result {
                Ok(_) => {
                    // everything ok
                }
                Err(e) => {
                    log::error!("{}", e);
                    panic!("{}", e);
                }
            }
        }
    }

    pub async fn download_prices(&self, filter: SecurityFilter) {
        log::debug!("download filter: {:?}", filter);

        let securities = self.dal.get_securities(filter);

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

        if securities.len() == 0 {
            print!("No Securities found for the given parameters.");
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

    fn get_prices(&self) -> Vec<Price> {
        // let filter = PriceFilter { security_id: None, date: (), time: () }
        let prices = self.dal.get_prices(None);

        // log::debug!("fetched prices: {prices:?}");

        prices
    }

    /// Prune historical prices for the given symbol, leaving only the latest.
    /// If no symbol is given, it prunes all existing symbols.
    /// Returns the number of items removed.
    pub fn prune(&self, symbol: &Option<String>) -> u16 {
        log::trace!("Pruning symbol: {:?}", symbol);

        let mut security_ids = vec![];

        if symbol.is_some() {
            // get id
            let symb = symbol.as_ref().unwrap().as_str();
            let security = self.dal.get_security_by_symbol(symb);
            security_ids.push(security.id);
        } else {
            // load all symbols
            security_ids = self.dal
                .get_ids_of_symbols_with_prices()
                .expect("Error fetching symbol ids.");
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

        return count;
    }

    /// Export prices in ledger format
    pub fn export(&self) {
        // get prices
        let mut prices = self.get_prices();

        // sort by date
        prices.sort_by(|a, b| b.date.cmp(&a.date));

        todo!("incomplete");

        // format in ledger format
        // get export destination from configuration
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i32) -> anyhow::Result<u16, Error> {
        log::trace!("pruning prices for security id: {:?}", security_id);

        let mut count = 0;
        // get prices for the given security
        let prices = self.dal
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

            self.dal.delete_price(price.id)?;
            count += 1;
        }

        return Ok(count);
    }
}

// fn get_dal() -> impl Dal {
//     let dal = database::init_dal();
//     dal
// }

async fn download_price(symbol: SecuritySymbol, currency: &str, agent: &str) -> Option<Price> {
    // todo: there must be a symbol
    let mut dl = Quote::new();

    dl.set_source(agent);
    dl.set_currency(currency);

    let prices = dl.fetch(&symbol.namespace, vec![symbol.mnemonic]).await;

    if prices.len() == 0 {
        println!("Did not receive any prices");
        return None;
    }

    let price = prices[0].to_owned();
    Some(price)
}
