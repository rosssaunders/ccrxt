#[cfg(test)]
mod tests {
    use crate::bitget::public::rest::RestClient;
    use crate::bitget::public::rest::candlestick::GetCandlestickRequest;
    use crate::bitget::public::rest::coin_info::GetCoinInfoRequest;
    use crate::bitget::public::rest::history_candlestick::GetHistoryCandlestickRequest;
    use crate::bitget::public::rest::market_trades::GetMarketTradesRequest;
    use crate::bitget::public::rest::merge_depth::GetMergeDepthRequest;
    use crate::bitget::public::rest::orderbook::GetOrderbookRequest;
    use crate::bitget::public::rest::recent_trades::GetRecentTradesRequest;
    use crate::bitget::public::rest::symbol_info::GetSymbolInfoRequest;
    use crate::bitget::public::rest::ticker::GetTickerRequest;
    use crate::bitget::rate_limit::RateLimiter;
    use crate::bitget::{CandlestickGranularity, DepthType, PricePrecision};
    use chrono::{Duration, Utc};
    use reqwest::Client;

    fn create_test_client() -> RestClient {
        let client = Client::new();
        let rate_limiter = RateLimiter::default();
        RestClient::new("https://api.bitget.com", rate_limiter, client)
    }

    #[tokio::test]
    async fn test_get_coin_info() {
        let client = create_test_client();
        let response = client.get_coin_info(GetCoinInfoRequest::new()).await;
        assert!(response.is_ok(), "Failed to get coin info: {:?}", response);
    }

    #[tokio::test]
    async fn test_get_symbol_info() {
        let client = create_test_client();
        let response = client.get_symbol_info(GetSymbolInfoRequest::new()).await;
        assert!(
            response.is_ok(),
            "Failed to get symbol info: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_specific_symbol_info() {
        let client = create_test_client();
        let response = client
            .get_symbol_info(GetSymbolInfoRequest::new().symbol("BTCUSDT"))
            .await;
        assert!(
            response.is_ok(),
            "Failed to get specific symbol info: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_vip_fee_rate() {
        let client = create_test_client();
        let response = client.get_vip_fee_rate().await;
        assert!(
            response.is_ok(),
            "Failed to get VIP fee rate: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_ticker() {
        let client = create_test_client();
        let response = client
            .get_ticker(GetTickerRequest::new().symbol("BTCUSDT"))
            .await;
        assert!(response.is_ok(), "Failed to get ticker: {:?}", response);
    }

    #[tokio::test]
    async fn test_get_merge_depth() {
        let client = create_test_client();
        let response = client
            .get_merge_depth(
                GetMergeDepthRequest::new("BTCUSDT")
                    .precision(PricePrecision::Scale1)
                    .limit(50),
            )
            .await;
        assert!(
            response.is_ok(),
            "Failed to get merge depth: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_orderbook() {
        let client = create_test_client();
        let response = client
            .get_orderbook(
                GetOrderbookRequest::new("BTCUSDT")
                    .depth_type(DepthType::Step0)
                    .limit(100),
            )
            .await;
        assert!(response.is_ok(), "Failed to get orderbook: {:?}", response);
    }

    #[tokio::test]
    async fn test_get_candlestick() {
        let client = create_test_client();
        let response = client
            .get_candlestick(
                GetCandlestickRequest::new("BTCUSDT", CandlestickGranularity::OneMinute).limit(10),
            )
            .await;
        assert!(
            response.is_ok(),
            "Failed to get candlestick: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_history_candlestick() {
        let client = create_test_client();
        let end_time = 1659080270000; // Sample timestamp
        let response = client
            .get_history_candlestick(
                GetHistoryCandlestickRequest::new(
                    "BTCUSDT",
                    CandlestickGranularity::OneMinute,
                    end_time,
                )
                .limit(10),
            )
            .await;
        assert!(
            response.is_ok(),
            "Failed to get history candlestick: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_recent_trades() {
        let client = create_test_client();
        let response = client
            .get_recent_trades(GetRecentTradesRequest::new("BTCUSDT").limit(10))
            .await;
        assert!(
            response.is_ok(),
            "Failed to get recent trades: {:?}",
            response
        );
    }

    #[tokio::test]
    async fn test_get_market_trades() {
        let client = create_test_client();
        // Use current time minus 1 hour for startTime
        let now = Utc::now();
        let start_time = (now - Duration::hours(1)).timestamp_millis() as u64;
        let end_time = now.timestamp_millis() as u64;
        let response = client
            .get_market_trades(
                GetMarketTradesRequest::new("BTCUSDT")
                    .start_time(start_time)
                    .end_time(end_time)
                    .limit(10),
            )
            .await;
        assert!(
            response.is_ok(),
            "Failed to get market trades: {:?}",
            response
        );
    }
}
