use serde::{Deserialize, Serialize};
use serde_json::Value;
use super::enums::WebSocketEventType;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookEntry(pub String, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookSnapshot {
    pub last_update_id: u64,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    #[serde(rename = "e")]
    pub event_type: WebSocketEventType,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "pu")]
    pub previous_final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<OrderBookEntry>,
    #[serde(rename = "a")]
    pub asks: Vec<OrderBookEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    OrderBook(OrderBookUpdate),
    Raw(Value),
}

impl crate::websockets::VenueMessage for WebSocketMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceHeaders {
    #[serde(rename = "X-MBX-USED-WEIGHT-1M")]
    pub used_weight_1m: Option<u32>,
    #[serde(rename = "X-MBX-ORDER-COUNT-1M")]
    pub order_count_1m: Option<u32>,
    #[serde(rename = "X-MBX-ORDER-COUNT-1D")]
    pub order_count_1d: Option<u32>,
    #[serde(rename = "X-MBX-ORDER-COUNT-1S")]
    pub order_count_1s: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceResponse<T> {
    pub data: T,
    pub rate_limit_duration: Duration,
    pub request_duration: Duration,
    pub headers: BinanceHeaders,
} 