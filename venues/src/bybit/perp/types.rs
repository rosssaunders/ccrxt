use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel(pub String, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshotData {
    pub s: String,
    pub b: Vec<OrderBookLevel>,
    pub a: Vec<OrderBookLevel>,
    pub ts: u64,
    pub u: u64,
    pub seq: u64,
    pub cts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub retCode: i32,
    pub retMsg: String,
    pub result: OrderBookSnapshotData,
    pub retExtInfo: Value,
    pub time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdateData {
    pub s: String,
    pub b: Vec<OrderBookLevel>,
    pub a: Vec<OrderBookLevel>,
    pub u: u64,
    pub seq: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub topic: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub ts: u64,
    pub data: OrderBookUpdateData,
    #[serde(default)]
    pub cs: Option<u64>,
    pub cts: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub op: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionResponse {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub op: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    OrderBook(OrderBookUpdate),
    Ping(PingResponse),
    Subscription(SubscriptionResponse),
    Raw(Value),
}

impl crate::websockets::VenueMessage for WebSocketMessage {} 