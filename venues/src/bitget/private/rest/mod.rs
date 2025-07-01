mod client;
mod get_account_assets;
mod spot;

// Re-export client
pub use client::RestClient;
// Re-export endpoints
pub use get_account_assets::{AssetInfo, GetAccountAssetsRequest, GetAccountAssetsResponse};
// Re-export spot trading endpoints
// Removed unused pub use spot::* statement
