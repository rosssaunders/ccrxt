use serde::{Deserialize, Serialize};

use super::RestClient;

const MARGIN_LOAN_RECORDS_ENDPOINT: &str = "/margin/loan_records";
const MARGIN_LOANS_ENDPOINT: &str = "/margin/loans";

/// Request parameters for listing margin loans with comprehensive filtering options.
///
/// Used to query margin lending and borrowing history with extensive filtering
/// capabilities for status, side, currency, and sorting options for detailed
/// loan portfolio management and analysis.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLoansRequest {
    /// Loan status filter (e.g., "open", "finished", "cancelled").
    ///
    /// When specified, returns only loans with the matching status.
    /// Common values include "open" for active loans, "finished" for completed loans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Side filter for loan operation type ("lend" for lending, "borrow" for borrowing).
    ///
    /// Allows filtering to show only lending operations or only borrowing operations.
    /// Essential for separating loan portfolio by operation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,

    /// Currency filter for specific asset loans (e.g., "BTC", "ETH", "USDT").
    ///
    /// When specified, returns only loans for the specified currency.
    /// Useful for analyzing lending/borrowing activity for specific assets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Trading pair filter for loans tied to specific markets (e.g., "BTC_USDT").
    ///
    /// Filters loans to those associated with a particular trading pair.
    /// Helpful for margin trading analysis on specific markets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,

    /// Sort field for result ordering (e.g., "create_time", "amount", "rate").
    ///
    /// Determines the field used for sorting the returned loan records.
    /// Common options include sorting by creation time or loan amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Reverse sort order flag for descending order when true.
    ///
    /// When true, sorts results in descending order. When false or omitted,
    /// sorts in ascending order based on the sort_by field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_sort: Option<bool>,

    /// Page number for pagination (1-based indexing, default: 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return per page (1-100, default: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Request to create a loan
#[derive(Debug, Clone, Serialize)]
pub struct CreateLoanRequest {
    /// Side (lend, borrow)
    pub side: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan amount
    pub amount: String,

    /// Interest rate (for lending)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,

    /// Days to lend/borrow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,

    /// Auto renew enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

/// Request to modify a loan
#[derive(Debug, Clone, Serialize)]
pub struct ModifyLoanRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// New loan amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,

    /// New interest rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,

    /// Auto renew setting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
}

/// Loan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loan {
    /// Loan ID
    pub id: String,

    /// Side (lend, borrow)
    pub side: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan rate
    pub rate: String,

    /// Original amount
    pub amount: String,

    /// Days
    pub days: i32,

    /// Auto renew enabled
    pub auto_renew: bool,

    /// In use amount
    pub in_use: String,

    /// Left amount
    pub left: String,

    /// Loan status
    pub status: String,

    /// Creation time
    pub create_time: i64,

    /// Update time
    pub update_time: i64,
}

/// Request to repay a loan
#[derive(Debug, Clone, Serialize)]
pub struct RepayLoanRequest {
    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Repayment mode (all, partial)
    pub mode: String,

    /// Amount to repay (for partial repayment)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

/// Loan repayment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRepayment {
    /// Loan ID
    pub loan_id: String,

    /// Repayment ID
    pub repay_id: String,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Principal amount repaid
    pub principal: String,

    /// Interest amount repaid
    pub interest: String,

    /// Repayment time
    pub repay_time: i64,
}

/// Request parameters for repayment records
#[derive(Debug, Clone, Serialize, Default)]
pub struct RepaymentRecordsRequest {
    /// Loan ID
    pub loan_id: String,
}

/// Request parameters for loan records
#[derive(Debug, Clone, Serialize, Default)]
pub struct LoanRecordsRequest {
    /// Loan record ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_record_id: Option<String>,

    /// Status filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Page number (default: 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,

    /// Maximum number of records to return (1-100, default: 100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Loan record information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRecord {
    /// Loan record ID
    pub id: String,

    /// Loan ID
    pub loan_id: String,

    /// Borrower user ID
    pub borrower_id: i64,

    /// Lender user ID
    pub lender_id: i64,

    /// Currency
    pub currency: String,

    /// Currency pair
    pub currency_pair: String,

    /// Loan rate
    pub rate: String,

    /// Amount
    pub amount: String,

    /// Days
    pub days: i32,

    /// Status
    pub status: String,

    /// Repaid amount
    pub repaid: String,

    /// Paid interest
    pub paid_interest: String,

    /// Unpaid interest
    pub unpaid_interest: String,

    /// Creation time
    pub create_time: i64,

    /// Expire time
    pub expire_time: i64,
}

impl RestClient {
    /// Margin Loans
    ///
    /// Retrieve a list of margin loans for lending or borrowing with comprehensive
    /// filtering options for status, side, currency, and sorting capabilities for
    /// detailed loan portfolio management and analysis.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/#list-margin-loans)
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - Request parameters for filtering margin loans
    ///
    /// # Returns
    /// List of margin loan information matching the specified criteria
    pub async fn list_margin_loans(
        &self,
        params: ListLoansRequest,
    ) -> crate::gateio::spot::RestResult<Vec<Loan>> {
        self.get_with_query(MARGIN_LOANS_ENDPOINT, &params).await
    }

