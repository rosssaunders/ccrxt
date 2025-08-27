use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const TAKER_FLOW_ENDPOINT: &str = "/api/v5/rubik/stat/option/taker-block-volume";

/// Request parameters for the get taker flow request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTakerFlowRequest {
    /// currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can provide only one latest piece of data
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get taker flow request
/// The return value array order is: [ts,callBuyVol,callSellVol,putBuyVol,putSellVol,callBlockVol,putBlockVol]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakerFlowData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// call option buy volume, in settlement currency
    #[serde(rename = "callBuyVol")]
    pub call_buy_vol: String,

    /// call option sell volume, in settlement currency
    #[serde(rename = "callSellVol")]
    pub call_sell_vol: String,

    /// put option buy volume, in settlement currency
    #[serde(rename = "putBuyVol")]
    pub put_buy_vol: String,

    /// put option sell volume, in settlement currency
    #[serde(rename = "putSellVol")]
    pub put_sell_vol: String,

    /// call block volume
    #[serde(rename = "callBlockVol")]
    pub call_block_vol: String,

    /// put block volume
    #[serde(rename = "putBlockVol")]
    pub put_block_vol: String,
}

impl RestClient {
    /// Get taker flow
    ///
    /// This shows the relative buy/sell volume for calls and puts. It shows whether traders are bullish or bearish on price and volatility.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-taker-flow)
    pub async fn get_taker_flow(&self, request: GetTakerFlowRequest) -> RestResult<TakerFlowData> {
        self.send_get_request(
            TAKER_FLOW_ENDPOINT,
            Some(&request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_taker_flow_request_serialization() {
        let request = GetTakerFlowRequest {
            ccy: "BTC".to_string(),
            period: Some("8H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetTakerFlowRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_taker_flow_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "callBuyVol": "123.45",
            "callSellVol": "234.56",
            "putBuyVol": "345.67",
            "putSellVol": "456.78",
            "callBlockVol": "567.89",
            "putBlockVol": "678.90"
        }"#;

        let data: TakerFlowData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.call_buy_vol, "123.45");
        assert_eq!(data.call_sell_vol, "234.56");
        assert_eq!(data.put_buy_vol, "345.67");
        assert_eq!(data.put_sell_vol, "456.78");
        assert_eq!(data.call_block_vol, "567.89");
        assert_eq!(data.put_block_vol, "678.90");
    }

    #[test]
    fn test_taker_flow_array_format() {
        // Test the array format mentioned in docs: [ts,callBuyVol,callSellVol,putBuyVol,putSellVol,callBlockVol,putBlockVol]
        let json_array =
            r#"["1597026383085", "123.45", "234.56", "345.67", "456.78", "567.89", "678.90"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 7);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "123.45");
        assert_eq!(array_data[2], "234.56");
        assert_eq!(array_data[3], "345.67");
        assert_eq!(array_data[4], "456.78");
        assert_eq!(array_data[5], "567.89");
        assert_eq!(array_data[6], "678.90");
    }
}
