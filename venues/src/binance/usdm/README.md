General Endpoints
GET /fapi/v1/ping
Test connectivity to the Rest API

GET /fapi/v1/time
Get the current server time

GET /fapi/v1/exchangeInfo
Exchange trading rules and symbol information

Market Data Endpoints
GET /fapi/v1/depth
Query symbol orderbook

GET /fapi/v1/trades
Get recent market trades

GET /fapi/v1/historicalTrades
Get older market historical trades

GET /fapi/v1/aggTrades
Get compressed/aggregate market trades

GET /fapi/v1/klines
Kline/candlestick bars for a symbol

GET /fapi/v1/continuousKlines
Kline/candlestick bars for a specific contract type

GET /fapi/v1/indexPriceKlines
Kline/candlestick bars for the index price of a pair

GET /fapi/v1/markPriceKlines
Kline/candlestick bars for the mark price of a symbol

GET /fapi/v1/premiumIndexKlines
Premium index kline bars of a symbol

GET /fapi/v1/premiumIndex
Mark Price and Funding Rate

GET /fapi/v1/fundingRate
Get Funding Rate History

GET /fapi/v1/fundingInfo
Query funding rate info for symbols that had funding interval/cap/floor adjustment

GET /fapi/v1/ticker/24hr
24hr rolling window price change statistics

GET /fapi/v1/ticker/price
Latest price for a symbol or symbols

GET /fapi/v2/ticker/price
Latest price for a symbol or symbols (V2)

GET /fapi/v1/ticker/bookTicker
Best price/qty on the order book for a symbol or symbols

GET /futures/data/delivery-price
Quarterly contract settlement price

GET /fapi/v1/openInterest
Present open interest of a specific symbol

GET /futures/data/openInterestHist
Open interest statistics

GET /futures/data/topLongShortPositionRatio
Top trader long/short ratio (positions)

GET /futures/data/topLongShortAccountRatio
Top trader long/short ratio (accounts)

GET /futures/data/globalLongShortAccountRatio
Global long/short ratio

GET /futures/data/takerlongshortRatio
Taker buy/sell volume

GET /futures/data/basis
Query future basis

GET /fapi/v1/indexInfo
Composite index symbol information

GET /fapi/v1/assetIndex
Multi-assets mode asset index

GET /fapi/v1/constituents
Query index price constituents

## Implemented Private REST API Endpoints

### Order Management ✅
- `POST /fapi/v1/order` - New Order (order.rs)
- `PUT /fapi/v1/order` - Modify Order (order.rs) 
- `DELETE /fapi/v1/order` - Cancel Order (order.rs)
- `POST /fapi/v1/order/test` - Test New Order (order.rs)
- `POST /fapi/v1/batchOrders` - Place Multiple Orders (batch_order.rs)
- `PUT /fapi/v1/batchOrders` - Modify Multiple Orders (batch_order_modify.rs)
- `DELETE /fapi/v1/batchOrders` - Cancel Multiple Orders (batch_order_modify.rs)
- `GET /fapi/v1/orderAmendment` - Get Order Amendment History (order_amendment.rs)
- `DELETE /fapi/v1/allOpenOrders` - Cancel All Open Orders (cancel_all_orders.rs)
- `POST /fapi/v1/countdownCancelAll` - Auto-Cancel All Open Orders (cancel_all_orders.rs)
- `GET /fapi/v1/order` - Query Order (query_order.rs)
- `GET /fapi/v1/allOrders` - All Orders (all_orders.rs)
- `GET /fapi/v1/openOrders` - Open Orders (open_orders.rs)
- `GET /fapi/v1/openOrder` - Query Current Open Order (current_open_order.rs)
- `GET /fapi/v1/forceOrders` - User's Force Orders (force_orders.rs)

### Account & Trading Info ✅

- `GET /fapi/v1/userTrades` - Account Trade List (account_trades.rs)
- `GET /fapi/v2/positionRisk` - Position Information (position_risk.rs)
- `GET /fapi/v3/account` - Account Information (account.rs)

### Position & Margin Management ✅

- `POST /fapi/v1/marginType` - Change Margin Type (margin_type.rs)
- `POST /fapi/v1/positionSide/dual` - Change Position Mode (position_mode.rs)
- `POST /fapi/v1/leverage` - Change Initial Leverage (leverage.rs)
- `POST /fapi/v1/multiAssetsMargin` - Change Multi-Assets Mode (multi_assets_mode.rs)
- `POST /fapi/v1/positionMargin` - Modify Isolated Position Margin (position_margin.rs)

#### Position & Margin Management (Advanced) ✅

- `GET /fapi/v3/positionRisk` - Position Information V3 (position_risk_v3.rs)
- `GET /fapi/v1/adlQuantile` - Position ADL Quantile Estimation (adl_quantile.rs)
- `GET /fapi/v1/positionMargin/history` - Get Position Margin Change History (position_margin_history.rs)

#### Account Information (Advanced) ✅

- `GET /fapi/v3/balance` - Future Account Balance V3 (balance_v3.rs)
- `GET /fapi/v2/balance` - Future Account Balance V2 (balance_v2.rs)
- `GET /fapi/v2/account` - Account Information V2 (account_v2.rs)
- `GET /fapi/v1/commissionRate` - User Commission Rate (commission_rate.rs)
- `GET /fapi/v1/accountConfig` - Get User Account Configuration (account_config.rs)
- `GET /fapi/v1/symbolConfig` - Get User Symbol Configuration (symbol_config.rs)
- `GET /fapi/v1/rateLimit/order` - Query Current Order Count Usage (rate_limit_order.rs)
- `GET /fapi/v1/leverageBracket` - Notional and Leverage Brackets (leverage_bracket.rs)
- `GET /fapi/v1/multiAssetsMargin` - Get Multi-Assets Mode Status (multi_assets_margin_status.rs)
- `GET /fapi/v1/positionSide/dual` - Get Current Position Mode (position_mode_status.rs)
- `GET /fapi/v1/income` - Get Income History (income_history.rs)
- `GET /fapi/v1/apiTradingStatus` - Account API Trading Status (api_trading_status.rs)

#### Data Download (Async) ✅

- `GET /fapi/v1/income/asyn` - Get Download Id For Income History (income_download.rs)
- `GET /fapi/v1/income/asyn/id` - Get Download Link For Income History (income_download.rs)
- `GET /fapi/v1/order/asyn` - Get Download Id For Order History (order_download.rs)
- `GET /fapi/v1/order/asyn/id` - Get Download Link For Order History (order_download.rs)
- `GET /fapi/v1/trade/asyn` - Get Download Id For Trade History (trade_download.rs)
- `GET /fapi/v1/trade/asyn/id` - Get Download Link For Trade History (trade_download.rs)

#### Fee Management ✅

- `POST /fapi/v1/feeBurn` - Toggle BNB Burn On Spot Trade And Margin Interest (fee_management.rs)
- `GET /fapi/v1/feeBurn` - Get BNB Burn Status (fee_management.rs)

#### Convert Endpoints ✅

- `GET /fapi/v1/convert/exchangeInfo` - Query Convert Exchange Info (convert.rs)
- `POST /fapi/v1/convert/getQuote` - Send Quote Request (convert.rs)
- `POST /fapi/v1/convert/acceptQuote` - Accept Quote (convert.rs)
- `GET /fapi/v1/convert/orderStatus` - Order Status (convert.rs)

#### Portfolio Margin ✅

- `GET /fapi/v1/pmAccountInfo` - Portfolio Margin Account Information (portfolio_margin.rs)
