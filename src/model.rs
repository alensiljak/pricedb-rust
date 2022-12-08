/*
 * Model structs
 *
 * Diesel uses i32, others i64
 *
 */

use std::fmt::Display;

use rust_decimal::{prelude::ToPrimitive, Decimal};
use sea_query::enum_def;
// use crate::database::schema::price;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[enum_def]
// #[derive(diesel::Queryable, diesel::Identifiable)]
// #[diesel(table_name = price)]
pub struct Price {
    pub id: i32,
    pub security_id: i32,
    pub date: String,
    pub time: Option<String>,
    pub value: i32,
    pub denom: i32,
    // pub value_dec: Decimal,
    pub currency: String,
}

impl Price {
    pub fn new() -> Price {
        let result = Price {
            id: 0,
            security_id: 0,
            date: String::default(),
            time: None,
            value: 0,
            denom: 0,
            currency: String::default(),
            // value_dec: dec!(0),
        };
        return result;
    }

    pub fn to_decimal(&self) -> Decimal {
        let scale = self.scale();

        Decimal::new(self.value.into(), scale)
    }

    pub fn scale(&self) -> u32 {
        let denom_f = self.denom as f64;
        let scale = denom_f.log10();

        scale as u32
    }
}

// #[derive(Debug, Clone, PartialEq)]
// #[derive(diesel::Insertable)]
// #[diesel(table_name = price)]
// pub struct NewPrice {
//     pub security_id: i32,
//     pub date: String,
//     pub time: Option<String>,
//     pub value: i32,
//     pub denom: i32,
//     // pub value_dec: Decimal,
//     pub currency: String,
// }

pub(crate) struct PriceFilter {
    pub security_id: Option<i32>,
    pub date: Option<String>,
    pub time: Option<String>,
}

impl PriceFilter {
    pub fn new() -> PriceFilter {
        PriceFilter {
            security_id: None,
            date: None,
            time: None,
        }
    }
}

#[derive(Debug, Default)]
// #[derive(diesel::Queryable)]
#[enum_def]
#[allow(unused)]
pub struct Security {
    pub id: i32,
    pub namespace: Option<String>,
    pub symbol: String,
    pub updater: Option<String>,
    pub currency: Option<String>,
    pub ledger_symbol: Option<String>,
    pub notes: Option<String>,
    // prices
}

impl Security {
    /// Creates a new instance
    #[allow(unused)]
    pub fn new() -> Security {
        Security {
            id: 0,
            namespace: Some("".to_string()),
            symbol: "".to_string(),
            currency: Some("".to_string()),
            updater: Some("".to_string()),
            ledger_symbol: Some("".to_string()),
            notes: Some("".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct SecurityFilter {
    pub currency: Option<String>,
    pub agent: Option<String>,
    pub exchange: Option<String>,
    pub symbol: Option<String>,
}

impl SecurityFilter {
    pub fn new() -> SecurityFilter {
        SecurityFilter {
            currency: None,
            agent: None,
            exchange: None,
            symbol: None,
        }
    }
}

#[derive(Debug)]
pub struct SecuritySymbol {
    pub namespace: String,
    pub mnemonic: String,
}

impl SecuritySymbol {
    /// Parse symbol syntax, i.e. "XETRA:EL4X"
    pub fn parse(symbol: &str) -> SecuritySymbol {
        let mut namespace = String::default();
        let mut mnemonic = symbol.to_owned();

        let parts = symbol.split(":");

        log::debug!("parts: {:?}", &parts);

        let vec: Vec<&str> = parts.collect();
        log::debug!("parts vector = {:?}", vec);
        if vec.len() > 1 {
            namespace = vec[0].to_string();
            mnemonic = vec[1].to_string();
        }

        return SecuritySymbol {
            namespace,
            mnemonic,
        };
    }
}

impl Display for SecuritySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.mnemonic)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = SecuritySymbol::parse("XETRA:EL4X");

        assert_eq!(s.namespace, "XETRA");
        assert_eq!(s.mnemonic, "EL4X");
    }

    #[test]
    fn test_parse_currency() {
        let s = SecuritySymbol::parse("AUD");

        assert_eq!(s.namespace, "");
        assert_eq!(s.mnemonic, "AUD");
    }

    #[test]
    fn scale_calculation() {
        let mut p = Price::new();

        // we need only the price values
        p.value = 12345;
        p.denom = 100;
        assert_eq!(2, p.scale());

        p.value = 12345;
        p.denom = 1000;
        assert_eq!(3, p.scale());
    }

    #[test]
    fn price_value() {
        let mut p = Price::new();
        // we need only the price values
        p.value = 12345;
        p.denom = 100;

        let actual = p.to_decimal();

        assert_eq!(actual, Decimal::from_str_exact("123.45").unwrap());
        assert_eq!(actual.to_string(), "123.45");
    }
}
