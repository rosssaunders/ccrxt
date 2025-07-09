use dotenv::dotenv;
use std::env;
use venues::gateio::{
    PrivateRestClient, PublicRestClient,
    private::rest::{
        CreateOrderRequest, ListOrdersRequest, CreatePriceOrderRequest,
        margin_accounts::MarginAccountsRequest, margin_loans::CreateLoanRequest,
    },
    public::rest::{TickersRequest, TradingFeeRequest},
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
    let public_client = PublicRestClient::new(false)?; // false = live trading, true = testnet
    let private_client = PrivateRestClient::new(api_key, api_secret, false)?;

    println!("=== Gate.io Spot Trading Examples ===\n");

    // 1. Public Market Data
    println!("1. Getting market data...");
    
    // Get all tickers
    let tickers = public_client.get_tickers(TickersRequest::default()).await?;
    println!("   Total trading pairs: {}", tickers.len());
    
    // Get specific ticker
    let btc_ticker = public_client.get_tickers(TickersRequest {
        currency_pair: Some("BTC_USDT".to_string()),
        timezone: None,
    }).await?;
    
    if let Some(ticker) = btc_ticker.first() {
        println!("   BTC/USDT Price: {} USDT", ticker.last);
        println!("   24h Change: {}%", ticker.change_percentage);
        println!("   24h Volume: {} BTC", ticker.base_volume);
    }
    
    // Get trading fees
    let fees = public_client.get_trading_fee(TradingFeeRequest {
        currency_pair: Some("BTC_USDT".to_string()),
    }).await?;
    println!("   BTC/USDT Trading Fees - Maker: {}%, Taker: {}%", fees.maker_fee, fees.taker_fee);

    // 2. Account Information
    println!("\n2. Getting account information...");
    
    // Get spot account balances
    let balances = private_client.get_spot_accounts(None).await?;
    println!("   Account currencies: {}", balances.len());
    
    for balance in balances.iter().take(5) { // Show first 5 currencies
        if balance.available.parse::<f64>().unwrap_or(0.0) > 0.0 {
            println!("   {}: {} available, {} locked", 
                balance.currency, balance.available, balance.locked);
        }
    }

    // 3. Order Management
    println!("\n3. Order management examples...");
    
    // List recent orders
    let recent_orders = private_client.list_orders(ListOrdersRequest {
        currency_pair: Some("BTC_USDT".to_string()),
        status: Some("finished".to_string()),
        limit: Some(5),
        ..Default::default()
    }).await?;
    
    println!("   Recent finished orders: {}", recent_orders.len());
    for order in &recent_orders {
        println!("   Order #{}: {} {} @ {} (Status: {})", 
            order.id, order.side, order.amount, order.price.as_deref().unwrap_or("market"), order.status);
    }

    // Example: Create a limit buy order (commented out for safety)
    /*
    let order_request = CreateOrderRequest {
        currency_pair: "BTC_USDT".to_string(),
        order_type: "limit".to_string(),
        account: "spot".to_string(),
        side: "buy".to_string(),
        amount: "0.001".to_string(), // 0.001 BTC
        price: Some("30000".to_string()), // $30,000 (below market for safety)
        time_in_force: "gtc".to_string(),
        text: Some("example_order".to_string()),
        ..Default::default()
    };
    
    let new_order = private_client.create_order(order_request).await?;
    println!("   Created order: #{}", new_order.id);
    
    // Cancel the order immediately
    let cancelled_order = private_client.cancel_order(&new_order.id, "BTC_USDT", "spot").await?;
    println!("   Cancelled order: #{} (Status: {})", cancelled_order.id, cancelled_order.status);
    */

    // 4. Price Orders (Conditional Orders)
    println!("\n4. Price order (conditional order) example...");
    
    // Example: Create a price order (stop-loss) - commented out for safety
    /*
    let price_order_request = CreatePriceOrderRequest {
        currency_pair: "BTC_USDT".to_string(),
        order_type: "limit".to_string(),
        account: "spot".to_string(),
        side: "sell".to_string(),
        amount: "0.001".to_string(),
        price: Some("35000".to_string()),
        time_in_force: "gtc".to_string(),
        trigger_price: "40000".to_string(), // Trigger when BTC goes above $40k
        rule: ">=".to_string(),
        expiration: Some(86400), // 24 hours
        text: Some("stop_loss_example".to_string()),
    };
    
    let price_order = private_client.create_price_order(price_order_request).await?;
    println!("   Created price order: #{}", price_order.id);
    */

    // List existing price orders
    let price_orders = private_client.list_price_orders(Default::default()).await?;
    println!("   Active price orders: {}", price_orders.len());

    // 5. Margin Trading Example
    println!("\n5. Margin trading information...");
    
    // Get margin accounts
    let margin_accounts = private_client.get_margin_accounts(MarginAccountsRequest {
        currency_pair: Some("BTC_USDT".to_string()),
    }).await?;
    
    for account in &margin_accounts {
        println!("   Margin Account {}: Risk Level {}", account.currency_pair, account.risk);
        println!("     Base ({}) - Available: {}, Borrowed: {}", 
            account.base.currency, account.base.available, account.base.borrowed);
        println!("     Quote ({}) - Available: {}, Borrowed: {}", 
            account.quote.currency, account.quote.available, account.quote.borrowed);
    }

    println!("\n=== Examples completed successfully! ===");
    
    Ok(())
}

/*
To run this example:

1. Create a .env file in the venues directory with:
   GATEIO_API_KEY=your_api_key_here
   GATEIO_API_SECRET=your_api_secret_here

2. Run with:
   cargo run --example gateio_spot_trading_example

Note: Uncomment the order creation examples only if you want to place actual orders.
The examples above only read data and won't modify your account.
*/