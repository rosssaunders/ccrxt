use std::collections::BTreeMap;
use crate::OrderBook;
use std::fmt::Display;

/// Trait that all venues must implement
pub trait Venue: Display + Clone + Copy + PartialEq + Eq {
    /// Whether this venue's prices are denominated in USD
    fn is_usd_denominated(&self) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VenueSource {
    USD,    // USD denominated
    USDT,   // USDT denominated
}

impl VenueSource {
    pub fn is_usd_denominated(&self) -> bool {
        matches!(self, VenueSource::USD)
    }
}

impl Display for VenueSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenueSource::USD => write!(f, "USD"),
            VenueSource::USDT => write!(f, "USDT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AggregatedLevel {
    pub size: f64,
    pub sources: Vec<(String, f64)>, // Vector of (venue_name, size) pairs
}

impl AggregatedLevel {
    fn new() -> Self {
        Self {
            size: 0.0,
            sources: Vec::with_capacity(3), // Pre-allocate for our 3 venues
        }
    }

    fn update_source(&mut self, source: &str, size: f64) {
        // Update or remove the source
        if let Some(pos) = self.sources.iter().position(|(src, _)| src == source) {
            if size > 0.0 {
                self.sources[pos].1 = size;
            } else {
                self.sources.remove(pos);
            }
        } else if size > 0.0 {
            self.sources.push((source.to_string(), size));
        }

        // Recalculate total size
        self.size = self.sources.iter().map(|(_, size)| size).sum();
    }
}

#[derive(Debug)]
pub struct AggregatedOrderBook {
    bids: BTreeMap<i64, AggregatedLevel>,  // Price points stored as fixed point integers
    asks: BTreeMap<i64, AggregatedLevel>,
    price_precision: u32,
    usdt_rate: f64,  // Current USDT/USD rate
}

impl AggregatedOrderBook {
    pub fn new(price_precision: u32) -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            price_precision,
            usdt_rate: 1.0,
        }
    }

    pub fn update_usdt_rate(&mut self, rate: f64) {
        if (self.usdt_rate - rate).abs() > f64::EPSILON {
            self.usdt_rate = rate;
            // Clear the book as all prices need to be recalculated
            self.bids.clear();
            self.asks.clear();
        }
    }

    #[inline(always)]
    fn price_to_fixed(&self, mut price: f64, is_usd: bool) -> i64 {
        // Convert USD prices to USDT equivalent
        if is_usd {
            price *= self.usdt_rate;
        }
        (price * (10_f64.powi(self.price_precision as i32))) as i64
    }

    #[inline(always)]
    fn fixed_to_price(&self, fixed: i64, is_usd: bool) -> f64 {
        let price = fixed as f64 / (10_f64.powi(self.price_precision as i32));
        if is_usd {
            price / self.usdt_rate
        } else {
            price
        }
    }

    // Update a single price level from a specific venue
    #[inline(always)]
    pub fn update<T: Venue>(&mut self, price: f64, size: f64, is_bid: bool, venue: &T) {
        let fixed_price = self.price_to_fixed(price, venue.is_usd_denominated());
        let side = if is_bid { &mut self.bids } else { &mut self.asks };

        side.entry(fixed_price)
            .or_insert_with(AggregatedLevel::new)
            .update_source(&venue.to_string(), size);

        // Remove the price level if there's no liquidity left
        if side.get(&fixed_price).map_or(false, |level| level.size <= 0.0) {
            side.remove(&fixed_price);
        }
    }

    // Update from a venue's order book
    pub fn update_from_venue<T: Venue>(&mut self, orderbook: &OrderBook, venue: &T) {
        // First, clear all entries from this venue
        self.clear_venue(&venue.to_string());

        // Then apply all levels from the venue orderbook
        let (bids, asks) = orderbook.get_depth_with_prices(usize::MAX);
        
        // Process bids and asks
        for (price, level) in bids {
            self.update(price, level.size, true, venue);
        }
        
        for (price, level) in asks {
            self.update(price, level.size, false, venue);
        }
    }

    // Clear all entries from a specific venue
    pub fn clear_venue(&mut self, venue_name: &str) {
        // Helper closure to clear a side
        let clear_side = |side: &mut BTreeMap<i64, AggregatedLevel>| {
            let mut to_remove = Vec::new();
            
            for (price, level) in side.iter_mut() {
                level.update_source(venue_name, 0.0);
                if level.size <= 0.0 {
                    to_remove.push(*price);
                }
            }
            
            for price in to_remove {
                side.remove(&price);
            }
        };

        clear_side(&mut self.bids);
        clear_side(&mut self.asks);
    }

    // Get best bid and ask with source information
    #[inline(always)]
    pub fn best_bid_ask(&self) -> Option<(AggregatedLevel, AggregatedLevel)> {
        let best_bid = self.bids.iter().next_back().map(|(_, level)| level.clone());
        let best_ask = self.asks.iter().next().map(|(_, level)| level.clone());
        
        match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some((bid, ask)),
            _ => None,
        }
    }

    // Get best bid and ask prices in USDT
    #[inline(always)]
    pub fn best_bid_ask_prices(&self) -> Option<(f64, f64)> {
        let best_bid = self.bids.iter().next_back().map(|(&price, _)| price);
        let best_ask = self.asks.iter().next().map(|(&price, _)| price);
        
        match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some((
                bid as f64 / (10_f64.powi(self.price_precision as i32)),
                ask as f64 / (10_f64.powi(self.price_precision as i32))
            )),
            _ => None,
        }
    }

    // Get best bid and ask prices in source currency (USD for CoinM, USDT for others)
    #[inline(always)]
    pub fn best_bid_ask_prices_by_venue<T: Venue>(&self, venue: &T) -> Option<(f64, f64)> {
        let best_bid = self.bids.iter().next_back().map(|(&price, _)| self.fixed_to_price(price, venue.is_usd_denominated()));
        let best_ask = self.asks.iter().next().map(|(&price, _)| self.fixed_to_price(price, venue.is_usd_denominated()));
        
        match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some((bid, ask)),
            _ => None,
        }
    }

    // Get price levels up to specified depth with source information
    #[inline(always)]
    pub fn get_depth(&self, depth: usize) -> (Vec<(&AggregatedLevel, f64)>, Vec<(&AggregatedLevel, f64)>) {
        let bids: Vec<(&AggregatedLevel, f64)> = self.bids
            .iter()
            .rev()
            .take(depth)
            .map(|(&price, level)| (
                level,
                price as f64 / (10_f64.powi(self.price_precision as i32))
            ))
            .collect();
        
        let asks: Vec<(&AggregatedLevel, f64)> = self.asks
            .iter()
            .take(depth)
            .map(|(&price, level)| (
                level,
                price as f64 / (10_f64.powi(self.price_precision as i32))
            ))
            .collect();

        (bids, asks)
    }

    // Get volume available from a specific venue at a price level
    #[inline(always)]
    pub fn volume_from_venue<T: Venue>(&self, price: f64, is_bid: bool, venue: &T) -> f64 {
        let fixed_price = self.price_to_fixed(price, venue.is_usd_denominated());
        let side = if is_bid { &self.bids } else { &self.asks };

        side.get(&fixed_price)
            .and_then(|level| level.sources.iter().find(|(src, _)| src == &venue.to_string()))
            .map(|(_, size)| *size)
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregated_orderbook() {
        let mut aob = AggregatedOrderBook::new(8);
        
        // Test updates from different venues
        aob.update(100.0, 1.0, true, VenueSource::USD);
        aob.update(100.0, 2.0, true, VenueSource::USDT);
        
        // Check aggregated size
        if let Some((bid, _)) = aob.best_bid_ask() {
            assert_eq!(bid.size, 3.0);
            assert_eq!(bid.sources.len(), 2);
            
            let usd_size = bid.sources.iter()
                .find(|(src, _)| *src == "USD")
                .map(|(_, size)| *size)
                .unwrap();
            assert_eq!(usd_size, 1.0);
        }

        // Test clearing venue
        aob.clear_venue("USD");
        
        if let Some((bid, _)) = aob.best_bid_ask() {
            assert_eq!(bid.size, 2.0);
            assert_eq!(bid.sources.len(), 1);
        }

        // Test USD/USDT conversion
        aob.update_usdt_rate(0.99); // USDT is trading at $0.99
        aob.update(100.0, 1.0, true, VenueSource::USD); // $100 USD
        aob.update(99.0, 1.0, true, VenueSource::USDT);  // 99 USDT

        // Both orders should now be at the same price level in USDT terms
        if let Some((bid, _)) = aob.best_bid_ask() {
            assert_eq!(bid.size, 2.0);
            assert_eq!(bid.sources.len(), 2);
        }
    }
} 