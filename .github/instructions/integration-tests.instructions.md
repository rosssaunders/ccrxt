---
applyTo: "tests/**"
---

# Integration Test Standards

## Import Requirements

- Integration tests MUST use only top-level exports from venue modules.
- DO NOT import from nested modules like `venues::<venue>::public::rest::*` or `venues::<venue>::private::rest::*`.
- All required types must be re-exported at the top level of the venue module (e.g., `venues::<venue>::*`).
- If a type is not available at the top level, it MUST be added to the venue's `mod.rs` re-exports before writing the integration test.

### Examples

**✅ CORRECT:**

```rust
use venues::deribit::{
    AccountTier, Currency, GetComboDetailsRequest, GetComboIdsRequest, GetCombosRequest,
    PublicRestClient, RateLimiter,
};
```

**❌ INCORRECT:**

```rust
use venues::deribit::{AccountTier, Currency, PublicRestClient, RateLimiter};
use venues::deribit::public::rest::{GetComboDetailsRequest, GetCombosRequest};
```

## Test Structure

- Each integration test file MUST test only one venue or logical grouping of functionality.
- Tests MUST be organized by endpoint or feature area.
- Each test MUST have a clear, descriptive name that indicates what it's testing.
- Use helper functions to reduce code duplication (e.g., `create_test_client()`).

## Single Endpoint Focus

- **Each test should test only ONE endpoint**. DO NOT write comprehensive tests that call multiple endpoints unless they are logically related and necessary for the test and you need to use data from a previous call to feed into the next.
- Tests should focus on verifying the endpoint can be called successfully and returns expected response structure.
- Use `println!` to display results for manual verification rather than complex assertions.
- **DO NOT write "comprehensive" or "workflow" tests** that chain multiple API calls together - these create unnecessary maintenance overhead.

### Assertion Guidelines

- **Avoid assertions on dynamic data** (prices, quantities, timestamps, etc.) that may change between test runs.
- **Only assert on structural elements** you can be certain will always be present:
  - Response structure (e.g., `response.jsonrpc == "2.0"`)
  - Required fields exist (e.g., `!response.result.is_empty()` for arrays)
  - Basic data types and formats
- **DO NOT assert on**:
  - Specific market data values
  - Array lengths (unless guaranteed to be non-empty)
  - Timestamp values
  - Price or quantity values

### Examples

**✅ CORRECT:**

```rust
#[tokio::test]
async fn test_get_combo_ids() {
    let client = create_test_client();
    let request = GetComboIdsRequest { currency: Currency::BTC, state: None };

    let result = client.get_combo_ids(request).await;
    assert!(result.is_ok(), "get_combo_ids should succeed: {:?}", result.err());

    let response = result.unwrap();
    assert_eq!(response.jsonrpc, "2.0");
    println!("Found {} combo IDs", response.result.len());
}
```

**❌ INCORRECT:**

```rust
#[tokio::test]
async fn test_comprehensive_workflow() {
    let client = create_test_client();

    // DON'T DO THIS - testing multiple endpoints in one test
    let combo_ids = client.get_combo_ids(request1).await.unwrap();
    let combo_details = client.get_combo_details(request2).await.unwrap();
    let currencies = client.get_currencies().await.unwrap();

    // DON'T DO THIS - asserting on dynamic data
    assert_eq!(combo_ids.result.len(), 5);
    assert_eq!(combo_details.result.mark_price, 50000.0);
}
```

## Error Handling

- All API calls in integration tests MUST use proper error handling with descriptive assertions.
- Use `assert!(result.is_ok(), "description: {:?}", result.err())` for API calls that should succeed.
- Include meaningful error messages that help debug test failures.

## Documentation

- Each integration test file MUST include a top-level doc comment explaining what the file tests.
- Complex test scenarios MUST include comments explaining the test logic.
- Tests that require specific market conditions or data availability MUST document these requirements.
