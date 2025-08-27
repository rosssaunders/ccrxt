use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, InstrumentType, RestResult, public_client::RestClient};

/// Endpoint URL for getting a specific block ticker
const GET_BLOCK_TICKER_ENDPOINT: &str = "api/v5/market/block-ticker";

/// Request parameters for getting a specific block ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockTickerRequest {
    /// Instrument ID (e.g., "BTC-USD-SWAP")
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// Single block ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleBlockTicker {
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
    /// Get block ticker
    ///
    /// Retrieve the latest block trading volume in the last 24 hours for a specific instrument.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#block-trading-rest-api-get-block-ticker)
    ///
    /// Rate limit: 20 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The block ticker request parameters
    ///
    /// # Returns
    /// Response containing block ticker data for the specified instrument
    pub async fn get_block_ticker(
        &self,
        request: GetBlockTickerRequest,
    ) -> RestResult<SingleBlockTicker> {
        self.send_get_request(
            GET_BLOCK_TICKER_ENDPOINT,
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
    fn test_get_block_ticker_request_serialization() {
        let request = GetBlockTickerRequest {
            inst_id: "BTC-USD-SWAP".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instId").and_then(|v| v.as_str()),
            Some("BTC-USD-SWAP")
        );
    }

    #[test]
    fn test_block_ticker_deserialization() {
        let ticker_json = json!({
            "instId": "BTC-USD-SWAP",
            "instType": "SWAP",
            "volCcy24h": "987654.32",
            "vol24h": "100.25",
            "ts": "1597026383085"
        });

        let ticker: SingleBlockTicker = serde_json::from_value(ticker_json).unwrap();
        assert_eq!(ticker.inst_id, "BTC-USD-SWAP");
        assert_eq!(ticker.inst_type, InstrumentType::Swap);
        assert_eq!(ticker.vol_ccy_24h, "987654.32");
        assert_eq!(ticker.vol_24h, "100.25");
        assert_eq!(ticker.ts, "1597026383085");
    }

    #[test]
    fn test_get_block_ticker_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USD-SWAP",
                    "instType": "SWAP",
                    "volCcy24h": "987654.32",
                    "vol24h": "100.25",
                    "ts": "1597026383085"
                }
            ]
        });

        let response: ApiResponse<SingleBlockTicker> =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data.first().unwrap().inst_id, "BTC-USD-SWAP");
        assert_eq!(
            response.data.first().unwrap().inst_type,
            InstrumentType::Swap
        );
    }

    #[test]
    fn test_block_ticker_serialization_roundtrip() {
        let original = SingleBlockTicker {
            inst_id: "ETH-USDT".to_string(),
            inst_type: InstrumentType::Spot,
            vol_ccy_24h: "123456.78".to_string(),
            vol_24h: "45.67".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: SingleBlockTicker = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.vol_ccy_24h, deserialized.vol_ccy_24h);
        assert_eq!(original.vol_24h, deserialized.vol_24h);
        assert_eq!(original.ts, deserialized.ts);
    }

    #[test]
    fn test_get_block_ticker_request_roundtrip() {
        let original = GetBlockTickerRequest {
            inst_id: "BTC-USDT".to_string(),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetBlockTickerRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_id, deserialized.inst_id);
    }
}
