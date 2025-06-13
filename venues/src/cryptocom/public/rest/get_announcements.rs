//! Request and response structs for public/get-announcements endpoint
//!
//! Fetches all announcements in Crypto.com Exchange.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use crate::crypto_com::enums::{AnnouncementCategory, ProductType, ImpactedStatus};

/// Request for public/get-announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementsRequest {
    /// Filter by category: list, delist, event, product, system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AnnouncementCategory>,

    /// Filter by product type. e.g. Spot, Derivative, OTC, Staking, TradingArena etc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
}

/// Response for public/get-announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementsResponse {
    /// Response id
    pub id: i64,

    /// Method name
    pub method: Cow<'static, str>,

    /// Response code
    pub code: i32,

    /// Result data
    pub result: AnnouncementsResult,
}

/// Result data for announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementsResult {
    /// List of announcements
    pub data: Vec<Announcement>,
}

/// Announcement object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    /// Announcement id
    pub id: Cow<'static, str>,

    /// Type of announcement
    pub category: Cow<'static, str>,

    /// Type of product
    pub product_type: Cow<'static, str>,

    /// Announced timestamp (ms)
    pub announced_at: i64,

    /// Title of announcement
    pub title: Cow<'static, str>,

    /// Content of announcement
    pub content: Cow<'static, str>,

    /// Instrument name (nullable)
    pub instrument_name: Option<Cow<'static, str>>,

    /// Impacted parameters
    pub impacted_params: ImpactedParams,

    /// Announcements start time timestamp (ms)
    pub start_time: i64,

    /// Announcements end time timestamp (ms)
    pub end_time: i64,
}

/// Impacted parameters for announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactedParams {
    /// Spot trading impacted
    pub spot_trading_impacted: Cow<'static, str>,

    /// Derivative trading impacted
    pub derivative_trading_impacted: Cow<'static, str>,

    /// Margin trading impacted
    pub margin_trading_impacted: Cow<'static, str>,

    /// OTC trading impacted
    pub otc_trading_impacted: Cow<'static, str>,

    /// Convert impacted
    pub convert_impacted: Cow<'static, str>,

    /// Staking impacted
    pub staking_impacted: Cow<'static, str>,

    /// Trading bot impacted
    pub trading_bot_impacted: Cow<'static, str>,

    /// Crypto wallet impacted
    pub crypto_wallet_impacted: Cow<'static, str>,

    /// Fiat wallet impacted
    pub fiat_wallet_impacted: Cow<'static, str>,

    /// Login impacted
    pub login_impacted: Cow<'static, str>,
}
