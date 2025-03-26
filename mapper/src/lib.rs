pub mod binance;
pub mod okx;
pub mod bybit;
pub mod coinbase;

pub trait OrderBookDecoder {
    type Update;
    
    fn decode_update(&self, update: &Self::Update) -> Vec<(f64, f64, bool)>;
}

pub use binance::{BinanceSpotDecoder, BinanceCoinMDecoder, BinanceUsdMDecoder};
pub use okx::OkxDecoder;
pub use bybit::{BybitSpotDecoder, BybitPerpDecoder};
pub use coinbase::CoinbaseAdvancedTradeDecoder; 