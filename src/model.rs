/*!
 * Model definitions
 */

use std::fmt::Display;

use rust_decimal::Decimal;
use sea_query::enum_def;

#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[enum_def]
pub struct Price {
    /// Symbol in format NAMESPACE:SYMBOL
    pub symbol: String,
    pub id: i64,
    pub date: String,
    pub time: String,
    pub value: i64,
    pub denom: i64,
    pub currency: String,
}

impl Price {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_decimal(&self) -> Decimal {
        let scale = self.scale();

        Decimal::new(self.value, scale)
    }

    pub fn scale(&self) -> u32 {
        let denom_f = self.denom as f64;
        let scale = denom_f.log10();

        scale as u32
    }

    pub fn default_time() -> String {
        "00:00:00".to_owned()
    }
}

#[derive(Debug, Default)]
pub struct PriceFilter {
    pub symbol: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
}

impl PriceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SecurityFilter {
    pub currency: Option<String>,
    pub agent: Option<String>,
    pub exchange: Option<String>,
    pub symbol: Option<String>,
}

impl SecurityFilter {
    pub fn new() -> Self {
        Self {
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
    pub fn new(symbol: &str) -> SecuritySymbol {
        // Create default values
        let mut namespace = String::default();
        let mut mnemonic = symbol.to_uppercase();

        // Try simple parsing.
        let parts: Vec<&str> = symbol.split(':').collect();
        if parts.len() > 1 {
            namespace = parts[0].to_uppercase();
            mnemonic = parts[1].to_uppercase();
        }
        log::debug!("parts: {:?}", &parts);

        SecuritySymbol {
            namespace,
            mnemonic,
        }
    }

    pub fn new_separated(exchange: &str, symbol: &str) -> Self {
        Self {
            namespace: exchange.to_uppercase(),
            mnemonic: symbol.to_uppercase()
        }
    }
}

impl Display for SecuritySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.mnemonic)?;
        Ok(())
    }
}

impl From<&str> for SecuritySymbol {
    fn from(item: &str) -> Self {
        SecuritySymbol::new(item)
    }
}

// pub struct PriceWSymbol {
//     pub id: i64,
//     pub namespace: String,
//     pub symbol: String,
//     pub ledger_symbol: String,
//     pub date: String,
//     pub time: String,
//     pub value: Decimal,
//     pub currency: String,
// }

// impl PriceWSymbol {
//     pub fn from(price: &Price, sec: &Security) -> Self {
//         Self {
//             id: price.id,
//             namespace: sec.namespace.to_owned().unwrap_or_default(),
//             symbol: sec.symbol.to_owned(),
//             ledger_symbol: sec.ledger_symbol.to_owned().unwrap_or_default(),
//             date: price.date.to_owned(),
//             time: price.time.to_owned(),
//             value: price.to_decimal(),
//             currency: price.currency.to_owned(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = SecuritySymbol::new("XETRA:EL4X");

        assert_eq!(s.namespace, "XETRA");
        assert_eq!(s.mnemonic, "EL4X");
    }

    #[test]
    fn test_parse_currency() {
        let s = SecuritySymbol::new("AUD");

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

    #[test]
    /// What is the default?
    fn test_sec_filter_default() {
        let def = SecurityFilter::default();
        let new = SecurityFilter::new();

        assert_eq!(def, new);
    }
}
