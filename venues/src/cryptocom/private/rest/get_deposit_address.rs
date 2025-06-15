use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Request parameters for get deposit address
#[derive(Debug, Clone, Serialize)]
pub struct GetDepositAddressRequest {
    /// Currency symbol e.g. BTC, CRO
    pub currency: String,
}

/// Deposit address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Deposit address ID
    pub id: String,
    /// Currency symbol e.g. BTC, CRO
    pub currency: String,
    /// Network e.g. ETH, CRO
    pub network: String,
    /// Deposit address with Address Tag (if any)
    pub address: String,
    /// Creation timestamp
    pub create_time: u64,
    /// Address status: "0" - Inactive, "1" - Active
    pub status: String,
}

/// Response for get deposit address endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositAddressResponse {
    /// Array of deposit addresses
    pub deposit_address_list: Vec<DepositAddress>,
}

impl RestClient {
    /// Fetch deposit address
    ///
    /// Fetches deposit address. Withdrawal setting must be enabled for your API Key.
    /// If you do not see the option when viewing your API Keys, this feature is not yet available for you.
    ///
    /// See: <>
    ///
    /// # Arguments
    /// * `currency` - Currency symbol e.g. BTC, CRO
    ///
    /// # Returns
    /// List of deposit addresses for the specified currency
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_deposit_address(&self, currency: &str) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;

        let params = json!({
            "currency": currency
        });

        self.send_signed_request("private/get-deposit-address", params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_get_deposit_address_request_structure() {
        let request = GetDepositAddressRequest {
            currency: "CRO".to_string(),
        };

        let json_value = serde_json::to_value(&request).unwrap();
        assert_eq!(json_value.get("currency").unwrap(), "CRO");
    }

    #[test]
    fn test_deposit_address_structure() {
        let address_json = json!({
            "currency": "CRO",
            "create_time": 1615886328000_u64,
            "id": "12345",
            "address": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "status": "1",
            "network": "CRO"
        });

        let address: DepositAddress = serde_json::from_value(address_json).unwrap();
        assert_eq!(address.currency, "CRO");
        assert_eq!(address.create_time, 1615886328000);
        assert_eq!(address.id, "12345");
        assert_eq!(
            address.address,
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        );
        assert_eq!(address.status, "1");
        assert_eq!(address.network, "CRO");
    }

    #[test]
    fn test_deposit_address_erc20() {
        let address_json = json!({
            "currency": "CRO",
            "create_time": 1615886332000_u64,
            "id": "12346",
            "address": "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
            "status": "1",
            "network": "ETH"
        });

        let address: DepositAddress = serde_json::from_value(address_json).unwrap();
        assert_eq!(address.currency, "CRO");
        assert_eq!(address.create_time, 1615886332000);
        assert_eq!(address.id, "12346");
        assert_eq!(
            address.address,
            "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"
        );
        assert_eq!(address.status, "1");
        assert_eq!(address.network, "ETH");
    }

    #[test]
    fn test_get_deposit_address_response_structure() {
        let response_json = json!({
            "deposit_address_list": [
                {
                    "currency": "CRO",
                    "create_time": 1615886328000_u64,
                    "id": "12345",
                    "address": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                    "status": "1",
                    "network": "CRO"
                },
                {
                    "currency": "CRO",
                    "create_time": 1615886332000_u64,
                    "id": "12346",
                    "address": "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy",
                    "status": "1",
                    "network": "ETH"
                }
            ]
        });

        let response: GetDepositAddressResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.deposit_address_list.len(), 2);

        let cro_address = &response.deposit_address_list.first().unwrap();
        assert_eq!(cro_address.currency, "CRO");
        assert_eq!(cro_address.network, "CRO");
        assert_eq!(cro_address.status, "1");

        let eth_address = &response.deposit_address_list.get(1).unwrap();
        assert_eq!(eth_address.currency, "CRO");
        assert_eq!(eth_address.network, "ETH");
        assert_eq!(eth_address.status, "1");
    }

    #[test]
    fn test_deposit_address_inactive_status() {
        let address_json = json!({
            "currency": "BTC",
            "create_time": 1615886328000_u64,
            "id": "12347",
            "address": "bc1qxyztuvwxyz123456789abcdef",
            "status": "0",
            "network": "BTC"
        });

        let address: DepositAddress = serde_json::from_value(address_json).unwrap();
        assert_eq!(address.currency, "BTC");
        assert_eq!(address.status, "0"); // Inactive
        assert_eq!(address.network, "BTC");
    }
}
