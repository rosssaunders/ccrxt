use venues::coinbase::advanced_trade::websocket::CoinbaseAdvancedTradeWebSocketAuthenticated;
use venues::websockets::{WebSocketConnection, BoxResult};
use futures::StreamExt;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Load environment variables
    dotenv().ok();
    
    // Get JWT token from environment
    let jwt = env::var("COINBASE_JWT_TOKEN")
        .expect("COINBASE_JWT_TOKEN must be set in .env file");
    
    // Example 1: Basic authenticated connection and user channel subscription
    async fn basic_auth_example(jwt: String) -> BoxResult<()> {
        // Create a new authenticated websocket instance
        let mut ws = CoinbaseAdvancedTradeWebSocketAuthenticated::new(jwt);
        
        // Connect to the websocket
        ws.connect().await?;
        
        // Subscribe to user channel for specific products
        let product_ids = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];
        ws.subscribe_user(Some(product_ids.clone())).await?;
        
        // Get the message stream
        let mut stream = ws.message_stream();
        
        // Process incoming messages
        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => println!("Received user message: {:?}", msg),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        
        Ok(())
    }

    // Example 2: Futures balance summary subscription
    async fn futures_balance_example(jwt: String) -> BoxResult<()> {
        let mut ws = CoinbaseAdvancedTradeWebSocketAuthenticated::new(jwt);
        ws.connect().await?;
        
        // Subscribe to futures balance summary
        ws.subscribe_futures_balance_summary().await?;
        
        let mut stream = ws.message_stream();
        
        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => println!("Received futures balance: {:?}", msg),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        
        Ok(())
    }

    // Example 3: Complete workflow with multiple subscriptions and cleanup
    async fn complete_workflow_example(jwt: String) -> BoxResult<()> {
        let mut ws = CoinbaseAdvancedTradeWebSocketAuthenticated::new(jwt);
        ws.connect().await?;
        
        // Subscribe to user channel for specific products
        let product_ids = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];
        ws.subscribe_user(Some(product_ids.clone())).await?;
        
        // Subscribe to futures balance summary
        ws.subscribe_futures_balance_summary().await?;
        
        // Send periodic pings to keep connection alive
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        let mut stream = ws.message_stream();
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Err(e) = ws.ping().await {
                        eprintln!("Error sending ping: {:?}", e);
                    }
                }
                message = stream.next() => {
                    match message {
                        Some(Ok(msg)) => println!("Received message: {:?}", msg),
                        Some(Err(e)) => eprintln!("Error: {:?}", e),
                        None => break,
                    }
                }
            }
        }
        
        // Cleanup subscriptions
        ws.unsubscribe_user(Some(product_ids)).await?;
        ws.unsubscribe_futures_balance_summary().await?;
        
        // Disconnect
        ws.disconnect().await?;
        
        Ok(())
    }

    // Example 4: Error handling and reconnection
    async fn error_handling_example(jwt: String) -> BoxResult<()> {
        let mut ws = CoinbaseAdvancedTradeWebSocketAuthenticated::new(jwt);
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 3;
        
        while retry_count < MAX_RETRIES {
            match ws.connect().await {
                Ok(_) => {
                    println!("Successfully connected to websocket");
                    break;
                }
                Err(e) => {
                    eprintln!("Connection attempt {} failed: {:?}", retry_count + 1, e);
                    retry_count += 1;
                    if retry_count < MAX_RETRIES {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2u64.pow(retry_count))).await;
                    }
                }
            }
        }
        
        if retry_count == MAX_RETRIES {
            return Err("Failed to connect after maximum retries".into());
        }
        
        // Subscribe to user channel
        ws.subscribe_user(Some(vec!["BTC-USD".to_string()])).await?;
        
        let mut stream = ws.message_stream();
        
        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => println!("Received message: {:?}", msg),
                Err(e) => {
                    eprintln!("Error received: {:?}", e);
                    // Handle specific error types here
                    match e.downcast_ref::<venues::coinbase::advanced_trade::CoinbaseAdvancedTradeError>() {
                        Some(venues::coinbase::advanced_trade::CoinbaseAdvancedTradeError::WebSocketError(_)) => {
                            // Handle websocket errors
                            break;
                        }
                        _ => {
                            // Handle other errors
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    // Run examples
    basic_auth_example(jwt.clone()).await?;
    futures_balance_example(jwt.clone()).await?;
    complete_workflow_example(jwt.clone()).await?;
    error_handling_example(jwt).await?;
    
    Ok(())
} 