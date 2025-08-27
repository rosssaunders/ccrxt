// Public financial product endpoints

// ETH staking endpoints
pub mod get_eth_staking_apy_history;

// SOL staking endpoints
pub mod get_sol_staking_apy_history;

// Simple earn endpoints
pub mod get_public_borrow_history;
pub mod get_public_borrow_info;

// Re-export key types
pub use get_eth_staking_apy_history::{EthStakingApyHistoryData, GetEthStakingApyHistoryRequest};
pub use get_public_borrow_history::{GetPublicBorrowHistoryRequest, PublicBorrowHistoryData};
pub use get_public_borrow_info::{GetPublicBorrowInfoRequest, PublicBorrowInfoData};
pub use get_sol_staking_apy_history::{GetSolStakingApyHistoryRequest, SolStakingApyHistoryData};

pub use crate::okx::public_client::RestClient;
