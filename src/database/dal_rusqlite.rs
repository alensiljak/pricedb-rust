/****
* DAL implemented with rusqlite
* Using SeaQuery to generate queries with variable parameters for filtering.
*
* Example for a query: https://stackoverflow.com/questions/67089430/how-do-we-use-select-query-with-an-external-where-parameter-in-rusqlite
*/
use rusqlite::{named_params, Connection, Row};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};

use crate::{model::{
    Price, PriceFilter, PriceIden, Security, SecurityFilter, SecurityIden, SecuritySymbol,
}, database::mappers_rusqlite::*};

use super::Dal;

#[allow(unused)]
pub struct RuSqliteDal {
    pub(crate) conn: Connection,
}

impl RuSqliteDal {
    pub(crate) fn new(conn_str: String) -> RuSqliteDal {
        RuSqliteDal {
            conn: open_connection(&conn_str),
        }
    }
}

impl Dal for RuSqliteDal {
    fn add_price(&self, new_price: &Price) -> usize {
        log::debug!("inserting {:?}", new_price);

        let (sql, values) = generate_insert_price(new_price);

        log::debug!("inserting price: {:?}", sql);
        log::debug!("values: {:?}", values);

        let result = self
            .conn
            .execute(sql.as_str(), &*values.as_params())
            .expect("price inserted");

        result
    }

    fn delete_price(&self, id: i32) -> anyhow::Result<usize> {
        todo!()
    }

    fn get_ids_of_symbols_with_prices(&self) -> anyhow::Result<Vec<i32>> {
        let sql = "select distinct security_id from price";
        let mut stmt = self.conn.prepare(sql).expect("Error");
        let ids = stmt
            .query_map([], |row| {
                let id = row.get::<usize, i32>(0).expect("error");
                //log::debug!("row: {:?}", id);
                return Ok(id);
            })
            .expect("Mapped rows");

        // let count = rows.count();
        // log::debug!("fetched {:?} rows", count);

        let mut result: Vec<i32> = vec![];

        for row in ids {
            let id = row.expect("Error reading row");
            // log::debug!("id: {:?}", id);
            result.push(id);
        }

        return Ok(result);
    }

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price> {
        let mut result: Vec<Price> = vec![];

        let (sql, values) = match filter {
            Some(_) => generate_price_query_with_filter(&filter.unwrap()),
            None => todo!("test this case!"), //"select * from price".to_owned()
        };

        log::debug!("get prices, sql: {:?}", sql);

        let mut statement = self.conn.prepare(&sql).unwrap();

        let sec_iter = statement
            .query_map(&*values.as_params(), |row| {
                // map
                let sec = map_row_to_price(row);
                // log::debug!("parsed: {:?}", sec);
                Ok(sec)
            })
            .expect("Filtered Securities");

        for item in sec_iter {
            match item {
                Ok(sec) => result.push(sec),
                Err(_) => todo!(),
            }
        }

        log::debug!("found prices: {:?}", result);

        result
    }

    fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>> {
        let mut result: Vec<Price> = vec![];
        let sql = "select * from price where security_id=? order by date desc, time desc;";
        let mut stmt = self.conn.prepare(sql).expect("Error");

        let rows = stmt
            .query_map([security_id], |row| {
                let price = map_price(row);
                // log::debug!("price read {:?}", price);

                return Ok(price);
            })
            .expect("Error");

        // let cursor: Vec<Result<Price, rusqlite::Error>> = rows.collect();
        // log::debug!("cursor: {:?}", cursor);

        for row in rows {
            //let record = map_price(&row);
            let record = row.expect("error extracting price");
            result.push(record);
            // log::debug!("row: {:?}", row);
        }
        return Ok(result);
    }

