use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::kucoin::spot::{ResponseHeaders, RestResponse, Result};

const WITHDRAWAL_QUOTAS_ENDPOINT: &str = "/api/v1/withdrawals/quotas";

/// Request for getting withdrawal quotas
#[derive(Debug, Clone, Serialize)]
pub struct GetWithdrawalQuotasRequest {
    /// Currency code
    pub currency: String,

    /// Chain name (optional)
    pub chain: Option<String>,
}

/// Withdrawal quota information
#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawalQuota {
    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Available amount for withdrawal
    #[serde(rename = "availableAmount")]
    pub available_amount: String,

    /// Remaining daily quota
    #[serde(rename = "remainAmount")]
    pub remain_amount: String,

    /// Withdrawal minimum amount
    #[serde(rename = "withdrawMinSize")]
    pub withdraw_min_size: String,

    /// Limitation of amount
    #[serde(rename = "limitBTCAmount")]
    pub limit_btc_amount: String,

    /// Inner transfer minimum fee
    #[serde(rename = "innerWithdrawMinFee")]
    pub inner_withdraw_min_fee: String,

    /// KuCoin withdrawal fee
    #[serde(rename = "withdrawMinFee")]
    pub withdraw_min_fee: String,

    /// Is withdrawal enabled
    #[serde(rename = "isWithdrawEnabled")]
    pub is_withdraw_enabled: bool,

    /// Withdrawal precision
    pub precision: i32,
}

impl RestClient {
    /// Get withdrawal quotas for a currency
    ///
    /// Reference: https://docs.kucoin.com/#get-withdrawal-quotas
    pub async fn get_withdrawal_quotas(
        &self,
        request: GetWithdrawalQuotasRequest,
    ) -> Result<(WithdrawalQuota, ResponseHeaders)> {
        let mut params = HashMap::new();
        params.insert("currency".to_string(), request.currency);

        if let Some(chain) = request.chain {
            params.insert("chain".to_string(), chain);
        }

        let (response, headers): (RestResponse<WithdrawalQuota>, ResponseHeaders) =
            self.get(WITHDRAWAL_QUOTAS_ENDPOINT, Some(params)).await?;

        Ok((response.data, headers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdrawal_quotas_request_creation() {
        let request = GetWithdrawalQuotasRequest {
            currency: "BTC".to_string(),
            chain: Some("btc".to_string()),
        };
        assert_eq!(request.currency, "BTC");
        assert_eq!(request.chain, Some("btc".to_string()));
    }
}
