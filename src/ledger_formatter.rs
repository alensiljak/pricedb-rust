use crate::model::Price;

/**
 * Formats prices for Ledger
 *
 */

/// format a list of prices
pub(crate) fn format_prices(prices: Vec<Price>) -> String {
    let mut output = String::default();
    for price in prices {
        // output.push_str(format_price(price));
        output += "\n";

        todo!("complete");
    }
    output
}

/**
 * Formats single Price record.
 * ledger price format, ISO format supported:
 * P 2004-06-21 02:17:58 VTI $27.76
 */
fn format_price(price: Price) -> String {
    todo!("load symbols")

    // let mnemonic = price.symbol.mnemonic;
    // format!("P {date_time} {mnemonic} {price.value} {price.currency}")
}
