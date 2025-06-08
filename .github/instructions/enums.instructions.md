---
applyTo: "venues/src/**"
---

# Enum Usage

- All response structs MUST use enums for fields with fixed sets of values.
- Enums MUST be defined in the venue's enums.rs file.
- Enums MUST implement: Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize.
- Enum variants MUST use the venue's API naming convention.
- DO NOT use String or raw strings for enum fields.
