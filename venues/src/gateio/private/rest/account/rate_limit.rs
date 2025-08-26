use serde::Deserialize;

use super::{RestClient, RestResult};

/// Endpoint path for retrieving user transaction rate limit information.
const ACCOUNT_RATE_LIMIT_ENDPOINT: &str = "/account/rate_limit";

/// Represents a single account rate limit item.
///
/// Contains frequency limit level, fill rate, total fill ratio, update time, and type (e.g., spot, futures).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccountRateLimitItem {
    /// Frequency limit level (see venue docs for details).
    pub tier: String,

    /// Fill rate for the account.
    pub ratio: String,

    /// Total fill ratio of main account.
    pub main_ratio: String,

    /// Update time (as string, usually a timestamp).
    pub updated_at: String,

    /// Type of account (e.g., "spot", "futures").
    #[serde(rename = "type")]
    pub kind: String,
}

impl RestClient {
    /// Get user transaction rate limit information
    ///
    /// Returns a list of account rate limit items for the authenticated user.
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#get-user-transaction-rate-limit-information)
    ///
    /// Rate limit: See venue documentation for details.
    ///
    /// # Returns
    /// * `Vec<AccountRateLimitItem>` - List of account rate limit items.
    pub async fn get_account_rate_limit(&self) -> RestResult<Vec<AccountRateLimitItem>> {
        self.send_get_request(ACCOUNT_RATE_LIMIT_ENDPOINT, Option::<&()>::None)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_rate_limit_item_full() {
        let json = r#"{
            "type": "spot",
            "tier": "1",
            "ratio": "0.5",
            "main_ratio": "0.3",
            "updated_at": "1728230400"
        }"#;
        let item: AccountRateLimitItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.kind, "spot");
        assert_eq!(item.tier, "1");
        assert_eq!(item.ratio, "0.5");
        assert_eq!(item.main_ratio, "0.3");
        assert_eq!(item.updated_at, "1728230400");
    }

    #[test]
    fn test_account_rate_limit_item_minimal() {
        let json = r#"{
            "type": "futures",
            "tier": "2",
            "ratio": "1",
            "main_ratio": "1",
            "updated_at": "0"
        }"#;
        let item: AccountRateLimitItem = serde_json::from_str(json).unwrap();
        assert_eq!(item.kind, "futures");
        assert_eq!(item.tier, "2");
        assert_eq!(item.ratio, "1");
        assert_eq!(item.main_ratio, "1");
        assert_eq!(item.updated_at, "0");
    }
}
