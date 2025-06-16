use serde::{Deserialize, Serialize};

use super::client::RestClient;
use super::user_balance::PositionBalance;
use crate::cryptocom::RestResult;

/// Subaccount balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubaccountBalance {
    /// Sub account ID
    pub account: String,
    /// Instrument name of the balance e.g. USD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Balance that user can open new order (Margin Balance - Initial Margin)
    pub total_available_balance: String,
    /// Positive cash balance on eligible collateral tokens + Negative balance on all tokens + Unrealised PnL - Fee reserves
    pub total_margin_balance: String,
    /// Total margin requirement to support positions and all open orders IM and haircut from risk asset holdings
    pub total_initial_margin: String,
    /// Total maintenance margin requirement for all positions
    pub total_maintenance_margin: String,
    /// Position value in USD
    pub total_position_cost: String,
    /// Wallet Balance (Deposits - Withdrawals + Realized PnL - Fees)
    pub total_cash_balance: String,
    /// Collateral Value
    pub total_collateral_value: String,
    /// Current unrealized profit and loss from all open positions
    pub total_session_unrealized_pnl: String,
    /// Current realized profit and loss from all open positions
    pub total_session_realized_pnl: String,
    /// The actual leverage used (all open positions combined)
    pub total_effective_leverage: String,
    /// Maximum position size allowed (for all open positions combined)
    pub position_limit: String,
    /// Combined position size of all open positions + order exposure on all instruments
    pub used_position_limit: String,
    /// Describes whether the account is under liquidation
    pub is_liquidating: bool,
    /// Collateral balances
    pub position_balances: Vec<PositionBalance>,
}

/// Response for get subaccount balances endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubaccountBalancesResponse {
    /// Array of subaccount balance data
    pub data: Vec<SubaccountBalance>,
}

impl RestClient {
    /// Get wallet balances for all sub-accounts
    ///
    /// Returns the user's wallet balances for all sub-accounts.
    ///
    /// See: <https://exchange-docs.crypto.com/derivatives/index.html>
    ///
    /// Rate limit: No rate limit
    ///
    /// # Returns
    /// Subaccount balance information for all sub-accounts
    pub async fn get_subaccount_balances(&self) -> RestResult<GetSubaccountBalancesResponse> {
        // Empty struct to represent request with no parameters
        #[derive(Debug, Clone, Serialize)]
        struct EmptyRequest {}

        self.send_signed_request("private/get-subaccount-balances", EmptyRequest {})
            .await
    }
}

#[cfg(test)]
mod tests {
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    use super::*;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    #[allow(dead_code)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        #[allow(dead_code)]
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_subaccount_balance_structure() {
        let balance_json = json!({
            "account": "a0d206a1-6b06-47c5-9cd3-8bc6ef0915c5",
            "instrument_name": "USD",
            "total_available_balance": "0.00000000",
            "total_margin_balance": "0.00000000",
            "total_initial_margin": "0.00000000",
            "total_maintenance_margin": "0.00000000",
            "total_position_cost": "0.00000000",
            "total_cash_balance": "0.00000000",
            "total_collateral_value": "0.00000000",
            "total_session_unrealized_pnl": "0.00000000",
            "total_session_realized_pnl": "0.00000000",
            "total_effective_leverage": "0.00000000",
            "position_limit": "3000000.00000000",
            "used_position_limit": "0.00000000",
            "is_liquidating": false,
            "position_balances": []
        });

        let balance: SubaccountBalance = serde_json::from_value(balance_json).unwrap();
        assert_eq!(balance.account, "a0d206a1-6b06-47c5-9cd3-8bc6ef0915c5");
        assert_eq!(balance.instrument_name, Some("USD".to_string()));
        assert!(!balance.is_liquidating);
        assert_eq!(balance.position_balances.len(), 0);
        assert_eq!(balance.total_available_balance, "0.00000000");
    }

