---
applyTo: "venues/src/**"
---

# General Coding Standards

- All code must be as high performant and low latency as possible in Rust.
- Prefer complex but fast (and correct) code over cleaner yet slower code.
- Supporting code (e.g., websockets) must be clean, generic, and venue-agnostic.
- **Respect the repository's clippy rules** as defined in `clippy.toml`. All code must pass clippy checks with the project's configured settings.
- All logging and debugging output MUST use a structured logging facade (`log` or `tracing`). DO NOT use `println!` or `eprintln!` for debugging or production code.
- Prefer idiomatic Rust constructs: use iterator adapters (e.g., `filter_map`, `collect`) over manual loops, implement `Display` instead of custom `to_string` methods, and use `#[derive(...)]` for trivial trait implementations (e.g., `Debug`, `Clone`, `Copy`).
- Avoid code duplication: extract helpers for repeated logic (e.g., window trimming, error message extraction, client request logic).
- Use `Cow<'static, str>` for struct fields that may be either static or owned strings.
- Accept `serde::Serialize` for request bodies where possible, rather than raw strings.
