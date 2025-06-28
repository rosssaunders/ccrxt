use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::bybit::{AccountType, EndpointType, RestResult};

/// Request for getting wallet balance from ByBit V5 API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletBalanceRequest {
    /// Account type to query balance for
    #[serde(rename = "accountType")]
    pub account_type: AccountType,
    /// Coin to query balance for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetWalletBalanceRequest {
    /// Create a new wallet balance request
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    /// Set the coin filter
    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }
}

/// Individual balance data for a coin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceData {
    /// Coin symbol
    pub coin: String,
    /// Available balance (equity that can be used for trading)
    #[serde(rename = "availableToWithdraw")]
    pub available_to_withdraw: String,
    /// Equity (total equity, including unrealized PnL)
    pub equity: String,
    /// Total wallet balance
    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,
    /// Borrowed amount
    pub borrowed: String,
    /// Available balance
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    /// Accrued interest
    #[serde(rename = "accruedInterest")]
    pub accrued_interest: String,
    /// Total order initial margin
    #[serde(rename = "totalOrderIM")]
    pub total_order_im: String,
    /// Total position initial margin
    #[serde(rename = "totalPositionIM")]
    pub total_position_im: String,
    /// Total position maintenance margin
    #[serde(rename = "totalPositionMM")]
    pub total_position_mm: String,
    /// Unrealized profit and loss
    #[serde(rename = "unrealisedPnl")]
    pub unrealised_pnl: String,
    /// Cumulative realized profit and loss
    #[serde(rename = "cumRealisedPnl")]
    pub cum_realised_pnl: String,
    /// Bonus amount
    pub bonus: String,
    /// Collateral switch (for portfolio margin mode)
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    /// Margin collateral amount
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    /// Whether spot hedging is enabled
    #[serde(rename = "spotHedgingQty")]
    pub spot_hedging_qty: String,
}

/// Wallet balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Account type
    #[serde(rename = "accountType")]
    pub account_type: String,
    /// List of coin balances
    pub coin: Vec<BalanceData>,
    /// Total equity in USDT
    #[serde(rename = "totalEquity")]
    pub total_equity: String,
    /// Total wallet balance in USDT
    #[serde(rename = "totalWalletBalance")]
    pub total_wallet_balance: String,
    /// Total margin balance in USDT
    #[serde(rename = "totalMarginBalance")]
    pub total_margin_balance: String,
    /// Total available balance in USDT
    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,
    /// Total perpetual unrealized PnL in USDT
    #[serde(rename = "totalPerpUPL")]
    pub total_perp_upl: String,
    /// Total initial margin in USDT
    #[serde(rename = "totalInitialMargin")]
    pub total_initial_margin: String,
    /// Total maintenance margin in USDT
    #[serde(rename = "totalMaintenanceMargin")]
    pub total_maintenance_margin: String,
    /// Account maintenance margin rate
    #[serde(rename = "accountMMRate")]
    pub account_mm_rate: String,
    /// Account initial margin rate
    #[serde(rename = "accountIMRate")]
    pub account_im_rate: String,
    /// Account LTV
    #[serde(rename = "accountLTV")]
    pub account_ltv: String,
}

/// Response for get wallet balance endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletBalanceResponse {
    /// Response code (0 for success)
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    /// Response message
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    /// Extended response information
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: serde_json::Value,
    /// Response data
    pub result: WalletBalanceResult,
    /// Response timestamp
    pub time: u64,
}

/// Result data for wallet balance response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalanceResult {
    /// List of wallet balances (typically one per account type)
    pub list: Vec<WalletBalance>,
}

