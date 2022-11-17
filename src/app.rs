/*
 * Application
 */

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use tracing::{debug};

use crate::{model::Security, dal_diesel::establish_connection};

#[derive(Debug)]
pub(crate) struct DownloadOptions {
    pub exchange: Option<String>,
    pub mnemonic: Option<String>,
    pub currency: Option<String>,
    pub agent: Option<String>,
}

pub(crate) fn download_prices(options: DownloadOptions) {
    debug!("download options: {:?}", options);
    
    // securities = self.__get_securities(currency, agent, mnemonic, exchange)
    //let securities: Vec<String> = vec![];
    let securities = get_securities(options.currency, options.agent, 
        options.mnemonic, options.exchange);

    debug!("securities: {:?}", securities);

    // if len(securities) == 0:
    // print('No Securities found for the given parameters.')

}

/**
Fetches the securities that match the given filters
*/
fn get_securities(currency_filter: Option<String>, agent_filter: Option<String>, 
    mnemonic_filter: Option<String>, exchange_filter: Option<String>) -> Vec<Security> {

    // todo: pass the filter

    use crate::schema::security::dsl::*;

    let mut query = security.into_boxed();
    if let Some(mut currency_val) = currency_filter {
        currency_val = currency_val.to_uppercase();
        query = query.filter(currency.eq(currency_val));
    }
    if let Some(agent_val) = agent_filter {
        query = query.filter(currency.eq(agent_val));
    }
    if let Some(mnemonic_val) = mnemonic_filter {
        query = query.filter(currency.eq(mnemonic_val));
    }
    if let Some(exchange_val) = exchange_filter {
        query = query.filter(currency.eq(exchange_val));
    }

    let conn = &mut establish_connection();
    let result = query
        .load::<Security>(conn)
        .expect("Error loading securities");

    // debug!("securities: {:?}", result);

    return result;
}