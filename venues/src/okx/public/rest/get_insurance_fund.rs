use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::okx::{AdlType, EndpointType, InstrumentType, RestResult};


const PUBLIC_INSURANCE_FUND_ENDPOINT: &str = "api/v5/public/insurance-fund";
/// Insurance fund type for filtering insurance fund data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InsuranceFundType {
    /// Regular update
    RegularUpdate,
    /// Liquidation balance deposit
    LiquidationBalanceDeposit,
    /// Bankruptcy loss
    BankruptcyLoss,
    /// Platform revenue
    PlatformRevenue,
    /// ADL historical data
    Adl,
}

/// Request parameters for getting insurance fund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInsuranceFundRequest {
    /// Instrument type (required)
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Type of insurance fund data (optional)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub fund_type: Option<InsuranceFundType>,
    /// Underlying (conditional - required for FUTURES/SWAP/OPTION)
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// Instrument family (conditional - required for FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Currency (conditional - only applicable to MARGIN)
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Pagination - records newer than the requested ts
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Pagination - records earlier than the requested ts
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results per request (max 100, default 100)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Insurance fund detail data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceFundDetail {
    /// The balance of insurance fund
    pub balance: String,
    /// The change in the balance of insurance fund
    /// Applicable when type is liquidation_balance_deposit, bankruptcy_loss or platform_revenue
    pub amt: String,
    /// The currency of insurance fund
    pub ccy: String,
    /// The type of insurance fund
    #[serde(rename = "type")]
    pub fund_type: String,
    /// Maximum insurance fund balance in the past eight hours
    /// Only applicable when type is adl
    #[serde(rename = "maxBal", skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<String>,
    /// Timestamp when insurance fund balance reached maximum in the past eight hours
    /// Only applicable when type is adl
    #[serde(rename = "maxBalTs", skip_serializing_if = "Option::is_none")]
    pub max_balance_timestamp: Option<String>,
    /// Real-time insurance fund decline rate (compare balance and maxBal)
    /// Only applicable when type is adl (Deprecated)
    #[serde(rename = "decRate", skip_serializing_if = "Option::is_none")]
    pub decline_rate: Option<String>,
    /// ADL related events
    /// Only applicable when type is adl
    #[serde(rename = "adlType", skip_serializing_if = "Option::is_none")]
    pub adl_type: Option<AdlType>,
    /// The update timestamp of insurance fund (Unix timestamp in milliseconds)
    pub ts: String,
}

/// Response for getting insurance fund
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInsuranceFundResponse {
    /// Response code ("0" for success)
    pub code: String,
    /// Response message
    pub msg: String,
    /// Insurance fund data
    pub data: Vec<InsuranceFundData>,
}

/// Insurance fund data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceFundData {
    /// The total balance of insurance fund, in USD
    pub total: String,
    /// Instrument family (applicable to FUTURES/SWAP/OPTION)
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    /// Insurance fund details
    pub details: Vec<InsuranceFundDetail>,
}

