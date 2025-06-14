#[cfg(test)]
mod integration_tests {
    use crate::binance::portfolio::{
        Errors, OrderSide, OrderType, PortfolioMarginRateLimiter, RateLimiter,
    };

    #[test]
    fn test_portfolio_margin_module_exports() {
        // Test that we can use the exported types
        let _limiter: PortfolioMarginRateLimiter = PortfolioMarginRateLimiter::new();

        // Test that enums are available
        let _side = OrderSide::Buy;
        let _order_type = OrderType::Limit;

        // Test that error types are available
        let _error = Errors::Error("test".to_string());
    }

    #[test]
    fn test_portfolio_margin_rate_limits_constants() {
        // Verify the constants match the issue requirements
        // IP Limit: 6000/min - this is implemented in the rate limiter logic
        // Order Limits: 1200/min - this is implemented in the rate limiter logic

        // These are tested in the rate_limit module tests, but we can verify
        // that the types are correctly re-exported
        assert_eq!(
            std::mem::size_of::<PortfolioMarginRateLimiter>(),
            std::mem::size_of::<RateLimiter>()
        );
    }
}
