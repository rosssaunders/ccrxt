use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Request parameters for the premium index endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PremiumIndexRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    #[serde(rename = "pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Represents the premium index response for a single symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    /// Trading symbol.
    pub symbol: String,

    /// Trading pair.
    pub pair: String,

    /// Mark price.
    pub mark_price: String,

    /// Index price.
    pub index_price: String,

    /// Estimated settle price, only useful in the last hour before the settlement starts.
    pub estimated_settle_price: String,

    /// The latest funding rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub last_funding_rate: String,

    /// The base asset interest rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub interest_rate: String,

    /// Next funding time for perpetual contract symbols only. For delivery symbols, "" will be shown.
    pub next_funding_time: u64,

    /// Timestamp.
    pub time: u64,
}

impl RestClient {
    /// Query index price and mark price.
    ///
    /// [Official API docs](https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-and-Mark-Price)
    ///
    /// Weight: 10
    pub async fn get_premium_index(
        &self,
        params: PremiumIndexRequest,
    ) -> RestResult<Vec<PremiumIndex>> {
        let params_opt = if params.symbol.is_some() || params.pair.is_some() {
            Some(params)
        } else {
            None
        };

        self.send_request(
            "/dapi/v1/premiumIndex",
            reqwest::Method::GET,
            params_opt,
            10,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_premium_index_request_serialization_with_symbol() {
        let request = PremiumIndexRequest {
            symbol: Some("BTCUSD_PERP".to_string()),
            pair: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "symbol=BTCUSD_PERP");
    }

    #[test]
    fn test_premium_index_request_serialization_with_pair() {
        let request = PremiumIndexRequest {
            symbol: None,
            pair: Some("BTCUSD".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "pair=BTCUSD");
    }

    #[test]
    fn test_premium_index_request_serialization_empty() {
        let request = PremiumIndexRequest {
            symbol: None,
            pair: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_premium_index_response_deserialization() {
        let json = r#"[{
            "symbol": "BTCUSD_PERP",
            "pair": "BTCUSD",
            "markPrice": "45000.50000000",
            "indexPrice": "45001.00000000",
            "estimatedSettlePrice": "45000.75000000",
            "lastFundingRate": "0.00010000",
            "interestRate": "0.00005000",
            "nextFundingTime": 1625097600000,
            "time": 1625097600000
        }]"#;

        let response: Vec<PremiumIndex> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        
        let item = &response[0];
        assert_eq!(item.symbol, "BTCUSD_PERP");
        assert_eq!(item.pair, "BTCUSD");
        assert_eq!(item.mark_price, "45000.50000000");
        assert_eq!(item.index_price, "45001.00000000");
        assert_eq!(item.estimated_settle_price, "45000.75000000");
        assert_eq!(item.last_funding_rate, "0.00010000");
        assert_eq!(item.interest_rate, "0.00005000");
        assert_eq!(item.next_funding_time, 1625097600000);
        assert_eq!(item.time, 1625097600000);
    }

    #[test]
    fn test_premium_index_response_delivery_contract() {
        let json = r#"[{
            "symbol": "BTCUSD_240329",
            "pair": "BTCUSD",
            "markPrice": "45000.50000000",
            "indexPrice": "45001.00000000",
            "estimatedSettlePrice": "45000.75000000",
            "lastFundingRate": "",
            "interestRate": "",
            "nextFundingTime": 0,
            "time": 1625097600000
        }]"#;

        let response: Vec<PremiumIndex> = serde_json::from_str(json).unwrap();
        assert_eq!(response.len(), 1);
        
        let item = &response[0];
        assert_eq!(item.symbol, "BTCUSD_240329");
        assert_eq!(item.last_funding_rate, "");
        assert_eq!(item.interest_rate, "");
        assert_eq!(item.next_funding_time, 0);
    }
}