impl RestClient {
    /// Get insurance fund balance information
    ///
    /// Retrieve insurance fund balance information for different instrument types.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-public-rest-api-get-insurance-fund
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The insurance fund request parameters
    ///
    /// # Returns
    /// Response containing insurance fund balance information
    pub async fn get_insurance_fund(
        &self,
        request: GetInsuranceFundRequest,
    ) -> RestResult<GetInsuranceFundResponse> {
        self.send_request(
            PUBLIC_INSURANCE_FUND_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::PublicInsuranceFund,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_insurance_fund_request_minimal() {
        let request = GetInsuranceFundRequest {
            inst_type: InstrumentType::Margin,
            fund_type: None,
            underlying: None,
            inst_family: None,
            currency: Some("USD".to_string()),
            before: None,
            after: None,
            limit: None,
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("MARGIN")
        );
        assert_eq!(serialized.get("ccy").and_then(|v| v.as_str()), Some("USD"));
        // Optional fields should not be present when None
        assert!(serialized.get("type").is_none());
        assert!(serialized.get("uly").is_none());
        assert!(serialized.get("instFamily").is_none());
    }

    #[test]
    fn test_get_insurance_fund_request_with_all_params() {
        let request = GetInsuranceFundRequest {
            inst_type: InstrumentType::Futures,
            fund_type: Some(InsuranceFundType::RegularUpdate),
            underlying: Some("BTC-USD".to_string()),
            inst_family: Some("BTC-USD".to_string()),
            currency: None,
            before: Some("1597026383085".to_string()),
            after: Some("1597026383000".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        assert_eq!(
            serialized.get("instType").and_then(|v| v.as_str()),
            Some("FUTURES")
        );
        assert_eq!(
            serialized.get("type").and_then(|v| v.as_str()),
            Some("regular_update")
        );
        assert_eq!(
            serialized.get("uly").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("instFamily").and_then(|v| v.as_str()),
            Some("BTC-USD")
        );
        assert_eq!(
            serialized.get("before").and_then(|v| v.as_str()),
            Some("1597026383085")
        );
        assert_eq!(
            serialized.get("after").and_then(|v| v.as_str()),
            Some("1597026383000")
        );
        assert_eq!(serialized.get("limit").and_then(|v| v.as_str()), Some("50"));
    }

    #[test]
    fn test_insurance_fund_detail_structure() {
        let detail_json = json!({
            "balance": "10000.5",
            "amt": "100.25",
            "ccy": "USDT",
            "type": "regular_update",
            "ts": "1597026383085"
        });

        let detail: InsuranceFundDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.balance, "10000.5");
        assert_eq!(detail.amt, "100.25");
        assert_eq!(detail.ccy, "USDT");
        assert_eq!(detail.fund_type, "regular_update");
        assert_eq!(detail.ts, "1597026383085");
        assert!(detail.max_balance.is_none());
        assert!(detail.max_balance_timestamp.is_none());
        assert!(detail.decline_rate.is_none());
        assert!(detail.adl_type.is_none());
    }

    #[test]
    fn test_insurance_fund_detail_with_adl() {
        let detail_json = json!({
            "balance": "10000.5",
            "amt": "",
            "ccy": "USDT",
            "type": "adl",
            "maxBal": "12000.0",
            "maxBalTs": "1597026383085",
            "decRate": "0.15",
            "adlType": "rate_adl_start",
            "ts": "1597026383085"
        });

        let detail: InsuranceFundDetail = serde_json::from_value(detail_json).unwrap();
        assert_eq!(detail.balance, "10000.5");
        assert_eq!(detail.amt, "");
        assert_eq!(detail.ccy, "USDT");
        assert_eq!(detail.fund_type, "adl");
        assert_eq!(detail.max_balance, Some("12000.0".to_string()));
        assert_eq!(
            detail.max_balance_timestamp,
            Some("1597026383085".to_string())
        );
        assert_eq!(detail.decline_rate, Some("0.15".to_string()));
        assert_eq!(detail.adl_type, Some(AdlType::RateAdlStart));
        assert_eq!(detail.ts, "1597026383085");
    }

    #[test]
    fn test_get_insurance_fund_response_structure() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "total": "50000.123456",
                    "instFamily": "BTC-USD",
                    "instType": "FUTURES",
                    "details": [
                        {
                            "balance": "10000.5",
                            "amt": "100.25",
                            "ccy": "USDT",
                            "type": "regular_update",
                            "ts": "1597026383085"
                        }
                    ]
                }
            ]
        });

        let response: GetInsuranceFundResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.msg, "");
        assert_eq!(response.data.len(), 1);

        let fund_data = &response.data[0];
        assert_eq!(fund_data.total, "50000.123456");
        assert_eq!(fund_data.inst_family, Some("BTC-USD".to_string()));
        assert_eq!(fund_data.inst_type, InstrumentType::Futures);
        assert_eq!(fund_data.details.len(), 1);

        let detail = &fund_data.details[0];
        assert_eq!(detail.balance, "10000.5");
        assert_eq!(detail.amt, "100.25");
        assert_eq!(detail.ccy, "USDT");
        assert_eq!(detail.fund_type, "regular_update");
        assert_eq!(detail.ts, "1597026383085");
    }

    #[test]
    fn test_insurance_fund_types_serialization() {
        // Test all insurance fund types
        assert_eq!(
            serde_json::to_value(&InsuranceFundType::RegularUpdate).unwrap(),
            json!("regular_update")
        );
        assert_eq!(
            serde_json::to_value(&InsuranceFundType::LiquidationBalanceDeposit).unwrap(),
            json!("liquidation_balance_deposit")
        );
        assert_eq!(
            serde_json::to_value(&InsuranceFundType::BankruptcyLoss).unwrap(),
            json!("bankruptcy_loss")
        );
        assert_eq!(
            serde_json::to_value(&InsuranceFundType::PlatformRevenue).unwrap(),
            json!("platform_revenue")
        );
        assert_eq!(
            serde_json::to_value(&InsuranceFundType::Adl).unwrap(),
            json!("adl")
        );
    }

    #[test]
    fn test_adl_types_serialization() {
        // Test all ADL types
        assert_eq!(
            serde_json::to_value(&AdlType::RateAdlStart).unwrap(),
            json!("rate_adl_start")
        );
        assert_eq!(
            serde_json::to_value(&AdlType::BalAdlStart).unwrap(),
            json!("bal_adl_start")
        );
        assert_eq!(
            serde_json::to_value(&AdlType::PosAdlStart).unwrap(),
            json!("pos_adl_start")
        );
        assert_eq!(
            serde_json::to_value(&AdlType::AdlEnd).unwrap(),
            json!("adl_end")
        );
    }

    #[test]
    fn test_request_serialization_roundtrip() {
        let original = GetInsuranceFundRequest {
            inst_type: InstrumentType::Swap,
            fund_type: Some(InsuranceFundType::Adl),
            underlying: Some("ETH-USD".to_string()),
            inst_family: None,
            currency: None,
            before: None,
            after: None,
            limit: Some("25".to_string()),
        };

        let serialized = serde_json::to_value(&original).unwrap();
        let deserialized: GetInsuranceFundRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(original.inst_type, deserialized.inst_type);
        assert_eq!(original.fund_type, deserialized.fund_type);
        assert_eq!(original.underlying, deserialized.underlying);
        assert_eq!(original.inst_family, deserialized.inst_family);
        assert_eq!(original.currency, deserialized.currency);
        assert_eq!(original.before, deserialized.before);
        assert_eq!(original.after, deserialized.after);
        assert_eq!(original.limit, deserialized.limit);
    }
}
