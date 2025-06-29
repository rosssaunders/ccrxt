//! Enums for Bullish Exchange API

use serde::{Deserialize, Serialize};

/// Order side for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type for trading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Limit order
    #[serde(rename = "LMT")]
    Limit,
    /// Market order
    #[serde(rename = "MKT")]
    Market,
}

/// Time in force for orders
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good till cancelled
    Gtc,
    /// Fill or kill
    Fok,
    /// Immediate or cancel
    Ioc,
}

/// Order status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

/// Candlestick intervals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandleInterval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    #[test]
    fn test_order_side_serialization() {
        let buy = OrderSide::Buy;
        let sell = OrderSide::Sell;

        assert_eq!(serde_json::to_string(&buy).unwrap(), "\"BUY\"");
        assert_eq!(serde_json::to_string(&sell).unwrap(), "\"SELL\"");

        let buy_from_json: OrderSide = serde_json::from_str("\"BUY\"").unwrap();
        let sell_from_json: OrderSide = serde_json::from_str("\"SELL\"").unwrap();

        assert_eq!(buy_from_json, OrderSide::Buy);
        assert_eq!(sell_from_json, OrderSide::Sell);
    }

    #[test]
    fn test_order_type_serialization() {
        let limit = OrderType::Limit;
        let market = OrderType::Market;

        assert_eq!(serde_json::to_string(&limit).unwrap(), "\"LMT\"");
        assert_eq!(serde_json::to_string(&market).unwrap(), "\"MKT\"");

        let limit_from_json: OrderType = serde_json::from_str("\"LMT\"").unwrap();
        let market_from_json: OrderType = serde_json::from_str("\"MKT\"").unwrap();

        assert_eq!(limit_from_json, OrderType::Limit);
        assert_eq!(market_from_json, OrderType::Market);
    }

    #[test]
    fn test_time_in_force_serialization() {
        let gtc = TimeInForce::Gtc;
        let fok = TimeInForce::Fok;
        let ioc = TimeInForce::Ioc;

        assert_eq!(serde_json::to_string(&gtc).unwrap(), "\"GTC\"");
        assert_eq!(serde_json::to_string(&fok).unwrap(), "\"FOK\"");
        assert_eq!(serde_json::to_string(&ioc).unwrap(), "\"IOC\"");
    }

    #[test]
    fn test_order_status_serialization() {
        let open = OrderStatus::Open;
        let filled = OrderStatus::Filled;

        assert_eq!(serde_json::to_string(&open).unwrap(), "\"OPEN\"");
        assert_eq!(serde_json::to_string(&filled).unwrap(), "\"FILLED\"");
    }

    #[test]
    fn test_candle_interval_serialization() {
        let one_minute = CandleInterval::OneMinute;
        let one_hour = CandleInterval::OneHour;
        let one_day = CandleInterval::OneDay;

        assert_eq!(serde_json::to_string(&one_minute).unwrap(), "\"1m\"");
        assert_eq!(serde_json::to_string(&one_hour).unwrap(), "\"1h\"");
        assert_eq!(serde_json::to_string(&one_day).unwrap(), "\"1d\"");

        let one_minute_from_json: CandleInterval = serde_json::from_str("\"1m\"").unwrap();
        let one_hour_from_json: CandleInterval = serde_json::from_str("\"1h\"").unwrap();
        let one_day_from_json: CandleInterval = serde_json::from_str("\"1d\"").unwrap();

        assert_eq!(one_minute_from_json, CandleInterval::OneMinute);
        assert_eq!(one_hour_from_json, CandleInterval::OneHour);
        assert_eq!(one_day_from_json, CandleInterval::OneDay);
    }
}
