use serde::{Deserialize, Serialize};

use crate::binance::coinm::{RestResult, public::rest::RestClient};

/// Endpoint path for querying premium index.
const PREMIUM_INDEX_ENDPOINT: &str = "/dapi/v1/premiumIndex";

/// Request parameters for the premium index endpoint.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndexRequest {
    /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
    ///
    /// If provided, filters the result to this symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Trading pair (e.g., "BTCUSD"). Optional.
    ///
    /// If provided, filters the result to this pair.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
}

/// Represents the premium index response for a single symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PremiumIndex {
    /// Trading symbol.
    ///
    /// Example: "BTCUSD_PERP"
    pub symbol: String,

    /// Trading pair.
    ///
    /// Example: "BTCUSD"
    pub pair: String,

    /// Mark price.
    ///
    /// The current mark price for the symbol.
    pub mark_price: String,

    /// Index price.
    ///
    /// The current index price for the symbol.
    pub index_price: String,

    /// Estimated settle price, only useful in the last hour before the settlement starts.
    ///
    /// For perpetual contracts, this is the estimated settle price. For delivery contracts, may be empty.
    pub estimated_settle_price: String,

    /// The latest funding rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    ///
    /// For perpetual contracts, this is the last funding rate. For delivery contracts, will be empty.
    pub last_funding_rate: String,

    /// The base asset interest rate, for perpetual contract symbols only. For delivery symbols, "" will be shown.
    ///
    /// For perpetual contracts, this is the interest rate. For delivery contracts, will be empty.
    pub interest_rate: String,

    /// Next funding time for perpetual contract symbols only. For delivery symbols, 0 will be shown.
    ///
    /// Unix timestamp in milliseconds.
    pub next_funding_time: u64,

    /// Timestamp.
    ///
    /// Unix timestamp in milliseconds.
    pub time: u64,
}

impl RestClient {
    /// Index Price and Mark Price
    ///
    /// Query index price and mark price for coin-margined futures.
    ///
    /// [docs]: https://developers.binance.com/docs/derivatives/coin-margined-futures/market-data/rest-api/Index-Price-and-Mark-Price
    ///
    /// Rate limit: 10
    ///
    /// # Arguments
    /// * `params` - The request parameters for filtering by symbol or pair.
    ///
    /// # Returns
    /// Returns a vector of `PremiumIndex` for each symbol or pair.
    pub async fn get_premium_index(
        &self,
        params: PremiumIndexRequest,
    ) -> RestResult<Vec<PremiumIndex>> {
        let params_opt = if params.symbol.is_some() || params.pair.is_some() {
            Some(params)
        } else {
            None
        };

        self.send_get_request(PREMIUM_INDEX_ENDPOINT, params_opt, 10)
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
