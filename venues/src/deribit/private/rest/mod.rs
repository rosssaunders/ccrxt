// Private REST endpoints organized by documentation sections
pub mod account_management;
pub mod block_rfq;
pub mod block_trading;
pub mod combo_trading;
pub mod session_management;
pub mod trading;
pub mod wallet;

// Re-export all types from organized sections
// Re-export with specific naming to avoid conflicts
pub use account_management::*;
pub use block_rfq::*;
// Import with aliases to avoid Trade struct conflicts
pub use block_trading::execute_block_trade::{
    ExecuteBlockTradeRequest, ExecuteBlockTradeResponse, ExecuteBlockTradeResult,
    Trade as ExecuteBlockTrade,
};
pub use block_trading::simulate_block_trade::{
    SimulateBlockTradeRequest, SimulateBlockTradeResponse, Trade as SimulateBlockTrade,
};
// Import block_trading items individually to avoid conflicts
pub use block_trading::{
    ApproveBlockTradeRequest, ApproveBlockTradeResponse, ExecutedTrade,
    GetPendingBlockTradesRequest, GetPendingBlockTradesResponse,
    InvalidateBlockTradeSignatureRequest, InvalidateBlockTradeSignatureResponse, PendingBlockTrade,
    PendingBlockTradeState, PendingBlockTradeTrade,
};
pub use combo_trading::*;
pub use session_management::*;
// Import trading::* first
pub use trading::*;
pub use wallet::*;
