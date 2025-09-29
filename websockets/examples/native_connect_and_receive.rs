//! Example: Native WebSocket connect, send, and receive (binary/text)
//!
//! What it demonstrates:
//! - Creating a native WebSocket client (tokio-tungstenite based)
//! - Using ConnectionController to track state and receive events
//! - Handling binary (Bytes) and text messages distinctly
//! - When to use this: any venue that streams pub/sub data or supports request/response over WS
//!
//! Not a protocol example: this shows the transport layer only. For venue JSON protocols
//! (e.g., JSON-RPC), see the request/response correlation example.
//!
//! Requirements:
//! - Network access to the endpoint
//! - No credentials (public endpoints)
//!
//! Run (from repo root):
//!   cargo run --example native_connect_and_receive -- \
//!     --url wss://ws.kraken.com
//!
//! Notes:
//! - This example connects and reads a handful of frames, then exits.
//! - Replace the URL with a public WS endpoint of your choice.

use bytes::Bytes;
#[cfg(not(target_arch = "wasm32"))]
use websockets::native::NativeWebSocketClient;
use websockets::{WebSocketEvent, connection::ConnectionController};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure a default rustls CryptoProvider is installed for TLS.
    #[cfg(not(target_arch = "wasm32"))]
    {
        use rustls::crypto::CryptoProvider;
        if CryptoProvider::get_default().is_none() {
            let _ = CryptoProvider::install_default(rustls::crypto::ring::default_provider());
        }
    }
    let url = std::env::args()
        .skip_while(|a| a != "--url")
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("--url <WS_URL> required");
            std::process::exit(2);
        });

    #[cfg(not(target_arch = "wasm32"))]
    {
        let client = NativeWebSocketClient::new();
        let mut controller = ConnectionController::new(client);
        controller.connect(&url).await?;
        println!("state: {:?}", controller.state());

        // Optional: send a ping-like binary payload (depends on the server; often ignored)
        let _ = controller.send(Bytes::from_static(b"hello")).await;

        // Read a few frames then exit
        for _ in 0..5 {
            match controller.next_event().await? {
                Some(WebSocketEvent::Binary(b)) => println!("binary {} bytes", b.len()),
                Some(WebSocketEvent::Text(s)) => println!("text: {}", truncate(&s, 120)),
                Some(WebSocketEvent::Connected) => println!("connected"),
                Some(WebSocketEvent::Disconnected) => {
                    println!("disconnected");
                    break;
                }
                Some(WebSocketEvent::Error(e)) => {
                    eprintln!("error: {}", e);
                    break;
                }
                None => {}
            }
        }

        controller.disconnect().await?;
        println!("done");
    }

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s.to_owned()
    }
}
