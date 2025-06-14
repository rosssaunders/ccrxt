---
applyTo: "venues/src/**"
---

# Websocket Implementations

- All websocket implementations MUST implement the common websocket trait at the root.
- Websocket implementations MUST be venue-agnostic in the common crate.
- Venue-specific logic MUST be in the venue's module.
- Websocket connections MUST handle reconnection and error recovery.
- All websocket messages MUST be properly typed with request/response structs.
