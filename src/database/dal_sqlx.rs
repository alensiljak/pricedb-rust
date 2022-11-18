/*
 * sqlx DAL
 */

use super::Dal;

pub struct SqlxDal {}

impl Dal for SqlxDal {
    fn get_securities(&self, currency: Option<String>, agent: Option<String>, 
        mnemonic: Option<String>, exchange: Option<String>) -> Vec<crate::model::Security> {
        todo!("implement");
    }
}

fn test() {
    
}