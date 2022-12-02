/*
 * Model structs
 *
 * Diesel uses i32, others i64
 *
 */

use crate::database::schema::price;

#[derive(Debug, Clone)]
#[allow(unused)]
#[derive(diesel::Queryable, diesel::Identifiable)]
#[diesel(table_name = price)]
pub struct Price {
    pub id: i32,
    pub security_id: i32,
    pub date: String,
    pub time: Option<String>,
    pub value: i32,
    pub denom: i32,
    // pub value_dec: Decimal,
    pub currency: String,
}

#[derive(Debug, Clone)]
#[derive(diesel::Insertable)]
#[diesel(table_name = price)]
pub struct NewPrice {
    pub security_id: i32,
    pub date: String,
    pub time: Option<String>,
    pub value: i32,
    pub denom: i32,
    // pub value_dec: Decimal,
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
            // value_dec: dec!(0),
        };
        return result;
    }

    pub fn for_insert() -> NewPrice {
        NewPrice {
            security_id: 0,
            date: String::default(),
            time: None,
            value: 0,
            denom: 0,
            currency: String::default(),
        }
    }
}

pub(crate) struct PriceFilter {
    pub security_id: Option<i32>,
    pub date: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, diesel::Queryable)]
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
            notes: Some("".to_string()),
        }
    }
}

#[derive(Debug)]
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
    //// Parse symbol syntax "XETRA:EL4X"
    // #[allow(dead_code)]
    // pub fn parse(symbol: String) -> (String, String) {
    //     let mut namespace = String::from("");
    //     let mnemonic = symbol.clone();

    //     let parts = symbol.split(":");

    //     log::debug!("parts: {:?}", parts);

    //     if parts.count() > 1 {
    //         // let vec: Vec<&str> = parts.collect();
    //         // namespace = vec.get(0).expect("namespace not provided");
    //         // mnemonic = vec.get(1).unwrap();
    //     }

    //     todo!("complete");

    //     return (namespace, mnemonic);
    // }
}

#[cfg(test)]
mod tests {
    // use super::SecuritySymbol;

    // #[test]
    // fn test_parse() {
    //     let s = SecuritySymbol::parse("XETRA:EL4X".to_string());

    //     assert_eq!(s.0, "XETRA");
    //     assert_eq!(s.1, "EL4X");
    // }
}
