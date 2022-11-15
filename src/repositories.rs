/*
 * Operations on the database
 */

use tracing::debug;

pub(crate) fn test_db() {
    let connection = sqlite::open(":memory:").unwrap();

    let insert_query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";

    connection.execute(insert_query).unwrap();

    // query
    let query = "SELECT * FROM users;";
    let result = connection.execute(query).unwrap();
    debug!("sqlite: {:?}", result);


}
