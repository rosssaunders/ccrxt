// Private REST endpoints module for Binance Spot


// Trading endpoints
pub mod amend_order;
pub mod cancel_all_orders;
pub mod cancel_order;
pub mod cancel_replace;
pub mod order;
pub mod test_order;

// Order list endpoints (OCO, OTO, OTOCO)
pub mod cancel_orderlist;
pub mod oco_order;
pub mod oco_orderlist;
pub mod oto_order;
pub mod otoco_order;

// SOR endpoints
pub mod new_sor_order;
pub mod test_sor_order;

// Account information endpoints
pub mod account;
pub mod account_commission;
pub mod all_orders;
pub mod my_trades;
pub mod open_orders;
pub mod query_order;

// Order list query endpoints
pub mod all_orderlist;
pub mod open_orderlist;
pub mod query_orderlist;

// Specialized endpoints
pub mod my_allocations;
pub mod my_prevented_matches;
pub mod order_amendments;
pub mod rate_limit_order;

