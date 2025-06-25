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
