---
applyTo: "**"
---

# Example Code Instructions

- All example code for a venue must be placed in this directory: `venues/examples/<venue>/`.
- Example files should be named according to the endpoint or feature they demonstrate (e.g., `ws_hello_example.rs`, `rest_get_combo_ids_example.rs`).
- Each example file should:
  - Include a top-level doc comment describing what the example demonstrates.
  - Be self-contained and runnable (with clear instructions if credentials or setup are required).
  - Use only public APIs from the venue crate.
  - Include comments explaining each major step.
- Unit tests for example code may be included in the same file, but should not require network access or credentials.
- Integration tests must NOT be placed here; they belong in the `tests/` directory at the repo root.
- If the example demonstrates a public endpoint, it should not require credentials. If credentials are required, document how to provide them securely (never hard-code secrets).
- Follow the project's general coding and documentation standards.