    /// Get a specific loan
    ///
    /// This endpoint returns details for a specific loan by ID.
    pub async fn get_loan(&self, loan_id: &str) -> crate::gateio::spot::RestResult<Loan> {
        let endpoint = format!("/margin/loans/{}", loan_id);
        self.get(&endpoint).await
    }

    /// Create a loan
    ///
    /// This endpoint creates a new loan for lending or borrowing.
    pub async fn create_loan(
        &self,
        request: CreateLoanRequest,
    ) -> crate::gateio::spot::RestResult<Loan> {
        self.post("/margin/loans", &request).await
    }

    /// Modify a loan
    ///
    /// This endpoint modifies an existing loan's parameters.
    pub async fn modify_loan(
        &self,
        loan_id: &str,
        request: ModifyLoanRequest,
    ) -> crate::gateio::spot::RestResult<Loan> {
        let endpoint = format!("/margin/loans/{}", loan_id);
        self.patch(&endpoint, &request).await
    }

    /// Cancel a loan
    ///
    /// This endpoint cancels an existing loan.
    pub async fn cancel_loan(
        &self,
        loan_id: &str,
        currency: &str,
        currency_pair: &str,
    ) -> crate::gateio::spot::RestResult<Loan> {
        let endpoint = format!(
            "/margin/loans/{}?currency={}&currency_pair={}",
            loan_id, currency, currency_pair
        );
        self.delete(&endpoint).await
    }

    /// Repay a loan
    ///
    /// This endpoint creates a repayment for a loan.
    pub async fn repay_loan(
        &self,
        loan_id: &str,
        request: RepayLoanRequest,
    ) -> crate::gateio::spot::RestResult<Vec<LoanRepayment>> {
        let endpoint = format!("/margin/loans/{}/repayment", loan_id);
        self.post(&endpoint, &request).await
    }

    /// Get repayment records for a loan
    ///
    /// This endpoint returns repayment records for a specific loan.
    pub async fn get_repayment_records(
        &self,
        loan_id: &str,
    ) -> crate::gateio::spot::RestResult<Vec<LoanRepayment>> {
        let endpoint = format!("/margin/loans/{}/repayment", loan_id);
        self.get(&endpoint).await
    }

    /// Get loan records
    ///
    /// This endpoint returns loan records showing lending/borrowing activity.
    pub async fn get_loan_records(
        &self,
        params: LoanRecordsRequest,
    ) -> crate::gateio::spot::RestResult<Vec<LoanRecord>> {
        self.get_with_query(MARGIN_LOAN_RECORDS_ENDPOINT, &params)
            .await
    }

    /// Get a specific loan record
    ///
    /// This endpoint returns details for a specific loan record by ID.
    pub async fn get_loan_record(
        &self,
        loan_record_id: &str,
        params: LoanRecordsRequest,
    ) -> crate::gateio::spot::RestResult<LoanRecord> {
        let endpoint = format!("/margin/loan_records/{}", loan_record_id);
        self.get_with_query(&endpoint, &params).await
    }

