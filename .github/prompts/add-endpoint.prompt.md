---
mode: 'agent'
tools: ['githubRepo', 'codebase']
description: 'Add a new REST API endpoint for an existing venue (Rust, venues crate)'
---

# Add a New REST API Endpoint for an Existing Venue

Your goal is to add a new REST API endpoint for an existing venue in the `venues` crate, following the conventions and structure of the `account_trades` implementation.

## Instructions

1. **Endpoint Selection**
   - Ask the user for the endpoint to implement (e.g., `/dapi/v1/positionRisk`).
   - Ask for the expected request parameters and response fields, or fetch them from the official API docs if a link is provided.

2. **File and Module Structure**
   - Place the new endpoint in the correct module path, e.g.:
     ```
     venues/src/binance/coinm/private/rest/{endpoint_name}.rs
     ```
   - If the endpoint is public, use `public/rest/` instead of `private/rest/`.

3. **Request Struct**
   - Define a `Request` struct for the endpoint parameters.
   - Use `#[derive(Debug, Clone, Serialize, Default)]`.
   - Add field-level documentation and serde attributes as in `AccountTradeListRequest`.

4. **Response Struct(s)**
   - Define one or more response structs, using `#[derive(Debug, Clone, Deserialize)]`.
   - Document each field, using serde attributes for renaming as needed.

5. **RestClient Implementation**
   - Add a method to `RestClient` for the new endpoint, **in the same file as the request and response structs** (e.g., `order.rs`), following the pattern:
     ```rust
     impl RestClient {
         pub async fn get_{endpoint_name}(
             &self,
             params: {RequestStruct},
         ) -> RestResult<{ResponseType}> {
             let weight = ...; // set according to endpoint docs
             self.send_signed_request(
                 "{endpoint_path}",
                 reqwest::Method::GET, // or POST, etc.
                 params,
                 weight,
                 false, // or true if order endpoint
             )
             .await
         }
     }
     ```
   - Document the method with endpoint details and a link to the official API docs.

6. **Testing and Example Usage**
   - Optionally, add or update an example command in the CLI (e.g., in `venues/examples/binancecoinm/src/commands/`).
   - Provide a sample usage snippet.

## Example (based on account_trades.rs)

```rust
// Request struct
#[derive(Debug, Clone, Serialize, Default)]
pub struct AccountTradeListRequest {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    // ...other fields...

    #[serde(rename = "timestamp")]
    pub timestamp: u64,
}

// Response struct
#[derive(Debug, Clone, Deserialize)]
pub struct AccountTrade {
    #[serde(rename = "symbol")]
    pub symbol: String,
    
    #[serde(rename = "id")]
    pub id: u64,

    // ...other fields...
}

// RestClient method
impl RestClient {
    pub async fn get_account_trades(
        &self,
        params: AccountTradeListRequest,
    ) -> RestResult<Vec<AccountTrade>> {
        let weight = if params.pair.is_some() { 40 } else { 20 };
        self.send_signed_request(
            "/dapi/v1/userTrades",
            reqwest::Method::GET,
            params,
            weight,
            false,
        )
        .await
    }
}
```

## Requirements

- Each endpoint must be implemented in its own file. Do not combine multiple endpoints in a single file.
- Follow Rust idioms and the code style of the `venues` crate.
- Use clear, concise documentation for all structs and methods.
- Use serde attributes for all request/response fields.
- Ensure the endpoint is rate-limited and authenticated as required.
- If unsure, refer to the `account_trades.rs` implementation for structure and style.

---

**Prompt variables:**
- `${endpoint}`: The endpoint path (e.g., `/dapi/v1/positionRisk`)
- `${requestFields}`: List of request parameters
- `${responseFields}`: List of response fields
- `${isPublic}`: Whether the endpoint is public or private

---

**Ready to add a new endpoint! Please provide the endpoint path and details, or a link to the official API documentation.**
