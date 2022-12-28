/*!
 * sqlx DAL
 */

use async_trait::async_trait;
use sqlx::{Connection, Row, SqliteConnection};

use crate::model::{Price, PriceFilter, Security, SecurityFilter, SecuritySymbol};

use super::Dal;

pub struct SqlxDal {
    // pub(crate) conn_str: String,
    conn: SqliteConnection,
}

#[async_trait]
impl Dal for SqlxDal {
    fn add_price(&self, new_price: &Price) -> usize {
        todo!("complete")
    }

    fn add_security(&self, security: &Security) -> usize {
        todo!("complete");
    }

    fn create_tables(&self) {
        let sql = crate::database::db_schema::get_drop_security();
    }

    fn delete_price(&self, id: i64) -> anyhow::Result<usize> {
        todo!("complete")
    }

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price> {
        todo!("complete")
    }

    fn get_prices_for_security(&self, security_id: i64) -> anyhow::Result<Vec<Price>> {
        async_std::task::block_on(async {
            let result = self.get_prices_for_security_async(security_id).await;
        });

        let result = vec![];
        return Ok(result);
    }

    fn get_prices_with_symbols(&self) -> Vec<Price> {
        todo!()
    }

    fn get_securities(&self, filter: Option<SecurityFilter>) -> Vec<Security> {
        todo!("implement");
    }

    fn get_securitiess_having_prices(&self) -> Vec<Security> {
        todo!()
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        let mut result = Security::default();
        async_std::task::block_on(async {
            result = self.get_security_by_symbol_async(symbol).await;
        });

        result
    }

    fn get_tables(&self) -> Vec<std::string::String> {
        todo!()
    }

    fn update_price(&self, price: &Price) -> anyhow::Result<usize> {
        todo!("complete")
    }
}

impl SqlxDal {
    fn new(conn_str: &str) -> Self {
        let conn: SqliteConnection =
            async_std::task::block_on(async { open_connection(conn_str).await });

        Self {
            //conn_str: conn_str.to_string(),
            conn,
        }
    }

    async fn get_ids_of_symbols_with_prices(&mut self) -> anyhow::Result<Vec<i64>> {
        let mut result: Vec<i64> = vec![];

        let rows = sqlx::query("select security_id from price")
            .fetch_all(&mut self.conn)
            .await
            .expect("Error fetching prices");
        //symbol_ids
        for row in rows {
            //let x = row.security_id;
            // row.try_get("security_id");
            result.push(row.get(0));
        }

        return Ok(result);
    }

    // async fn get_symbols(&self) -> Vec<SecuritySymbol> {
    //     async {
    //         let conn = self.get_connection().into();
    //         let mut symbols = query_as!(SecuritySymbol, "select namespace, mnemonic from symbols")
    //             .fetch_all(&mut conn)
    //             .await;
    //     };
    //     return vec![];
    // }

    async fn get_security_by_symbol_async(&mut self, symbol: &str) -> Security {
        let result = sqlx::query_as!(Security, r#"select * from security where symbol=?"#, symbol)
            .fetch_one(&mut self.conn)
            .await
            .expect("Error getting Security by symbol");

        return result;
    }

    async fn get_prices_for_security_async(&mut self, security_id: i64) -> Vec<Price> {
        let result = sqlx::query_as!(
            Price,
            "select * from price where security_id=? order by date desc, time desc;",
            security_id
        )
        .fetch_all(&mut self.conn)
        .await
        .expect("Error fetching prices");

        // let stream =
        //     sqlx::query("select * from price where security_id=? order by date desc, time desc;")
        //         .bind(security_id)
        //         // .map(|row: SqliteRow| {
        //         //     log::debug!("row: {:?}", row.column(0));
        //         // })
        //         .fetch(&mut conn);

        // let recs = sqlx::query!(
        //     r#"select * from price where security_id=? order by date desc, time desc;"#,
        // );
        // log::debug!("stream: {:?}", stream);

        result
    }
}

async fn open_connection(conn_str: &str) -> SqliteConnection {
    let conn = SqliteConnection::connect(conn_str)
        .await
        .expect("sqlite connection");
    conn
}

// Tests

#[cfg(test)]
mod tests {
    use super::{Dal, SqlxDal};

    const CONN_STR: &str = ":memory:";

    #[rstest::fixture]
    fn dal() -> SqlxDal {
        let dal = SqlxDal::new(CONN_STR);

        // set-up schema
        dal.create_tables();

        // populate dummy data

        dal
    }

    /// Uses actual database file.
    #[test]
    fn get_sec_by_symbol_test() {
        let conn_str = crate::load_config().price_database_path;
        let dal = SqlxDal::new(&conn_str);
        let symbol = "TCBT";
        let actual = dal.get_security_by_symbol(symbol);

        println!("symbol: {:?}", actual);

        assert_ne!(actual.id, 0);
        assert_eq!(actual.symbol, symbol);
    }

    #[rstest::rstest]
    fn get_prices_for_sec_test(dal: SqlxDal) {
        // dal.get_prices_for_security(security_id)
    }
}
