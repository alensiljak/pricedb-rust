/*!
 * Formats prices for Ledger
 */
use as_symbols::SymbolMetadata;

use crate::model::Price;

/// format a list of prices
pub(crate) fn format_prices(prices: Vec<Price>, securities: Vec<SymbolMetadata>) -> String {
    let mut output = String::default();

    for price in prices {
        // log::debug!("fomatting {price:?}");
        // log::debug!("sec: {securities:?}");

        // find the matching symbol by id
        let sec = securities
            .iter()
            .find(|sm| sm.symbol_w_namespace() == price.symbol)
            .expect("a matching symbol");
        // let sec = securities[&price.symbol];

        output += format_price(&price, sec).as_str();
        output += "\n";
    }
    output
}

/** Formats a single Price record.
 * ledger price format, ISO format supported:
 * P 2004-06-21 02:17:58 VTI $27.76
 */
fn format_price(price: &Price, sec: &SymbolMetadata) -> String {
    let date = price.date.to_owned();
    let time = price.time.to_owned();
    let date_time = format!("{date} {time}");

    let symbol = match &sec.ledger_symbol {
        Some(ledger_symbol) => Some(ledger_symbol.to_owned()),
        None => Some(sec.symbol.to_owned()),
    }
    .expect("valid symbol");
    let value = price.to_decimal();
    let currency = &price.currency;

    format!("P {date_time} {symbol} {value} {currency}")
}

#[cfg(test)]
mod tests {

    // use super::*;

    use as_symbols::SymbolMetadata;

    use crate::{ledger_formatter::format_price, model::Price};

    #[test]
    fn test_single_price_formatting() {
        let mut sm = SymbolMetadata::new();
        sm.symbol = "EL4X".into();
        sm.ledger_symbol = Some("EL4X_DE".into());

        let price = Price {
            symbol: "EL4X".to_string(),
            id: 113,
            date: "2022-12-01".into(),
            time: "12:25:34".into(),
            value: 12534,
            denom: 100,
            currency: "EUR".into(),
        };

        let actual = format_price(&price, &sm);

        // println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 12:25:34 EL4X_DE 125.34 EUR");
    }

    #[test]
    fn test_format_price_wo_time() {
        let mut sm = SymbolMetadata::new();
        sm.symbol = "VAS".into();
        sm.ledger_symbol = Some("VAS_AX".into());

        let price = Price {
            symbol: "VAS".into(),
            id: 113,
            date: "2022-12-01".into(),
            time: Price::default_time(),
            value: 12534,
            denom: 100,
            currency: "AUD".into(),
        };

        let actual = format_price(&price, &sm);

        // println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 00:00:00 VAS_AX 125.34 AUD");
    }
}
