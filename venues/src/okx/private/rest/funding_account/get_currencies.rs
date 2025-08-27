use serde::{Deserialize, Serialize};

use crate::okx::{EndpointType, RestResult, private_client::RestClient};

/// Endpoint URL for getting currencies
const ASSET_CURRENCIES_ENDPOINT: &str = "api/v5/asset/currencies";

/// Request parameters for getting currencies
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrenciesRequest {
    /// Single currency or multiple currencies separated with comma, e.g. BTC or BTC,ETH
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Currency information
#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    /// Currency, e.g. BTC
    pub ccy: String,

    /// Name of currency. There is no related name when it is not shown.
    pub name: String,

    /// The logo link of currency
    #[serde(rename = "logoLink")]
    pub logo_link: String,

    /// Chain name, e.g. USDT-ERC20, USDT-TRC20
    pub chain: String,

    /// Contract address
    #[serde(rename = "ctAddr")]
    pub ct_addr: String,

    /// The availability to deposit from chain
    #[serde(rename = "canDep")]
    pub can_dep: bool,

    /// The availability to withdraw to chain
    #[serde(rename = "canWd")]
    pub can_wd: bool,

    /// The availability to internal transfer
    #[serde(rename = "canInternal")]
    pub can_internal: bool,

    /// Estimated opening time for deposit, Unix timestamp format in milliseconds
    #[serde(rename = "depEstOpenTime")]
    pub dep_est_open_time: String,

    /// Estimated opening time for withdraw, Unix timestamp format in milliseconds
    #[serde(rename = "wdEstOpenTime")]
    pub wd_est_open_time: String,

    /// The minimum deposit amount of currency in a single transaction
    #[serde(rename = "minDep")]
    pub min_dep: String,

    /// The minimum on-chain withdrawal amount of currency in a single transaction
    #[serde(rename = "minWd")]
    pub min_wd: String,

    /// The minimum internal transfer amount of currency in a single transaction
    #[serde(rename = "minInternal")]
    pub min_internal: String,

    /// The maximum amount of currency on-chain withdrawal in a single transaction
    #[serde(rename = "maxWd")]
    pub max_wd: String,

    /// The withdrawal precision, indicating the number of digits after the decimal point
    #[serde(rename = "wdTickSz")]
    pub wd_tick_sz: String,

    /// The withdrawal limit in the past 24 hours, unit in USD
    #[serde(rename = "wdQuota")]
    pub wd_quota: String,

    /// The amount of currency withdrawal used in the past 24 hours, unit in USD
    #[serde(rename = "usedWdQuota")]
    pub used_wd_quota: String,

    /// The fixed withdrawal fee
    pub fee: String,

    /// The minimum withdrawal fee for normal address (Deprecated)
    #[serde(rename = "minFee")]
    pub min_fee: String,

    /// The maximum withdrawal fee for normal address (Deprecated)
    #[serde(rename = "maxFee")]
    pub max_fee: String,

    /// The minimum withdrawal fee for contract address (Deprecated)
    #[serde(rename = "minFeeForCtAddr")]
    pub min_fee_for_ct_addr: String,

    /// The maximum withdrawal fee for contract address (Deprecated)
    #[serde(rename = "maxFeeForCtAddr")]
    pub max_fee_for_ct_addr: String,

    /// Burning fee rate, e.g "0.05" represents "5%"
    #[serde(rename = "burningFeeRate")]
    pub burning_fee_rate: String,

    /// If current chain is main net, then it will return true
    #[serde(rename = "mainNet")]
    pub main_net: bool,

    /// Whether tag/memo information is required for withdrawal
    #[serde(rename = "needTag")]
    pub need_tag: bool,

    /// The minimum number of blockchain confirmations to acknowledge fund deposit
    #[serde(rename = "minDepArrivalConfirm")]
    pub min_dep_arrival_confirm: String,

    /// The minimum number of blockchain confirmations required for withdrawal of a deposit
    #[serde(rename = "minWdUnlockConfirm")]
    pub min_wd_unlock_confirm: String,

    /// The fixed deposit limit, unit in USD
    #[serde(rename = "depQuotaFixed")]
    pub dep_quota_fixed: String,

    /// The used amount of fixed deposit quota, unit in USD
    #[serde(rename = "usedDepQuotaFixed")]
    pub used_dep_quota_fixed: String,

    /// The layer2 network daily deposit limit
    #[serde(rename = "depQuotaDailyLayer2")]
    pub dep_quota_daily_layer2: String,
}

