/*!
 * sqlx DAL
 */

use async_trait::async_trait;
use sqlx::{sqlite::SqliteRow, Connection, Row, SqliteConnection};

use crate::model::{NewPrice, Price, PriceFilter, Security, SecurityFilter};

use super::Dal;

pub struct SqlxDal {
    pub(crate) conn_str: String,
}

#[async_trait]
impl Dal for SqlxDal {
    fn add_price(&self, new_price: &NewPrice) {
        todo!("complete")
    }

    fn delete_price(&self, id: i32) -> anyhow::Result<usize> {
        todo!("complete")
    }

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price> {
        todo!("complete")
    }

    async fn get_securities(&self, filter: SecurityFilter) -> Vec<Security> {
        todo!("implement");
    }

    async fn get_security_by_symbol(&self, symbol: &String) -> Security {
        let mut conn = self.get_connection().await.expect("Error opening database");

        let result = sqlx::query_as!(Security, "select * from security where symbol=?", symbol)
            .fetch_one(&mut conn)
            .await
            .expect("Error getting Security by symbol");

        return result;
    }

    async fn get_symbols(&self) -> Vec<crate::model::SecuritySymbol> {
        // async {
        // let conn = self.get_connection().into();
        // let mut symbols = query_as!(SecuritySymbol,
        //     "select namespace, mnemonic from symbols")
        // .fetch_all(&mut conn).await;
        // };

        return vec![];
    }

    async fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>> {
        let mut conn = self.get_connection().await;

        // let result = sqlx::query_as!(
        //     Price,
        //     "select * from price where security_id=? order by date desc, time desc;",
        //     security_id
        // )
        // .fetch_all(&mut conn)
        // .await
        // .expect("Error fetching prices");

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

        let result = vec![];
        return Ok(result);
    }

    async fn get_ids_of_symbols_with_prices(&self) -> anyhow::Result<Vec<i32>> {
        let mut result: Vec<i64> = vec![];

        let mut conn = self.get_connection().await?;

        let rows = sqlx::query("select security_id from price")
            .fetch_all(&mut conn)
            .await
            .expect("Error fetching prices");
        //symbol_ids
        for row in rows {
            //let x = row.security_id;
            result.push(row.get(0));
        }

        return Ok(result);
    }

    fn update_price(&self, id: i32, price: &Price) -> anyhow::Result<usize> {
        todo!("complete")
    }
}

impl SqlxDal {
    async fn get_connection(&self) -> SqliteConnection {
        // anyhow::Result<>
        let conn = SqliteConnection::connect(&self.conn_str)
            .await
            .expect("sqlite connection");
        conn
    }
}
