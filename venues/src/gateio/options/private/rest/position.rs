use serde::{Deserialize, Serialize};

/// Options position information (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size
    pub size: String,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Mark IV (implied volatility)
    pub mark_iv: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Pending orders
    pub pending_orders: i32,

    /// Close order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_order: Option<serde_json::Value>,

    /// Delta
    pub delta: String,

    /// Gamma
    pub gamma: String,

    /// Vega
    pub vega: String,

    /// Theta
    pub theta: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_position_deserialization() {
        let json = r#"{
            "user": 12345,
            "contract": "BTC-20240101-50000-C",
            "size": "10",
            "entry_price": "0.08",
            "mark_price": "0.10",
            "mark_iv": "0.25",
            "realised_pnl": "200.0",
            "unrealised_pnl": "150.0",
            "pending_orders": 2,
            "close_order": null,
            "delta": "0.65",
            "gamma": "0.015",
            "vega": "0.08",
            "theta": "-0.002"
        }"#;

        let position: OptionsPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.user, 12345);
        assert_eq!(position.contract, "BTC-20240101-50000-C");
        assert_eq!(position.size, "10");
        assert_eq!(position.entry_price, "0.08");
        assert_eq!(position.mark_price, "0.10");
        assert_eq!(position.mark_iv, "0.25");
        assert_eq!(position.realised_pnl, "200.0");
        assert_eq!(position.unrealised_pnl, "150.0");
        assert_eq!(position.pending_orders, 2);
        assert_eq!(position.close_order, None);
        assert_eq!(position.delta, "0.65");
        assert_eq!(position.gamma, "0.015");
        assert_eq!(position.vega, "0.08");
        assert_eq!(position.theta, "-0.002");
    }

    #[test]
    fn test_options_position_negative_pnl() {
        let json = r#"{
            "user": 67890,
            "contract": "ETH-20240101-3000-P",
            "size": "-5",
            "entry_price": "0.05",
            "mark_price": "0.03",
            "mark_iv": "0.35",
            "realised_pnl": "-100.0",
            "unrealised_pnl": "-75.0",
            "pending_orders": 1,
            "close_order": {"id": 123456, "type": "limit"},
            "delta": "-0.45",
            "gamma": "0.012",
            "vega": "0.06",
            "theta": "-0.001"
        }"#;

        let position: OptionsPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.user, 67890);
        assert_eq!(position.contract, "ETH-20240101-3000-P");
        assert_eq!(position.size, "-5");
        assert_eq!(position.entry_price, "0.05");
        assert_eq!(position.mark_price, "0.03");
        assert_eq!(position.mark_iv, "0.35");
        assert_eq!(position.realised_pnl, "-100.0");
        assert_eq!(position.unrealised_pnl, "-75.0");
        assert_eq!(position.pending_orders, 1);
        assert!(position.close_order.is_some());
        assert_eq!(position.delta, "-0.45");
        assert_eq!(position.gamma, "0.012");
        assert_eq!(position.vega, "0.06");
        assert_eq!(position.theta, "-0.001");
    }

    #[test]
    fn test_options_position_call_greeks() {
        let json = r#"{
            "user": 12345,
            "contract": "BTC-20240101-50000-C",
            "size": "10",
            "entry_price": "0.08",
            "mark_price": "0.10",
            "mark_iv": "0.25",
            "realised_pnl": "0.0",
            "unrealised_pnl": "200.0",
            "pending_orders": 0,
            "delta": "0.75",
            "gamma": "0.02",
            "vega": "0.12",
            "theta": "-0.005"
        }"#;

        let position: OptionsPosition = serde_json::from_str(json).unwrap();
        // Call option typically has positive delta (0 to 1)
        assert_eq!(position.delta, "0.75");
        // Gamma is always positive
        assert_eq!(position.gamma, "0.02");
        // Vega is always positive
        assert_eq!(position.vega, "0.12");
        // Theta is typically negative (time decay)
        assert_eq!(position.theta, "-0.005");
    }

    #[test]
    fn test_options_position_put_greeks() {
        let json = r#"{
            "user": 12345,
            "contract": "BTC-20240101-50000-P",
            "size": "10",
            "entry_price": "0.08",
            "mark_price": "0.10",
            "mark_iv": "0.25",
            "realised_pnl": "0.0",
            "unrealised_pnl": "200.0",
            "pending_orders": 0,
            "delta": "-0.25",
            "gamma": "0.015",
            "vega": "0.08",
            "theta": "-0.003"
        }"#;

        let position: OptionsPosition = serde_json::from_str(json).unwrap();
        // Put option typically has negative delta (-1 to 0)
        assert_eq!(position.delta, "-0.25");
        // Gamma is always positive
        assert_eq!(position.gamma, "0.015");
        // Vega is always positive
        assert_eq!(position.vega, "0.08");
        // Theta is typically negative (time decay)
        assert_eq!(position.theta, "-0.003");
    }

    #[test]
    fn test_options_position_short() {
        let json = r#"{
            "user": 54321,
            "contract": "ETH-20240101-3000-C",
            "size": "-8",
            "entry_price": "0.12",
            "mark_price": "0.08",
            "mark_iv": "0.30",
            "realised_pnl": "320.0",
            "unrealised_pnl": "240.0",
            "pending_orders": 1,
            "delta": "-0.60",
            "gamma": "-0.018",
            "vega": "-0.10",
            "theta": "0.004"
        }"#;

        let position: OptionsPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.user, 54321);
        assert_eq!(position.contract, "ETH-20240101-3000-C");
        assert_eq!(position.size, "-8"); // Short position (negative size)
        assert_eq!(position.entry_price, "0.12");
        assert_eq!(position.mark_price, "0.08");
        assert_eq!(position.mark_iv, "0.30");
        assert_eq!(position.realised_pnl, "320.0");
        assert_eq!(position.unrealised_pnl, "240.0"); // Profit from short position
        assert_eq!(position.pending_orders, 1);
        // For short positions, Greeks are typically negated
        assert_eq!(position.delta, "-0.60");
        assert_eq!(position.gamma, "-0.018");
        assert_eq!(position.vega, "-0.10");
        assert_eq!(position.theta, "0.004"); // Positive theta for short positions (time decay benefits)
    }
}