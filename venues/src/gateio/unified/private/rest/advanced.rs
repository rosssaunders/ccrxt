use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for unified mode
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedModeRequest {
    /// Mode (true for unified, false for classic)
    pub unified: bool,
}

/// Unified mode response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedModeResponse {
    /// User ID
    pub user_id: i64,

    /// Unified mode status
    pub unified: bool,
}

/// Request parameters for unified currencies
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedCurrenciesRequest {
    /// Currency filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

/// Unified currency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCurrency {
    /// Currency code
    pub currency: String,

    /// Currency name
    pub name: String,

    /// Delisted status
    pub delisted: bool,

    /// Withdraw disabled
    pub withdraw_disabled: bool,

    /// Withdraw delayed
    pub withdraw_delayed: bool,

    /// Deposit disabled
    pub deposit_disabled: bool,

    /// Trade disabled
    pub trade_disabled: bool,

    /// Fixed rate
    pub fixed_rate: String,

    /// Cross margin supported
    pub cross_margin: bool,

    /// Lendable
    pub lendable: bool,

    /// Borrowable
    pub borrowable: bool,
}

/// Request parameters for unified borrowable
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedBorrowableRequest {
    /// Currency to borrow
    pub currency: String,
}

/// Unified borrowable response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedBorrowableResponse {
    /// Currency
    pub currency: String,

    /// Borrowable amount
    pub borrowable: String,
}

/// Request parameters for batch borrowable
#[derive(Debug, Clone, Serialize)]
pub struct BatchBorrowableRequest {
    /// Currencies to check
    pub currencies: Vec<String>,
}

/// Batch borrowable response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchBorrowableResponse {
    /// Currency
    pub currency: String,

    /// Borrowable amount
    pub borrowable: String,
}

/// Request parameters for transferable
#[derive(Debug, Clone, Serialize)]
pub struct UnifiedTransferableRequest {
    /// Currency to transfer
    pub currency: String,

    /// From account
    pub from: String,

    /// To account
    pub to: String,
}

/// Unified transferable response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTransferableResponse {
    /// Currency
    pub currency: String,

    /// Transferable amount
    pub transferable: String,
}

/// Currency discount tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyDiscountTier {
    /// Currency
    pub currency: String,

    /// Tier level
    pub tier: i32,

    /// Discount rate
    pub discount_rate: String,

    /// Minimum amount for this tier
    pub min_amount: String,

    /// Maximum amount for this tier
    pub max_amount: String,
}

/// Loan margin tier information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanMarginTier {
    /// Currency
    pub currency: String,

    /// Tier level
    pub tier: i32,

    /// Margin rate
    pub margin_rate: String,

    /// Minimum amount
    pub min_amount: String,

    /// Maximum amount
    pub max_amount: String,
}

/// Risk unit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskUnit {
    /// Currency
    pub currency: String,

    /// Spot hedge required
    pub spot_hedge_required: bool,

    /// Futures hedge required
    pub futures_hedge_required: bool,

    /// Options hedge required
    pub options_hedge_required: bool,
}

/// Request parameters for estimate rate
#[derive(Debug, Clone, Serialize)]
pub struct EstimateRateRequest {
    /// Currencies
    pub currencies: Vec<String>,
}

/// Rate estimate response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateEstimate {
    /// Currency
    pub currency: String,

    /// Estimated rate
    pub rate: String,
}

/// Request parameters for historical loan rates
#[derive(Debug, Clone, Serialize, Default)]
pub struct HistoricalLoanRateRequest {
    /// Currency
    pub currency: String,

    /// Start time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,

    /// End time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,

    /// Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Historical loan rate record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalLoanRate {
    /// Timestamp
    pub time: i64,

    /// Currency
    pub currency: String,

    /// Loan rate
    pub rate: String,
}

/// Leverage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageConfig {
    /// Currency
    pub currency: String,

    /// Maximum leverage
    pub max_leverage: String,

    /// Minimum size
    pub min_size: String,

    /// Maximum size
    pub max_size: String,

    /// Maintenance margin rate
    pub maintenance_rate: String,
}

/// Request to set leverage
#[derive(Debug, Clone, Serialize)]
pub struct SetLeverageConfigRequest {
    /// Currency
    pub currency: String,

    /// Leverage
    pub leverage: String,
}

/// Portfolio calculator request
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioCalculatorRequest {
    /// Spot balances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_balances: Option<Vec<BalanceEntry>>,

    /// Futures positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub futures_positions: Option<Vec<PositionEntry>>,

    /// Options positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_positions: Option<Vec<PositionEntry>>,
}

/// Balance entry for portfolio calculation
#[derive(Debug, Clone, Serialize)]
pub struct BalanceEntry {
    /// Currency
    pub currency: String,

    /// Amount
    pub amount: String,
}

/// Position entry for portfolio calculation
#[derive(Debug, Clone, Serialize)]
pub struct PositionEntry {
    /// Contract
    pub contract: String,

    /// Size
    pub size: String,
}

/// Portfolio calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioCalculationResult {
    /// Total balance
    pub total_balance: String,

    /// Total margin
    pub total_margin: String,

    /// Available margin
    pub available_margin: String,

    /// Risk level
    pub risk_level: String,

    /// Maintenance margin
    pub maintenance_margin: String,
}

