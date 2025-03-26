use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct VenueMetrics {
    pub updates_processed: u64,
    pub reconnects: u64,
    pub last_update_latency_ms: u64,
    pub avg_update_latency_ms: f64,
    pub best_bid: f64,
    pub best_ask: f64,
    pub last_update_time: SystemTime,
}

impl Default for VenueMetrics {
    fn default() -> Self {
        Self {
            updates_processed: 0,
            reconnects: 0,
            last_update_latency_ms: 0,
            avg_update_latency_ms: 0.0,
            best_bid: 0.0,
            best_ask: 0.0,
            last_update_time: SystemTime::now(),
        }
    }
}

impl VenueMetrics {
    pub fn update_latency(&mut self, latency_ms: u64) {
        self.last_update_latency_ms = latency_ms;
        self.avg_update_latency_ms = (self.avg_update_latency_ms * self.updates_processed as f64 + latency_ms as f64) / 
                                   (self.updates_processed as f64 + 1.0);
        self.updates_processed += 1;
        self.last_update_time = SystemTime::now();
    }

    pub fn update_prices(&mut self, best_bid: f64, best_ask: f64) {
        self.best_bid = best_bid;
        self.best_ask = best_ask;
    }
} 