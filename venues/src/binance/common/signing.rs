/// Appends a timestamp and signature to a query string.
/// 
/// # Arguments
/// - `query_str`: The base query string (without timestamp or signature)
/// - `sign_fn`: A closure that takes the query string (with timestamp) and returns the signature
/// 
/// # Returns
/// The full query string with timestamp and signature appended.
pub fn append_timestamp_and_signature<F>(
    mut query_str: String,
    sign_fn: F,
) -> Result<String, YourErrorType>
where
    F: Fn(&str) -> Result<String, YourErrorType>,
{
    let timestamp = chrono::Utc::now().timestamp_millis();
    if !query_str.is_empty() {
        query_str.push('&');
    }
    query_str.push_str(&format!("timestamp={}", timestamp));
    let signature = sign_fn(&query_str)?;
    query_str.push_str(&format!("&signature={}", signature));
    Ok(query_str)
}