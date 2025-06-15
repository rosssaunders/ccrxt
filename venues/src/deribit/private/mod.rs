pub mod rest;
pub mod verify_block_trade;

#[cfg(test)]
mod integration_tests;

pub use rest::RestClient;
pub use verify_block_trade::{
    VerifyBlockTradeRequest, VerifyBlockTradeResponse, VerifyBlockTradeResult,
    Trade, TradeRole, TradeDirection,
};