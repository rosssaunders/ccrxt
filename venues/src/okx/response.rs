use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Standard OKX API response wrapper
#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
pub struct OkxApiResponse<T> {
    /// Response code: "0" for success
    pub code: String,

    /// Response message
    pub msg: String,

    /// Response data
    #[serde(deserialize_with = "deserialize_data_vec")]
    pub data: Vec<T>,
}

fn deserialize_data_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let v = Value::deserialize(deserializer)?;
    match v {
        Value::Array(items) => items
            .into_iter()
            .map(|item| T::deserialize(item).map_err(serde::de::Error::custom))
            .collect(),
        other => Ok(vec![
            T::deserialize(other).map_err(serde::de::Error::custom)?,
        ]),
    }
}
