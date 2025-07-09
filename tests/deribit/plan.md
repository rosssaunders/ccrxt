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
   - ✅ Created comprehensive integration test file: `tests/deribit_public_integration_tests.rs`
   - ✅ Tests 15 different public endpoints with over 23 individual test functions
   - ✅ Includes error handling, rate limiting, and multi-endpoint integration tests

### ✅ IMPLEMENTED AND TESTED ENDPOINTS
- ✅ get_combo_ids (✅ tests for BTC and ETH currencies)
- ✅ get_combo_details (✅ test with dynamic combo ID retrieval)
- ✅ get_combos (✅ test for BTC with legs validation)
- ✅ get_contract_size (✅ test for BTC-PERPETUAL)
- ✅ get_currencies (✅ comprehensive test with BTC/ETH validation)
- ✅ get_funding_rate_value (✅ test for BTC-PERPETUAL)
- ✅ get_index_price (✅ test for btc_usd index)
- ✅ get_instruments (✅ tests for BTC and BTC futures specifically)
- ✅ get_last_settlements_by_currency (✅ test for BTC futures)
- ✅ get_last_trades_by_currency (✅ test for BTC futures)
- ✅ get_supported_index_names (✅ comprehensive test)
- ✅ get_book_summary_by_currency (✅ test for BTC futures)
- ✅ get_book_summary_by_instrument (✅ test for BTC-PERPETUAL)
- ✅ get_status (✅ platform status test)
- ✅ get_time (✅ server time test)

### 📋 Endpoints Still To Implement (Future Work)
- [ ] get_apr_history (venues/src/deribit/public/rest/get_apr_history.rs)
- [ ] get_book_summary_by_currency (venues/src/deribit/public/rest/get_book_summary_by_currency.rs)
- [ ] get_book_summary_by_instrument (venues/src/deribit/public/rest/get_book_summary_by_instrument.rs)
- [ ] get_combo_details (venues/src/deribit/public/rest/get_combo_details.rs)
- [ ] get_combo_ids (venues/src/deribit/public/rest/get_combo_ids.rs)
- [ ] get_combos (venues/src/deribit/public/rest/get_combos.rs)
- [ ] get_contract_size (venues/src/deribit/public/rest/get_contract_size.rs)
- [ ] get_currencies (venues/src/deribit/public/rest/get_currencies.rs)
- [ ] get_delivery_prices (venues/src/deribit/public/rest/get_delivery_prices.rs)
- [ ] get_expirations (venues/src/deribit/public/rest/get_expirations.rs)
- [ ] get_funding_chart_data (venues/src/deribit/public/rest/get_funding_chart_data.rs)
- [ ] get_funding_rate_history (venues/src/deribit/public/rest/get_funding_rate_history.rs)
- [ ] get_funding_rate_value (venues/src/deribit/public/rest/get_funding_rate_value.rs)
- [ ] get_historical_volatility (venues/src/deribit/public/rest/get_historical_volatility.rs)
- [ ] get_index_price_names (venues/src/deribit/public/rest/get_index_price_names.rs)
- [ ] get_index_price (venues/src/deribit/public/rest/get_index_price.rs)
- [ ] get_index (venues/src/deribit/public/rest/get_index.rs)
- [ ] get_instrument (venues/src/deribit/public/rest/get_instrument.rs)
- [ ] get_instruments (venues/src/deribit/public/rest/get_instruments.rs)
- [ ] get_last_settlements_by_currency (venues/src/deribit/public/rest/get_last_settlements_by_currency.rs)
- [ ] get_last_trades_by_instrument_and_time (venues/src/deribit/public/rest/get_last_trades_by_instrument_and_time.rs)
- [ ] get_mark_price_history (venues/src/deribit/public/rest/get_mark_price_history.rs)
- [ ] get_last_trades_by_currency_and_time (venues/src/deribit/public/rest/get_last_trades_by_currency_and_time.rs)
- [ ] get_volatility_index_data (venues/src/deribit/public/rest/get_volatility_index_data.rs)
- [ ] get_supported_index_names (venues/src/deribit/public/rest/get_supported_index_names.rs)


2. **Integration Test File Location**  
   - Create a new integration test file:
     - Path: `tests/deribit/public_endpoints_integration.rs`

3. **Test Structure and Execution**  
   - Initialize a `RestClient` without credentials.
   - For each public endpoint:
     - Construct a valid API request.
     - Call the endpoint.
     - Validate the response structure and data (using assertions such as `assert_eq!` and `assert!`).
   - Make sure no credentials are required and no API state is modified.

4. **Logging and Reporting**  
   - Use the configured structured logging (e.g., `log` or `tracing`) to output debug and success messages.
   - Ensure each test logs which endpoint is being tested and the outcome.

5. **GitHub Actions Setup**  
   - Configure a GitHub Action workflow to run these tests nightly.
   - Use a command like `cargo test --test public_endpoints_integration` in the workflow step.

6. **Error Handling**  
   - Use proper assertions (`assert!`, `assert_eq!`) to catch unexpected results.
   - Do not use `panic!` for error handling.
   - Handle any API-specific errors gracefully and log them appropriately.

7. **Documentation and Maintenance**  
   - Update this document as new endpoints are implemented or existing ones are modified.
   - Ensure that any changes in API behavior are also reflected in the test assertions.

---

## Future Steps for Private Endpoints Integration Tests

- **Endpoint Identification**: List all private endpoints and required testnet usage.
- **File Location**: Create a file at `tests/deribit/private_endpoints_integration.rs`.
- **Credential Handling**: Use testnet credentials handled securely via `SecretString`.
- **Test Execution**: Similar structure to public tests, with added authentication steps.
- **GitHub Actions**: Adjust workflow to include environment variables or GitHub Secrets for private tests.

---

## Notes

- Integration tests reside strictly in the `tests/` directory as per project conventions.
- Unit tests should remain co-located with source files, and integration tests must not.
- Follow the repository’s guidelines on error handling, logging, and rate limiting.
- Update the venue README with details on API documentation, authentication