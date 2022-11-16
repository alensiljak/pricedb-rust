/*
 * Application
 */

use tracing::debug;

use crate::repositories::{self, SecurityRepository};

#[derive(Debug)]
pub(crate) struct DownloadOptions {
    pub currency: String
}

pub(crate) fn download_prices(options: DownloadOptions) {
    debug!("download options: {:?}", options);
    
    let currency = options.currency.to_uppercase().as_str();
    // todo: agent
    // todo: symbol
    // todo: exchange

    // securities = self.__get_securities(currency, agent, mnemonic, exchange)
    //let securities: Vec<String> = vec![];
    let securities = get_securities();

    debug!("securities: {:?}", securities);

    // if len(securities) == 0:
    // print('No Securities found for the given parameters.')

}

/**
Fetches the securities that match the given filters
*/
fn get_securities() -> Vec<String> {
    // todo: pass the filter
    
    let sec_repo = SecurityRepository {};
    let result = sec_repo.all();

    return result;
}