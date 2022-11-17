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

use diesel::{sqlite::SqliteConnection, Connection};

use crate::database;

pub fn establish_connection() -> SqliteConnection {
    let db_path = database::load_db_path();

    let x = SqliteConnection::establish(db_path.as_str()).unwrap();
    return x;
}