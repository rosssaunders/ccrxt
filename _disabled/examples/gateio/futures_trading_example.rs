use dotenv::dotenv;
use std::env;
use venues::gateio::{
    PrivateRestClient,
    private::rest::{
        futures::{
            FuturesAccountsRequest, FuturesPositionsRequest, CreateFuturesOrderRequest,
            ListFuturesOrdersRequest, SetLeverageRequest,
        },
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let api_key = env::var("GATEIO_API_KEY")
        .expect("GATEIO_API_KEY environment variable not set");
    let api_secret = env::var("GATEIO_API_SECRET")
        .expect("GATEIO_API_SECRET environment variable not set");

    // Initialize private client
    let private_client = PrivateRestClient::new(api_key, api_secret, false)?;

    println!("=== Gate.io Futures Trading Examples ===\n");

    // Settlement currencies to demonstrate
    let settlements = vec!["usdt", "btc"];

    for settle in &settlements {
        println!("=== {} Futures ===", settle.to_uppercase());

        // 1. Account Information
        println!("1. Getting futures account information...");
        
        let account = private_client.get_futures_accounts(FuturesAccountsRequest {
            settle: settle.to_string(),
        }).await?;
        
        println!("   Total Balance: {} {}", account.total, account.currency);
        println!("   Available: {} {}", account.available, account.currency);
        println!("   Position Margin: {} {}", account.position_margin, account.currency);
        println!("   Order Margin: {} {}", account.order_margin, account.currency);
        println!("   Unrealized PnL: {} {}", account.unrealised_pnl, account.currency);
        
        if account.in_dual_mode {
            println!("   Account is in dual mode");
        }

        // 2. Positions
        println!("\n2. Getting futures positions...");
        
        let positions = private_client.get_futures_positions(FuturesPositionsRequest {
            settle: settle.to_string(),
            contract: None,
            holding: None,
            limit: Some(10),
            offset: None,
        }).await?;
        
        println!("   Active positions: {}", positions.len());
        
        for position in &positions {
            if position.size != 0 {
                let side = if position.size > 0 { "LONG" } else { "SHORT" };
                println!("   {} {}: Size {} @ {} (PnL: {}, Margin: {})", 
                    side, position.contract, position.size.abs(), 
                    position.entry_price, position.unrealised_pnl, position.margin);
                println!("     Leverage: {}x, Liq Price: {}", 
                    position.leverage, position.liq_price);
            }
        }
        
        if positions.is_empty() {
            println!("   No active positions");
        }

        // 3. Recent Orders
        println!("\n3. Getting recent futures orders...");
        
        let orders = private_client.list_futures_orders(ListFuturesOrdersRequest {
            settle: settle.to_string(),
            status: Some("finished".to_string()),
            contract: None,
            limit: Some(5),
            ..Default::default()
        }).await?;
        
        println!("   Recent finished orders: {}", orders.len());
        
        for order in &orders {
            let side = if order.size > 0 { "BUY" } else { "SELL" };
            println!("   Order #{}: {} {} {} @ {} (Status: {})",
                order.id, side, order.size.abs(), order.contract,
                order.price.as_deref().unwrap_or("market"), order.status);
            
            if let Some(finish_reason) = &order.finish_as {
                println!("     Finished as: {}", finish_reason);
            }
        }

        // 4. Example Order Creation (commented out for safety)
        println!("\n4. Order creation example (commented out for safety)...");
        
        /*
        // Example: Create a limit order for BTC perpetual
        let contract = if settle == "usdt" { "BTC_USDT" } else { "BTC_USD" };
        
        let order_request = CreateFuturesOrderRequest {
            settle: settle.to_string(),
            contract: contract.to_string(),
            size: 100, // $100 worth (positive for long, negative for short)
            price: Some("30000".to_string()), // Below market price for safety
            tif: Some("gtc".to_string()),
            text: Some("example_futures_order".to_string()),
            reduce_only: Some(false),
            close: Some(false),
            iceberg: None,
            auto_size: None,
        };
        
        let new_order = private_client.create_futures_order(order_request).await?;
        println!("   Created futures order: #{}", new_order.id);
        
        // Cancel the order immediately
        let cancelled_order = private_client.cancel_futures_order(settle, &new_order.id.to_string()).await?;
        println!("   Cancelled order: #{} (Status: {})", cancelled_order.id, cancelled_order.status);
        */

        // 5. Leverage Management Example (commented out for safety)
        println!("\n5. Leverage management example (commented out for safety)...");
        
        /*
        // Example: Set leverage for BTC perpetual
        let leverage_request = SetLeverageRequest {
            settle: settle.to_string(),
            contract: contract.to_string(),
            leverage: "10".to_string(), // 10x leverage
            cross_leverage_limit: None,
        };
        
        let leverage_response = private_client.set_position_leverage(leverage_request).await?;
        println!("   Set leverage for {}: {}x", contract, leverage_response.leverage);
        */

        println!("   Leverage management allows setting position leverage from 1x to 100x");
        println!("   Higher leverage increases both potential profits and losses");

        println!();
    }

    // 6. Risk Management Tips
    println!("=== Risk Management Tips ===");
    println!("• Always use stop-loss orders in volatile markets");
    println!("• Start with lower leverage (2x-5x) until experienced");
    println!("• Never risk more than you can afford to lose");
    println!("• Monitor liquidation prices closely");
    println!("• Use position sizing to manage risk");
    println!("• Consider using reduce-only orders to close positions");

    println!("\n=== Futures examples completed successfully! ===");
    
    Ok(())
}

/*
To run this example:

1. Create a .env file in the venues directory with:
   GATEIO_API_KEY=your_api_key_here
   GATEIO_API_SECRET=your_api_secret_here

2. Run with:
   cargo run --example gateio_futures_trading_example

Note: This example only reads account data and doesn't place orders.
Uncomment order creation examples only if you want to place actual orders.

Important: Futures trading involves significant risk of loss. Only trade with funds
you can afford to lose and always use proper risk management.
*/