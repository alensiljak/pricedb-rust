/*
 * Application
 */

use log::debug;

use crate::database::{self, Dal};

#[derive(Debug)]
pub(crate) struct DownloadOptions {
    pub exchange: Option<String>,
    pub mnemonic: Option<String>,
    pub currency: Option<String>,
    pub agent: Option<String>,
}

pub struct App {}

impl App {
    pub(crate) fn download_prices(
        &self,
        exchange: &Option<String>,
        mnemonic: &Option<String>,
        currency: &Option<String>,
        agent: &Option<String>,
    ) {
        debug!("download options: {:?} {:?} {:?} {:?}", 
        exchange, mnemonic, currency, agent);

        // securities = self.__get_securities(currency, agent, mnemonic, exchange)
        //let securities: Vec<String> = vec![];

        let dal = database::init_db();
        let securities =
            dal.get_securities(currency.clone(), agent.clone(), mnemonic.clone(), exchange.clone());

        debug!("securities: {:?}", securities);

        // if len(securities) == 0:
        // print('No Securities found for the given parameters.')
    }
}
