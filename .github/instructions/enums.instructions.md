---
applyTo: "venues/src/**"
---

# Enum Usage

- All response structs MUST use enums for fields with fixed sets of values.
- Enums MUST be defined in the venue's enums.rs file.
- Enums MUST implement: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
- Enum variants MUST use the venue's API naming convention.
- DO NOT use String or raw strings for enum fields.
- The `RestClient` method for each endpoint MUST be implemented in the same file as the request and response structs for that endpoint (e.g., in `order.rs`), not in `client.rs`. This keeps endpoint logic together and consistent with project conventions.
