use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::cryptocom::RestResult;
use super::client::RestClient;

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

/// Response for user balance endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalanceResponse {
    /// Array of user balance data
    pub data: Vec<UserBalance>,
}

/// Balance history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceHistoryEntry {
    /// timestamp
    pub t: u64,
    /// total cash balance
    pub c: String,
}

/// Response for user balance history endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalanceHistoryResponse {
    /// instrument name of the balance e.g. USD
    pub instrument_name: String,
    /// Array of balance history data
    pub data: Vec<BalanceHistoryEntry>,
}

/// Parameters for user balance history request
#[derive(Debug, Clone, Serialize)]
pub struct UserBalanceHistoryParams {
    /// H1 means every hour, D1 means every day. Omit for 'D1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    /// Can be millisecond or nanosecond. Exclusive. If not provided, will be current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// If timeframe is D1, max limit will be 30 (days). If timeframe is H1, max limit will be 120 (hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl RestClient {
    /// Get user balance
    /// 
    /// Returns the user's wallet balance.
    /// 
    /// # Returns
    /// User balance information including position balances and margin details
    pub async fn get_user_balance(&self) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = json!({});
        
        let signature = self.sign_request("private/user-balance", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/user-balance",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/user-balance", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Get user balance history
    /// 
    /// Returns the user's balance history. This call may temporarily have discrepancies with that shown on the GUI.
    /// 
    /// # Arguments
    /// * `timeframe` - H1 means every hour, D1 means every day. Omit for 'D1'
    /// * `end_time` - Can be millisecond or nanosecond. Exclusive. If not provided, will be current time.
    /// * `limit` - If timeframe is D1, max limit will be 30 (days). If timeframe is H1, max limit will be 120 (hours).
    /// 
    /// # Returns
    /// User balance history information
    pub async fn get_user_balance_history(
        &self,
        timeframe: Option<String>,
        end_time: Option<u64>,
        limit: Option<i32>
    ) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        
        let mut params = json!({});
        if let Some(tf) = timeframe {
            params["timeframe"] = Value::String(tf);
        }
        if let Some(et) = end_time {
            params["end_time"] = Value::Number(et.into());
        }
        if let Some(l) = limit {
            params["limit"] = Value::Number(l.into());
        }
        
        let signature = self.sign_request("private/user-balance-history", id, &params, nonce)?;
        
        let request_body = json!({
            "id": id,
            "method": "private/user-balance-history",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self.client
            .post(&format!("{}/v1/private/user-balance-history", self.base_url))
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
    fn test_user_balance_structures() {
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
        assert_eq!(balance.is_liquidating, false);
        assert_eq!(balance.position_balances.len(), 1);
        assert_eq!(balance.position_balances[0].instrument_name, "CRO");
    }

    #[test]
    fn test_balance_history_structures() {
        let history_json = json!({
            "instrument_name": "USD",
            "data": [
                {
                    "t": 1629478800000_u64,
                    "c": "811.621851"
                }
            ]
        });

        let history: UserBalanceHistoryResponse = serde_json::from_value(history_json).unwrap();
        assert_eq!(history.instrument_name, "USD");
        assert_eq!(history.data.len(), 1);
        assert_eq!(history.data[0].t, 1629478800000_u64);
        assert_eq!(history.data[0].c, "811.621851");
    }

    #[test]
    fn test_balance_history_params_serialization() {
        let params = UserBalanceHistoryParams {
            timeframe: Some("H1".to_string()),
            end_time: Some(1629478800000_u64),
            limit: Some(10),
        };

        let json_value = serde_json::to_value(params).unwrap();
        assert_eq!(json_value["timeframe"], "H1");
        assert_eq!(json_value["end_time"], 1629478800000_u64);
        assert_eq!(json_value["limit"], 10);
    }

    #[test]
    fn test_balance_history_params_optional_fields() {
        let params = UserBalanceHistoryParams {
            timeframe: None,
            end_time: None,
            limit: None,
        };

        let json_value = serde_json::to_value(params).unwrap();
        assert_eq!(json_value, json!({}));
    }
}