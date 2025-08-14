use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use crate::rate_limit_compat::{Quota, RateLimiter};
use crate::time_compat::Duration;

/// Enum for limit targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitTarget {
    Ip,
    ApiKey,
    Uid,
}

/// Struct describing a single endpoint's rate limit.
#[derive(Debug, Clone)]
pub struct EndpointRateLimit {
    pub path: Cow<'static, str>,
    pub target: LimitTarget,
    pub quota: Quota,
}

/// All BitMart contract endpoint rate limits.
pub static ENDPOINT_LIMITS: Lazy<Vec<EndpointRateLimit>> = Lazy::new(|| {
    vec![
        // Public endpoints (IP)
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/details"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/depth"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/open-interest"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/funding-rate"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/funding-rate-history"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/kline"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/markprice-kline"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/leverage-bracket"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/public/market-trade"),
            target: LimitTarget::Ip,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        // Private endpoints (API Key or UID)
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/submit-order"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/cancel-order"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(40).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/cancel-orders"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/submit-plan-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/cancel-plan-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(40).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/submit-tp-sl-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/modify-plan-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/modify-preset-plan-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/modify-tp-sl-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/modify-limit-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/cancel-all-after"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(4).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/submit-trail-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/cancel-trail-order"),
            target: LimitTarget::Uid,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/set-position-mode"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/get-position-mode"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/get-open-orders"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(50).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/order"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(50).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/order-history"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(6).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/trades"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(6).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/transaction-history"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(6).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/assets-detail"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/position"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(6).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/position-v2"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(6).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/submit-leverage"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/v1/transfer-contract"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(1).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/v1/transfer-contract-list"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(1).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/current-plan-order"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(50).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/position-risk"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(24).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/contract/private/trade-fee-rate"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(2).unwrap()),
        },
        // Sub-account endpoints (API Key)
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/main/v1/sub-to-main"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(8).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/main/v1/main-to-sub"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(8).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/sub/v1/sub-to-main"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(8).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/main/v1/wallet"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(12).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/v1/transfer-history"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(8).unwrap()),
        },
        EndpointRateLimit {
            path: Cow::Borrowed("/account/contract/sub-account/main/v1/transfer-list"),
            target: LimitTarget::ApiKey,
            quota: Quota::with_period(Duration::from_secs(2))
                .unwrap()
                .allow_burst(NonZeroU32::new(8).unwrap()),
        },
    ]
});

/// Rate limiter manager for BitMart contract endpoints.
#[derive(Debug, Default)]
pub struct BitmartRateLimiter {
    /// Keyed by (endpoint, target identifier)
    pub limiters: HashMap<
        (Cow<'static, str>, String),
        Arc<RateLimiter<String>>,
    >,
}

impl BitmartRateLimiter {
    pub fn new() -> Self {
        Self {
            limiters: HashMap::new(),
        }
    }

    /// Get or create a rate limiter for a given endpoint and target identifier (IP, API key, or UID).
    pub fn get_limiter(
        &mut self,
        endpoint: &str,
        target: LimitTarget,
        id: &str,
    ) -> Arc<RateLimiter<String>>
    {
        let key = (Cow::Owned(endpoint.to_string()), id.to_string());
        if let Some(limiter) = self.limiters.get(&key) {
            limiter.clone()
        } else {
            let quota = ENDPOINT_LIMITS
                .iter()
                .find(|e| e.path == endpoint && e.target == target)
                .map(|e| e.quota)
                .unwrap_or_else(|| Quota::per_second(NonZeroU32::new(1).unwrap()));
            let limiter = Arc::new(RateLimiter::keyed(quota));
            self.limiters.insert(key, limiter.clone());
            limiter
        }
    }
}
