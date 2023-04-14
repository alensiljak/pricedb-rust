use std::fs;

use chrono::NaiveDateTime;

/**
 * Parses (loads and saves) Price records in the ledger-formatted price
 * text file.
 */

#[derive(Debug, Default)]
pub struct PriceRecord {
    datetime: NaiveDateTime,
    symbol: String,
    currency: String
}

impl PriceRecord {
    pub fn new() -> Self {
        PriceRecord::default()
    }
}

pub fn load_prices(prices_path: &str) -> Vec<PriceRecord> {
    let content = fs::read_to_string(prices_path).expect("Error reading rates file");
    // log::debug!("price file: {:?}", content);
    let lines = content.lines();
    // log::debug!("price lines: {:?}", lines);

    let mut prices = vec![];
    for line in lines {
        let price = parse_line(line);
        prices.push(price);
    }

    prices
}

/// Parses price line
/// P 2023-04-14 00:00:00 GBP 1.132283 EUR
fn parse_line(line: &str) -> PriceRecord {
    let line_parts_iter: Vec<&str> = line.split_whitespace().collect();
    log::debug!("line parts: {:?}", line_parts_iter);

    // 5 parts => no time
    // 6 parts => have time
    todo!("complete parsing");

    let result = PriceRecord::new();

    result
}
