use rust_decimal::{Decimal, prelude::ToPrimitive};

use crate::model::{Price, SecuritySymbol};

/**
 * Formats prices for Ledger
 *
 */

#[allow(unused)]
/// format a list of prices
pub(crate) fn format_prices(prices: Vec<Price>) -> String {
    let mut output = String::default();

    todo!("load symbols");
    let symbol = SecuritySymbol::parse("symbol");

    for price in prices {
        output += format_price(&price, &symbol).as_str();
        output += "\n";

        todo!("complete");
    }
    output
}

#[allow(unused)]
/** Formats a single Price record.
 * ledger price format, ISO format supported:
 * P 2004-06-21 02:17:58 VTI $27.76
 */
fn format_price(price: &Price, symbol: &SecuritySymbol) -> String {
    let date = price.date.to_owned();
    let time = match &price.time {
        Some(price_time) => price_time.to_owned(),
        None => String::default(),
    };
    let date_time = format!("{date} {time}");

    let mnemonic = &symbol.mnemonic;
    //let value = price.value.to_f32().unwrap() / price.denom.to_f32().unwrap();
    let value = price.to_decimal();
    let currency = &price.currency;

    format!("P {date_time} {mnemonic} {value} {currency}")
}

#[cfg(test)]
mod tests {
    use crate::model::{Price, SecuritySymbol};

    use super::*;

    #[test]
    fn test_single_price_formatting() {
        let symbol = SecuritySymbol::parse("XETRA:EL4X");
        let price = Price {
            id: 113,
            security_id: 26,
            date: "2022-12-01".into(),
            time: Some("12:25:34".into()),
            value: 12534,
            denom: 100,
            currency: "EUR".into(),
        };

        let actual = format_price(&price, &symbol);

        println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 12:25:34 EL4X 125.34 EUR");
    }
}
