//! Assets endpoint for Bullish Exchange API

use serde::{Deserialize, Serialize};

use super::client::RestClient;
use crate::bullish::{EndpointType, RestResult};

/// Endpoint URL path for assets
const ENDPOINT_PATH: &str = "/trading-api/v1/assets";

/// Endpoint URL path for single asset (with parameter)
const SINGLE_ASSET_ENDPOINT_PATH: &str = "/trading-api/v1/assets/{}";

/// Asset status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AssetStatus {
    Active,
    Inactive,
    Suspended,
}

/// Asset information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// Asset ID
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// Asset symbol
    pub symbol: String,
    /// Asset name
    pub name: String,
    /// Decimal precision for this asset
    pub precision: String,
    /// Minimum balance to earn interest
    #[serde(rename = "minBalanceInterest")]
    pub min_balance_interest: String,
    /// Annual percentage rate
    pub apr: String,
    /// Minimum fee
    #[serde(rename = "minFee")]
    pub min_fee: String,
    /// Maximum borrow amount
    #[serde(rename = "maxBorrow")]
    pub max_borrow: String,
    /// Total offered loan quantity
    #[serde(rename = "totalOfferedLoanQuantity")]
    pub total_offered_loan_quantity: String,
    /// Loan borrowed quantity
    #[serde(rename = "loanBorrowedQuantity")]
    pub loan_borrowed_quantity: String,
    /// Collateral bands
    #[serde(rename = "collateralBands")]
    pub collateral_bands: Vec<CollateralBand>,
    /// Underlying asset information
    #[serde(rename = "underlyingAsset")]
    pub underlying_asset: UnderlyingAsset,
}

/// Collateral band information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralBand {
    /// Collateral percentage
    #[serde(rename = "collateralPercentage")]
    pub collateral_percentage: String,
    /// Band limit in USD
    #[serde(rename = "bandLimitUSD")]
    pub band_limit_usd: String,
}

/// Underlying asset information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnderlyingAsset {
    /// Symbol
    pub symbol: String,
    /// Asset ID
    #[serde(rename = "assetId")]
    pub asset_id: String,
    /// BPM minimum return start
    #[serde(rename = "bpmMinReturnStart")]
    pub bpm_min_return_start: String,
    /// BPM minimum return end
    #[serde(rename = "bpmMinReturnEnd")]
    pub bpm_min_return_end: String,
    /// BPM maximum return start
    #[serde(rename = "bpmMaxReturnStart")]
    pub bpm_max_return_start: String,
    /// BPM maximum return end
    #[serde(rename = "bpmMaxReturnEnd")]
    pub bpm_max_return_end: String,
    /// Market risk floor percentage start
    #[serde(rename = "marketRiskFloorPctStart")]
    pub market_risk_floor_pct_start: String,
    /// Market risk floor percentage end
    #[serde(rename = "marketRiskFloorPctEnd")]
    pub market_risk_floor_pct_end: String,
    /// BPM transition datetime start
    #[serde(rename = "bpmTransitionDateTimeStart")]
    pub bpm_transition_datetime_start: String,
    /// BPM transition datetime end
    #[serde(rename = "bpmTransitionDateTimeEnd")]
    pub bpm_transition_datetime_end: String,
}

/// Network information for crypto assets
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetNetwork {
    /// Network name (e.g., "Ethereum", "Bitcoin")
    pub network: String,
    /// Network display name
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// Whether this network is enabled
    pub enabled: bool,
    /// Contract address (for tokens)
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    /// Minimum deposit amount for this network
    #[serde(rename = "minDeposit")]
    pub min_deposit: String,
    /// Minimum withdrawal amount for this network
    #[serde(rename = "minWithdrawal")]
    pub min_withdrawal: String,
    /// Withdrawal fee for this network
    #[serde(rename = "withdrawalFee")]
    pub withdrawal_fee: String,
    /// Required confirmations for deposits
    #[serde(rename = "depositConfirmations")]
    pub deposit_confirmations: u32,
}

/// Response for assets query
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsResponse {
    /// List of assets
    pub data: Vec<Asset>,
}

/// Single asset response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleAssetResponse {
    /// Asset details
    pub data: Asset,
}

impl RestClient {
    /// Get all assets
    ///
    /// Retrieve information for all assets available on the exchange.
    ///
    /// # Returns
    /// List of all assets with their properties and trading parameters
    pub async fn get_assets(&self) -> RestResult<Vec<Asset>> {
        self.send_request(
            ENDPOINT_PATH,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicAssets,
        )
        .await
    }

