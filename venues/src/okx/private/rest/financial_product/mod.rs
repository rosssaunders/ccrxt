// Financial product endpoints (staking, earning, etc.)

// On-chain Earn endpoints
pub mod get_active_orders;
pub mod get_order_history;
pub mod post_staking_defi_purchase;

// ETH Staking endpoints
pub mod get_eth_staking_balance;
pub mod get_eth_staking_product_info;
pub mod get_eth_staking_purchase_redeem_history;

// SOL Staking endpoints
pub mod get_sol_staking_product_info;

// Simple Earn endpoints
pub mod get_saving_balance;
