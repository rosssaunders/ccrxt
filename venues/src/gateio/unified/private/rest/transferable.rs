use serde::{Deserialize, Serialize};

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