    /// Search for the securities with the given filter.
    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security> {
        let mut result: Vec<Security> = vec![];

        // assemble the sql statement
        // let sql = "select * from security";
        let (sql, values) = generate_select_security_with_filter(&filter);

        log::debug!("securities sql: {:?}", sql);

        let mut statement = self.conn.prepare(&sql).unwrap();

        let sec_iter = statement
            .query_map(&*values.as_params(), |row| {
                // map
                let sec = map_row_to_security(row);
                // log::debug!("parsed: {:?}", sec);
                Ok(sec)
            })
            .expect("Filtered Securities");

        for item in sec_iter {
            match item {
                Ok(sec) => result.push(sec),
                Err(_) => todo!(),
            }
        }

        // log::debug!("securities: {:?}", result);

        return result;
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        log::trace!("fetching security by symbol {:?}", symbol);

        let sql = "select * from security where symbol = :symbol";
        let mut stmt = self.conn.prepare(sql).expect("Statement");
        // let params = (1, symbol);
        let params = named_params! { ":symbol": symbol };
        let security = stmt
            .query_row(params, |r| {
                let result = map_row_to_security(r);

                log::debug!("row fetched: {:?}", result);

                return Ok(result);
            })
            .expect("Error fetching security");

        log::debug!("query result: {:?}", security);

        return security;
    }

    fn get_symbols(&self) -> Vec<SecuritySymbol> {
        todo!()
    }

    fn update_price(&self, id: i32, price: &Price) -> anyhow::Result<usize> {
        todo!()
    }
}

fn map_price(row: &Row) -> Price {
    let price = Price {
        id: row.get(0).expect("error reading field"),
        security_id: row.get(1).expect("error"),
        date: row.get(2).expect("error"),
        time: row.get(3).expect("error"),
        value: row.get(4).expect("error"),
        denom: row.get(5).expect("error"),
        currency: row.get(6).expect("error"),
    };
    price
}

/// rusqlite connection
fn open_connection(conn_str: &String) -> Connection {
    Connection::open(conn_str).expect("open sqlite connection")
}

/// Generates SELECT statement with the given parameters/filters.
fn generate_select_security_with_filter(filter: &SecurityFilter) -> (String, RusqliteValues) {
    let query = Query::select()
        // Order of columns:
        .column(SecurityIden::Id)
        .column(SecurityIden::Namespace)
        .column(SecurityIden::Symbol)
        .column(SecurityIden::Updater)
        .column(SecurityIden::Currency)
        .column(SecurityIden::LedgerSymbol)
        .column(SecurityIden::Notes)
        //
        .from(SecurityIden::Table)
        .conditions(
            filter.agent.is_some(),
            |q| {
                if let Some(agent) = filter.agent.to_owned() {
                    q.and_where(Expr::col(SecurityIden::Updater).eq(agent));
                }
            },
            |q| {},
        )
        .conditions(
            filter.currency.is_some(),
            |q| {
                if let Some(cur) = filter.currency.to_owned() {
                    let uppercase_cur = cur.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Currency).eq(uppercase_cur));
                }
            },
            |q| {},
        )
        .conditions(
            filter.exchange.is_some(),
            |q| {
                if let Some(exc) = filter.exchange.to_owned() {
                    let uppercase_exc = exc.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Namespace).eq(uppercase_exc));
                }
            },
            |q| {},
        )
        .conditions(
            filter.symbol.is_some(),
            |q| {
                if let Some(sym) = filter.symbol.to_owned() {
                    let uppercase_sym = sym.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Symbol).eq(uppercase_sym));
                }
            },
            |q| {},
        )
        .to_owned();

    // query.build(SqliteQueryBuilder)
    //query.to_string(SqliteQueryBuilder)
    query.build_rusqlite(SqliteQueryBuilder)
}

