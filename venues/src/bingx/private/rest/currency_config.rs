use serde::{Deserialize, Serialize};

use crate::bingx::{EndpointType, RestResult};

use super::RestClient;

const CURRENCY_CONFIG_ENDPOINT: &str = "/openApi/wallets/v1/capital/config/getall";

/// Request for getting currency deposit and withdrawal data
#[derive(Debug, Clone, Serialize)]
pub struct GetCurrencyConfigRequest {
    /// Coin identification (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Execution window time, cannot be greater than 60000 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_window: Option<i64>,
    /// Timestamp of initiating the request, Unit: milliseconds
    pub timestamp: i64,
}

/// Network information for a currency
#[derive(Debug, Clone, Deserialize)]
pub struct NetworkInfo {
    /// Network name
    pub name: String,
    /// Network identification
    pub network: String,
    /// Whether the currency is enabled for deposit
    #[serde(rename = "depositEnable")]
    pub deposit_enable: bool,
    /// Minimum deposit amount
    #[serde(rename = "depositMin")]
    pub deposit_min: String,
    /// Minimum number of confirmed blocks
    #[serde(rename = "minConfirm")]
    pub min_confirm: i32,
    /// Is it the default network
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    /// Is the coin open for withdrawal
    #[serde(rename = "withdrawEnable")]
    pub withdraw_enable: bool,
    /// Withdrawal fee
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: String,
    /// Maximum withdrawal amount (withdrawal limit)
    #[serde(rename = "withdrawMax")]
    pub withdraw_max: String,
    /// Minimum withdrawal amount
    #[serde(rename = "withdrawMin")]
    pub withdraw_min: String,
    /// Description of withdrawal
    #[serde(rename = "withdrawDesc")]
    pub withdraw_desc: Option<String>,
    /// Withdrawal precision
    #[serde(rename = "withdrawPrecision")]
    pub withdraw_precision: i32,
    /// Deposit precision
    #[serde(rename = "depositPrecision")]
    pub deposit_precision: i32,
    /// Contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    /// Whether memo or tag is required
    #[serde(rename = "needTagOrMemo")]
    pub need_tag_or_memo: bool,
}

/// Currency configuration information
#[derive(Debug, Clone, Deserialize)]
pub struct CurrencyConfig {
    /// Coin identification
    pub coin: String,
    /// Coin name
    pub name: String,
    /// Network information
    #[serde(rename = "networkList")]
    pub network_list: Vec<NetworkInfo>,
}

/// Response for currency deposit and withdrawal data
#[derive(Debug, Clone, Deserialize)]
pub struct GetCurrencyConfigResponse {
    /// List of currency configurations
    pub data: Vec<CurrencyConfig>,
}

impl RestClient {
    /// Get currency deposit and withdrawal data
    ///
    /// Retrieves information about coins and their deposit/withdrawal limits and settings.
    ///
    /// # Arguments
    /// * `request` - The currency config request parameters
    ///
    /// # Returns
    /// A result containing the currency config response or an error
    ///
    /// # Rate Limits
    /// - UID rate limit: 5/s
    /// - IP rate limit group 2
    ///
    /// # API Permissions
    /// - Read permission required
    pub async fn get_currency_config(
        &self,
        request: &GetCurrencyConfigRequest,
    ) -> RestResult<GetCurrencyConfigResponse> {
        self.send_request(
            CURRENCY_CONFIG_ENDPOINT,
            reqwest::Method::GET,
            Some(request),
            EndpointType::AccountApiGroup2,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_config_request_serialization() {
        let request = GetCurrencyConfigRequest {
            coin: Some("BTC".to_string()),
            recv_window: Some(5000),
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("coin=BTC"));
        assert!(serialized.contains("recv_window=5000"));
        assert!(serialized.contains("timestamp=1640995200000"));
    }

    #[test]
    fn test_currency_config_request_minimal() {
        let request = GetCurrencyConfigRequest {
            coin: None,
            recv_window: None,
            timestamp: 1640995200000,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("timestamp=1640995200000"));
        assert!(!serialized.contains("coin"));
        assert!(!serialized.contains("recv_window"));
    }

    #[test]
    fn test_network_info_deserialization() {
        let json = r#"{
            "name": "Bitcoin",
            "network": "BTC",
            "depositEnable": true,
            "depositMin": "0.0001",
            "minConfirm": 1,
            "isDefault": true,
            "withdrawEnable": true,
            "withdrawFee": "0.0005",
            "withdrawMax": "100",
            "withdrawMin": "0.001",
            "withdrawDesc": "Bitcoin network",
            "withdrawPrecision": 8,
            "depositPrecision": 8,
            "contractAddress": null,
            "needTagOrMemo": false
        }"#;

        let network: NetworkInfo = serde_json::from_str(json).unwrap();
        assert_eq!(network.name, "Bitcoin");
        assert_eq!(network.network, "BTC");
        assert!(network.deposit_enable);
        assert!(network.withdraw_enable);
        assert!(!network.need_tag_or_memo);
    }
}
