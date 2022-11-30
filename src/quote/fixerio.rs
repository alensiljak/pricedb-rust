use anyhow::Result;
use async_trait::async_trait;

/// Fixerio downloader
use crate::model::{SecuritySymbol, Price};

use super::Downloader;

pub struct Fixerio {}

impl Fixerio {
    pub fn new() -> Fixerio {
        Fixerio {  }
    }
}

#[async_trait]
impl Downloader for Fixerio {
    async fn download(&self, security_symbol: SecuritySymbol, currency: &str) -> Result<Price> {
        todo!()
    }
}