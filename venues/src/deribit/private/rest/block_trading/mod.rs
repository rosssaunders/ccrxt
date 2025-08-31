// Block Trading endpoints as documented in docs/private_block_trading.md

pub mod approve_block_trade;
pub mod execute_block_trade;
pub mod get_pending_block_trades;
pub mod invalidate_block_trade_signature;
pub mod simulate_block_trade;

// Re-export all types with specific imports to avoid conflicts
pub use approve_block_trade::{ApproveBlockTradeRequest, ApproveBlockTradeResponse, Role};
pub use execute_block_trade::{
    Direction, ExecuteBlockTradeRequest, ExecuteBlockTradeResponse, ExecutedTrade, Trade,
};
pub use get_pending_block_trades::*;
pub use invalidate_block_trade_signature::*;
pub use simulate_block_trade::{SimulateBlockTradeRequest, SimulateBlockTradeResponse};
