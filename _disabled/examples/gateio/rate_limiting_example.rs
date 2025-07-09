use dotenv::dotenv;
use std::env;
use tokio::time::{sleep, Duration};
use venues::gateio::{
    PrivateRestClient, PublicRestClient, GateIoError,
    private::rest::ListOrdersRequest,
    public::rest::TickersRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let api_key = env::var("GATEIO_API_KEY")
        .expect("GATEIO_API_KEY environment variable not set");
    let api_secret = env::var("GATEIO_API_SECRET")
        .expect("GATEIO_API_SECRET environment variable not set");

    // Initialize clients
    let public_client = PublicRestClient::new(false)?;
    let private_client = PrivateRestClient::new(api_key, api_secret, false)?;

    println!("=== Gate.io Rate Limiting Examples ===\n");

    // 1. Demonstrate rate limiting behavior
    println!("1. Testing rate limits with rapid requests...");
    
    let start_time = std::time::Instant::now();
    let mut successful_requests = 0;
    let mut rate_limited_requests = 0;
    
    // Make multiple rapid requests to test rate limiting
    for i in 1..=20 {
        match public_client.get_tickers(TickersRequest {
            currency_pair: Some("BTC_USDT".to_string()),
            timezone: None,
        }).await {
            Ok(_) => {
                successful_requests += 1;
                println!("   Request {}: Success", i);
            }
            Err(GateIoError::RateLimitExceeded { message }) => {
                rate_limited_requests += 1;
                println!("   Request {}: Rate limited - {}", i, message);
                
                // Wait before retrying
                sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                println!("   Request {}: Other error - {}", i, e);
            }
        }
        
        // Small delay between requests
        sleep(Duration::from_millis(100)).await;
    }
    
    let elapsed = start_time.elapsed();
    println!("   Results after {:.2}s:", elapsed.as_secs_f64());
    println!("   Successful: {}, Rate limited: {}", successful_requests, rate_limited_requests);

    // 2. Demonstrate error handling with retries
    println!("\n2. Demonstrating retry logic for rate-limited requests...");
    
    async fn make_request_with_retry(
        client: &PrivateRestClient,
        max_retries: u32,
    ) -> Result<Vec<venues::gateio::private::rest::Order>, GateIoError> {
        let mut retries = 0;
        
        loop {
            match client.list_orders(ListOrdersRequest {
                currency_pair: Some("BTC_USDT".to_string()),
                status: Some("finished".to_string()),
                limit: Some(5),
                ..Default::default()
            }).await {
                Ok(result) => return Ok(result),
                Err(e) if e.is_retryable() && retries < max_retries => {
                    retries += 1;
                    let delay = e.retry_delay_secs().unwrap_or(5);
                    println!("   Retry {} after {} seconds due to: {}", retries, delay, e);
                    sleep(Duration::from_secs(delay)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
    
    match make_request_with_retry(&private_client, 3).await {
        Ok(orders) => {
            println!("   Successfully retrieved {} orders with retry logic", orders.len());
        }
        Err(e) => {
            println!("   Failed after retries: {}", e);
        }
    }

    // 3. Demonstrate different endpoint categories and their limits
    println!("\n3. Testing different endpoint categories...");
    
    let endpoint_tests = vec![
        ("Public market data", "public endpoints"),
        ("Private account data", "spot_other"),
        ("Order management", "spot_order_placement"),
    ];
    
    for (description, category) in endpoint_tests {
        println!("   Testing {}: category '{}'", description, category);
        
        let start = std::time::Instant::now();
        
        // Test based on category
        let result = match category {
            "public endpoints" => {
                public_client.get_tickers(TickersRequest::default()).await.map(|t| t.len())
            }
            "spot_other" => {
                private_client.list_orders(ListOrdersRequest {
                    limit: Some(1),
                    ..Default::default()
                }).await.map(|o| o.len())
            }
            _ => {
                // For order placement category, just test account access
                private_client.get_spot_accounts(None).await.map(|a| a.len())
            }
        };
        
        match result {
            Ok(count) => {
                println!("     Success: {} items in {:.2}s", count, start.elapsed().as_secs_f64());
            }
            Err(e) => {
                println!("     Error: {} in {:.2}s", e, start.elapsed().as_secs_f64());
            }
        }
    }

    // 4. Rate limit monitoring
    println!("\n4. Rate limit monitoring...");
    
    // This would require access to the rate limiter instance
    println!("   Rate limiting features:");
    println!("   • Automatic throttling based on endpoint categories");
    println!("   • Different limits for trading vs. data endpoints");
    println!("   • Built-in backoff and retry mechanisms");
    println!("   • Usage tracking and warnings");

    // 5. Best practices for rate limiting
    println!("\n5. Rate Limiting Best Practices:");
    println!("   • Use batch endpoints when available (e.g., batch_orders)");
    println!("   • Implement exponential backoff for retries");
    println!("   • Monitor rate limit headers in responses");
    println!("   • Cache frequently accessed static data");
    println!("   • Use WebSocket for real-time data instead of polling");
    println!("   • Distribute requests across time to avoid bursts");
    println!("   • Different endpoints have different rate limits:");
    println!("     - Public: 1000 req/10s");
    println!("     - Spot trading: 10-200 req/s depending on operation");
    println!("     - Futures: 200 req/10s");
    println!("     - Wallet: 100 req/10s");

    // 6. Error categorization
    println!("\n6. Error handling strategies:");
    
    let error_examples = vec![
        ("Rate limit exceeded", "Retry with exponential backoff"),
        ("Authentication failed", "Check API keys, don't retry"),
        ("Invalid parameter", "Fix request, don't retry"),
        ("Network timeout", "Retry with short delay"),
        ("Maintenance mode", "Retry with longer delay"),
    ];
    
    for (error_type, strategy) in error_examples {
        println!("   {}: {}", error_type, strategy);
    }

    println!("\n=== Rate limiting examples completed successfully! ===");
    
    Ok(())
}

/*
To run this example:

1. Create a .env file in the venues directory with:
   GATEIO_API_KEY=your_api_key_here
   GATEIO_API_SECRET=your_api_secret_here

2. Run with:
   cargo run --example gateio_rate_limiting_example

This example demonstrates:
- How rate limiting works in practice
- Proper error handling and retry logic
- Different rate limits for different endpoint categories
- Best practices for staying within limits

Note: This example may trigger rate limits intentionally for demonstration.
In production, use proper throttling to avoid hitting limits.
*/