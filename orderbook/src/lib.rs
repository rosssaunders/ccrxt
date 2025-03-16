use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Level {
    pub size: f64,
}

#[derive(Debug)]
pub struct OrderBook {
    // Using BTreeMap for O(log n) lookups and ordered iteration
    // The bool in the tuple indicates side (true for ask, false for bid)
    bids: BTreeMap<i64, Level>,  // Price points stored as fixed point integers
    asks: BTreeMap<i64, Level>,
    price_precision: u32,        // Number of decimal places for price
}

impl OrderBook {
    pub fn new(price_precision: u32) -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            price_precision,
        }
    }

    #[inline(always)]
    fn price_to_fixed(&self, price: f64) -> i64 {
        (price * (10_f64.powi(self.price_precision as i32))) as i64
    }

    // Fast update for a price level
    #[inline(always)]
    pub fn update(&mut self, price: f64, size: f64, is_bid: bool) {
        let fixed_price = self.price_to_fixed(price);
        let side = if is_bid { &mut self.bids } else { &mut self.asks };

        if size <= 0.0 {
            side.remove(&fixed_price);
        } else {
            side.insert(fixed_price, Level { size });
        }
    }

    // Optimized batch update
    pub fn batch_update(&mut self, updates: &[(f64, f64, bool)]) {
        // Process updates in two passes for better cache locality
        for &(price, size, is_bid) in updates.iter() {
            if is_bid {
                self.update(price, size, true);
            }
        }
        
        for &(price, size, is_bid) in updates.iter() {
            if !is_bid {
                self.update(price, size, false);
            }
        }
    }

    // Get best bid and ask with O(1) complexity
    #[inline(always)]
    pub fn best_bid_ask(&self) -> Option<(Level, Level)> {
        let best_bid = self.bids.iter().next_back().map(|(_, level)| level.clone());
        let best_ask = self.asks.iter().next().map(|(_, level)| level.clone());
        
        match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => Some((bid, ask)),
            _ => None,
        }
    }

    // Get best bid and ask prices
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

    // Get price precision
    #[inline(always)]
    pub fn price_precision(&self) -> u32 {
        self.price_precision
    }

    // Get price levels up to specified depth
    #[inline(always)]
    pub fn get_depth(&self, depth: usize) -> (Vec<&Level>, Vec<&Level>) {
        let bids: Vec<&Level> = self.bids
            .values()
            .rev()
            .take(depth)
            .collect();
        
        let asks: Vec<&Level> = self.asks
            .values()
            .take(depth)
            .collect();

        (bids, asks)
    }

    // Get price levels up to specified depth with prices
    #[inline(always)]
    pub fn get_depth_with_prices(&self, depth: usize) -> (Vec<(f64, &Level)>, Vec<(f64, &Level)>) {
        let bids: Vec<(f64, &Level)> = self.bids
            .iter()
            .rev()
            .take(depth)
            .map(|(&price, level)| (
                price as f64 / (10_f64.powi(self.price_precision as i32)),
                level
            ))
            .collect();
        
        let asks: Vec<(f64, &Level)> = self.asks
            .iter()
            .take(depth)
            .map(|(&price, level)| (
                price as f64 / (10_f64.powi(self.price_precision as i32)),
                level
            ))
            .collect();

        (bids, asks)
    }

    // Calculate total volume up to a price level
    #[inline(always)]
    pub fn volume_at_price(&self, price: f64, is_bid: bool) -> f64 {
        let fixed_price = self.price_to_fixed(price);
        let side = if is_bid { &self.bids } else { &self.asks };

        if is_bid {
            side.range(fixed_price..)
                .map(|(_, level)| level.size)
                .sum()
        } else {
            side.range(..=fixed_price)
                .map(|(_, level)| level.size)
                .sum()
        }
    }

    // Apply a snapshot to reset the order book
    pub fn apply_snapshot(&mut self, bids: Vec<(f64, f64)>, asks: Vec<(f64, f64)>) {
        self.bids.clear();
        self.asks.clear();

        for (price, size) in bids {
            self.update(price, size, true);
        }

        for (price, size) in asks {
            self.update(price, size, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_updates() {
        let mut ob = OrderBook::new(8);
        
        // Test snapshot
        ob.apply_snapshot(
            vec![(100.0, 1.0), (99.0, 2.0)],
            vec![(101.0, 1.0), (102.0, 2.0)]
        );

        // Test best bid/ask
        let (bid, ask) = ob.best_bid_ask().unwrap();
        assert_eq!(bid.size, 1.0);  // Size at price 100.0
        assert_eq!(ask.size, 1.0);  // Size at price 101.0

        // Test update
        ob.update(100.0, 0.0, true); // Remove bid
        assert_eq!(ob.best_bid_ask().unwrap().0.size, 2.0);  // Size at price 99.0

        // Test batch update
        ob.batch_update(&[
            (98.0, 3.0, true),
            (103.0, 3.0, false),
            (97.0, 4.0, true),
        ]);

        let (bids, asks) = ob.get_depth(5);
        assert_eq!(bids.len(), 3);
        assert_eq!(asks.len(), 3);
    }
}

pub mod aggregated;
pub use aggregated::{AggregatedOrderBook, AggregatedLevel, VenueSource};
