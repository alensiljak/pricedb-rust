use std::fs;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

/**
 * Parses (loads and saves) Price records in the ledger-formatted price
 * text file.
 */

#[derive(Debug, Default)]
pub struct PriceRecord {
    datetime: NaiveDateTime,
    symbol: String,
    value: Decimal,
    currency: String,
}

impl PriceRecord {}

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
    let line_parts: Vec<&str> = line.split_whitespace().collect();
    // log::debug!("line parts: {:?}", line_parts);

    let parts_num = line_parts.len();
    // log::debug!("parts count: {:?}", parts_num);

    // parse
    let result;
    if parts_num == 5 {
        // 5 parts => no time
        result = parse_with_no_time(&line_parts);
    } else if parts_num == 6 {
        // 6 parts => have time
        result = parse_with_time(&line_parts);
    } else {
        panic!("invalid number of parts parsed from the line!");
    }
    // log::debug!("result: {:?}", result);

    result
}

fn parse_with_time(items: &Vec<&str>) -> PriceRecord {
    // now add time
    let date_time_string = format!("{0} {1}", items[1].to_owned(), items[2].to_owned());
    // log::debug!("date time string: {:?}", date_time_string);

    PriceRecord {
        datetime: NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %H:%M:%S")
            .expect("parsed date/time"),
        symbol: items[3].to_owned(),
        value: Decimal::from_str_exact(items[4]).expect("parsed value"),
        currency: items[5].to_owned(),
    }
}

fn parse_with_no_time(items: &Vec<&str>) -> PriceRecord {
    let date_time_string = format!("{0} 00:00:00", items[1].to_owned());

    PriceRecord {
        datetime: NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %H:%M:%S")
            .expect("parsed date"),
        symbol: items[2].to_owned(),
        value: Decimal::from_str_exact(items[3]).expect("parsed value"),
        currency: items[4].to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDateTime, Timelike};

    #[test]
    fn test_parsing_date_time() {
        let date_time_string = "2022-03-04 17:01:02";
        let result = NaiveDateTime::parse_from_str(&date_time_string, "%Y-%m-%d %H:%M:%S");
        assert!(result.is_ok());

        let actual = result.expect("parsed");

        assert_eq!(2022, actual.date().year());
        assert_eq!(3, actual.date().month());
        assert_eq!(4, actual.date().day());
        assert_eq!(17, actual.time().hour());
        assert_eq!(1, actual.time().minute());
        assert_eq!(2, actual.time().second());
    }
}
