//! Request and response structs for public/get-announcements endpoint
//!
//! Fetches all announcements in Crypto.com Exchange.
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::cryptocom::{AnnouncementCategory, ApiResult, ImpactedStatus, ProductType, RestResult};

/// Endpoint path for the get-announcements API
const ANNOUNCEMENTS_ENDPOINT: &str = "public/get-announcements";

/// Request parameters for the public/get-announcements endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAnnouncementsRequest {
    /// Filter by category: list, delist, event, product, system. Optional.
    /// Must match one of the AnnouncementCategory enum variants.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<AnnouncementCategory>,

    /// Filter by product type. e.g., Spot, Derivative, OTC, Staking, TradingArena etc. Optional.
    /// Must match one of the ProductType enum variants.
    #[serde(rename = "product_type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
}

/// Response for public/get-announcements
pub type GetAnnouncementsResponse = ApiResult<AnnouncementsResult>;

/// Result data for announcements
#[derive(Debug, Clone, Deserialize)]
pub struct AnnouncementsResult {
    /// List of announcements
    #[serde(rename = "data")]
    pub data: Vec<Announcement>,
}

/// Announcement object
#[derive(Debug, Clone, Deserialize)]
pub struct Announcement {
    /// Announcement id
    #[serde(rename = "id")]
    pub id: Cow<'static, str>,

    /// Type of announcement
    #[serde(rename = "category")]
    pub category: AnnouncementCategory,

    /// Type of product
    #[serde(rename = "product_type")]
    pub product_type: ProductType,

    /// Announced timestamp (ms)
    #[serde(rename = "announced_at")]
    pub announced_at: i64,

    /// Title of announcement
    #[serde(rename = "title")]
    pub title: Cow<'static, str>,

    /// Content of announcement
    #[serde(rename = "content")]
    pub content: Cow<'static, str>,

    /// Instrument name (nullable)
    #[serde(rename = "instrument_name")]
    pub instrument_name: Option<Cow<'static, str>>,

    /// Impacted parameters
    #[serde(rename = "impacted_params")]
    pub impacted_params: Option<ImpactedParams>,

    /// Announcements start time timestamp (ms)
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,

    /// Announcements end time timestamp (ms)
    #[serde(rename = "end_time")]
    pub end_time: Option<i64>,
}

/// Impacted parameters for announcement
#[derive(Debug, Clone, Deserialize)]
pub struct ImpactedParams {
    /// Spot trading impacted
    #[serde(rename = "spot_trading_impacted")]
    pub spot_trading_impacted: ImpactedStatus,

    /// Derivative trading impacted
    #[serde(rename = "derivative_trading_impacted")]
    pub derivative_trading_impacted: ImpactedStatus,

    /// Margin trading impacted
    #[serde(rename = "margin_trading_impacted")]
    pub margin_trading_impacted: ImpactedStatus,

    /// OTC trading impacted
    #[serde(rename = "otc_trading_impacted")]
    pub otc_trading_impacted: ImpactedStatus,

    /// Convert impacted
    #[serde(rename = "convert_impacted")]
    pub convert_impacted: ImpactedStatus,

    /// Staking impacted
    #[serde(rename = "staking_impacted")]
    pub staking_impacted: ImpactedStatus,

    /// Trading bot impacted
    #[serde(rename = "trading_bot_impacted")]
    pub trading_bot_impacted: ImpactedStatus,

    /// Crypto wallet impacted
    #[serde(rename = "crypto_wallet_impacted")]
    pub crypto_wallet_impacted: ImpactedStatus,

    /// Fiat wallet impacted
    #[serde(rename = "fiat_wallet_impacted")]
    pub fiat_wallet_impacted: ImpactedStatus,

    /// Login impacted
    #[serde(rename = "login_impacted")]
    pub login_impacted: ImpactedStatus,
}

impl RestClient {
    /// Calls the public/get-announcements endpoint.
    ///
    /// Fetches all announcements in Crypto.com Exchange.
    ///
    /// [Official API docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#public-get-announcements)
    pub async fn get_announcements(
        &self,
        params: GetAnnouncementsRequest,
    ) -> RestResult<GetAnnouncementsResponse> {
        self.send_get_request(
            ANNOUNCEMENTS_ENDPOINT,
            Some(&params),
            crate::cryptocom::EndpointType::PublicGetAnnouncements,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptocom::{AnnouncementCategory, ProductType};

    #[test]
    fn test_announcements_request_building() {
        let req = GetAnnouncementsRequest {
            category: Some(AnnouncementCategory::System),
            product_type: Some(ProductType::Spot),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("category"));
        assert!(json.contains("product_type"));
    }
}
