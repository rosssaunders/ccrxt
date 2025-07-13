use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for futures positions
#[derive(Debug, Clone, Serialize, Default)]
pub struct FuturesPositionsRequest {
    /// Settlement currency
    pub settle: String,

    /// Contract filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,

    /// Hold mode (0: both, 1: long only, 2: short only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holding: Option<i32>,

    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Futures position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPosition {
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

    /// Cross margin mode
    pub cross_leverage_limit: String,

    /// Position mode (single or dual)
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Request to set leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetLeverageRequest {
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

/// Leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageResponse {
    /// Leverage value
    pub leverage: String,

    /// Cross margin leverage limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_leverage_limit: Option<String>,
}

/// Request to update position margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdatePositionMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount (positive to add, negative to remove)
    pub change: String,
}

/// Position margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
}

/// Risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

/// Request to set cross margin mode
#[derive(Debug, Clone, Serialize)]
pub struct CrossModeRequest {
    /// Mode ("cross" for cross margin)
    pub mode: String,
}

/// Request to enable/disable dual mode
#[derive(Debug, Clone, Serialize)]
pub struct DualModeRequest {
    /// Settlement currency
    pub settle: String,
    /// Enable dual mode
    pub dual_mode: bool,
}

/// Dual mode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeResponse {
    /// Whether dual mode is enabled
    pub dual_mode: bool,
}

/// Position information in dual mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModePosition {
    /// User ID
    pub user: i64,
    /// Contract name
    pub contract: String,
    /// Long position size
    pub long_size: i64,
    /// Short position size
    pub short_size: i64,
    /// Long position entry price
    pub long_entry_price: String,
    /// Short position entry price
    pub short_entry_price: String,
    /// Long position leverage
    pub long_leverage: String,
    /// Short position leverage
    pub short_leverage: String,
    /// Long position margin
    pub long_margin: String,
    /// Short position margin
    pub short_margin: String,
    /// Long position PnL
    pub long_pnl: String,
    /// Short position PnL
    pub short_pnl: String,
    /// Mark price
    pub mark_price: String,
}

/// Request to update dual mode margin
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeMarginRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Change amount
    pub change: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode margin response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeMarginResponse {
    /// New margin amount
    pub margin: String,
}

/// Request to update dual mode leverage
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeLeverageRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Leverage value
    pub leverage: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode leverage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeLeverageResponse {
    /// Leverage value
    pub leverage: String,
}

/// Request to update dual mode risk limit
#[derive(Debug, Clone, Serialize)]
pub struct UpdateDualModeRiskLimitRequest {
    /// Settlement currency
    pub settle: String,
    /// Contract name
    pub contract: String,
    /// Risk limit value
    pub risk_limit: String,
    /// Position side ("long" or "short")
    pub side: String,
}

/// Dual mode risk limit response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeRiskLimitResponse {
    /// Risk limit value
    pub risk_limit: String,
}

impl RestClient {
    /// Get futures positions
    ///
    /// This endpoint returns all futures positions for the authenticated user.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#list-all-positions-of-a-user>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `params` - The futures positions request parameters
    ///
    /// # Returns
    /// List of positions
    pub async fn get_futures_positions(
        &self,
        params: FuturesPositionsRequest,
    ) -> crate::gateio::perpetual::Result<Vec<FuturesPosition>> {
        let endpoint = format!("/futures/{}/positions", params.settle);
        self.get_with_query(&endpoint, &params).await
    }

    /// Get a specific futures position
    ///
    /// This endpoint returns details for a specific futures position.
    ///
    /// See: Gate.io API documentation
    /// <https://www.gate.com/docs/developers/apiv4/#get-single-position>
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Position details
    pub async fn get_futures_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::perpetual::Result<FuturesPosition> {
        let endpoint = format!("/futures/{}/positions/{}", settle, contract);
        self.get(&endpoint).await
    }

