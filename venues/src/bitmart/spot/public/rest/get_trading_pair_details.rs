use serde::{Deserialize, Serialize};

use super::client::RestClient;

use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

const TRADING_PAIR_DETAILS_ENDPOINT: &str = "/spot/v1/symbols/details";

/// Request parameters for getting trading pair details
#[derive(Debug, Serialize, Default)]
pub struct GetTradingPairDetailsRequest {
    // No parameters needed for this endpoint
}

/// Trading pair details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPairDetail {
    /// Trading pair name
    pub symbol: String,
    /// Trading pair id
    pub symbol_id: i64,
    /// Base currency
    pub base_currency: String,
    /// Quote currency
    pub quote_currency: String,
    /// The minimum order quantity is also the minimum order quantity increment
    pub quote_increment: String,
    /// Minimum order quantity
    pub base_min_size: String,
    /// Minimum price accuracy (decimal places), used to query k-line and depth
    pub price_min_precision: i32,
    /// Maximum price accuracy (decimal places), used to query k-line and depth
    pub price_max_precision: i32,
    /// Expiration time of trading pair
    pub expiration: String,
    /// Minimum order amount
    pub min_buy_amount: String,
    /// Minimum sell amount
    pub min_sell_amount: String,
    /// Trade Status
    /// - `trading` = is trading
    /// - `pre-trade` = pre-open
    pub trade_status: String,
}

/// Response for trading pair details endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradingPairDetailsResponse {
    /// Array of trading pair details
    pub symbols: Vec<TradingPairDetail>,
}

impl RestClient {
    /// Get Trading Pair Details (V1)
    ///
    /// Get a detailed list of all trading pairs on the platform
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/public_market_data.md
    ///
    /// Rate limit: 10 times/2sec per IP
    ///
    /// # Returns
    /// Detailed list of all trading pairs on the platform
    pub async fn get_trading_pair_details(
        &self,
        _request: GetTradingPairDetailsRequest,
    ) -> RestResult<GetTradingPairDetailsResponse> {
        self.send_request(
            TRADING_PAIR_DETAILS_ENDPOINT,
            reqwest::Method::GET,
            Option::<&()>::None, // No query parameters
            EndpointType::SpotPublicMarket,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trading_pair_details_request_default() {
        let request = GetTradingPairDetailsRequest::default();
        // Request has no fields to check
        let _ = request;
    }

    #[test]
    fn test_trading_pair_detail_structure() {
        let detail = TradingPairDetail {
            symbol: "GXC_BTC".to_string(),
            symbol_id: 1024,
            base_currency: "GXC".to_string(),
            quote_currency: "BTC".to_string(),
            quote_increment: "1.00000000".to_string(),
            base_min_size: "1.00000000".to_string(),
            price_min_precision: 6,
            price_max_precision: 8,
            expiration: "NA".to_string(),
            min_buy_amount: "0.00010000".to_string(),
            min_sell_amount: "0.00010000".to_string(),
            trade_status: "trading".to_string(),
        };

        assert_eq!(detail.symbol, "GXC_BTC");
        assert_eq!(detail.symbol_id, 1024);
        assert_eq!(detail.base_currency, "GXC");
        assert_eq!(detail.quote_currency, "BTC");
        assert_eq!(detail.quote_increment, "1.00000000");
        assert_eq!(detail.base_min_size, "1.00000000");
        assert_eq!(detail.price_min_precision, 6);
        assert_eq!(detail.price_max_precision, 8);
        assert_eq!(detail.expiration, "NA");
        assert_eq!(detail.min_buy_amount, "0.00010000");
        assert_eq!(detail.min_sell_amount, "0.00010000");
        assert_eq!(detail.trade_status, "trading");
    }

    #[test]
    fn test_trading_pair_detail_serialization_roundtrip() {
        let detail = TradingPairDetail {
            symbol: "BTC_USDT".to_string(),
            symbol_id: 1,
            base_currency: "BTC".to_string(),
            quote_currency: "USDT".to_string(),
            quote_increment: "0.01".to_string(),
            base_min_size: "0.00001".to_string(),
            price_min_precision: 2,
            price_max_precision: 8,
            expiration: "NA".to_string(),
            min_buy_amount: "1.00".to_string(),
            min_sell_amount: "1.00".to_string(),
            trade_status: "trading".to_string(),
        };

        let serialized = serde_json::to_string(&detail).unwrap();
        let deserialized: TradingPairDetail = serde_json::from_str(&serialized).unwrap();

        assert_eq!(detail.symbol, deserialized.symbol);
        assert_eq!(detail.symbol_id, deserialized.symbol_id);
        assert_eq!(detail.base_currency, deserialized.base_currency);
        assert_eq!(detail.quote_currency, deserialized.quote_currency);
        assert_eq!(detail.quote_increment, deserialized.quote_increment);
        assert_eq!(detail.base_min_size, deserialized.base_min_size);
        assert_eq!(detail.price_min_precision, deserialized.price_min_precision);
        assert_eq!(detail.price_max_precision, deserialized.price_max_precision);
        assert_eq!(detail.expiration, deserialized.expiration);
        assert_eq!(detail.min_buy_amount, deserialized.min_buy_amount);
        assert_eq!(detail.min_sell_amount, deserialized.min_sell_amount);
        assert_eq!(detail.trade_status, deserialized.trade_status);
    }

    #[test]
    fn test_get_trading_pair_details_response_structure() {
        let response = GetTradingPairDetailsResponse {
            symbols: vec![TradingPairDetail {
                symbol: "GXC_BTC".to_string(),
                symbol_id: 1024,
                base_currency: "GXC".to_string(),
                quote_currency: "BTC".to_string(),
                quote_increment: "1.00000000".to_string(),
                base_min_size: "1.00000000".to_string(),
                price_min_precision: 6,
                price_max_precision: 8,
                expiration: "NA".to_string(),
                min_buy_amount: "0.00010000".to_string(),
                min_sell_amount: "0.00010000".to_string(),
                trade_status: "trading".to_string(),
            }],
        };

        assert_eq!(response.symbols.len(), 1);
        assert_eq!(response.symbols[0].symbol, "GXC_BTC");
        assert_eq!(response.symbols[0].symbol_id, 1024);
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "symbols": [
                {
                    "symbol": "GXC_BTC",
                    "symbol_id": 1024,
                    "base_currency": "GXC",
                    "quote_currency": "BTC",
                    "quote_increment": "1.00000000",
                    "base_min_size": "1.00000000",
                    "price_min_precision": 6,
                    "price_max_precision": 8,
                    "expiration": "NA",
                    "min_buy_amount": "0.00010000",
                    "min_sell_amount": "0.00010000",
                    "trade_status": "trading"
                }
            ]
        }"#;

        let response: GetTradingPairDetailsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbols.len(), 1);
        assert_eq!(response.symbols[0].symbol, "GXC_BTC");
        assert_eq!(response.symbols[0].symbol_id, 1024);
        assert_eq!(response.symbols[0].base_currency, "GXC");
        assert_eq!(response.symbols[0].quote_currency, "BTC");
        assert_eq!(response.symbols[0].trade_status, "trading");
    }
}