    /// Modify a loan record
    ///
    /// This endpoint modifies an existing loan record.
    pub async fn modify_loan_record(
        &self,
        loan_record_id: &str,
        auto_renew: bool,
    ) -> crate::gateio::spot::RestResult<LoanRecord> {
        let endpoint = format!("/margin/loan_records/{}", loan_record_id);
        let request = serde_json::json!({
            "auto_renew": auto_renew
        });
        self.patch(&endpoint, &request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test margin loans request serialization with default parameters.
    #[test]
    fn test_list_loans_request_default() {
        let request = ListLoansRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_list_loans_request_with_all_filters() {
        let request = ListLoansRequest {
            status: Some("open".to_string()),
            side: Some("borrow".to_string()),
            currency: Some("BTC".to_string()),
            currency_pair: Some("BTC_USDT".to_string()),
            sort_by: Some("create_time".to_string()),
            reverse_sort: Some(true),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "open");
        assert_eq!(json["side"], "borrow");
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["sort_by"], "create_time");
        assert_eq!(json["reverse_sort"], true);
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 50);
    }

    #[test]
    fn test_list_loans_request_different_statuses() {
        let statuses = vec!["open", "finished"];

        for status in statuses {
            let request = ListLoansRequest {
                status: Some(status.to_string()),
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["status"], status);
        }
    }

    #[test]
    fn test_list_loans_request_different_sides() {
        let sides = vec!["lend", "borrow"];

        for side in sides {
            let request = ListLoansRequest {
                side: Some(side.to_string()),
                ..Default::default()
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["side"], side);
        }
    }

    #[test]
    fn test_create_loan_request_borrowing() {
        let request = CreateLoanRequest {
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "10000.0".to_string(),
            rate: None,
            days: Some(30),
            auto_renew: Some(false),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "borrow");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["amount"], "10000.0");
        assert_eq!(json["days"], 30);
        assert_eq!(json["auto_renew"], false);

        // rate should be omitted for borrowing
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("rate"));
    }

    #[test]
    fn test_create_loan_request_lending() {
        let request = CreateLoanRequest {
            side: "lend".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "5000.0".to_string(),
            rate: Some("0.05".to_string()),
            days: Some(7),
            auto_renew: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "lend");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["amount"], "5000.0");
        assert_eq!(json["rate"], "0.05");
        assert_eq!(json["days"], 7);
        assert_eq!(json["auto_renew"], true);
    }

    #[test]
    fn test_modify_loan_request_amount_only() {
        let request = ModifyLoanRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: Some("0.5".to_string()),
            rate: None,
            auto_renew: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "BTC");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["amount"], "0.5");

        // rate and auto_renew should be omitted
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("rate"));
        assert!(!obj.contains_key("auto_renew"));
    }

    #[test]
    fn test_modify_loan_request_rate_only() {
        let request = ModifyLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "ETH_USDT".to_string(),
            amount: None,
            rate: Some("0.08".to_string()),
            auto_renew: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "ETH_USDT");
        assert_eq!(json["rate"], "0.08");

        // amount and auto_renew should be omitted
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
        assert!(!obj.contains_key("auto_renew"));
    }

    #[test]
    fn test_loan_deserialization() {
        let json = r#"{
            "id": "12345678",
            "side": "borrow",
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "rate": "0.05",
            "amount": "10000.0",
            "days": 30,
            "auto_renew": false,
            "in_use": "5000.0",
            "left": "5000.0",
            "status": "open",
            "create_time": 1640995200,
            "update_time": 1640995300
        }"#;

        let loan: Loan = serde_json::from_str(json).unwrap();
        assert_eq!(loan.id, "12345678");
        assert_eq!(loan.side, "borrow");
        assert_eq!(loan.currency, "USDT");
        assert_eq!(loan.currency_pair, "BTC_USDT");
        assert_eq!(loan.rate, "0.05");
        assert_eq!(loan.amount, "10000.0");
        assert_eq!(loan.days, 30);
        assert!(!loan.auto_renew);
        assert_eq!(loan.in_use, "5000.0");
        assert_eq!(loan.left, "5000.0");
        assert_eq!(loan.status, "open");
        assert_eq!(loan.create_time, 1640995200);
        assert_eq!(loan.update_time, 1640995300);
    }

    #[test]
    fn test_loan_different_sides() {
        let sides = vec!["lend", "borrow"];

        for side in sides {
            let json = format!(
                r#"{{
                "id": "12345678",
                "side": "{}",
                "currency": "USDT",
                "currency_pair": "BTC_USDT",
                "rate": "0.05",
                "amount": "10000.0",
                "days": 30,
                "auto_renew": false,
                "in_use": "5000.0",
                "left": "5000.0",
                "status": "open",
                "create_time": 1640995200,
                "update_time": 1640995300
            }}"#,
                side
            );

            let loan: Loan = serde_json::from_str(&json).unwrap();
            assert_eq!(loan.side, side);
        }
    }

    #[test]
    fn test_loan_different_statuses() {
        let statuses = vec!["open", "finished", "cancelled"];

        for status in statuses {
            let json = format!(
                r#"{{
                "id": "12345678",
                "side": "borrow",
                "currency": "USDT",
                "currency_pair": "BTC_USDT",
                "rate": "0.05",
                "amount": "10000.0",
                "days": 30,
                "auto_renew": false,
                "in_use": "5000.0",
                "left": "5000.0",
                "status": "{}",
                "create_time": 1640995200,
                "update_time": 1640995300
            }}"#,
                status
            );

            let loan: Loan = serde_json::from_str(&json).unwrap();
            assert_eq!(loan.status, status);
        }
    }

    #[test]
    fn test_repay_loan_request_full_repayment() {
        let request = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "all".to_string(),
            amount: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["mode"], "all");

        // amount should be omitted for full repayment
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
    }

    #[test]
    fn test_repay_loan_request_partial_repayment() {
        let request = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "partial".to_string(),
            amount: Some("2500.0".to_string()),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["mode"], "partial");
        assert_eq!(json["amount"], "2500.0");
    }

    #[test]
    fn test_loan_repayment_deserialization() {
        let json = r#"{
            "loan_id": "12345678",
            "repay_id": "87654321",
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "principal": "2000.0",
            "interest": "25.0",
            "repay_time": 1640995500
        }"#;

        let repayment: LoanRepayment = serde_json::from_str(json).unwrap();
        assert_eq!(repayment.loan_id, "12345678");
        assert_eq!(repayment.repay_id, "87654321");
        assert_eq!(repayment.currency, "USDT");
        assert_eq!(repayment.currency_pair, "BTC_USDT");
        assert_eq!(repayment.principal, "2000.0");
        assert_eq!(repayment.interest, "25.0");
        assert_eq!(repayment.repay_time, 1640995500);
    }

    #[test]
    fn test_loan_records_request_default() {
        let request = LoanRecordsRequest::default();

        let json = serde_json::to_value(&request).unwrap();
        let obj = json.as_object().unwrap();
        assert!(obj.is_empty()); // No fields should be serialized when None
    }

    #[test]
    fn test_loan_records_request_with_filters() {
        let request = LoanRecordsRequest {
            loan_record_id: Some("123456".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(25),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["loan_record_id"], "123456");
        assert_eq!(json["status"], "open");
        assert_eq!(json["page"], 1);
        assert_eq!(json["limit"], 25);
    }

    #[test]
    fn test_loan_record_deserialization() {
        let json = r#"{
            "id": "record123",
            "loan_id": "loan456",
            "borrower_id": 789,
            "lender_id": 101112,
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "rate": "0.05",
            "amount": "10000.0",
            "days": 30,
            "status": "active",
            "repaid": "2500.0",
            "paid_interest": "50.0",
            "unpaid_interest": "25.0",
            "create_time": 1640995200,
            "expire_time": 1643587200
        }"#;

        let record: LoanRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.id, "record123");
        assert_eq!(record.loan_id, "loan456");
        assert_eq!(record.borrower_id, 789);
        assert_eq!(record.lender_id, 101112);
        assert_eq!(record.currency, "USDT");
        assert_eq!(record.currency_pair, "BTC_USDT");
        assert_eq!(record.rate, "0.05");
        assert_eq!(record.amount, "10000.0");
        assert_eq!(record.days, 30);
        assert_eq!(record.status, "active");
        assert_eq!(record.repaid, "2500.0");
        assert_eq!(record.paid_interest, "50.0");
        assert_eq!(record.unpaid_interest, "25.0");
        assert_eq!(record.create_time, 1640995200);
        assert_eq!(record.expire_time, 1643587200);
    }

    #[test]
    fn test_list_loans_request_realistic_borrow_filter_scenario() {
        // Scenario: List all active borrowing loans
        let request = ListLoansRequest {
            status: Some("open".to_string()),
            side: Some("borrow".to_string()),
            currency: None,
            currency_pair: None,
            sort_by: Some("create_time".to_string()),
            reverse_sort: Some(false),
            page: Some(1),
            limit: Some(100),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["status"], "open");
        assert_eq!(json["side"], "borrow");
        assert_eq!(json["sort_by"], "create_time");
        assert_eq!(json["reverse_sort"], false);
    }

    #[test]
    fn test_list_loans_request_realistic_lending_filter_scenario() {
        // Scenario: List lending loans for USDT
        let request = ListLoansRequest {
            status: None,
            side: Some("lend".to_string()),
            currency: Some("USDT".to_string()),
            currency_pair: None,
            sort_by: Some("rate".to_string()),
            reverse_sort: Some(true),
            page: Some(1),
            limit: Some(50),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "lend");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["sort_by"], "rate");
        assert_eq!(json["reverse_sort"], true);
    }

    #[test]
    fn test_create_loan_request_realistic_short_term_borrow_scenario() {
        // Scenario: Short-term USDT borrowing for BTC trading
        let request = CreateLoanRequest {
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "50000.0".to_string(),
            rate: None,
            days: Some(7),
            auto_renew: Some(false),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "borrow");
        assert_eq!(json["amount"], "50000.0");
        assert_eq!(json["days"], 7);
        assert_eq!(json["auto_renew"], false);
    }

    #[test]
    fn test_create_loan_request_realistic_long_term_lending_scenario() {
        // Scenario: Long-term USDT lending with auto-renew
        let request = CreateLoanRequest {
            side: "lend".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "25000.0".to_string(),
            rate: Some("0.08".to_string()),
            days: Some(90),
            auto_renew: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["side"], "lend");
        assert_eq!(json["rate"], "0.08");
        assert_eq!(json["days"], 90);
        assert_eq!(json["auto_renew"], true);
    }

    #[test]
    fn test_modify_loan_request_realistic_rate_adjustment_scenario() {
        // Scenario: Adjust lending rate to be more competitive
        let request = ModifyLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: None,
            rate: Some("0.06".to_string()),
            auto_renew: Some(true),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["rate"], "0.06");
        assert_eq!(json["auto_renew"], true);

        // amount should be omitted
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
    }

    #[test]
    fn test_loan_realistic_active_borrowing_scenario() {
        let json = r#"{
            "id": "borrow_001",
            "side": "borrow",
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "rate": "0.05",
            "amount": "100000.0",
            "days": 30,
            "auto_renew": false,
            "in_use": "75000.0",
            "left": "25000.0",
            "status": "open",
            "create_time": 1640995200,
            "update_time": 1640995200
        }"#;

        let loan: Loan = serde_json::from_str(json).unwrap();
        assert_eq!(loan.side, "borrow");
        assert_eq!(loan.status, "open");

        // Verify utilization ratio
        let amount: f64 = loan.amount.parse().unwrap();
        let in_use: f64 = loan.in_use.parse().unwrap();
        let left: f64 = loan.left.parse().unwrap();

        assert_eq!(amount, in_use + left);
        let utilization = in_use / amount;
        assert_eq!(utilization, 0.75); // 75% utilization
    }

    #[test]
    fn test_loan_realistic_lending_scenario() {
        let json = r#"{
            "id": "lend_001",
            "side": "lend",
            "currency": "USDT",
            "currency_pair": "ETH_USDT",
            "rate": "0.08",
            "amount": "50000.0",
            "days": 90,
            "auto_renew": true,
            "in_use": "50000.0",
            "left": "0.0",
            "status": "open",
            "create_time": 1640995200,
            "update_time": 1640995200
        }"#;

        let loan: Loan = serde_json::from_str(json).unwrap();
        assert_eq!(loan.side, "lend");
        assert!(loan.auto_renew);

        // Verify fully utilized lending
        let in_use: f64 = loan.in_use.parse().unwrap();
        let left: f64 = loan.left.parse().unwrap();
        assert_eq!(left, 0.0);
        assert!(in_use > 0.0);
    }

    #[test]
    fn test_repay_loan_request_realistic_emergency_repayment_scenario() {
        // Scenario: Emergency full repayment to avoid liquidation
        let request = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "all".to_string(),
            amount: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["mode"], "all");

        // No amount specified for full repayment
        let obj = json.as_object().unwrap();
        assert!(!obj.contains_key("amount"));
    }

    #[test]
    fn test_loan_repayment_realistic_partial_repayment_scenario() {
        let json = r#"{
            "loan_id": "borrow_001",
            "repay_id": "repay_001",
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "principal": "25000.0",
            "interest": "156.25",
            "repay_time": 1641600000
        }"#;

        let repayment: LoanRepayment = serde_json::from_str(json).unwrap();
        assert_eq!(repayment.currency, "USDT");

        // Verify interest calculation (5% annual rate for ~30 days)
        let principal: f64 = repayment.principal.parse().unwrap();
        let interest: f64 = repayment.interest.parse().unwrap();
        let interest_rate = interest / principal;
        assert!((interest_rate - 0.00625).abs() < 0.001); // ~0.625% monthly
    }

    #[test]
    fn test_loan_record_realistic_matched_loan_scenario() {
        let json = r#"{
            "id": "match_001",
            "loan_id": "lend_001",
            "borrower_id": 12345,
            "lender_id": 67890,
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "rate": "0.06",
            "amount": "20000.0",
            "days": 30,
            "status": "active",
            "repaid": "5000.0",
            "paid_interest": "25.0",
            "unpaid_interest": "75.0",
            "create_time": 1640995200,
            "expire_time": 1643587200
        }"#;

        let record: LoanRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.status, "active");
        assert!(record.borrower_id != record.lender_id);

        // Verify outstanding balance
        let amount: f64 = record.amount.parse().unwrap();
        let repaid: f64 = record.repaid.parse().unwrap();
        let outstanding = amount - repaid;
        assert_eq!(outstanding, 15000.0);
    }

    #[test]
    fn test_create_loan_request_different_currencies() {
        let currencies = vec![
            ("BTC", "BTC_USDT"),
            ("ETH", "ETH_USDT"),
            ("USDT", "BTC_USDT"),
            ("USDC", "ETH_USDC"),
            ("BNB", "BNB_USDT"),
        ];

        for (currency, currency_pair) in currencies {
            let request = CreateLoanRequest {
                side: "borrow".to_string(),
                currency: currency.to_string(),
                currency_pair: currency_pair.to_string(),
                amount: "1000.0".to_string(),
                rate: None,
                days: Some(7),
                auto_renew: Some(false),
            };

            let json = serde_json::to_value(&request).unwrap();
            assert_eq!(json["currency"], currency);
            assert_eq!(json["currency_pair"], currency_pair);
        }
    }

    #[test]
    fn test_loan_interest_calculations() {
        let json = r#"{
            "id": "calc_test",
            "side": "borrow",
            "currency": "USDT",
            "currency_pair": "BTC_USDT",
            "rate": "0.1",
            "amount": "10000.0",
            "days": 365,
            "auto_renew": false,
            "in_use": "10000.0",
            "left": "0.0",
            "status": "open",
            "create_time": 1640995200,
            "update_time": 1640995200
        }"#;

        let loan: Loan = serde_json::from_str(json).unwrap();

        // Calculate expected annual interest
        let principal: f64 = loan.amount.parse().unwrap();
        let rate: f64 = loan.rate.parse().unwrap();
        let days = loan.days as f64;

        let annual_interest = principal * rate;
        let daily_interest = annual_interest / 365.0;
        let total_interest = daily_interest * days;

        assert_eq!(annual_interest, 1000.0); // 10% of 10,000
        assert_eq!(total_interest, 1000.0); // Full year
    }

    #[test]
    fn test_list_loans_request_clone() {
        let original = ListLoansRequest {
            status: Some("open".to_string()),
            side: Some("borrow".to_string()),
            currency: Some("BTC".to_string()),
            currency_pair: Some("BTC_USDT".to_string()),
            sort_by: Some("create_time".to_string()),
            reverse_sort: Some(true),
            page: Some(1),
            limit: Some(50),
        };

        let cloned = original.clone();
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.sort_by, original.sort_by);
        assert_eq!(cloned.reverse_sort, original.reverse_sort);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
    }

    #[test]
    fn test_create_loan_request_clone() {
        let original = CreateLoanRequest {
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "10000.0".to_string(),
            rate: Some("0.05".to_string()),
            days: Some(30),
            auto_renew: Some(false),
        };

        let cloned = original.clone();
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.rate, original.rate);
        assert_eq!(cloned.days, original.days);
        assert_eq!(cloned.auto_renew, original.auto_renew);
    }

    #[test]
    fn test_loan_clone() {
        let original = Loan {
            id: "12345678".to_string(),
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            auto_renew: false,
            in_use: "5000.0".to_string(),
            left: "5000.0".to_string(),
            status: "open".to_string(),
            create_time: 1640995200,
            update_time: 1640995300,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.side, original.side);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.rate, original.rate);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.days, original.days);
        assert_eq!(cloned.auto_renew, original.auto_renew);
        assert_eq!(cloned.in_use, original.in_use);
        assert_eq!(cloned.left, original.left);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.create_time, original.create_time);
        assert_eq!(cloned.update_time, original.update_time);
    }

    #[test]
    fn test_repay_loan_request_clone() {
        let original = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "partial".to_string(),
            amount: Some("2500.0".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.mode, original.mode);
        assert_eq!(cloned.amount, original.amount);
    }

    #[test]
    fn test_loan_repayment_clone() {
        let original = LoanRepayment {
            loan_id: "12345678".to_string(),
            repay_id: "87654321".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            principal: "2000.0".to_string(),
            interest: "25.0".to_string(),
            repay_time: 1640995500,
        };

        let cloned = original.clone();
        assert_eq!(cloned.loan_id, original.loan_id);
        assert_eq!(cloned.repay_id, original.repay_id);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.principal, original.principal);
        assert_eq!(cloned.interest, original.interest);
        assert_eq!(cloned.repay_time, original.repay_time);
    }

    #[test]
    fn test_loan_records_request_clone() {
        let original = LoanRecordsRequest {
            loan_record_id: Some("123456".to_string()),
            status: Some("open".to_string()),
            page: Some(1),
            limit: Some(25),
        };

        let cloned = original.clone();
        assert_eq!(cloned.loan_record_id, original.loan_record_id);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.page, original.page);
        assert_eq!(cloned.limit, original.limit);
    }

    #[test]
    fn test_loan_record_clone() {
        let original = LoanRecord {
            id: "record123".to_string(),
            loan_id: "loan456".to_string(),
            borrower_id: 789,
            lender_id: 101112,
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            status: "active".to_string(),
            repaid: "2500.0".to_string(),
            paid_interest: "50.0".to_string(),
            unpaid_interest: "25.0".to_string(),
            create_time: 1640995200,
            expire_time: 1643587200,
        };

        let cloned = original.clone();
        assert_eq!(cloned.id, original.id);
        assert_eq!(cloned.loan_id, original.loan_id);
        assert_eq!(cloned.borrower_id, original.borrower_id);
        assert_eq!(cloned.lender_id, original.lender_id);
        assert_eq!(cloned.currency, original.currency);
        assert_eq!(cloned.currency_pair, original.currency_pair);
        assert_eq!(cloned.rate, original.rate);
        assert_eq!(cloned.amount, original.amount);
        assert_eq!(cloned.days, original.days);
        assert_eq!(cloned.status, original.status);
        assert_eq!(cloned.repaid, original.repaid);
        assert_eq!(cloned.paid_interest, original.paid_interest);
        assert_eq!(cloned.unpaid_interest, original.unpaid_interest);
        assert_eq!(cloned.create_time, original.create_time);
        assert_eq!(cloned.expire_time, original.expire_time);
    }

    #[test]
    fn test_list_loans_request_debug() {
        let request = ListLoansRequest {
            status: Some("open".to_string()),
            side: Some("borrow".to_string()),
            currency: Some("BTC".to_string()),
            currency_pair: Some("BTC_USDT".to_string()),
            sort_by: Some("create_time".to_string()),
            reverse_sort: Some(true),
            page: Some(1),
            limit: Some(50),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("ListLoansRequest"));
        assert!(debug_str.contains("open"));
        assert!(debug_str.contains("borrow"));
        assert!(debug_str.contains("BTC"));
    }

    #[test]
    fn test_loan_debug() {
        let loan = Loan {
            id: "12345678".to_string(),
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            auto_renew: false,
            in_use: "5000.0".to_string(),
            left: "5000.0".to_string(),
            status: "open".to_string(),
            create_time: 1640995200,
            update_time: 1640995300,
        };

        let debug_str = format!("{:?}", loan);
        assert!(debug_str.contains("Loan"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("borrow"));
        assert!(debug_str.contains("open"));
    }

    #[test]
    fn test_loan_repayment_debug() {
        let repayment = LoanRepayment {
            loan_id: "12345678".to_string(),
            repay_id: "87654321".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            principal: "2000.0".to_string(),
            interest: "25.0".to_string(),
            repay_time: 1640995500,
        };

        let debug_str = format!("{:?}", repayment);
        assert!(debug_str.contains("LoanRepayment"));
        assert!(debug_str.contains("12345678"));
        assert!(debug_str.contains("2000.0"));
    }

    #[test]
    fn test_loan_serialization() {
        let loan = Loan {
            id: "12345678".to_string(),
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            auto_renew: false,
            in_use: "5000.0".to_string(),
            left: "5000.0".to_string(),
            status: "open".to_string(),
            create_time: 1640995200,
            update_time: 1640995300,
        };

        let json = serde_json::to_value(&loan).unwrap();
        assert_eq!(json["id"], "12345678");
        assert_eq!(json["side"], "borrow");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["rate"], "0.05");
        assert_eq!(json["amount"], "10000.0");
        assert_eq!(json["days"], 30);
        assert_eq!(json["auto_renew"], false);
        assert_eq!(json["in_use"], "5000.0");
        assert_eq!(json["left"], "5000.0");
        assert_eq!(json["status"], "open");
        assert_eq!(json["create_time"], 1640995200);
        assert_eq!(json["update_time"], 1640995300);
    }

    #[test]
    fn test_loan_repayment_serialization() {
        let repayment = LoanRepayment {
            loan_id: "12345678".to_string(),
            repay_id: "87654321".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            principal: "2000.0".to_string(),
            interest: "25.0".to_string(),
            repay_time: 1640995500,
        };

        let json = serde_json::to_value(&repayment).unwrap();
        assert_eq!(json["loan_id"], "12345678");
        assert_eq!(json["repay_id"], "87654321");
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["principal"], "2000.0");
        assert_eq!(json["interest"], "25.0");
        assert_eq!(json["repay_time"], 1640995500);
    }

    #[test]
    fn test_loan_record_serialization() {
        let record = LoanRecord {
            id: "record123".to_string(),
            loan_id: "loan456".to_string(),
            borrower_id: 789,
            lender_id: 101112,
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            status: "active".to_string(),
            repaid: "2500.0".to_string(),
            paid_interest: "50.0".to_string(),
            unpaid_interest: "25.0".to_string(),
            create_time: 1640995200,
            expire_time: 1643587200,
        };

        let json = serde_json::to_value(&record).unwrap();
        assert_eq!(json["id"], "record123");
        assert_eq!(json["loan_id"], "loan456");
        assert_eq!(json["borrower_id"], 789);
        assert_eq!(json["lender_id"], 101112);
        assert_eq!(json["currency"], "USDT");
        assert_eq!(json["currency_pair"], "BTC_USDT");
        assert_eq!(json["rate"], "0.05");
        assert_eq!(json["amount"], "10000.0");
        assert_eq!(json["days"], 30);
        assert_eq!(json["status"], "active");
        assert_eq!(json["repaid"], "2500.0");
        assert_eq!(json["paid_interest"], "50.0");
        assert_eq!(json["unpaid_interest"], "25.0");
        assert_eq!(json["create_time"], 1640995200);
        assert_eq!(json["expire_time"], 1643587200);
    }

    #[test]
    fn test_create_loan_request_endpoint_validation() {
        let request = CreateLoanRequest {
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "10000.0".to_string(),
            rate: None,
            days: Some(30),
            auto_renew: Some(false),
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("side"));
        assert!(json.as_object().unwrap().contains_key("currency"));
        assert!(json.as_object().unwrap().contains_key("currency_pair"));
        assert!(json.as_object().unwrap().contains_key("amount"));

        // Verify required fields are strings
        assert!(json["side"].is_string());
        assert!(json["currency"].is_string());
        assert!(json["currency_pair"].is_string());
        assert!(json["amount"].is_string());
    }

    #[test]
    fn test_repay_loan_request_endpoint_validation() {
        let request = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "all".to_string(),
            amount: None,
        };

        let json = serde_json::to_value(&request).unwrap();
        assert!(json.as_object().unwrap().contains_key("currency"));
        assert!(json.as_object().unwrap().contains_key("currency_pair"));
        assert!(json.as_object().unwrap().contains_key("mode"));

        // Verify required fields are strings
        assert!(json["currency"].is_string());
        assert!(json["currency_pair"].is_string());
        assert!(json["mode"].is_string());
    }

    #[test]
    fn test_loan_round_trip() {
        let original = Loan {
            id: "12345678".to_string(),
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            auto_renew: false,
            in_use: "5000.0".to_string(),
            left: "5000.0".to_string(),
            status: "open".to_string(),
            create_time: 1640995200,
            update_time: 1640995300,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Loan = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.side, original.side);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.rate, original.rate);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.days, original.days);
        assert_eq!(deserialized.auto_renew, original.auto_renew);
        assert_eq!(deserialized.in_use, original.in_use);
        assert_eq!(deserialized.left, original.left);
        assert_eq!(deserialized.status, original.status);
        assert_eq!(deserialized.create_time, original.create_time);
        assert_eq!(deserialized.update_time, original.update_time);
    }

    #[test]
    fn test_loan_repayment_round_trip() {
        let original = LoanRepayment {
            loan_id: "12345678".to_string(),
            repay_id: "87654321".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            principal: "2000.0".to_string(),
            interest: "25.0".to_string(),
            repay_time: 1640995500,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LoanRepayment = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.loan_id, original.loan_id);
        assert_eq!(deserialized.repay_id, original.repay_id);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.principal, original.principal);
        assert_eq!(deserialized.interest, original.interest);
        assert_eq!(deserialized.repay_time, original.repay_time);
    }

    #[test]
    fn test_loan_record_round_trip() {
        let original = LoanRecord {
            id: "record123".to_string(),
            loan_id: "loan456".to_string(),
            borrower_id: 789,
            lender_id: 101112,
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            rate: "0.05".to_string(),
            amount: "10000.0".to_string(),
            days: 30,
            status: "active".to_string(),
            repaid: "2500.0".to_string(),
            paid_interest: "50.0".to_string(),
            unpaid_interest: "25.0".to_string(),
            create_time: 1640995200,
            expire_time: 1643587200,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LoanRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, original.id);
        assert_eq!(deserialized.loan_id, original.loan_id);
        assert_eq!(deserialized.borrower_id, original.borrower_id);
        assert_eq!(deserialized.lender_id, original.lender_id);
        assert_eq!(deserialized.currency, original.currency);
        assert_eq!(deserialized.currency_pair, original.currency_pair);
        assert_eq!(deserialized.rate, original.rate);
        assert_eq!(deserialized.amount, original.amount);
        assert_eq!(deserialized.days, original.days);
        assert_eq!(deserialized.status, original.status);
        assert_eq!(deserialized.repaid, original.repaid);
        assert_eq!(deserialized.paid_interest, original.paid_interest);
        assert_eq!(deserialized.unpaid_interest, original.unpaid_interest);
        assert_eq!(deserialized.create_time, original.create_time);
        assert_eq!(deserialized.expire_time, original.expire_time);
    }

    #[test]
    fn test_list_loans_request_optional_fields_behavior() {
        // Test with all fields
        let request_full = ListLoansRequest {
            status: Some("open".to_string()),
            side: Some("borrow".to_string()),
            currency: Some("BTC".to_string()),
            currency_pair: Some("BTC_USDT".to_string()),
            sort_by: Some("create_time".to_string()),
            reverse_sort: Some(true),
            page: Some(1),
            limit: Some(50),
        };

        // Test with no fields
        let request_empty = ListLoansRequest::default();

        let json_full = serde_json::to_value(&request_full).unwrap();
        let json_empty = serde_json::to_value(&request_empty).unwrap();

        // Full request should have all fields
        let obj_full = json_full.as_object().unwrap();
        assert_eq!(obj_full.len(), 8);

        // Empty request should have no fields
        let obj_empty = json_empty.as_object().unwrap();
        assert_eq!(obj_empty.len(), 0);
    }

    #[test]
    fn test_create_loan_request_optional_fields_behavior() {
        // Test borrowing request (no rate)
        let borrow_request = CreateLoanRequest {
            side: "borrow".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "10000.0".to_string(),
            rate: None,
            days: Some(30),
            auto_renew: Some(false),
        };

        // Test lending request (with rate)
        let lend_request = CreateLoanRequest {
            side: "lend".to_string(),
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: "10000.0".to_string(),
            rate: Some("0.05".to_string()),
            days: Some(30),
            auto_renew: Some(false),
        };

        let json_borrow = serde_json::to_value(&borrow_request).unwrap();
        let json_lend = serde_json::to_value(&lend_request).unwrap();

        // Borrow request should omit rate
        let obj_borrow = json_borrow.as_object().unwrap();
        assert!(!obj_borrow.contains_key("rate"));
        // Fields: side, currency, currency_pair, amount, days, auto_renew (6 total)
        assert_eq!(obj_borrow.len(), 6);

        // Lend request should include rate
        let obj_lend = json_lend.as_object().unwrap();
        assert!(obj_lend.contains_key("rate"));
        // Fields: side, currency, currency_pair, amount, rate, days, auto_renew (7 total)
        assert_eq!(obj_lend.len(), 7); // All fields
    }

    #[test]
    fn test_modify_loan_request_optional_fields_behavior() {
        // Test with only amount modification
        let amount_only = ModifyLoanRequest {
            currency: "BTC".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            amount: Some("0.5".to_string()),
            rate: None,
            auto_renew: None,
        };

        // Test with only rate modification
        let rate_only = ModifyLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "ETH_USDT".to_string(),
            amount: None,
            rate: Some("0.08".to_string()),
            auto_renew: None,
        };

        let json_amount = serde_json::to_value(&amount_only).unwrap();
        let json_rate = serde_json::to_value(&rate_only).unwrap();

        // Amount-only modification
        let obj_amount = json_amount.as_object().unwrap();
        assert!(obj_amount.contains_key("amount"));
        assert!(!obj_amount.contains_key("rate"));
        assert!(!obj_amount.contains_key("auto_renew"));
        assert_eq!(obj_amount.len(), 3); // currency, currency_pair, amount

        // Rate-only modification
        let obj_rate = json_rate.as_object().unwrap();
        assert!(!obj_rate.contains_key("amount"));
        assert!(obj_rate.contains_key("rate"));
        assert!(!obj_rate.contains_key("auto_renew"));
        assert_eq!(obj_rate.len(), 3); // currency, currency_pair, rate
    }

    #[test]
    fn test_repay_loan_request_optional_amount_behavior() {
        // Test full repayment (no amount)
        let full_repay = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "all".to_string(),
            amount: None,
        };

        // Test partial repayment (with amount)
        let partial_repay = RepayLoanRequest {
            currency: "USDT".to_string(),
            currency_pair: "BTC_USDT".to_string(),
            mode: "partial".to_string(),
            amount: Some("2500.0".to_string()),
        };

        let json_full = serde_json::to_value(&full_repay).unwrap();
        let json_partial = serde_json::to_value(&partial_repay).unwrap();

        // Full repayment should omit amount
        let obj_full = json_full.as_object().unwrap();
        assert!(!obj_full.contains_key("amount"));
        assert_eq!(obj_full.len(), 3); // currency, currency_pair, mode

        // Partial repayment should include amount
        let obj_partial = json_partial.as_object().unwrap();
        assert!(obj_partial.contains_key("amount"));
        assert_eq!(obj_partial.len(), 4); // All fields
    }
}
