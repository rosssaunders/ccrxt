use serde::{Deserialize, Serialize};

use super::client::RestClient;

use crate::bitmart::RestResult;
use crate::bitmart::rate_limit::EndpointType;

const DEPOSIT_WITHDRAW_HISTORY_ENDPOINT: &str = "/account/v2/deposit-withdraw/history";

/// Request parameters for getting deposit and withdraw history
#[derive(Debug, Serialize, Default)]
pub struct GetDepositWithdrawHistoryRequest {
    /// Token symbol, e.g., 'BTC' (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Type
    /// - `deposit` = deposit
    /// - `withdraw` = withdraw
    pub operation_type: String,
    /// Default: 90 days from current timestamp (milliseconds)
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Default: present timestamp (milliseconds)
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// Recent N records (value range 1-1000)
    #[serde(rename = "N")]
    pub limit: i32,
}

/// Deposit or withdraw history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositWithdrawRecord {
    /// Withdraw id
    pub withdraw_id: Option<String>,
    /// Deposit id
    pub deposit_id: Option<String>,
    /// Type
    /// - `deposit` = deposit
    /// - `withdraw` = withdraw
    pub operation_type: String,
    /// Token symbol, e.g., 'BTC'
    pub currency: String,
    /// The request timestamp is accurate to milliseconds(UTC-0)
    pub apply_time: i64,
    /// Actual amount received
    pub arrival_amount: String,
    /// Fee
    pub fee: String,
    /// Status
    /// - `0` = Create
    /// - `1` = Submitted, waiting for withdrawal
    /// - `2` = Processing
    /// - `3` = Done
    /// - `4` = Cancel
    /// - `5` = Fail
    pub status: i32,
    /// Address
    pub address: String,
    /// Address tag
    pub address_memo: String,
    /// Hash record
    pub tx_id: String,
}

/// Response for deposit and withdraw history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositWithdrawHistoryResponse {
    /// Array of deposit/withdraw records
    pub records: Vec<DepositWithdrawRecord>,
}

