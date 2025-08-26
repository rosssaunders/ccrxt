pub mod accounts;
pub mod batch_borrowable;
pub mod borrowable;
pub mod get_unified_mode;
pub mod loan_records;
pub mod loans;
pub mod set_unified_mode;
pub mod transferable;
pub mod transferables;
pub mod unified_mode_types;

pub use crate::gateio::{PrivateRestClient as RestClient, RestResult};

// https://raw.githubusercontent.com/rosssaunders/coincise/refs/heads/main/docs/gateio/private_rest_api_unified.md
