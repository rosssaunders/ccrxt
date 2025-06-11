---
applyTo: "venues/src/**"
---

# Structs and Documentation

- All structs MUST have a doc comment explaining their purpose and usage.
- All struct fields MUST have doc comments with:
  - Purpose, valid values/ranges, constraints, relationships, units/formats.
- Use Rust snake_case for fields; map to API names with serde attributes.
- Request/Response structs MUST have a blank line between each field.
- Field names in serde attributes MUST exactly match the venue's API docs.
- For all struct fields representing headers or similar, implement `Display` for the type rather than custom `to_string` methods.
