use serde::{Deserialize, Serialize};
use serde_json;

use crate::cryptocom::{ApiResult, PrivateRestClient as RestClient, RestResult};

const USER_BALANCE_ENDPOINT: &str = "exchange/v1/private/user-balance";

/// Position balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionBalance {
    /// Instrument name of the collateral e.g. USD, CRO, USDT, or DAI
    pub instrument_name: String,

    /// Quantity of the collateral
    pub quantity: String,

    /// Market value of the collateral
    pub market_value: String,

    /// true or false
    pub collateral_eligible: String,

    /// Show haircut for eligible collateral token
    pub haircut: String,

    /// Collateral amount derived by market_value minus haircut
    pub collateral_amount: String,

    /// Max withdrawal balance of the collateral
    pub max_withdrawal_balance: String,

    /// Fund/balance in use, not available for new orders or additional trading activities
    pub reserved_qty: String,
}

/// User balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalance {
    /// Balance that user can open new order (Margin Balance - Initial Margin)
    pub total_available_balance: String,

    /// Positive cash balance on eligible collateral tokens + Negative balance on all tokens + Unrealised PnL - Fee reserves
    pub total_margin_balance: String,

    /// Total margin requirement to support positions and all open orders IM and haircut from risk asset holdings
    pub total_initial_margin: String,

    /// initial margin requirement to support open positions and orders
    pub total_position_im: String,

    /// the total haircut on eligible collateral token assets
    pub total_haircut: String,

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

    /// Instrument name of the balance e.g. USD
    pub instrument_name: String,

    /// Current realized profit and loss from all open positions
    pub total_session_realized_pnl: String,

    /// Describes whether the account is under liquidation
    pub is_liquidating: bool,

    /// The actual leverage used (all open positions combined)
    pub total_effective_leverage: String,

    /// Maximum position size allowed (for all open positions combined)
    pub position_limit: String,

    /// Combined position size of all open positions + order exposure on all instruments
    pub used_position_limit: String,

    /// Collateral balances
    pub position_balances: Vec<PositionBalance>,
}

/// User balance data result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalanceResult {
    /// Array of user balance data
    pub data: Vec<UserBalance>,
}

/// Response wrapper for user balance endpoint
pub type UserBalanceResponse = ApiResult<UserBalanceResult>;

impl RestClient {
    /// Get user balance
    ///
    /// Returns the user's wallet balance with margin details and position balances.
    ///
    /// [docs](https://exchange-docs.crypto.com/exchange/v1/rest-ws/index.html#private-user-balance)
    ///
    /// Rate limit: No rate limit
    ///
    /// # Returns
    /// User balance information including position balances and margin details
    pub async fn get_user_balance(&self) -> RestResult<UserBalanceResponse> {
        let params = serde_json::json!({});

        self.send_signed_request(USER_BALANCE_ENDPOINT, params)
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
    fn test_position_balance_structure() {
        let position_balance_json = json!({
            "instrument_name": "CRO",
            "quantity": "24422.72427884",
            "market_value": "4776.107959969951",
            "collateral_eligible": "true",
            "haircut": "0.5",
            "collateral_amount": "4537.302561971453",
            "max_withdrawal_balance": "24422.72427884",
            "reserved_qty": "0.00000000"
        });

        let position_balance: PositionBalance =
            serde_json::from_value(position_balance_json).unwrap();
        assert_eq!(position_balance.instrument_name, "CRO");
        assert_eq!(position_balance.collateral_eligible, "true");
        assert_eq!(position_balance.quantity, "24422.72427884");
    }

    #[test]
    fn test_user_balance_structure() {
        let balance_json = json!({
            "total_available_balance": "4721.05898582",
            "total_margin_balance": "7595.42571782",
            "total_initial_margin": "2874.36673202",
            "total_position_im": "486.31273202",
            "total_haircut": "2388.054",
            "total_maintenance_margin": "1437.18336601",
            "total_position_cost": "14517.54641301",
            "total_cash_balance": "7890.00320721",
            "total_collateral_value": "7651.18811483",
            "total_session_unrealized_pnl": "-55.76239701",
            "instrument_name": "USD",
            "total_session_realized_pnl": "0.00000000",
            "is_liquidating": false,
            "total_effective_leverage": "1.90401230",
            "position_limit": "3000000.00000000",
            "used_position_limit": "40674.69622001",
            "position_balances": [
                {
                    "instrument_name": "CRO",
                    "quantity": "24422.72427884",
                    "market_value": "4776.107959969951",
                    "collateral_eligible": "true",
                    "haircut": "0.5",
                    "collateral_amount": "4537.302561971453",
                    "max_withdrawal_balance": "24422.72427884",
                    "reserved_qty": "0.00000000"
                }
            ]
        });

        let balance: UserBalance = serde_json::from_value(balance_json).unwrap();
        assert_eq!(balance.instrument_name, "USD");
        assert!(!balance.is_liquidating);
        assert_eq!(balance.position_balances.len(), 1);
        assert_eq!(
            balance.position_balances.first().unwrap().instrument_name,
            "CRO"
        );
    }

    #[test]
    fn test_user_balance_response_structure() {
        let response_json = json!({
            "code": 0,
            "id": 1,
            "result": {
                "data": [
                    {
                        "total_available_balance": "4721.05898582",
                        "total_margin_balance": "7595.42571782",
                        "total_initial_margin": "2874.36673202",
                        "total_position_im": "486.31273202",
                        "total_haircut": "2388.054",
                        "total_maintenance_margin": "1437.18336601",
                        "total_position_cost": "14517.54641301",
                        "total_cash_balance": "7890.00320721",
                        "total_collateral_value": "7651.18811483",
                        "total_session_unrealized_pnl": "-55.76239701",
                        "instrument_name": "USD",
                        "total_session_realized_pnl": "0.00000000",
                        "is_liquidating": false,
                        "total_effective_leverage": "1.90401230",
                        "position_limit": "3000000.00000000",
                        "used_position_limit": "40674.69622001",
                        "position_balances": []
                    }
                ]
            }
        });

        let response: UserBalanceResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.result.data.len(), 1);
        assert_eq!(response.result.data.first().unwrap().instrument_name, "USD");
        assert!(!response.result.data.first().unwrap().is_liquidating);
    }
}
