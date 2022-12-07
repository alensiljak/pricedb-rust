use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::{prelude::ToPrimitive, Decimal};
use serde_json::Value;

use crate::model::{NewPrice, Price, SecuritySymbol};

use super::Downloader;

pub(crate) struct VanguardAuDownloader {
    funds_map: HashMap<&'static str, &'static str>,
}

impl VanguardAuDownloader {
    pub fn new() -> VanguardAuDownloader {
        let funds_map = HashMap::from([
            ("VANGUARD:BOND", "8123"),
            ("VANGUARD:HINT", "8146"),
            ("VANGUARD:PROP", "8147"),
            ("VANGUARD:HY", "8148"),
        ]);

        VanguardAuDownloader { funds_map }
    }

    /// Fetches retail fund prices.
    async fn dl_fund_data(&self) -> Result<String> {
        let url =
            "https://api.vanguard.com/rs/gre/gra/1.7.0/datasets/auw-retail-listview-data.jsonp";

        let response = reqwest::get(url).await?;

        let content = response.text().await?;
        // log::debug!("Vanguard response: {:?}", content);

        // clean-up the response
        let content_json: Value;
        if content.starts_with("callback(") {
            log::trace!("cleaning up callback!");

            let length = content.len() - 1;
            let new_content = &content[9..length];

            content_json = serde_json::from_str(new_content)?;
        } else {
            content_json = serde_json::from_str(content.as_str())?;
        }

        let data = content_json["fundData"].to_string();

        // log::debug!("The thing is {:?}", data);

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

        return fund_info;
    }

    fn parse_price(&self, fund_info: &FundInfo) -> Result<NewPrice> {
        let mut price = Price::for_insert();

        let x = NaiveDate::parse_from_str(fund_info.date.as_str(), "%d %b %Y").unwrap();
        price.date = x.to_string();

        // price.symbol = SecuritySymbol {
        //     namespace: "VANGUARD".to_string(),
        //     mnemonic: symbol.mnemonic,
        // };

        let value_str = fund_info.value.strip_prefix("$").unwrap();
        let value = Decimal::from_str(value_str)?;
        price.value = value.mantissa().to_i32().unwrap();
        price.denom = value.scale() as i32;

        price.currency = "AUD".to_string();

        Ok(price)
    }
}

#[async_trait]
impl Downloader for VanguardAuDownloader {
    async fn download(&self, security_symbol: SecuritySymbol, _currency: &str) -> Result<NewPrice> {
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

#[derive(Debug)]
struct FundInfo {
    name: String,
    identifier: String,
    date: String,
    value: String,
    mstar_id: String,
    // currency: String,
}

impl FundInfo {
    fn new() -> FundInfo {
        FundInfo {
            name: String::default(),
            identifier: String::default(),
            date: String::default(),
            value: String::default(),
            mstar_id: String::default(),
            // currency: String::default(),
        }
    }
}
