use futures::StreamExt;
use tokio_tungstenite::connect_async;

/// Debug example to see raw WebSocket messages from Binance
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Binance Raw WebSocket Debug");
    println!("===========================\n");
    
    // Connect directly to Binance WebSocket
    let url = "wss://stream.binance.com:9443/ws";
    let (mut ws_stream, _) = connect_async(url).await?;
    println!("✅ Connected to {}\n", url);
    
    // Subscribe to BTC/USDT trades
    let subscribe_msg = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": ["btcusdt@depth5@100ms"],
        "id": 1
    });
    
    use futures::SinkExt;
    ws_stream.send(tokio_tungstenite::tungstenite::Message::Text(subscribe_msg.to_string().into())).await?;
    println!("📤 Sent subscription request\n");
    
    // Read messages
    let mut count = 0;
    while let Some(msg) = ws_stream.next().await {
        match msg? {
            tokio_tungstenite::tungstenite::Message::Text(text) => {
                println!("📦 Message #{}:", count + 1);
                
                // Try to parse and pretty print
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    println!("{}\n", serde_json::to_string_pretty(&json)?);
                } else {
                    println!("Raw text: {}\n", text);
                }
                
                count += 1;
                if count >= 30 {
                    println!("Received {} messages, stopping...", count);
                    break;
                }
            }
            _ => {}
        }
    }
    
    Ok(())
}