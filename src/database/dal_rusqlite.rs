/****
* DAL implemented with rusqlite
* Using SeaQuery to generate queries with variable parameters for filtering.
*
* Example for a query: https://stackoverflow.com/questions/67089430/how-do-we-use-select-query-with-an-external-where-parameter-in-rusqlite
*/
use rusqlite::{named_params, Connection};
use sea_query::{ColumnDef, Expr, Query, SqliteQueryBuilder, Table};
use sea_query_rusqlite::{RusqliteBinder, RusqliteValues};

use crate::{database::mappers_rusqlite::*, model::*};

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

    #[allow(unused_variables)]
    fn delete_price(&self, id: i32) -> anyhow::Result<usize> {
        let (sql, values) = generate_delete_price(id);
        let result = self.conn.execute(&sql, &*values.as_params())?;
        Ok(result)
    }

    fn get_securitiess_having_prices(&self) -> Vec<Security> {
        let (sql, values) = generate_select_securities_having_prices();

        let mut stmt = self.conn.prepare(&sql).expect("Statement");
        let result = stmt
            .query_map(&*values.as_params(), |row| {
                let sec = map_row_to_security(row);
                Ok(sec)
            })
            .expect("securities mapped")
            .flatten()
            .collect();

        result
    }

    fn get_prices(&self, filter: Option<PriceFilter>) -> Vec<Price> {
        let filter_internal: PriceFilter = match filter {
            Some(_) => filter.unwrap(),
            None => PriceFilter::new(), // empty filter required
        };

        let (sql, values) = generate_select_price_with_filter(&filter_internal);

        log::debug!("get prices, sql: {:?}", sql);

        let mut statement = self.conn.prepare(&sql).unwrap();

        let prices = statement
            .query_map(&*values.as_params(), |row| {
                // map
                let sec = map_row_to_price(row);
                Ok(sec)
            })
            .expect("Filtered Securities")
            .flatten()
            .collect();

        prices
    }

    fn get_prices_for_security(&self, security_id: i32) -> anyhow::Result<Vec<Price>> {
        let sql = "select * from price where security_id=? order by date desc, time desc;";
        let mut stmt = self.conn.prepare(sql).expect("Error");

        let prices = stmt
            .query_map([security_id], |row| {
                let price = map_row_to_price(row);
                Ok(price)
            })
            .expect("mapped rows")
            .flatten()
            .collect();

        Ok(prices)
    }

    fn get_prices_with_symbols(&self) -> Vec<Price> {
        let sql = generate_select_prices_symbols(PriceSymbolOrder::Symbol);

        let mut statement = self.conn.prepare(&sql).unwrap();

        let prices = statement
            .query_map([], |row| {
                let sec = map_row_to_price(row);
                Ok(sec)
            })
            .expect("Filtered Securities")
            .flatten()
            .collect();

        prices
    }

    /// Search for the securities with the given filter.
    fn get_securities(&self, filter: SecurityFilter) -> Vec<Security> {
        // assemble the sql statement
        // let sql = "select * from security";
        let (sql, values) = generate_select_security_with_filter(&filter);

        log::debug!("securities sql: {:?}", sql);

        let mut statement = self.conn.prepare(&sql).unwrap();

        let result = statement
            .query_map(&*values.as_params(), |row| {
                let sec = map_row_to_security(row);
                Ok(sec)
            })
            .expect("Filtered Securities")
            .flatten()
            .collect();

        // log::debug!("securities: {:?}", result);

        result
    }

    fn get_security_by_symbol(&self, symbol: &str) -> Security {
        log::trace!("fetching security by symbol {:?}", symbol);

        let sql = "select * from security where symbol = :symbol";
        let mut stmt = self.conn.prepare(sql).expect("Statement");
        // let params = (1, symbol);
        let params = named_params! { ":symbol": symbol.to_uppercase() };
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

    fn update_price(&self, price: &Price) -> anyhow::Result<usize> {
        let (sql, params) = generate_update_price(price);

        log::debug!("updating price record: {sql:?}, {params:?}");

        let result = self.conn.execute(&sql, &*params.as_params())?;

        Ok(result)
    }

    //// Schema

    fn create_tables(&self) {
        // drop Security table, if exists

        let sql = Table::drop()
            .table(SecurityIden::Table)
            .if_exists()
            .build(SqliteQueryBuilder);
        self.conn.execute(&sql, []).expect("result");

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

        self.conn.execute(&sql, []).expect("result");

        // drop Prices table, if exists

        let sql = Table::drop()
            .table(PriceIden::Table)
            .if_exists()
            .build(SqliteQueryBuilder);
        self.conn.execute(&sql, []).expect("result");

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

        let result = self.conn.execute(&sql, []).expect("result");

        assert_eq!(0, result);
    }
}