    /// Get specific asset by symbol
    ///
    /// Retrieve detailed information for a specific asset.
    ///
    /// # Arguments
    /// * `symbol` - Asset symbol
    ///
    /// # Returns
    /// Detailed asset information including network details and trading parameters
    pub async fn get_asset(&self, symbol: &str) -> RestResult<Asset> {
        let url = SINGLE_ASSET_ENDPOINT_PATH.replace("{}", symbol);

        self.send_request(
            &url,
            reqwest::Method::GET,
            None::<&()>,
            EndpointType::PublicAssets,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_status_serialization() {
        assert_eq!(
            serde_json::to_string(&AssetStatus::Active).unwrap(),
            "\"ACTIVE\""
        );
        assert_eq!(
            serde_json::to_string(&AssetStatus::Inactive).unwrap(),
            "\"INACTIVE\""
        );
        assert_eq!(
            serde_json::to_string(&AssetStatus::Suspended).unwrap(),
            "\"SUSPENDED\""
        );
    }

    #[test]
    fn test_asset_deserialization() {
        let json = r#"{
            "assetId": "BTC",
            "symbol": "BTC",
            "name": "Bitcoin",
            "precision": "8",
            "minBalanceInterest": "0.01",
            "apr": "0.05",
            "minFee": "0.001",
            "maxBorrow": "1000",
            "totalOfferedLoanQuantity": "50000",
            "loanBorrowedQuantity": "25000",
            "collateralBands": [
                {
                    "collateralPercentage": "80",
                    "bandLimitUSD": "100000"
                },
                {
                    "collateralPercentage": "70",
                    "bandLimitUSD": "500000"
                }
            ],
            "underlyingAsset": {
                "symbol": "BTC",
                "assetId": "BTC",
                "bpmMinReturnStart": "0.01",
                "bpmMinReturnEnd": "0.02",
                "bpmMaxReturnStart": "0.05",
                "bpmMaxReturnEnd": "0.10",
                "marketRiskFloorPctStart": "0.01",
                "marketRiskFloorPctEnd": "0.02",
                "bpmTransitionDateTimeStart": "2024-01-01T00:00:00Z",
                "bpmTransitionDateTimeEnd": "2024-12-31T23:59:59Z"
            }
        }"#;

        let asset: Asset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.symbol, "BTC");
        assert_eq!(asset.asset_id, "BTC");
        assert_eq!(asset.name, "Bitcoin");
        assert_eq!(asset.precision, "8");
        assert_eq!(asset.apr, "0.05");
        assert_eq!(asset.min_fee, "0.001");
        assert_eq!(asset.max_borrow, "1000");
        assert_eq!(asset.collateral_bands.len(), 2);
        assert_eq!(asset.collateral_bands[0].collateral_percentage, "80");
        assert_eq!(asset.collateral_bands[0].band_limit_usd, "100000");
        assert_eq!(asset.underlying_asset.symbol, "BTC");
    }

    #[test]
    fn test_assets_response_deserialization() {
        let json = r#"{
            "data": [
                {
                    "assetId": "BTC",
                    "symbol": "BTC",
                    "name": "Bitcoin",
                    "precision": "8",
                    "minBalanceInterest": "0.01",
                    "apr": "0.05",
                    "minFee": "0.001",
                    "maxBorrow": "1000",
                    "totalOfferedLoanQuantity": "50000",
                    "loanBorrowedQuantity": "25000",
                    "collateralBands": [
                        {
                            "collateralPercentage": "80",
                            "bandLimitUSD": "100000"
                        }
                    ],
                    "underlyingAsset": {
                        "symbol": "BTC",
                        "assetId": "BTC",
                        "bpmMinReturnStart": "0.01",
                        "bpmMinReturnEnd": "0.02",
                        "bpmMaxReturnStart": "0.05",
                        "bpmMaxReturnEnd": "0.10",
                        "marketRiskFloorPctStart": "0.01",
                        "marketRiskFloorPctEnd": "0.02",
                        "bpmTransitionDateTimeStart": "2024-01-01T00:00:00Z",
                        "bpmTransitionDateTimeEnd": "2024-12-31T23:59:59Z"
                    }
                }
            ]
        }"#;

        let response: AssetsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].symbol, "BTC");
    }
}
