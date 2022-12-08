/*
 * Integration tests
 */
use test_log::test;

use pricedb::App;

#[test]
fn test_prune() {
    let app = App::new();

    let symbol = Some("IPRP".to_string());
    let actual = app.prune(&symbol);

    let expected = 0;
    assert_eq!(actual, expected);
}
