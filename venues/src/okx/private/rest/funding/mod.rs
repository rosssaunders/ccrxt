// Funding Account endpoints
pub mod get_interest_accrued;
pub mod get_interest_limits;
pub mod get_interest_rate;
pub mod get_max_loan;
pub mod get_max_withdrawal;
pub mod get_quick_margin_borrow_repay_history;
pub mod get_spot_borrow_repay_history;
pub mod quick_margin_borrow_repay;
pub mod set_auto_loan;
pub mod set_auto_repay;
pub mod spot_manual_borrow_repay;

pub use crate::okx::private_client::RestClient;
