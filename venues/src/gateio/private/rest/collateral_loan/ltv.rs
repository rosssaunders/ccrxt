use serde::{Deserialize, Serialize};

use super::{RestClient, RestResult};

const LTV_ENDPOINT: &str = "/loan/collateral/ltv";

/// Query parameters for user's collateralization ratio and remaining borrowable currencies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanLtvRequest {
    /// Collateral currency (required)
    pub collateral_currency: String,

    /// Borrowed currency (required)
    pub borrow_currency: String,
}

/// Response for user's collateralization ratio and remaining borrowable currencies.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CollateralLoanLtvResponse {
    /// Collateral currency
    pub collateral_currency: String,

    /// Borrowed currency
    pub borrow_currency: String,

    /// Initial collateralization rate
    pub init_ltv: String,

    /// Warning collateralization rate
    pub alert_ltv: String,

    /// Liquidation collateralization rate
    pub liquidate_ltv: String,

    /// Minimum borrowable amount for the loan currency
    pub min_borrow_amount: String,

    /// Remaining borrowable amount for the loan currency
    pub left_borrowable_amount: String,
}

impl RestClient {
    /// Query user's collateralization ratio and remaining borrowable currencies
    ///
    /// Returns user's collateralization ratio and remaining borrowable currencies for a given pair.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-collateralization-ratio-and-remaining-borrowable-currencies)
    ///
    /// # Arguments
    /// * `request` - Query parameters for LTV info
    ///
    /// # Returns
    /// LTV info response
    pub async fn get_collateral_loan_ltv_info(
        &self,
        request: CollateralLoanLtvRequest,
    ) -> RestResult<CollateralLoanLtvResponse> {
        self.send_get_request(LTV_ENDPOINT, Some(&request)).await
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_serialize_request() {
        let req = CollateralLoanLtvRequest {
            collateral_currency: "BTC".to_string(),
            borrow_currency: "USDT".to_string(),
        };
        let qs = serde_urlencoded::to_string(&req).unwrap();
        assert!(qs.contains("collateral_currency=BTC"));
        assert!(qs.contains("borrow_currency=USDT"));
    }

    #[test]
    fn test_deserialize_response() {
        let json = r#"{
            "collateral_currency": "BTC",
            "borrow_currency": "USDT",
            "init_ltv": "0.5",
            "alert_ltv": "0.6",
            "liquidate_ltv": "0.8",
            "min_borrow_amount": "100",
            "left_borrowable_amount": "500"
        }"#;
        let resp: CollateralLoanLtvResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.collateral_currency, "BTC");
        assert_eq!(resp.borrow_currency, "USDT");
        assert_eq!(resp.init_ltv, "0.5");
        assert_eq!(resp.left_borrowable_amount, "500");
    }
}
