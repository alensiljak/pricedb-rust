use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeZone, FixedOffset};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde_json::Value;

use crate::model::{Price, SecuritySymbol};

use anyhow::{Ok, Result};

use super::Downloader;

/// YahooFinanceDownloader
#[derive(Debug)]
pub struct YahooFinanceDownloader {
    url: String,
    namespaces: HashMap<&'static str, &'static str>,
}

impl YahooFinanceDownloader {
    pub fn new() -> Self {
        let namespaces = HashMap::from([
            ("AMS", "AS"),
            ("ASX", "AX"),
            ("BATS", ""),
            ("BVME", "MI"),
            ("FWB", "F"),
            ("LSE", "L"),
            ("NASDAQ", ""),
            ("NYSE", ""),
            ("NYSEARCA", ""),
            ("XETRA", "DE"),
        ]);

        Self {
            url: "https://query1.finance.yahoo.com/v8/finance/chart/".to_string(),
            namespaces,
        }
    }

    fn assemble_url(&self, symbol: &SecuritySymbol) -> String {
        let current_namespace = symbol.namespace.as_str();
        let mut local_namespace: &str = current_namespace;

        if self.namespaces.contains_key(current_namespace) {
            local_namespace = self.namespaces[current_namespace];
        }

        let mut url = format!("{}{}", self.url, symbol.mnemonic);

        if local_namespace != "" {
            url = format!("{}.{}", url, local_namespace);
        }

        url
    }

    /// Extract the Price from JSON.
    ///
    fn get_price_from_json(&self, body: &Value) -> Result<Price> {
        let chart = &body["chart"];
        let error = &chart["error"];

        // todo: ensure that there is no error!
        //log::debug!("error? {:?}", error);
        assert_eq!(*error, Value::Null);

        let mut result = Price::new();

        let meta = &body["chart"]["result"][0]["meta"];
        assert_ne!(*meta, Value::Null);

        // Price

        let market_price = meta["regularMarketPrice"].as_f64().unwrap();
        // log::debug!("market price {:?}", market_price);
        // Parse using Decimal.
        let d = Decimal::from_f64(market_price).unwrap();
        // log::debug!("Decimal -> {:?} {:?}", d.mantissa(), d.scale());
        result.value = d.mantissa().to_i32().unwrap();
        result.denom = 10_i32.pow(d.scale()); // in 10^3 = 1000, scale=3, denom=1000

        // Currency

        result.currency = meta["currency"].as_str().unwrap().to_string();

        // Date

        let seconds = meta["regularMarketTime"].as_i64().unwrap();
        // log::debug!("seconds {:?}", seconds);
        let offset = meta["gmtoffset"].as_i64().unwrap().to_i32().unwrap();
        let fo = FixedOffset::east_opt(offset).unwrap();

        let utc = NaiveDateTime::from_timestamp_opt(seconds, 0).unwrap();
        // log::debug!("time {:?}", date_time);
        let dt_fo = fo.from_utc_datetime(&utc);

        let date_str = dt_fo.date_naive().to_string();
        // log::debug!("Parsed date is {:?}", date_str);
        result.date = date_str;

        // Time

        let time_str = dt_fo.time().to_string();
        // log::debug!("Parsed time is {:?}", time_str);
        result.time = Some(time_str);

        Ok(result)
    }
}

#[async_trait]
impl Downloader for YahooFinanceDownloader {
    async fn download(&self, security_symbol: SecuritySymbol, _currency: &str) -> Result<Price> {
        let url = self.assemble_url(&security_symbol);

        log::debug!("fetching from {:?}", url);

        let body = reqwest::get(url)
            .await?
            //.text()
            .json::<Value>()
            .await?;

        // log::debug!("something downloaded: {:?}", body);

        let result = self.get_price_from_json(&body)?;

        Ok(result)
    }
}

/// # Tests
#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, TimeZone, FixedOffset};

    use crate::quote::Downloader;
    #[allow(unused_imports)]
    use crate::{model::SecuritySymbol, quote::yahoo_finance_downloader::YahooFinanceDownloader};

    #[test]
    fn test_assemble_url_xetra() {
        let x = YahooFinanceDownloader::new();
        let s = SecuritySymbol {
            namespace: "XETRA".to_string(),
            mnemonic: "EL4X".to_string(),
        };

        let first = x.assemble_url(&s);
        assert_eq!(
            "https://query1.finance.yahoo.com/v8/finance/chart/EL4X.DE",
            first
        );
    }

    #[test]
    fn test_assemble_url_vhyl() {
        let x = YahooFinanceDownloader::new();
        let s = SecuritySymbol {
            namespace: "".to_string(),
            mnemonic: "BND".to_string(),
        };

        let first = x.assemble_url(&s);
        assert_eq!(
            "https://query1.finance.yahoo.com/v8/finance/chart/BND",
            first
        );
    }

    #[test_log::test(tokio::test)]
    async fn test_download() {
        let o = YahooFinanceDownloader::new();
        let symbol = SecuritySymbol {
            namespace: "XETRA".to_string(),
            mnemonic: "EL4X".to_string(),
        };
        let currency = "EUR";

        let result = o.download(symbol, currency).await.expect("Huston?");

        log::debug!("downloaded {:?}", result);

        assert_eq!(result.currency, "EUR");
    }

    /// Download and parse the result for VHYL
    #[test_log::test(tokio::test)]
    async fn test_download_and_parsing_wo_namespace() {
        let o = YahooFinanceDownloader::new();
        let symbol = SecuritySymbol {
            namespace: "".to_string(),
            mnemonic: "BND".to_string(),
        };
        let currency = "USD";

        let result = o.download(symbol, currency).await.expect("Huston?");

        log::debug!("downloaded {:?}", result);

        assert_eq!(result.currency, "USD");
    }

    #[test]
    /// Various options for parsing timestamps.
    fn test_parsing_time() {
        let secs = 1670429622;

        // let time = chrono::offset::Utc::now();

        // let ts_millis = NaiveDateTime::from_timestamp_millis(seconds).unwrap();
        // println!("millis: {:?}", ts_millis);
        
        let ndt_ts_opt = NaiveDateTime::from_timestamp_opt(secs, 0).unwrap();
        // println!("opts: {:?}", ts_opts);
        assert_eq!(ndt_ts_opt.to_string(), "2022-12-07 16:13:42");

        // let ts_opt = Utc.timestamp_opt(seconds, 0);
        // println!("ts_opt {:?}", ts_opt);
        
        // assert_eq!(Utc.timestamp_opt(seconds, 0).unwrap().to_string(), "2015-05-15 00:00:00 UTC");
        // let dt_utc = Utc.timestamp_opt(secs, 0).unwrap();
        //dt_utc.with_timezone(tz);

        // get the offset from json
        let offset = 3600;

        let fo = FixedOffset::east_opt(offset).unwrap();
        println!("offset: {:?}", fo);
        let dt_fo = fo.from_utc_datetime(&ndt_ts_opt);
        // println!("dt fixed offset: {:?}", dt_fo);
        assert_eq!(dt_fo.to_string(), "2022-12-07 17:13:42 +01:00");

        //let dt: DateTime<> = DateTime::from_utc(ndt_ts_opt, fo);

        // let fixed_dt = dt.with_timezone(&FixedOffset::east_opt(9*3600).unwrap());
        //let tz: dyn TimeZone = TimeZone::from_offset(&offset);
        //FixedOffset::from_utc_datetime(&self, &utc);
        //DateTime::with_timezone(&self, tz)

    }
}
