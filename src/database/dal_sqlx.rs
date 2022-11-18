/*
 * sqlx DAL
 */

use super::Dal;

struct Sqlx_Dal {}

impl Dal for Sqlx_Dal {
    fn get_securities(currency: Option<String>, agent: Option<String>, 
        mnemonic: Option<String>, exchange: Option<String>) -> Vec<crate::model::Security> {
        todo!("implement");
    }
}

fn test() {
    
}