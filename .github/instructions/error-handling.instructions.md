---
applyTo: "venues/src/**"
---

# Error Handling

- Each venue MUST define an error enum with code, message, and relevant fields.
- Each venue MUST define an error response struct with all relevant fields.
- Implement From<ErrorResponse> for the error enum, mapping all known codes.
- Each HTTP status code MUST map to a specific error code.
- Each error variant MUST have a doc comment explaining the error.
- Each venue MUST define a Result type alias for its error type.
- All functions MUST return the venue's Result type and use the ? operator.
- Error messages MUST be preserved from the API.
- NEVER use regex for parsing error messages.
- Prefer derive-based error mapping (e.g., using `num_enum` or `strum`) for error enums with many codes, or only match on codes you explicitly care about and use a catch-all for others.
- Use `thiserror` for error enums and automatic `#[from]` conversions where possible.
