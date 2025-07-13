use serde::{Deserialize, Serialize};

use super::RestClient;

const DELIVERY_POSITIONS_ENDPOINT: &str = "/delivery/{}/positions";
const DELIVERY_POSITION_ENDPOINT: &str = "/delivery/{}/positions/{}";
const DELIVERY_POSITION_LEVERAGE_ENDPOINT: &str = "/delivery/{}/positions/{}/leverage";
const DELIVERY_POSITION_MARGIN_ENDPOINT: &str = "/delivery/{}/positions/{}/margin";
const DELIVERY_POSITION_RISK_LIMIT_ENDPOINT: &str = "/delivery/{}/positions/{}/risk_limit";

/// Request parameters for delivery positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeliveryPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Maximum number of records to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Page offset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Delivery position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size (positive for long, negative for short)
    pub size: i64,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub margin: String,

    /// Leverage
    pub leverage: String,

    /// Risk limit
    pub risk_limit: String,

    /// Liquidation price
    pub liq_price: String,

    /// Bankruptcy price
    pub bankruptcy_price: String,

    /// Cross margin leverage limit
    pub cross_leverage_limit: String,

    /// Position mode
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Request to set delivery leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetDeliveryLeverageRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Delivery leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryLeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Request to update delivery position margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryPositionMarginRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Delivery position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryPositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update delivery risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDeliveryRiskLimitRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract name
    pub contract: String,

    /// Risk limit value
    pub risk_limit: String,
}

/// Delivery risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

impl RestClient {
    /// Get delivery positions
    ///
    /// This endpoint returns all delivery positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `params` - The delivery positions request parameters
    ///
    /// # Returns
    /// List of delivery positions
    pub async fn get_delivery_positions(
        &self,
        params: DeliveryPositionsRequest,
    ) -> crate::gateio::delivery::Result<Vec<DeliveryPosition>> {
        let endpoint = DELIVERY_POSITIONS_ENDPOINT.replace("{}", &params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific delivery position
    ///
    /// This endpoint returns details for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Specific delivery position details
    pub async fn get_delivery_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::delivery::Result<DeliveryPosition> {
        let endpoint = DELIVERY_POSITION_ENDPOINT
            .replace("{}", settle)
            .replace("{}", contract);
        self.get(&endpoint).await
    }

    /// Set delivery position leverage
    ///
    /// This endpoint sets the leverage for a specific delivery contract position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The leverage setting request parameters
    ///
    /// # Returns
    /// Updated leverage information
    pub async fn set_delivery_position_leverage(
        &self,
        request: SetDeliveryLeverageRequest,
    ) -> crate::gateio::delivery::Result<DeliveryLeverageResponse> {
        let endpoint = DELIVERY_POSITION_LEVERAGE_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }

    /// Update delivery position margin
    ///
    /// Adjusts the margin for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The margin update request parameters
    ///
    /// # Returns
    /// Updated position margin information
    pub async fn update_delivery_position_margin(
        &self,
        request: UpdateDeliveryPositionMarginRequest,
    ) -> crate::gateio::delivery::Result<DeliveryPositionMarginResponse> {
        let endpoint = DELIVERY_POSITION_MARGIN_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }

    /// Update delivery position risk limit
    ///
    /// Changes the risk limit for a specific delivery position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 10 requests per second
    ///
    /// # Arguments
    /// * `request` - The risk limit update request parameters
    ///
    /// # Returns
    /// Updated risk limit information
    pub async fn update_delivery_position_risk_limit(
        &self,
        request: UpdateDeliveryRiskLimitRequest,
    ) -> crate::gateio::delivery::Result<DeliveryRiskLimitResponse> {
        let endpoint = DELIVERY_POSITION_RISK_LIMIT_ENDPOINT
            .replace("{}", &request.settle)
            .replace("{}", &request.contract);
        self.post(&endpoint, &request).await
    }
}
