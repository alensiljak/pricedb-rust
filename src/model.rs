/*
 * Model structs
 */

use diesel::prelude::*;

#[derive(Debug, Queryable, PartialEq)]
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
            notes: Some("".to_string())
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct SecuritySymbol {
    pub namespace: String,
    pub mnemonic: String
}
