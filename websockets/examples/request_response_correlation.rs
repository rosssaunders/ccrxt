//! Example: Request/Response correlation with RequestManager (JSON-RPC style)
//!
//! What it demonstrates:
//! - Generating unique IDs with RequestIdGenerator
//! - Registering a pending request and awaiting its response via oneshot
//! - Parsing a text message to extract an `id` and fulfilling the pending request
//! - When to use this: venues that use request/response over WS (e.g., JSON-RPC 2.0)
//!
//! This example uses a simulated flow to avoid network:
//! - We create an ID and register a waiter
//! - We "receive" a JSON response that includes the ID
//! - We fulfill the pending request based on the parsed ID
//! - Finally, we await the registered receiver to get the payload
//!
//! Run:
//!   cargo run --example request_response_correlation

use serde::{Deserialize, Serialize};
use serde_json as json;
use websockets::{RequestIdGenerator, RequestManager};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    id: u64,
    result: T,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ids = RequestIdGenerator::new();
    let mgr = RequestManager::new();

    // Create a request ID and register to await the response
    let id = ids.next_id();
    let rx = mgr.register(id);

    // Simulate a response coming back from the wire
    let simulated = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: "ok".to_string(),
    };
    let wire = json::to_string(&simulated)?;

    // Parse the wire message, extract ID, and fulfill the pending waiter
    let parsed: JsonRpcResponse<String> = json::from_str(&wire)?;
    let payload = wire.into_bytes();
    let fulfilled = mgr.fulfill(parsed.id, payload);
    assert!(fulfilled, "expected pending waiter to exist");

    // Await the response via the oneshot receiver
    let body = futures::executor::block_on(rx)?;
    println!(
        "received {} bytes: {}",
        body.len(),
        truncate(std::str::from_utf8(&body).unwrap_or("<invalid>"), 120)
    );

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s.to_owned()
    }
}
