pub mod account_book;
pub mod amend_batch_orders;
pub mod amend_order;
pub mod cancel_all_orders;
pub mod cancel_batch_orders;
pub mod cancel_order;
pub mod countdown_cancel_all;
pub mod create_batch_orders;
pub mod create_order;
pub mod cross_liquidate_orders;
pub mod get_order;
pub mod list_open_orders;
pub mod list_orders;
pub mod price_orders;
pub mod spot_accounts;
pub mod spot_trades;

pub use crate::gateio::{
    OrderSide, OrderStatus, OrderType, PrivateRestClient as RestClient, RestResult, StpMode,
    TimeInForce,
};
