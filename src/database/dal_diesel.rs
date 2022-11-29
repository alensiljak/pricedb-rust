/*
 * DAL with diesel
 * diesel.rs
 * See:
 *   - [Getting Started](https://diesel.rs/guides/getting-started) guide.
 *   - [Sqlite Examples](https://github.com/diesel-rs/diesel/tree/2.0.x/examples/sqlite)
 *
 * Install with:
 * `cargo install diesel_cli --no-default-features --features sqlite`
 *
 * Run:
 * `diesel --database-url=sqlite://path-to/prices.db`
 *
 * Examples
 * - https://stackoverflow.com/questions/65039754/rust-diesel-conditionally-filter-a-query
 */

use diesel::prelude::*;
use diesel::{sqlite::SqliteConnection, Connection};
use diesel::{QueryDsl, RunQueryDsl};

use crate::model::Security;

use super::Dal;

pub struct DieselDal {
    pub(crate) conn_str: String,

}

pub fn establish_connection(db_path: String) -> SqliteConnection {
    let x = SqliteConnection::establish(db_path.as_str()).unwrap();
    return x;
}

impl Dal for DieselDal {
    /**
    Fetches the securities that match the given filters
    */
    fn get_securities(
        &self,
        currency: &Option<String>,
        agent: &Option<String>,
        mnemonic: &Option<String>,
        exchange: &Option<String>,
    ) -> Vec<Security> {
        // todo: pass the filter

        // use crate::database::schema::security::dsl::*;
        use crate::database::schema::security;

        let mut query = security::table.into_boxed::<>();
        if let Some(mut currency_val) = currency {
            currency_val = currency_val.to_uppercase();
            query = query.filter(security::dsl::currency.eq(currency_val));
        }
        if let Some(agent_val) = agent {
            query = query.filter(security::dsl::updater.eq(agent_val));
        }
        if let Some(mnemonic_val) = mnemonic {
            query = query.filter(security::dsl::symbol.eq(mnemonic_val));
        }
        if let Some(exchange_val) = exchange {
            query = query.filter(security::dsl::namespace.eq(exchange_val));
        }

        let conn = &mut establish_connection(self.conn_str);
        let result = query
            .load::<Security>(conn)
            .expect("Error loading securities");

        // debug!("securities: {:?}", result);

        return result;
    }

    fn delete_price(&self, id: i64) -> anyhow::Result<()> {
        todo!()
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        todo!()
    }

    fn get_symbols(&self) -> Vec<crate::model::SecuritySymbol> {
        todo!()
    }

    fn get_prices_for_security(&self, security_id: i64) -> anyhow::Result<Vec<crate::model::Price>> {
        todo!()
    }

    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i64>> {
        todo!()
    }
}
