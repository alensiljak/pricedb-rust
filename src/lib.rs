use config::PriceDbConfig;
/*
 * Application
 * Exposing the main app functionality as a library, for testing purposes.
 */
mod config;
mod database;
mod ledger_formatter;
pub mod model;
mod quote;

use crate::{
    database::Dal,
    model::{Price, PriceFilter, SecurityFilter, SecuritySymbol},
    quote::Quote,
};

use std::{fs, vec};

use anyhow::Error;

pub struct App {
    dal: Box<dyn Dal>,
}

const APP_NAME: &str = "pricedb";

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

            let mut should_update = false;
            //let mut for_update = existing.clone();
            let mut for_update = Price::new();

            for_update.id = existing.id;

            // log::debug!("clone of the price to update: {:?}", for_update);

            if existing.value != new_price.value {
                log::info!(
                    "Updating value from {:?} to {}",
                    existing.value,
                    new_price.value
                );
                for_update.value = new_price.value;
                should_update = true;
            }
            if existing.denom != new_price.denom {
                log::info!("Updating denom {} to {}", existing.denom, new_price.denom);
                for_update.denom = new_price.denom;
                should_update = true;
            }

            // Exit if there's nothing to update.
            if !should_update {
                log::debug!("Nothing to update");
                return;
            };

            log::debug!("updating record {new_price:?}");

            let update_result = self.dal.update_price(&for_update);
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

    pub fn config_show(&self) {
        let path =
            confy::get_configuration_file_path(APP_NAME, None).expect("configuration path");
        let cfg = load_config().expect("configuration");

        println!("Configuration file: {path:?}");
        println!("{cfg:?}");
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
        let prices = self.dal.get_prices(None);

        // log::debug!("fetched prices: {prices:?}");

        // todo: sort by namespace/symbol?
        // prices.sort_by(compare);
        //prices.sort_by(|a, b| b.date.cmp(&a.date));

        prices
    }

    pub fn list_prices(
        &self,
        _date: &Option<String>,
        _currency: &Option<String>,
        _last: &Option<String>,
    ) {
        // load and show all prices
        let prices = self.dal.get_prices(None);
        for price in prices {
            println!("{price:?}");
        }
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
            security_ids = self
                .dal
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

        return count;
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
        let securities = self.dal.get_securitiess_having_prices();
        // log::debug!("{securities:?}");
        // let mut sec_map: HashMap<i32, Security> = HashMap::new();
        // for sec in securities {
        //     sec_map.insert(sec.id, sec);
        // }

        // format in ledger format
        let output = ledger_formatter::format_prices(prices, &securities);

        // log::debug!("output: {output:?}");

        // get export destination from configuration
        let cfg = load_config().expect("configuration");
        let target = cfg.export_destination;

        log::debug!("saving to {target:?}");

        save_text_file(output, target);
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i32) -> anyhow::Result<u16, Error> {
        log::trace!("pruning prices for security id: {:?}", security_id);

        let mut count = 0;
        // get prices for the given security
        let prices = self
            .dal
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

fn load_config() -> Result<PriceDbConfig, anyhow::Error> {
    let config: PriceDbConfig = confy::load(APP_NAME, None)?;

    Ok(config)
}

fn save_text_file(contents: String, location: String) {
    fs::write(location, contents).expect("file saved");
}
