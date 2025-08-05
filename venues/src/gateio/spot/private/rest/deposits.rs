use serde::{Deserialize, Serialize};

use super::RestClient;

const DEPOSIT_ADDRESS_ENDPOINT: &str = "/wallet/deposit_address";
const DEPOSITS_ENDPOINT: &str = "/wallet/deposits";

/// Request parameters for obtaining a deposit address.
///
/// Used to generate or retrieve deposit addresses for a specific cryptocurrency.
/// Each currency may support multiple blockchain networks with different addresses.
#[derive(Debug, Clone, Serialize)]
pub struct DepositAddressRequest {
    /// Cryptocurrency symbol for which to get the deposit address (e.g., "BTC", "ETH", "USDT").
    pub currency: String,
}

/// Comprehensive deposit address information for a cryptocurrency.
///
/// Contains the primary deposit address and additional multichain addresses
/// for cryptocurrencies that support multiple blockchain networks. Includes
/// any required payment IDs or memos for certain networks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositAddress {
    /// Primary cryptocurrency symbol (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Primary deposit address for the default blockchain network.
    pub address: String,

    /// List of all available blockchain addresses for this cryptocurrency across different networks.
    pub multichain_addresses: Vec<MultichainAddress>,
}

/// Individual blockchain network address information.
///
/// Represents a deposit address on a specific blockchain network for cryptocurrencies
/// that support multiple chains. Includes network-specific requirements like payment IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultichainAddress {
    /// Blockchain network name (e.g., "BTC", "ETH", "BSC", "TRON").
    pub chain: String,

    /// Deposit address on this specific blockchain network.
    pub address: String,

    /// Payment ID or memo required for this network (used by some exchanges and tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Human-readable name for the payment ID field (e.g., "Memo", "Tag", "Payment ID").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_name: Option<String>,

    /// Error message if address generation failed for this blockchain network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtain_failed: Option<String>,
}

/// Request parameters for querying deposit history.
///
/// Provides filtering options to retrieve deposit records within specific time ranges,
/// for particular currencies, or with pagination controls. All parameters are optional
/// to allow flexible querying of deposit history.
#[derive(Debug, Clone, Serialize, Default)]
pub struct DepositsRequest {
    /// Filter deposits by specific cryptocurrency symbol (e.g., "BTC", "ETH"). If omitted, returns all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time for filtering deposits (Unix timestamp in seconds). If omitted, returns from earliest available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time for filtering deposits (Unix timestamp in seconds). If omitted, returns up to current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of deposit records to return (1-100, default: 100). Controls page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Number of records to skip for pagination. Used with limit for paginated results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Detailed record of a cryptocurrency deposit transaction.
///
/// Contains comprehensive information about a deposit including transaction details,
/// network information, and current status. Used for tracking deposit history
/// and transaction verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// Unique deposit identifier assigned by the exchange.
    pub id: String,

    /// Blockchain transaction hash for verification on the network explorer.
    pub txid: String,

    /// Cryptocurrency symbol that was deposited (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Blockchain network used for this deposit (e.g., "BTC", "ETH", "BSC", "TRON").
    pub chain: String,

    /// Amount of cryptocurrency deposited as a string to preserve precision.
    pub amount: String,

    /// Deposit address where the funds were sent to.
    pub address: String,

    /// Payment ID, memo, or tag if required by the network or exchange (e.g., for XRP, EOS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Current status of the deposit ("pending", "confirmed", "failed", "cancelled").
    pub status: String,

    /// Unix timestamp when the deposit was recorded (seconds since epoch).
    pub timestamp: String,
}

impl RestClient {
    /// Generate currency deposit address
    ///
    /// Generates or retrieves the deposit address for a specific cryptocurrency. The response
    /// includes addresses for all supported blockchain networks for that currency, allowing
    /// deposits from multiple networks where applicable.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#generate-currency-deposit-address
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Currency specification for which to generate the deposit address
    ///
    /// # Returns
    /// Complete deposit address information including multichain addresses and payment requirements
    pub async fn get_deposit_address(
        &self,
        params: DepositAddressRequest,
    ) -> crate::gateio::spot::RestResult<DepositAddress> {
        self.get_with_query(DEPOSIT_ADDRESS_ENDPOINT, &params).await
    }

    /// Retrieve deposit records
    ///
    /// Retrieves the deposit history for the authenticated user with optional filtering
    /// by currency, time range, and pagination. Provides comprehensive transaction details
    /// including blockchain information and current status.
    ///
    /// [docs]: https://www.gate.io/docs/developers/apiv4/en/#retrieve-deposit-records
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Filtering and pagination parameters for deposit history query
    ///
    /// # Returns
    /// List of deposit records matching the specified criteria with complete transaction details
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
