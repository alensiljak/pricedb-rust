use rstest::fixture;
/*
 * Integration tests
 */

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
