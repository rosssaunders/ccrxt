use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

/// Request parameters for getting actual trade fee rate
#[derive(Debug, Serialize, Default)]
pub struct GetActualTradeFeeRateRequest {
    /// Trading pair (e.g. BMX_USDT)
    pub symbol: String,
}

/// Response for actual trade fee rate endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActualTradeFeeRateResponse {
    /// Trading pair
    pub symbol: String,
    /// Taker fee rate (Buy)
    pub buy_taker_fee_rate: String,
    /// Taker fee rate (Sell)
    pub sell_taker_fee_rate: String,
    /// Maker fee rate (Buy)
    pub buy_maker_fee_rate: String,
    /// Maker fee rate (Sell)
    pub sell_maker_fee_rate: String,
}

impl RestClient {
    /// Get actual trade fee rate
    ///
    /// For the actual fee rate of the trading pairs
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Actual trade fee rate information
    pub async fn get_actual_trade_fee_rate(
        &self,
        request: GetActualTradeFeeRateRequest,
    ) -> RestResult<GetActualTradeFeeRateResponse> {
        self.send_request(
            "/spot/v1/trade_fee",
            reqwest::Method::GET,
            Some(&request),
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = GetActualTradeFeeRateRequest {
            symbol: "BTC_USDT".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BTC_USDT"));
        assert!(serialized.contains("symbol"));
    }

    #[test]
    fn test_get_actual_trade_fee_rate_response_structure() {
        let response = GetActualTradeFeeRateResponse {
            symbol: "BTC_USDT".to_string(),
            buy_taker_fee_rate: "0.0008".to_string(),
            sell_taker_fee_rate: "0.0008".to_string(),
            buy_maker_fee_rate: "0.0006".to_string(),
            sell_maker_fee_rate: "0.0006".to_string(),
        };

        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.buy_taker_fee_rate, "0.0008");
        assert_eq!(response.sell_taker_fee_rate, "0.0008");
        assert_eq!(response.buy_maker_fee_rate, "0.0006");
        assert_eq!(response.sell_maker_fee_rate, "0.0006");
    }

    #[test]
    fn test_different_symbol_response() {
        let response = GetActualTradeFeeRateResponse {
            symbol: "ETH_USDT".to_string(),
            buy_taker_fee_rate: "0.001".to_string(),
            sell_taker_fee_rate: "0.001".to_string(),
            buy_maker_fee_rate: "0.0008".to_string(),
            sell_maker_fee_rate: "0.0008".to_string(),
        };

        assert_eq!(response.symbol, "ETH_USDT");
        assert_eq!(response.buy_taker_fee_rate, "0.001");
        assert_eq!(response.sell_taker_fee_rate, "0.001");
        assert_eq!(response.buy_maker_fee_rate, "0.0008");
        assert_eq!(response.sell_maker_fee_rate, "0.0008");
    }

    #[test]
    fn test_trade_fee_rate_serialization_roundtrip() {
        let response = GetActualTradeFeeRateResponse {
            symbol: "BMX_USDT".to_string(),
            buy_taker_fee_rate: "0.0025".to_string(),
            sell_taker_fee_rate: "0.0025".to_string(),
            buy_maker_fee_rate: "0.002".to_string(),
            sell_maker_fee_rate: "0.002".to_string(),
        };

        let serialized = serde_json::to_string(&response).unwrap();
        let deserialized: GetActualTradeFeeRateResponse =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(response.symbol, deserialized.symbol);
        assert_eq!(response.buy_taker_fee_rate, deserialized.buy_taker_fee_rate);
        assert_eq!(
            response.sell_taker_fee_rate,
            deserialized.sell_taker_fee_rate
        );
        assert_eq!(response.buy_maker_fee_rate, deserialized.buy_maker_fee_rate);
        assert_eq!(
            response.sell_maker_fee_rate,
            deserialized.sell_maker_fee_rate
        );
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "symbol": "BTC_USDT",
            "buy_taker_fee_rate": "0.0008",
            "sell_taker_fee_rate": "0.0008",
            "buy_maker_fee_rate": "0.0006",
            "sell_maker_fee_rate": "0.0006"
        }"#;

        let response: GetActualTradeFeeRateResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.symbol, "BTC_USDT");
        assert_eq!(response.buy_taker_fee_rate, "0.0008");
        assert_eq!(response.sell_taker_fee_rate, "0.0008");
        assert_eq!(response.buy_maker_fee_rate, "0.0006");
        assert_eq!(response.sell_maker_fee_rate, "0.0006");
    }

    #[test]
    fn test_request_with_different_symbols() {
        let btc_request = GetActualTradeFeeRateRequest {
            symbol: "BTC_USDT".to_string(),
        };
        let eth_request = GetActualTradeFeeRateRequest {
            symbol: "ETH_USDT".to_string(),
        };
        let bmx_request = GetActualTradeFeeRateRequest {
            symbol: "BMX_USDT".to_string(),
        };

        assert_eq!(btc_request.symbol, "BTC_USDT");
        assert_eq!(eth_request.symbol, "ETH_USDT");
        assert_eq!(bmx_request.symbol, "BMX_USDT");
    }
}
