use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, InstrumentType, RestResult, public_client::RestClient};

/// Endpoint URL for getting block tickers
const GET_BLOCK_TICKERS_ENDPOINT: &str = "api/v5/market/block-tickers";

/// Request parameters for getting block tickers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockTickersRequest {
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// Instrument family (e.g., "BTC-USD")
    /// Applicable to FUTURES/SWAP/OPTION
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Block ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTicker {
    /// Instrument ID
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    /// 24h trading volume, with a unit of currency.
    /// If it is a derivatives contract, the value is the number of base currency.
    /// If it is SPOT/MARGIN, the value is the quantity in quote currency.
    #[serde(rename = "volCcy24h")]
    pub vol_ccy_24h: String,

    /// 24h trading volume, with a unit of contract.
    /// If it is a derivatives contract, the value is the number of contracts.
    /// If it is SPOT/MARGIN, the value is the quantity in base currency.
    #[serde(rename = "vol24h")]
    pub vol_24h: String,

    /// Block ticker data generation time
    /// Unix timestamp format in milliseconds, e.g. "1597026383085"
    pub ts: String,
}

impl RestClient {
    /// Get block tickers
    ///
    /// Retrieve the latest block trading volume in the last 24 hours.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-block-tickers)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The block tickers request parameters
    ///
    /// # Returns
    /// Response containing block ticker data for the specified instrument type
    pub async fn get_block_tickers(
        &self,
        request: GetBlockTickersRequest,
    ) -> RestResult<BlockTicker> {
        self.send_get_request(
            GET_BLOCK_TICKERS_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_block_tickers_request_serialization() {
        let request = GetBlockTickersRequest {
            inst_type: InstrumentType::Spot,
            inst_family: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("SPOT")
        );
        assert!(serialized.get("instFamily").is_none());
    }

    #[test]
    fn test_get_block_tickers_request_with_family() {
        let request = GetBlockTickersRequest {
            inst_type: InstrumentType::Futures,
            inst_family: Some("BTC-USD".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
    }

    #[test]
    fn test_block_ticker_deserialization() {
        let ticker_json = json!({
            "instId": "BTC-USDT",
            "instType": "SPOT",
            "volCcy24h": "123456.78",
            "vol24h": "2.5",
            "ts": "1597026383085"
        });

        let ticker: BlockTicker = serde_json::from_value(ticker_json).unwrap();
        assert_eq!(ticker.inst_id, "BTC-USDT");
        assert_eq!(ticker.inst_type, InstrumentType::Spot);
        assert_eq!(ticker.vol_ccy_24h, "123456.78");
        assert_eq!(ticker.vol_24h, "2.5");
        assert_eq!(ticker.ts, "1597026383085");
    }

    #[test]
    fn test_get_block_tickers_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "instType": "SPOT",
                    "volCcy24h": "123456.78",
                    "vol24h": "2.5",
                    "ts": "1597026383085"
                },
                {
                    "instId": "ETH-USDT",
                    "instType": "SPOT",
                    "volCcy24h": "654321.12",
                    "vol24h": "15.8",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: ApiResponse<BlockTicker> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USDT");
        assert_eq!(response.data.get(1).unwrap().inst_id, "ETH-USDT");
    }

    #[test]
    fn test_block_ticker_serialization_roundtrip() {
        let original = BlockTicker {
            inst_id: "BTC-USD-SWAP".to_string(),
            inst_type: InstrumentType::Swap,
            vol_ccy_24h: "987654.32".to_string(),
            vol_24h: "100.25".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: BlockTicker = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.vol_ccy_24h, deserialized.vol_ccy_24h);
        assert_eq!(original.vol_24h, deserialized.vol_24h);
        assert_eq!(original.ts, deserialized.ts);
    }
}
