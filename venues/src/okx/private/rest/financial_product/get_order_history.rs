use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const ORDER_HISTORY_ENDPOINT: &str = "/api/v5/finance/staking-defi/orders-history";

/// Request parameters for getting order history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    /// Product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    /// Protocol type
    /// defi: on-chain earn
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_type: Option<String>,

    /// Investment currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Pagination of data to return records earlier than the requested ID.
    /// The value passed is the corresponding ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested ID.
    /// The value passed is the corresponding ordId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Number of results per request. The default is 100. The maximum is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Investment data within order history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryInvestData {
    /// Investment currency, e.g. BTC
    pub ccy: String,

    /// Invested amount
    pub amt: String,
}

/// Earning data within order history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEarningData {
    /// Earning currency, e.g. BTC
    pub ccy: String,

    /// Earning type
    /// 0: Estimated earning, 1: Cumulative earning
    #[serde(rename = "earningType")]
    pub earning_type: String,

    /// Cumulative earning of redeemed orders
    /// This field is just valid when the order is in redemption state
    #[serde(rename = "realizedEarnings")]
    pub realized_earnings: String,
}

/// Response data for order history
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderHistoryData {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Product ID
    #[serde(rename = "productId")]
    pub product_id: String,

    /// Order state
    /// 3: Completed (including canceled and redeemed)
    pub state: String,

    /// Protocol
    pub protocol: String,

    /// Protocol type
    /// defi: on-chain earn
    #[serde(rename = "protocolType")]
    pub protocol_type: String,

    /// Protocol term
    /// Returns the days of fixed term and returns 0 for flexible product
    pub term: String,

    /// Estimated APY
    /// If the estimated APY is 7%, this field is 0.07
    /// Retain to 4 decimal places (truncated)
    pub apy: String,

    /// Investment data
    #[serde(rename = "investData")]
    pub invest_data: Vec<HistoryInvestData>,

    /// Earning data
    #[serde(rename = "earningData")]
    pub earning_data: Vec<HistoryEarningData>,

    /// Order purchased time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "purchasedTime")]
    pub purchased_time: String,

    /// Order redeemed time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "redeemedTime")]
    pub redeemed_time: String,

    /// Order tag
    pub tag: String,
}

impl RestClient {
    /// Get order history for On-chain Earn
    ///
    /// Retrieves historical orders for On-chain Earn products.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-on-chain-earn-get-order-history)
    ///
    /// Rate limit: 3 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Arguments
    /// * `request` - Request parameters for filtering order history
    ///
    /// # Returns
    /// A vector of order history data
    pub async fn get_staking_order_history(
        &self,
        request: GetOrderHistoryRequest,
    ) -> RestResult<Vec<OrderHistoryData>> {
        self.send_get_request(
            ORDER_HISTORY_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_get_order_history_request_serialization() {
        let request = GetOrderHistoryRequest {
            product_id: Some("PROD123".to_string()),
            protocol_type: Some("defi".to_string()),
            ccy: Some("BTC".to_string()),
            after: Some("order123".to_string()),
            before: Some("order456".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("productId=PROD123"));
        assert!(serialized.contains("protocolType=defi"));
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("after=order123"));
        assert!(serialized.contains("before=order456"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_order_history_request_minimal() {
        let request = GetOrderHistoryRequest {
            product_id: None,
            protocol_type: None,
            ccy: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_history_invest_data_serialization() {
        let data = HistoryInvestData {
            ccy: "BTC".to_string(),
            amt: "2.0".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: HistoryInvestData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_history_earning_data_serialization() {
        let data = HistoryEarningData {
            ccy: "BTC".to_string(),
            earning_type: "1".to_string(),
            realized_earnings: "0.08".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: HistoryEarningData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_order_history_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "ccy": "BTC",
                "ordId": "67890",
                "productId": "PROD456",
                "state": "3",
                "protocol": "StakeProtocol",
                "protocolType": "defi",
                "term": "60",
                "apy": "0.0600",
                "investData": [
                    {
                        "ccy": "BTC",
                        "amt": "2.0"
                    }
                ],
                "earningData": [
                    {
                        "ccy": "BTC",
                        "earningType": "1",
                        "realizedEarnings": "0.12"
                    }
                ],
                "purchasedTime": "1597026383085",
                "redeemedTime": "1599618383085",
                "tag": "hist_order456"
            }
        ]"#;

        let data: Vec<OrderHistoryData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 1);

        let order = &data[0];
        assert_eq!(order.ccy, "BTC");
        assert_eq!(order.ord_id, "67890");
        assert_eq!(order.state, "3");
        assert_eq!(order.protocol_type, "defi");
        assert_eq!(order.invest_data.len(), 1);
        assert_eq!(order.earning_data.len(), 1);
        assert_eq!(order.earning_data[0].realized_earnings, "0.12");
    }

    #[test]
    fn test_order_history_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<OrderHistoryData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_get_order_history_request_default_limit() {
        let request = GetOrderHistoryRequest {
            product_id: None,
            protocol_type: Some("defi".to_string()),
            ccy: None,
            after: None,
            before: None,
            limit: Some("100".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("protocolType=defi"));
        assert!(serialized.contains("limit=100"));
    }
}
