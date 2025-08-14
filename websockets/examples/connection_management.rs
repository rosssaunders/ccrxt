use std::time::Duration;

use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use websockets::{
    builder::WebSocketClientBuilder, ConnectionState, DisconnectReason, VenueMessage,
    WebSocketConnection, WebSocketEvent,
};

/// Example message type for demonstration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ExampleMessage {
    Subscribe { channel: String },
    Unsubscribe { channel: String },
    Data { channel: String, data: String },
    Error { message: String },
}

impl VenueMessage for ExampleMessage {}

/// Example of user-controlled reconnection with exponential backoff
async fn maintain_connection(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut backoff = Duration::from_secs(1);
    let max_backoff = Duration::from_secs(60);
    let mut attempt = 0;

    loop {
        println!("Connection attempt #{}", attempt + 1);
        
        // Create a new WebSocket client
        let mut client = WebSocketClientBuilder::new(url)
            .header("User-Agent", "ExampleClient/1.0")
            .build::<ExampleMessage>()?;

        // Try to connect
        match client.connect().await {
            Ok(_) => {
                println!("Connected successfully!");
                backoff = Duration::from_secs(1); // Reset backoff on successful connection
                attempt = 0;

                // Handle the connection
                match handle_connection(&mut client).await {
                    Ok(_) => {
                        println!("Connection closed normally");
                    }
                    Err(e) => {
                        println!("Connection error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
                attempt += 1;
                
                // Decide if we should retry
                if attempt >= 5 {
                    println!("Max reconnection attempts reached. Giving up.");
                    return Err("Max reconnection attempts exceeded".into());
                }
                
                println!("Retrying in {:?}...", backoff);
                sleep(backoff).await;
                
                // Exponential backoff
                backoff = (backoff * 2).min(max_backoff);
            }
        }
    }
}

/// Handle an active WebSocket connection
async fn handle_connection(
    client: &mut impl WebSocketConnection<ExampleMessage>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Subscribe to a channel after connecting (using state check)
    send_with_state_check(
        client,
        ExampleMessage::Subscribe {
            channel: "ticker".to_string(),
        },
    )
    .await?;

    println!("Subscribed to ticker channel");

    // Get the event stream
    let mut events = client.event_stream();

    // Process events
    while let Some(event) = events.next().await {
        match event {
            WebSocketEvent::Connected => {
                println!("Event: Connected");
            }
            WebSocketEvent::Disconnected { reason } => {
                println!("Event: Disconnected - {:?}", reason);
                
                // Decide whether to reconnect based on the reason
                match reason {
                    DisconnectReason::UserInitiated => {
                        // User initiated disconnect, don't reconnect
                        return Ok(());
                    }
                    DisconnectReason::RemoteClosed { code, reason } => {
                        // Check if it's a normal closure
                        if code == 1000 {
                            println!("Normal closure: {}", reason);
                            return Ok(());
                        }
                        // Otherwise, we might want to reconnect
                        println!("Remote closed with code {}: {}", code, reason);
                        break;
                    }
                    DisconnectReason::NetworkError { details } => {
                        println!("Network error: {}", details);
                        // Network errors usually mean we should try to reconnect
                        break;
                    }
                    DisconnectReason::ProtocolError { details } => {
                        println!("Protocol error: {}", details);
                        // Protocol errors might be recoverable
                        break;
                    }
                    DisconnectReason::InvalidMessage { details } => {
                        println!("Invalid message: {}", details);
                        // Continue processing, don't disconnect
                    }
                }
            }
            WebSocketEvent::Error { error } => {
                println!("Event: Error - {}", error);
                // Errors don't necessarily mean disconnection
                // Continue processing
            }
            WebSocketEvent::Message { message } => {
                println!("Event: Message - {:?}", message);
                
                // Handle different message types
                match message {
                    ExampleMessage::Data { channel, data } => {
                        println!("Received data on {}: {}", channel, data);
                    }
                    ExampleMessage::Error { message } => {
                        println!("Server error: {}", message);
                    }
                    _ => {}
                }
            }
            WebSocketEvent::PingReceived { data } => {
                println!("Event: Ping received with {} bytes", data.len());
            }
            WebSocketEvent::PongReceived { data } => {
                println!("Event: Pong received with {} bytes", data.len());
            }
        }
    }

    Ok(())
}

/// Example of checking connection state before operations
async fn send_with_state_check(
    client: &mut impl WebSocketConnection<ExampleMessage>,
    message: ExampleMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check connection state before sending
    match client.connection_state() {
        ConnectionState::Connected => {
            client.send(message).await?;
            Ok(())
        }
        ConnectionState::Connecting => {
            println!("Cannot send: connection is still being established");
            Err("Connection not ready".into())
        }
        ConnectionState::Disconnected => {
            println!("Cannot send: not connected");
            Err("Not connected".into())
        }
        ConnectionState::Disconnecting => {
            println!("Cannot send: disconnection in progress");
            Err("Disconnecting".into())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("WebSocket Connection Management Example");
    println!("=======================================");
    println!();
    println!("This example demonstrates:");
    println!("1. User-controlled reconnection with exponential backoff");
    println!("2. Handling different disconnection reasons");
    println!("3. Processing WebSocket events");
    println!("4. Checking connection state before operations");
    println!();

    // Note: This URL is just an example and won't actually connect
    // In a real scenario, you would use a valid WebSocket endpoint
    let url = "wss://echo.websocket.org";
    
    if let Err(e) = maintain_connection(url).await {
        eprintln!("Application error: {}", e);
    }
}