impl RestClient {
    /// Get unified mode status
    ///
    /// This endpoint returns the current unified mode status.
    pub async fn get_unified_mode(&self) -> crate::gateio::unified::Result<UnifiedModeResponse> {
        self.get("/unified/unified_mode").await
    }

    /// Set unified mode
    ///
    /// This endpoint enables or disables unified account mode.
    pub async fn set_unified_mode(
        &self,
        request: UnifiedModeRequest,
    ) -> crate::gateio::unified::Result<UnifiedModeResponse> {
        self.put("/unified/unified_mode", &request).await
    }

    /// Get unified currencies
    ///
    /// This endpoint returns currency information for unified accounts.
    pub async fn get_unified_currencies(
        &self,
        params: UnifiedCurrenciesRequest,
    ) -> crate::gateio::unified::Result<Vec<UnifiedCurrency>> {
        self.get_with_query("/unified/currencies", &params).await
    }

    /// Get unified borrowable amount
    ///
    /// This endpoint returns the amount that can be borrowed for a currency.
    pub async fn get_unified_borrowable(
        &self,
        params: UnifiedBorrowableRequest,
    ) -> crate::gateio::unified::Result<UnifiedBorrowableResponse> {
        self.get_with_query("/unified/borrowable", &params).await
    }

    /// Get batch borrowable amounts
    ///
    /// This endpoint returns borrowable amounts for multiple currencies.
    pub async fn get_batch_borrowable(
        &self,
        request: BatchBorrowableRequest,
    ) -> crate::gateio::unified::Result<Vec<BatchBorrowableResponse>> {
        self.post("/unified/batch_borrowable", &request).await
    }

    /// Get unified transferable amount
    ///
    /// This endpoint returns the amount that can be transferred between accounts.
    pub async fn get_unified_transferable(
        &self,
        params: UnifiedTransferableRequest,
    ) -> crate::gateio::unified::Result<UnifiedTransferableResponse> {
        self.get_with_query("/unified/transferable", &params).await
    }

    /// Get transferables for all currencies
    ///
    /// This endpoint returns transferable amounts for all currencies.
    pub async fn get_unified_transferables(
        &self,
    ) -> crate::gateio::unified::Result<Vec<UnifiedTransferableResponse>> {
        self.get("/unified/transferables").await
    }

    /// Get currency discount tiers
    ///
    /// This endpoint returns discount tier information for currencies.
    pub async fn get_currency_discount_tiers(
        &self,
    ) -> crate::gateio::unified::Result<Vec<CurrencyDiscountTier>> {
        self.get("/unified/currency_discount_tiers").await
    }

    /// Get loan margin tiers
    ///
    /// This endpoint returns loan margin tier information.
    pub async fn get_loan_margin_tiers(
        &self,
    ) -> crate::gateio::unified::Result<Vec<LoanMarginTier>> {
        self.get("/unified/loan_margin_tiers").await
    }

    /// Get risk units
    ///
    /// This endpoint returns risk unit configuration.
    pub async fn get_risk_units(&self) -> crate::gateio::unified::Result<Vec<RiskUnit>> {
        self.get("/unified/risk_units").await
    }

    /// Get estimated rates
    ///
    /// This endpoint returns estimated borrowing rates for currencies.
    pub async fn get_estimate_rate(
        &self,
        request: EstimateRateRequest,
    ) -> crate::gateio::unified::Result<Vec<RateEstimate>> {
        self.post("/unified/estimate_rate", &request).await
    }

    /// Get historical loan rates
    ///
    /// This endpoint returns historical borrowing rates.
    pub async fn get_history_loan_rate(
        &self,
        params: HistoricalLoanRateRequest,
    ) -> crate::gateio::unified::Result<Vec<HistoricalLoanRate>> {
        self.get_with_query("/unified/history_loan_rate", &params)
            .await
    }

    /// Get leverage configuration
    ///
    /// This endpoint returns leverage configuration for currencies.
    pub async fn get_leverage_user_currency_config(
        &self,
        currency: Option<&str>,
    ) -> crate::gateio::unified::Result<Vec<LeverageConfig>> {
        let mut endpoint = "/unified/leverage/user_currency_config".to_string();
        if let Some(currency) = currency {
            endpoint.push_str(&format!("?currency={}", currency));
        }
        self.get(&endpoint).await
    }

    /// Get current leverage setting
    ///
    /// This endpoint returns the current leverage setting for a currency.
    pub async fn get_leverage_user_currency_setting(
        &self,
        currency: &str,
    ) -> crate::gateio::unified::Result<LeverageConfig> {
        let endpoint = format!(
            "/unified/leverage/user_currency_setting?currency={}",
            currency
        );
        self.get(&endpoint).await
    }

    /// Set leverage for currency
    ///
    /// This endpoint sets the leverage for a specific currency.
    pub async fn set_leverage_user_currency_setting(
        &self,
        request: SetLeverageConfigRequest,
    ) -> crate::gateio::unified::Result<LeverageConfig> {
        self.post("/unified/leverage/user_currency_setting", &request)
            .await
    }

    /// Calculate portfolio metrics
    ///
    /// This endpoint calculates portfolio metrics based on provided positions.
    pub async fn portfolio_calculator(
        &self,
        request: PortfolioCalculatorRequest,
    ) -> crate::gateio::unified::Result<PortfolioCalculationResult> {
        self.post("/unified/portfolio_calculator", &request).await
    }
}
