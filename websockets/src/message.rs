use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use parking_lot::Mutex;
use tokio::sync::oneshot;

/// A monotonically increasing request identifier.
pub type RequestId = u64;

/// Thread-safe generator for request IDs.
#[derive(Debug, Default, Clone)]
pub struct RequestIdGenerator {
    counter: Arc<AtomicU64>,
}

impl RequestIdGenerator {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn next_id(&self) -> RequestId {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}

/// Tracks pending requests waiting for their correlated response.
#[derive(Debug, Default, Clone)]
pub struct RequestManager {
    inner: Arc<Mutex<HashMap<RequestId, oneshot::Sender<Vec<u8>>>>>,
}

impl RequestManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new request and get its ID and a receiver for the response.
    pub fn register(&self, id: RequestId) -> oneshot::Receiver<Vec<u8>> {
        let (tx, rx) = oneshot::channel();
        self.inner.lock().insert(id, tx);
        rx
    }

    /// Fulfill a pending request by ID; returns true if a waiter existed.
    pub fn fulfill(&self, id: RequestId, payload: Vec<u8>) -> bool {
        if let Some(sender) = self.inner.lock().remove(&id) {
            // It's okay if the receiver was dropped; just ignore the result.
            let _ = sender.send(payload);
            true
        } else {
            false
        }
    }

    /// Cancel a pending request, e.g., on disconnect.
    pub fn cancel_all(&self) {
        let mut map = self.inner.lock();
        for (_, sender) in map.drain() {
            let _ = sender.send(Vec::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    #[tokio::test]
    async fn request_manager_roundtrip() {
        let mgr = RequestManager::new();
        let id = 42;
        let rx = mgr.register(id);
        assert!(mgr.fulfill(id, vec![1, 2, 3]));
        let got = rx.await.expect("recv");
        assert_eq!(got, vec![1, 2, 3]);
    }

    #[test]
    fn id_generator_is_monotonic() {
        let g = RequestIdGenerator::new();
        let a = g.next_id();
        let b = g.next_id();
        let c = g.next_id();
        assert!(a < b && b < c);
    }

    #[tokio::test]
    async fn cancel_all_drains_waiters() {
        let mgr = RequestManager::new();
        let rx1 = mgr.register(1);
        let rx2 = mgr.register(2);
        mgr.cancel_all();
        // After cancel, receivers should complete (with empty payloads per impl)
        let v1 = rx1.await.expect("rx1");
        let v2 = rx2.await.expect("rx2");
        assert_eq!(v1, Vec::<u8>::new());
        assert_eq!(v2, Vec::<u8>::new());
    }

    #[test]
    fn duplicate_fulfill_only_first_wins() {
        let mgr = RequestManager::new();
        let rx = mgr.register(99);
        assert!(mgr.fulfill(99, b"one".to_vec()));
        // Second fulfill should return false as no waiter exists
        assert!(!mgr.fulfill(99, b"two".to_vec()));
        let got = futures::executor::block_on(async move { rx.await.expect("recv") });
        assert_eq!(got, b"one".to_vec());
    }

    proptest! {
        #[test]
        fn random_ids_roundtrip(ids in proptest::collection::vec(1u64..1_000_000, 1..50)) {
            let mgr = RequestManager::new();
            // register all
            let mut rxs = Vec::new();
            for &id in &ids {
                rxs.push((id, mgr.register(id)));
            }
            // fulfill all
            for &id in &ids {
                assert!(mgr.fulfill(id, id.to_le_bytes().to_vec()));
            }
            // drain
            for (id, rx) in rxs {
                let got = futures::executor::block_on(async move { rx.await.expect("recv") });
                assert_eq!(got, id.to_le_bytes().to_vec());
            }
        }
    }
}
