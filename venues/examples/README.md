# Coinbase Advanced Trade WebSocket Examples

This directory contains examples demonstrating how to use the Coinbase Advanced Trade WebSocket API.

## Authenticated WebSocket Example

The `authenticated_websocket.rs` example shows how to use the authenticated WebSocket connection to receive user-specific data like order updates and futures balance information.

### Setting Up the JWT Token

To use the authenticated WebSocket, you need to provide a valid JWT token. Here's how to set it up:

1. First, create a `.env` file in the `venues` directory:

```bash
touch venues/.env
```

2. Add your JWT token to the `.env` file:

```env
COINBASE_JWT_TOKEN=your_jwt_token_here
```

3. Update the example code to use the environment variable. In `authenticated_websocket.rs`, replace the hardcoded JWT with:

```rust
use dotenv::dotenv;
use std::env;

// ... rest of the imports ...

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Load environment variables
    dotenv().ok();
    
    // Get JWT token from environment
    let jwt = env::var("COINBASE_JWT_TOKEN")
        .expect("COINBASE_JWT_TOKEN must be set in .env file");
    
    // Run examples
    basic_auth_example(jwt.clone()).await?;
    futures_balance_example(jwt.clone()).await?;
    complete_workflow_example(jwt.clone()).await?;
    error_handling_example(jwt).await?;
    
    Ok(())
}
```

### Getting a JWT Token

To get a JWT token for Coinbase Advanced Trade:

1. Log in to your Coinbase Advanced Trade account
2. Go to API Management
3. Create a new API key with the following permissions:
   - `view` - to view account information
   - `trade` - to view order information
4. Copy the generated JWT token

### Running the Example

1. Make sure you have the `dotenv` dependency in your `Cargo.toml`:

```toml
[dependencies]
dotenv = "0.15"
```

2. Run the example:

```bash
cargo run --example authenticated_websocket
```

### Security Notes

- Never commit your `.env` file to version control
- Add `.env` to your `.gitignore` file
- Keep your JWT token secure and rotate it regularly
- Use the minimum required permissions for your API key

### Example Output

The example will demonstrate:
1. Basic user channel subscription
2. Futures balance summary subscription
3. Complete workflow with multiple subscriptions
4. Error handling and reconnection logic

Each example will print received messages to the console. 