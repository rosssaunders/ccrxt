use anyhow::{anyhow, Result};
use std::sync::Arc;
use venues::binance::coinm::NewOrderRequest;
use venues::binance::coinm::PrivateRestClient;
use venues::binance::coinm::{OrderSide, OrderType};

/// Example command to place a new order using the CLI.
pub async fn handle_order_command(
    client: Arc<PrivateRestClient>,
    symbol: String,
    side: String,
    order_type: String,
    quantity: Option<f64>,
    price: Option<f64>,
) -> Result<()> {
    let side = match side.to_uppercase().as_str() {
        "BUY" => OrderSide::Buy,
        "SELL" => OrderSide::Sell,
        _ => return Err(anyhow!("Invalid side. Must be 'BUY' or 'SELL'")),
    };
    let order_type = match order_type.to_uppercase().as_str() {
        "LIMIT" => OrderType::Limit,
        "MARKET" => OrderType::Market,
        _ => return Err(anyhow!("Invalid order type. Must be 'LIMIT' or 'MARKET'")),
    };
    let now = chrono::Utc::now().timestamp_millis() as u64;
    let req = NewOrderRequest {
        symbol,
        side,
        order_type,
        time_in_force: None, // Some(TimeInForce::GTC),
        quantity: quantity.map(|q| q.to_string()),
        price: price.map(|p| p.to_string()),
        timestamp: now,
        new_client_order_id: None,        // Optional, can be set if needed
        position_side: None,              // Optional, can be set if needed
        reduce_only: None,                // Optional, can be set if needed
        stop_price: None,                 // Optional, can be set if needed
        close_position: None,             // Optional, can be set if needed
        activation_price: None,           // Optional, can be set if needed
        callback_rate: None,              // Optional, can be set if needed
        working_type: None,               // Optional, can be set if needed
        price_protect: None,              // Optional, can be set if needed
        recv_window: None,                // Optional, can be set if needed
        new_order_resp_type: None,        // Optional, can be set if needed
        price_match: None,                // Optional, can be set if needed
        self_trade_prevention_mode: None, // Optional, can be set if needed
    };
    let resp = client.post_order(req).await?;
    println!(
        "Order placed: order_id={}, status={}, executed_qty={}",
        resp.data.order_id, resp.data.status, resp.data.executed_qty
    );
    Ok(())
}
