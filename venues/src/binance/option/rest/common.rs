// Common REST utilities for Binance Options API

use std::time::Duration;

/// Common request utilities
pub fn default_timeout() -> Duration {
    Duration::from_secs(30)
}