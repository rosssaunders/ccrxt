use super::{RestClient, RestResult};
use serde::{Deserialize, Serialize};

const DUAL_INVESTMENT_PLAN_ENDPOINT: &str = "/earn/dual/investment_plan";

/// Request parameters for Dual Investment product list.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DualInvestmentPlanRequest {
    /// Financial project ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

/// Represents a single Dual Investment product.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DualInvestmentPlan {
    /// Product ID
    pub id: i32,

    /// Product Name
    pub instrument_name: String,

    /// Investment Token
    pub invest_currency: String,

    /// Strike Token
    pub exercise_currency: String,

    /// Strike price
    pub exercise_price: f64,

    /// Settlement time
    pub delivery_time: i32,

    /// Minimum Units
    pub min_copies: i32,

    /// Maximum Units
    pub max_copies: i32,

    /// Value Per Unit
    pub per_value: String,

    /// Annual Yield
    pub apy_display: String,

    /// Start Time
    pub start_time: i32,

    /// End time
    pub end_time: i32,

    /// Status: NOTSTARTED, ONGOING, ENDED
    pub status: String,
}

impl RestClient {
    /// Dual Investment product list endpoint
    ///
    /// Returns a list of dual investment products.
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#dual-investment-product-list)
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The product list request parameters
    ///
    /// # Returns
    /// List of dual investment products
    pub async fn dual_investment_plan(
        &self,
        request: DualInvestmentPlanRequest,
    ) -> RestResult<Vec<DualInvestmentPlan>> {
        self.send_get_request::<Vec<DualInvestmentPlan>, _>(
            DUAL_INVESTMENT_PLAN_ENDPOINT,
            Some(&request),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dual_investment_plan_request_serialization() {
        let req = DualInvestmentPlanRequest { plan_id: Some(123) };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("plan_id"));
    }

    #[test]
    fn test_dual_investment_plan_deserialization() {
        let json = r#"{
            "id": 1,
            "instrument_name": "BTC-TEST",
            "type": "put",
            "invest_currency": "USDT",
            "exercise_currency": "BTC",
            "exercise_price": 123.45,
            "delivery_time": 123456,
            "min_copies": 1,
            "max_copies": 10,
            "per_value": "1",
            "apy_display": "0.01",
            "start_time": 123456,
            "end_time": 123456,
            "status": "ONGOING"
        }"#;
        let plan: DualInvestmentPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.id, 1);
        assert_eq!(plan.instrument_name, "BTC-TEST");
        assert_eq!(plan.status, "ONGOING");
    }
}