/// rusqlite connection
fn open_connection(conn_str: &String) -> Connection {
    Connection::open(conn_str).expect("open sqlite connection")
}

fn generate_select_price_with_filter(filter: &PriceFilter) -> (String, RusqliteValues) {
    Query::select()
        .columns(get_price_columns())
        .from(PriceIden::Table)
        .conditions(
            filter.security_id.is_some(),
            |q| {
                if let Some(val) = filter.security_id {
                    q.and_where(Expr::col(PriceIden::SecurityId).eq(val));
                }
            },
            |_q| {},
        )
        .conditions(
            filter.date.is_some(),
            |q| {
                if let Some(val) = filter.date.to_owned() {
                    q.and_where(Expr::col(PriceIden::Date).eq(val));
                }
            },
            |_q| {},
        )
        .conditions(
            filter.time.is_some(),
            |q| {
                if let Some(val) = filter.time.to_owned() {
                    q.and_where(Expr::col(PriceIden::Time).eq(val));
                }
            },
            |_q| {},
        )
        // .to_owned();
        .build_rusqlite(SqliteQueryBuilder)

    // query.build(SqliteQueryBuilder)
    //query.to_string(SqliteQueryBuilder)
}

fn generate_select_prices_symbols(order: PriceSymbolOrder) -> String {
    let sql = r#"SELECT price.id, security_id, date, time, value, denom, price.currency,
	    namespace, symbol
    FROM price INNER JOIN security
	    ON price.security_id = security.id"#;

    let order_str = match order {
        PriceSymbolOrder::Symbol => "ORDER BY namespace, symbol",
        PriceSymbolOrder::DateTime => "ORDER BY date, time",
    };

    let result = sql.to_owned() + order_str;

    result
}

fn get_price_columns() -> Vec<(PriceIden, PriceIden)> {
    vec![
        (PriceIden::Table, PriceIden::Id),
        (PriceIden::Table, PriceIden::SecurityId),
        (PriceIden::Table, PriceIden::Date),
        (PriceIden::Table, PriceIden::Time),
        (PriceIden::Table, PriceIden::Value),
        (PriceIden::Table, PriceIden::Denom),
        (PriceIden::Table, PriceIden::Currency),
    ]
}

fn get_security_columns() -> Vec<(SecurityIden, SecurityIden)> {
    vec![
        (SecurityIden::Table, SecurityIden::Id),
        (SecurityIden::Table, SecurityIden::Namespace),
        (SecurityIden::Table, SecurityIden::Symbol),
        (SecurityIden::Table, SecurityIden::Updater),
        (SecurityIden::Table, SecurityIden::Currency),
        (SecurityIden::Table, SecurityIden::LedgerSymbol),
        (SecurityIden::Table, SecurityIden::Notes),
    ]
}

