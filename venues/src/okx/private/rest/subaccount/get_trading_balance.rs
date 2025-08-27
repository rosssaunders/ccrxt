use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting sub-account trading balance
const GET_TRADING_BALANCE_ENDPOINT: &str = "api/v5/account/subaccount/balances";

/// Request to get sub-account trading balance
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTradingBalanceRequest {
    /// Sub-account name
    pub sub_acct: String,
}

/// Sub-account trading balance information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalance {
    /// Update time of account information, millisecond format of Unix timestamp
    pub u_time: String,

    /// The total amount of equity in USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_eq: Option<String>,

    /// Isolated margin equity in USD
    /// Applicable to Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_eq: Option<String>,

    /// Adjusted / Effective equity in USD
    /// The net fiat value of the assets in the account that can provide margins for spot,
    /// expiry futures, perpetual futures and options under the cross-margin mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adj_eq: Option<String>,

    /// Account level available equity, excluding currencies that are restricted due to
    /// the collateralized borrowing limit.
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_eq: Option<String>,

    /// Cross margin frozen for pending orders in USD
    /// Only applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_froz: Option<String>,

    /// Initial margin requirement in USD
    /// The sum of initial margins of all open positions and pending orders under cross-margin mode in USD.
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,

    /// Maintenance margin requirement in USD
    /// The sum of maintenance margins of all open positions and pending orders under cross-margin mode in USD.
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmr: Option<String>,

    /// Potential borrowing IMR of the account in USD
    /// Only applicable to Spot mode/Multi-currency margin/Portfolio margin. It is "" for other margin modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_froz: Option<String>,

    /// Maintenance margin ratio in USD
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,

    /// Notional value of positions in USD
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_usd: Option<String>,

    /// Notional value for Borrow in USD
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_usd_for_borrow: Option<String>,

    /// Notional value of positions for Perpetual Futures in USD
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_usd_for_swap: Option<String>,

    /// Notional value of positions for Expiry Futures in USD
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_usd_for_futures: Option<String>,

    /// Notional value of positions for Option in USD
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_usd_for_option: Option<String>,

    /// Cross-margin info of unrealized profit and loss at the account level in USD
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,

    /// Detailed asset information in all currencies
    pub details: Vec<TradingBalanceDetail>,
}

