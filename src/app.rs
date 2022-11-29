/*
 * Application
 */

use std::vec;

use anyhow::Error;

use crate::{
    database::{self, Dal},
    model::SecurityFilter,
};

pub struct App {
    dal: Box<dyn Dal>,
}

impl App {
    pub fn new() -> App {
        let dal = database::init_db();
        let result = App { dal: Box::new(dal) };
        return result;
    }

    pub(crate) fn download_prices(&self, filter: SecurityFilter) {
        log::debug!(
            "download options: {:?} {:?} {:?} {:?}",
            filter.exchange,
            filter.symbol,
            filter.currency,
            filter.agent
        );

        // securities = self.__get_securities(currency, agent, mnemonic, exchange)
        //let securities: Vec<String> = vec![];

        let securities = self.dal.get_securities(filter);

        log::debug!("securities: {:?}", securities);

        // if len(securities) == 0:
        // print('No Securities found for the given parameters.')
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
                .get_symbol_ids_with_prices()
                .expect("Error fetching symbol ids.");
            // log::debug!("symbol ids with prices: {:?}", security_ids);
        }

        let mut count = 0;
        // Send the symbols to the individual prune.
        for security_id in security_ids {
            if let Result::Ok(i) = self.prune_for_sec(security_id) {
                // success. Log only if something was deleted.
                if i > 0 {
                    log::debug!("deleted {:?} records for {:?}", i, security_id);
                }
            } else {
                // error?
                log::warn!("Error pruning for {:?}", security_id);
            }

            count += 1;
        }

        return count;
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i64) -> anyhow::Result<u16, Error> {
        // log::trace!("pruning prices for security id: {:?}", security_id);

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
