---
applyTo: "**"
---

- Unit tests should be in the same file as the code being tested. Unit tests do not require an API key or secret nor rely on the outside world.
- Integration tests should be in the root of the repo under the tests folder. Integration tests must only be in the `tests/` directory, never in examples or source files.
- **Do not use `panic!` in any code, including tests. Use `assert!`, `assert_eq!`, or `assert_matches!` for test failures instead.**
