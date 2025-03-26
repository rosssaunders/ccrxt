use venues::binance::spot::OrderBookUpdate as BinanceOrderBookUpdate;
use venues::binance::coinm::OrderBookUpdate as BinanceCoinMOrderBookUpdate;
use venues::binance::usdm::OrderBookUpdate as BinanceUsdMOrderBookUpdate;
use crate::OrderBookDecoder;

pub struct BinanceSpotDecoder;
pub struct BinanceCoinMDecoder;
pub struct BinanceUsdMDecoder;

impl OrderBookDecoder for BinanceSpotDecoder {
    type Update = BinanceOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Process bids
        for bid in &update.bids {
            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                updates.push((price, size, true));
            }
        }
        
        // Process asks
        for ask in &update.asks {
            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                updates.push((price, size, false));
            }
        }
        
        updates
    }
}

impl OrderBookDecoder for BinanceCoinMDecoder {
    type Update = BinanceCoinMOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Process bids
        for bid in &update.bids {
            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                updates.push((price, size, true));
            }
        }
        
        // Process asks
        for ask in &update.asks {
            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                updates.push((price, size, false));
            }
        }
        
        updates
    }
}

impl OrderBookDecoder for BinanceUsdMDecoder {
    type Update = BinanceUsdMOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Process bids
        for bid in &update.bids {
            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                updates.push((price, size, true));
            }
        }
        
        // Process asks
        for ask in &update.asks {
            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                updates.push((price, size, false));
            }
        }
        
        updates
    }
} 