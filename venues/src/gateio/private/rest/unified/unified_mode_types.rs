use serde::{Deserialize, Serialize};

/// Request to set unified account mode
#[derive(Debug, Clone, Serialize, Default)]
pub struct UnifiedModeRequest {
    /// Whether to enable unified account mode
    pub unified: bool,
}

/// Response for unified account mode operations
#[derive(Debug, Clone, Deserialize)]
pub struct UnifiedModeResponse {
    /// User ID
    pub user_id: i64,

    /// Whether unified account mode is enabled
    pub unified: bool,
}
