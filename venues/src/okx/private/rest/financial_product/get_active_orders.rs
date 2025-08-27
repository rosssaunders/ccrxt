use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const ACTIVE_ORDERS_ENDPOINT: &str = "/api/v5/finance/staking-defi/orders-active";

/// Request parameters for getting active orders
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetActiveOrdersRequest {
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

    /// Order state
    /// 8: Pending, 13: Cancelling, 9: Onchain, 1: Earning, 2: Redeeming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Investment data within an active order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvestData {
    /// Investment currency, e.g. BTC
    pub ccy: String,

    /// Invested amount
    pub amt: String,
}

/// Earning data within an active order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarningData {
    /// Earning currency, e.g. BTC
    pub ccy: String,

    /// Earning type
    /// 0: Estimated earning, 1: Cumulative earning
    #[serde(rename = "earningType")]
    pub earning_type: String,

    /// Earning amount
    pub earnings: String,
}

/// Fast redemption data within an active order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FastRedemptionData {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Redeeming amount
    #[serde(rename = "redeemingAmt")]
    pub redeeming_amt: String,
}

/// Response data for an active order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveOrderData {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Order ID
    #[serde(rename = "ordId")]
    pub ord_id: String,

    /// Product ID
    #[serde(rename = "productId")]
    pub product_id: String,

    /// Order state
    /// 8: Pending, 13: Cancelling, 9: Onchain, 1: Earning, 2: Redeeming
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
    pub invest_data: Vec<InvestData>,

    /// Earning data
    #[serde(rename = "earningData")]
    pub earning_data: Vec<EarningData>,

    /// Fast redemption data
    #[serde(rename = "fastRedemptionData")]
    pub fast_redemption_data: Vec<FastRedemptionData>,

    /// Order purchased time, Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(rename = "purchasedTime")]
    pub purchased_time: String,

    /// Estimated redemption settlement time
    #[serde(rename = "estSettlementTime")]
    pub est_settlement_time: String,

    /// Deadline for cancellation of redemption application
    #[serde(rename = "cancelRedemptionDeadline")]
    pub cancel_redemption_deadline: String,

    /// Order tag
    pub tag: String,
}

impl RestClient {
    /// Get active orders for On-chain Earn
    ///
    /// Retrieves active orders for On-chain Earn products.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-on-chain-earn-get-active-orders)
    ///
    /// Rate limit: 3 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Arguments
    /// * `request` - Request parameters for filtering active orders
    ///
    /// # Returns
    /// A vector of active order data
    pub async fn get_active_orders(
        &self,
        request: GetActiveOrdersRequest,
    ) -> RestResult<Vec<ActiveOrderData>> {
        self.send_get_request(
            ACTIVE_ORDERS_ENDPOINT,
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
    fn test_get_active_orders_request_serialization() {
        let request = GetActiveOrdersRequest {
            product_id: Some("PROD123".to_string()),
            protocol_type: Some("defi".to_string()),
            ccy: Some("BTC".to_string()),
            state: Some("1".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("productId=PROD123"));
        assert!(serialized.contains("protocolType=defi"));
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("state=1"));
    }

    #[test]
    fn test_get_active_orders_request_minimal() {
        let request = GetActiveOrdersRequest {
            product_id: None,
            protocol_type: None,
            ccy: None,
            state: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_invest_data_serialization() {
        let data = InvestData {
            ccy: "BTC".to_string(),
            amt: "1.5".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: InvestData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_earning_data_serialization() {
        let data = EarningData {
            ccy: "BTC".to_string(),
            earning_type: "1".to_string(),
            earnings: "0.05".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: EarningData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_active_order_data_deserialization_from_api() {
        let json_response = r#"[
            {
                "ccy": "BTC",
                "ordId": "12345",
                "productId": "PROD123",
                "state": "1",
                "protocol": "StakeProtocol",
                "protocolType": "defi",
                "term": "30",
                "apy": "0.0500",
                "investData": [
                    {
                        "ccy": "BTC",
                        "amt": "1.0"
                    }
                ],
                "earningData": [
                    {
                        "ccy": "BTC",
                        "earningType": "1",
                        "earnings": "0.04"
                    }
                ],
                "fastRedemptionData": [
                    {
                        "ccy": "BTC",
                        "redeemingAmt": "0.0"
                    }
                ],
                "purchasedTime": "1597026383085",
                "estSettlementTime": "1599618383085",
                "cancelRedemptionDeadline": "1599000000000",
                "tag": "order123"
            }
        ]"#;

        let data: Vec<ActiveOrderData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 1);

        let order = &data[0];
        assert_eq!(order.ccy, "BTC");
        assert_eq!(order.ord_id, "12345");
        assert_eq!(order.state, "1");
        assert_eq!(order.protocol_type, "defi");
        assert_eq!(order.invest_data.len(), 1);
        assert_eq!(order.earning_data.len(), 1);
        assert_eq!(order.fast_redemption_data.len(), 1);
    }

    #[test]
    fn test_active_order_data_empty_array() {
        let json_response = r#"[]"#;

        let data: Vec<ActiveOrderData> = serde_json::from_str(json_response).unwrap();
        assert!(data.is_empty());
    }
}
