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

use crate::{database, model::Security};

use super::Dal;

struct Diesel_dal {}

pub fn establish_connection() -> SqliteConnection {
    let db_path = database::load_db_path();

    let x = SqliteConnection::establish(db_path.as_str()).unwrap();
    return x;
}

impl Dal for Diesel_dal {
    /**
    Fetches the securities that match the given filters
    */
    fn get_securities(
        currency: Option<String>,
        agent: Option<String>,
        mnemonic: Option<String>,
        exchange: Option<String>,
    ) -> Vec<Security> {
        // todo: pass the filter

        use crate::database::schema::security::dsl::*;

        let mut query = security.into_boxed();
        if let Some(mut currency_val) = currency {
            currency_val = currency_val.to_uppercase();
            query = query.filter(currency.eq(currency_val));
        }
        if let Some(agent_val) = agent {
            query = query.filter(currency.eq(agent_val));
        }
        if let Some(mnemonic_val) = mnemonic {
            query = query.filter(currency.eq(mnemonic_val));
        }
        if let Some(exchange_val) = exchange {
            query = query.filter(currency.eq(exchange_val));
        }

        let conn = &mut establish_connection();
        let result = query
            .load::<Security>(conn)
            .expect("Error loading securities");

        // debug!("securities: {:?}", result);

        return result;
    }
}
