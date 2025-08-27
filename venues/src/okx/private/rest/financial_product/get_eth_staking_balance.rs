use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

const ETH_STAKING_BALANCE_ENDPOINT: &str = "/api/v5/finance/staking-defi/eth/balance";

/// Response data for ETH staking balance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthStakingBalanceData {
    /// Currency, e.g. BETH
    pub ccy: String,

    /// Currency amount
    pub amt: String,

    /// Latest interest accrual
    #[serde(rename = "latestInterestAccrual")]
    pub latest_interest_accrual: String,

    /// Total interest accrual
    #[serde(rename = "totalInterestAccrual")]
    pub total_interest_accrual: String,

    /// Query data time, Unix timestamp format in milliseconds, e.g. 1597026383085
    pub ts: String,
}

impl RestClient {
    /// Get ETH Staking balance
    ///
    /// The balance is a snapshot summarized all BETH assets (including assets in redeeming) in account.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#financial-product-eth-staking-get-balance)
    ///
    /// Rate limit: 6 requests per second
    /// Rate limit rule: User ID
    /// Permission: Read
    ///
    /// # Returns
    /// Balance information for ETH staking
    pub async fn get_eth_staking_balance(&self) -> RestResult<EthStakingBalanceData> {
        self.send_get_request(
            ETH_STAKING_BALANCE_ENDPOINT,
            None::<&()>,
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
    fn test_eth_staking_balance_data_serialization() {
        let data = EthStakingBalanceData {
            ccy: "BETH".to_string(),
            amt: "10.5".to_string(),
            latest_interest_accrual: "0.05".to_string(),
            total_interest_accrual: "1.25".to_string(),
            ts: "1597026383085".to_string(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: EthStakingBalanceData = serde_json::from_str(&serialized).unwrap();
        assert_eq!(data, deserialized);
    }

    #[test]
    fn test_eth_staking_balance_data_deserialization_from_api() {
        let json_response = r#"{
            "ccy": "BETH",
            "amt": "32.125",
            "latestInterestAccrual": "0.0825",
            "totalInterestAccrual": "2.45",
            "ts": "1597026383085"
        }"#;

        let data: EthStakingBalanceData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.ccy, "BETH");
        assert_eq!(data.amt, "32.125");
        assert_eq!(data.latest_interest_accrual, "0.0825");
        assert_eq!(data.total_interest_accrual, "2.45");
        assert_eq!(data.ts, "1597026383085");
    }

    #[test]
    fn test_eth_staking_balance_data_zero_amounts() {
        let json_response = r#"{
            "ccy": "BETH",
            "amt": "0",
            "latestInterestAccrual": "0",
            "totalInterestAccrual": "0",
            "ts": "1597026383085"
        }"#;

        let data: EthStakingBalanceData = serde_json::from_str(json_response).unwrap();
        assert_eq!(data.amt, "0");
        assert_eq!(data.latest_interest_accrual, "0");
        assert_eq!(data.total_interest_accrual, "0");
    }
}
