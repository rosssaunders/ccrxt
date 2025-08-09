---
applyTo: "venues/src/**"
---

# API Credential Handling

- All API keys, secrets, and passphrases MUST be passed as impl Into<SecretString>.
- All struct fields for credentials MUST use SecretString;
- DO NOT use String, &str, or any other type for credentials.
- Document credential fields as securely stored and expected as SecretString.

# Venue Credentials Structs

- Every venue with private/authenticated endpoints MUST define exactly one credentials struct with the name `Credentials` (for automatic discovery), for example:
- Placement:
  - File MUST live under `private/rest/credentials.rs` within the venue (e.g., `venues/src/okx/private/rest/credentials.rs`).
  - It MUST be re-exported from `private/rest/mod.rs` so callers can `use venues::<venue>::private::rest::CredentialsName`.
- Structure:
  - Derive: `Debug` (ensure secrets are redacted if custom), `Clone`, `Serialize`, `Deserialize`.
  - Use `#[serde(rename_all = "camelCase")]` and `#[serde(default)]` for optional fields.
  - Secret-bearing fields (keys, secrets, passphrases) use `SecretString`.
  - Include clear doc comments and mark optional fields explicitly.
- Client API:
  - Private `RestClient::new` MUST accept the venue’s `...Credentials` struct instead of separate key/secret/passphrase parameters.
  - Request signing and header building MUST read from the credentials struct and MUST NOT accept raw secrets elsewhere in public APIs.

# Examples

- Prefer construction like:
  - `let creds = Credentials { api_key, secret_key, passphrase};`
  - `let client = RestClient::new(base_url, http_client, rate_limiter, creds);`
