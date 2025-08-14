/// Time compatibility module for WASM and native platforms
/// 
/// This module provides a unified interface for time operations that works
/// both in native environments and WASM (WebAssembly) contexts.

#[cfg(not(feature = "wasm"))]
pub use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[cfg(feature = "wasm")]
pub use web_time::{Duration, Instant, SystemTime, UNIX_EPOCH};