/*!
 * Formats prices for Ledger
 */

use crate::model::PriceWSymbol;

/*
/// format a list of prices
pub(crate) fn format_prices(prices: Vec<Price>, securities: &[Security]) -> String {
    let mut output = String::default();

    for price in prices {
        // log::debug!("fomatting {price:?}");
        // log::debug!("sec: {securities:?}");

        // find the matching security by id
        let sec = securities
            .iter()
            .find(|x| x.id == price.security_id)
            .expect("a matching security");
        // let sec = securities[&price.security_id];

        output += format_price(&price, sec).as_str();
        output += "\n";
    }
    output
}
*/

pub fn format_prices_w_symbols(prices: Vec<PriceWSymbol>) -> String {
    let mut output = String::default();
    for pws in prices {
        output += format_price_w_symbol(&pws).as_str();
        output += "\n";
    }
    output
}

/** Formats a single Price record.
 * ledger price format, ISO format supported:
 * P 2004-06-21 02:17:58 VTI $27.76
 */
/*
fn format_price(price: &Price, sec: &Security) -> String {
    let date = price.date.to_owned();
    let time = price.time.to_owned();
    let date_time = format!("{date} {time}");

    let mnemonic = match sec.ledger_symbol {
        Some(_) => sec.ledger_symbol.to_owned(),
        None => Some(sec.symbol.to_owned()),
    }
    .expect("valid symbol");
    let value = price.to_decimal();
    let currency = &price.currency;

    format!("P {date_time} {mnemonic} {value} {currency}")
}
 */

fn format_price_w_symbol(pws: &PriceWSymbol) -> String {
    let date = pws.date.to_owned();
    let time = pws.time.to_owned();
    let date_time = format!("{date} {time}");

    let symbol = if pws.ledger_symbol.is_empty() {
        &pws.symbol
    } else {
        &pws.ledger_symbol
    };
    let value = pws.value;
    let currency = &pws.currency;

    format!("P {date_time} {symbol} {value} {currency}")
}

#[cfg(test)]
mod tests {
    

    // use super::*;

    /*
    #[test]
    fn test_single_price_formatting() {
        let mut symbol = Security::new();
        symbol.ledger_symbol = Some("EL4X_DE".into());

        let price = Price {
            id: 113,
            security_id: 26,
            date: "2022-12-01".into(),
            time: "12:25:34".into(),
            value: 12534,
            denom: 100,
            currency: "EUR".into(),
        };

        let actual = format_price(&price, &symbol);

        // println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 12:25:34 EL4X_DE 125.34 EUR");
    }
 */

 /*
    #[test]
    fn test_format_price_wo_time() {
        let mut symbol = Security::new();
        symbol.ledger_symbol = Some("VAS_AX".into());

        let price = Price {
            id: 113,
            security_id: 26,
            date: "2022-12-01".into(),
            time: Price::default_time(),
            value: 12534,
            denom: 100,
            currency: "AUD".into(),
        };

        let actual = format_price(&price, &symbol);

        // println!("{actual:?}");

        assert_eq!(actual, "P 2022-12-01 00:00:00 VAS_AX 125.34 AUD");
    }
     */
}
