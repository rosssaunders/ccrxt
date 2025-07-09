# Integration Test Plan: Deribit Venue

## Overview

This document outlines the plan to create two integration test suites for the Deribit venue:

- **Public Endpoints Integration Tests**: Tests all available public API endpoints.
- **Private Endpoints Integration Tests**: Tests endpoints requiring authentication using the testnet.

These tests will run nightly in a GitHub Action workflow.

---

## Plan for Public Endpoints

### ✅ COMPLETED

1. **Integration Test File Created**

   - ✅ Created comprehensive integration test file: `tests/deribit/public_integration_tests.rs`
   - ✅ Tests 45 individual test functions covering all available public endpoints
   - ✅ Includes error handling, rate limiting, concurrent requests, and multi-endpoint integration tests
   - ✅ All tests use only top-level exports and follow project standards

### ✅ IMPLEMENTED AND TESTED ENDPOINTS (All Available & Exported)

- ✅ get_combo_ids (✅ tests for BTC, ETH, and various currencies)
- ✅ get_combo_details (✅ test with dynamic combo ID retrieval)
- ✅ get_combos (✅ test for BTC with legs validation)
- ✅ get_contract_size (✅ tests for multiple instruments including BTC-PERPETUAL)
- ✅ get_currencies (✅ comprehensive test with detailed BTC/ETH validation)
- ✅ get_funding_rate_value (✅ tests for multiple instruments)
- ✅ get_index_price (✅ tests for btc_usd and other indices)
- ✅ get_instruments (✅ comprehensive tests for BTC/ETH, futures, options, perpetuals)
- ✅ get_last_settlements_by_currency (✅ tests for BTC and ETH)
- ✅ get_last_trades_by_currency (✅ tests for BTC and ETH with various parameters)
- ✅ get_supported_index_names (✅ comprehensive test with validation)
- ✅ get_book_summary_by_currency (✅ tests for BTC and ETH futures)
- ✅ get_book_summary_by_instrument (✅ tests for multiple instruments)
- ✅ get_status (✅ comprehensive platform status test)
- ✅ get_time (✅ server time test with validation)

### ✅ COMPREHENSIVE TEST COVERAGE INCLUDES

- ✅ Multi-currency tests (BTC, ETH)
- ✅ Multi-instrument tests (futures, options, perpetuals)
- ✅ Error handling and edge cases (invalid parameters, empty responses)
- ✅ Rate limiting tests (burst requests, tier validation)
- ✅ Concurrent request handling
- ✅ Data structure validation
- ✅ Response consistency checks
- ✅ Timestamp and field validation
- ✅ Large parameter testing (counts, limits)

### 📋 Endpoints Available in Codebase But Not Exported (Future Work)

The following endpoints exist in the codebase but are not currently exported at the top level:

- [ ] get_apr_history
- [ ] get_delivery_prices
- [ ] get_expirations
- [ ] get_funding_chart_data
- [ ] get_funding_rate_history
- [ ] get_historical_volatility
- [ ] get_index
- [ ] get_index_price_names
- [ ] get_instrument
- [ ] get_last_settlements_by_instrument
- [ ] get_last_trades_by_instrument
- [ ] get_last_trades_by_currency_and_time
- [ ] get_last_trades_by_instrument_and_time
- [ ] get_mark_price_history
- [ ] get_order_book
- [ ] get_order_book_by_instrument_id
- [ ] get_rfqs
- [ ] get_trade_volumes
- [ ] get_tradingview_chart_data
- [ ] get_volatility_index_data

*Note: These endpoints would need to be exported in the module hierarchy before tests can be written.*

### 📋 Additional Development Tasks (If Needed)

1. **Integration Test File Location**

   - Create a new integration test file:
     - Path: `tests/deribit/public_endpoints_integration.rs`

2. **Test Structure and Execution**

   - Initialize a `RestClient` without credentials.
   - For each public endpoint:
     - Construct a valid API request.
     - Call the endpoint.
     - Validate the response structure and data (using assertions such as `assert_eq!` and `assert!`).
   - Make sure no credentials are required and no API state is modified.

3. **Logging and Reporting**

   - Use the configured structured logging (e.g., `log` or `tracing`) to output debug and success messages.
   - Ensure each test logs which endpoint is being tested and the outcome.

4. **GitHub Actions Setup**

   - Configure a GitHub Action workflow to run these tests nightly.
   - Use a command like `cargo test --test public_endpoints_integration` in the workflow step.

5. **Error Handling**

   - Use proper assertions (`assert!`, `assert_eq!`) to catch unexpected results.
   - Do not use `panic!` for error handling.
   - Handle any API-specific errors gracefully and log them appropriately.

6. **Documentation and Maintenance**

   - Update this document as new endpoints are implemented or existing ones are modified.
   - Ensure that any changes in API behavior are also reflected in the test assertions.

---

## Future Steps for Private Endpoints Integration Tests

- **Endpoint Identification**: List all private endpoints and required testnet usage.
- **File Location**: Create a file at `tests/deribit/private_endpoints_integration.rs`.
- **Credential Handling**: Use testnet credentials handled securely via `SecretString`.
- **Test Execution**: Similar structure to public tests, with added authentication steps.
- **GitHub Actions**: Adjust workflow to include environment variables or GitHub Secrets for private tests.
- **Documentation**: Document the testnet setup and required environment variables.
- **Test Coverage**: Create comprehensive tests for all private endpoints.
- **Error Handling**: Test error scenarios for authentication failures and insufficient permissions.
- **Rate Limiting**: Test rate limiting for private endpoint usage.

---

## Notes

- Integration tests reside strictly in the `tests/` directory as per project conventions.
- Unit tests should remain co-located with source files, and integration tests must not.
- Follow the repository's guidelines on error handling, logging, and rate limiting.
- Update the venue README with details on API documentation, authentication, and testing.