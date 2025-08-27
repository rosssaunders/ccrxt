use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const OPEN_INTEREST_AND_VOLUME_STRIKE_ENDPOINT: &str =
    "/api/v5/rubik/stat/option/open-interest-volume-strike";

/// Request parameters for the get open interest and volume (strike) request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOpenInterestAndVolumeStrikeRequest {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,

    /// Contract expiry date, the format is YYYYMMdd, e.g. 20210623
    #[serde(rename = "expTime")]
    pub exp_time: String,

    /// Period, the default is 8H. e.g. [8H/1D]
    /// Each granularity can provide only one latest piece of data
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Response data for the get open interest and volume (strike) request
/// The return value array order is: [ts,strike,callOI,putOI,callVol,putVol]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenInterestAndVolumeStrikeData {
    /// Timestamp
    #[serde(rename = "ts")]
    pub ts: String,

    /// Strike price
    #[serde(rename = "strike")]
    pub strike: String,

    /// Total call open interest (coin as the unit)
    #[serde(rename = "callOI")]
    pub call_oi: String,

    /// Total put open interest (coin as the unit)
    #[serde(rename = "putOI")]
    pub put_oi: String,

    /// Total call trading volume (coin as the unit)
    #[serde(rename = "callVol")]
    pub call_vol: String,

    /// Total put trading volume (coin as the unit)
    #[serde(rename = "putVol")]
    pub put_vol: String,
}

impl RestClient {
    /// Get open interest and volume (strike)
    ///
    /// Retrieve the taker volume for both buyers and sellers of calls and puts.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-statistics-rest-api-get-open-interest-and-volume-strike)
    pub async fn get_open_interest_and_volume_strike(
        &self,
        request: GetOpenInterestAndVolumeStrikeRequest,
    ) -> RestResult<OpenInterestAndVolumeStrikeData> {
        self.send_get_request(
            OPEN_INTEREST_AND_VOLUME_STRIKE_ENDPOINT,
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
    fn test_get_open_interest_and_volume_strike_request_serialization() {
        let request = GetOpenInterestAndVolumeStrikeRequest {
            ccy: "BTC".to_string(),
            exp_time: "20210623".to_string(),
            period: Some("8H".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetOpenInterestAndVolumeStrikeRequest =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_open_interest_and_volume_strike_data_deserialization_from_api() {
        let json_response = r#"{
            "ts": "1597026383085",
            "strike": "45000",
            "callOI": "123.45",
            "putOI": "234.56",
            "callVol": "345.67",
            "putVol": "456.78"
        }"#;

        let data: OpenInterestAndVolumeStrikeData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ts, "1597026383085");
        assert_eq!(data.strike, "45000");
        assert_eq!(data.call_oi, "123.45");
        assert_eq!(data.put_oi, "234.56");
        assert_eq!(data.call_vol, "345.67");
        assert_eq!(data.put_vol, "456.78");
    }

    #[test]
    fn test_open_interest_and_volume_strike_array_format() {
        // Test the array format mentioned in docs: [ts,strike,callOI,putOI,callVol,putVol]
        let json_array = r#"["1597026383085", "45000", "123.45", "234.56", "345.67", "456.78"]"#;
        let array_data: Vec<String> = serde_json::from_str(json_array).unwrap();

        assert_eq!(array_data.len(), 6);
        assert_eq!(array_data[0], "1597026383085");
        assert_eq!(array_data[1], "45000");
        assert_eq!(array_data[2], "123.45");
        assert_eq!(array_data[3], "234.56");
        assert_eq!(array_data[4], "345.67");
        assert_eq!(array_data[5], "456.78");
    }
}
