---
applyTo: "venues/src/**"
---

# Adding a New REST API Endpoint for an Existing Venue (Rust, venues crate)

## Overview

This guide describes how to add a new REST API endpoint for an existing venue in the `venues` crate, following the conventions and structure of the `account_trades` implementation. 
It also details documentation and code style requirements for all structs and fields.

---

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

---

## 4. Response Struct(s)

- Define one or more response structs, using `#[derive(Debug, Clone, Deserialize)]`.
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

## 5. RestClient Implementation

- Add a method to `RestClient` for the new endpoint, **in the same file as the request and response structs**.
- Follow this pattern:
  ```rust
  impl RestClient {
      /// Calls the account trades endpoint.
      ///
      /// [Official API docs](https://api.binance.com/docs)
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
- Document the method with endpoint details and a link to the official API docs.
- Ensure the endpoint is rate-limited and authenticated as required.

---

## 6. Update `mod.rs` File

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

## 7. Testing and Example Usage

- Optionally, add or update an example command in the CLI (e.g., in `venues/examples/binancecoinm/src/commands/`).
- Provide a sample usage snippet.

---

## 8. Additional Requirements

- For all struct fields representing headers or similar, implement `Display` for the type rather than custom `to_string` methods.
- Follow Rust idioms and the code style of the `venues` crate.
- Use clear, concise documentation for all structs and methods.
- If unsure, refer to the `account_trades.rs` implementation for structure and style.

---
