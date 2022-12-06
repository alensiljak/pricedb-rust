use model::Price;

use crate::database::Dal;

/*
 * Exposing the main app functionality as a library, for testing purposes.
 */
pub mod app;
mod config;
mod database;
pub mod model;
mod quote;

/// Export prices in ledger format
pub fn export() {
    // get prices
    let prices = get_prices();

    //prices.sort_by(compare)
    todo!("sort by date");

    todo!("format in ledger format");
    todo!("get export destination from configuration");
}

fn get_prices() -> Vec<Price> {
    let dal = database::init_dal();
    // let filter = PriceFilter { security_id: None, date: (), time: () }
    let prices = dal.get_prices(None);

    // log::debug!("fetched prices: {prices:?}");

    prices
}