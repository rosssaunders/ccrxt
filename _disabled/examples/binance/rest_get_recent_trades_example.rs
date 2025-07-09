//! Example: Get recent trades for a symbol using Binance USDM REST API
//!
//! This example demonstrates how to fetch recent trades for BTCUSDT using the public endpoint.
//!
//! To run:
//! ```sh
//! cargo run --example rest_get_recent_trades_example
//! ```

use reqwest::Client;
use tokio::runtime::Runtime;
use venues::binance::usdm::RateLimiter;
use venues::binance::usdm::public::rest::{
    RestClient,
    recent_trades::{RecentTrade, RecentTradesRequest},
};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = RestClient::new(
            "https://fapi.binance.com",
            Client::new(),
            RateLimiter::default(),
        );

        // Fetch recent trades for BTCUSDT
        let params = RecentTradesRequest {
            symbol: "BTCUSDT".into(),
            limit: Some(5),
        };
        let trades: Vec<RecentTrade> = client
            .get_recent_trades(params)
            .await
            .expect("Failed to get recent trades");
        for trade in trades {
            println!(
                "Trade ID: {}, Price: {}, Qty: {}, Time: {}",
                trade.id, trade.price, trade.qty, trade.time
            );
        }
    });
}
