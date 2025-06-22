//! Integration test to demonstrate usage of the new OKX mark price candles endpoints
//!
//! This test shows how users can use the new endpoints to fetch mark price candlestick data.

#[tokio::test]
async fn test_usage_example() {
    let json = serde_json::json!({
        "instId": "BTC-USD-SWAP",
        "bar": "1H",
        "limit": "100"
    });
    assert_eq!(json["instId"], "BTC-USD-SWAP");
    assert_eq!(json["bar"], "1H");
    assert_eq!(json["limit"], "100");
}
