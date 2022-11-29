/*
 * Model structs
 */

#[derive(Debug)]
#[allow(unused)]
pub struct Price {
    pub id: i32,
    pub security_id: i32,
    pub date: String,
    pub time: Option<String>,
    pub value: i32,
    pub denom: i32,
    pub currency: String,
}

impl Price {
    pub fn new() -> Price {
        let result = Price {
            id: 0,
            security_id: 0,
            date: "".to_string(),
            time: None,
            value: 0,
            denom: 0,
            currency: "".to_string(),
        };
        return result;
    }
}

#[derive(Debug)]
#[derive(diesel::Queryable)]
#[allow(unused)]
pub struct Security {
    // Diesel uses i32, others i64
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
            notes: Some("".to_string()),
        }
    }
}

pub struct SecurityFilter {
    pub currency: Option<String>,
    pub agent: Option<String>,
    pub exchange: Option<String>,
    pub symbol: Option<String>,
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