/// Detailed trading balance for a specific currency
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingBalanceDetail {
    /// Currency
    pub ccy: String,

    /// Equity of currency
    pub eq: String,

    /// Cash balance
    pub cash_bal: String,

    /// Update time of currency balance information, Unix timestamp format in milliseconds
    pub u_time: String,

    /// Isolated margin equity of currency
    /// Applicable to Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_eq: Option<String>,

    /// Available equity of currency
    /// Applicable to Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_eq: Option<String>,

    /// Discount equity of currency in USD
    pub dis_eq: String,

    /// Frozen balance for Dip Sniper and Peak Sniper
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_bal: Option<String>,

    /// Available balance of currency
    pub avail_bal: String,

    /// Frozen balance of currency
    pub frozen_bal: String,

    /// Margin frozen for open orders
    /// Applicable to Spot mode/Futures mode/Multi-currency margin
    pub ord_frozen: String,

    /// Liabilities of currency
    /// It is a positive value, e.g. 21625.64
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liab: Option<String>,

    /// The sum of the unrealized profit & loss of all margin and derivatives positions of currency.
    /// Applicable to Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,

    /// Liabilities due to Unrealized loss of currency
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upl_liab: Option<String>,

    /// Cross liabilities of currency
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_liab: Option<String>,

    /// Trial fund balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_bal: Option<String>,

    /// Isolated liabilities of currency
    /// Applicable to Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_liab: Option<String>,

    /// Cross Maintenance margin ratio of currency
    /// The index for measuring the risk of a certain asset in the account.
    /// Applicable to Futures mode and when there is cross position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,

    /// Cross initial margin requirement at the currency level
    /// Applicable to Futures mode and when there is cross position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,

    /// Cross maintenance margin requirement at the currency level
    /// Applicable to Futures mode and when there is cross position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmr: Option<String>,

    /// Accrued interest of currency
    /// It is a positive value, e.g. 9.01
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,

    /// Risk indicator of auto liability repayment
    /// Divided into multiple levels from 0 to 5, the larger the number, the more likely the auto repayment will be triggered.
    /// Applicable to Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twap: Option<String>,

    /// Max loan of currency
    /// Applicable to cross of Spot mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_loan: Option<String>,

    /// Equity in USD of currency
    pub eq_usd: String,

    /// Potential borrowing IMR of currency in USD
    /// Applicable to Multi-currency margin/Portfolio margin. It is "" for other margin modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_froz: Option<String>,

    /// Leverage of currency
    /// Applicable to Futures mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_lever: Option<String>,

    /// Strategy equity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stgy_eq: Option<String>,

    /// Isolated unrealized profit and loss of currency
    /// Applicable to Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_upl: Option<String>,

    /// Spot in use amount
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_in_use_amt: Option<String>,

    /// User-defined spot risk offset amount
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_spot_in_use_amt: Option<String>,

    /// Max possible spot risk offset amount
    /// Applicable to Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_spot_in_use: Option<String>,

    /// Spot isolated balance
    /// Applicable to copy trading
    /// Applicable to Spot mode/Futures mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_iso_bal: Option<String>,

    /// Smart sync equity
    /// The default is "0", only applicable to copy trader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smt_sync_eq: Option<String>,

    /// Spot smart sync equity.
    /// The default is "0", only applicable to copy trader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_copy_trading_eq: Option<String>,

    /// Spot balance. The unit is currency, e.g. BTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_bal: Option<String>,

    /// Spot average cost price. The unit is USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_avg_px: Option<String>,

    /// Spot accumulated cost price. The unit is USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc_avg_px: Option<String>,

    /// Spot unrealized profit and loss. The unit is USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_upl: Option<String>,

    /// Spot unrealized profit and loss ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_upl_ratio: Option<String>,

    /// Spot accumulated profit and loss. The unit is USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,

    /// Spot accumulated profit and loss ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pnl_ratio: Option<String>,

    /// Platform level collateral restriction status
    /// 0: The restriction is not enabled.
    /// 1: The restriction is not enabled. But the crypto is close to the platform's collateral limit.
    /// 2: The restriction is enabled. This crypto can't be used as margin for your new orders. This may result in failed orders. But it will still be included in the account's adjusted equity and doesn't impact margin ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_res: Option<String>,

    /// Risk indicator of auto conversion.
    /// Divided into multiple levels from 1-5, the larger the number, the more likely the repayment will be triggered.
    /// The default will be 0, indicating there is no risk currently. 5 means this user is undergoing auto conversion now,
    /// 4 means this user will undergo auto conversion soon whereas 1/2/3 indicates there is a risk for auto conversion.
    /// Applicable to Spot mode/Futures mode/Multi-currency margin/Portfolio margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub col_borr_auto_conversion: Option<String>,

    /// Platform level collateralized borrow restriction (deprecated, use colRes instead)
    /// true/false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_restrict: Option<bool>,

    /// Collateral enabled
    /// true: Collateral enabled, false: Collateral disabled
    /// Applicable to Multi-currency margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_enabled: Option<bool>,

    /// Auto lend status
    /// unsupported: auto lend is not supported by this currency
    /// off: auto lend is supported but turned off
    /// pending: auto lend is turned on but pending matching
    /// active: auto lend is turned on and matched
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_status: Option<String>,

    /// Auto lend currency matched amount
    /// Return "0" when autoLendStatus is unsupported/off/pending. Return matched amount when autoLendStatus is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lend_mt_amt: Option<String>,
}

