# websockets/examples

Self-contained examples demonstrating the transport layer and correlation utilities.

## 1) native_connect_and_receive.rs

Purpose:

- Show how to connect with the native client, track state, and receive frames.
- Distinguish binary (Bytes) and text messages at the transport layer.

Use when:

- You want a minimal transport-only sample to verify connectivity and event handling.
- You’re connecting to a public WS endpoint that doesn’t require auth.

Run:

```bash
cargo run --example native_connect_and_receive -- --url wss://ws.kraken.com
```

Notes:

- Replace the URL with your venue’s public endpoint.
- This example sends a small binary payload; some servers ignore it.

## 2) request_response_correlation.rs

Purpose:

- Show how to correlate requests and responses using `RequestIdGenerator` and `RequestManager`.
- Demonstrates a JSON-RPC-style flow without network calls.

Use when:

- Your venue uses request/response over WS (e.g., JSON-RPC 2.0 with `id`).
- You need to await specific responses matching your request ID.

Run:

```bash
cargo run --example request_response_correlation
```

Notes:

- This is a self-contained simulation to keep it deterministic and fast.
- In a real client, parse the incoming message, extract the `id`, and call `fulfill(id, payload)`.