impl RestClient {
    /// Get currencies
    ///
    /// Retrieve a list of all currencies available which are related to the current account's KYC entity.
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#funding-account-rest-api-get-currencies)
    ///
    /// Rate limit: 6 requests per second
    ///
    /// # Arguments
    /// * `request` - The currencies request parameters
    ///
    /// # Returns
    /// A result containing the list of currencies
    pub async fn get_currencies(&self, request: GetCurrenciesRequest) -> RestResult<Currency> {
        self.send_get_request(
            ASSET_CURRENCIES_ENDPOINT,
            Some(&request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_currencies_request_serialization() {
        let request = GetCurrenciesRequest {
            ccy: Some("BTC,ETH".to_string()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"ccy\":\"BTC,ETH\""));
    }

    #[test]
    fn test_get_currencies_request_empty() {
        let request = GetCurrenciesRequest { ccy: None };

        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_currency_deserialization() {
        let currency_json = json!({
            "ccy": "BTC",
            "name": "Bitcoin",
            "logoLink": "https://example.com/btc.png",
            "chain": "BTC-Bitcoin",
            "ctAddr": "",
            "canDep": true,
            "canWd": true,
            "canInternal": true,
            "depEstOpenTime": "",
            "wdEstOpenTime": "",
            "minDep": "0.0005",
            "minWd": "0.001",
            "minInternal": "0.00000001",
            "maxWd": "500",
            "wdTickSz": "0.00000001",
            "wdQuota": "10000000",
            "usedWdQuota": "0",
            "fee": "0.0004",
            "minFee": "0.0002",
            "maxFee": "0.0006",
            "minFeeForCtAddr": "0.0002",
            "maxFeeForCtAddr": "0.0006",
            "burningFeeRate": "0",
            "mainNet": true,
            "needTag": false,
            "minDepArrivalConfirm": "2",
            "minWdUnlockConfirm": "6",
            "depQuotaFixed": "",
            "usedDepQuotaFixed": "",
            "depQuoteDailyLayer2": ""
        });

        let currency: Currency = serde_json::from_value(currency_json).unwrap();
        assert_eq!(currency.ccy, "BTC");
        assert_eq!(currency.name, "Bitcoin");
        assert!(currency.can_dep);
        assert!(currency.main_net);
        assert!(!currency.need_tag);
    }

    #[test]
    fn test_full_response_deserialization() {
        let response_json = json!({
            "code": "0",
            "msg": "",
            "data": [
                {
                    "ccy": "BTC",
                    "name": "Bitcoin",
                    "logoLink": "https://example.com/btc.png",
                    "chain": "BTC-Bitcoin",
                    "ctAddr": "",
                    "canDep": true,
                    "canWd": true,
                    "canInternal": true,
                    "depEstOpenTime": "",
                    "wdEstOpenTime": "",
                    "minDep": "0.0005",
                    "minWd": "0.001",
                    "minInternal": "0.00000001",
                    "maxWd": "500",
                    "wdTickSz": "0.00000001",
                    "wdQuota": "10000000",
                    "usedWdQuota": "0",
                    "fee": "0.0004",
                    "minFee": "0.0002",
                    "maxFee": "0.0006",
                    "minFeeForCtAddr": "0.0002",
                    "maxFeeForCtAddr": "0.0006",
                    "burningFeeRate": "0",
                    "mainNet": true,
                    "needTag": false,
                    "minDepArrivalConfirm": "2",
                    "minWdUnlockConfirm": "6",
                    "depQuotaFixed": "",
                    "usedDepQuotaFixed": "",
                    "depQuoteDailyLayer2": ""
                }
            ]
        });

        let response: ApiResponse<Currency> = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].ccy, "BTC");
    }
}
