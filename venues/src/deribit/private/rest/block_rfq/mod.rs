// Block RFQ endpoints as documented in docs/private_block_rfq.md

pub mod add_block_rfq_quote;
pub mod create_block_rfq;
pub mod edit_block_rfq_quote;
pub mod get_block_rfq_makers;
pub mod get_block_rfq_quotes;
pub mod get_block_rfq_user_info;
pub mod send_rfq;

// Re-export all types
pub use add_block_rfq_quote::{
    AddBlockRfqQuoteRequest, AddBlockRfqQuoteResponse, ResponseHedge as AddQuoteResponseHedge,
    ResponseLeg as AddQuoteResponseLeg,
};
pub use create_block_rfq::{
    CreateBlockRfqRequest, CreateBlockRfqResponse, ResponseHedge as CreateRfqResponseHedge,
    ResponseLeg as CreateRfqResponseLeg,
};
pub use edit_block_rfq_quote::*;
pub use get_block_rfq_makers::*;
pub use get_block_rfq_quotes::*;
pub use get_block_rfq_user_info::*;
pub use send_rfq::*;
