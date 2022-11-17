/*
 * Model structs
 */

use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(unused)]
 pub struct Security {
    pub id: i64,
    pub symbol: String,
    pub namespace: String,
    pub updater: String,
    pub currency: String,
    pub ledger_symbol: Option<String>
    // prices
}

impl Security {
    /// Creates a new instance
    #[allow(unused)]
    pub fn new() -> Security {
        Security {
            id: 0,
            namespace: "".to_string(),
            symbol: "".to_string(),
            currency: "".to_string(),
            updater: "".to_string(),
            ledger_symbol: Some("".to_string())
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
pub struct SecuritySymbol {
    pub namespace: String,
    pub mnemonic: String
}
