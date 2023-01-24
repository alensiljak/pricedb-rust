use rstest::{fixture, rstest};
/*
 * Integration tests
 */
use test_log::test;

use pricedb::{
    config::PriceDbConfig,
    model::Price,
    App,
};

/// Sets up an in-memory database.
#[fixture]
fn app() -> App {
    let cfg = PriceDbConfig::default();

    // initialize database
    let app = App::new(cfg);

    app
}

#[fixture]
fn new_price() -> Price {
    Price {
        symbol: "VTI".to_owned(),
        id: 0,
        date: "2022-12-01".into(),
        time: "13:25:44".into(),
        value: 1033,
        denom: 100,
        currency: "EUR".into()
    }
}

/// Populates the database
#[fixture]
fn app_with_data(app: App) -> App {
    // let sec = Security {
    //     id: 1,
    //     namespace: Some("AMS".to_owned()),
    //     symbol: "IPRP".to_owned(),
    //     currency: Some("EUR".to_owned()),
    //     ledger_symbol: Some("IPRP_AS".to_owned()),
    //     updater: Some("yahoo_finance".to_owned()),
    //     notes: None,
    // };
    // app.get_dal().add_security(&sec);

    // add some prices
    let mut price = Price::new();
    price.symbol = "AMS:VHYL".into();
    app.get_dal().add_price(&price);

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
fn test_prune(app: App) {
    let symbol = Some("IPRP".to_string());
    let actual = app.prune(&symbol);

    let expected = 1;
    assert_eq!(actual, expected);
}

#[rstest]
fn test_prune_all(app_with_data: App) {
    let actual = app_with_data.prune(&None);

    log::debug!("deleted: {:?}", actual);
    assert_ne!(0, actual);
    assert_eq!(1, actual);
}

/// Try multiple actions
#[rstest]
fn roundtrip(app: App, new_price: Price) {
    // add price
    app.add_price(&new_price);

    app.prune(&None);

    // retrieve list
    let output = app.list_prices(&None, &None, &None);

    assert_eq!(
        "VTI 2022-12-01 13:25:44 10.33 EUR",
        output
    );
}

/*
#[test]
/// Displays the performance of finding an element in a vector, comparing
/// find() and filter().
fn test_indexing_performance() {
    use std::time::Instant;

    let tuples = vec![(1, "one"), (2, "two"), (3, "three")];

    // Measure the time it takes to search the vector of tuples using the find() method
    let start = Instant::now();
    let _result = tuples.iter().find(|t| t.0 == 2);
    let elapsed = start.elapsed();
    println!("find() method: {:?}", elapsed);

    // Measure the time it takes to search the vector of tuples using the filter() method
    let start = Instant::now();
    for tuple in tuples.iter().filter(|t| t.0 == 2) {
        println!("{:?}", tuple);
    }
    let elapsed = start.elapsed();
    println!("filter() method: {:?}", elapsed);

    assert!(false);
}
*/
