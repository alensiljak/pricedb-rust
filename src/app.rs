/*
 * Application
 */

use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use tracing::{debug, error};

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
fn get_securities(currency: Option<String>, agent: Option<String>, 
    mnemonic: Option<String>, exchange: Option<String>) -> Vec<Security> {
    // todo: pass the filter

    use crate::schema::security::dsl::*;

    let conn = &mut establish_connection();
    let data = security.load::<Security>(conn);

    let result = security
        //.select(selection)
        .filter(symbol.eq("BRD"))
        .load::<Security>(conn)
        .expect("Error loading securities");

    todo!("load securities");

    // match result {
    //     Ok(value) => return value,
    //     Err(e) => {
    //         error!("Error fetching securities: {:?}", e);
    //         return vec![];
    //     }
    // }
}