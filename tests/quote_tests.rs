use std::fs;

use pricedb::{App, config::PriceDbConfig, model::SecurityFilter};

/**
 * Tests for the `quote` functionality.
 */

fn create_test_file(filename: &str, content: &str) {
    //let path = format!("")
    fs::write(filename, content).unwrap();

    //PriceFlatFile::
}

// #[tokio::test]
// async fn test_addition() {
//     let cfg = PriceDbConfig::default();
//     let app = App::new(cfg);
//     let price_path = "price_test.txt";

//     let content = r#"P 2023-04-14 GBP 1.132283 EUR
// P 2023-04-15 12:00:00 VEUR_AS 1.5 EUR
// P 2023-03-11 USD 1.11 EUR
// "#;
//     create_test_file(price_path, content);

//     let mut filter = SecurityFilter::new();
//     filter.symbol = Some("VEUR".into());

//     app.dl_quote("tests/symbols.csv", price_path, filter).await;

//     let expected = r#"P 2023-03-11 USD 1.11 EUR
// P 2023-04-14 GBP 1.132283 EUR
// P 2023-04-15 12:00:00 VEUR_AS 1.5 EUR
// "#;

//     let actual = fs::read_to_string(price_path).unwrap();
//     assert_eq!(expected, actual);
// }