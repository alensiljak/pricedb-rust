use rstest::{fixture, rstest};
/*
 * Integration tests
 */
use test_log::test;

use pricedb::{App, config::PriceDbConfig, model::{Price, Security}};

#[fixture]
fn app() -> App {
    let cfg = PriceDbConfig {
        alphavantage_api_key: String::default(),
        export_destination: String::default(),
        fixerio_api_key: String::default(),
        price_database_path: ":memory:".to_owned(),
    };

    let app = App::new(cfg);

    // initialize database
    app
}

#[fixture]
fn new_price() -> Price {
    Price::default()
}

/// Populates the database
#[fixture]
fn app_with_data(app: App) -> App {
    let sec = Security {
        id: 1,
        namespace: Some("AMS".to_owned()),
        symbol: "IPRP".to_owned(),
        currency: Some("EUR".to_owned()),
        ledger_symbol: Some("IPRP_AS".to_owned()),
        updater: Some("yahoo_finance".to_owned()),
        notes: None
    };
    app.get_dal().add_security(&sec);

    app
}

/**
 * Just a basic smoke test to see that the pruninng runs.
 * Tests the use of the database.
 *
 * The test assumes that IPRP security exists in the db.
 * It expects 1 record to be processed.
 */
#[rstest]
fn test_prune(app_with_data: App) {
    let app = app_with_data;

    let symbol = Some("IPRP".to_string());
    let actual = app.prune(&symbol);

    let expected = 1;
    assert_eq!(actual, expected);
}

/// Try multiple actions
#[rstest]
fn roundtrip(app: App, new_price: Price) {
    // add price
    app.add_price(new_price);

    app.prune(&None);

    // retrieve list
    let output = app.list_prices(&None, &None, &None);

    assert_eq!(r#"Price { id: 1, security_id: 0, date: "", time: None, value: 0, denom: 0, currency: "" }"#, output);
}
