pub mod rest;
pub mod websocket;

pub use self::rest::RestClient;
pub use self::rest::{GetCurrencyListRequest, GetCurrencyListResponse, Currency};
pub use self::rest::{GetTradingPairsListRequest, GetTradingPairsListResponse};
pub use self::rest::{GetTradingPairDetailsRequest, GetTradingPairDetailsResponse, TradingPairDetail};
pub use self::rest::{GetTickerAllPairsRequest, GetTickerAllPairsResponse, TickerArrayData};
pub use self::rest::{GetTickerRequest, GetTickerResponse, TickerData};
pub use self::rest::{GetLatestKlineRequest, GetLatestKlineResponse, LatestKlineData};
pub use self::rest::{GetHistoryKlineRequest, GetHistoryKlineResponse, HistoryKlineData};
pub use self::rest::{GetDepthRequest, GetDepthResponse, DepthData, OrderBookEntry};
pub use self::rest::{GetRecentTradesRequest, GetRecentTradesResponse, TradeData};
pub use self::websocket::WsClient as WebSocketClient;
pub use self::websocket::{
    DataResponse as WsDataResponse, DepthData as WsDepthData, DepthEntry, DepthLevel,
    ErrorResponse as WsErrorResponse, EventResponse as WsEventResponse, Operation as WsOperation,
    PublicChannel, TickerData as WsTickerData, WsError, WsMessage, WsResponse,
    BITMART_WS_PUBLIC_URL,
};