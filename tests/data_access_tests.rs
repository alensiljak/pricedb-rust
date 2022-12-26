/*
 * Using PriceDb as a library.
 * Accessing the data from an external application.
 */

#[test]
fn read_securities() {
    let cfg = pricedb::load_config();
    let p = pricedb::App::new(cfg);

    let dal = p.get_dal();
    let secs = dal.get_securities(None);

    assert!(!secs.is_empty());
    assert!(secs.len() > 1);
}