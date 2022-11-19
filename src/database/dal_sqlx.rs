/*
 * sqlx DAL
 */

use async_trait::async_trait;
use sqlx::{Connection, SqliteConnection};

use crate::model::{Price, Security};

use super::Dal;

pub struct SqlxDal {
    pub(crate) conn_str: String,
}

#[async_trait]
impl Dal for SqlxDal {
    async fn get_securities(
        &self,
        currency: Option<String>,
        agent: Option<String>,
        mnemonic: Option<String>,
        exchange: Option<String>,
    ) -> Vec<crate::model::Security> {
        todo!("implement");
    }

    async fn get_security_by_symbol(&self, symbol: &String) -> Security {
        let mut conn = self.get_connection().await.expect("Error opening database");

        let result = sqlx::query_as!(Security, 
            "select * from security where symbol=?", symbol)
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

    async fn get_prices_for_security(&self, security_id: &i64) -> anyhow::Result<Vec<Price>> {
        let mut conn = self.get_connection().await.expect("Error opening database");

        let result = sqlx::query_as!(
            Price,
            "select * from price where security_id=?",
            security_id
        )
        .fetch_all(&mut conn)
        .await
        .expect("Error fetching prices");

        return Ok(result);
    }

    async fn get_symbol_ids_with_prices(&self) -> anyhow::Result<Vec<i64>> {
        let mut result: Vec<i64> = vec![];

        let mut conn = self.get_connection().await.expect("Error opening database");

        let rows = sqlx::query!(r#"select security_id from price"#)
            .fetch_all(&mut conn)
            .await
            .expect("Error fetching prices");
        //symbol_ids
        for row in rows {
            //let x = row.security_id;
            result.push(row.security_id);
        }

        return Ok(result);
    }
}

impl SqlxDal {
    async fn get_connection(&self) -> anyhow::Result<SqliteConnection> {
        let conn = SqliteConnection::connect(&self.conn_str).await?;
        return Ok(conn);
    }
}