#[allow(unused_variables)]
fn generate_price_query_with_filter(filter: &PriceFilter) -> (String, RusqliteValues) {
    let query = Query::select()
        // Order of columns:
        .column(PriceIden::Id)
        .column(PriceIden::SecurityId)
        .column(PriceIden::Date)
        .column(PriceIden::Time)
        .column(PriceIden::Value)
        .column(PriceIden::Denom)
        .column(PriceIden::Currency)
        //
        .from(PriceIden::Table)
        .conditions(
            filter.security_id.is_some(),
            |q| {
                if let Some(val) = filter.security_id {
                    q.and_where(Expr::col(PriceIden::SecurityId).eq(val));
                }
            },
            |q| {},
        )
        .conditions(
            filter.date.is_some(),
            |q| {
                if let Some(val) = filter.date.to_owned() {
                    q.and_where(Expr::col(PriceIden::Date).eq(val));
                }
            },
            |q| {},
        )
        .conditions(
            filter.time.is_some(),
            |q| {
                if let Some(val) = filter.time.to_owned() {
                    q.and_where(Expr::col(PriceIden::Time).eq(val));
                }
            },
            |q| {},
        )
        .to_owned();

    // query.build(SqliteQueryBuilder)
    //query.to_string(SqliteQueryBuilder)
    query.build_rusqlite(SqliteQueryBuilder)
}

#[cfg(test)]
mod tests {
    use sea_query::{ColumnDef, Table};
    use sea_query_rusqlite::RusqliteValue;
    use test_log::test;

    use crate::model::SecurityFilter;

    use super::*;

    /// Creates a dummy dal and prepares an in-memory test database.
    fn get_test_dal() -> RuSqliteDal {
        let dal = RuSqliteDal::new(":memory:".to_string());
        prepare_test_db(&dal);
        insert_dummy_prices(&dal);
        insert_dummy_securities(&dal);

        dal
    }

