pub mod candlesticks_index_price;
pub mod candlesticks_mark_price;
pub mod candlesticks_standard;
pub mod contracts;
pub mod insurance;
pub mod order_book;
pub mod risk_limit_tiers;
pub mod tickers;
pub mod trades;

// Re-export delivery types for easier access
pub use contracts::{DeliveryContract, DeliveryContractRequest, DeliveryContractsRequest};
pub use insurance::{DeliveryInsurance, DeliveryInsuranceRequest};
pub use order_book::{DeliveryOrderBook, DeliveryOrderBookEntry, DeliveryOrderBookRequest};
pub use risk_limit_tiers::{DeliveryRiskLimitTier, DeliveryRiskLimitTiersRequest};
pub use tickers::{DeliveryTicker, DeliveryTickersRequest};
pub use trades::{DeliveryTrade, DeliveryTradesRequest};

pub use crate::gateio::public_client::PublicRestClient as RestClient;
