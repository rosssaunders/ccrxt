//! Example: Binance USDM public REST API endpoints
//
// This example demonstrates how to use the public endpoints:
// - ping
// - get server time
// - get order book for BTCUSDT
//
// No credentials are required.
//
// To run:
// ```sh
// cargo run --example usdm_public_endpoints_example
// ```

use reqwest::Client;
use tokio::runtime::Runtime;
use venues::binance::usdm::RateLimiter;
use venues::binance::usdm::public::rest::{RestClient, public_endpoints};

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let client = RestClient::new(
            "https://fapi.binance.com",
            Client::new(),
            RateLimiter::default(),
        );

        // Test ping
        client.ping().await.expect("Ping failed");
        println!("Ping successful");

        // Get server time
        let time = client
            .get_server_time()
            .await
            .expect("Failed to get server time");
        println!("Server time: {}", time.serverTime);

        // Get order book for BTCUSDT
        let order_book = client
            .get_order_book("BTCUSDT", Some(5))
            .await
            .expect("Failed to get order book");
        println!("Order book (top 5):");
        println!("Bids: {:?}", order_book.bids);
        println!("Asks: {:?}", order_book.asks);
    });
}
