use venues::bybit::spot::OrderBookUpdate as BybitOrderBookUpdate;
use venues::bybit::perp::OrderBookUpdate as BybitPerpOrderBookUpdate;
use crate::OrderBookDecoder;

pub struct BybitSpotDecoder;
pub struct BybitPerpDecoder;

impl OrderBookDecoder for BybitSpotDecoder {
    type Update = BybitOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Process bids
        for bid in &update.data.b {
            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                updates.push((price, size, true));
            }
        }
        
        // Process asks
        for ask in &update.data.a {
            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                updates.push((price, size, false));
            }
        }
        
        updates
    }
}

impl OrderBookDecoder for BybitPerpDecoder {
    type Update = BybitPerpOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Process bids
        for bid in &update.data.b {
            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                updates.push((price, size, true));
            }
        }
        
        // Process asks
        for ask in &update.data.a {
            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                updates.push((price, size, false));
            }
        }
        
        updates
    }
} 