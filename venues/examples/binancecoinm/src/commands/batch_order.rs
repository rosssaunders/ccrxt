use std::sync::Arc;

use anyhow::{anyhow, Result};
use venues::binance::coinm::PrivateRestClient;
use venues::binance::coinm::{BatchOrderRequest, BatchOrderResult, PlaceBatchOrdersRequest};
use venues::binance::coinm::{OrderSide, OrderType, TimeInForce};

pub async fn handle_batch_order_command(
    client: Arc<PrivateRestClient>,
    symbol: String,
    side: String,
    order_type: String,
    quantity: f64,
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

    // Create a batch order request
    let batch_req = BatchOrderRequest {
        symbol: symbol.clone(),
        side,
        position_side: None,
        order_type,
        time_in_force: Some(TimeInForce::GTC),
        quantity: quantity.to_string(),
        reduce_only: None,
        price: price.map(|p| p.to_string()),
        new_client_order_id: None,
        stop_price: None,
        activation_price: None,
        callback_rate: None,
        working_type: None,
        price_protect: None,
        new_order_resp_type: None,
        price_match: None,
        self_trade_prevention_mode: None,
    };

    let batch_req2 = BatchOrderRequest {
        symbol: symbol.clone(),
        side,
        position_side: None,
        order_type,
        time_in_force: Some(TimeInForce::GTC),
        quantity: "200".to_string(),
        reduce_only: None,
        price: price.map(|p| p.to_string()),
        new_client_order_id: None,
        stop_price: None,
        activation_price: None,
        callback_rate: None,
        working_type: None,
        price_protect: None,
        new_order_resp_type: None,
        price_match: None,
        self_trade_prevention_mode: None,
    };

    let request = PlaceBatchOrdersRequest {
        batch_orders: vec![batch_req, batch_req2],
        recv_window: None,
        timestamp: now,
    };

    let response = client.place_batch_orders(request).await?;

    // Print the results
    println!("Batch order placed for {}:", symbol);
    for (i, order) in response.data.iter().enumerate() {
        match order {
            BatchOrderResult::Ok(order) => {
                println!("\nOrder {}:", i + 1);
                println!("  Order ID: {}", order.order_id);
                println!("  Client Order ID: {}", order.client_order_id);
                println!("  Status: {:?}", order.status);
                println!("  Price: {}", order.price);
                println!("  Quantity: {}", order.orig_qty);
                println!("  Executed Quantity: {}", order.executed_qty);
                println!("  Average Price: {}", order.avg_price);
            }
            BatchOrderResult::Err(err) => {
                println!("\nOrder {}:", i + 1);
                println!("  Error: {:?}", err);
            }
        }
    }

    Ok(())
}
