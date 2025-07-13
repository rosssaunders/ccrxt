use venues::bybit::enums::{Category, Interval};
use venues::bybit::public::rest::RestClient;

#[tokio::main]
async fn main() {
    let client = RestClient::default();

    // Test premium index price kline
    let request = venues::bybit::public::rest::GetPremiumIndexPriceKlineRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        interval: Interval::Day,
        start: None,
        end: None,
        limit: Some(1),
    };

    match client.get_premium_index_price_kline(request).await {
        Ok(response) => {
            println!("Premium index price kline response:");
            println!("Symbol: {}", response.result.symbol);
            println!("Number of klines: {}", response.result.list.len());
            if !response.result.list.is_empty() {
                let kline = &response.result.list[0];
                println!(
                    "First kline: start={}, open={}, high={}, low={}, close={}",
                    kline.start_time,
                    kline.open_price,
                    kline.high_price,
                    kline.low_price,
                    kline.close_price
                );
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
