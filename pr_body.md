I successfully added the new API error to the Binance CoinM error handling system:

1. Added a new error variant `InvalidMarginParameter(i32)` to the `BinanceCoinMError` enum with proper documentation.
2. Added the error mapping in the `From<BinanceErrorResponse>` implementation to handle error code -5000.

The error will now be properly recognized and handled when encountered during API interactions. The changes maintain the existing pattern of error handling in the codebase.
