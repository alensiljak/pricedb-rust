///
/// Vanguard AU price downloader using the detail data.
/// Valid as of 2023-05.
///
use super::Downloader;
use crate::model::{Price, SecuritySymbol};
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};

pub(crate) struct VanguardAu3Downloader {
    funds_map: HashMap<&'static str, &'static str>,
}

impl VanguardAu3Downloader {
    pub fn new() -> Self {
        let funds_map = HashMap::from([
            ("VANGUARD:BOND", "8123"),
            ("VANGUARD:HINT", "8146"),
            ("VANGUARD:PROP", "8147"),
            ("VANGUARD:HY", "8148"),
        ]);

        Self { funds_map }
    }

    fn get_url(&self, symbol: &SecuritySymbol) -> String {
        let sec_symbol = symbol.to_string();
        let fund_id = self.funds_map.get(sec_symbol.as_str()).unwrap();
        let result = format!(
            "https://www.vanguard.com.au/personal/api/products/personal/fund/{}/detail?limit=-1",
            fund_id
        );

        // log::debug!("url: {:?}", result);

        result
    }

    /// Returns the latest retail fund price.
    /// (date, price, currency)
    async fn dl_price(&self, symbol: &SecuritySymbol) -> Result<(String, String, String)> {
        let url = self.get_url(symbol);

        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        // Parse
        let content_json: Value = serde_json::from_str(content.as_str())?;
        let data = &content_json["data"][0];

        let prices = &data["navPrices"];
        let latest = &prices[0];

        let date = latest["asOfDate"].to_string().replace("\"", "");
        let price = latest["price"].to_string();
        let currency = latest["currencyCode"].to_string().replace("\"", "");

        Ok((date, price, currency))
    }

    fn parse_price(&self, date: String, price: String, currency: String) -> Result<Price> {
        let mut p = Price::new();

        let x = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
        p.date = x.to_string();

        let value = Decimal::from_str(&price)?;
        p.value = value.mantissa().to_i64().unwrap();
        p.denom = 10_i64.pow(value.scale()); // in 10^3 = 1000, scale=3, denom=1000

        p.currency = currency;

        Ok(p)
    }
}

#[async_trait]
impl Downloader for VanguardAu3Downloader {
    async fn download(&self, security_symbol: &SecuritySymbol, _currency: &str) -> Result<Price> {
        if security_symbol
            .namespace
            .ne("VANGUARD".to_uppercase().as_str())
        {
            panic!("Only Vanguard symbols are handled by this downloader!");
        }

        let (date, price, currency) = self.dl_price(security_symbol).await?;

        let price = self.parse_price(date, price, currency)?;

        Ok(price)
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::SecuritySymbol, quote::Downloader};

    use super::VanguardAu3Downloader;

    #[test]
    fn test_url_generation() {
        let symbol = SecuritySymbol::new("VANGUARD:HY");
        let dl = VanguardAu3Downloader::new();

        let actual = dl.get_url(&symbol);

        assert_eq!(
            "https://www.vanguard.com.au/personal/api/products/personal/fund/8148/prices?limit=-1",
            actual
        );
    }

    /// Dev debug test. Uncomment to execute.
    // #[tokio::test]
    async fn test_hy_price_dl() {
        let dl = VanguardAu3Downloader::new();
        let symbol = SecuritySymbol::new("VANGUARD:HY");

        let actual = dl.download(&symbol, "AUD").await.expect("downloaded price");

        // currency
        assert!(!actual.currency.is_empty());
        assert_eq!("AUD", actual.currency);
        // value
        assert!(actual.value != 0);
        // date
        assert!(!actual.date.is_empty());
    }
}
