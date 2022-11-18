/*
 * Application
 */

use log::debug;

use crate::database;

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
    let securities = database::get_securities(options.currency, options.agent, 
        options.mnemonic, options.exchange);

    debug!("securities: {:?}", securities);

    // if len(securities) == 0:
    // print('No Securities found for the given parameters.')

}

