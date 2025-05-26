use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// A simple paginator for endpoints that use monotonically increasing ID-based pagination.
/// This version is more explicit and easier to understand than the generic MonotonicIdPaginator.
/// 
/// # Example
/// ```rust
/// let mut paginator = SimpleIdPaginator::new(client, initial_req);
/// let mut all_items = Vec::new();
/// 
/// while let Some(items) = paginator.next_page().await? {
///     all_items.extend(items);
///     if all_items.len() >= max_items {
///         break;
///     }
/// }
/// ```
pub struct SimpleIdPaginator<Req, C> {
    client: C,
    request: Req,
    last_id: u64,
    is_done: bool,
}

impl<Req, C> SimpleIdPaginator<Req, C>
where
    Req: Clone + Send + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Creates a new simple ID-based paginator.
    /// 
    /// # Arguments
    /// * `client` - The API client
    /// * `request` - The initial request with pagination parameters
    pub fn new(client: C, request: Req) -> Self {
        Self {
            client,
            request,
            last_id: 0,
            is_done: false,
        }
    }

    /// Fetches the next page of results.
    /// 
    /// # Type Parameters
    /// * `T` - The item type that contains an ID field (e.g., Trade)
    /// * `E` - The error type
    /// 
    /// # Arguments
    /// * `fetch_page` - Closure that fetches a single page of results
    /// * `update_request` - Closure that updates the request with the next ID
    /// * `get_id` - Closure that extracts the ID from an item
    /// 
    /// # Returns
    /// * `Ok(Some(items))` - A page of items
    /// * `Ok(None)` - No more items to fetch
    /// * `Err(e)` - An error occurred
    pub async fn next_page<T, E, F, U, G>(
        &mut self,
        mut fetch_page: F,
        mut update_request: U,
        get_id: G,
    ) -> Result<Option<Vec<T>>, E>
    where
        T: Clone + Send + 'static,
        E: std::error::Error,
        F: FnMut(C, Req) -> Pin<Box<dyn Future<Output = Result<Vec<T>, E>> + Send>>,
        U: FnMut(&mut Req, u64),
        G: Fn(&T) -> u64,
    {
        if self.is_done {
            return Ok(None);
        }

        // Update the request with the next ID
        let next_id = self.last_id + 1;
        let mut current_req = self.request.clone();
        update_request(&mut current_req, next_id);

        // Fetch the page
        let items = fetch_page(self.client.clone(), current_req).await?;
        
        if items.is_empty() {
            self.is_done = true;
            return Ok(None);
        }

        // Update the last seen ID
        if let Some(last_item) = items.iter().max_by_key(|item| get_id(item)) {
            self.last_id = get_id(last_item);
        }

        Ok(Some(items))
    }

    /// Resets the paginator to start from the beginning
    pub fn reset(&mut self) {
        self.last_id = 0;
        self.is_done = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[derive(Debug, Clone)]
    struct MockItem {
        id: u64,
    }

    #[derive(Debug, Clone)]
    struct MockRequest {
        from_id: Option<u64>,
    }

    #[derive(Debug, Clone)]
    struct MockClient {
        counter: Arc<AtomicU64>,
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                counter: Arc::new(AtomicU64::new(0)),
            }
        }
    }

    #[tokio::test]
    async fn test_simple_paginator() {
        let client = MockClient::new();
        let request = MockRequest { from_id: None };
        let mut paginator = SimpleIdPaginator::new(client.clone(), request);

        let mut all_items = Vec::new();
        let mut page_count = 0;

        while let Some(items) = paginator.next_page(
            |client, req| {
                Box::pin(async move {
                    let current = client.counter.fetch_add(1, Ordering::SeqCst);
                    if current >= 5 {
                        Ok(Vec::new())
                    } else {
                        Ok(vec![
                            MockItem { id: current * 2 + 1 },
                            MockItem { id: current * 2 + 2 },
                        ])
                    }
                })
            },
            |req, next_id| {
                req.from_id = Some(next_id);
            },
            |item| item.id,
        ).await.unwrap() {
            all_items.extend(items);
            page_count += 1;
        }

        assert_eq!(page_count, 3);
        assert_eq!(all_items.len(), 6);
        assert_eq!(all_items[0].id, 1);
        assert_eq!(all_items[5].id, 6);
    }
} 