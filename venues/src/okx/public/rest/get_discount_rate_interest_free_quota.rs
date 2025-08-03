use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{EndpointType, RestResult};

const PUBLIC_DISCOUNT_RATE_INTEREST_FREE_QUOTA_ENDPOINT: &str =
    "api/v5/public/discount-rate-interest-free-quota";
/// Request parameters for getting discount rate and interest-free quota
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiscountRateInterestFreeQuotaRequest {
    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Discount level (Deprecated)
    #[serde(rename = "discountLv", skip_serializing_if = "Option::is_none")]
    pub discount_lv: Option<String>,
}

/// Individual discount detail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountDetail {
    /// Discount rate
    #[serde(rename = "discountRate")]
    pub discount_rate: String,
    /// Tier - upper bound. "" means positive infinity
    #[serde(rename = "maxAmt")]
    pub max_amt: String,
    /// Tier - lower bound. The minimum is 0
    #[serde(rename = "minAmt")]
    pub min_amt: String,
    /// Tiers
    pub tier: String,
    /// Liquidation penalty rate
    #[serde(rename = "liqPenaltyRate")]
    pub liq_penalty_rate: String,
    /// Discount equity in currency for quick calculation if your equity is the maxAmt
    #[serde(rename = "disCcyEq")]
    pub dis_ccy_eq: String,
}

/// Individual discount rate and interest-free quota entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountRateInterestFreeQuota {
    /// Currency
    pub ccy: String,
    /// Platform level collateralized borrow restriction
    #[serde(rename = "collateralRestrict")]
    pub collateral_restrict: bool,
    /// Interest-free quota
    pub amt: String,
    /// Discount rate level (Deprecated)
    #[serde(rename = "discountLv")]
    pub discount_lv: String,
    /// Minimum discount rate when it exceeds the maximum amount of the last tier
    #[serde(rename = "minDiscountRate")]
    pub min_discount_rate: String,
    /// New discount details
    pub details: Vec<DiscountDetail>,
}

/// Response for getting discount rate and interest-free quota
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDiscountRateInterestFreeQuotaResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Discount rate and interest-free quota data
    pub data: Vec<DiscountRateInterestFreeQuota>,
}

impl RestClient {
    /// Get discount rate and interest-free quota
    ///
    /// Retrieve discount rate level and interest-free quota.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-discount-rate-and-interest-free-quota
    ///
    /// Rate limit: 2 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The discount rate and interest-free quota request parameters
    ///
    /// # Returns
    /// Response containing the discount rate and interest-free quota information
    ///

