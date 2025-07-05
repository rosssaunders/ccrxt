use std::env;

use venues::gateio::{
    OrderStatus, Result,
    private::rest::{
        RestClient, account_book::GetAccountBookRequest, list_open_orders::ListOpenOrdersRequest,
        list_orders::ListOrdersRequest, spot_trades::GetMyTradesRequest,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load API credentials from environment variables
    // You need to set these in your .env file:
    // GATEIO_API_KEY=your_api_key_here
    // GATEIO_API_SECRET=your_api_secret_here
    dotenv::dotenv().ok();

    let api_key = env::var("GATEIO_API_KEY").expect("GATEIO_API_KEY environment variable not set");
    let api_secret =
        env::var("GATEIO_API_SECRET").expect("GATEIO_API_SECRET environment variable not set");

    // Create a new private REST client (using testnet for safety)
    let client = RestClient::new(api_key, api_secret, true)?;

    println!("Gate.io Private Trading Example");
    println!("===============================");

    // ===== SPOT ACCOUNT MANAGEMENT =====
    println!("\n💰 === SPOT ACCOUNT INFORMATION ===");

    // Get all spot account balances
    println!("\n📊 Fetching spot account balances...");
    match client.list_spot_accounts(None).await {
        Ok(accounts) => {
            let non_zero_accounts: Vec<_> = accounts
                .iter()
                .filter(|acc| {
                    let available: f64 = acc.available.parse().unwrap_or(0.0);
                    let locked: f64 = acc.locked.parse().unwrap_or(0.0);
                    available > 0.0 || locked > 0.0
                })
                .collect();

            println!(
                "✅ Found {} total currencies, {} with balance",
                accounts.len(),
                non_zero_accounts.len()
            );

            for account in non_zero_accounts.iter().take(10) {
                println!(
                    "  💰 {}: available={}, locked={}",
                    account.currency, account.available, account.locked
                );
            }
        }
        Err(e) => println!("❌ Error fetching accounts: {}", e),
    }

    // Get account history
    println!("\n📜 Fetching recent account history...");
    let account_book_request = GetAccountBookRequest {
        limit: Some(5),
        ..Default::default()
    };

    match client.get_account_book(account_book_request).await {
        Ok(history) => {
            println!("✅ Found {} recent account changes", history.len());
            for entry in history.iter().take(5) {
                println!(
                    "  📝 {} {}: {} (balance: {})",
                    entry.currency, entry.entry_type, entry.change, entry.balance
                );
            }
        }
        Err(e) => println!("❌ Error fetching account history: {}", e),
    }

    // ===== ORDER MANAGEMENT =====
    println!("\n📋 === ORDER MANAGEMENT ===");

    // Get open orders
    println!("\n🔍 Fetching open orders...");
    let open_orders_request = ListOpenOrdersRequest {
        currency_pair: None,
        page: None,
        limit: None,
        side: None,
        account: None,
    };
    match client.list_open_orders(open_orders_request).await {
        Ok(orders) => {
            println!("✅ Found {} open orders", orders.len());
            for order in orders.iter().take(5) {
                println!(
                    "  📊 {} {}: {} {} @ {} (status: {:?})",
                    order.currency_pair,
                    order.side,
                    order.amount,
                    order.order_type,
                    order.price,
                    order.status
                );
            }
        }
        Err(e) => println!("❌ Error fetching open orders: {}", e),
    }

    // Get order history
    println!("\n📚 Fetching order history...");
    let orders_request = ListOrdersRequest {
        currency_pair: None,
        status: Some(OrderStatus::Closed),
        page: None,
        limit: Some(5),
        account: None,
        from: None,
        to: None,
        side: None,
    };
    match client.list_orders(orders_request).await {
        Ok(orders) => {
            println!("✅ Found {} historical orders", orders.len());
            for order in orders.iter().take(3) {
                println!(
                    "  📈 {} {}: {} {} @ {} (filled: {})",
                    order.currency_pair,
                    order.side,
                    order.amount,
                    order.order_type,
                    order.price,
                    order.filled_amount
                );
            }
        }
        Err(e) => println!("❌ Error fetching order history: {}", e),
    }

    // ===== TRADING EXAMPLE (COMMENTED OUT FOR SAFETY) =====
    println!("\n🚫 === TRADING EXAMPLE (DISABLED FOR SAFETY) ===");
    println!("⚠️  Uncomment the code below to test order placement");
    println!("⚠️  Make sure you're on testnet and understand the risks!");

    /*
    // Example: Create a limit buy order for BTC_USDT
    println!("\n📤 Creating a test limit buy order...");
    let order_request = CreateOrderRequest::limit(
        "BTC_USDT".to_string(),
        OrderSide::Buy,
        "0.001".to_string(),  // Very small amount
        "30000".to_string(),  // Low price (unlikely to fill)
    ).with_time_in_force(TimeInForce::GoodTillCanceled)
     .with_text("test_order_from_rust".to_string());

    match client.create_order(order_request).await {
        Ok(order) => {
            println!("✅ Order created successfully!");
            println!("  📊 Order ID: {}", order.id);
            println!("  💰 Amount: {} {} @ {}", order.amount, order.currency_pair, order.price);

            // Cancel the order immediately
            println!("\n🗑️ Cancelling the test order...");
            match client.cancel_order(&order.id, &order.currency_pair).await {
                Ok(cancelled_order) => {
                    println!("✅ Order cancelled successfully!");
                    println!("  📊 Status: {:?}", cancelled_order.status);
                }
                Err(e) => println!("❌ Error cancelling order: {}", e),
            }
        }
        Err(e) => println!("❌ Error creating order: {}", e),
    }
    */

    // ===== TRADING HISTORY =====
    println!("\n📊 === TRADING HISTORY ===");

    // Get recent trades
    println!("\n🔄 Fetching recent trades...");
    let trades_request = GetMyTradesRequest {
        limit: Some(5),
        ..Default::default()
    };

    match client.get_my_trades(trades_request).await {
        Ok(trades) => {
            println!("✅ Found {} recent trades", trades.len());
            for trade in trades.iter().take(5) {
                let side_emoji = if trade.side == "buy" { "🟢" } else { "🔴" };
                let role_emoji = if trade.role == "maker" { "👥" } else { "⚡" };

                println!(
                    "  {} {} {} {}: {} @ {} (fee: {} {})",
                    side_emoji,
                    role_emoji,
                    trade.side,
                    trade.currency_pair,
                    trade.amount,
                    trade.price,
                    trade.fee,
                    trade.fee_currency
                );
            }
        }
        Err(e) => println!("❌ Error fetching trades: {}", e),
    }

    // ===== UNIFIED ACCOUNT (if enabled) =====
    println!("\n🔗 === UNIFIED ACCOUNT ===");

    // Get unified account info
    println!("\n🏦 Fetching unified account information...");
    match client.get_unified_account(None).await {
        Ok(account) => {
            println!("✅ Unified account information:");
            println!("  👤 User ID: {}", account.user_id);
            println!("  🔒 Locked: {}", account.locked);
            println!("  💰 Total balance: {} USDT", account.total);
            println!("  📊 Risk ratio: {}", account.risk);

            let non_zero_balances: Vec<_> = account
                .balances
                .iter()
                .filter(|(_, balance)| {
                    balance.available.parse::<f64>().unwrap_or(0.0) > 0.0 
                        || balance.borrowed.parse::<f64>().unwrap_or(0.0) > 0.0
                })
                .collect();

            println!("  📈 Currencies with balance: {}", non_zero_balances.len());
            for (currency, balance) in non_zero_balances.iter().take(5) {
                println!(
                    "    💰 {}: available={}, borrowed={}",
                    currency, balance.available, balance.borrowed
                );
            }
        }
        Err(e) => println!(
            "❌ Error fetching unified account (maybe not enabled): {}",
            e
        ),
    }

    // Get supported loan currencies
    println!("\n💳 Fetching supported loan currencies...");
    println!("⚠️  This feature requires specific API methods that may not be available");
    // Note: The client may not expose all possible Gate.io API endpoints
    // You can implement additional methods as needed for your use case

    // ===== UTILITY FUNCTIONS DEMO =====
    println!("\n🛠️ === UTILITY FUNCTIONS DEMO ===");
    println!("Note: Some advanced trading features may require specific API methods");
    println!("that are not yet implemented in this client.");

    println!("\n✅ Gate.io private trading example completed!");
    println!("\n💡 Tips:");
    println!("  • Always test on testnet first");
    println!("  • Check your API key permissions");
    println!("  • Monitor your rate limits");
    println!("  • Use proper error handling in production");

    Ok(())
}
