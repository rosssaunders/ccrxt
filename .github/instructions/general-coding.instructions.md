---
applyTo: "venues/src/**"
---

# General Coding Standards

- All code must be as high performant and low latency as possible in Rust.
- Prefer complex but fast (and correct) code over cleaner yet slower code.
- Supporting code (e.g., websockets) must be clean, generic, and venue-agnostic.
- **Respect the repository's clippy rules** as defined in `clippy.toml`. All code must pass clippy checks with the project's configured settings.
- All logging and debugging output MUST use a structured logging facade (`log` or `tracing`). DO NOT use `println!` or `eprintln!` for debugging or production code.
- **Import and Namespace Usage**: All types and functions MUST be imported at the top of the file and used by their short names throughout the code. DO NOT use fully qualified paths (e.g., `crate::module::Type`) when the type is already imported. Instead of `crate::binance::coinm::ResponseHeaders::default()`, import `ResponseHeaders` and use `ResponseHeaders::default()`.
- Prefer idiomatic Rust constructs: use iterator adapters (e.g., `filter_map`, `collect`) over manual loops, implement `Display` instead of custom `to_string` methods, and use `#[derive(...)]` for trivial trait implementations (e.g., `Debug`, `Clone`, `Copy`).
- Use `#[derive(Default)]` instead of manual `Default` implementations when all fields can use their default values (especially for structs with all `Option<T>` fields).
- Avoid code duplication: extract helpers for repeated logic (e.g., window trimming, error message extraction, client request logic).
- Use `Cow<'static, str>` for struct fields that may be either static or owned strings.
- Accept `serde::Serialize` for request bodies where possible, rather than raw strings.

## Documentation Standards

**All endpoint wrapper functions MUST follow the standardized documentation format**:

```rust
/// [Brief one-line description starting with action verb]
///
/// [Detailed description explaining purpose, when to use, important context or limitations]
///
/// **API Reference**: [Direct link to official exchange documentation]
/// **Endpoint**: `[HTTP_METHOD] /path/to/endpoint`
/// **Rate Limit**: [Limit info with weight if applicable]
/// **Authentication**: [public | private]
/// **Permissions**: [Required API key permissions]
/// **Scope**: [Required scopes for venues that use them]
///
/// # Arguments
/// * `param_name` - [Description with type, constraints]
/// * `optional_param` - [Description] (Optional: default behavior)
///
/// # Returns
/// [Clear description of return value structure and contents]
///
/// # Errors
/// * `ErrorType` - [When this error occurs]
/// * `AnotherError` - [Condition for this error]
```

### Documentation Requirements

1. **Brief Description**: One line, start with action verb (Get, Create, Cancel, Update, etc.)
2. **Detailed Description**: Explain business purpose and when to use, mention limitations
3. **API Reference**: Always include direct link to official exchange documentation
4. **Endpoint**: Show HTTP method and exact path with path parameters in curly braces
5. **Rate Limit**: Specify requests per time unit, include weight if applicable
6. **Authentication/Permissions**: Clearly state public/private and required permissions
7. **Arguments**: Document every parameter with type and constraints, mark optional parameters
8. **Returns**: Describe structure and meaning of returned data
9. **Errors**: Document common error conditions and when they occur

**This documentation standard is MANDATORY for all endpoint wrapper functions and will be used by the UI to provide rich endpoint information, search functionality, and links to official documentation.**