    pub async fn get_discount_rate_interest_free_quota(
        &self,
        request: &GetDiscountRateInterestFreeQuotaRequest,
    ) -> RestResult<GetDiscountRateInterestFreeQuotaResponse> {
        self.send_request(
            PUBLIC_DISCOUNT_RATE_INTEREST_FREE_QUOTA_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::PublicMarketData,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_discount_rate_request_structure() {
        let request = GetDiscountRateInterestFreeQuotaRequest {
            ccy: Some("BTC".to_string()),
            discount_lv: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("ccy").and_then(|v| v.as_str()), Some("BTC"));
        assert!(serialized.get("discountLv").is_none());
    }

    #[test]
    fn test_get_discount_rate_request_minimal() {
        let request = GetDiscountRateInterestFreeQuotaRequest {
            ccy: None,
            discount_lv: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        // Optional fields should not be present when None
        assert!(serialized.get("ccy").is_none());
        assert!(serialized.get("discountLv").is_none());
    }

    #[test]
    fn test_get_discount_rate_request_with_all_fields() {
        let request = GetDiscountRateInterestFreeQuotaRequest {
            ccy: Some("USDT".to_string()),
            discount_lv: Some("1".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(serialized.get("ccy").and_then(|v| v.as_str()), Some("USDT"));
        assert_eq!(
            serialized.get("discountLv").and_then(|v| v.as_str()),
            Some("1")
        );
    }

    #[test]
    fn test_discount_detail_structure() {
        let detail_json = json!({
            "discountRate": "0.95",
            "maxAmt": "1000",
            "minAmt": "0",
            "tier": "1",
            "liqPenaltyRate": "0.05",
            "disCcyEq": "950"
        });

        let detail: DiscountDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.discount_rate, "0.95");
        assert_eq!(detail.max_amt, "1000");
        assert_eq!(detail.min_amt, "0");
        assert_eq!(detail.tier, "1");
        assert_eq!(detail.liq_penalty_rate, "0.05");
        assert_eq!(detail.dis_ccy_eq, "950");
    }

    #[test]
    fn test_discount_rate_interest_free_quota_structure() {
        let quota_json = json!({
            "ccy": "BTC",
            "collateralRestrict": true,
            "amt": "10",
            "discountLv": "1",
            "minDiscountRate": "0.9",
            "details": [
                {
                    "discountRate": "0.95",
                    "maxAmt": "1000",
                    "minAmt": "0",
                    "tier": "1",
                    "liqPenaltyRate": "0.05",
                    "disCcyEq": "950"
                }
            ]
        });

        let quota: DiscountRateInterestFreeQuota = serde_json::from_value(quota_json).unwrap();
        assert_eq!(quota.ccy, "BTC");
        assert!(quota.collateral_restrict);
        assert_eq!(quota.amt, "10");
        assert_eq!(quota.discount_lv, "1");
        assert_eq!(quota.min_discount_rate, "0.9");
        assert_eq!(quota.details.len(), 1);
        assert_eq!(quota.details[0].discount_rate, "0.95");
        assert_eq!(quota.details[0].tier, "1");
    }

    #[test]
    fn test_get_discount_rate_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "collateralRestrict": true,
                    "amt": "10",
                    "discountLv": "1",
                    "minDiscountRate": "0.9",
                    "details": [
                        {
                            "discountRate": "0.95",
                            "maxAmt": "1000",
                            "minAmt": "0",
                            "tier": "1",
                            "liqPenaltyRate": "0.05",
                            "disCcyEq": "950"
                        }
                    ]
                }
            ]
        });

        let response: GetDiscountRateInterestFreeQuotaResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ccy, "BTC");
        assert!(response.data[0].collateral_restrict);
        assert_eq!(response.data[0].amt, "10");
        assert_eq!(response.data[0].details.len(), 1);
    }

    #[test]
    fn test_discount_rate_response_with_multiple_currencies() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "collateralRestrict": true,
                    "amt": "10",
                    "discountLv": "1",
                    "minDiscountRate": "0.9",
                    "details": []
                },
                {
                    "ccy": "ETH",
                    "collateralRestrict": false,
                    "amt": "100",
                    "discountLv": "2",
                    "minDiscountRate": "0.85",
                    "details": [
                        {
                            "discountRate": "0.92",
                            "maxAmt": "",
                            "minAmt": "1000",
                            "tier": "2",
                            "liqPenaltyRate": "0.08",
                            "disCcyEq": "920"
                        }
                    ]
                }
            ]
        });

        let response: GetDiscountRateInterestFreeQuotaResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 2);

        // First currency
        assert_eq!(response.data[0].ccy, "BTC");
        assert!(response.data[0].collateral_restrict);
        assert_eq!(response.data[0].details.len(), 0);

        // Second currency
        assert_eq!(response.data[1].ccy, "ETH");
        assert!(!response.data[1].collateral_restrict);
        assert_eq!(response.data[1].details.len(), 1);
        assert_eq!(response.data[1].details[0].max_amt, ""); // Empty string for positive infinity
        assert_eq!(response.data[1].details[0].min_amt, "1000");
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetDiscountRateInterestFreeQuotaRequest {
            ccy: Some("USDT".to_string()),
            discount_lv: Some("3".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetDiscountRateInterestFreeQuotaRequest =
            serde_json::from_value(serialized).unwrap();

        assert_eq!(original.ccy, deserialized.ccy);
        assert_eq!(original.discount_lv, deserialized.discount_lv);
    }

    #[test]
    fn test_discount_detail_with_empty_max_amt() {
        let detail_json = json!({
            "discountRate": "0.85",
            "maxAmt": "",
            "minAmt": "5000",
            "tier": "3",
            "liqPenaltyRate": "0.15",
            "disCcyEq": "4250"
        });

        let detail: DiscountDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.discount_rate, "0.85");
        assert_eq!(detail.max_amt, ""); // Positive infinity represented as empty string
        assert_eq!(detail.min_amt, "5000");
        assert_eq!(detail.tier, "3");
    }
}
