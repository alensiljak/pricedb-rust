use rusqlite::{Connection, Result};

/// rusqlite connection
fn open_connection() -> Result<Connection> {
    let con_str = load_db_path();
    let connection = Connection::open(con_str)?;
    return Ok(connection);
}
