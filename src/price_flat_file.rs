/**
* Maintains the prices in a flat-file in Ledger format.
* P 2023-04-14 00:00:00 GBP 1.132283 EUR
*/
use std::{collections::HashMap, fs, fmt::Display};

use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::model::Price;

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Default)]
pub struct PriceRecord {
    pub datetime: NaiveDateTime,
    pub symbol: String,
    pub value: Decimal,
    pub currency: String,
}

impl Display for PriceRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_time_string = if self.datetime.time().to_string() == "00:00:00" {
            format!("{}", self.datetime.date().to_string())
        } else {
            format!("{} {}", self.datetime.date().to_string(), self.datetime.time().to_string())
        };

        write!(f, "P {} {} {} {}", date_time_string, self.symbol, self.value, self.currency)?;
        Ok(())
    }
}

impl From<&Price> for PriceRecord {
    fn from(item: &Price) -> Self {
        let time = if item.time == "" {
            "00:00:00"
        } else {
            item.time.as_str()
        };
        let date_time = format!("{0} {1}", item.date, time);

        PriceRecord {
            datetime: NaiveDateTime::parse_from_str(&date_time, DATE_TIME_FORMAT).expect("parsed date/time"),
            symbol: item.symbol.to_owned(),
            value: item.to_decimal(),
            currency: item.currency.to_owned(),
        }
    }
}

#[derive(Default)]
pub struct PriceFlatFile {
    file_path: String,
    pub prices: HashMap<String, PriceRecord>,
}

impl PriceFlatFile {
    /// Load prices from a text file.
    pub fn load(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Error reading rates file");
        // log::debug!("price file: {:?}", content);
        let lines = content.lines();
        // log::debug!("price lines: {:?}", lines);

        //let mut prices = vec![];
        let mut prices: HashMap<String, PriceRecord> = HashMap::new();

        for line in lines {
            let price = parse_line(line);
            // prices.push(price);
            prices.insert(price.symbol.to_owned(), price);
        }

        Self {
            file_path: file_path.to_owned(),
            prices: prices,
        }
    }

    pub fn save(&self) {
        // order by date/time
        let mut sorted: Vec<(&String, &PriceRecord)> = self.prices.iter().collect();
        sorted.sort_by_key(|item| (&item.1.datetime, &item.1.symbol));

        let mut output = String::default();

        // self.prices.values()
        for price in sorted {
            // log::debug!("price output {:?}", price.1.to_string());
            output += &price.1.to_string();
            output += "\n";
        }
        
        fs::write(&self.file_path, output).expect("saved successfully");
    }
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
        datetime: NaiveDateTime::parse_from_str(&date_time_string, DATE_TIME_FORMAT)
            .expect("parsed date/time"),
        symbol: items[3].to_owned(),
        value: Decimal::from_str_exact(items[4]).expect("parsed value"),
        currency: items[5].to_owned(),
    }
}

