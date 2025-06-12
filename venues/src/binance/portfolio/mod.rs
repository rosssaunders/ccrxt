mod enums;

pub use enums::*;

#[cfg(test)]
mod tests {
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
    fn test_enum_display() {
        assert_eq!(format!("{}", OrderSide::Buy), "BUY");
        assert_eq!(format!("{}", TimeInForce::GTX), "GTX");
        assert_eq!(format!("{}", SideEffectType::NoSideEffect), "NO_SIDE_EFFECT");
        assert_eq!(format!("{}", PriceMatch::Opponent5), "OPPONENT_5");
        assert_eq!(format!("{}", StrategyType::TakeProfit), "TAKE_PROFIT");
        assert_eq!(format!("{}", ContractStatus::PreSettle), "PRE_SETTLE");
    }

    #[test]
    fn test_all_enums_creation() {
        // Test that all enums can be created without issues
        let _ = OrderSide::Buy;
        let _ = PositionSide::Long;
        let _ = TimeInForce::GTC;
        let _ = StopLimitTimeInForce::IOC;
        let _ = SideEffectType::NoSideEffect;
        let _ = PriceMatch::Queue10;
        let _ = SelfTradePreventionMode::ExpireBoth;
        let _ = OrderResponseType::Result;
        let _ = OrderType::Market;
        let _ = StrategyType::TrailingStopMarket;
        let _ = WorkingType::MarkPrice;
        let _ = OrderStatus::PartiallyFilled;
        let _ = StrategyStatus::Finished;
        let _ = ContractType::CurrentQuarter;
        let _ = ContractStatus::Close;
        let _ = RateLimitType::Orders;
        let _ = RateLimitInterval::Minute;
    }
}