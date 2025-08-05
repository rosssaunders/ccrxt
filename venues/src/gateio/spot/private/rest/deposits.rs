use serde::{Deserialize, Serialize};

use super::RestClient;

const DEPOSIT_ADDRESS_ENDPOINT: &str = "/wallet/deposit_address";
const DEPOSITS_ENDPOINT: &str = "/wallet/deposits";

/// Request parameters for deposit address
#[derive(Debug, Clone, Serialize)]
pub struct DepositAddressRequest {
    /// Currency
    pub currency: String,
}

/// Deposit address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Currency
    pub currency: String,

    /// Deposit address
    pub address: String,

    /// Address name
    pub multichain_addresses: Vec<MultichainAddress>,
}

/// Multichain address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultichainAddress {
    /// Chain name
    pub chain: String,

    /// Address
    pub address: String,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Payment name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_name: Option<String>,

    /// Obtain failed reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtain_failed: Option<String>,
}

/// Request parameters for deposits
#[derive(Debug, Clone, Serialize, Default)]
pub struct DepositsRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time (Unix timestamp in seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Deposit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// Deposit ID
    pub id: String,

    /// Transaction ID
    pub txid: String,

    /// Currency
    pub currency: String,

    /// Chain
    pub chain: String,

    /// Amount
    pub amount: String,

    /// Address
    pub address: String,

    /// Payment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Status
    pub status: String,

    /// Timestamp
    pub timestamp: String,
}

impl RestClient {
    /// Get deposit address
    ///
    /// This endpoint returns the deposit address for a specific currency.
    ///
    /// [docs]: https://www.gate.com/docs/developers/apiv4/#generate-currency-deposit-address
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `params` - The deposit address request parameters
    ///
    /// # Returns
    /// Deposit address information for the specified currency
    pub async fn get_deposit_address(
        &self,
        params: DepositAddressRequest,
    ) -> crate::gateio::spot::RestResult<DepositAddress> {
        self.get_with_query(DEPOSIT_ADDRESS_ENDPOINT, &params).await
    }

