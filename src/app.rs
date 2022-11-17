/*
 * Application
 */

use tracing::{debug, error};

use crate::{model::Security};

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
    
    //let sec_repo = SecurityRepository {};
    // let result = sec_repo.query(currency, agent, 
    //     mnemonic, exchange);

    // let db = Database::new();
    // let data = Security::select_all(&mut db.rb).await.unwrap();
    todo!("load securities");

    // match result {
    //     Ok(value) => return value,
    //     Err(e) => {
    //         error!("Error fetching securities: {:?}", e);
    //         return vec![];
    //     }
    // }
}