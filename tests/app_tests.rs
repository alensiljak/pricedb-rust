/*
 * Integration tests
 */
use test_log::test;

use pricedb::App;

/**
 * Just a basic smoke test to see that the pruninng runs.
 * Tests the use of the database.
 * 
 * The test assumes that IPRP security exists in the db.
 * It expects 1 record to be processed.
 */
#[test]
fn test_prune() {
    let app = App::new();

    let symbol = Some("IPRP".to_string());
    let actual = app.prune(&symbol);

    let expected = 1;
    assert_eq!(actual, expected);
}

// /// Try multiple actions
// #[test]
// fn roundtrip( ) {
//     let app = App::new();
// }