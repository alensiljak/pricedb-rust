use model::Price;

use crate::database::Dal;

/*
 * Exposing the main app functionality as a library, for testing purposes.
 */
pub mod app;
pub mod model;

mod config;
mod database;
mod ledger_formatter;
mod quote;

/// Export prices in ledger format
pub fn export() {
    // get prices
    let mut prices = get_prices();

    // sort by date
    prices.sort_by(|a, b| b.date.cmp(&a.date));

    todo!("incomplete");

    // format in ledger format
    // get export destination from configuration
}

fn get_prices() -> Vec<Price> {
    let dal = database::init_dal();
    // let filter = PriceFilter { security_id: None, date: (), time: () }
    let prices = dal.get_prices(None);

    // log::debug!("fetched prices: {prices:?}");

    prices
}