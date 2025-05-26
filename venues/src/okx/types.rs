use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel(pub String, pub String, pub String, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub asks: Vec<OrderBookLevel>,
    pub bids: Vec<OrderBookLevel>,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub action: String,
    pub arg: WebSocketArg,
    pub data: Vec<OrderBookData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketArg {
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookData {
    pub asks: Vec<OrderBookLevel>,
    pub bids: Vec<OrderBookLevel>,
    pub checksum: Option<i64>,
    #[serde(rename = "prevSeqId")]
    pub prev_seq_id: Option<u64>,
    #[serde(rename = "seqId")]
    pub seq_id: Option<u64>,
    pub ts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketRequest {
    pub op: String,
    pub args: Vec<WebSocketChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketChannel {
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub event: String,
    pub arg: WebSocketChannel,
    pub code: Option<String>,
    pub msg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    OrderBook(OrderBookUpdate),
    Response(WebSocketResponse),
    Raw(Value),
}

impl crate::websockets::VenueMessage for WebSocketMessage {}
