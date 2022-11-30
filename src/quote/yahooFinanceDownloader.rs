use std::collections::HashMap;

use crate::model::{Price, SecuritySymbol};

use super::Downloader;

/// YahooFinanceDownloader
#[derive(Debug)]
pub struct YahooFinanceDownloader {
    url: String,
    namespaces: HashMap<&'static str, &'static str>,
}

impl YahooFinanceDownloader {
    pub fn new() -> YahooFinanceDownloader {
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

        YahooFinanceDownloader {
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
}

impl Downloader for YahooFinanceDownloader {
    fn download(
        &self,
        security_symbol: crate::model::SecuritySymbol,
        currency: &str,
    ) -> crate::model::Price {
        let url = self.assemble_url(&security_symbol);

        todo!("download price here");

        // todo!("parse the price");

        // todo!("replace")
        let result = Price::new();

        result
    }
}

/// # Tests
#[cfg(test)]
mod tests {
    //#[warn(unused_imports)]
    #[allow(unused_imports)]
    use crate::{quote::yahooFinanceDownloader::YahooFinanceDownloader, model::SecuritySymbol};

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
            mnemonic: "VHYL".to_string(),
        };

        let first = x.assemble_url(&s);
        assert_eq!(
            "https://query1.finance.yahoo.com/v8/finance/chart/VHYL",
            first
        );
    }

}
