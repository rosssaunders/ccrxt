rustlang wrappers around crypto exchanges

Rules

1. All venues to implement the low latency APIs. If Websocket available, use that over REST calls.
2. All venue rate limiting to be implemented exactly.
3. All wrappers around the endpoints should be pure. Not fixes and helper functions.
4. All websockets to implement the common websocket trait.