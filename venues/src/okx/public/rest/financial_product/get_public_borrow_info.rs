use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, public_client::RestClient};

const PUBLIC_BORROW_INFO_ENDPOINT: &str = "/api/v5/finance/savings/lending-rate-summary";

/// Request parameters for public borrow info
#[derive(Debug, Clone, Serialize)]
pub struct GetPublicBorrowInfoRequest {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Response data for public borrow info
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicBorrowInfoData {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// 24H average borrowing amount (deprecated)
    #[serde(rename = "avgAmt")]
    pub avg_amt: String,

    /// 24H average borrowing amount in USD value (deprecated)
    #[serde(rename = "avgAmtUsd")]
    pub avg_amt_usd: String,

    /// 24H average lending rate
    #[serde(rename = "avgRate")]
    pub avg_rate: String,

    /// Last annual interest rate
    #[serde(rename = "preRate")]
    pub pre_rate: String,

    /// Next estimate annual interest rate
    #[serde(rename = "estRate")]
    pub est_rate: String,
}

impl RestClient {
    /// Get public borrow info
    ///
    /// Public endpoint that retrieves lending rate summary information.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-simple-earn-flexible-get-public-borrow-info-public)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: IP
    ///
    /// # Arguments
    /// * `request` - Request parameters including optional currency filter
    ///
    /// # Returns
    /// A vector of public borrow info data
    pub async fn get_public_borrow_info(
        &self,
        request: GetPublicBorrowInfoRequest,
    ) -> RestResult<Vec<PublicBorrowInfoData>> {
        self.send_get_request(
            PUBLIC_BORROW_INFO_ENDPOINT,
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
    fn test_get_public_borrow_info_request_serialization() {
        let request = GetPublicBorrowInfoRequest {
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_get_public_borrow_info_request_no_currency() {
        let request = GetPublicBorrowInfoRequest { ccy: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_public_borrow_info_data_serialization() {
        let data = PublicBorrowInfoData {
            ccy: "BTC".to_string(),
            avg_amt: "100.5".to_string(),
            avg_amt_usd: "5000000.0".to_string(),
            avg_rate: "0.05".to_string(),
            pre_rate: "0.048".to_string(),
            est_rate: "0.052".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: PublicBorrowInfoData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_public_borrow_info_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "ccy": "BTC",
                "avgAmt": "120.5",
                "avgAmtUsd": "6000000.0",
                "avgRate": "0.048",
                "preRate": "0.045",
                "estRate": "0.050"
            },
            {
                "ccy": "ETH",
                "avgAmt": "2500.0",
                "avgAmtUsd": "4000000.0",
                "avgRate": "0.035",
                "preRate": "0.032",
                "estRate": "0.038"
            }
        ]"#;

        let data: Vec<PublicBorrowInfoData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].ccy, "BTC");
        assert_eq!(data[0].avg_rate, "0.048");
        assert_eq!(data[1].ccy, "ETH");
        assert_eq!(data[1].avg_rate, "0.035");
    }

    #[test]
    fn test_public_borrow_info_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<PublicBorrowInfoData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }
}
