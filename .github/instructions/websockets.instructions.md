---
applyTo: "venues/src/**"
---

# Websocket Implementations

- All websocket implementations MUST implement the common websocket trait at the root.
- Websocket implementations MUST be venue-agnostic in the common crate.
- Venue-specific logic MUST be in the venue's module.
- Websocket connections MUST handle reconnection and error recovery.
- All websocket messages MUST be properly typed with request/response structs.

# WebSocket Endpoint File Structure

- For each WebSocket message type (e.g., hello, subscribe, unsubscribe), the request struct, response struct(s), and the execution function (e.g., send_hello, subscribe, etc.) MUST all be implemented in a single file named to match the message (e.g., hello.rs for the hello message).
- The WebSocket client (e.g., client.rs) MUST only orchestrate and delegate to these per-endpoint files. It MUST NOT contain message construction, serialization, or endpoint-specific logic directly.
- This mirrors the modularity and separation required for REST endpoints.
- All message struct definitions, serialization, deserialization, and execution logic for a WebSocket message MUST be in the corresponding endpoint file.
