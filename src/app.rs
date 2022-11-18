/*
 * Application
 */

use log::debug;

use crate::database::{self, Dal};

pub struct App {
    dal: Box<dyn Dal>,
}

impl App {
    pub fn new() -> App {
        let dal = database::init_db();
        let result = App { dal: Box::new(dal) };
        return result;
    }

    pub(crate) fn download_prices(
        &self,
        exchange: &Option<String>,
        mnemonic: &Option<String>,
        currency: &Option<String>,
        agent: &Option<String>,
    ) {
        debug!(
            "download options: {:?} {:?} {:?} {:?}",
            exchange, mnemonic, currency, agent
        );

        // securities = self.__get_securities(currency, agent, mnemonic, exchange)
        //let securities: Vec<String> = vec![];

        let securities = self.dal.get_securities(
            currency.clone(),
            agent.clone(),
            mnemonic.clone(),
            exchange.clone(),
        );

        debug!("securities: {:?}", securities);

        // if len(securities) == 0:
        // print('No Securities found for the given parameters.')
    }

    pub fn prune(&self, symbol: &Option<String>) {
        log::trace!("In prune. Incomplete. symbol: {:?}", symbol);
    }
}
