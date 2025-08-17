---
applyTo: "venues/src/**"
---

# General Coding Standards

- **NO BACKWARDS COMPATIBILITY REQUIRED**: Until the first release, there is zero need for backwards compatibility. Breaking changes are encouraged to improve the API. This rule will be removed after the first release.
- All code must be as high performant and low latency as possible in Rust.
- Prefer complex but fast (and correct) code over cleaner yet slower code.
- Supporting code (e.g., websockets) must be clean, generic, and venue-agnostic.
- **CRITICAL**: All HTTP wrappers around REST API endpoints MUST NOT pass HTTP verbs as parameters to avoid branch prediction penalties. Use verb-specific functions (send_get_request, send_post_request, etc.) instead of generic functions that take HTTP method as parameter. See `http-performance.instructions.md` for detailed requirements.
- **Respect the repository's clippy rules** as defined in `clippy.toml`. All code must pass clippy checks with the project's configured settings.
- Doc comment links:
	- For external URLs in Rust doc comments, always use inline Markdown links `[label](url)`.
	- Do NOT use reference-style link definitions like `[label]: url` inside doc comments (especially inside list items); rustdoc won’t render them there and Clippy will warn with `doc_nested_refdefs`.
	- Example (preferred):
    
		/// - [docs](https://bingx-api.github.io/docs/#/en-us/spot/trade-api.html#Cancel%20all%20Open%20Orders%20on%20a%20Symbol)
    
	- Use rustdoc intra-doc links for items within the crate (e.g., `[Type]`, `[module::Type]`); reserve the inline form above for external links.
- All logging and debugging output MUST use a structured logging facade (`log` or `tracing`). DO NOT use `println!` or `eprintln!` for debugging or production code.
- **Import and Namespace Usage**: All types and functions MUST be imported at the top of the file and used by their short names throughout the code. DO NOT use fully qualified paths (e.g., `crate::module::Type`) when the type is already imported. Instead of `crate::binance::coinm::ResponseHeaders::default()`, import `ResponseHeaders` and use `ResponseHeaders::default()`.
- Prefer idiomatic Rust constructs: use iterator adapters (e.g., `filter_map`, `collect`) over manual loops, implement `Display` instead of custom `to_string` methods, and use `#[derive(...)]` for trivial trait implementations (e.g., `Debug`, `Clone`, `Copy`).
- Use `#[derive(Default)]` instead of manual `Default` implementations when all fields can use their default values (especially for structs with all `Option<T>` fields).
- Avoid code duplication: extract helpers for repeated logic (e.g., window trimming, error message extraction, client request logic).
- Use `Cow<'static, str>` for struct fields that may be either static or owned strings.
- Accept `serde::Serialize` for request bodies where possible, rather than raw strings.

- DTO struct formatting (request/response types):
	- Insert exactly one blank line between each field in DTO structs to maximize diff readability and reduce merge conflicts.
	- A “field” means the complete block of its doc comment(s), attributes (e.g., serde), and the field declaration line; the blank line goes between these blocks, not inside them.
	- Applies to all structs that represent wire types (typically those with `Serialize`/`Deserialize` derives) across venues.
	- Do not add extra blank lines above the first field or below the last field.
