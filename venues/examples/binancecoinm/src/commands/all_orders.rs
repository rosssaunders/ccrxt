use venues::binance::coinm::AllOrdersRequest;
use venues::binance::coinm::PrivateRestClient;

pub async fn run_all_orders(client: &PrivateRestClient, symbol: String, limit: u32) {
    let params = AllOrdersRequest {
        symbol: Some(symbol),
        pair: None,
        order_id: None,
        start_time: None,
        end_time: None,
        limit: Some(limit),
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };
    match client.get_all_orders(params).await {
        Ok(orders) => {
            println!("All Orders:");
            for order in orders.data {
                println!("{:#?}", order);
            }
        }
        Err(e) => eprintln!("Error fetching all orders: {e:?}"),
    }
}