    fn prepare_test_db(dal: &RuSqliteDal) {
        // drop Security table, if exists

        let sql = Table::drop()
            .table(SecurityIden::Table)
            .if_exists()
            .build(SqliteQueryBuilder);
        dal.conn.execute(&sql, []).expect("result");

        // create Prices table

        let sql = Table::create()
            .table(SecurityIden::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(SecurityIden::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(SecurityIden::Namespace).string().null())
            .col(ColumnDef::new(SecurityIden::Symbol).string())
            .col(ColumnDef::new(SecurityIden::Updater).string().null())
            .col(ColumnDef::new(SecurityIden::Currency).string().null())
            .col(ColumnDef::new(SecurityIden::LedgerSymbol).string().null())
            .col(ColumnDef::new(SecurityIden::Notes).string().null())
            .build(SqliteQueryBuilder);

        dal.conn.execute(&sql, []).expect("result");

        // drop Prices table, if exists

        let sql = Table::drop()
            .table(PriceIden::Table)
            .if_exists()
            .build(SqliteQueryBuilder);
        dal.conn.execute(&sql, []).expect("result");

        // create Prices table

        let sql = Table::create()
            .table(PriceIden::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(PriceIden::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(PriceIden::SecurityId).integer())
            .col(ColumnDef::new(PriceIden::Date).string())
            .col(ColumnDef::new(PriceIden::Time).string().null())
            .col(ColumnDef::new(PriceIden::Value).integer())
            .col(ColumnDef::new(PriceIden::Denom).integer())
            .col(ColumnDef::new(PriceIden::Currency).string())
            .build(SqliteQueryBuilder);

        let result = dal.conn.execute(&sql, []).expect("result");

        assert_eq!(0, result);
    }

    fn create_dummy_price(security_id: i32, value: i32, denom_opt: Option<i32>) -> Price {
        let date: String = chrono::Local::now().date_naive().to_string();
        Price {
            id: i32::default(),
            security_id,
            date,
            time: None,
            value,
            denom: match denom_opt {
                Some(val) => val,
                None => 100,
            },
            currency: "EUR".to_string(),
        }
    }

    fn insert_dummy_prices(dal: &RuSqliteDal) {
        dal.add_price(&create_dummy_price(1, 12345, None));
        dal.add_price(&create_dummy_price(1, 10101, None));
        dal.add_price(&create_dummy_price(2, 1234, None));
        dal.add_price(&create_dummy_price(3, 123456789, Some(10000)));
        dal.add_price(&create_dummy_price(4, 123456, Some(1000)));
    }

    fn insert_dummy_securities(dal: &RuSqliteDal) {
        let sql = "INSERT INTO Security (id, namespace, symbol, currency) VALUES (?1, ?2, ?3, ?4)";
        dal.conn
            .execute(sql, (1, "NULL", "VTI", "USD"))
            .expect("inserted record");
        dal.conn
            .execute(sql, (2, "XETRA", "EL49", "EUR"))
            .expect("inserted record");
        dal.conn
            .execute(sql, (3, "ASX", "VAS", "AUD"))
            .expect("inserted record");
        dal.conn
            .execute(sql, (4, "LSE", "VHYL", "GBP"))
            .expect("inserted record");
    }

    // #[test]
    // fn test_conditions() {
    //     let mut cond = Cond::all();
    //     cond = cond.add(Expr::col(PriceIden::SecurityId).eq(130));
    //     println!("Condition: {:?}", cond);
    //     assert!(false)
    // }

    #[test]
    fn test_sec_query_wo_params() {
        let filter = SecurityFilter {
            currency: None,
            agent: None,
            exchange: None,
            symbol: None,
        };
        let (sql, values) = generate_select_security_with_filter(&filter);

        let expected = "SELECT \"id\", \"namespace\", \"symbol\", \"updater\", \"currency\", \"ledger_symbol\", \"notes\" FROM \"security\"";
        assert_eq!(expected, sql);
    }

    #[test]
    fn test_sec_query_w_params() {
        let filter = SecurityFilter {
            currency: Some("AUD".to_owned()),
            agent: None,
            exchange: None,
            symbol: None,
        };
        let (sql, values) = generate_select_security_with_filter(&filter);

        print!("{:?}", values);

        let expected = "SELECT \"id\", \"namespace\", \"symbol\", \"updater\", \"currency\", \"ledger_symbol\", \"notes\" FROM \"security\" WHERE \"currency\" = ?";
        assert_eq!(expected, sql);
        let exp_val = RusqliteValue(sea_query::Value::String(Some(Box::new("AUD".to_owned()))));
        assert_eq!(exp_val, values.0[0]);
    }

    #[test]
    fn test_null_param() {
        let sql = r#"SELECT * 
        FROM MY_TABLE 
        WHERE @parameter IS NULL OR NAME = @parameter;"#;
    }

    #[test]
    /// Test loading prices with an empty filter.
    /// Loads all prices
    fn test_get_prices_w_empty_filter() {
        let dal = get_test_dal();

        let filter = PriceFilter {
            security_id: None,
            date: None,
            time: None,
        };
        let actual = dal.get_prices(Some(filter));

        println!("prices: {:?}", actual);

        assert!(actual.len() == 5);
    }

    #[test]
    /// Test loading prices with a security id.
    fn test_get_prices_w_filter() {
        let dal = get_test_dal();

        let filter = PriceFilter {
            security_id: Some(1),
            date: None,
            time: None,
        };
        let actual = dal.get_prices(Some(filter));

        println!("prices: {:?}", actual);

        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn test_get_securities_wo_filter() {
        let dal = get_test_dal();

        let filter = SecurityFilter::new();

        let securities = dal.get_securities(filter);

        assert_ne!(securities.len(), 0);
        assert_eq!(securities.len(), 4);
    }

    #[test]
    fn test_get_security_by_symbol() {
        let dal = get_test_dal();

        let symbol = "EL49";

        let actual = dal.get_security_by_symbol(symbol);

        assert_eq!(actual.currency, Some("EUR".to_string()));
        assert_eq!(actual.namespace, Some("XETRA".to_string()));
    }
}
