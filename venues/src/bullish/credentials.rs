//! Bullish API credentials structure (moved from `private/rest/credentials.rs`).
//!
//! Provides secure storage wrappers for API key/secret. Secrets must never be logged
//! or serialized in plain text. Manual serde impl hides actual secret values.

use std::fmt;

use secrets::SecretString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
};

/// Credentials for Bullish private REST API authentication.
///
/// Provide securely (env vars / secret manager). Do NOT hard-code.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API key used to authenticate requests. Stored securely as `SecretString`.
    pub api_key: SecretString,
    /// API secret used for HMAC signing. Stored securely as `SecretString`.
    pub api_secret: SecretString,
}

impl Serialize for Credentials {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Credentials", 2)?;
        st.serialize_field("apiKey", &"***")?;
        st.serialize_field("apiSecret", &"***")?;
        st.end()
    }
}

impl<'de> Deserialize<'de> for Credentials {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CredsVisitor;
        impl<'de> Visitor<'de> for CredsVisitor {
            type Value = Credentials;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct Credentials")
            }
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut api_key: Option<String> = None;
                let mut api_secret: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "apiKey" => {
                            if api_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiKey"));
                            }
                            api_key = Some(map.next_value()?);
                        }
                        "apiSecret" => {
                            if api_secret.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiSecret"));
                            }
                            api_secret = Some(map.next_value()?);
                        }
                        _ => {
                            let _ignored: serde_json::Value = map.next_value()?;
                        }
                    }
                }
                let api_key = api_key.ok_or_else(|| serde::de::Error::missing_field("apiKey"))?;
                let api_secret =
                    api_secret.ok_or_else(|| serde::de::Error::missing_field("apiSecret"))?;
                Ok(Credentials {
                    api_key: SecretString::new(api_key.into_boxed_str()),
                    api_secret: SecretString::new(api_secret.into_boxed_str()),
                })
            }
        }
        deserializer.deserialize_struct("Credentials", &["apiKey", "apiSecret"], CredsVisitor)
    }
}
