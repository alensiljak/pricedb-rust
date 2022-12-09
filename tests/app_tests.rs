use rstest::{fixture, rstest};
/*
 * Integration tests
 */
use test_log::test;

use pricedb::{App, config::PriceDbConfig, model::Price};

#[fixture]
fn app() -> App {
    let cfg = PriceDbConfig {
        alphavantage_api_key: String::default(),
        export_destination: String::default(),
        fixerio_api_key: String::default(),
        price_database_path: ":memory:".to_owned(),
    };
    let app = App::new(cfg);
    app
}

#[fixture]
fn new_price() -> Price {
    Price::default()
}

/**
 * Just a basic smoke test to see that the pruninng runs.
 * Tests the use of the database.
 *
 * The test assumes that IPRP security exists in the db.
 * It expects 1 record to be processed.
 */
#[rstest]
fn test_prune(app: App) {

    let symbol = Some("IPRP".to_string());
    let actual = app.prune(&symbol);

    let expected = 1;
    assert_eq!(actual, expected);
}

/// Try multiple actions
#[rstest]
fn roundtrip(app: App, new_price: Price) {
    app.add_price(new_price);
    let output = app.list_prices(&None, &None, &None);

    assert_eq!("", output);
}
