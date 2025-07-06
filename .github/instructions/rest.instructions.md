---
applyTo: "venues/src/**/rest/**"
---

# Adding a New REST API Endpoint for an Existing Venue (Rust, venues crate)

## Overview

This guide describes how to add a new REST API endpoint for an existing venue in the `venues` crate, following the conventions and structure of the `account_trades` implementation.
It also details documentation and code style requirements for all structs and fields.

## 1. Endpoint Selection

- Obtain the endpoint path (e.g., `/dapi/v1/positionRisk`).
- Gather the expected request parameters and response fields, or fetch them from the official API docs if a link is provided.

---

## 2. File and Module Structure

- Place the new endpoint in the correct module path, e.g.:
  - `venues/src/binance/coinm/private/rest/{endpoint_name}.rs`
  - If the endpoint is public, use `public/rest/` instead of `private/rest/`.
- Each endpoint MUST be implemented in its own file. Do NOT combine multiple endpoints in a single file.

---

## 3. Request Struct

- Define a `Request` struct for the endpoint parameters.
- Use `#[derive(Debug, Clone, Serialize, Default)]`.
- **Documentation:**
  - All structs MUST have a doc comment explaining their purpose and usage.
  - All struct fields MUST have doc comments with:
    - Purpose, valid values/ranges, constraints, relationships, units/formats.
  - Use Rust snake_case for fields; map to API names with serde attributes.
  - Field names in serde attributes MUST exactly match the venue's API docs.
  - There MUST be a blank line between each field.
- Example:

  ```rust
  /// Request parameters for the account trade list endpoint.
  #[derive(Debug, Clone, Serialize, Default)]
  pub struct AccountTradeListRequest {
      /// Trading symbol (e.g., "BTCUSD_PERP"). Optional.
      #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
      pub symbol: Option<String>;

      // ...other fields...

      /// Request timestamp (milliseconds since epoch).
      #[serde(rename = "timestamp")]
      pub timestamp: u64;
  }
  ```

# Request Struct Simplicity

- Do NOT add builder methods or `impl` blocks for request structs unless explicitly required by the API or project maintainers.
- Use only `#[derive(Default)]` for structs with all-optional fields; construct with `StructName::default()` for an empty instance.
- Do NOT add `new()` constructors for simple request structs—use the default derive.
- Do NOT add `with_*` builder methods unless the struct is complex and builder pattern is explicitly requested.
- **DO NOT add calculation/utility methods to request or response structs** (e.g., `average_trade_size()`, `calculate_ratio()`, `is_valid()`). These structs should be simple data containers only.
- **DO NOT add helper methods, validation methods, or any business logic to structs**. Keep structs as pure data transfer objects.

---

## 4. Response Struct(s)

- Define one or more response structs, using `#[derive(Debug, Clone, Deserialize)]`.
- **DO NOT add any `impl` blocks to response structs.** No calculation methods, utility methods, or business logic should be added.
- **Response structs should be pure data containers** that only hold deserialized API response data.
- **Documentation:**
  - All structs and fields must be documented as above.
  - Use serde attributes for all fields.
- Example:

  ```rust
  /// Represents a single account trade.
  #[derive(Debug, Clone, Deserialize)]
  pub struct AccountTrade {
      /// Trading symbol.
      #[serde(rename = "symbol")]
      pub symbol: String;

      /// Trade ID.
      #[serde(rename = "id")]
      pub id: u64;

      // ...other fields...
  }
  ```

---

## 5. Endpoint Constants

- **All endpoint URL paths MUST be defined as constants** to allow reuse across endpoint wrapper functions, rate limiting, and other code.
- Define the constant at the top of the file (after imports, before structs).
- Use `SCREAMING_SNAKE_CASE` naming convention.
- Example:

  ```rust
  const ACCOUNT_TRADES_ENDPOINT: &str = "/dapi/v1/userTrades";
  ```

---

## 6. RestClient Implementation

