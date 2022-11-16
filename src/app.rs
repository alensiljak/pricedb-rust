/*
 * Application
 */

use tracing::debug;

use crate::{repositories::{self, SecurityRepository}, model::Security};

#[derive(Debug)]
pub(crate) struct DownloadOptions {
    pub currency: String
}

pub(crate) fn download_prices(options: DownloadOptions) {
    debug!("download options: {:?}", options);
    
    let currency = options.currency.to_uppercase();
    // todo: agent
    // todo: symbol
    // todo: exchange

    // securities = self.__get_securities(currency, agent, mnemonic, exchange)
    //let securities: Vec<String> = vec![];
    let securities = get_securities(currency, agent, mnemonic, exchange);

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
    
    let sec_repo = SecurityRepository {};
    let result = sec_repo.q;

    return result;
}