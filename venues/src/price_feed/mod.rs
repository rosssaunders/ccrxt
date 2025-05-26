use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct PriceFeed {
    rate: Arc<RwLock<f64>>,
    last_update: Arc<RwLock<Instant>>,
    stale_after: Duration,
}

impl PriceFeed {
    pub fn new(initial_rate: f64, stale_after: Duration) -> Self {
        Self {
            rate: Arc::new(RwLock::new(initial_rate)),
            last_update: Arc::new(RwLock::new(Instant::now())),
            stale_after,
        }
    }

    pub async fn update_rate(&self, new_rate: f64) {
        let mut rate = self.rate.write().await;
        let mut last_update = self.last_update.write().await;
        *rate = new_rate;
        *last_update = Instant::now();
    }

    pub async fn get_rate(&self) -> Option<f64> {
        let last_update = *self.last_update.read().await;
        if last_update.elapsed() > self.stale_after {
            None
        } else {
            Some(*self.rate.read().await)
        }
    }

    pub async fn is_stale(&self) -> bool {
        let last_update = *self.last_update.read().await;
        last_update.elapsed() > self.stale_after
    }
}

#[derive(Debug, Clone)]
pub struct UsdConverter {
    usdc_usdt_feed: PriceFeed,
}

impl UsdConverter {
    pub fn new(stale_after: Duration) -> Self {
        Self {
            usdc_usdt_feed: PriceFeed::new(1.0, stale_after),
        }
    }

    pub async fn update_usdc_usdt_rate(&self, rate: f64) {
        self.usdc_usdt_feed.update_rate(rate).await;
    }

    pub async fn convert_usd_to_usdt(&self, usd_amount: f64) -> Option<f64> {
        self.usdc_usdt_feed
            .get_rate()
            .await
            .map(|rate| usd_amount * rate)
    }

    pub async fn convert_usdt_to_usd(&self, usdt_amount: f64) -> Option<f64> {
        self.usdc_usdt_feed
            .get_rate()
            .await
            .map(|rate| usdt_amount / rate)
    }

    pub async fn is_stale(&self) -> bool {
        self.usdc_usdt_feed.is_stale().await
    }
}
