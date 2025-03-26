use venues::okx::OrderBookUpdate as OkxOrderBookUpdate;
use crate::OrderBookDecoder;

pub struct OkxDecoder;

impl OrderBookDecoder for OkxDecoder {
    type Update = OkxOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        for data in &update.data {
            // Process bids
            for bid in &data.bids {
                if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                    updates.push((price, size, true));
                }
            }
            
            // Process asks
            for ask in &data.asks {
                if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                    updates.push((price, size, false));
                }
            }
        }
        
        updates
    }
} 