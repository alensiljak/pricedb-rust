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

    /// Prune historical prices for the given symbol, leaving only the latest.
    /// If no symbol is given, it prunes all existing symbols.
    /// Returns the number of items removed.
    pub async fn prune(&self, symbol: &Option<String>) -> u16 {
        log::trace!("In prune. Incomplete. symbol: {:?}", symbol);
        let mut symbols = vec![];

        if symbol.is_some() {
            symbols.push(symbol);
        } else {
            // load all symbols
            let symbol_ids = self.dal.get_symbol_ids_with_prices().await;
            debug!("symbol ids with prices: {:?}", symbol_ids);
        }

        let mut count = 0;
        // Send the symbols to the individual prune.
        for symbol in symbols {
            todo!("prune each symbol");

            // count += 1;
        }

        return count;
    }

    /// Deletes price history for the given Security, leaving only the latest price.
    fn prune_for_sec(&self, security_id: i32) -> u16 {
        debug!("pruning prices for {:?}", security_id);

        let count = 0;
        // todo: get prices for the given security
        // todo: skip the first
        // todo: delete

        return count;
    }
}
