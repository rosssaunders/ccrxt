use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

/// A specialized paginator for endpoints that use monotonically increasing ID-based pagination.
/// 
/// This paginator handles tracking of the last seen ID internally and provides
/// a clean interface for fetching paginated results. It's optimized for endpoints
/// that use `from_id` or similar ID-based pagination mechanisms where IDs are
/// guaranteed to be monotonically increasing (each new item has a higher ID than
/// all previous items).
/// 
/// # Type Parameters
/// * `Req` - The request type that supports ID-based pagination
/// * `C` - The client type (e.g., Arc<Client>)
/// 
/// # Example
/// ```rust
/// let paginator = MonotonicIdPaginator::new(client, initial_req);
/// let all_items = paginator.fetch_all(|client, req| {
///     Box::pin(async move {
///         let resp = client.get_items(req).await?;
///         Ok(resp.data)
///     })
/// }, Some(10)).await?;
/// ```
pub struct MonotonicIdPaginator<Req, C> {
    client: C,
    request: Req,
    last_id: Arc<RwLock<u64>>,
}

impl<Req, C> MonotonicIdPaginator<Req, C>
where
    Req: Clone + Send + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Creates a new ID-based paginator.
    /// 
    /// # Arguments
    /// * `client` - The API client
    /// * `request` - The initial request with pagination parameters
    pub fn new(client: C, request: Req) -> Self {
        Self {
            client,
            request,
            last_id: Arc::new(RwLock::new(0)),
        }
    }

    /// Fetches all pages of results using ID-based pagination.
    /// 
    /// # Type Parameters
    /// * `T` - The item type that contains an ID field (e.g., Trade)
    /// * `E` - The error type
    /// 
    /// # Arguments
    /// * `fetch_page` - Closure that fetches a single page of results
    /// * `max_pages` - Optional maximum number of pages to fetch
    /// * `prepare_next_request` - Closure that updates the request with the next ID
    /// * `get_id` - Closure that extracts the ID from an item
    /// 
    /// # Returns
    /// A vector of all accumulated items, or the first error encountered.
    pub async fn fetch_all<T, E, F, U, G>(
        &self,
        mut fetch_page: F,
        max_pages: Option<usize>,
        mut prepare_next_request: U,
        get_id: G,
    ) -> Result<Vec<T>, E>
    where
        T: Clone + Send + 'static,
        E: std::error::Error,
        F: FnMut(C, Req) -> Pin<Box<dyn Future<Output = Result<Vec<T>, E>> + Send>>,
        U: FnMut(&mut Req, u64),
        G: Fn(&T) -> u64,
    {
        let mut all_items = Vec::new();
        let mut page_count = 0;
        let mut current_req = self.request.clone();

        loop {
            // Update the request with the next ID
            let next_id = *self.last_id.read().unwrap() + 1;
            prepare_next_request(&mut current_req, next_id);

            // Fetch the page
            let resp = fetch_page(self.client.clone(), current_req.clone()).await?;
            
            if resp.is_empty() {
                break;
            }

            // Sort by ID to ensure we track the highest ID
            let mut sorted_resp = resp.clone();
            sorted_resp.sort_by(|a, b| get_id(a).cmp(&get_id(b)));

            // Update the last seen ID
            if let Some(last_item) = sorted_resp.last() {
                *self.last_id.write().unwrap() = get_id(last_item);
            }

            all_items.extend_from_slice(&resp);
            page_count += 1;

            if let Some(max) = max_pages {
                if page_count >= max {
                    break;
                }
            }
        }

        Ok(all_items)
    }
}

mod simple_paginator;
pub use simple_paginator::SimpleIdPaginator;
