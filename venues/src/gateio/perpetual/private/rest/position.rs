use serde::{Deserialize, Serialize};

/// Futures position information (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesPosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Position size (positive for long, negative for short)
    pub size: i64,

    /// Average entry price
    pub entry_price: String,

    /// Mark price
    pub mark_price: String,

    /// Realized PnL
    pub realised_pnl: String,

    /// Unrealized PnL
    pub unrealised_pnl: String,

    /// Position margin
    pub margin: String,

    /// Leverage
    pub leverage: String,

    /// Risk limit
    pub risk_limit: String,

    /// Liquidation price
    pub liq_price: String,

    /// Bankruptcy price
    pub bankruptcy_price: String,

    /// Cross margin mode
    pub cross_leverage_limit: String,

    /// Position mode (single or dual)
    pub mode: String,

    /// Last update timestamp
    pub update_time: i64,
}

/// Position information in dual mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModePosition {
    /// User ID
    pub user: i64,

    /// Contract name
    pub contract: String,

    /// Long position size
    pub long_size: i64,

    /// Short position size
    pub short_size: i64,

    /// Long position entry price
    pub long_entry_price: String,

    /// Short position entry price
    pub short_entry_price: String,

    /// Long position leverage
    pub long_leverage: String,

    /// Short position leverage
    pub short_leverage: String,

    /// Long position margin
    pub long_margin: String,

    /// Short position margin
    pub short_margin: String,

    /// Long position PnL
    pub long_pnl: String,

    /// Short position PnL
    pub short_pnl: String,

    /// Mark price
    pub mark_price: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_position_deserialization() {
        let json = r#"{
            "user": 12345,
            "contract": "BTC_USDT",
            "size": 1000,
            "entry_price": "43000.0",
            "mark_price": "43250.0",
            "realised_pnl": "150.5",
            "unrealised_pnl": "250.0",
            "margin": "4300.0",
            "leverage": "10",
            "risk_limit": "1000000",
            "liq_price": "38700.0",
            "bankruptcy_price": "38500.0",
            "cross_leverage_limit": "25",
            "mode": "single",
            "update_time": 1640995200
        }"#;

        let position: FuturesPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.user, 12345);
        assert_eq!(position.contract, "BTC_USDT");
        assert_eq!(position.size, 1000);
        assert_eq!(position.entry_price, "43000.0");
        assert_eq!(position.mark_price, "43250.0");
        assert_eq!(position.realised_pnl, "150.5");
        assert_eq!(position.unrealised_pnl, "250.0");
        assert_eq!(position.margin, "4300.0");
        assert_eq!(position.leverage, "10");
        assert_eq!(position.risk_limit, "1000000");
        assert_eq!(position.liq_price, "38700.0");
        assert_eq!(position.bankruptcy_price, "38500.0");
        assert_eq!(position.cross_leverage_limit, "25");
        assert_eq!(position.mode, "single");
        assert_eq!(position.update_time, 1640995200);
    }

    #[test]
    fn test_dual_mode_position_deserialization() {
        let json = r#"{
            "user": 12345,
            "contract": "BTC_USDT",
            "long_size": 1000,
            "short_size": 500,
            "long_entry_price": "43000.0",
            "short_entry_price": "43500.0",
            "long_leverage": "10",
            "short_leverage": "5",
            "long_margin": "4300.0",
            "short_margin": "8700.0",
            "long_pnl": "250.0",
            "short_pnl": "-125.0",
            "mark_price": "43250.0"
        }"#;

        let position: DualModePosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.user, 12345);
        assert_eq!(position.contract, "BTC_USDT");
        assert_eq!(position.long_size, 1000);
        assert_eq!(position.short_size, 500);
        assert_eq!(position.long_entry_price, "43000.0");
        assert_eq!(position.short_entry_price, "43500.0");
        assert_eq!(position.long_leverage, "10");
        assert_eq!(position.short_leverage, "5");
        assert_eq!(position.long_margin, "4300.0");
        assert_eq!(position.short_margin, "8700.0");
        assert_eq!(position.long_pnl, "250.0");
        assert_eq!(position.short_pnl, "-125.0");
        assert_eq!(position.mark_price, "43250.0");
    }
}