fn parse_with_no_time(items: &Vec<&str>) -> PriceRecord {
    let date_time_string = format!("{0} 00:00:00", items[1].to_owned());

    PriceRecord {
        datetime: NaiveDateTime::parse_from_str(&date_time_string, DATE_TIME_FORMAT)
            .expect("parsed date"),
        symbol: items[2].to_owned(),
        value: Decimal::from_str_exact(items[3]).expect("parsed value"),
        currency: items[4].to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDateTime, Timelike};
    use rust_decimal::{prelude::FromPrimitive, Decimal};

    use crate::{price_flat_file::{PriceFlatFile, DATE_TIME_FORMAT}, model::Price};

    use super::PriceRecord;

    #[test]
    fn test_parsing_date_time() {
        let date_time_string = "2022-03-04 17:01:02";
        let result = NaiveDateTime::parse_from_str(&date_time_string, DATE_TIME_FORMAT);
        assert!(result.is_ok());

        let actual = result.expect("parsed");

        assert_eq!(2022, actual.date().year());
        assert_eq!(3, actual.date().month());
        assert_eq!(4, actual.date().day());
        assert_eq!(17, actual.time().hour());
        assert_eq!(1, actual.time().minute());
        assert_eq!(2, actual.time().second());
    }

    #[test]
    fn test_parsing_empty_time() {
        // Price -> PriceRecord
        let price = Price {
            symbol: "HY".into(),
            id: 1,
            date: "2023-03-04".into(),
            time: "".into(),
            value: 150.into(),
            denom: 10,
            currency: "EUR".into(),
        };

        let date_time_string = "2023-03-04 00:00:00";
        let result = NaiveDateTime::parse_from_str(&date_time_string, DATE_TIME_FORMAT);
        assert!(result.is_ok());

        let actual = PriceRecord::from(&price);

        assert_eq!(result.unwrap(), actual.datetime);
    }

    #[test]
    fn test_load() {
        let actual = PriceFlatFile::load("tests/prices.txt");

        // test the number of records in the file.
        assert_eq!(3, actual.prices.len());
    }

    #[test]
    fn test_add() {
        // Create an empty list
        let mut prices_file = PriceFlatFile::default();

        let price = PriceRecord {
            datetime: NaiveDateTime::parse_from_str("2023-03-04 12:24:36", DATE_TIME_FORMAT)
                .expect("date"),
            symbol: "EL4X_DE".into(),
            value: Decimal::from_str_exact("150").expect("parsed"),
            currency: "EUR".into(),
        };
        // Add a new price to the list
        prices_file.prices.insert(price.symbol.to_owned(), price);
        assert_eq!(1, prices_file.prices.len());
    }

    /// Add a price with the same symbol to test replacement.
    #[test]
    fn test_add_new_value() {
        // Create an empty list
        let mut prices_file = PriceFlatFile::default();

        let price = PriceRecord {
            datetime: NaiveDateTime::parse_from_str("2023-03-04 12:24:36", DATE_TIME_FORMAT)
                .expect("date"),
            symbol: "EL4X_DE".into(),
            value: Decimal::from_i16(150).expect("parsed"),
            currency: "EUR".into(),
        };
        // Add a new price to the list
        prices_file.prices.insert(price.symbol.to_owned(), price);
        assert_eq!(1, prices_file.prices.len());
        assert_eq!(
            Decimal::from_i16(150).unwrap(),
            prices_file.prices.values().next().expect("got first").value
        );

        // Add a price for the same symbol
        let price2 = PriceRecord {
            datetime: NaiveDateTime::parse_from_str("2023-03-04 13:00:00", DATE_TIME_FORMAT)
                .expect("date"),
            symbol: "EL4X_DE".into(),
            value: Decimal::from_i16(155).expect("parsed"),
            currency: "EUR".into(),
        };
        // Add a new price to the list
        prices_file.prices.insert(price2.symbol.to_owned(), price2);

        // Still must have only one record.
        assert_eq!(1, prices_file.prices.len());
        // with the new value.
        assert_eq!(
            Decimal::from_i16(155).unwrap(),
            prices_file.prices.values().next().expect("got first").value
        );
    }

    #[test]
    fn test_format_wo_time() {
        let price = PriceRecord {
            datetime: NaiveDateTime::parse_from_str("2023-04-15 00:00:00", DATE_TIME_FORMAT).unwrap(),
            symbol: "VEUR_AS".into(),
            value: Decimal::from_f32(13.24).unwrap(),
            currency: "EUR".into(),
        };

        let actual = price.to_string();

        assert_eq!("P 2023-04-15 VEUR_AS 13.24 EUR", actual);
    }

    #[test]
    fn test_format_with_time() {
        let price = PriceRecord {
            datetime: NaiveDateTime::parse_from_str("2023-04-15 10:00:00", DATE_TIME_FORMAT).unwrap(),
            symbol: "VEUR_AS".into(),
            value: Decimal::from_f32(13.24).unwrap(),
            currency: "EUR".into(),
        };

        let actual = price.to_string();

        assert_eq!("P 2023-04-15 10:00:00 VEUR_AS 13.24 EUR", actual);
    }
}
