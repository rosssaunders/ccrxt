//! Integration tests for Binance Spot API endpoints
//!
//! This module contains comprehensive integration tests for all Binance Spot public REST API endpoints.
//! These tests run against the live Binance API using real market data to ensure correctness,
//! catch breaking changes, and provide a reference for expected API behavior.

mod binance;

pub use binance::*;
