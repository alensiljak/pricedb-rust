/**
 * Vanguard AU price downloader.
 * Deprecated as of 2023-04.
 */
use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde_json::Value;

use crate::model::{Price, SecuritySymbol};

use super::Downloader;

pub(crate) struct VanguardAuDownloader {
    funds_map: HashMap<&'static str, &'static str>,
}

impl VanguardAuDownloader {
    pub fn new() -> Self {
        let funds_map = HashMap::from([
            ("VANGUARD:BOND", "8123"),
            ("VANGUARD:HINT", "8146"),
            ("VANGUARD:PROP", "8147"),
            ("VANGUARD:HY", "8148"),
        ]);

        Self { funds_map }
    }

    /// Fetches retail fund prices.
    async fn dl_fund_data(&self) -> Result<String> {
        let url =
            "https://api.vanguard.com/rs/gre/gra/1.7.0/datasets/auw-retail-listview-data.jsonp";

        let response = reqwest::get(url).await?;
        let content = response.text().await?;

        // clean-up the response
        let content_json: Value = if content.starts_with("callback(") {
            log::trace!("cleaning up callback!");

            let length = content.len() - 1;
            let new_content = &content[9..length];

            serde_json::from_str(new_content)?
        } else {
            serde_json::from_str(content.as_str())?
        };

        let data = content_json["fundData"].to_string();

        Ok(data)
    }

    /// Extracts the price value from json response.
    ///
    /// Returns the Price object with name, identifier, date, value, mstar_id.
    fn get_fund_info(&self, fund_data: Value, fund_id: &str) -> FundInfo {
        let info_json = &fund_data[fund_id];
        // log::debug!("info_json: {:?}", info_json);

        let mut fund_info = FundInfo::new();

        fund_info.name = info_json["name"].as_str().unwrap().to_string();
        fund_info.identifier = info_json["identifier"].as_str().unwrap().to_string();
        fund_info.date = info_json["asOfDate"].as_str().unwrap().to_string();
        // Using NAV price for the value.
        fund_info.value = info_json["navPrice"].as_str().unwrap().to_string();
        fund_info.mstar_id = info_json["mStarID"].as_str().unwrap().to_string();
        // Currency?
        // fund_info.currency = info_json["currency"]["currencyCode"].as_str().unwrap().to_string();

        fund_info
    }

    fn parse_price(&self, fund_info: &FundInfo) -> Result<Price> {
        let mut price = Price::new();

        let x = NaiveDate::parse_from_str(fund_info.date.as_str(), "%d %b %Y").unwrap();
        price.date = x.to_string();

        // price.symbol = SecuritySymbol {
        //     namespace: "VANGUARD".to_string(),
        //     mnemonic: symbol.mnemonic,
        // };

        let value_str = fund_info.value.strip_prefix('$').unwrap();
        let value = Decimal::from_str(value_str)?;
        price.value = value.mantissa().to_i64().unwrap();
        price.denom = 10_i64.pow(value.scale()); // in 10^3 = 1000, scale=3, denom=1000

        price.currency = "AUD".to_string();

        Ok(price)
    }
}

#[async_trait]
impl Downloader for VanguardAuDownloader {
    async fn download(&self, security_symbol: &SecuritySymbol, _currency: &str) -> Result<Price> {
        if security_symbol
            .namespace
            .ne("VANGUARD".to_uppercase().as_str())
        {
            panic!("Only Vanguard symbols are handled by this downloader!");
        }

        let fund_data_str = self.dl_fund_data().await?;
        let fund_data: Value = serde_json::from_str(fund_data_str.as_str())?;

        // log::debug!("fund data = {:?}", fund_data);

        let symbol_display = security_symbol.to_string();
        log::debug!("symbol display: {:?}", symbol_display);

        let fund_id = self.funds_map[symbol_display.as_str()];
        log::debug!("fund id = {:?}", fund_id);

        let fund_info = self.get_fund_info(fund_data, fund_id);
        log::debug!("fund info: {:?}", fund_info);

        let new_price = self.parse_price(&fund_info)?;

        log::debug!("Parsed price: {:?}", new_price);

        Ok(new_price)
    }
}

#[derive(Debug, Default)]
pub(crate) struct FundInfo {
    pub name: String,
    pub identifier: String,
    pub date: String,
    pub value: String,
    pub mstar_id: String,
    // currency: String,
}

impl FundInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    // use crate::{quote::Downloader, model::SecuritySymbol};

    // use super::VanguardAuDownloader;

    // Dev debug test. Uncomment to execute.
    // #[tokio::test]
    // async fn test_hy_price_dl() {
    //     let dl = VanguardAuDownloader::new();
    //     let symbol = SecuritySymbol::new("VANGUARD:HY");

    //     let actual = dl.download(&symbol, "AUD").await.expect("downloaded price");

    //     assert!(!actual.currency.is_empty());
    //     assert!(actual.value != 0);
    // }
}