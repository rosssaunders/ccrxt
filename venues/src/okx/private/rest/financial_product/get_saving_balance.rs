use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const SAVING_BALANCE_ENDPOINT: &str = "/api/v5/finance/savings/balance";

/// Request parameters for getting saving balance
#[derive(Debug, Clone, Serialize)]
pub struct GetSavingBalanceRequest {
    /// Currency, e.g. BTC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Response data for saving balance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavingBalanceData {
    /// Currency
    pub ccy: String,

    /// Currency amount
    pub amt: String,

    /// Currency earnings
    pub earnings: String,

    /// Lending rate
    pub rate: String,

    /// Lending amount
    #[serde(rename = "loanAmt")]
    pub loan_amt: String,

    /// Pending amount
    #[serde(rename = "pendingAmt")]
    pub pending_amt: String,

    /// Redempting amount (Deprecated)
    #[serde(rename = "redemptAmt")]
    pub redempt_amt: String,
}

impl RestClient {
    /// Get saving balance
    ///
    /// Retrieves the balance information for simple earn flexible products.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-simple-earn-flexible-get-saving-balance)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Arguments
    /// * `request` - Request parameters for filtering saving balance
    ///
    /// # Returns
    /// A vector of saving balance data
    pub async fn get_saving_balance(
        &self,
        request: GetSavingBalanceRequest,
    ) -> RestResult<Vec<SavingBalanceData>> {
        self.send_get_request(
            SAVING_BALANCE_ENDPOINT,
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
    fn test_get_saving_balance_request_serialization() {
        let request = GetSavingBalanceRequest {
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_saving_balance_data_serialization() {
        let data = SavingBalanceData {
            ccy: "BTC".to_string(),
            amt: "1.5".to_string(),
            earnings: "0.025".to_string(),
            rate: "0.05".to_string(),
            loan_amt: "1.0".to_string(),
            pending_amt: "0.5".to_string(),
            redempt_amt: "0.0".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: SavingBalanceData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_saving_balance_data_deserialization_from_api() {
        let json_response = r#"[{
            "ccy": "USDT",
            "amt": "1000.0",
            "earnings": "5.5",
            "rate": "0.055",
            "loanAmt": "900.0",
            "pendingAmt": "100.0",
            "redemptAmt": "0"
        }]"#;

        let data: Vec<SavingBalanceData> = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].ccy, "USDT");
        assert_eq!(data[0].amt, "1000.0");
        assert_eq!(data[0].earnings, "5.5");
    }
}
