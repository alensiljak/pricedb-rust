/*
 * sqlx DAL
 */

use async_trait::async_trait;
use sqlx::{query_as, Connection, SqliteConnection};

use crate::model::SecuritySymbol;

use super::Dal;

pub struct SqlxDal {
    pub(crate) conn_str: String,
}

#[async_trait]
impl Dal for SqlxDal {
    fn get_securities(
        &self,
        currency: Option<String>,
        agent: Option<String>,
        mnemonic: Option<String>,
        exchange: Option<String>,
    ) -> Vec<crate::model::Security> {
        todo!("implement");
    }

    fn get_symbols(&self) -> Vec<crate::model::SecuritySymbol> {
        // async {
        // let conn = self.get_connection().into();
        // let mut symbols = query_as!(SecuritySymbol,
        //     "select namespace, mnemonic from symbols")
        // .fetch_all(&mut conn).await;
        // };

        return vec![];
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
