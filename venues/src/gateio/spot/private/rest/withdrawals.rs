use serde::{Deserialize, Serialize};

use super::RestClient;

const WITHDRAWALS_ENDPOINT: &str = "/wallet/withdrawals";
const WITHDRAWAL_FEES_ENDPOINT: &str = "/wallet/fee";

/// Request parameters for retrieving withdrawal history.
///
/// Used to filter and paginate withdrawal records with support for currency-specific
/// queries, time range filtering, and result pagination for efficient data retrieval.
#[derive(Debug, Clone, Serialize, Default)]
pub struct WithdrawalsRequest {
    /// Currency code filter for specific asset withdrawals (e.g., "BTC", "ETH", "USDT").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Start time filter as Unix timestamp in seconds for withdrawal history range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time filter as Unix timestamp in seconds for withdrawal history range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Maximum number of withdrawal records to return (valid range: 1-100, default: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset for pagination through large withdrawal datasets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Complete withdrawal transaction record with status and blockchain details.
///
/// Contains comprehensive withdrawal information including transaction identifiers,
/// amounts, fees, destination addresses, and processing status for tracking
/// and auditing withdrawal operations across different blockchain networks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// Unique withdrawal identifier assigned by the exchange system.
    pub id: String,

    /// Blockchain transaction hash for on-chain withdrawals (available after confirmation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,

    /// Currency code of the withdrawn asset (e.g., "BTC", "ETH", "USDT").
    pub currency: String,

    /// Blockchain network or chain used for the withdrawal (e.g., "ETH", "TRC20", "BEP20").
    pub chain: String,

    /// Withdrawal amount as string to preserve precision for all decimal places.
    pub amount: String,

    /// Network fee charged for the withdrawal as string to preserve precision.
    pub fee: String,

    /// Destination address where the funds were sent.
    pub address: String,

    /// Payment identifier or memo required by some blockchains (e.g., XRP tag, EOS memo).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,

    /// Current withdrawal status (e.g., "pending", "processing", "completed", "failed", "cancelled").
    pub status: String,

    /// Withdrawal creation timestamp as string representation of Unix timestamp.
    pub timestamp: String,
}

/// Request parameters for retrieving withdrawal fee information.
///
/// Used to query withdrawal fees for specific currencies or all supported
/// assets, providing cost estimation for withdrawal transactions.
#[derive(Debug, Clone, Serialize, Default)]
pub struct WithdrawalFeesRequest {
    /// Currency code filter for specific asset fee information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Comprehensive withdrawal fee structure for a specific currency.
///
/// Provides complete fee information including minimum and maximum withdrawal
/// limits, fixed network fees, and percentage-based charges for accurate
/// cost calculation and withdrawal planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalFee {
    /// Currency code for which the fee information applies.
    pub currency: String,

    /// Minimum withdrawal amount allowed for this currency as string to preserve precision.
    pub min_amount: String,

    /// Maximum withdrawal amount allowed for this currency as string to preserve precision.
    pub max_amount: String,

    /// Fixed fee amount charged regardless of withdrawal size as string to preserve precision.
    pub fixed: String,

    /// Percentage fee rate applied to withdrawal amount as string to preserve precision.
    pub percent: String,
}

impl RestClient {
    /// Retrieve withdrawal history
    ///
    /// Retrieves historical withdrawal records for the authenticated user with support for
    /// currency filtering, time range queries, and pagination. Provides comprehensive
    /// withdrawal tracking including transaction status, fees, and blockchain confirmations.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#retrieve-withdrawal-records)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Withdrawal history request parameters including optional currency filter, time range, and pagination
    ///
    /// # Returns
    /// Vector of withdrawal records matching the specified criteria with complete transaction details
    pub async fn get_withdrawals(
        &self,
        params: WithdrawalsRequest,
    ) -> crate::gateio::spot::RestResult<Vec<WithdrawalRecord>> {
        self.get_with_query(WITHDRAWALS_ENDPOINT, &params).await
    }

