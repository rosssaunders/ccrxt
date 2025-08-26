use serde::{Deserialize, Serialize};

/// Futures contract information (common struct used by multiple endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesContract {
    /// Futures contract
    pub name: String,

    /// Underlying
    pub underlying: Option<String>,

    /// Quote currency
    pub quote_currency: Option<String>,

    /// Settlement currency
    pub settle_currency: Option<String>,

    /// Contract type (inverse, linear)
    #[serde(rename = "type")]
    pub contract_type: String,

    /// Quantitative scale
    pub quanto_multiplier: String,

    /// Leverage amount
    pub leverage_min: String,

    /// Leverage amount
    pub leverage_max: String,

    /// Maintenance rate
    pub maintenance_rate: String,

    /// Mark price
    pub mark_price: String,

    /// Index price
    pub index_price: String,

    /// Last trading time
    pub last_price: String,

    /// Maker fee rate
    pub maker_fee_rate: String,

    /// Taker fee rate  
    pub taker_fee_rate: String,

    /// Value of each contract
    pub order_price_round: String,

    /// Minimum order price increment
    pub mark_price_round: String,

    /// Funding rate
    pub funding_rate: String,

    /// Funding interval (in seconds)
    pub funding_interval: i64,

    /// Next funding time
    pub funding_next_apply: i64,

    /// Risk limit base
    pub risk_limit_base: String,

    /// Risk limit step
    pub risk_limit_step: String,

    /// Maximum risk limit  
    pub risk_limit_max: String,

    /// Minimum order size
    pub order_size_min: i64,

    /// Maximum order size
    pub order_size_max: i64,

    /// Order price deviation from current mark price
    pub order_price_deviate: String,

    /// Reference discount rate for buying
    pub ref_discount_rate: String,

    /// Reference rebate rate for selling
    pub ref_rebate_rate: String,

    /// Current orderbook ID
    pub orderbook_id: i64,

    /// Trade ID
    pub trade_id: i64,

    /// Trade size
    pub trade_size: i64,

    /// Position size
    pub position_size: i64,

    /// Configuration change time
    pub config_change_time: f64,

    /// Whether the contract is delisted
    pub in_delisting: Option<bool>,

    /// Total traded volume in quote currency
    pub orders_limit: i32,

    /// Whether inverse contract
    pub enable_bonus: Option<bool>,

    /// Enable credit trading
    pub enable_credit: Option<bool>,

    /// Create time
    pub create_time: Option<f64>,

    /// Funding rate high limit
    pub funding_cap: Option<String>,

    /// Funding rate low limit  
    pub funding_floor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_futures_contract_deserialization() {
        let json = r#"{
            "name": "BTC_USDT",
            "underlying": "BTC",
            "quote_currency": "USDT",
            "settle_currency": "USDT",
            "type": "linear",
            "quanto_multiplier": "1",
            "leverage_min": "1",
            "leverage_max": "100",
            "maintenance_rate": "0.005",
            "mark_price": "43250.5",
            "index_price": "43248.2",
            "last_price": "43251.0",
            "maker_fee_rate": "-0.00025",
            "taker_fee_rate": "0.00075",
            "order_price_round": "0.1",
            "mark_price_round": "0.01",
            "funding_rate": "0.0001",
            "funding_interval": 28800,
            "funding_next_apply": 1640995200,
            "risk_limit_base": "1000000",
            "risk_limit_step": "1000000",
            "risk_limit_max": "50000000",
            "order_size_min": 1,
            "order_size_max": 1000000,
            "order_price_deviate": "0.5",
            "ref_discount_rate": "0.1",
            "ref_rebate_rate": "0.2",
            "orderbook_id": 123456789,
            "trade_id": 987654321,
            "trade_size": 1000000,
            "position_size": 500000,
            "config_change_time": 1640908800.0,
            "in_delisting": false,
            "orders_limit": 200,
            "enable_bonus": true,
            "enable_credit": false,
            "create_time": 1640000000.0,
            "funding_cap": "0.001",
            "funding_floor": "-0.001"
        }"#;

        let contract: FuturesContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "BTC_USDT");
        assert_eq!(contract.underlying, Some("BTC".to_string()));
        assert_eq!(contract.quote_currency, Some("USDT".to_string()));
        assert_eq!(contract.settle_currency, Some("USDT".to_string()));
        assert_eq!(contract.contract_type, "linear");
        assert_eq!(contract.quanto_multiplier, "1");
        assert_eq!(contract.leverage_min, "1");
        assert_eq!(contract.leverage_max, "100");
        assert_eq!(contract.maintenance_rate, "0.005");
        assert_eq!(contract.mark_price, "43250.5");
        assert_eq!(contract.index_price, "43248.2");
        assert_eq!(contract.last_price, "43251.0");
        assert_eq!(contract.maker_fee_rate, "-0.00025");
        assert_eq!(contract.taker_fee_rate, "0.00075");
        assert_eq!(contract.order_price_round, "0.1");
        assert_eq!(contract.mark_price_round, "0.01");
        assert_eq!(contract.funding_rate, "0.0001");
        assert_eq!(contract.funding_interval, 28800);
        assert_eq!(contract.funding_next_apply, 1640995200);
        assert_eq!(contract.risk_limit_base, "1000000");
        assert_eq!(contract.risk_limit_step, "1000000");
        assert_eq!(contract.risk_limit_max, "50000000");
        assert_eq!(contract.order_size_min, 1);
        assert_eq!(contract.order_size_max, 1000000);
        assert_eq!(contract.order_price_deviate, "0.5");
        assert_eq!(contract.ref_discount_rate, "0.1");
        assert_eq!(contract.ref_rebate_rate, "0.2");
        assert_eq!(contract.orderbook_id, 123456789);
        assert_eq!(contract.trade_id, 987654321);
        assert_eq!(contract.trade_size, 1000000);
        assert_eq!(contract.position_size, 500000);
        assert_eq!(contract.config_change_time, 1640908800.0);
        assert_eq!(contract.in_delisting, Some(false));
        assert_eq!(contract.orders_limit, 200);
        assert_eq!(contract.enable_bonus, Some(true));
        assert_eq!(contract.enable_credit, Some(false));
        assert_eq!(contract.create_time, Some(1640000000.0));
        assert_eq!(contract.funding_cap, Some("0.001".to_string()));
        assert_eq!(contract.funding_floor, Some("-0.001".to_string()));
    }

    #[test]
    fn test_futures_contract_minimal() {
        let json = r#"{
            "name": "ETH_USDT",
            "type": "linear",
            "quanto_multiplier": "1",
            "leverage_min": "1",
            "leverage_max": "50",
            "maintenance_rate": "0.01",
            "mark_price": "2650.5",
            "index_price": "2649.8",
            "last_price": "2651.0",
            "maker_fee_rate": "-0.00025",
            "taker_fee_rate": "0.00075",
            "order_price_round": "0.01",
            "mark_price_round": "0.01",
            "funding_rate": "0.0001",
            "funding_interval": 28800,
            "funding_next_apply": 1640995200,
            "risk_limit_base": "500000",
            "risk_limit_step": "500000",
            "risk_limit_max": "20000000",
            "order_size_min": 1,
            "order_size_max": 100000,
            "order_price_deviate": "0.5",
            "ref_discount_rate": "0.1",
            "ref_rebate_rate": "0.2",
            "orderbook_id": 123456,
            "trade_id": 654321,
            "trade_size": 50000,
            "position_size": 25000,
            "config_change_time": 1640908800.0,
            "orders_limit": 200
        }"#;

        let contract: FuturesContract = serde_json::from_str(json).unwrap();
        assert_eq!(contract.name, "ETH_USDT");
        assert!(contract.underlying.is_none());
        assert!(contract.quote_currency.is_none());
        assert!(contract.settle_currency.is_none());
        assert!(contract.in_delisting.is_none());
        assert!(contract.enable_bonus.is_none());
        assert!(contract.enable_credit.is_none());
        assert!(contract.create_time.is_none());
        assert!(contract.funding_cap.is_none());
        assert!(contract.funding_floor.is_none());
    }
}
