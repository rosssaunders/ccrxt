use venues::binance::coinm::{PositionRiskRequest, PrivateRestClient};

pub async fn run_position_risk(client: &PrivateRestClient) {
    // Example: fetch all positions
    let params = PositionRiskRequest {
        margin_asset: None,
        pair: None,
        recv_window: None,
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    match client.get_position_risk(params).await {
        Ok(positions) => {
            println!("Position Risk:");
            for pos in positions.data {
                println!("{pos:#?}");
            }
        }
        Err(e) => eprintln!("Error fetching position risk: {e:?}"),
    }
}