    /// Get deposit history
    ///
    /// This endpoint returns the deposit history for the authenticated user.
    ///
    /// [docs]: https://www.gate.com/docs/developers/apiv4/#retrieve-deposit-records
    ///
    /// Rate limit: varies by endpoint type
    ///
    /// # Arguments
    /// * `params` - The deposits request parameters
    ///
    /// # Returns
    /// List of deposit records matching the criteria
    pub async fn get_deposits(
        &self,
        params: DepositsRequest,
    ) -> crate::gateio::spot::RestResult<Vec<DepositRecord>> {
        self.get_with_query(DEPOSITS_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_address_request() {
        let request = DepositAddressRequest {
            currency: "BTC".to_string(),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
    }

    #[test]
    fn test_deposit_address_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "multichain_addresses": [
                {
                    "chain": "BTC",
                    "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                    "payment_id": null,
                    "payment_name": null,
                    "obtain_failed": null
                },
                {
                    "chain": "BSC",
                    "address": "0x123456789abcdef123456789abcdef123456789a",
                    "payment_id": "memo123",
                    "payment_name": "BSC BTC",
                    "obtain_failed": null
                }
            ]
        }"#;

        let address: DepositAddress = serde_json::from_str(json).unwrap();
        assert_eq!(address.currency, "BTC");
        assert_eq!(
            address.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(address.multichain_addresses.len(), 2);

        let btc_chain = &address.multichain_addresses[0];
        assert_eq!(btc_chain.chain, "BTC");
        assert_eq!(
            btc_chain.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert!(btc_chain.payment_id.is_none());

        let bsc_chain = &address.multichain_addresses[1];
        assert_eq!(bsc_chain.chain, "BSC");
        assert_eq!(bsc_chain.payment_id.as_ref().unwrap(), "memo123");
        assert_eq!(bsc_chain.payment_name.as_ref().unwrap(), "BSC BTC");
    }

    #[test]
    fn test_deposits_request_default() {
        let request = DepositsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_deposits_request_full() {
        let request = DepositsRequest {
            currency: Some("BTC".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(50),
            offset: Some(10),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 50);
        assert_eq!(json["offset"], 10);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_deposit_record_deserialization() {
        let json = r#"{
            "id": "deposit_123456",
            "txid": "tx_abcdef123456789",
            "currency": "BTC",
            "chain": "BTC",
            "amount": "0.5",
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "payment_id": "memo123",
            "status": "confirmed",
            "timestamp": "1640995200"
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "deposit_123456");
        assert_eq!(record.txid, "tx_abcdef123456789");
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.chain, "BTC");
        assert_eq!(record.amount, "0.5");
        assert_eq!(record.address, "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(record.payment_id.as_ref().unwrap(), "memo123");
        assert_eq!(record.status, "confirmed");
        assert_eq!(record.timestamp, "1640995200");
    }

    #[test]
    fn test_deposit_record_without_payment_id() {
        let json = r#"{
            "id": "deposit_123456",
            "txid": "tx_abcdef123456789",
            "currency": "BTC",
            "chain": "BTC",
            "amount": "0.5",
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "payment_id": null,
            "status": "confirmed",
            "timestamp": "1640995200"
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();
        assert!(record.payment_id.is_none());
    }

    #[test]
    fn test_multichain_address_with_all_fields() {
        let json = r#"{
            "chain": "BSC",
            "address": "0x123456789abcdef123456789abcdef123456789a",
            "payment_id": "memo123",
            "payment_name": "BSC Chain",
            "obtain_failed": "Network maintenance"
        }"#;

        let multichain: MultichainAddress = serde_json::from_str(json).unwrap();
        assert_eq!(multichain.chain, "BSC");
        assert_eq!(
            multichain.address,
            "0x123456789abcdef123456789abcdef123456789a"
        );
        assert_eq!(multichain.payment_id.as_ref().unwrap(), "memo123");
        assert_eq!(multichain.payment_name.as_ref().unwrap(), "BSC Chain");
        assert_eq!(
            multichain.obtain_failed.as_ref().unwrap(),
            "Network maintenance"
        );
    }

    #[test]
    fn test_multichain_address_minimal() {
        let json = r#"{
            "chain": "ETH",
            "address": "0xabcdef123456789abcdef123456789abcdef123456"
        }"#;

        let multichain: MultichainAddress = serde_json::from_str(json).unwrap();
        assert_eq!(multichain.chain, "ETH");
        assert_eq!(
            multichain.address,
            "0xabcdef123456789abcdef123456789abcdef123456"
        );
        assert!(multichain.payment_id.is_none());
        assert!(multichain.payment_name.is_none());
        assert!(multichain.obtain_failed.is_none());
    }

    #[test]
    fn test_deposit_status_types() {
        let statuses = vec!["pending", "confirmed", "failed", "cancelled"];

        for status in statuses {
            let json = format!(
                r#"{{
                "id": "deposit_123456",
                "txid": "tx_abcdef123456789",
                "currency": "BTC",
                "chain": "BTC",
                "amount": "0.5",
                "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
                "status": "{}",
                "timestamp": "1640995200"
            }}"#,
                status
            );

            let record: DepositRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.status, status);
        }
    }

    #[test]
    fn test_realistic_deposit_scenario() {
        let json = r#"{
            "id": "deposit_456789",
            "txid": "tx_real_transaction_123",
            "currency": "BTC",
            "chain": "BTC",
            "amount": "0.05",
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "status": "confirmed",
            "timestamp": "1704067200"
        }"#;

        let record: DepositRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "BTC");
        assert_eq!(record.amount, "0.05");
        assert_eq!(record.status, "confirmed");

        // Verify timestamp parsing
        let timestamp: i64 = record.timestamp.parse().unwrap();
        assert!(timestamp > 0);
    }

    #[test]
    fn test_high_precision_amounts() {
        let deposit = DepositRecord {
            id: "deposit_123".to_string(),
            txid: "tx_123".to_string(),
            currency: "BTC".to_string(),
            chain: "BTC".to_string(),
            amount: "0.123456789012345678".to_string(),
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            payment_id: None,
            status: "confirmed".to_string(),
            timestamp: "1640995200".to_string(),
        };

        let json = serde_json::to_value(&deposit).unwrap();
        assert_eq!(json["amount"], "0.123456789012345678");

        // Verify precision is maintained
        assert_eq!(deposit.amount.len(), 20); // 18 decimal places + "0."
    }

    #[test]
    fn test_serialization_round_trip() {
        let multichain = MultichainAddress {
            chain: "BSC".to_string(),
            address: "0x123456789abcdef123456789abcdef123456789a".to_string(),
            payment_id: Some("memo123".to_string()),
            payment_name: Some("BSC Chain".to_string()),
            obtain_failed: None,
        };

        let json = serde_json::to_string(&multichain).unwrap();
        let deserialized: MultichainAddress = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.chain, multichain.chain);
        assert_eq!(deserialized.address, multichain.address);
        assert_eq!(deserialized.payment_id, multichain.payment_id);
        assert_eq!(deserialized.payment_name, multichain.payment_name);
        assert_eq!(deserialized.obtain_failed, multichain.obtain_failed);
    }
}