impl RestClient {
    /// Get wallet balance for a specific account type
    ///
    /// This endpoint retrieves the wallet balance information for the specified account type.
    /// Rate limit varies by account type:
    /// - SPOT: 20/s
    /// - CONTRACT: 10/s or 50/s depending on account configuration
    /// - UNIFIED: 50/s
    ///
    /// # Arguments
    /// * `request` - The wallet balance request containing account type and optional coin filter
    ///
    /// # Returns
    /// A result containing the wallet balance response or an error
    pub async fn get_wallet_balance(
        &self,
        request: GetWalletBalanceRequest,
    ) -> RestResult<GetWalletBalanceResponse> {
        self.send_signed_request(
            "/v5/account/wallet-balance",
            reqwest::Method::GET,
            request,
            EndpointType::Account,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wallet_balance_request_structure() {
        let request = GetWalletBalanceRequest::new(AccountType::Spot);
        assert_eq!(request.account_type, AccountType::Spot);
        assert!(request.coin.is_none());
    }

    #[test]
    fn test_get_wallet_balance_request_with_coin() {
        let request =
            GetWalletBalanceRequest::new(AccountType::Unified).with_coin("BTC".to_string());
        assert_eq!(request.account_type, AccountType::Unified);
        assert_eq!(request.coin, Some("BTC".to_string()));
    }

    #[test]
    fn test_get_wallet_balance_request_serialization() {
        let request = GetWalletBalanceRequest::new(AccountType::Contract);
        let serialized = serde_urlencoded::to_string(&request).unwrap_or_else(|e| {
            eprintln!("Failed to serialize GetWalletBalanceRequest: {}", e);
            String::new()
        });
        assert!(serialized.contains("accountType=CONTRACT"));
    }

    #[test]
    fn test_balance_data_structure() {
        let balance_data = BalanceData {
            coin: "BTC".to_string(),
            available_to_withdraw: "1.5".to_string(),
            equity: "1.5".to_string(),
            wallet_balance: "1.5".to_string(),
            borrowed: "0".to_string(),
            available_to_borrow: "0".to_string(),
            accrued_interest: "0".to_string(),
            total_order_im: "0".to_string(),
            total_position_im: "0".to_string(),
            total_position_mm: "0".to_string(),
            unrealised_pnl: "0".to_string(),
            cum_realised_pnl: "0".to_string(),
            bonus: "0".to_string(),
            collateral_switch: true,
            margin_collateral: false,
            spot_hedging_qty: "0".to_string(),
        };

        assert_eq!(balance_data.coin, "BTC");
        assert_eq!(balance_data.available_to_withdraw, "1.5");
    }

    #[test]
    fn test_wallet_balance_response_structure() {
        let response_json = r#"
        {
            "retCode": 0,
            "retMsg": "OK",
            "retExtInfo": {},
            "result": {
                "list": [
                    {
                        "accountType": "UNIFIED",
                        "coin": [
                            {
                                "coin": "BTC",
                                "availableToWithdraw": "1.5",
                                "equity": "1.5",
                                "walletBalance": "1.5",
                                "borrowed": "0",
                                "availableToBorrow": "0",
                                "accruedInterest": "0",
                                "totalOrderIM": "0",
                                "totalPositionIM": "0",
                                "totalPositionMM": "0",
                                "unrealisedPnl": "0",
                                "cumRealisedPnl": "0",
                                "bonus": "0",
                                "collateralSwitch": true,
                                "marginCollateral": false,
                                "spotHedgingQty": "0"
                            }
                        ],
                        "totalEquity": "50000",
                        "totalWalletBalance": "50000",
                        "totalMarginBalance": "50000",
                        "totalAvailableBalance": "50000",
                        "totalPerpUPL": "0",
                        "totalInitialMargin": "0",
                        "totalMaintenanceMargin": "0",
                        "accountMMRate": "0",
                        "accountIMRate": "0",
                        "accountLTV": "0"
                    }
                ]
            },
            "time": 1672738134824
        }
        "#;

        let response: GetWalletBalanceResponse = match serde_json::from_str(response_json) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to deserialize GetWalletBalanceResponse: {}", e);
                return;
            }
        };
        assert_eq!(response.ret_code, 0);
        assert_eq!(response.ret_msg, "OK");
        assert_eq!(response.result.list.len(), 1);
        assert_eq!(
            response.result.list.first().map(|a| &a.account_type),
            Some(&"UNIFIED".to_string())
        );
        assert_eq!(response.result.list.first().map(|a| a.coin.len()), Some(1));
        assert_eq!(
            response
                .result
                .list
                .first()
                .and_then(|a| a.coin.first())
                .map(|c| &c.coin),
            Some(&"BTC".to_string())
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let request = GetWalletBalanceRequest::new(AccountType::Spot).with_coin("USDT".to_string());

        let serialized = serde_json::to_string(&request).unwrap_or_else(|e| {
            eprintln!("Serialization failed: {}", e);
            String::new()
        });
        let deserialized: GetWalletBalanceRequest = match serde_json::from_str(&serialized) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Deserialization failed: {}", e);
                return;
            }
        };

        assert_eq!(request.account_type, deserialized.account_type);
        assert_eq!(request.coin, deserialized.coin);
    }
}