    /// Set position leverage
    ///
    /// This endpoint sets the leverage for a specific contract position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The leverage request parameters
    ///
    /// # Returns
    /// Leverage response
    pub async fn set_position_leverage(
        &self,
        request: SetLeverageRequest,
    ) -> crate::gateio::perpetual::Result<LeverageResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/leverage",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position margin
    ///
    /// Adjusts the margin for a specific position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The margin update request parameters
    ///
    /// # Returns
    /// Margin response
    pub async fn update_position_margin(
        &self,
        request: UpdatePositionMarginRequest,
    ) -> crate::gateio::perpetual::Result<PositionMarginResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position risk limit
    ///
    /// Changes the risk limit for a specific position.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The risk limit update request parameters
    ///
    /// # Returns
    /// Risk limit response
    pub async fn update_position_risk_limit(
        &self,
        request: UpdateRiskLimitRequest,
    ) -> crate::gateio::perpetual::Result<RiskLimitResponse> {
        let endpoint = format!(
            "/futures/{}/positions/{}/risk_limit",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Switch to cross margin mode
    ///
    /// Switches all positions to cross margin mode.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    ///
    /// # Returns
    /// Empty response indicating success
    pub async fn switch_to_cross_margin(
        &self,
        settle: &str,
    ) -> crate::gateio::perpetual::Result<()> {
        let endpoint = format!("/futures/{}/positions/cross_mode", settle);
        let request = CrossModeRequest {
            mode: "cross".to_string(),
        };
        self.post::<serde_json::Value>(&endpoint, &request).await?;
        Ok(())
    }

    /// Enable or disable dual mode
    ///
    /// Dual mode allows holding both long and short positions of the same contract simultaneously.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode request parameters
    ///
    /// # Returns
    /// Dual mode response
    pub async fn set_dual_mode(
        &self,
        request: DualModeRequest,
    ) -> crate::gateio::perpetual::Result<DualModeResponse> {
        let endpoint = format!("/futures/{}/dual_mode", request.settle);
        self.post(&endpoint, &request).await
    }

    /// Get position detail in dual mode
    ///
    /// Retrieves detailed position information when dual mode is enabled.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `settle` - Settlement currency
    /// * `contract` - Contract name
    ///
    /// # Returns
    /// Dual mode position details
    pub async fn get_dual_mode_position(
        &self,
        settle: &str,
        contract: &str,
    ) -> crate::gateio::perpetual::Result<DualModePosition> {
        let endpoint = format!("/futures/{}/dual_comp/positions/{}", settle, contract);
        self.get(&endpoint).await
    }

    /// Update position margin in dual mode
    ///
    /// Adjusts margin for a specific position in dual mode.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode margin update request parameters
    ///
    /// # Returns
    /// Dual mode margin response
    pub async fn update_dual_mode_position_margin(
        &self,
        request: UpdateDualModeMarginRequest,
    ) -> crate::gateio::perpetual::Result<DualModeMarginResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/margin",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position leverage in dual mode
    ///
    /// Changes leverage for a specific position in dual mode.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode leverage update request parameters
    ///
    /// # Returns
    /// Dual mode leverage response
    pub async fn update_dual_mode_position_leverage(
        &self,
        request: UpdateDualModeLeverageRequest,
    ) -> crate::gateio::perpetual::Result<DualModeLeverageResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/leverage",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }

    /// Update position risk limit in dual mode
    ///
    /// Changes risk limit for a specific position in dual mode.
    ///
    /// See: Gate.io API documentation
    ///
    /// Rate limit: 100 requests per second
    ///
    /// # Arguments
    /// * `request` - The dual mode risk limit update request parameters
    ///
    /// # Returns
    /// Dual mode risk limit response
    pub async fn update_dual_mode_position_risk_limit(
        &self,
        request: UpdateDualModeRiskLimitRequest,
    ) -> crate::gateio::perpetual::Result<DualModeRiskLimitResponse> {
        let endpoint = format!(
            "/futures/{}/dual_comp/positions/{}/risk_limit",
            request.settle, request.contract
        );
        self.post(&endpoint, &request).await
    }
}
