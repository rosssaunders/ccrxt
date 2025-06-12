//! Request and response structs for public/get-risk-parameters endpoint
//!
//! Provides information on risk parameter settings for Smart Cross Margin.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Response for public/get-risk-parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRiskParametersResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: RiskParametersResult,
}

/// Result data for risk parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskParametersResult {
    /// Default max product leverage for margin trading unless specified in base_currency_config array
    pub default_max_product_leverage_for_spot: Option<Cow<'static, str>>,

    /// Default max product leverage for perpetuals unless specified in base_currency_config array
    pub default_max_product_leverage_for_perps: Option<Cow<'static, str>>,

    /// Default max product leverage for futures unless specified in base_currency_config array
    pub default_max_product_leverage_for_futures: Option<Cow<'static, str>>,

    /// Default additional margin rate / haircut rate for holding 1 unit of positions unless specified in base_currency_config array
    pub default_unit_margin_rate: Option<Cow<'static, str>>,

    /// Refer to specified collateral cap for each token in base_currency_config array. Field is omitted if the token is not eligible as collateral
    pub default_collateral_cap: Option<Cow<'static, str>>,

    /// Last update time (ms)
    pub update_timestamp_ms: i64,

    /// Specific risk parameters for base tokens
    pub base_currency_config: Vec<BaseCurrencyConfig>,
}

/// Base currency config for risk parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCurrencyConfig {
    /// Instrument name
    pub instrument_name: Cow<'static, str>,

    /// The maximum $notional that is counted towards the margin balance. Omitted if not eligible as collateral
    pub collateral_cap_notional: Option<Cow<'static, str>>,

    /// Minimum haircut rate. Omitted if not eligible as collateral
    pub minimum_haircut: Option<Cow<'static, str>>,

    /// The max product leverage for margin trading on this token
    pub max_product_leverage_for_spot: Option<Cow<'static, str>>,

    /// The max product leverage for perpetuals on this base token
    pub max_product_leverage_for_perps: Option<Cow<'static, str>>,

    /// The max product leverage for futures on this base token
    pub max_product_leverage_for_futures: Option<Cow<'static, str>>,

    /// The additional margin rate / haircut rate for holding 1 unit of positions with this base token
    pub unit_margin_rate: Option<Cow<'static, str>>,

    /// Max negative asset balance user can hold on the base token. Omitted if no short sell permitted
    pub max_short_sell_limit: Option<Cow<'static, str>>,

    /// Max spot order notional user can place in rolling 24-hour window. Omitted if unlimited
    pub daily_notional_limit: Option<Cow<'static, str>>,

    /// Max $notional per spot order on this base token
    pub order_limit: Option<Cow<'static, str>>,

    /// Max $notional per spot order on this base token
    pub max_order_notional_usd: Option<Cow<'static, str>>,

    /// Min $notional per spot order on this base token
    pub min_order_notional_usd: Option<Cow<'static, str>>,
}
