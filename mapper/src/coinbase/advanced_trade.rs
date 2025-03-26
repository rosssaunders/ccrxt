use venues::coinbase::advanced_trade::websocket::OrderBookUpdate as CoinbaseOrderBookUpdate;
use venues::coinbase::advanced_trade::websocket::OrderSide;
use rust_decimal::prelude::*;
use crate::OrderBookDecoder;

pub struct CoinbaseAdvancedTradeDecoder;

impl OrderBookDecoder for CoinbaseAdvancedTradeDecoder {
    type Update = CoinbaseOrderBookUpdate;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)> {
        let mut updates = Vec::new();
        
        // Convert Decimal to f64
        let price = update.price_level.to_f64().unwrap_or_default();
        let size = update.new_quantity.to_f64().unwrap_or_default();
        
        // true for bid, false for ask
        let is_bid = matches!(update.side, OrderSide::Buy);
        updates.push((price, size, is_bid));
        
        updates
    }
} 