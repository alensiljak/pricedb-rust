/*
 * Model structs
 */

#[derive(Debug)]
#[allow(unused)]
pub struct Price {
    pub id: i64,
    pub security_id: i64,
    pub date: String,
    pub time: Option<String>,
    pub value: i64,
    pub denom: i64,
    pub currency: String
}

#[derive(Debug)]
#[allow(unused)]
pub struct Security {
    pub id: i64,
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
#[allow(unused)]
pub struct SecuritySymbol {
    pub namespace: String,
    pub mnemonic: String,
}

impl SecuritySymbol {
    /// Parse symbol syntax "XETRA:EL4X"
    pub fn parse(symbol: String) -> (String, String) {
        let namespace = String::from("");
        let mnemonic = symbol.clone();

        let parts = symbol.split(":");

        log::debug!("parts: {:?}", parts);

        if parts.count() > 1 {
            // let vec: Vec<&str> = parts.collect();
            // namespace = vec.get(0).expect("namespace not provided");
            // mnemonic = vec.get(1).unwrap();
        }

        return (namespace, mnemonic);
    }
}