impl RestClient {
    /// Get deposit and withdraw history
    ///
    /// Search for all existed withdraws and deposits and return their latest status.
    ///
    /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/funding_account.md
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Deposit and withdraw history information
    pub async fn get_deposit_withdraw_history(
        &self,
        request: GetDepositWithdrawHistoryRequest,
    ) -> RestResult<GetDepositWithdrawHistoryResponse> {
        self.send_request(
            DEPOSIT_WITHDRAW_HISTORY_ENDPOINT,
            reqwest::Method::GET,
            Some(&request),
            EndpointType::FundingAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_history_request() {
        let request = GetDepositWithdrawHistoryRequest {
            currency: None,
            operation_type: "deposit".to_string(),
            start_time: None,
            end_time: None,
            limit: 100,
        };
        assert_eq!(request.operation_type, "deposit");
        assert_eq!(request.limit, 100);
        assert!(request.currency.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
    }

    #[test]
    fn test_withdraw_history_request() {
        let request = GetDepositWithdrawHistoryRequest {
            currency: None,
            operation_type: "withdraw".to_string(),
            start_time: None,
            end_time: None,
            limit: 50,
        };
        assert_eq!(request.operation_type, "withdraw");
        assert_eq!(request.limit, 50);
        assert!(request.currency.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
    }

    #[test]
    fn test_request_with_currency() {
        let request = GetDepositWithdrawHistoryRequest {
            currency: Some("BTC".to_string()),
            operation_type: "deposit".to_string(),
            start_time: None,
            end_time: None,
            limit: 100,
        };
        assert_eq!(request.currency, Some("BTC".to_string()));
    }

    #[test]
    fn test_request_with_time_range() {
        let request = GetDepositWithdrawHistoryRequest {
            currency: None,
            operation_type: "withdraw".to_string(),
            start_time: Some(1739499865000),
            end_time: Some(1739586265000),
            limit: 100,
        };
        assert_eq!(request.start_time, Some(1739499865000));
        assert_eq!(request.end_time, Some(1739586265000));
    }

    #[test]
    fn test_request_serialization() {
        let request = GetDepositWithdrawHistoryRequest {
            currency: Some("BMX".to_string()),
            operation_type: "withdraw".to_string(),
            start_time: Some(1739499865000),
            end_time: None,
            limit: 100,
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("BMX"));
        assert!(serialized.contains("withdraw"));
        assert!(serialized.contains("1739499865000"));
        assert!(serialized.contains("\"N\":100"));
        // Should not contain endTime since it's None
        assert!(!serialized.contains("endTime"));
    }

    #[test]
    fn test_deposit_withdraw_record_structure() {
        let record = DepositWithdrawRecord {
            withdraw_id: Some("1679952".to_string()),
            deposit_id: None,
            operation_type: "withdraw".to_string(),
            currency: "BMX".to_string(),
            apply_time: 1588867374000,
            arrival_amount: "59.000000000000".to_string(),
            fee: "1.000000000000".to_string(),
            status: 0,
            address: "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5".to_string(),
            address_memo: "".to_string(),
            tx_id: "".to_string(),
        };

        assert_eq!(record.withdraw_id, Some("1679952".to_string()));
        assert_eq!(record.deposit_id, None);
        assert_eq!(record.operation_type, "withdraw");
        assert_eq!(record.currency, "BMX");
        assert_eq!(record.apply_time, 1588867374000);
        assert_eq!(record.arrival_amount, "59.000000000000");
        assert_eq!(record.fee, "1.000000000000");
        assert_eq!(record.status, 0);
        assert_eq!(
            record.address,
            "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5"
        );
        assert_eq!(record.address_memo, "");
        assert_eq!(record.tx_id, "");
    }

    #[test]
    fn test_deposit_record_structure() {
        let record = DepositWithdrawRecord {
            withdraw_id: None,
            deposit_id: Some("12345".to_string()),
            operation_type: "deposit".to_string(),
            currency: "BTC".to_string(),
            apply_time: 1588867374000,
            arrival_amount: "0.50000000".to_string(),
            fee: "0.00000000".to_string(),
            status: 3,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            address_memo: "".to_string(),
            tx_id: "abcdef1234567890".to_string(),
        };

        assert_eq!(record.withdraw_id, None);
        assert_eq!(record.deposit_id, Some("12345".to_string()));
        assert_eq!(record.operation_type, "deposit");
        assert_eq!(record.status, 3); // Done
    }

    #[test]
    fn test_record_serialization_roundtrip() {
        let record = DepositWithdrawRecord {
            withdraw_id: Some("1679952".to_string()),
            deposit_id: None,
            operation_type: "withdraw".to_string(),
            currency: "USDT".to_string(),
            apply_time: 1588867374000,
            arrival_amount: "100.000000000000".to_string(),
            fee: "5.000000000000".to_string(),
            status: 3,
            address: "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string(),
            address_memo: "".to_string(),
            tx_id: "abc123def456".to_string(),
        };

        let serialized = serde_json::to_string(&record).unwrap();
        let deserialized: DepositWithdrawRecord = serde_json::from_str(&serialized).unwrap();

        assert_eq!(record.withdraw_id, deserialized.withdraw_id);
        assert_eq!(record.deposit_id, deserialized.deposit_id);
        assert_eq!(record.operation_type, deserialized.operation_type);
        assert_eq!(record.currency, deserialized.currency);
        assert_eq!(record.apply_time, deserialized.apply_time);
        assert_eq!(record.arrival_amount, deserialized.arrival_amount);
        assert_eq!(record.fee, deserialized.fee);
        assert_eq!(record.status, deserialized.status);
        assert_eq!(record.address, deserialized.address);
        assert_eq!(record.address_memo, deserialized.address_memo);
        assert_eq!(record.tx_id, deserialized.tx_id);
    }

    #[test]
    fn test_get_deposit_withdraw_history_response_structure() {
        let response = GetDepositWithdrawHistoryResponse {
            records: vec![DepositWithdrawRecord {
                withdraw_id: Some("1679952".to_string()),
                deposit_id: None,
                operation_type: "withdraw".to_string(),
                currency: "BMX".to_string(),
                apply_time: 1588867374000,
                arrival_amount: "59.000000000000".to_string(),
                fee: "1.000000000000".to_string(),
                status: 0,
                address: "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5".to_string(),
                address_memo: "".to_string(),
                tx_id: "".to_string(),
            }],
        };

        assert_eq!(response.records.len(), 1);
        assert_eq!(response.records[0].withdraw_id, Some("1679952".to_string()));
        assert_eq!(response.records[0].operation_type, "withdraw");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "records": [
                {
                    "withdraw_id": "1679952",
                    "deposit_id": null,
                    "operation_type": "withdraw",
                    "currency": "BMX",
                    "apply_time": 1588867374000,
                    "arrival_amount": "59.000000000000",
                    "fee": "1.000000000000",
                    "status": 0,
                    "address": "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5",
                    "address_memo": "",
                    "tx_id": ""
                }
            ]
        }"#;

        let response: GetDepositWithdrawHistoryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.records.len(), 1);
        assert_eq!(response.records[0].withdraw_id, Some("1679952".to_string()));
        assert_eq!(response.records[0].deposit_id, None);
        assert_eq!(response.records[0].operation_type, "withdraw");
        assert_eq!(response.records[0].currency, "BMX");
        assert_eq!(response.records[0].apply_time, 1588867374000);
        assert_eq!(response.records[0].arrival_amount, "59.000000000000");
        assert_eq!(response.records[0].fee, "1.000000000000");
        assert_eq!(response.records[0].status, 0);
        assert_eq!(
            response.records[0].address,
            "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5"
        );
        assert_eq!(response.records[0].address_memo, "");
        assert_eq!(response.records[0].tx_id, "");
    }
}
