use venues::binance::spot::OrderBookUpdate as BinanceOrderBookUpdate;
use venues::okx::OrderBookUpdate as OkxOrderBookUpdate;
use venues::bybit::spot::OrderBookUpdate as BybitOrderBookUpdate;

pub trait OrderBookDecoder {
    type Update;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)>;
}

pub struct BinanceSpotDecoder;

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

pub struct BybitSpotDecoder;

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