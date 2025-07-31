use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bitmart::{RestResult, rate_limit::EndpointType};

const DEPOSIT_WITHDRAW_DETAIL_ENDPOINT: &str = "/account/v1/deposit-withdraw/detail";

/// Request parameters for getting deposit or withdraw detail
#[derive(Debug, Serialize, Default)]
pub struct GetDepositWithdrawDetailRequest {
    /// `withdraw_id` or `deposit_id`
    pub id: String,
}

/// Deposit or withdraw detail record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositWithdrawDetail {
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

/// Response for deposit or withdraw detail endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositWithdrawDetailResponse {
    /// Deposit or withdraw record detail
    pub record: DepositWithdrawDetail,
}

impl RestClient {
    /// Get A Deposit Or Withdraw Detail
    ///
    /// Query a single charge record
    ///
    /// Note: This endpoint is not available for sub-account
    ///
    /// [docs]: https://developer-pro.bitmart.com/en/spot/#get-a-deposit-or-withdraw-detail-keyed
    ///
    /// Rate limit: 12 times/2 sec per API key
    ///
    /// # Arguments
    /// * `request` - The request parameters
    ///
    /// # Returns
    /// Deposit or withdraw detail information
    pub async fn get_deposit_withdraw_detail(
        &self,
        request: GetDepositWithdrawDetailRequest,
    ) -> RestResult<GetDepositWithdrawDetailResponse> {
        self.send_get_request(
            DEPOSIT_WITHDRAW_DETAIL_ENDPOINT,
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
    fn test_withdraw_detail_request() {
        let request = GetDepositWithdrawDetailRequest {
            id: "1679952".to_string(),
        };
        assert_eq!(request.id, "1679952");
    }

    #[test]
    fn test_deposit_detail_request() {
        let request = GetDepositWithdrawDetailRequest {
            id: "123456".to_string(),
        };
        assert_eq!(request.id, "123456");
    }

    #[test]
    fn test_request_serialization() {
        let request = GetDepositWithdrawDetailRequest {
            id: "1679952".to_string(),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("1679952"));
        assert!(serialized.contains("id"));
    }

    #[test]
    fn test_deposit_withdraw_detail_structure() {
        let detail = DepositWithdrawDetail {
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

        assert_eq!(detail.withdraw_id, Some("1679952".to_string()));
        assert_eq!(detail.deposit_id, None);
        assert_eq!(detail.operation_type, "withdraw");
        assert_eq!(detail.currency, "BMX");
        assert_eq!(detail.apply_time, 1588867374000);
        assert_eq!(detail.arrival_amount, "59.000000000000");
        assert_eq!(detail.fee, "1.000000000000");
        assert_eq!(detail.status, 0);
        assert_eq!(
            detail.address,
            "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5"
        );
        assert_eq!(detail.address_memo, "");
        assert_eq!(detail.tx_id, "");
    }

    #[test]
    fn test_deposit_detail_structure() {
        let detail = DepositWithdrawDetail {
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

        assert_eq!(detail.withdraw_id, None);
        assert_eq!(detail.deposit_id, Some("12345".to_string()));
        assert_eq!(detail.operation_type, "deposit");
        assert_eq!(detail.status, 3); // Done
        assert_eq!(detail.tx_id, "abcdef1234567890");
    }

    #[test]
    fn test_detail_serialization_roundtrip() {
        let detail = DepositWithdrawDetail {
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

        let serialized = serde_json::to_string(&detail).unwrap();
        let deserialized: DepositWithdrawDetail = serde_json::from_str(&serialized).unwrap();

        assert_eq!(detail.withdraw_id, deserialized.withdraw_id);
        assert_eq!(detail.deposit_id, deserialized.deposit_id);
        assert_eq!(detail.operation_type, deserialized.operation_type);
        assert_eq!(detail.currency, deserialized.currency);
        assert_eq!(detail.apply_time, deserialized.apply_time);
        assert_eq!(detail.arrival_amount, deserialized.arrival_amount);
        assert_eq!(detail.fee, deserialized.fee);
        assert_eq!(detail.status, deserialized.status);
        assert_eq!(detail.address, deserialized.address);
        assert_eq!(detail.address_memo, deserialized.address_memo);
        assert_eq!(detail.tx_id, deserialized.tx_id);
    }

    #[test]
    fn test_get_deposit_withdraw_detail_response_structure() {
        let response = GetDepositWithdrawDetailResponse {
            record: DepositWithdrawDetail {
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
            },
        };

        assert_eq!(response.record.withdraw_id, Some("1679952".to_string()));
        assert_eq!(response.record.operation_type, "withdraw");
        assert_eq!(response.record.currency, "BMX");
    }

    #[test]
    fn test_response_json_parsing() {
        let json = r#"{
            "record": {
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
        }"#;

        let response: GetDepositWithdrawDetailResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.record.withdraw_id, Some("1679952".to_string()));
        assert_eq!(response.record.deposit_id, None);
        assert_eq!(response.record.operation_type, "withdraw");
        assert_eq!(response.record.currency, "BMX");
        assert_eq!(response.record.apply_time, 1588867374000);
        assert_eq!(response.record.arrival_amount, "59.000000000000");
        assert_eq!(response.record.fee, "1.000000000000");
        assert_eq!(response.record.status, 0);
        assert_eq!(
            response.record.address,
            "0xe57b69a8776b378604079650B73cdFFBDFe668Bb5"
        );
        assert_eq!(response.record.address_memo, "");
        assert_eq!(response.record.tx_id, "");
    }
}
