/*
 * Model structs
 */
#[derive(Debug)]
#[allow(unused)]
 pub(crate) struct Security {
    pub(crate) id: i64,
    pub(crate) symbol: String,
    pub(crate) namespace: String,
    pub(crate) updater: String,
    pub(crate) currency: String,
    pub(crate) ledger_symbol: String
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
            ledger_symbol: "".to_string()
        }
    }
}