- Add a method to `RestClient` for the new endpoint, **in the same file as the request and response structs**.
- **Use the endpoint constant defined in step 5** instead of hardcoding the URL path.
- **All endpoint wrapper functions MUST include a doc comment above the function, following this standard:**
  - Brief summary of the endpoint’s purpose.
  - Details of what the function does.
  - Link to the official API documentation.
  - Rate limit information.
  - Arguments (with a brief description for each).
  - Return value (with a brief description).
- Example:
  ```rust
  /// Cancel all orders (v4)
  ///
  /// Cancels all outstanding orders for a symbol and/or side.
  ///
  /// See: https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/bitmart/spot/spot___margin_trading.md
  ///
  /// Rate limit: varies by endpoint type
  ///
  /// # Arguments
  /// * `request` - The cancel all request parameters
  ///
  /// # Returns
  /// Empty response - success indicated by HTTP status
  pub async fn cancel_all_orders(
      &self,
      request: CancelAllOrdersRequest,
  ) -> RestResult<CancelAllOrdersResponse> {
      shared::send_signed_request(
          self,
          CANCEL_ALL_ORDERS_ENDPOINT,  // Use the constant here
          reqwest::Method::POST,
          request,
          10,
          false,
      )
      .await
  }
  ```
- Ensure the endpoint is rate-limited and authenticated as required.
- Do NOT add "helper" functions for venue REST endpoints. Endpoint functions must match the venue API exactly, without additional abstraction or helpers.
- Endpoint functions must take a struct for parameters, except for parameters that appear in the URL path, which may be individual arguments.
- Do NOT include example code snippets, usage examples, or sample invocations above or within endpoint wrapper functions. All example code must be placed in the appropriate `venues/examples/<venue>/` directory as per the example code instructions.\*\*

---

## **Parameter Struct Rule (MANDATORY)**

- **All endpoint functions MUST take a single struct for parameters.**
- **Do NOT use individual function arguments for endpoint parameters (except for URL path parameters).**
- The ONLY exception is for parameters that are part of the URL path (not query/body), which may be passed as individual arguments.
- This rule is mandatory for all new and existing endpoints.

### Common Mistakes to Avoid

- ❌ `pub async fn foo(&self, a: String, b: u64)`
- ✅ `pub async fn foo(&self, params: FooRequest)`
- Do not split parameters into multiple arguments unless they are part of the URL path.

### Parameter Struct Checklist

| Rule                          | Allowed | Not Allowed |
| ----------------------------- | ------- | ----------- |
| Endpoint params as struct     | ✅      | ❌          |
| Multiple params (not in path) | ❌      | ✅          |

### Example (Correct)

```rust
pub async fn submit_transfer_to_user(
    &self,
    params: SubmitTransferToUserRequest,
) -> RestResult<SubmitTransferToUserResponse> {
    // ...
}
```

### Example (Incorrect)

```rust
pub async fn submit_transfer_to_user(
    &self,
    currency: String,
    amount: f64,
    destination: String,
) -> RestResult<SubmitTransferToUserResponse> {
    // ...
}
```

---

## 7. Update `mod.rs` File

- After creating a new endpoint file, add a corresponding `mod` declaration to the appropriate `mod.rs` file (e.g., `venues/src/binance/coinm/private/rest/mod.rs`).
- **Each endpoint import (`mod`) and each `pub use` MUST be on its own line.**  
  This reduces the risk of merge conflicts when multiple endpoints are added concurrently.
- Example:

  ```rust
  pub mod account_trades;
  pub mod position_risk;
  pub mod new_endpoint; // Add your endpoint here, on its own line

  // If you need to re-export specific items from a module, each `pub use` must also be on its own line:
  pub use self::rest::{GetHistoryIndexCandlesRequest, GetHistoryIndexCandlesResponse, IndexCandle};
  ```

## 8. Testing and Example Usage

- Optionally, add or update an example command in the CLI (e.g., in `venues/examples/binancecoinm/src/commands/`).
- Provide a sample usage snippet.

---

## 9. Additional Requirements

- For all struct fields representing headers or similar, implement `Display` for the type rather than custom `to_string` methods.
- Follow Rust idioms and the code style of the `venues` crate.
- Use clear, concise documentation for all structs and methods.
- If unsure, refer to the `account_trades.rs` implementation for structure and style.

---
