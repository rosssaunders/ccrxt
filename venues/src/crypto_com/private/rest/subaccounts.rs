use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::cryptocom::RestResult;
use super::client::RestClient;
use super::balance::PositionBalance;

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

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Account ID
    pub account_id: String,
    /// Position quantity
    pub quantity: String,
    /// Position cost or value in USD
    pub cost: String,
    /// Profit and loss for the open position
    pub open_position_pnl: String,
    /// Open position cost
    pub open_pos_cost: String,
    /// Profit and loss in the current trading session
    pub session_pnl: String,
    /// Updated time (Unix timestamp)
    pub update_timestamp_ms: u64,
    /// e.g. BTCUSD-PERP
    pub instrument_name: String,
    /// e.g. Perpetual Swap
    #[serde(rename = "type")]
    pub position_type: String,
}

/// Response for get positions endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionsResponse {
    /// Array of position data
    pub data: Vec<Position>,
}

/// Parameters for get positions request
#[derive(Debug, Clone, Serialize)]
pub struct GetPositionsParams {
    /// e.g. BTCUSD-PERP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
}

impl RestClient {
    /// Get user's wallet balances of all sub-accounts
    /// 
    /// Returns the user's wallet balances of all sub-accounts.
    /// 
    /// # Returns
    /// Subaccount balance information for all sub-accounts
    pub async fn get_subaccount_balances(&self) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = json!({});
        
        let signature = self.sign_request("private/get-subaccount-balances", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/get-subaccount-balances",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/get-subaccount-balances", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Get user's positions
    /// 
    /// Returns the user's position.
    /// 
    /// # Arguments
    /// * `instrument_name` - Optional instrument name filter (e.g. BTCUSD-PERP)
    /// 
    /// # Returns
    /// Position information for all or specified instruments
    pub async fn get_positions(&self, instrument_name: Option<&str>) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        
        let mut params = json!({});
        if let Some(instrument) = instrument_name {
            params["instrument_name"] = Value::String(instrument.to_string());
        }
        
        let signature = self.sign_request("private/get-positions", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/get-positions",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/get-positions", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }
    
    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }
    
    impl PlainTextSecret {
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
        assert_eq!(balance.is_liquidating, false);
        assert_eq!(balance.position_balances.len(), 0);
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
        assert_eq!(balance.position_balances[0].instrument_name, "BTC");
    }

    #[test]
    fn test_position_structure() {
        let position_json = json!({
            "account_id": "858dbc8b-22fd-49fa-bff4-d342d98a8acb",
            "quantity": "-0.1984",
            "cost": "-10159.573500",
            "open_position_pnl": "-497.743736",
            "open_pos_cost": "-10159.352200",
            "session_pnl": "2.236145",
            "update_timestamp_ms": 1613552240770_u64,
            "instrument_name": "BTCUSD-PERP",
            "type": "PERPETUAL_SWAP"
        });

        let position: Position = serde_json::from_value(position_json).unwrap();
        assert_eq!(position.account_id, "858dbc8b-22fd-49fa-bff4-d342d98a8acb");
        assert_eq!(position.instrument_name, "BTCUSD-PERP");
        assert_eq!(position.position_type, "PERPETUAL_SWAP");
        assert_eq!(position.quantity, "-0.1984");
    }

    #[test]
    fn test_get_positions_params_serialization() {
        let params = GetPositionsParams {
            instrument_name: Some("BTCUSD-PERP".to_string()),
        };

        let json_value = serde_json::to_value(params).unwrap();
        assert_eq!(json_value["instrument_name"], "BTCUSD-PERP");
    }

    #[test]
    fn test_get_positions_params_optional() {
        let params = GetPositionsParams {
            instrument_name: None,
        };

        let json_value = serde_json::to_value(params).unwrap();
        assert_eq!(json_value, json!({}));
    }
}