/// Generates SELECT statement with the given parameters/filters.
fn generate_select_security_with_filter(filter: &SecurityFilter) -> (String, RusqliteValues) {
    let query = Query::select()
        // Order of columns:
        .columns(get_security_columns())
        //
        .from(SecurityIden::Table)
        .conditions(
            filter.agent.is_some(),
            |q| {
                if let Some(agent) = filter.agent.to_owned() {
                    q.and_where(Expr::col(SecurityIden::Updater).eq(agent));
                }
            },
            |_q| {},
        )
        .conditions(
            filter.currency.is_some(),
            |q| {
                if let Some(cur) = filter.currency.to_owned() {
                    let uppercase_cur = cur.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Currency).eq(uppercase_cur));
                }
            },
            |_q| {},
        )
        .conditions(
            filter.exchange.is_some(),
            |q| {
                if let Some(exc) = filter.exchange.to_owned() {
                    let uppercase_exc = exc.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Namespace).eq(uppercase_exc));
                }
            },
            |_q| {},
        )
        .conditions(
            filter.symbol.is_some(),
            |q| {
                if let Some(sym) = filter.symbol.to_owned() {
                    let uppercase_sym = sym.to_uppercase();
                    q.and_where(Expr::col(SecurityIden::Symbol).eq(uppercase_sym));
                }
            },
            |_q| {},
        )
        // .to_owned();
        .build_rusqlite(SqliteQueryBuilder);

    // query.build(SqliteQueryBuilder)
    // query.to_string(SqliteQueryBuilder)
    // query.build_rusqlite(SqliteQueryBuilder)
    query
}

/// Select all securities that have linked price records.
fn generate_select_securities_having_prices() -> (String, RusqliteValues) {
    Query::select()
        .columns(get_security_columns())
        .from(SecurityIden::Table)
        .inner_join(
            PriceIden::Table,
            Expr::tbl(SecurityIden::Table, SecurityIden::Id)
                .equals(PriceIden::Table, PriceIden::SecurityId),
        )
        .order_by(
            (SecurityIden::Table, SecurityIden::Namespace),
            sea_query::Order::Asc,
        )
        .order_by(
            (SecurityIden::Table, SecurityIden::Symbol),
            sea_query::Order::Asc,
        )
        .build_rusqlite(SqliteQueryBuilder)
}

#[allow(dead_code)]
enum PriceSymbolOrder {
    Symbol,
    DateTime,
}

#[cfg(test)]
mod tests {
    use sea_query_rusqlite::RusqliteValue;
    use test_log::test;

    use crate::model::SecurityFilter;

    use super::*;

    /// Creates a dummy dal and prepares an in-memory test database.
    fn get_test_dal() -> RuSqliteDal {
        let dal = RuSqliteDal::new(":memory:".to_string());
        dal.create_tables();

        insert_dummy_prices(&dal);
        insert_dummy_securities(&dal);

        dal
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

    #[test]
    fn test_sec_query_wo_params() {
        let filter = SecurityFilter {
            currency: None,
            agent: None,
            exchange: None,
            symbol: None,
        };
        let (sql, values) = generate_select_security_with_filter(&filter);

        let expected = r#"SELECT "security"."id", "security"."namespace", "security"."symbol", "security"."updater", "security"."currency", "security"."ledger_symbol", "security"."notes" FROM "security""#;
        assert_eq!(expected, sql);

        // There are no parameters.
        assert!(values.0.len() == 0);
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

        let expected = r#"SELECT "security"."id", "security"."namespace", "security"."symbol", "security"."updater", "security"."currency", "security"."ledger_symbol", "security"."notes" FROM "security" WHERE "currency" = ?"#;
        assert_eq!(expected, sql);
        let exp_val = RusqliteValue(sea_query::Value::String(Some(Box::new("AUD".to_owned()))));
        assert_eq!(exp_val, values.0[0]);
    }

    // #[test]
    // fn test_null_param() {
    //     let sql = r#"SELECT *
    //     FROM MY_TABLE
    //     WHERE @parameter IS NULL OR NAME = @parameter;"#;
    // }

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

    #[test]
    fn test_select_securities_having_prices() {
        let (sql, values) = generate_select_securities_having_prices();

        assert_eq!(
            r#"SELECT "security"."id", "security"."namespace", "security"."symbol", "security"."updater", "security"."currency", "security"."ledger_symbol", "security"."notes" FROM "security" INNER JOIN "price" ON "security"."id" = "price"."security_id" ORDER BY "security"."namespace" ASC, "security"."symbol" ASC"#,
            sql
        );
        assert!(values.0.len() == 0);
    }
}
