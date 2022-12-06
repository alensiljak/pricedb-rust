/*
 * Fixerio price downloader, integration tests
 */
use pricedb::quote::fixerio::*;

#[test]
fn test_instantiation() {
    let f = Fixerio::new();

    assert_ne!(key, String::default());
    assert_eq!(key.len(), 32);
}