impl RestClient {
    /// Get sub-account trading balance
    ///
    /// Query detailed balance info of Trading Account of a sub-account via the master account
    /// (applies to master accounts only)
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#sub-account-rest-api-get-sub-account-trading-balance)
    ///
    /// Rate limit: 6 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The trading balance request parameters
    ///
    /// # Returns
    /// A result containing the detailed trading balance information for the sub-account
    pub async fn get_trading_balance(
        &self,
        request: GetTradingBalanceRequest,
    ) -> RestResult<TradingBalance> {
        self.send_get_request(
            GET_TRADING_BALANCE_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::ApiResponse;

    #[test]
    fn test_get_trading_balance_request_serialization() {
        let request = GetTradingBalanceRequest {
            sub_acct: "test_sub_001".to_string(),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("subAcct=test_sub_001"));
    }

    #[test]
    fn test_trading_balance_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uTime": "1597026383085",
                    "totalEq": "91.83200000",
                    "isoEq": "0",
                    "adjEq": "91.83200000",
                    "availEq": "91.83200000",
                    "ordFroz": "0",
                    "imr": "0",
                    "mmr": "0",
                    "borrowFroz": "0",
                    "mgnRatio": "",
                    "notionalUsd": "0",
                    "notionalUsdForBorrow": "0",
                    "notionalUsdForSwap": "0",
                    "notionalUsdForFutures": "0",
                    "notionalUsdForOption": "0",
                    "upl": "0",
                    "details": [
                        {
                            "ccy": "BTC",
                            "eq": "1",
                            "cashBal": "1",
                            "uTime": "1597026383085",
                            "isoEq": "0",
                            "availEq": "1",
                            "disEq": "50000",
                            "fixedBal": "0",
                            "availBal": "1",
                            "frozenBal": "0",
                            "ordFrozen": "0",
                            "liab": "0",
                            "upl": "0",
                            "uplLiab": "0",
                            "crossLiab": "0",
                            "rewardBal": "0",
                            "isoLiab": "0",
                            "mgnRatio": "",
                            "imr": "0",
                            "mmr": "0",
                            "interest": "0",
                            "twap": "0",
                            "maxLoan": "10000",
                            "eqUsd": "50000",
                            "borrowFroz": "0",
                            "notionalLever": "0.1",
                            "stgyEq": "0",
                            "isoUpl": "0",
                            "spotInUseAmt": "0",
                            "clSpotInUseAmt": "0",
                            "maxSpotInUse": "0",
                            "spotIsoBal": "0",
                            "smtSyncEq": "0",
                            "spotCopyTradingEq": "0",
                            "spotBal": "1",
                            "openAvgPx": "50000",
                            "accAvgPx": "50000",
                            "spotUpl": "0",
                            "spotUplRatio": "0",
                            "totalPnl": "0",
                            "totalPnlRatio": "0",
                            "colRes": "0",
                            "colBorrAutoConversion": "0",
                            "collateralRestrict": false,
                            "collateralEnabled": true,
                            "autoLendStatus": "off",
                            "autoLendMtAmt": "0"
                        }
                    ]
                }
            ]
        }"#;

        let response: ApiResponse<TradingBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let balance = &response.data[0];
        assert_eq!(balance.u_time, "1597026383085");
        assert_eq!(balance.total_eq, Some("91.83200000".to_string()));
        assert_eq!(balance.details.len(), 1);

        let detail = &balance.details[0];
        assert_eq!(detail.ccy, "BTC");
        assert_eq!(detail.eq, "1");
        assert_eq!(detail.cash_bal, "1");
        assert_eq!(detail.avail_bal, "1");
        assert_eq!(detail.frozen_bal, "0");
        assert_eq!(detail.eq_usd, "50000");
        assert_eq!(detail.spot_bal, Some("1".to_string()));
        assert_eq!(detail.open_avg_px, Some("50000".to_string()));
        assert_eq!(detail.collateral_enabled, Some(true));
        assert_eq!(detail.auto_lend_status, Some("off".to_string()));
    }

    #[test]
    fn test_trading_balance_deserialization_minimal() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "uTime": "1597026383086",
                    "details": [
                        {
                            "ccy": "ETH",
                            "eq": "10",
                            "cashBal": "10",
                            "uTime": "1597026383086",
                            "disEq": "30000",
                            "availBal": "10",
                            "frozenBal": "0",
                            "ordFrozen": "0",
                            "eqUsd": "30000"
                        }
                    ]
                }
            ]
        }"#;

        let response: ApiResponse<TradingBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let balance = &response.data[0];
        assert_eq!(balance.u_time, "1597026383086");
        assert!(balance.total_eq.is_none());
        assert_eq!(balance.details.len(), 1);

        let detail = &balance.details[0];
        assert_eq!(detail.ccy, "ETH");
        assert_eq!(detail.eq, "10");
        assert_eq!(detail.cash_bal, "10");
        assert_eq!(detail.dis_eq, "30000");
        assert_eq!(detail.avail_bal, "10");
        assert_eq!(detail.frozen_bal, "0");
        assert_eq!(detail.ord_frozen, "0");
        assert_eq!(detail.eq_usd, "30000");
        assert!(detail.spot_bal.is_none());
        assert!(detail.collateral_enabled.is_none());
    }
}
