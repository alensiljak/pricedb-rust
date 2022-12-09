/*
Database implementation with rbatis
 */

use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;

pub struct Database {
    pub rb: Rbatis
}

impl Database {
    pub fn new() -> Self {
        let db_path: String = load_db_path();

        let rb = repositories::initialize_database(db_path);
        let result = Database {
            rb
        };
        return result;
    }
}

/// Initialize database with Rbatis.
pub fn initialize_database(db_path: String) -> Rbatis {
    let rb = Rbatis::new();

    let conn_str = format!("sqlite://{}", db_path);
    rb.init(SqliteDriver {}, &conn_str).unwrap();

    return rb;
}

/*
 * Repositories with rbatis
 */

use rbatis::crud;

use crate::model::Security;

crud!(Security{});
