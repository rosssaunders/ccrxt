use std::time::Duration;
use tokio::time::sleep;
use venues::cryptocom::{EndpointType, RateLimitError, RateLimiter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Crypto.com Rate Limiter Example");
    println!("================================");

    // Create a rate limiter
    let limiter = RateLimiter::new();

    // Example 1: Basic usage with different endpoint types
    println!("\n1. Basic rate limiting example:");

    let endpoints_to_test = vec![
        EndpointType::PrivateCreateOrder,
        EndpointType::PrivateGetOrderDetail,
        EndpointType::PublicGetBook,
        EndpointType::UserApi,
    ];

    for endpoint in endpoints_to_test {
        let rate_limit = endpoint.rate_limit();
        println!(
            "   {:?}: {} requests per {:?}",
            endpoint, rate_limit.max_requests, rate_limit.window
        );

        // Check if we can make a request
        match limiter.check_limits(endpoint).await {
            | Ok(()) => {
                println!("   ✓ Request allowed");
                limiter.increment_request(endpoint).await;
            },
            | Err(e) => {
                println!("   ✗ Request denied: {}", e);
            },
        }
    }

    // Example 2: Testing rate limit enforcement
    println!("\n2. Rate limit enforcement example:");
    let endpoint = EndpointType::PrivateGetTrades; // 1 request per second

    println!("   Testing PrivateGetTrades (1 request per second):");

    // First request should succeed
    match limiter.check_limits(endpoint).await {
        | Ok(()) => {
            println!("   ✓ First request allowed");
            limiter.increment_request(endpoint).await;
        },
        | Err(e) => println!("   ✗ First request denied: {}", e),
    }

    // Second request should fail
    match limiter.check_limits(endpoint).await {
        | Ok(()) => {
            println!("   ✗ Second request unexpectedly allowed");
        },
        | Err(e) => println!("   ✓ Second request correctly denied: {}", e),
    }

    // Example 3: Using path-based endpoint detection
    println!("\n3. Path-based endpoint detection:");
    let test_paths = vec![
        "private/create-order",
        "private/get-order-detail",
        "public/get-book",
        "public/staking/get-products",
        "private/some-other-endpoint",
    ];

    for path in test_paths {
        let endpoint_type = EndpointType::from_path(path);
        let rate_limit = endpoint_type.rate_limit();
        println!(
            "   Path '{}' -> {:?} ({} req/{:?})",
            path, endpoint_type, rate_limit.max_requests, rate_limit.window
        );
    }

    // Example 4: Usage statistics
    println!("\n4. Usage statistics:");
    let endpoint = EndpointType::PrivateOther; // 3 requests per 100ms

    // Make some requests
    for i in 1..=3 {
        limiter.increment_request(endpoint).await;
        let (current, max) = limiter.get_usage(endpoint).await;
        println!("   After {} request(s): {}/{} used", i, current, max);
    }

    // Example 5: Rate limit recovery
    println!("\n5. Rate limit recovery example:");
    let endpoint = EndpointType::PrivateGetTrades; // 1 request per second

    // This should fail since we made a request earlier
    match limiter.check_limits(endpoint).await {
        | Ok(()) => println!("   ✗ Request unexpectedly allowed"),
        | Err(e) => println!("   ✓ Request correctly denied: {}", e),
    }

    println!("   Waiting 1.1 seconds for rate limit to reset...");
    sleep(Duration::from_millis(1100)).await;

    // Clean up old timestamps
    limiter.cleanup_old_timestamps().await;

    // Should work now
    match limiter.check_limits(endpoint).await {
        | Ok(()) => {
            println!("   ✓ Request allowed after waiting");
            limiter.increment_request(endpoint).await;
        },
        | Err(e) => println!("   ✗ Request still denied: {}", e),
    }

    // Example 6: Error handling patterns
    println!("\n6. Error handling patterns:");
    let endpoint = EndpointType::PrivateCreateOrder; // 15 requests per 100ms

    // Fill up the rate limit
    for _ in 0..15 {
        limiter.increment_request(endpoint).await;
    }

    // Try one more request that should fail
    match limiter.check_limits(endpoint).await {
        | Ok(()) => {
            println!("   Making API call...");
        },
        | Err(RateLimitError::RateLimitExceeded {
            endpoint,
            current,
            max,
            window,
        }) => {
            println!("   Rate limit exceeded for {:?}!", endpoint);
            println!("   Current usage: {}/{} requests", current, max);
            println!("   Window: {:?}", window);
            println!("   Recommended action: Wait and retry with exponential backoff");
        },
    }

    println!("\nExample completed successfully!");
    Ok(())
}
