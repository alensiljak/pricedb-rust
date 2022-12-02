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

use crate::model::{NewPrice, Price, Security, SecurityFilter};

use super::Dal;

pub struct DieselDal {
    pub(crate) conn_str: String,
}

pub fn establish_connection(db_path: &str) -> SqliteConnection {
    let x = SqliteConnection::establish(db_path).unwrap();
    return x;
}

impl Dal for DieselDal {
    fn get_prices(&self, filter: crate::model::PriceFilter) -> Vec<Price> {
        use crate::database::schema::price::dsl::*;
        let mut query = price.into_boxed();

        if let Some(security_id_val) = filter.security_id {
            query = query.filter(security_id.eq(security_id_val));
        }
        if let Some(date_val) = filter.date {
            query = query.filter(date.eq(date_val));
        }
        if let Some(time_val) = filter.time {
            query = query.filter(time.eq(time_val));
        }

        let conn = &mut establish_connection(&self.conn_str);
        let result = query.load::<Price>(conn).expect("Error loading prices");

        // log::debug!("prices: {:?}", result);

        return result;
    }

    /**
    Fetches the securities that match the given filters
    */
    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security> {
        use crate::database::schema::security::dsl::*;

        let mut query = security.into_boxed();
        if let Some(mut currency_val) = filter.currency {
            currency_val = currency_val.to_uppercase();
            query = query.filter(currency.eq(currency_val));
        }
        if let Some(agent_val) = filter.agent {
            //agent_val = agent_val.to_uppercase();
            query = query.filter(updater.eq(agent_val));
        }
        if let Some(mut mnemonic_val) = filter.symbol {
            mnemonic_val = mnemonic_val.to_uppercase();
            query = query.filter(symbol.eq(mnemonic_val));
        }
        if let Some(mut exchange_val) = filter.exchange {
            exchange_val = exchange_val.to_uppercase();
            query = query.filter(namespace.eq(exchange_val));
        }

        let conn = &mut establish_connection(&self.conn_str);
        let result = query
            .load::<Security>(conn)
            .expect("Error loading securities");

        // log::debug!("securities: {:?}", result);

        return result;
    }

    fn delete_price(&self, id_to_delete: i32) -> Result<usize, anyhow::Error> {
        use crate::database::schema::price::dsl::*;

        let conn = &mut establish_connection(&self.conn_str);

        let result = diesel::delete(price.filter(id.eq(id_to_delete))).execute(conn)?;

        Ok(result)
    }

    #[allow(unused_variables)]
    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        todo!("implement")
    }

    fn get_symbols(&self) -> Vec<crate::model::SecuritySymbol> {
        todo!()
    }

    fn get_prices_for_security(
        &self,
        security_id_param: i32,
    ) -> anyhow::Result<Vec<crate::model::Price>> {
        use crate::database::schema::price::dsl::*;

        let conn = &mut establish_connection(&self.conn_str);

        let prices = price
            .filter(security_id.eq(security_id_param))
            .order_by(date.desc())
            .then_order_by(time.desc())
            .load::<Price>(conn)?;

        Ok(prices)
    }

    fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i32>> {
        use crate::database::schema::price::dsl::*;

        let conn = &mut establish_connection(&self.conn_str);

        let ids = price.select(security_id).distinct().load(conn)?;

        Ok(ids)
    }

    /**
     * Inserts a new Price record.
     */
    fn add_price(&self, new_price: &NewPrice) {
        use crate::database::schema::price::dsl::*;

        log::debug!("inserting {:?}", new_price);

        let conn = &mut establish_connection(&self.conn_str);

        diesel::insert_into(price)
            .values(new_price)
            .execute(conn)
            .expect("yo?");
    }

    /**
     * Update an existing price record.
     */
    fn update_price(&self, existing_id: i32, price_values: &Price) -> anyhow::Result<usize> {
        use crate::database::schema::price::dsl::*;

        let conn = &mut establish_connection(&self.conn_str);

        // Update an existing price
        let update_result = diesel::update(price.filter(id.eq(existing_id)))
            .set((value.eq(price_values.value), (denom.eq(price_values.denom))))
            .execute(conn)?;

        Ok(update_result)
    }
}