    #[test]
    fn test_subaccount_balance_with_position_balances() {
        let balance_json = json!({
            "account": "49786818-6ead-40c4-a008-ea6b0fa5cf96",
            "instrument_name": "USD",
            "total_available_balance": "20823.62250000",
            "total_margin_balance": "20823.62250000",
            "total_initial_margin": "0.00000000",
            "total_maintenance_margin": "0.00000000",
            "total_position_cost": "0.00000000",
            "total_cash_balance": "21919.55000000",
            "total_collateral_value": "20823.62250000",
            "total_session_unrealized_pnl": "0.00000000",
            "total_session_realized_pnl": "0.00000000",
            "total_effective_leverage": "0.00000000",
            "position_limit": "3000000.00000000",
            "used_position_limit": "0.00000000",
            "is_liquidating": false,
            "position_balances": [
                {
                    "instrument_name": "BTC",
                    "quantity": "1.0000000000",
                    "market_value": "21918.5500000000",
                    "collateral_eligible": "true",
                    "haircut": "0.5500000000",
                    "collateral_amount": "21918.0000000000",
                    "max_withdrawal_balance": "1.0000000000",
                    "reserved_qty": "0.00000000"
                }
            ]
        });

        let balance: SubaccountBalance = serde_json::from_value(balance_json).unwrap();
        assert_eq!(balance.account, "49786818-6ead-40c4-a008-ea6b0fa5cf96");
        assert_eq!(balance.position_balances.len(), 1);
        assert_eq!(
            balance.position_balances.first().unwrap().instrument_name,
            "BTC"
        );
        assert_eq!(
            balance.position_balances.first().unwrap().quantity,
            "1.0000000000"
        );
        assert_eq!(balance.total_available_balance, "20823.62250000");
    }

    #[test]
    fn test_subaccount_balance_without_instrument_name() {
        let balance_json = json!({
            "account": "test-account-uuid",
            "total_available_balance": "1000.00000000",
            "total_margin_balance": "1000.00000000",
            "total_initial_margin": "0.00000000",
            "total_maintenance_margin": "0.00000000",
            "total_position_cost": "0.00000000",
            "total_cash_balance": "1000.00000000",
            "total_collateral_value": "1000.00000000",
            "total_session_unrealized_pnl": "0.00000000",
            "total_session_realized_pnl": "0.00000000",
            "total_effective_leverage": "0.00000000",
            "position_limit": "3000000.00000000",
            "used_position_limit": "0.00000000",
            "is_liquidating": false,
            "position_balances": []
        });

        let balance: SubaccountBalance = serde_json::from_value(balance_json).unwrap();
        assert_eq!(balance.account, "test-account-uuid");
        assert_eq!(balance.instrument_name, None);
        assert_eq!(balance.total_available_balance, "1000.00000000");
    }

    #[test]
    fn test_get_subaccount_balances_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "account": "account-1",
                    "instrument_name": "USD",
                    "total_available_balance": "100.00000000",
                    "total_margin_balance": "100.00000000",
                    "total_initial_margin": "0.00000000",
                    "total_maintenance_margin": "0.00000000",
                    "total_position_cost": "0.00000000",
                    "total_cash_balance": "100.00000000",
                    "total_collateral_value": "100.00000000",
                    "total_session_unrealized_pnl": "0.00000000",
                    "total_session_realized_pnl": "0.00000000",
                    "total_effective_leverage": "0.00000000",
                    "position_limit": "3000000.00000000",
                    "used_position_limit": "0.00000000",
                    "is_liquidating": false,
                    "position_balances": []
                },
                {
                    "account": "account-2",
                    "total_available_balance": "200.00000000",
                    "total_margin_balance": "200.00000000",
                    "total_initial_margin": "0.00000000",
                    "total_maintenance_margin": "0.00000000",
                    "total_position_cost": "0.00000000",
                    "total_cash_balance": "200.00000000",
                    "total_collateral_value": "200.00000000",
                    "total_session_unrealized_pnl": "0.00000000",
                    "total_session_realized_pnl": "0.00000000",
                    "total_effective_leverage": "0.00000000",
                    "position_limit": "3000000.00000000",
                    "used_position_limit": "0.00000000",
                    "is_liquidating": false,
                    "position_balances": []
                }
            ]
        });

        let response: GetSubaccountBalancesResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data.first().unwrap().account, "account-1");
        assert_eq!(
            response.data.first().unwrap().instrument_name,
            Some("USD".to_string())
        );
        assert_eq!(response.data.get(1).unwrap().account, "account-2");
        assert_eq!(response.data.get(1).unwrap().instrument_name, None);
    }
}
