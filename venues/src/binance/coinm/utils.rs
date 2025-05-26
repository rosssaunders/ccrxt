/// Appends a timestamp and signature to a query string.
///
/// This utility function is used for signing requests to private endpoints.
/// It adds the current UTC timestamp in milliseconds and generates a signature
/// using the provided signing function.
///
/// # Arguments
/// * `query_str` - The base query string (without timestamp or signature)
/// * `sign_fn` - A closure that takes the query string (with timestamp) and returns the signature
///
/// # Returns
/// The full query string with timestamp and signature appended.
pub fn append_signature<F, E>(mut query_str: String, sign_fn: F) -> Result<String, E>
where
    F: Fn(&str) -> Result<String, E>,
{
    if !query_str.is_empty() {
        query_str.push('&');
    }
    let signature = sign_fn(&query_str)?;
    query_str.push_str(&format!("&signature={}", signature));
    Ok(query_str)
}
