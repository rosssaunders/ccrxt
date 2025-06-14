---
applyTo: "venues/src/**"
---

# API Credential Handling

- All API keys, secrets, and passphrases MUST be passed as impl Into<SecretString>.
- All struct fields for credentials MUST use SecretString.
- DO NOT use String, &str, or any other type for credentials.
- Document credential fields as securely stored and expected as SecretString.
