use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, RestResult};

/// Endpoint URL for getting account balance
const GET_ACCOUNT_BALANCE_ENDPOINT: &str = "api/v5/account/balance";

/// Request to get account balance
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountBalanceRequest {
    /// Currency, e.g. "BTC"
    /// If not provided, all balances will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Account balance details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Update time
    pub u_time: String,

    /// Total equity in USD
    pub total_eq: String,

    /// Isolated margin equity in USD
    /// Applicable to Single-currency margin mode and Multi-currency margin mode
    pub iso_eq: Option<String>,

    /// Adjusted / Effective equity in USD
    /// The net fiat value of the assets in the account
    pub adj_eq: Option<String>,

    /// Order frozen amount
    pub ord_froz: Option<String>,

    /// Initial margin requirement in USD
    /// The sum of initial margins of all open positions and pending orders under the account
    pub imr: Option<String>,

    /// Maintenance margin requirement in USD
    /// The sum of maintenance margins of all open positions under the account
    pub mmr: Option<String>,

    /// Borrowed amount in USD
    pub borrowed: Option<String>,

    /// Interest
    pub interest: Option<String>,

    /// Notional value of positions in USD
    pub notional_usd: Option<String>,

    /// Margin ratio
    pub mgn_ratio: Option<String>,

    /// Balance details for each currency
    pub details: Vec<BalanceDetail>,
}

/// Balance detail for a specific currency
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceDetail {
    /// Currency
    pub ccy: String,

    /// Equity of the currency
    pub eq: String,

    /// Cash balance
    pub cash_bal: String,

    /// Update time
    pub u_time: String,

    /// Isolated margin equity of the currency
    /// Applicable to Single-currency margin mode and Multi-currency margin mode
    pub iso_eq: Option<String>,

    /// Available equity of the currency
    pub avail_eq: Option<String>,

    /// Discount equity of the currency in USD
    pub dis_eq: String,

    /// Available balance of the currency
    pub avail_bal: String,

    /// Frozen balance of the currency
    pub frozen_bal: String,

    /// Order frozen amount
    pub ord_frozen: String,

    /// Liability of the currency
    pub liab: Option<String>,

    /// Unpaid interest of the currency
    pub upl: Option<String>,

    /// Unrealized profit and loss of the currency
    pub upl_liab: Option<String>,

    /// Cross liability of the currency
    pub cross_liab: Option<String>,

    /// Isolated liability of the currency
    pub iso_liab: Option<String>,

    /// Margin ratio of the currency
    pub mgn_ratio: Option<String>,

    /// Interest
    pub interest: Option<String>,

    /// Risk ratio
    pub twap: Option<String>,

    /// Max loan
    pub max_loan: Option<String>,

    /// Equity in USD
    pub eq_usd: String,

    /// Borrowed amount of the currency
    pub borrowed: Option<String>,

    /// Strategy equity
    pub strategy_eq: Option<String>,

    /// Isolated unrealized profit and loss of the currency
    pub iso_upl: Option<String>,

    /// Spot in use amount
    /// Applicable to Multi-currency margin mode
    pub spot_in_use_amt: Option<String>,

    /// Strategy frozen balance
    pub strategy_frozen_bal: Option<String>,

    /// Spot isolated unrealized profit and loss
    /// Applicable to Single-currency margin mode and Multi-currency margin mode
    pub spot_iso_upl: Option<String>,
}

impl RestClient {
    /// Get account balance
    ///
    /// Retrieve the balances of assets and the amount that is available or on hold.
    ///
    /// [docs]: https://www.okx.com/docs-v5/en/#rest-api-account-rest-api-get-balance
    ///
    /// Rate limit: 10 requests per 2 seconds
    ///
    /// # Arguments
    /// * `request` - The account balance request parameters
    ///
    /// # Returns
    /// A result containing the account balance information including equity and available funds
    pub async fn get_account_balance(
        &self,
        request: GetAccountBalanceRequest,
    ) -> RestResult<AccountBalance> {
        self.send_get_request(
            GET_ACCOUNT_BALANCE_ENDPOINT,
            request,
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_account_balance_request_serialization() {
        let request = GetAccountBalanceRequest {
            ccy: Some("BTC".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("ccy=BTC"));
    }

    #[test]
    fn test_get_account_balance_all_currencies() {
        let request = GetAccountBalanceRequest { ccy: None };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_account_balance_deserialization() {
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
                    "ordFroz": "0",
                    "imr": "0",
                    "mmr": "0",
                    "borrowed": "0",
                    "interest": "0",
                    "notionalUsd": "0",
                    "mgnRatio": "",
                    "details": [
                        {
                            "ccy": "BTC",
                            "eq": "1",
                            "cashBal": "1",
                            "uTime": "1597026383085",
                            "isoEq": "0",
                            "availEq": "1",
                            "disEq": "50000",
                            "availBal": "1",
                            "frozenBal": "0",
                            "ordFrozen": "0",
                            "liab": "0",
                            "upl": "0",
                            "uplLiab": "0",
                            "crossLiab": "0",
                            "isoLiab": "0",
                            "mgnRatio": "",
                            "interest": "0",
                            "twap": "0",
                            "maxLoan": "10000",
                            "eqUsd": "50000",
                            "borrowed": "0",
                            "strategyEq": "0",
                            "isoUpl": "0",
                            "spotInUseAmt": "",
                            "strategyFrozenBal": "0",
                            "spotIsoUpl": "0"
                        }
                    ]
                }
            ]
        }"#;

        let response: OkxApiResponse<AccountBalance> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let balance = &response.data[0];
        assert_eq!(balance.total_eq, "91.83200000");
        assert_eq!(balance.details.len(), 1);
        assert_eq!(balance.details[0].ccy, "BTC");
        assert_eq!(balance.details[0].eq, "1");
        assert_eq!(balance.details[0].avail_bal, "1");
    }
}