    /// Retrieve withdrawal fee information
    ///
    /// Retrieves current withdrawal fee structures for supported currencies including
    /// minimum and maximum withdrawal amounts, fixed network fees, and percentage charges.
    /// Essential for cost calculation and withdrawal planning across different assets.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#retrieve-withdrawal-status)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Withdrawal fee request parameters with optional currency filter for specific asset information
    ///
    /// # Returns
    /// Vector of withdrawal fee structures containing complete cost information for each currency
    pub async fn get_withdrawal_fees(
        &self,
        params: WithdrawalFeesRequest,
    ) -> crate::gateio::spot::RestResult<Vec<WithdrawalFee>> {
        self.get_with_query(WITHDRAWAL_FEES_ENDPOINT, &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic withdrawal history request serialization with default parameters.
    #[test]
    fn test_withdrawals_request_default() {
        let request = WithdrawalsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_withdrawals_request_full() {
        let request = WithdrawalsRequest {
            currency: Some("ETH".to_string()),
            from: Some(1640995200),
            to: Some(1640995800),
            limit: Some(25),
            offset: Some(5),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "ETH");
        assert_eq!(json["from"], 1640995200);
        assert_eq!(json["to"], 1640995800);
        assert_eq!(json["limit"], 25);
        assert_eq!(json["offset"], 5);

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_withdrawal_record_deserialization() {
        let json = r#"{
            "id": "withdrawal_789012",
            "txid": "tx_fedcba987654321",
            "currency": "ETH",
            "chain": "ETH",
            "amount": "2.5",
            "fee": "0.005",
            "address": "0x123456789abcdef123456789abcdef123456789a",
            "payment_id": "memo456",
            "status": "pending",
            "timestamp": "1640995400"
        }"#;

        let record: WithdrawalRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "withdrawal_789012");
        assert_eq!(record.txid.as_ref().unwrap(), "tx_fedcba987654321");
        assert_eq!(record.currency, "ETH");
        assert_eq!(record.chain, "ETH");
        assert_eq!(record.amount, "2.5");
        assert_eq!(record.fee, "0.005");
        assert_eq!(record.address, "0x123456789abcdef123456789abcdef123456789a");
        assert_eq!(record.payment_id.as_ref().unwrap(), "memo456");
        assert_eq!(record.status, "pending");
        assert_eq!(record.timestamp, "1640995400");
    }

    #[test]
    fn test_withdrawal_record_without_optional_fields() {
        let json = r#"{
            "id": "withdrawal_789012",
            "txid": null,
            "currency": "ETH",
            "chain": "ETH",
            "amount": "2.5",
            "fee": "0.005",
            "address": "0x123456789abcdef123456789abcdef123456789a",
            "payment_id": null,
            "status": "pending",
            "timestamp": "1640995400"
        }"#;

        let record: WithdrawalRecord = serde_json::from_str(json).unwrap();
        assert!(record.txid.is_none());
        assert!(record.payment_id.is_none());
    }

    #[test]
    fn test_withdrawal_fees_request_default() {
        let request = WithdrawalFeesRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty());
    }

    #[test]
    fn test_withdrawal_fees_request_with_currency() {
        let request = WithdrawalFeesRequest {
            currency: Some("BTC".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");

        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
    }

    #[test]
    fn test_withdrawal_fee_deserialization() {
        let json = r#"{
            "currency": "BTC",
            "min_amount": "0.001",
            "max_amount": "100.0",
            "fixed": "0.0005",
            "percent": "0.1"
        }"#;

        let fee: WithdrawalFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency, "BTC");
        assert_eq!(fee.min_amount, "0.001");
        assert_eq!(fee.max_amount, "100.0");
        assert_eq!(fee.fixed, "0.0005");
        assert_eq!(fee.percent, "0.1");
    }

    #[test]
    fn test_withdrawal_status_types() {
        let statuses = vec!["pending", "processing", "completed", "failed", "cancelled"];

        for status in statuses {
            let json = format!(
                r#"{{
                "id": "withdrawal_789012",
                "currency": "ETH",
                "chain": "ETH",
                "amount": "2.5",
                "fee": "0.005",
                "address": "0x123456789abcdef123456789abcdef123456789a",
                "status": "{}",
                "timestamp": "1640995400"
            }}"#,
                status
            );

            let record: WithdrawalRecord = serde_json::from_str(&json).unwrap();
            assert_eq!(record.status, status);
        }
    }

    #[test]
    fn test_realistic_withdrawal_scenario() {
        let json = r#"{
            "id": "withdrawal_real_789",
            "txid": "tx_real_fed123",
            "currency": "ETH",
            "chain": "ETH",
            "amount": "1.5",
            "fee": "0.01",
            "address": "0x123456789abcdef123456789abcdef123456789a",
            "status": "completed",
            "timestamp": "1704070800"
        }"#;

        let record: WithdrawalRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.currency, "ETH");
        assert_eq!(record.amount, "1.5");
        assert_eq!(record.fee, "0.01");
        assert_eq!(record.status, "completed");

        // Verify withdrawal amount and fee
        let amount: f64 = record.amount.parse().unwrap();
        let fee: f64 = record.fee.parse().unwrap();
        assert!(amount > fee);
    }

    #[test]
    fn test_withdrawal_fee_scenarios() {
        let fees = vec![
            ("BTC", "0.001", "100.0", "0.0005", "0.1"),
            ("ETH", "0.01", "1000.0", "0.005", "0.1"),
            ("USDT", "10.0", "100000.0", "1.0", "0.1"),
        ];

        for (currency, min_amount, max_amount, fixed, percent) in fees {
            let fee = WithdrawalFee {
                currency: currency.to_string(),
                min_amount: min_amount.to_string(),
                max_amount: max_amount.to_string(),
                fixed: fixed.to_string(),
                percent: percent.to_string(),
            };

            let json = serde_json::to_value(&fee).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["min_amount"], min_amount);
            assert_eq!(json["max_amount"], max_amount);
            assert_eq!(json["fixed"], fixed);
            assert_eq!(json["percent"], percent);
        }
    }

    #[test]
    fn test_large_amounts() {
        let withdrawal = WithdrawalRecord {
            id: "withdrawal_large".to_string(),
            txid: Some("tx_large".to_string()),
            currency: "USDT".to_string(),
            chain: "TRC20".to_string(),
            amount: "1000000.123456".to_string(),
            fee: "25.0".to_string(),
            address: "TRXAddress123".to_string(),
            payment_id: None,
            status: "completed".to_string(),
            timestamp: "1640995400".to_string(),
        };

        let json = serde_json::to_value(&withdrawal).unwrap();
        assert_eq!(json["amount"], "1000000.123456");
        assert_eq!(json["fee"], "25.0");

        // Verify large amount handling
        let amount: f64 = withdrawal.amount.parse().unwrap();
        assert!(amount > 1000000.0);
    }
}
