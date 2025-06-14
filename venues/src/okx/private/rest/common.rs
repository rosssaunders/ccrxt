use serde::Deserialize;

/// Standard OKX API response wrapper
#[derive(Debug, Clone, Deserialize)]
pub struct OkxApiResponse<T> {
    /// Response code: "0" for success
    pub code: String,
    
    /// Response message
    pub msg: String,
    
    /// Response data
    pub data: Vec<T>,
}