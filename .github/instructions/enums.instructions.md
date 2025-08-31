---
applyTo: "venues/src/**"
---

# Enum Usage

- All response structs MUST use enums for fields with fixed sets of values.
- Enums MUST be defined in the venue's enums.rs file, EXCEPT for endpoint-specific enums.
- If an enum is specific to a single endpoint (e.g., only used by one request/response struct), the enum definition SHOULD live in the endpoint-specific file alongside the request and response structs.
- Enums MUST implement: Debug, Clone, PartialEq, Eq, Serialize, Deserialize. Derive `Copy` ONLY when the enum is a closed / fully-known set (i.e. there is no catch-all variant with owned data). See Forward Compatibility section below.
- Enum variants MUST use the venue's API naming convention.
- Enum names and variants MUST match idiomatic Rust conventions (PascalCase for types and variants, SCREAMING_SNAKE_CASE for constants, etc.).
- DO NOT use String or raw strings for enum fields.
- The `RestClient` method for each endpoint MUST be implemented in the same file as the request and response structs for that endpoint (e.g., in `order.rs`), not in `client.rs`. This keeps endpoint logic together and consistent with project conventions.

## Forward Compatibility (CRITICAL)

Remote venues can (and do) add new enum values without notice. Deserialization MUST **never** fail just because an unknown variant appears. To guarantee this:

1. For ANY enum whose set of possible values is controlled by a remote API and may expand, include a catch-all variant that stores the original string (name MUST be `Other` unless venue docs dictate `Unknown`).
2. Implement (or derive via an internal macro if introduced later) custom `Serialize` / `Deserialize` so that:
   - Known strings map to their concrete variants.
   - All other strings map to `Other(String)` preserving the raw server value (so we can log / re-emit / round-trip).
3. DO NOT use `#[serde(other)]` because it discards the original string and prevents round-tripping / debugging.
4. Such enums CANNOT implement `Copy` (because `Other` owns a `String`). This is acceptable and REQUIRED for correctness.
5. Prefer `&str` conversion helpers (`impl AsRef<str>` or `fn as_str(&self) -> &str`) instead of exposing internal representation.

### Minimal Pattern

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
	Limit,
	Market,
	Other(String), // catch-all forward-compatible variant
}

impl<'de> serde::Deserialize<'de> for OrderType {
	fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		let s = String::deserialize(de)?;
		Ok(match s.as_str() {
			"limit" => OrderType::Limit,
			"market" => OrderType::Market,
			_ => OrderType::Other(s),
		})
	}
}

impl serde::Serialize for OrderType {
	fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
		ser.serialize_str(match self {
			OrderType::Limit => "limit",
			OrderType::Market => "market",
			OrderType::Other(s) => s.as_str(),
		})
	}
}

impl OrderType {
	pub fn as_str(&self) -> &str {
		match self {
			OrderType::Limit => "limit",
			OrderType::Market => "market",
			OrderType::Other(s) => s.as_str(),
		}
	}
}
```

### When a Closed Enum Is Acceptable

Only omit the `Other(String)` variant (and derive `Copy`) when BOTH conditions hold:

1. The venue's documentation states the set is fixed / versioned OR the field is an internal choice we control (not coming from remote JSON).
2. A new value appearing would represent a protocol-breaking change that we WANT to surface as an explicit error elsewhere.

If unsure, TREAT IT AS OPEN and include the catch-all.

### Testing Requirement

Each forward-compatible enum MUST have a unit test that:

1. Deserializes a known value.
2. Deserializes an unknown value (e.g. `"__new_variant__"`) and asserts it becomes `Other("__new_variant__".into())`.
3. Serializes both the known and unknown forms and round-trips them.

### Prohibited Patterns

- Panicking or erroring on unknown variants.
- Using `#[serde(other)]` when the original string isn't preserved.
- Mapping unknown values to a fixed `Unknown` that loses information.

Non-compliance here creates fragile code that breaks when exchanges deploy new features; treat violations as bugs.
