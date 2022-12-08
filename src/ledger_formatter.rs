/*
 * Formats prices for Ledger
 */

use crate::{
    model::{Price, Security},
};

/// format a list of prices
pub(crate) fn format_prices(prices: Vec<Price>, symbols: &Vec<Security>) -> String {
    let mut output = String::default();

    for price in prices {
        // find the matching symbol
        let symbol = symbols.iter().find(|x| x.id == price.security_id).expect("a matching symbol");

        output += format_price(&price, symbol).as_str();
        output += "\n";

        todo!("complete");
    }
    output
}

/** Formats a single Price record.
 * ledger price format, ISO format supported:
 * P 2004-06-21 02:17:58 VTI $27.76
 */
fn format_price(price: &Price, symbol: &Security) -> String {
    let date = price.date.to_owned();
    let time = match &price.time {
        Some(price_time) => price_time.to_owned(),
        None => String::default(),
    };
    let date_time = format!("{date} {time}");

    let mnemonic = match symbol.ledger_symbol {
        Some(_) => symbol.ledger_symbol.to_owned(),
        None => Some(symbol.symbol.to_owned()),
    }.expect("valid symbol");
    let value = price.to_decimal();
    let currency = &price.currency;

    format!("P {date_time} {mnemonic} {value} {currency}")
}

#[cfg(test)]
mod tests {
    use crate::model::Price;

    use super::*;

    #[test]
    fn test_single_price_formatting() {
        let mut symbol = Security::new();
        symbol.ledger_symbol = Some("EL4X_DE".into());

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

        // println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 12:25:34 EL4X_DE 125.34 EUR");
    }
}
