# [#](#futures) Futures

Perpetual futures

## [#](#query-all-futures-contracts) Query all futures contracts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-all-futures-contracts](https://www.gate.io/docs/developers/apiv4/en/#query-all-futures-contracts)

> Code samples

`GET /futures/{settle}/contracts`

_Query all futures contracts_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturescontracts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturescontracts-parameters)

| Name   | In    | Type    | Required | Description                                         |
| ------ | ----- | ------- | -------- | --------------------------------------------------- |
| settle | path  | string  | true     | Settle currency                                     |
| limit  | query | integer | false    | Maximum number of records returned in a single list |
| offset | query | integer | false    | List offset, starting from 0                        |

#### [#](#enumerated-values-30) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "name": "BTC_USDT",
    "type": "direct",
    "quanto_multiplier": "0.0001",
    "ref_discount_rate": "0",
    "order_price_deviate": "0.5",
    "maintenance_rate": "0.005",
    "mark_type": "index",
    "last_price": "38026",
    "mark_price": "37985.6",
    "index_price": "37954.92",
    "funding_rate_indicative": "0.000219",
    "mark_price_round": "0.01",
    "funding_offset": 0,
    "in_delisting": false,
    "risk_limit_base": "1000000",
    "interest_rate": "0.0003",
    "order_price_round": "0.1",
    "order_size_min": 1,
    "ref_rebate_rate": "0.2",
    "funding_interval": 28800,
    "risk_limit_step": "1000000",
    "leverage_min": "1",
    "leverage_max": "100",
    "risk_limit_max": "8000000",
    "maker_fee_rate": "-0.00025",
    "taker_fee_rate": "0.00075",
    "funding_rate": "0.002053",
    "order_size_max": 1000000,
    "funding_next_apply": 1610035200,
    "short_users": 977,
    "config_change_time": 1609899548,
    "trade_size": 28530850594,
    "position_size": 5223816,
    "long_users": 455,
    "funding_impact_value": "60000",
    "orders_limit": 50,
    "trade_id": 10851092,
    "orderbook_id": 2129638396,
    "enable_bonus": true,
    "enable_credit": true,
    "create_time": 1669688556,
    "funding_cap_ratio": "0.75",
    "status": "trading",
    "launch_time": 1609899548,
    "delisting_time": 1609899548,
    "delisted_time": 1609899548
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturescontracts-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturescontracts-responses)

| Status | Meaning                                                                    | Description                 | Schema                          |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[Contract](#schemacontract)\] |

This operation does not require authentication

## [#](#query-single-contract-information) Query single contract information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-contract-information](https://www.gate.io/docs/developers/apiv4/en/#query-single-contract-information)

> Code samples

`GET /futures/{settle}/contracts/{contract}`

_Query single contract information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturescontract-parameters](https://www.gate.io/docs/developers/apiv4/en/#getfuturescontract-parameters)

| Name     | In   | Type   | Required | Description      |
| -------- | ---- | ------ | -------- | ---------------- |
| settle   | path | string | true     | Settle currency  |
| contract | path | string | true     | Futures contract |

#### [#](#enumerated-values-31) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "name": "BTC_USDT",
  "type": "direct",
  "quanto_multiplier": "0.0001",
  "ref_discount_rate": "0",
  "order_price_deviate": "0.5",
  "maintenance_rate": "0.005",
  "mark_type": "index",
  "last_price": "38026",
  "mark_price": "37985.6",
  "index_price": "37954.92",
  "funding_rate_indicative": "0.000219",
  "mark_price_round": "0.01",
  "funding_offset": 0,
  "in_delisting": false,
  "risk_limit_base": "1000000",
  "interest_rate": "0.0003",
  "order_price_round": "0.1",
  "order_size_min": 1,
  "ref_rebate_rate": "0.2",
  "funding_interval": 28800,
  "risk_limit_step": "1000000",
  "leverage_min": "1",
  "leverage_max": "100",
  "risk_limit_max": "8000000",
  "maker_fee_rate": "-0.00025",
  "taker_fee_rate": "0.00075",
  "funding_rate": "0.002053",
  "order_size_max": 1000000,
  "funding_next_apply": 1610035200,
  "short_users": 977,
  "config_change_time": 1609899548,
  "trade_size": 28530850594,
  "position_size": 5223816,
  "long_users": 455,
  "funding_impact_value": "60000",
  "orders_limit": 50,
  "trade_id": 10851092,
  "orderbook_id": 2129638396,
  "enable_bonus": true,
  "enable_credit": true,
  "create_time": 1669688556,
  "funding_cap_ratio": "0.75",
  "status": "trading",
  "launch_time": 1609899548,
  "delisting_time": 1609899548,
  "delisted_time": 1609899548
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturescontract-responses](https://www.gate.io/docs/developers/apiv4/en/#getfuturescontract-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Contract information | [Contract](#schemacontract) |

This operation does not require authentication

## [#](#query-futures-market-depth-information) Query futures market depth information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-depth-information](https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-depth-information)

> Code samples

`GET /futures/{settle}/order_book`

_Query futures market depth information_

Bids will be sorted by price from high to low, while asks sorted reversely

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-parameters)

| Name     | In    | Type    | Required | Description                                                                                   |
| -------- | ----- | ------- | -------- | --------------------------------------------------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                                                               |
| contract | query | string  | true     | Futures contract                                                                              |
| interval | query | string  | false    | Price precision for depth aggregation, 0 means no aggregation, defaults to 0 if not specified |
| limit    | query | integer | false    | Number of depth levels                                                                        |
| with_id  | query | boolean | false    | Whether to return depth update ID. This ID increments by 1 each time depth changes            |

#### [#](#enumerated-values-32) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "id": 123456,
  "current": 1623898993.123,
  "update": 1623898993.121,
  "asks": [
    {
      "p": "1.52",
      "s": 100
    },
    {
      "p": "1.53",
      "s": 40
    }
  ],
  "bids": [
    {
      "p": "1.17",
      "s": 150
    },
    {
      "p": "1.16",
      "s": 203
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-responses)

| Status | Meaning                                                                    | Description            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Depth query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesorderbook-responseschema)

Status Code **200**

| Name                        | Type           | Description                                                                                                    |
| --------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------- |
| » id                        | integer(int64) | Order Book ID. Increases by 1 on every order book change. Set `with_id=true` to include this field in response |
| » current                   | number(double) | Response data generation timestamp                                                                             |
| » update                    | number(double) | Order book changed timestamp                                                                                   |
| » asks                      | array          | Ask Depth                                                                                                      |
| »» futures_order_book_item  | object         | none                                                                                                           |
| »»» p                       | string         | Price (quote currency)                                                                                         |
| »»» s                       | integer(int64) | Amount                                                                                                         |
| »» bids                     | array          | Bid Depth                                                                                                      |
| »»» futures_order_book_item | object         | none                                                                                                           |
| »»»» p                      | string         | Price (quote currency)                                                                                         |
| »»»» s                      | integer(int64) | Amount                                                                                                         |

This operation does not require authentication

## [#](#futures-market-transaction-records) Futures market transaction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-transaction-records](https://www.gate.io/docs/developers/apiv4/en/#futures-market-transaction-records)

> Code samples

`GET /futures/{settle}/trades`

_Futures market transaction records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                                                                                                                                                                          |
| contract | query | string         | true     | Futures contract                                                                                                                                                                                         |
| limit    | query | integer        | false    | Maximum number of records returned in a single list                                                                                                                                                      |
| offset   | query | integer        | false    | List offset, starting from 0                                                                                                                                                                             |
| last_id  | query | string         | false    | Use the ID of the last record in the previous list as the starting point for the next list.This field is no longer supported. For new requests, please use the fromand tofields to specify the time rang |
| from     | query | integer(int64) | false    | Specify starting time in Unix seconds. If not specified, `to` and `limit` will be used to limit response items.                                                                                          |
| to       | query | integer(int64) | false    | Specify end time in Unix seconds, default to current time.                                                                                                                                               |

#### [#](#detailed-descriptions-23) Detailed descriptions

**from**: Specify starting time in Unix seconds. If not specified, `to` and
`limit` will be used to limit response items. If items between `from` and `to`
are more than `limit`, only `limit` number will be returned.

#### [#](#enumerated-values-33) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "id": 121234231,
    "create_time": 1514764800,
    "contract": "BTC_USDT",
    "size": -100,
    "price": "100.123"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturestrades-responseschema)

Status Code **200**

| Name             | Type           | Description                                                                                                                                                                                                                                                                                                                                      |
| ---------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| _None_           | array          | none                                                                                                                                                                                                                                                                                                                                             |
| » id             | integer(int64) | Fill ID                                                                                                                                                                                                                                                                                                                                          |
| » create_time    | number(double) | Fill Time                                                                                                                                                                                                                                                                                                                                        |
| » create_time_ms | number(double) | Trade time, with millisecond precision to 3 decimal places                                                                                                                                                                                                                                                                                       |
| » contract       | string         | Futures contract                                                                                                                                                                                                                                                                                                                                 |
| » size           | integer(int64) | Trading size                                                                                                                                                                                                                                                                                                                                     |
| » price          | string         | Trade price (quote currency)                                                                                                                                                                                                                                                                                                                     |
| » is_internal    | boolean        | Whether it is an internal trade. Internal trade refers to the takeover of liquidation orders by the insurance fund and ADL users. Since it is not a normal matching on the market depth, the trade price may deviate from the market, and it will not be recorded in the K-line. If it is not an internal trade, this field will not be returned |

This operation does not require authentication

## [#](#futures-market-k-line-chart) Futures market K-line chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-k-line-chart](https://www.gate.io/docs/developers/apiv4/en/#futures-market-k-line-chart)

> Code samples

`GET /futures/{settle}/candlesticks`

_Futures market K-line chart_

Return specified contract candlesticks. If prefix `contract` with `mark_`, the
contract's mark price candlesticks are returned; if prefix with `index_`, index
price candlesticks will be returned.

Maximum of 2000 points are returned in one query. Be sure not to exceed the
limit when specifying `from`, `to` and `interval`

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                                                        |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| settle   | path  | string         | true     | Settle currency                                                                                                                                                    |
| contract | query | string         | true     | Futures contract                                                                                                                                                   |
| from     | query | integer(int64) | false    | Start time of candlesticks, formatted in Unix timestamp in seconds. Default to`to - 100 * interval` if not specified                                               |
| to       | query | integer(int64) | false    | Specify the end time of the K-line chart, defaults to current time if not specified, note that the time format is Unix timestamp with second precision             |
| limit    | query | integer        | false    | Maximum number of recent data points to return. `limit` conflicts with `from` and `to`. If either `from` or `to` is specified, request will be rejected.           |
| interval | query | string         | false    | Interval time between data points. Note that `1w` means natural week(Mon-Sun), while `7d` means every 7d since unix 0. 30d represents a natural month, not 30 days |
| timezone | query | string         | false    | Time zone: all/utc0/utc8, default utc0                                                                                                                             |

#### [#](#enumerated-values-34) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |
| interval  | 10s   |
| interval  | 1m    |
| interval  | 5m    |
| interval  | 15m   |
| interval  | 30m   |
| interval  | 1h    |
| interval  | 4h    |
| interval  | 8h    |
| interval  | 1d    |
| interval  | 7d    |

> Example responses

> 200 Response

```
[
  {
    "t": 1539852480,
    "v": 97151,
    "c": "1.032",
    "h": "1.032",
    "l": "1.032",
    "o": "1.032",
    "sum": "3580"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturescandlesticks-responseschema)

Status Code **200**

| Name     | Type           | Description                                                              |
| -------- | -------------- | ------------------------------------------------------------------------ |
| _None_   | array          | \[data point in every timestamp\]                                        |
| » _None_ | object         | data point in every timestamp                                            |
| »» t     | number(double) | Unix timestamp in seconds                                                |
| »» v     | integer(int64) | size volume (contract size). Only returned if `contract` is not prefixed |
| »» c     | string         | Close price (quote currency)                                             |
| »» h     | string         | Highest price (quote currency)                                           |
| »» l     | string         | Lowest price (quote currency)                                            |
| »» o     | string         | Open price (quote currency)                                              |
| »» sum   | string         | Trading volume (unit: Quote currency)                                    |

This operation does not require authentication

## [#](#premium-index-k-line-chart) Premium Index K-line chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#premium-index-k-line-chart](https://www.gate.io/docs/developers/apiv4/en/#premium-index-k-line-chart)

> Code samples

`GET /futures/{settle}/premium_index`

_Premium Index K-line chart_

Maximum of 1000 points can be returned in a query. Be sure not to exceed the
limit when specifying from, to and interval

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                                                                                                                          |
| contract | query | string         | true     | Futures contract                                                                                                                                         |
| from     | query | integer(int64) | false    | Start time of candlesticks, formatted in Unix timestamp in seconds. Default to`to - 100 * interval` if not specified                                     |
| to       | query | integer(int64) | false    | Specify the end time of the K-line chart, defaults to current time if not specified, note that the time format is Unix timestamp with second precision   |
| limit    | query | integer        | false    | Maximum number of recent data points to return. `limit` conflicts with `from` and `to`. If either `from` or `to` is specified, request will be rejected. |
| interval | query | string         | false    | Time interval between data points                                                                                                                        |

#### [#](#enumerated-values-35) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |
| interval  | 10s   |
| interval  | 1m    |
| interval  | 5m    |
| interval  | 15m   |
| interval  | 30m   |
| interval  | 1h    |
| interval  | 4h    |
| interval  | 8h    |
| interval  | 1d    |
| interval  | 7d    |

> Example responses

> 200 Response

```
[
  {
    "t": 1539852480,
    "c": "0",
    "h": "0.00023",
    "l": "0",
    "o": "0"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturespremiumindex-responseschema)

Status Code **200**

| Name     | Type           | Description                   |
| -------- | -------------- | ----------------------------- |
| » _None_ | object         | data point in every timestamp |
| »» t     | number(double) | Unix timestamp in seconds     |
| »» c     | string         | Close price                   |
| »» h     | string         | Highest price                 |
| »» l     | string         | Lowest price                  |
| »» o     | string         | Open price                    |

This operation does not require authentication

## [#](#get-all-futures-trading-statistics) Get all futures trading statistics

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-all-futures-trading-statistics](https://www.gate.io/docs/developers/apiv4/en/#get-all-futures-trading-statistics)

> Code samples

`GET /futures/{settle}/tickers`

_Get all futures trading statistics_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-parameters)

| Name     | In    | Type   | Required | Description                                             |
| -------- | ----- | ------ | -------- | ------------------------------------------------------- |
| settle   | path  | string | true     | Settle currency                                         |
| contract | query | string | false    | Futures contract, return related data only if specified |

#### [#](#enumerated-values-36) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "contract": "BTC_USDT",
    "last": "6432",
    "low_24h": "6278",
    "high_24h": "6790",
    "change_percentage": "4.43",
    "total_size": "32323904",
    "volume_24h": "184040233284",
    "volume_24h_btc": "28613220",
    "volume_24h_usd": "184040233284",
    "volume_24h_base": "28613220",
    "volume_24h_quote": "184040233284",
    "volume_24h_settle": "28613220",
    "mark_price": "6534",
    "funding_rate": "0.0001",
    "funding_rate_indicative": "0.0001",
    "index_price": "6531",
    "highest_bid": "34089.7",
    "highest_size": "100",
    "lowest_ask": "34217.9",
    "lowest_size": "1000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturestickers-responseschema)

Status Code **200**

| Name                      | Type   | Description                                                                                                            |
| ------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------- |
| » contract                | string | Futures contract                                                                                                       |
| » last                    | string | Last trading price                                                                                                     |
| » change_percentage       | string | Price change percentage. Negative values indicate price decrease, e.g. -7.45                                           |
| » total_size              | string | Contract total size                                                                                                    |
| » low_24h                 | string | 24-hour lowest price                                                                                                   |
| » high_24h                | string | 24-hour highest price                                                                                                  |
| » volume_24h              | string | 24-hour trading volume                                                                                                 |
| » volume_24h_btc          | string | 24-hour trading volume in BTC (deprecated, use `volume_24h_base`, `volume_24h_quote`, `volume_24h_settle` instead)     |
| » volume_24h_usd          | string | 24-hour trading volume in USD (deprecated, use `volume_24h_base`, `volume_24h_quote`, `volume_24h_settle` instead)     |
| » volume_24h_base         | string | 24-hour trading volume in base currency                                                                                |
| » volume_24h_quote        | string | 24-hour trading volume in quote currency                                                                               |
| » volume_24h_settle       | string | 24-hour trading volume in settle currency                                                                              |
| » mark_price              | string | Recent mark price                                                                                                      |
| » funding_rate            | string | Funding rate                                                                                                           |
| » funding_rate_indicative | string | Indicative Funding rate in next period. (deprecated. use `funding_rate`)                                               |
| » index_price             | string | Index price                                                                                                            |
| » quanto_base_rate        | string | Exchange rate of base currency and settlement currency in Quanto contract. Does not exists in contracts of other types |
| » lowest_ask              | string | Recent lowest ask                                                                                                      |
| » lowest_size             | string | The latest seller's lowest price order quantity                                                                        |
| » highest_bid             | string | Recent highest bid                                                                                                     |
| » highest_size            | string | The latest buyer's highest price order volume                                                                          |
| » change_utc0             | string | utc0 涨跌百分比，跌用负数标识，如 -7.45                                                                                |
| » change_utc8             | string | utc8 涨跌百分比，跌用负数标识，如 -7.45                                                                                |
| » change_price            | string | 24h 涨跌额，跌用负数标识，如 -7.45                                                                                     |
| » change_utc0_price       | string | utc0 涨跌额，跌用负数标识，如 -7.45                                                                                    |
| » change_utc8_price       | string | utc8 涨跌额，跌用负数标识，如 -7.45                                                                                    |

This operation does not require authentication

## [#](#futures-market-historical-funding-rate) Futures market historical funding rate

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-historical-funding-rate](https://www.gate.io/docs/developers/apiv4/en/#futures-market-historical-funding-rate)

> Code samples

`GET /futures/{settle}/funding_rate`

_Futures market historical funding rate_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-parameters)

| Name     | In    | Type           | Required | Description                                         |
| -------- | ----- | -------------- | -------- | --------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                     |
| contract | query | string         | true     | Futures contract                                    |
| limit    | query | integer        | false    | Maximum number of records returned in a single list |
| from     | query | integer(int64) | false    | Start timestamp                                     |
| to       | query | integer(int64) | false    | Termination Timestamp                               |

#### [#](#detailed-descriptions-24) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-37) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "t": 1543968000,
    "r": "0.000157"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-responses)

| Status | Meaning                                                                    | Description              | Schema     |
| ------ | -------------------------------------------------------------------------- | ------------------------ | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | History query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesfundingratehistory-responseschema)

Status Code **200**

| Name | Type           | Description               |
| ---- | -------------- | ------------------------- |
| » t  | integer(int64) | Unix timestamp in seconds |
| » r  | string         | Funding rate              |

This operation does not require authentication

## [#](#futures-market-insurance-fund-history) Futures market insurance fund history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-insurance-fund-history](https://www.gate.io/docs/developers/apiv4/en/#futures-market-insurance-fund-history)

> Code samples

`GET /futures/{settle}/insurance`

_Futures market insurance fund history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-parameters)

| Name   | In    | Type    | Required | Description                                         |
| ------ | ----- | ------- | -------- | --------------------------------------------------- |
| settle | path  | string  | true     | Settle currency                                     |
| limit  | query | integer | false    | Maximum number of records returned in a single list |

#### [#](#enumerated-values-38) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "t": 1543968000,
    "b": "83.0031"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesinsuranceledger-responseschema)

Status Code **200**

| Name   | Type           | Description               |
| ------ | -------------- | ------------------------- |
| _None_ | array          | none                      |
| » t    | integer(int64) | Unix timestamp in seconds |
| » b    | string         | Insurance balance         |

This operation does not require authentication

## [#](#futures-statistics) Futures statistics

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-statistics](https://www.gate.io/docs/developers/apiv4/en/#futures-statistics)

> Code samples

`GET /futures/{settle}/contract_stats`

_Futures statistics_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-parameters)

| Name     | In    | Type           | Required | Description      |
| -------- | ----- | -------------- | -------- | ---------------- |
| settle   | path  | string         | true     | Settle currency  |
| contract | query | string         | true     | Futures contract |
| from     | query | integer(int64) | false    | Start timestamp  |
| interval | query | string         | false    | none             |
| limit    | query | integer        | false    | none             |

#### [#](#enumerated-values-39) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1603865400,
    "lsr_taker": 100,
    "lsr_account": 0.5,
    "long_liq_size": 0,
    "short_liq_size": 0,
    "open_interest": 124724,
    "short_liq_usd": 0,
    "mark_price": "8865",
    "top_lsr_size": 1.02,
    "short_liq_amount": 0,
    "long_liq_amount": 0,
    "open_interest_usd": 1511,
    "top_lsr_account": 1.5,
    "long_liq_usd": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-responses](https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcontractstats-responseschema)

Status Code **200**

| Name                | Type           | Description                                 |
| ------------------- | -------------- | ------------------------------------------- |
| » time              | integer(int64) | Stat timestamp                              |
| » lsr_taker         | number         | Long/short taker ratio                      |
| » lsr_account       | number         | Long/short position user ratio              |
| » long_liq_size     | integer(int64) | Long liquidation size (contracts)           |
| » long_liq_amount   | number(double) | Long liquidation amount (base currency)     |
| » long_liq_usd      | number(double) | Long liquidation volume (quote currency)    |
| » short_liq_size    | integer(int64) | Short liquidation size (contracts)          |
| » short_liq_amount  | number(double) | Short liquidation amount (base currency)    |
| » short_liq_usd     | number(double) | Short liquidation volume (quote currency)   |
| » open_interest     | integer(int64) | Total open interest size (contracts)        |
| » open_interest_usd | number(double) | Total open interest volume (quote currency) |
| » top_lsr_account   | number(double) | Top trader long/short account ratio         |
| » top_lsr_size      | number(double) | Top trader long/short position ratio        |
| » mark_price        | number(double) | 标记Price                                   |

This operation does not require authentication

## [#](#query-index-constituents) Query index constituents

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-index-constituents](https://www.gate.io/docs/developers/apiv4/en/#query-index-constituents)

> Code samples

`GET /futures/{settle}/index_constituents/{index}`

_Query index constituents_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-parameters](https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-parameters)

| Name   | In   | Type   | Required | Description     |
| ------ | ---- | ------ | -------- | --------------- |
| settle | path | string | true     | Settle currency |
| index  | path | string | true     | Index name      |

#### [#](#enumerated-values-40) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "index": "BTC_USDT",
  "constituents": [
    {
      "exchange": "Binance",
      "symbols": [
        "BTC_USDT"
      ]
    },
    {
      "exchange": "Gate.com",
      "symbols": [
        "BTC_USDT"
      ]
    },
    {
      "exchange": "Huobi",
      "symbols": [
        "BTC_USDT"
      ]
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-responses](https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getindexconstituents-responseschema)

Status Code **200**

| Name                | Type   | Description  |
| ------------------- | ------ | ------------ |
| » index             | string | Index name   |
| » constituents      | array  | Constituents |
| »» IndexConstituent | object | none         |
| »»» exchange        | string | Exchange     |
| »»» symbols         | array  | Symbol list  |

This operation does not require authentication

## [#](#query-liquidation-order-history) Query liquidation order history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-order-history](https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-order-history)

> Code samples

`GET /futures/{settle}/liq_orders`

_Query liquidation order history_

The time interval between from and to is maximum 3600. Some private fields are
not returned by public interfaces, refer to field descriptions for details

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |

#### [#](#detailed-descriptions-25) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-41) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1548654951,
    "contract": "BTC_USDT",
    "size": 600,
    "order_size": -600,
    "order_price": "3405",
    "fill_price": "3424",
    "left": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listliquidatedorders-responseschema)

Status Code **200**

| Name          | Type           | Description                           |
| ------------- | -------------- | ------------------------------------- |
| » time        | integer(int64) | Liquidation time                      |
| » contract    | string         | Futures contract                      |
| » size        | integer(int64) | User position size                    |
| » order_size  | integer(int64) | Number of forced liquidation orders   |
| » order_price | string         | Liquidation order price               |
| » fill_price  | string         | Liquidation order average taker price |
| » left        | integer(int64) | System liquidation order maker size   |

This operation does not require authentication

## [#](#query-risk-limit-tiers) Query risk limit tiers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-tiers](https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-tiers)

> Code samples

`GET /futures/{settle}/risk_limit_tiers`

_Query risk limit tiers_

When the 'contract' parameter is not passed, the default is to query the risk
limits for the top 100 markets.'Limit' and 'offset' correspond to pagination
queries at the market level, not to the length of the returned array. This only
takes effect empty.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-parameters)

| Name     | In    | Type    | Required | Description                                             |
| -------- | ----- | ------- | -------- | ------------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                         |
| contract | query | string  | false    | Futures contract, return related data only if specified |
| limit    | query | integer | false    | Maximum number of records returned in a single list     |
| offset   | query | integer | false    | List offset, starting from 0                            |

#### [#](#enumerated-values-42) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "maintenance_rate": "0.01",
    "tier": 1,
    "initial_rate": "0.02",
    "leverage_max": "50",
    "risk_limit": "20000",
    "contract": "ZTX_USDT",
    "deduction": "0"
  },
  {
    "maintenance_rate": "0.013",
    "tier": 2,
    "initial_rate": "0.025",
    "leverage_max": "40",
    "risk_limit": "30000",
    "contract": "ZTX_USDT",
    "deduction": "60"
  },
  {
    "maintenance_rate": "0.015",
    "tier": 3,
    "initial_rate": "0.02857",
    "leverage_max": "35",
    "risk_limit": "50000",
    "contract": "ZTX_USDT",
    "deduction": "120"
  },
  {
    "maintenance_rate": "0.02",
    "tier": 4,
    "initial_rate": "0.03333",
    "leverage_max": "30",
    "risk_limit": "70000",
    "contract": "ZTX_USDT",
    "deduction": "370"
  },
  {
    "maintenance_rate": "0.025",
    "tier": 5,
    "initial_rate": "0.04",
    "leverage_max": "25",
    "risk_limit": "100000",
    "contract": "ZTX_USDT",
    "deduction": "720"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesrisklimittiers-responseschema)

Status Code **200**

| Name                | Type         | Description                                                                           |
| ------------------- | ------------ | ------------------------------------------------------------------------------------- |
| _None_              | array        | \[Retrieve risk limit configurations for different tiers under a specified contract\] |
| » _None_            | object       | Retrieve risk limit configurations for different tiers under a specified contract     |
| »» tier             | integer(int) | Tier                                                                                  |
| »» risk_limit       | string       | Position risk limit                                                                   |
| »» initial_rate     | string       | Initial margin rate                                                                   |
| »» maintenance_rate | string       | Maintenance margin rate                                                               |
| »» leverage_max     | string       | Maximum leverage                                                                      |
| »» contract         | string       | Market, only visible when market pagination is requested                              |
| »» deduction        | string       | Maintenance margin quick calculation deduction amount                                 |

This operation does not require authentication

## [#](#get-futures-account) Get futures account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-futures-account](https://www.gate.io/docs/developers/apiv4/en/#get-futures-account)

> Code samples

`GET /futures/{settle}/accounts`

_Get futures account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-parameters)

| Name   | In   | Type   | Required | Description     |
| ------ | ---- | ------ | -------- | --------------- |
| settle | path | string | true     | Settle currency |

#### [#](#enumerated-values-43) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 1666,
  "currency": "USDT",
  "total": "9707.803567115145",
  "unrealised_pnl": "3371.248828",
  "position_margin": "38.712189181",
  "order_margin": "0",
  "available": "9669.091377934145",
  "point": "0",
  "bonus": "0",
  "in_dual_mode": false,
  "enable_evolved_classic": false,
  "cross_initial_margin": "61855.56788525",
  "cross_maintenance_margin": "682.04678105",
  "cross_order_margin": "0",
  "cross_unrealised_pnl": "1501.178222634128",
  "cross_available": "27549.406108813951",
  "cross_margin_balance": "10371.77306201952",
  "cross_mmr": "797.2134",
  "cross_imr": "116.6097",
  "isolated_position_margin": "0",
  "history": {
    "dnw": "10000",
    "pnl": "68.3685",
    "fee": "-1.645812875",
    "refr": "0",
    "fund": "-358.919120009855",
    "point_dnw": "0",
    "point_fee": "0",
    "point_refr": "0",
    "bonus_dnw": "0",
    "bonus_offset": "0"
  },
  "enable_tiered_mm": true
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccounts-responseschema)

Status Code **200**

| Name                                                                           | Type    | Description                                                                                                                                                                                     |
| ------------------------------------------------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » total                                                                        | string  | total is the balance after the user's accumulated deposit, withdraw, profit and loss (including realized profit and loss, fund, fee and referral rebate), excluding unrealized profit and loss. |
| total = SUM(history_dnw, history_pnl, history_fee, history_refr, history_fund) |
| » unrealised_pnl                                                               | string  | Unrealized PNL                                                                                                                                                                                  |
| » position_margin                                                              | string  | Position margin                                                                                                                                                                                 |
| » order_margin                                                                 | string  | Order margin of unfinished orders                                                                                                                                                               |
| » available                                                                    | string  | Available balance for transferring or trading (including bonus. Bonus cannot be withdrawn, so transfer amount needs to deduct bonus)                                                            |
| » point                                                                        | string  | Point card amount                                                                                                                                                                               |
| » currency                                                                     | string  | Settlement currency                                                                                                                                                                             |
| » in_dual_mode                                                                 | boolean | Whether dual mode is enabled                                                                                                                                                                    |
| » position_mode                                                                | string  | 持仓模式，single-单向持仓，dual-双向持仓，split-分仓(in_dual_mode失效了)                                                                                                                        |
| » enable_credit                                                                | boolean | Whether portfolio margin account mode is enabled                                                                                                                                                |
| » position_initial_margin                                                      | string  | Initial margin occupied by positions, applicable to unified account mode                                                                                                                        |
| » maintenance_margin                                                           | string  | Maintenance margin occupied by positions, applicable to new classic account margin mode and unified account mode                                                                                |
| » bonus                                                                        | string  | Bonus                                                                                                                                                                                           |
| » enable_evolved_classic                                                       | boolean | Classic account margin mode, true-new mode, false-old mode                                                                                                                                      |
| » cross_order_margin                                                           | string  | Cross margin order margin, applicable to new classic account margin mode                                                                                                                        |
| » cross_initial_margin                                                         | string  | Cross margin initial margin, applicable to new classic account margin mode                                                                                                                      |
| » cross_maintenance_margin                                                     | string  | Cross margin maintenance margin, applicable to new classic account margin mode                                                                                                                  |
| » cross_unrealised_pnl                                                         | string  | Cross margin unrealized P&L, applicable to new classic account margin mode                                                                                                                      |
| » cross_available                                                              | string  | Cross margin available balance, applicable to new classic account margin mode                                                                                                                   |
| » cross_margin_balance                                                         | string  | Cross margin balance, applicable to new classic account margin mode                                                                                                                             |
| » cross_mmr                                                                    | string  | Cross margin maintenance margin rate, applicable to new classic account margin mode                                                                                                             |
| » cross_imr                                                                    | string  | Cross margin initial margin rate, applicable to new classic account margin mode                                                                                                                 |
| » isolated_position_margin                                                     | string  | Isolated position margin, applicable to new classic account margin mode                                                                                                                         |
| » enable_new_dual_mode                                                         | boolean | Whether to open a new two-way position mode                                                                                                                                                     |
| » margin_mode                                                                  | integer | Margin mode, 0-classic margin mode, 1-cross-currency margin mode, 2-combined margin mode                                                                                                        |
| » enable_tiered_mm                                                             | boolean | Whether to enable tiered maintenance margin calculation                                                                                                                                         |
| » position_voucher_total                                                       | string  | Total Position Experience Coupon Amount in Account                                                                                                                                              |
| » history                                                                      | object  | Statistical data                                                                                                                                                                                |
| »» dnw                                                                         | string  | total amount of deposit and withdraw                                                                                                                                                            |
| »» pnl                                                                         | string  | total amount of trading profit and loss                                                                                                                                                         |
| »» fee                                                                         | string  | total amount of fee                                                                                                                                                                             |
| »» refr                                                                        | string  | total amount of referrer rebates                                                                                                                                                                |
| »» fund                                                                        | string  | total amount of funding costs                                                                                                                                                                   |
| »» point_dnw                                                                   | string  | total amount of point deposit and withdraw                                                                                                                                                      |
| »» point_fee                                                                   | string  | total amount of point fee                                                                                                                                                                       |
| »» point_refr                                                                  | string  | total amount of referrer rebates of point fee                                                                                                                                                   |
| »» bonus_dnw                                                                   | string  | total amount of perpetual contract bonus transfer                                                                                                                                               |
| »» bonus_offset                                                                | string  | total amount of perpetual contract bonus deduction                                                                                                                                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-futures-account-change-history) Query futures account change history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-account-change-history](https://www.gate.io/docs/developers/apiv4/en/#query-futures-account-change-history)

> Code samples

`GET /futures/{settle}/account_book`

_Query futures account change history_

If the contract field is passed, only records containing this field after
2023-10-30 can be filtered.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| type     | query | string         | false    | Changing Type:                                          |

#### [#](#detailed-descriptions-26) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

**type**: Changing Type:

- dnw: Deposit & Withdraw
- pnl: Profit & Loss by reducing position
- fee: Trading fee
- refr: Referrer rebate
- fund: Funding
- point_dnw: point_fee: POINT Trading fee
- point_refr: POINT Referrer rebate

#### [#](#enumerated-values-44) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1682294400.123456,
    "change": "0.000010152188",
    "balance": "4.59316525194",
    "text": "ETH_USD:6086261",
    "type": "fee",
    "contract": "ETH_USD",
    "trade_id": "1",
    "id": "1"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listfuturesaccountbook-responseschema)

Status Code **200**

| Name      | Type           | Description          |
| --------- | -------------- | -------------------- |
| _None_    | array          | none                 |
| » time    | number(double) | Change time          |
| » change  | string         | Change amount        |
| » balance | string         | Balance after change |
| » type    | string         | Changing Type：      |

\- dnw: Deposit & Withdraw  
\- pnl: Profit & Loss by reducing position  
\- fee: Trading fee  
\- refr: Referrer rebate  
\- fund: Funding  
\- point_dnw: point_fee: POINT Trading fee  
\- point_refr: POINT Referrer rebate  
\- bonus_offset: bouns deduction | | » text | string | Comment | | » contract |
string | Futures contract, the field is only available for data after 2023-10-30
| | » trade_id | string | trade id | | » id | string | Account change record ID
|

#### [#](#enumerated-values-45) Enumerated Values

| Property | Value        |
| -------- | ------------ |
| type     | dnw          |
| type     | pnl          |
| type     | fee          |
| type     | refr         |
| type     | fund         |
| type     | point_dnw    |
| type     | point_fee    |
| type     | point_refr   |
| type     | bonus_offset |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-user-position-list) Get user position list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-user-position-list](https://www.gate.io/docs/developers/apiv4/en/#get-user-position-list)

> Code samples

`GET /futures/{settle}/positions`

_Get user position list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpositions-parameters](https://www.gate.io/docs/developers/apiv4/en/#listpositions-parameters)

| Name    | In    | Type    | Required | Description                                           |
| ------- | ----- | ------- | -------- | ----------------------------------------------------- |
| settle  | path  | string  | true     | Settle currency                                       |
| holding | query | boolean | false    | Return only real positions - true, return all - false |
| limit   | query | integer | false    | Maximum number of records returned in a single list   |
| offset  | query | integer | false    | List offset, starting from 0                          |

#### [#](#enumerated-values-46) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpositions-responses](https://www.gate.io/docs/developers/apiv4/en/#listpositions-responses)

| Status | Meaning                                                                    | Description                 | Schema                          |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-single-position-information) Get single position information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-single-position-information](https://www.gate.io/docs/developers/apiv4/en/#get-single-position-information)

> Code samples

`GET /futures/{settle}/positions/{contract}`

_Get single position information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getposition-parameters](https://www.gate.io/docs/developers/apiv4/en/#getposition-parameters)

| Name     | In   | Type   | Required | Description      |
| -------- | ---- | ------ | -------- | ---------------- |
| settle   | path | string | true     | Settle currency  |
| contract | path | string | true     | Futures contract |

#### [#](#enumerated-values-47) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 10000,
  "contract": "BTC_USDT",
  "size": -9440,
  "leverage": "0",
  "risk_limit": "100",
  "leverage_max": "100",
  "maintenance_rate": "0.005",
  "value": "3568.62",
  "margin": "4.431548146258",
  "entry_price": "3779.55",
  "liq_price": "99999999",
  "mark_price": "3780.32",
  "unrealised_pnl": "-0.000507486844",
  "realised_pnl": "0.045543982432",
  "pnl_pnl": "0.045543982432",
  "pnl_fund": "0",
  "pnl_fee": "0",
  "history_pnl": "0",
  "last_close_pnl": "0",
  "realised_point": "0",
  "history_point": "0",
  "adl_ranking": 5,
  "pending_orders": 16,
  "close_order": {
    "id": 232323,
    "price": "3779",
    "is_liq": false
  },
  "mode": "single",
  "update_time": 1684994406,
  "update_id": 1,
  "cross_leverage_limit": "0",
  "risk_limit_table": "BIG_HOT_COIN_50X_V2",
  "average_maintenance_rate": "0.005"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getposition-responses](https://www.gate.io/docs/developers/apiv4/en/#getposition-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-margin) Update position margin

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-margin](https://www.gate.io/docs/developers/apiv4/en/#update-position-margin)

> Code samples

`POST /futures/{settle}/positions/{contract}/margin`

_Update position margin_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionmargin-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatepositionmargin-parameters)

| Name     | In    | Type   | Required | Description                                                                |
| -------- | ----- | ------ | -------- | -------------------------------------------------------------------------- |
| settle   | path  | string | true     | Settle currency                                                            |
| contract | path  | string | true     | Futures contract                                                           |
| change   | query | string | true     | Margin change amount, positive number increases, negative number decreases |

#### [#](#enumerated-values-48) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 10000,
  "contract": "BTC_USDT",
  "size": -9440,
  "leverage": "0",
  "risk_limit": "100",
  "leverage_max": "100",
  "maintenance_rate": "0.005",
  "value": "3568.62",
  "margin": "4.431548146258",
  "entry_price": "3779.55",
  "liq_price": "99999999",
  "mark_price": "3780.32",
  "unrealised_pnl": "-0.000507486844",
  "realised_pnl": "0.045543982432",
  "pnl_pnl": "0.045543982432",
  "pnl_fund": "0",
  "pnl_fee": "0",
  "history_pnl": "0",
  "last_close_pnl": "0",
  "realised_point": "0",
  "history_point": "0",
  "adl_ranking": 5,
  "pending_orders": 16,
  "close_order": {
    "id": 232323,
    "price": "3779",
    "is_liq": false
  },
  "mode": "single",
  "update_time": 1684994406,
  "update_id": 1,
  "cross_leverage_limit": "0",
  "risk_limit_table": "BIG_HOT_COIN_50X_V2",
  "average_maintenance_rate": "0.005"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionmargin-responses](https://www.gate.io/docs/developers/apiv4/en/#updatepositionmargin-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-leverage) Update position leverage

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage](https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage)

> Code samples

`POST /futures/{settle}/positions/{contract}/leverage`

_Update position leverage_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionleverage-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatepositionleverage-parameters)

| Name                 | In    | Type           | Required | Description                                             |
| -------------------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle               | path  | string         | true     | Settle currency                                         |
| contract             | path  | string         | true     | Futures contract                                        |
| leverage             | query | string         | true     | New position leverage                                   |
| cross_leverage_limit | query | string         | false    | Cross margin leverage (valid only when `leverage` is 0) |
| pid                  | query | integer(int32) | false    | 产品ID                                                  |

#### [#](#enumerated-values-49) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 10000,
  "contract": "BTC_USDT",
  "size": -9440,
  "leverage": "0",
  "risk_limit": "100",
  "leverage_max": "100",
  "maintenance_rate": "0.005",
  "value": "3568.62",
  "margin": "4.431548146258",
  "entry_price": "3779.55",
  "liq_price": "99999999",
  "mark_price": "3780.32",
  "unrealised_pnl": "-0.000507486844",
  "realised_pnl": "0.045543982432",
  "pnl_pnl": "0.045543982432",
  "pnl_fund": "0",
  "pnl_fee": "0",
  "history_pnl": "0",
  "last_close_pnl": "0",
  "realised_point": "0",
  "history_point": "0",
  "adl_ranking": 5,
  "pending_orders": 16,
  "close_order": {
    "id": 232323,
    "price": "3779",
    "is_liq": false
  },
  "mode": "single",
  "update_time": 1684994406,
  "update_id": 1,
  "cross_leverage_limit": "0",
  "risk_limit_table": "BIG_HOT_COIN_50X_V2",
  "average_maintenance_rate": "0.005"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionleverage-responses](https://www.gate.io/docs/developers/apiv4/en/#updatepositionleverage-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#switch-position-margin-mode) Switch Position Margin Mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#switch-position-margin-mode](https://www.gate.io/docs/developers/apiv4/en/#switch-position-margin-mode)

> Code samples

`POST /futures/{settle}/positions/cross_mode`

_Switch Position Margin Mode_

> Body parameter

```
{
  "mode": "ISOLATED",
  "contract": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositioncrossmode-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatepositioncrossmode-parameters)

| Name       | In   | Type   | Required | Description                                                                                      |
| ---------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------ |
| settle     | path | string | true     | Settle currency                                                                                  |
| body       | body | object | true     | none                                                                                             |
| » mode     | body | string | true     | Cross margin or isolated margin mode. ISOLATED - isolated margin mode, CROSS - cross margin mode |
| » contract | body | string | true     | Futures market                                                                                   |

#### [#](#enumerated-values-50) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 10000,
  "contract": "BTC_USDT",
  "size": -9440,
  "leverage": "0",
  "risk_limit": "100",
  "leverage_max": "100",
  "maintenance_rate": "0.005",
  "value": "3568.62",
  "margin": "4.431548146258",
  "entry_price": "3779.55",
  "liq_price": "99999999",
  "mark_price": "3780.32",
  "unrealised_pnl": "-0.000507486844",
  "realised_pnl": "0.045543982432",
  "pnl_pnl": "0.045543982432",
  "pnl_fund": "0",
  "pnl_fee": "0",
  "history_pnl": "0",
  "last_close_pnl": "0",
  "realised_point": "0",
  "history_point": "0",
  "adl_ranking": 5,
  "pending_orders": 16,
  "close_order": {
    "id": 232323,
    "price": "3779",
    "is_liq": false
  },
  "mode": "single",
  "update_time": 1684994406,
  "update_id": 1,
  "cross_leverage_limit": "0",
  "risk_limit_table": "BIG_HOT_COIN_50X_V2",
  "average_maintenance_rate": "0.005"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositioncrossmode-responses](https://www.gate.io/docs/developers/apiv4/en/#updatepositioncrossmode-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#switch-between-cross-and-isolated-margin-modes-under-hedge-mode) Switch Between Cross and Isolated Margin Modes Under Hedge Mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#switch-between-cross-and-isolated-margin-modes-under-hedge-mode](https://www.gate.io/docs/developers/apiv4/en/#switch-between-cross-and-isolated-margin-modes-under-hedge-mode)

> Code samples

`POST /futures/{settle}/dual_comp/positions/cross_mode`

_Switch Between Cross and Isolated Margin Modes Under Hedge Mode_

> Body parameter

```
{
  "mode": "ISOLATED",
  "contract": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualcomppositioncrossmode-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedualcomppositioncrossmode-parameters)

| Name       | In   | Type   | Required | Description                                                                                      |
| ---------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------ |
| settle     | path | string | true     | Settle currency                                                                                  |
| body       | body | object | true     | none                                                                                             |
| » mode     | body | string | true     | Cross margin or isolated margin mode. ISOLATED - isolated margin mode, CROSS - cross margin mode |
| » contract | body | string | true     | Futures market                                                                                   |

#### [#](#enumerated-values-51) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualcomppositioncrossmode-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedualcomppositioncrossmode-responses)

| Status | Meaning                                                                    | Description      | Schema                          |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-risk-limit) Update position risk limit

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit](https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit)

> Code samples

`POST /futures/{settle}/positions/{contract}/risk_limit`

_Update position risk limit_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionrisklimit-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatepositionrisklimit-parameters)

| Name       | In    | Type   | Required | Description          |
| ---------- | ----- | ------ | -------- | -------------------- |
| settle     | path  | string | true     | Settle currency      |
| contract   | path  | string | true     | Futures contract     |
| risk_limit | query | string | true     | New risk limit value |

#### [#](#enumerated-values-52) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 10000,
  "contract": "BTC_USDT",
  "size": -9440,
  "leverage": "0",
  "risk_limit": "100",
  "leverage_max": "100",
  "maintenance_rate": "0.005",
  "value": "3568.62",
  "margin": "4.431548146258",
  "entry_price": "3779.55",
  "liq_price": "99999999",
  "mark_price": "3780.32",
  "unrealised_pnl": "-0.000507486844",
  "realised_pnl": "0.045543982432",
  "pnl_pnl": "0.045543982432",
  "pnl_fund": "0",
  "pnl_fee": "0",
  "history_pnl": "0",
  "last_close_pnl": "0",
  "realised_point": "0",
  "history_point": "0",
  "adl_ranking": 5,
  "pending_orders": 16,
  "close_order": {
    "id": 232323,
    "price": "3779",
    "is_liq": false
  },
  "mode": "single",
  "update_time": 1684994406,
  "update_id": 1,
  "cross_leverage_limit": "0",
  "risk_limit_table": "BIG_HOT_COIN_50X_V2",
  "average_maintenance_rate": "0.005"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatepositionrisklimit-responses](https://www.gate.io/docs/developers/apiv4/en/#updatepositionrisklimit-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#set-position-mode) Set position mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#set-position-mode](https://www.gate.io/docs/developers/apiv4/en/#set-position-mode)

> Code samples

`POST /futures/{settle}/dual_mode`

_Set position mode_

The prerequisite for changing mode is that all positions have no holdings and no
pending orders

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setdualmode-parameters](https://www.gate.io/docs/developers/apiv4/en/#setdualmode-parameters)

| Name      | In    | Type    | Required | Description                 |
| --------- | ----- | ------- | -------- | --------------------------- |
| settle    | path  | string  | true     | Settle currency             |
| dual_mode | query | boolean | true     | Whether to enable dual mode |

#### [#](#enumerated-values-53) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "user": 1666,
  "currency": "USDT",
  "total": "9707.803567115145",
  "unrealised_pnl": "3371.248828",
  "position_margin": "38.712189181",
  "order_margin": "0",
  "available": "9669.091377934145",
  "point": "0",
  "bonus": "0",
  "in_dual_mode": false,
  "enable_evolved_classic": false,
  "cross_initial_margin": "61855.56788525",
  "cross_maintenance_margin": "682.04678105",
  "cross_order_margin": "0",
  "cross_unrealised_pnl": "1501.178222634128",
  "cross_available": "27549.406108813951",
  "cross_margin_balance": "10371.77306201952",
  "cross_mmr": "797.2134",
  "cross_imr": "116.6097",
  "isolated_position_margin": "0",
  "history": {
    "dnw": "10000",
    "pnl": "68.3685",
    "fee": "-1.645812875",
    "refr": "0",
    "fund": "-358.919120009855",
    "point_dnw": "0",
    "point_fee": "0",
    "point_refr": "0",
    "bonus_dnw": "0",
    "bonus_offset": "0"
  },
  "enable_tiered_mm": true
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setdualmode-responses](https://www.gate.io/docs/developers/apiv4/en/#setdualmode-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Updated successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setdualmode-responseschema](https://www.gate.io/docs/developers/apiv4/en/#setdualmode-responseschema)

Status Code **200**

| Name                                                                           | Type    | Description                                                                                                                                                                                     |
| ------------------------------------------------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » total                                                                        | string  | total is the balance after the user's accumulated deposit, withdraw, profit and loss (including realized profit and loss, fund, fee and referral rebate), excluding unrealized profit and loss. |
| total = SUM(history_dnw, history_pnl, history_fee, history_refr, history_fund) |
| » unrealised_pnl                                                               | string  | Unrealized PNL                                                                                                                                                                                  |
| » position_margin                                                              | string  | Position margin                                                                                                                                                                                 |
| » order_margin                                                                 | string  | Order margin of unfinished orders                                                                                                                                                               |
| » available                                                                    | string  | Available balance for transferring or trading (including bonus. Bonus cannot be withdrawn, so transfer amount needs to deduct bonus)                                                            |
| » point                                                                        | string  | Point card amount                                                                                                                                                                               |
| » currency                                                                     | string  | Settlement currency                                                                                                                                                                             |
| » in_dual_mode                                                                 | boolean | Whether dual mode is enabled                                                                                                                                                                    |
| » position_mode                                                                | string  | 持仓模式，single-单向持仓，dual-双向持仓，split-分仓(in_dual_mode失效了)                                                                                                                        |
| » enable_credit                                                                | boolean | Whether portfolio margin account mode is enabled                                                                                                                                                |
| » position_initial_margin                                                      | string  | Initial margin occupied by positions, applicable to unified account mode                                                                                                                        |
| » maintenance_margin                                                           | string  | Maintenance margin occupied by positions, applicable to new classic account margin mode and unified account mode                                                                                |
| » bonus                                                                        | string  | Bonus                                                                                                                                                                                           |
| » enable_evolved_classic                                                       | boolean | Classic account margin mode, true-new mode, false-old mode                                                                                                                                      |
| » cross_order_margin                                                           | string  | Cross margin order margin, applicable to new classic account margin mode                                                                                                                        |
| » cross_initial_margin                                                         | string  | Cross margin initial margin, applicable to new classic account margin mode                                                                                                                      |
| » cross_maintenance_margin                                                     | string  | Cross margin maintenance margin, applicable to new classic account margin mode                                                                                                                  |
| » cross_unrealised_pnl                                                         | string  | Cross margin unrealized P&L, applicable to new classic account margin mode                                                                                                                      |
| » cross_available                                                              | string  | Cross margin available balance, applicable to new classic account margin mode                                                                                                                   |
| » cross_margin_balance                                                         | string  | Cross margin balance, applicable to new classic account margin mode                                                                                                                             |
| » cross_mmr                                                                    | string  | Cross margin maintenance margin rate, applicable to new classic account margin mode                                                                                                             |
| » cross_imr                                                                    | string  | Cross margin initial margin rate, applicable to new classic account margin mode                                                                                                                 |
| » isolated_position_margin                                                     | string  | Isolated position margin, applicable to new classic account margin mode                                                                                                                         |
| » enable_new_dual_mode                                                         | boolean | Whether to open a new two-way position mode                                                                                                                                                     |
| » margin_mode                                                                  | integer | Margin mode, 0-classic margin mode, 1-cross-currency margin mode, 2-combined margin mode                                                                                                        |
| » enable_tiered_mm                                                             | boolean | Whether to enable tiered maintenance margin calculation                                                                                                                                         |
| » position_voucher_total                                                       | string  | Total Position Experience Coupon Amount in Account                                                                                                                                              |
| » history                                                                      | object  | Statistical data                                                                                                                                                                                |
| »» dnw                                                                         | string  | total amount of deposit and withdraw                                                                                                                                                            |
| »» pnl                                                                         | string  | total amount of trading profit and loss                                                                                                                                                         |
| »» fee                                                                         | string  | total amount of fee                                                                                                                                                                             |
| »» refr                                                                        | string  | total amount of referrer rebates                                                                                                                                                                |
| »» fund                                                                        | string  | total amount of funding costs                                                                                                                                                                   |
| »» point_dnw                                                                   | string  | total amount of point deposit and withdraw                                                                                                                                                      |
| »» point_fee                                                                   | string  | total amount of point fee                                                                                                                                                                       |
| »» point_refr                                                                  | string  | total amount of referrer rebates of point fee                                                                                                                                                   |
| »» bonus_dnw                                                                   | string  | total amount of perpetual contract bonus transfer                                                                                                                                               |
| »» bonus_offset                                                                | string  | total amount of perpetual contract bonus deduction                                                                                                                                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-position-information-in-dual-mode) Get position information in dual mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-position-information-in-dual-mode](https://www.gate.io/docs/developers/apiv4/en/#get-position-information-in-dual-mode)

> Code samples

`GET /futures/{settle}/dual_comp/positions/{contract}`

_Get position information in dual mode_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdualmodeposition-parameters](https://www.gate.io/docs/developers/apiv4/en/#getdualmodeposition-parameters)

| Name     | In   | Type   | Required | Description      |
| -------- | ---- | ------ | -------- | ---------------- |
| settle   | path | string | true     | Settle currency  |
| contract | path | string | true     | Futures contract |

#### [#](#enumerated-values-54) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdualmodeposition-responses](https://www.gate.io/docs/developers/apiv4/en/#getdualmodeposition-responses)

| Status | Meaning                                                                    | Description      | Schema                          |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-margin-in-dual-mode) Update position margin in dual mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-margin-in-dual-mode](https://www.gate.io/docs/developers/apiv4/en/#update-position-margin-in-dual-mode)

> Code samples

`POST /futures/{settle}/dual_comp/positions/{contract}/margin`

_Update position margin in dual mode_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionmargin-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionmargin-parameters)

| Name      | In    | Type   | Required | Description                                                                |
| --------- | ----- | ------ | -------- | -------------------------------------------------------------------------- |
| settle    | path  | string | true     | Settle currency                                                            |
| contract  | path  | string | true     | Futures contract                                                           |
| change    | query | string | true     | Margin change amount, positive number increases, negative number decreases |
| dual_side | query | string | true     | Long or short position                                                     |

#### [#](#enumerated-values-55) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionmargin-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionmargin-responses)

| Status | Meaning                                                                    | Description      | Schema                          |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-leverage-in-dual-mode) Update position leverage in dual mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage-in-dual-mode](https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage-in-dual-mode)

> Code samples

`POST /futures/{settle}/dual_comp/positions/{contract}/leverage`

_Update position leverage in dual mode_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionleverage-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionleverage-parameters)

| Name                 | In    | Type   | Required | Description                                             |
| -------------------- | ----- | ------ | -------- | ------------------------------------------------------- |
| settle               | path  | string | true     | Settle currency                                         |
| contract             | path  | string | true     | Futures contract                                        |
| leverage             | query | string | true     | New position leverage                                   |
| cross_leverage_limit | query | string | false    | Cross margin leverage (valid only when `leverage` is 0) |

#### [#](#enumerated-values-56) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionleverage-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionleverage-responses)

| Status | Meaning                                                                    | Description      | Schema                          |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-risk-limit-in-dual-mode) Update position risk limit in dual mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit-in-dual-mode](https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit-in-dual-mode)

> Code samples

`POST /futures/{settle}/dual_comp/positions/{contract}/risk_limit`

_Update position risk limit in dual mode_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionrisklimit-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionrisklimit-parameters)

| Name       | In    | Type   | Required | Description          |
| ---------- | ----- | ------ | -------- | -------------------- |
| settle     | path  | string | true     | Settle currency      |
| contract   | path  | string | true     | Futures contract     |
| risk_limit | query | string | true     | New risk limit value |

#### [#](#enumerated-values-57) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user": 10000,
    "contract": "BTC_USDT",
    "size": -9440,
    "leverage": "0",
    "risk_limit": "100",
    "leverage_max": "100",
    "maintenance_rate": "0.005",
    "value": "3568.62",
    "margin": "4.431548146258",
    "entry_price": "3779.55",
    "liq_price": "99999999",
    "mark_price": "3780.32",
    "unrealised_pnl": "-0.000507486844",
    "realised_pnl": "0.045543982432",
    "pnl_pnl": "0.045543982432",
    "pnl_fund": "0",
    "pnl_fee": "0",
    "history_pnl": "0",
    "last_close_pnl": "0",
    "realised_point": "0",
    "history_point": "0",
    "adl_ranking": 5,
    "pending_orders": 16,
    "close_order": {
      "id": 232323,
      "price": "3779",
      "is_liq": false
    },
    "mode": "single",
    "update_time": 1684994406,
    "update_id": 1,
    "cross_leverage_limit": "0",
    "risk_limit_table": "BIG_HOT_COIN_50X_V2",
    "average_maintenance_rate": "0.005"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionrisklimit-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedualmodepositionrisklimit-responses)

| Status | Meaning                                                                    | Description      | Schema                          |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#place-futures-order) Place futures order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-futures-order](https://www.gate.io/docs/developers/apiv4/en/#place-futures-order)

> Code samples

`POST /futures/{settle}/orders`

_Place futures order_

- When placing an order, the number of contracts is specified `size`, not the
  number of coins. The number of coins corresponding to each contract is
  returned in the contract details interface `quanto_multiplier`
- 0 The order that was completed cannot be obtained after 10 minutes of
  withdrawal, and the order will be mentioned that the order does not exist
- Setting `reduce_only` to `true` can prevent the position from being penetrated
  when reducing the position
- In single-position mode, if you need to close the position, you need to set
  `size` to 0 and `close` to `true`
- In dual warehouse mode,
- Reduce position: reduce_only=true, size is a positive number that indicates
  short position, negative number that indicates long position
- Add number that indicates adding long positions, and negative numbers indicate
  adding short positions
- Close position: size=0, set the direction of closing position according to
  auto_size, and set `reduce_only` to true at the same time - reduce_only: Make
  sure to only perform position reduction operations to prevent increased
  positions
- Set `stp_act` to determine the use of a strategy that restricts user
  transactions. For detailed usage, refer to the body parameter `stp_act`

> Body parameter

```
{
  "contract": "BTC_USDT",
  "size": 6024,
  "iceberg": 0,
  "price": "3765",
  "tif": "gtc",
  "text": "t-my-custom-id",
  "stp_act": "-"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createfuturesorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createfuturesorder-parameters)

| Name           | In     | Type                                | Required | Description                                                                                                                                       |
| -------------- | ------ | ----------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| x-gate-exptime | header | string                              | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected  |
| body           | body   | [FuturesOrder](#schemafuturesorder) | true     | none                                                                                                                                              |
| » contract     | body   | string                              | true     | Futures contract                                                                                                                                  |
| » size         | body   | integer(int64)                      | true     | Required. Trading quantity. Positive for buy, negative for sell. Set to 0 for close position orders.                                              |
| » iceberg      | body   | integer(int64)                      | false    | Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden portions are charged taker fees.                                      |
| » price        | body   | string                              | false    | Order price. Price of 0 with `tif` set to `ioc` represents a market order.                                                                        |
| » close        | body   | boolean                             | false    | Set as `true` to close the position, with `size` set to 0                                                                                         |
| » reduce_only  | body   | boolean                             | false    | Set as `true` to be reduce-only order                                                                                                             |
| » tif          | body   | string                              | false    | Time in force                                                                                                                                     |
| » text         | body   | string                              | false    | Custom order information. If not empty, must follow the rules below:                                                                              |
| » auto_size    | body   | string                              | false    | Set side to close dual-mode position. `close_long` closes the long side; while `close_short` the short one. Note `size` also needs to be set to 0 |
| » stp_act      | body   | string                              | false    | Self-Trading Prevention Action. Users can use this field to set self-trade prevention strategies                                                  |

|» limit_vip|body|integer(int64)|false|限价委托成交的对手单User
VIP 等级，当前下单仅会跟小于等于对手单User
VIP 等级的单成交，仅支持传递11~16，默认是0| |»
pid|body|integer(int64)|false|仓位ID| |settle|path|string|true|Settle currency|

#### [#](#detailed-descriptions-27) Detailed descriptions

**» tif**: Time in force

- gtc: GoodTillCancelled
- ioc: ImmediateOrCancelled, taker only
- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
  fee
- fok: FillOrKill, fill either completely or none

**» text**: Custom order information. If not empty, must follow the rules below:

1.  Prefixed with `t-`
2.  No longer than 28 bytes without `t-` prefix
3.  Can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

In addition to user-defined information, the following are internal reserved
fields that identify the order source:

- web: Web
- api: API call
- app: Mobile app
- auto_deleveraging: Automatic deleveraging
- liquidation: Forced liquidation of positions under the old classic mode
- liq-xxx: a. Forced liquidation of positions under the new classic mode,
  including isolated margin, one-way cross margin, and non-hedged positions
  under two-way cross margin. b. Forced liquidation of isolated positions under
  the unified account single-currency margin mode
- hedge-liq-xxx: Forced liquidation of hedged positions under the new classic
  mode two-way cross margin, i.e., simultaneously closing long and short
  positions
- pm_liquidate: Forced liquidation under unified account multi-currency margin
  mode
- comb_margin_liquidate: Forced liquidation under unified account portfolio
  margin mode
- scm_liquidate: Forced liquidation of positions under unified account
  single-currency margin mode
- insurance: Insurance

**» stp_act**: Self-Trading Prevention Action. Users can use this field to set
self-trade prevention strategies

1.  After users join the `STP Group`, they can pass `stp_act` to limit the
    user's self-trade prevention strategy. If `stp_act` is not passed, the
    default is `cn` strategy.
2.  When the user does not join the `STP group`, an error will be returned when
    passing the `stp_act` parameter.
3.  If the user did not use `stp_act` when placing the order, `stp_act` will
    return '-'

- cn: Cancel newest, cancel new orders and keep old ones
- co: Cancel oldest, cancel old orders and keep new ones
- cb: Cancel both, both old and new orders will be cancelled

#### [#](#enumerated-values-58) Enumerated Values

| Parameter   | Value       |
| ----------- | ----------- |
| » tif       | gtc         |
| » tif       | ioc         |
| » tif       | poc         |
| » tif       | fok         |
| » auto_size | close_long  |
| » auto_size | close_short |
| » stp_act   | co          |
| » stp_act   | cn          |
| » stp_act   | cb          |
| » stp_act   | \-          |
| settle      | btc         |
| settle      | usdt        |

> Example responses

> 201 Response

```
{
  "id": 15675394,
  "user": 100000,
  "contract": "BTC_USDT",
  "create_time": 1546569968,
  "size": 6024,
  "iceberg": 0,
  "left": 6024,
  "price": "3765",
  "fill_price": "0",
  "mkfr": "-0.00025",
  "tkfr": "0.00075",
  "tif": "gtc",
  "refu": 0,
  "is_reduce_only": false,
  "is_close": false,
  "is_liq": false,
  "text": "t-my-custom-id",
  "status": "finished",
  "finish_time": 1514764900,
  "finish_as": "cancelled",
  "stp_id": 0,
  "stp_act": "-",
  "amend_text": "-"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createfuturesorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createfuturesorder-responses)

| Status | Meaning                                                                         | Description   | Schema                              |
| ------ | ------------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-futures-order-list) Query futures order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list)

> Code samples

`GET /futures/{settle}/orders`

_Query futures order list_

- Zero-fill order cannot be retrieved for 10 minutes after cancellation
- Historical orders, by default, only data within the past 6 months is
  supported. If you need to query data for a longer period, please use
  `GET /futures/{settle}/orders_timerange`.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listfuturesorders-parameters)

| Name     | In    | Type    | Required | Description                                                                                    |
| -------- | ----- | ------- | -------- | ---------------------------------------------------------------------------------------------- |
| contract | query | string  | false    | Futures contract, return related data only if specified                                        |
| status   | query | string  | true     | Query order list based on status                                                               |
| limit    | query | integer | false    | Maximum number of records returned in a single list                                            |
| offset   | query | integer | false    | List offset, starting from 0                                                                   |
| last_id  | query | string  | false    | Specify the currency name to query in batches, and support up to 100 pass parameters at a time |
| settle   | path  | string  | true     | Settle currency                                                                                |

#### [#](#enumerated-values-59) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "id": 15675394,
    "user": 100000,
    "contract": "BTC_USDT",
    "create_time": 1546569968,
    "size": 6024,
    "iceberg": 0,
    "left": 6024,
    "price": "3765",
    "fill_price": "0",
    "mkfr": "-0.00025",
    "tkfr": "0.00075",
    "tif": "gtc",
    "refu": 0,
    "is_reduce_only": false,
    "is_close": false,
    "is_liq": false,
    "text": "t-my-custom-id",
    "status": "finished",
    "finish_time": 1514764900,
    "finish_as": "cancelled",
    "stp_id": 0,
    "stp_act": "-",
    "amend_text": "-"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listfuturesorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listfuturesorders-responses)

| Status | Meaning                                                                    | Description                 | Schema                                  |
| ------ | -------------------------------------------------------------------------- | --------------------------- | --------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[FuturesOrder](#schemafuturesorder)\] |

### [#](#response-headers) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers](https://www.gate.io/docs/developers/apiv4/en/#response-headers)

| Status | Header              | Type    | Format | Description                     |
| ------ | ------------------- | ------- | ------ | ------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination  |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-orders-with-open-status) Cancel all orders with 'open' status

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status)

> Code samples

`DELETE /futures/{settle}/orders`

_Cancel all orders with 'open' status_

Zero-fill orders cannot be retrieved 10 minutes after order cancellation

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorders-parameters)

| Name                | In     | Type    | Required | Description                                                                                                                                              |
| ------------------- | ------ | ------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| x-gate-exptime      | header | string  | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected         |
| contract            | query  | string  | true     | Futures contract                                                                                                                                         |
| side                | query  | string  | false    | Specify all buy orders or all sell orders, both are included if not specified. Set to bid to cancel all buy orders, set to ask to cancel all sell orders |
| exclude_reduce_only | query  | boolean | false    | 是否排除仅减仓Order                                                                                                                                      |
| text                | query  | string  | false    | 取消Order的Note信息                                                                                                                                      |
| settle              | path   | string  | true     | Settle currency                                                                                                                                          |

#### [#](#enumerated-values-60) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "id": 15675394,
    "user": 100000,
    "contract": "BTC_USDT",
    "create_time": 1546569968,
    "size": 6024,
    "iceberg": 0,
    "left": 6024,
    "price": "3765",
    "fill_price": "0",
    "mkfr": "-0.00025",
    "tkfr": "0.00075",
    "tif": "gtc",
    "refu": 0,
    "is_reduce_only": false,
    "is_close": false,
    "is_liq": false,
    "text": "t-my-custom-id",
    "status": "finished",
    "finish_time": 1514764900,
    "finish_as": "cancelled",
    "stp_id": 0,
    "stp_act": "-",
    "amend_text": "-"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorders-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorders-responses)

| Status | Meaning                                                                    | Description                   | Schema                                  |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | --------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation successful | \[[FuturesOrder](#schemafuturesorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-futures-order-list-by-time-range) Query futures order list by time range

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list-by-time-range](https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list-by-time-range)

> Code samples

`GET /futures/{settle}/orders_timerange`

_Query futures order list by time range_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getorderswithtimerange-parameters](https://www.gate.io/docs/developers/apiv4/en/#getorderswithtimerange-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |

#### [#](#detailed-descriptions-28) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-61) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "id": 15675394,
    "user": 100000,
    "contract": "BTC_USDT",
    "create_time": 1546569968,
    "size": 6024,
    "iceberg": 0,
    "left": 6024,
    "price": "3765",
    "fill_price": "0",
    "mkfr": "-0.00025",
    "tkfr": "0.00075",
    "tif": "gtc",
    "refu": 0,
    "is_reduce_only": false,
    "is_close": false,
    "is_liq": false,
    "text": "t-my-custom-id",
    "status": "finished",
    "finish_time": 1514764900,
    "finish_as": "cancelled",
    "stp_id": 0,
    "stp_act": "-",
    "amend_text": "-"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getorderswithtimerange-responses](https://www.gate.io/docs/developers/apiv4/en/#getorderswithtimerange-responses)

| Status | Meaning                                                                    | Description                 | Schema                                  |
| ------ | -------------------------------------------------------------------------- | --------------------------- | --------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[FuturesOrder](#schemafuturesorder)\] |

### [#](#response-headers-2) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers-2](https://www.gate.io/docs/developers/apiv4/en/#response-headers-2)

| Status | Header              | Type    | Format | Description                     |
| ------ | ------------------- | ------- | ------ | ------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination  |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#place-batch-futures-orders) Place batch futures orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-batch-futures-orders](https://www.gate.io/docs/developers/apiv4/en/#place-batch-futures-orders)

> Code samples

`POST /futures/{settle}/batch_orders`

_Place batch futures orders_

- Up to 10 orders per request
- If any of the order's parameters are missing or in the wrong format, all of
  them will not be executed, and a http status 400 error will be returned
  directly
- If the parameters are checked and passed, all are executed. Even if there is a
  business logic error in the middle (such as insufficient funds), it will not
  affect other execution orders
- The returned result is in array format, and the order corresponds to the
  orders in the request body
- In the returned result, the `succeeded` field of type bool indicates whether
  the execution was successful or not
- If the execution is successful, the normal order content is included; if the
  execution fails, the `label` field is included to indicate the cause of the
  error
- In the rate limiting, each order is counted individually

> Body parameter

```
[
  {
    "contract": "BTC_USDT",
    "size": 6024,
    "iceberg": 0,
    "price": "3765",
    "tif": "gtc",
    "text": "t-my-custom-id",
    "stp_act": "-"
  }
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-parameters)

| Name           | In     | Type                                         | Required | Description                                                                                                                                      |
| -------------- | ------ | -------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string                                       | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array\[[FuturesOrder](#schemafuturesorder)\] | true     | none                                                                                                                                             |
| settle         | path   | string                                       | true     | Settle currency                                                                                                                                  |

#### [#](#enumerated-values-62) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "succeeded": true,
    "id": 15675394,
    "user": 100000,
    "contract": "BTC_USDT",
    "create_time": 1546569968,
    "size": 6024,
    "iceberg": 0,
    "left": 6024,
    "price": "3765",
    "fill_price": "0",
    "mkfr": "-0.00025",
    "tkfr": "0.00075",
    "tif": "gtc",
    "refu": 0,
    "is_reduce_only": false,
    "is_close": false,
    "is_liq": false,
    "text": "t-my-custom-id",
    "status": "finished",
    "finish_time": 1514764900,
    "finish_as": "cancelled",
    "stp_id": 0,
    "stp_act": "-",
    "amend_text": "-"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Request execution completed | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createbatchfuturesorder-responseschema)

Status Code **200**

| Name           | Type           | Description                                                                 |
| -------------- | -------------- | --------------------------------------------------------------------------- |
| _None_         | array          | \[Futures order details\]                                                   |
| » _None_       | object         | Futures order details                                                       |
| »» succeeded   | boolean        | Request execution result                                                    |
| »» label       | string         | Error label, only exists if execution fails                                 |
| »» detail      | string         | Error detail, only present if execution failed and details need to be given |
| »» id          | integer(int64) | Futures order ID                                                            |
| »» user        | integer        | User ID                                                                     |
| »» create_time | number(double) | Creation time of order                                                      |
| »» finish_time | number(double) | Order finished time. Not returned if order is open                          |
| »» finish_as   | string         | How the order was finished:                                                 |

\- filled: all filled  
\- cancelled: manually cancelled  
\- liquidated: cancelled because of liquidation  
\- ioc: time in force is `IOC`, finish immediately  
\- auto_deleveraged: finished by ADL  
\- reduce_only: cancelled because of increasing position while `reduce-only`
set  
\- position_closed: cancelled because the position was closed  
\- reduce_out: only reduce positions by excluding hard-to-fill orders  
\- stp: cancelled because self trade prevention | | »» status | string | Order
status

\- `open`: Pending  
\- `finished`: Completed | | »» contract | string | Futures contract | | »» size
| integer(int64) | Required. Trading quantity. Positive for buy, negative for
sell. Set to 0 for close position orders. | | »» iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | »» price | string | Order price. Price of 0
with `tif` set to `ioc` represents a market order. | | »» is_close | boolean |
Is the order to close position | | »» is_reduce_only | boolean | Is the order
reduce-only | | »» is_liq | boolean | Is the order for liquidation | | »» tif |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none | | »» left | integer(int64)
| Unfilled quantity | | »» fill_price | string | Fill price | | »» text | string
| User defined information. If not empty, must follow the rules below:

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)  
Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- web: from web  
\- api: from API  
\- app: from mobile phones  
\- auto_deleveraging: from ADL  
\- liquidation: from liquidation  
\- insurance: from insurance | | »» tkfr | string | Taker fee | | »» mkfr |
string | Maker fee | | »» refu | integer | Referrer user ID | | »» stp_act |
string | Self-Trading Prevention Action. Users can use this field to set
self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | »» stp_id |
integer | Orders between users in the same `stp_id` group are not allowed to be
self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` |

#### [#](#enumerated-values-63) Enumerated Values

| Property  | Value            |
| --------- | ---------------- |
| finish_as | filled           |
| finish_as | cancelled        |
| finish_as | liquidated       |
| finish_as | ioc              |
| finish_as | auto_deleveraged |
| finish_as | reduce_only      |
| finish_as | position_closed  |
| finish_as | reduce_out       |
| finish_as | stp              |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |
| tif       | fok              |
| stp_act   | co               |
| stp_act   | cn               |
| stp_act   | cb               |
| stp_act   | \-               |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-order-details-2) Query single order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-2](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-2)

> Code samples

`GET /futures/{settle}/orders/{order_id}`

_Query single order details_

- Zero-fill order cannot be retrieved for 10 minutes after cancellation
- Historical orders, by default, only data within the past 6 months is
  supported.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getfuturesorder-parameters)

| Name     | In   | Type   | Required | Description                                               |
| -------- | ---- | ------ | -------- | --------------------------------------------------------- |
| settle   | path | string | true     | Settle currency                                           |
| order_id | path | string | true     | Order ID returned, or user custom ID(i.e., `text` field). |

#### [#](#detailed-descriptions-29) Detailed descriptions

**order_id**: Order ID returned, or user custom ID(i.e., `text` field).
Operations based on custom ID can only be checked when the order is in
orderbook. finished, it can be checked within 60 seconds after the end of the
order. After that, only order ID is accepted.

#### [#](#enumerated-values-64) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "id": 15675394,
  "user": 100000,
  "contract": "BTC_USDT",
  "create_time": 1546569968,
  "size": 6024,
  "iceberg": 0,
  "left": 6024,
  "price": "3765",
  "fill_price": "0",
  "mkfr": "-0.00025",
  "tkfr": "0.00075",
  "tif": "gtc",
  "refu": 0,
  "is_reduce_only": false,
  "is_close": false,
  "is_liq": false,
  "text": "t-my-custom-id",
  "status": "finished",
  "finish_time": 1514764900,
  "finish_as": "cancelled",
  "stp_id": 0,
  "stp_act": "-",
  "amend_text": "-"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getfuturesorder-responses)

| Status | Meaning                                                                    | Description   | Schema                              |
| ------ | -------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-order-2) Cancel single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-2](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-2)

> Code samples

`DELETE /futures/{settle}/orders/{order_id}`

_Cancel single order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorder-parameters)

| Name           | In     | Type   | Required | Description                                                                                                                                      |
| -------------- | ------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| settle         | path   | string | true     | Settle currency                                                                                                                                  |
| order_id       | path   | string | true     | Order ID returned, or user custom ID(i.e., `text` field).                                                                                        |

#### [#](#detailed-descriptions-30) Detailed descriptions

**order_id**: Order ID returned, or user custom ID(i.e., `text` field).
Operations based on custom ID can only be checked when the order is in
orderbook. finished, it can be checked within 60 seconds after the end of the
order. After that, only order ID is accepted.

#### [#](#enumerated-values-65) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "id": 15675394,
  "user": 100000,
  "contract": "BTC_USDT",
  "create_time": 1546569968,
  "size": 6024,
  "iceberg": 0,
  "left": 6024,
  "price": "3765",
  "fill_price": "0",
  "mkfr": "-0.00025",
  "tkfr": "0.00075",
  "tif": "gtc",
  "refu": 0,
  "is_reduce_only": false,
  "is_close": false,
  "is_liq": false,
  "text": "t-my-custom-id",
  "status": "finished",
  "finish_time": 1514764900,
  "finish_as": "cancelled",
  "stp_id": 0,
  "stp_act": "-",
  "amend_text": "-"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorder-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelfuturesorder-responses)

| Status | Meaning                                                                    | Description   | Schema                              |
| ------ | -------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#amend-single-order-2) Amend single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amend-single-order-2](https://www.gate.io/docs/developers/apiv4/en/#amend-single-order-2)

> Code samples

`PUT /futures/{settle}/orders/{order_id}`

_Amend single order_

> Body parameter

```
{
  "size": 100,
  "price": "54321"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendfuturesorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#amendfuturesorder-parameters)

| Name           | In     | Type           | Required | Description                                                                                                                                      |
| -------------- | ------ | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string         | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | object         | true     | none                                                                                                                                             |
| » size         | body   | integer(int64) | false    | New order size, including filled part.                                                                                                           |
| » price        | body   | string         | false    | New order price                                                                                                                                  |
| » amend_text   | body   | string         | false    | Custom info during order amendment                                                                                                               |
| » text         | body   | string         | false    | 内部User可以在text修改信息。                                                                                                                     |
| settle         | path   | string         | true     | Settle currency                                                                                                                                  |
| order_id       | path   | string         | true     | Order ID returned, or user custom ID(i.e., `text` field).                                                                                        |

#### [#](#detailed-descriptions-31) Detailed descriptions

**» size**: New order size, including filled part.

- If new size is less than or equal to filled size, the order will be cancelled.
- Order side must be identical to the original one.
- Close order size cannot be changed.
- For reduce only orders, increasing size may leads to other reduce only orders
  being cancelled.
- If price is not changed, decreasing size will not change its precedence in
  order book, while increasing will move it to the last at current price.

**order_id**: Order ID returned, or user custom ID(i.e., `text` field).
Operations based on custom ID can only be checked when the order is in
orderbook. finished, it can be checked within 60 seconds after the end of the
order. After that, only order ID is accepted.

#### [#](#enumerated-values-66) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "id": 15675394,
  "user": 100000,
  "contract": "BTC_USDT",
  "create_time": 1546569968,
  "size": 6024,
  "iceberg": 0,
  "left": 6024,
  "price": "3765",
  "fill_price": "0",
  "mkfr": "-0.00025",
  "tkfr": "0.00075",
  "tif": "gtc",
  "refu": 0,
  "is_reduce_only": false,
  "is_close": false,
  "is_liq": false,
  "text": "t-my-custom-id",
  "status": "finished",
  "finish_time": 1514764900,
  "finish_as": "cancelled",
  "stp_id": 0,
  "stp_act": "-",
  "amend_text": "-"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendfuturesorder-responses](https://www.gate.io/docs/developers/apiv4/en/#amendfuturesorder-responses)

| Status | Meaning                                                                    | Description   | Schema                              |
| ------ | -------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-records-2) Query personal trading records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-2](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-2)

> Code samples

`GET /futures/{settle}/my_trades`

_Query personal trading records_

By default, only data within the past 6 months is supported. If you need to
query data for a longer period, please use
`GET /futures/{settle}/my_trades_timerange`.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#getmytrades-parameters)

| Name     | In    | Type           | Required | Description                                                                 |
| -------- | ----- | -------------- | -------- | --------------------------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                                             |
| contract | query | string         | false    | Futures contract, return related data only if specified                     |
| order    | query | integer(int64) | false    | Futures order ID, return related data only if specified                     |
| limit    | query | integer        | false    | Maximum number of records returned in a single list                         |
| offset   | query | integer        | false    | List offset, starting from 0                                                |
| last_id  | query | string         | false    | Specify the starting point for this list based on a previously retrieved id |

#### [#](#detailed-descriptions-32) Detailed descriptions

**last_id**: Specify the starting point for this list based on a previously
retrieved id

This parameter is deprecated. If you need to iterate through and retrieve more
records, we recommend using 'GET /futures/{settle}/my_trades_timerange'.

#### [#](#enumerated-values-67) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "id": 121234231,
    "create_time": 1514764800.123,
    "contract": "BTC_USDT",
    "order_id": "21893289839",
    "size": 100,
    "price": "100.123",
    "text": "t-123456",
    "fee": "0.01",
    "point_fee": "0",
    "role": "taker",
    "close_size": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytrades-responses](https://www.gate.io/docs/developers/apiv4/en/#getmytrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmytrades-responseschema)

Status Code **200**

| Name          | Type           | Description                 |
| ------------- | -------------- | --------------------------- |
| _None_        | array          | none                        |
| » id          | integer(int64) | Fill ID                     |
| » create_time | number(double) | Fill Time                   |
| » contract    | string         | Futures contract            |
| » order_id    | string         | Related order ID            |
| » size        | integer(int64) | Trading size                |
| » close_size  | integer(int64) | Number of closed positions: |

close_size=0 && size＞0 Open long position  
close_size=0 && size＜0 Open short position  
close_size>0 && size>0 && size <= close_size Close short position  
close_size>0 && size>0 && size > close_size Close short position and open long
position  
close_size<0 && size<0 && size >= close_size Close long position  
close_size<0 && size<0 && size < close_size Close long position and open short
position | | » price | string | Fill Price | | » role | string | Trade role.
taker - taker, maker - maker | | » text | string | Order custom information | |
» fee | string | Trade fee | | » point_fee | string | Points used to deduct
trade fee |

#### [#](#enumerated-values-68) Enumerated Values

| Property | Value |
| -------- | ----- |
| role     | taker |
| role     | maker |

### [#](#response-headers-3) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers-3](https://www.gate.io/docs/developers/apiv4/en/#response-headers-3)

| Status | Header              | Type    | Format | Description                     |
| ------ | ------------------- | ------- | ------ | ------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination  |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-records-by-time-range) Query personal trading records by time range

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-by-time-range](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-by-time-range)

> Code samples

`GET /futures/{settle}/my_trades_timerange`

_Query personal trading records by time range_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-parameters](https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |
| role     | query | string         | false    | Query role, maker or taker                              |

#### [#](#detailed-descriptions-33) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-69) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "trade_id": "121234231",
    "create_time": 1514764800.123,
    "contract": "BTC_USDT",
    "order_id": "21893289839",
    "size": 100,
    "price": "100.123",
    "text": "t-123456",
    "fee": "0.01",
    "point_fee": "0",
    "role": "taker",
    "close_size": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-responses](https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmytradeswithtimerange-responseschema)

Status Code **200**

| Name          | Type           | Description                 |
| ------------- | -------------- | --------------------------- |
| » trade_id    | string         | Fill ID                     |
| » create_time | number(double) | Fill Time                   |
| » contract    | string         | Futures contract            |
| » order_id    | string         | Related order ID            |
| » size        | integer(int64) | Trading size                |
| » close_size  | integer(int64) | Number of closed positions: |

close_size=0 && size＞0 Open long position  
close_size=0 && size＜0 Open short position  
close_size>0 && size>0 && size <= close_size Close short position  
close_size>0 && size>0 && size > close_size Close short position and open long
position  
close_size<0 && size<0 && size >= close_size Close long position  
close_size<0 && size<0 && size < close_size Close long position and open short
position | | » price | string | Fill Price | | » role | string | Trade role.
taker - taker, maker - maker | | » text | string | Order custom information | |
» fee | string | Trade fee | | » point_fee | string | Points used to deduct
trade fee |

#### [#](#enumerated-values-70) Enumerated Values

| Property | Value |
| -------- | ----- |
| role     | taker |
| role     | maker |

### [#](#response-headers-4) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers-4](https://www.gate.io/docs/developers/apiv4/en/#response-headers-4)

| Status | Header              | Type    | Format | Description                     |
| ------ | ------------------- | ------- | ------ | ------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination  |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-position-close-history) Query position close history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-position-close-history](https://www.gate.io/docs/developers/apiv4/en/#query-position-close-history)

> Code samples

`GET /futures/{settle}/position_close`

_Query position close history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-parameters](https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| side     | query | string         | false    | Query side. long or shot                                |
| pnl      | query | string         | false    | Query profit or loss                                    |

#### [#](#detailed-descriptions-34) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-71) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1546487347,
    "pnl": "0.00013",
    "pnl_pnl": "0.00011",
    "pnl_fund": "0.00001",
    "pnl_fee": "0.00001",
    "side": "long",
    "contract": "BTC_USDT",
    "text": "web",
    "max_size": "100",
    "accum_size": "100",
    "first_open_time": 1546487347,
    "long_price": "2026.87",
    "short_price": "2544.4"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-responses](https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listpositionclose-responseschema)

Status Code **200**

| Name       | Type           | Description         |
| ---------- | -------------- | ------------------- |
| _None_     | array          | none                |
| » time     | number(double) | Position close time |
| » contract | string         | Futures contract    |
| » side     | string         | Position side       |

\- `long`: Long position  
\- `short`: Short position | | » pnl | string | PnL | | » pnl_pnl | string |
PNL - Position P/L | | » pnl_fund | string | PNL - Funding Fees | | » pnl_fee |
string | PNL - Transaction Fees | | » text | string | Source of close order. See
`order.text` field for specific values | | » max_size | string | Max Trade Size
| | » accum_size | string | Cumulative closed position volume | | »
first_open_time | integer(int64) | First Open Time | | » long_price | string |
When side is 'long', it indicates the opening average price; when side is
'short', it indicates the closing average price | | » short_price | string |
When side is 'long', it indicates the closing average price; when side is
'short', it indicates the opening average price |

#### [#](#enumerated-values-72) Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | long  |
| side     | short |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-liquidation-history) Query liquidation history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-history](https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-history)

> Code samples

`GET /futures/{settle}/liquidates`

_Query liquidation history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidates-parameters](https://www.gate.io/docs/developers/apiv4/en/#listliquidates-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| at       | query | integer        | false    | Specify liquidation timestamp                           |

#### [#](#detailed-descriptions-35) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-73) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1548654951,
    "contract": "BTC_USDT",
    "size": 600,
    "leverage": "25",
    "margin": "0.006705256878",
    "entry_price": "3536.123",
    "liq_price": "3421.54",
    "mark_price": "3420.27",
    "order_id": 317393847,
    "order_price": "3405",
    "fill_price": "3424",
    "left": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidates-responses](https://www.gate.io/docs/developers/apiv4/en/#listliquidates-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listliquidates-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listliquidates-responseschema)

Status Code **200**

| Name          | Type           | Description                                            |
| ------------- | -------------- | ------------------------------------------------------ |
| _None_        | array          | none                                                   |
| » time        | integer(int64) | Liquidation time                                       |
| » contract    | string         | Futures contract                                       |
| » leverage    | string         | Position leverage. Not returned in public endpoints    |
| » size        | integer(int64) | Position size                                          |
| » margin      | string         | Position margin. Not returned in public endpoints      |
| » entry_price | string         | Average entry price. Not returned in public endpoints  |
| » liq_price   | string         | Liquidation price. Not returned in public endpoints    |
| » mark_price  | string         | Mark price. Not returned in public endpoints           |
| » order_id    | integer(int64) | Liquidation order ID. Not returned in public endpoints |
| » order_price | string         | Liquidation order price                                |
| » fill_price  | string         | Liquidation order average taker price                  |
| » left        | integer(int64) | Liquidation order maker size                           |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-adl-auto-deleveraging-order-information) Query ADL auto-deleveraging order information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-adl-auto-deleveraging-order-information](https://www.gate.io/docs/developers/apiv4/en/#query-adl-auto-deleveraging-order-information)

> Code samples

`GET /futures/{settle}/auto_deleverages`

_Query ADL auto-deleveraging order information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-parameters](https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-parameters)

| Name     | In    | Type           | Required | Description                                             |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                         |
| contract | query | string         | false    | Futures contract, return related data only if specified |
| limit    | query | integer        | false    | Maximum number of records returned in a single list     |
| offset   | query | integer        | false    | List offset, starting from 0                            |
| from     | query | integer(int64) | false    | Start timestamp                                         |
| to       | query | integer(int64) | false    | Termination Timestamp                                   |
| at       | query | integer        | false    | Specify auto-deleveraging timestamp                     |

#### [#](#detailed-descriptions-36) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-74) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "time": 1675841679,
    "contract": "ACH_USDT",
    "order_id": 73873128,
    "user": 1666,
    "cross_leverage_limit": "0",
    "leverage": "0",
    "entry_price": "2649.648633636364",
    "fill_price": "2790.8082",
    "position_size": 1,
    "trade_size": -10
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-responses](https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listautodeleverages-responseschema)

Status Code **200**

| Name                   | Type           | Description                                             |
| ---------------------- | -------------- | ------------------------------------------------------- |
| » time                 | integer(int64) | Automatic deleveraging time                             |
| » user                 | integer(int64) | User ID                                                 |
| » order_id             | integer(int64) | Order ID. Order IDs before 2023-02-20 are null          |
| » contract             | string         | Futures contract                                        |
| » leverage             | string         | Position leverage                                       |
| » cross_leverage_limit | string         | Cross margin leverage (valid only when `leverage` is 0) |
| » entry_price          | string         | Average entry price                                     |
| » fill_price           | string         | Average fill price                                      |
| » trade_size           | integer(int64) | Trading size                                            |
| » position_size        | integer(int64) | Positions after auto-deleveraging                       |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#countdown-cancel-orders-2) Countdown cancel orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders-2](https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders-2)

> Code samples

`POST /futures/{settle}/countdown_cancel_all`

_Countdown cancel orders_

Heartbeat detection for contract orders: When the user-set `timeout` time is
reached, if neither the existing countdown is canceled nor a new countdown is
set, the relevant contract orders will be automatically canceled. This API can
be called repeatedly to or cancel the countdown. Usage example: Repeatedly call
this API at 30-second intervals, setting the `timeout` to 30 (seconds) each
time. If this API is not called again within 30 seconds, all open orders on your
specified `market` will be automatically canceled. If the `timeout` is set to 0
within 30 seconds, the countdown timer will terminate, and the automatic order
cancellation function will be disabled.

> Body parameter

```
{
  "timeout": 30,
  "contract": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-parameters](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-parameters)

| Name       | In   | Type           | Required | Description               |
| ---------- | ---- | -------------- | -------- | ------------------------- |
| body       | body | object         | true     | none                      |
| » timeout  | body | integer(int32) | true     | Countdown time in seconds |
| » contract | body | string         | false    | Futures contract          |
| settle     | path | string         | true     | Settle currency           |

#### [#](#detailed-descriptions-37) Detailed descriptions

**» timeout**: Countdown time in seconds At least 5 seconds, 0 means cancel
countdown

#### [#](#enumerated-values-75) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "triggerTime": "1660039145000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-responses](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-responses)

| Status | Meaning                                                                    | Description                | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Countdown set successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-responseschema](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallfutures-responseschema)

Status Code **200**

_triggerTime_

| Name          | Type           | Description                                    |
| ------------- | -------------- | ---------------------------------------------- |
| » triggerTime | integer(int64) | Timestamp when countdown ends, in milliseconds |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-futures-market-trading-fee-rates) Query futures market trading fee rates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-trading-fee-rates](https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-trading-fee-rates)

> Code samples

`GET /futures/{settle}/fee`

_Query futures market trading fee rates_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-parameters](https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-parameters)

| Name     | In    | Type   | Required | Description                                             |
| -------- | ----- | ------ | -------- | ------------------------------------------------------- |
| settle   | path  | string | true     | Settle currency                                         |
| contract | query | string | false    | Futures contract, return related data only if specified |

#### [#](#enumerated-values-76) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "1INCH_USDT": {
    "taker_fee": "0.00025",
    "maker_fee": "-0.00010"
  },
  "AAVE_USDT": {
    "taker_fee": "0.00025",
    "maker_fee": "-0.00010"
  }
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-responses](https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getfuturesfee-responseschema)

Status Code **200**

_FuturesFee_

| Name                       | Type   | Description                                                                                          |
| -------------------------- | ------ | ---------------------------------------------------------------------------------------------------- |
| » **additionalProperties** | object | The returned result is a map type, where the key represents the market and taker and maker fee rates |
| »» taker_fee               | string | Taker fee                                                                                            |
| »» maker_fee               | string | maker fee                                                                                            |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-batch-orders-by-specified-id-list-2) Cancel batch orders by specified ID list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-batch-orders-by-specified-id-list-2](https://www.gate.io/docs/developers/apiv4/en/#cancel-batch-orders-by-specified-id-list-2)

> Code samples

`POST /futures/{settle}/batch_cancel_orders`

_Cancel batch orders by specified ID list_

Multiple different order IDs can be specified, maximum 20 records per request

> Body parameter

```
[
  "1",
  "2",
  "3"
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-parameters)

| Name           | In     | Type            | Required | Description                                                                                                                                      |
| -------------- | ------ | --------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string          | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array\[string\] | true     | none                                                                                                                                             |
| settle         | path   | string          | true     | Settle currency                                                                                                                                  |

#### [#](#enumerated-values-77) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "user_id": 111,
    "id": "123456",
    "succeeded": true,
    "message": ""
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-responses)

| Status | Meaning                                                                    | Description                            | Schema     |
| ------ | -------------------------------------------------------------------------- | -------------------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order cancellation operation completed | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchfutureorders-responseschema)

Status Code **200**

| Name                      | Type           | Description                                                    |
| ------------------------- | -------------- | -------------------------------------------------------------- |
| » FutureCancelOrderResult | object         | Order cancellation result                                      |
| »» id                     | string         | Order ID                                                       |
| »» user_id                | integer(int64) | User ID                                                        |
| »» succeeded              | boolean        | Whether cancellation succeeded                                 |
| »» message                | string         | Error description when cancellation fails, empty if successful |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-modify-orders-by-specified-ids) Batch modify orders by specified IDs

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-modify-orders-by-specified-ids](https://www.gate.io/docs/developers/apiv4/en/#batch-modify-orders-by-specified-ids)

> Code samples

`POST /futures/{settle}/batch_amend_orders`

_Batch modify orders by specified IDs_

Multiple different order IDs can be specified, maximum 10 orders per request

> Body parameter

```
[
  {
    "order_id": 121212,
    "amend_text": "batch amend text",
    "size": 100,
    "price": "54321"
  }
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-parameters)

| Name           | In     | Type                                                     | Required | Description                                                                                                                                      |
| -------------- | ------ | -------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string                                                   | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array\[[BatchAmendOrderReq](#schemabatchamendorderreq)\] | true     | none                                                                                                                                             |
| settle         | path   | string                                                   | true     | Settle currency                                                                                                                                  |

#### [#](#enumerated-values-78) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "succeeded": true,
    "id": 15675394,
    "user": 100000,
    "contract": "BTC_USDT",
    "create_time": 1546569968,
    "size": 6024,
    "iceberg": 0,
    "left": 6024,
    "price": "3765",
    "fill_price": "0",
    "mkfr": "-0.00025",
    "tkfr": "0.00075",
    "tif": "gtc",
    "refu": 0,
    "is_reduce_only": false,
    "is_close": false,
    "is_liq": false,
    "text": "t-my-custom-id",
    "status": "finished",
    "finish_time": 1514764900,
    "finish_as": "cancelled",
    "stp_id": 0,
    "stp_act": "-",
    "amend_text": "-"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-responses](https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Request execution completed | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#amendbatchfutureorders-responseschema)

Status Code **200**

| Name           | Type           | Description                                                                 |
| -------------- | -------------- | --------------------------------------------------------------------------- |
| _None_         | array          | \[Futures order details\]                                                   |
| » _None_       | object         | Futures order details                                                       |
| »» succeeded   | boolean        | Request execution result                                                    |
| »» label       | string         | Error label, only exists if execution fails                                 |
| »» detail      | string         | Error detail, only present if execution failed and details need to be given |
| »» id          | integer(int64) | Futures order ID                                                            |
| »» user        | integer        | User ID                                                                     |
| »» create_time | number(double) | Creation time of order                                                      |
| »» finish_time | number(double) | Order finished time. Not returned if order is open                          |
| »» finish_as   | string         | How the order was finished:                                                 |

\- filled: all filled  
\- cancelled: manually cancelled  
\- liquidated: cancelled because of liquidation  
\- ioc: time in force is `IOC`, finish immediately  
\- auto_deleveraged: finished by ADL  
\- reduce_only: cancelled because of increasing position while `reduce-only`
set  
\- position_closed: cancelled because the position was closed  
\- reduce_out: only reduce positions by excluding hard-to-fill orders  
\- stp: cancelled because self trade prevention | | »» status | string | Order
status

\- `open`: Pending  
\- `finished`: Completed | | »» contract | string | Futures contract | | »» size
| integer(int64) | Required. Trading quantity. Positive for buy, negative for
sell. Set to 0 for close position orders. | | »» iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | »» price | string | Order price. Price of 0
with `tif` set to `ioc` represents a market order. | | »» is_close | boolean |
Is the order to close position | | »» is_reduce_only | boolean | Is the order
reduce-only | | »» is_liq | boolean | Is the order for liquidation | | »» tif |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none | | »» left | integer(int64)
| Unfilled quantity | | »» fill_price | string | Fill price | | »» text | string
| User defined information. If not empty, must follow the rules below:

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)  
Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- web: from web  
\- api: from API  
\- app: from mobile phones  
\- auto_deleveraging: from ADL  
\- liquidation: from liquidation  
\- insurance: from insurance | | »» tkfr | string | Taker fee | | »» mkfr |
string | Maker fee | | »» refu | integer | Referrer user ID | | »» stp_act |
string | Self-Trading Prevention Action. Users can use this field to set
self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | »» stp_id |
integer | Orders between users in the same `stp_id` group are not allowed to be
self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` |

#### [#](#enumerated-values-79) Enumerated Values

| Property  | Value            |
| --------- | ---------------- |
| finish_as | filled           |
| finish_as | cancelled        |
| finish_as | liquidated       |
| finish_as | ioc              |
| finish_as | auto_deleveraged |
| finish_as | reduce_only      |
| finish_as | position_closed  |
| finish_as | reduce_out       |
| finish_as | stp              |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |
| tif       | fok              |
| stp_act   | co               |
| stp_act   | cn               |
| stp_act   | cb               |
| stp_act   | \-               |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-risk-limit-table-by-table-id) Query risk limit table by table_id

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-table-by-table-id](https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-table-by-table-id)

> Code samples

`GET /futures/{settle}/risk_limit_table`

_Query risk limit table by table_id_

Just pass table_id

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-parameters](https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-parameters)

| Name     | In    | Type   | Required | Description         |
| -------- | ----- | ------ | -------- | ------------------- |
| settle   | path  | string | true     | Settle currency     |
| table_id | query | string | true     | Risk limit table ID |

#### [#](#enumerated-values-80) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "tier": 1,
    "risk_limit": "10000",
    "initial_rate": "0.025",
    "maintenance_rate": "0.015",
    "leverage_max": "40",
    "deduction": "0"
  },
  {
    "tier": 2,
    "risk_limit": "30000",
    "initial_rate": "0.03333",
    "maintenance_rate": "0.02",
    "leverage_max": "30",
    "deduction": "50"
  },
  {
    "tier": 3,
    "risk_limit": "50000",
    "initial_rate": "0.04545",
    "maintenance_rate": "0.03",
    "leverage_max": "22",
    "deduction": "350"
  },
  {
    "tier": 4,
    "risk_limit": "70000",
    "initial_rate": "0.05555",
    "maintenance_rate": "0.04",
    "leverage_max": "18",
    "deduction": "850"
  },
  {
    "tier": 5,
    "risk_limit": "100000",
    "initial_rate": "0.1",
    "maintenance_rate": "0.085",
    "leverage_max": "10",
    "deduction": "4000"
  },
  {
    "tier": 6,
    "risk_limit": "150000",
    "initial_rate": "0.333",
    "maintenance_rate": "0.3",
    "leverage_max": "3",
    "deduction": "25500"
  },
  {
    "tier": 7,
    "risk_limit": "200000",
    "initial_rate": "0.5",
    "maintenance_rate": "0.45",
    "leverage_max": "2",
    "deduction": "48000"
  },
  {
    "tier": 8,
    "risk_limit": "300000",
    "initial_rate": "1",
    "maintenance_rate": "0.95",
    "leverage_max": "1",
    "deduction": "148000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-responses](https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getfuturesrisklimittable-responseschema)

Status Code **200**

| Name                | Type         | Description                                                |
| ------------------- | ------------ | ---------------------------------------------------------- |
| » _None_            | object       | Information for each tier of the gradient risk limit table |
| »» tier             | integer(int) | Tier                                                       |
| »» risk_limit       | string       | Position risk limit                                        |
| »» initial_rate     | string       | Initial margin rate                                        |
| »» maintenance_rate | string       | Maintenance margin rate                                    |
| »» leverage_max     | string       | Maximum leverage                                           |
| »» deduction        | string       | Maintenance margin quick calculation deduction amount      |

This operation does not require authentication

## [#](#create-price-triggered-order-2) Create price-triggered order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-2](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-2)

> Code samples

`POST /futures/{settle}/price_orders`

_Create price-triggered order_

> Body parameter

```
{
  "initial": {
    "contract": "BTC_USDT",
    "size": 100,
    "price": "5.03"
  },
  "trigger": {
    "strategy_type": 0,
    "price_type": 0,
    "price": "3000",
    "rule": 1,
    "expiration": 86400
  },
  "order_type": "close-long-order"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-parameters)

| Name             | In   | Type                                                            | Required | Description                                                                                                                                                                        |
| ---------------- | ---- | --------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body             | body | [FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder) | true     | none                                                                                                                                                                               |
| » initial        | body | object                                                          | true     | none                                                                                                                                                                               |
| »» contract      | body | string                                                          | true     | Futures contract                                                                                                                                                                   |
| »» size          | body | integer(int64)                                                  | false    | Represents the number of contracts that need to be closed, full closing: size=0                                                                                                    |
| »» price         | body | string                                                          | true     | Order price. Set to 0 to use market price                                                                                                                                          |
| »» close         | body | boolean                                                         | false    | When all positions are closed in a single position mode, it must be set to true to perform the closing operation                                                                   |
| »» tif           | body | string                                                          | false    | Time in force strategy, default is gtc, market orders currently only support ioc mode                                                                                              |
| »» text          | body | string                                                          | false    | The source of the order, including:                                                                                                                                                |
| »» reduce_only   | body | boolean                                                         | false    | When set to true, perform automatic position reduction operation. Set to true to ensure that the order will not open a new position, and is only used to close or reduce positions |
| »» auto_size     | body | string                                                          | false    | Single position mode: auto_size is not required                                                                                                                                    |
| » trigger        | body | object                                                          | true     | none                                                                                                                                                                               |
| »» strategy_type | body | integer(int32)                                                  | false    | Trigger Strategy                                                                                                                                                                   |
| »» price_type    | body | integer(int32)                                                  | false    | Reference price type. 0 - Latest trade price, 1 - Mark price, 2 - Index price                                                                                                      |
| »» price         | body | string                                                          | false    | Price value for price trigger, or spread value for spread trigger                                                                                                                  |
| »» rule          | body | integer(int32)                                                  | false    | Price Condition Type                                                                                                                                                               |
| »» expiration    | body | integer                                                         | false    | Maximum wait time for trigger condition (in seconds). Order will be cancelled if timeout                                                                                           |
| » order_type     | body | string                                                          | false    | Types of take-profit and stop-loss orders, including:                                                                                                                              |
| settle           | path | string                                                          | true     | Settle currency                                                                                                                                                                    |

#### [#](#detailed-descriptions-38) Detailed descriptions

**»» size**: Represents the number of contracts that need to be closed, full
closing: size=0 Partial closing: plan-close-short-position size>0 Partial
closing: plan-close-long-position size<0

**»» close**: When all positions are closed in a single position mode, it must
be set to true to perform the closing operation When partially closed positions
in single-store mode/double-store mode, you can not set close, or close=false

**»» tif**: Time in force strategy, default is gtc, market orders currently only
support ioc mode

- gtc: GoodTillCancelled
- ioc: ImmediateOrCancelled

**»» text**: The source of the order, including:

- web: Web
- api: API call
- app: Mobile app

**»» auto_size**: Single position mode: auto_size is not required Dual position
mode full closing (size=0): auto_size must be set, close_long for closing long
positions, close_short for closing short positions Dual position mode partial
closing (size≠0): auto_size is not required

**»» strategy_type**: Trigger Strategy

- 0: Price trigger, triggered when price meets conditions
- 1: Price spread trigger, i.e. the difference between the latest price
  specified in `price_type` and the second-last price Currently only supports 0
  (latest transaction price)

**»» rule**: Price Condition Type

- 1: Trigger when the price calculated based on `strategy_type` and `price_type`
  is greater than or equal to `Trigger.Price`, while Trigger.Price must >
  last_price
- 2: Trigger when the price calculated based on `strategy_type` and `price_type`
  is less than or equal to `Trigger.Price`, and Trigger.Price must < last_price

**» order_type**: Types of take-profit and stop-loss orders, including:

- `close-long-order`: Order take-profit/stop-loss, close long position
- `close-short-order`: Order take-profit/stop-loss, close short position
- `close-long-position`: Position take-profit/stop-loss, used to close all long
  positions
- `close-short-position`: Position take-profit/stop-loss, used to close all
  short positions
- `plan-close-long-position`: Position plan take-profit/stop-loss, used to close
  all or partial long positions
- `plan-close-short-position`: Position plan take-profit/stop-loss, used to
  close all or partial short positions

The two types of order take-profit/stop-loss are read-only and cannot be passed
in requests

#### [#](#enumerated-values-81) Enumerated Values

| Parameter        | Value |
| ---------------- | ----- |
| »» tif           | gtc   |
| »» tif           | ioc   |
| »» strategy_type | 0     |
| »» strategy_type | 1     |
| »» price_type    | 0     |
| »» price_type    | 1     |
| »» price_type    | 2     |
| »» rule          | 1     |
| »» rule          | 2     |
| settle           | btc   |
| settle           | usdt  |

> Example responses

> 201 Response

```
{
  "id": 1432329
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-responses)

| Status | Meaning                                                                         | Description                | Schema |
| ------ | ------------------------------------------------------------------------------- | -------------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggeredorder-responseschema)

Status Code **201**

_TriggerOrderResponse_

| Name | Type           | Description   |
| ---- | -------------- | ------------- |
| » id | integer(int64) | Auto order ID |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-auto-order-list) Query auto order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-auto-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-auto-order-list)

> Code samples

`GET /futures/{settle}/price_orders`

_Query auto order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpricetriggeredorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listpricetriggeredorders-parameters)

| Name     | In    | Type    | Required | Description                                             |
| -------- | ----- | ------- | -------- | ------------------------------------------------------- |
| status   | query | string  | true     | Query order list based on status                        |
| contract | query | string  | false    | Futures contract, return related data only if specified |
| limit    | query | integer | false    | Maximum number of records returned in a single list     |
| offset   | query | integer | false    | List offset, starting from 0                            |
| settle   | path  | string  | true     | Settle currency                                         |

#### [#](#enumerated-values-82) Enumerated Values

| Parameter | Value    |
| --------- | -------- |
| status    | open     |
| status    | finished |
| settle    | btc      |
| settle    | usdt     |

> Example responses

> 200 Response

```
[
  {
    "initial": {
      "contract": "BTC_USDT",
      "size": 100,
      "price": "5.03"
    },
    "trigger": {
      "strategy_type": 0,
      "price_type": 0,
      "price": "3000",
      "rule": 1,
      "expiration": 86400
    },
    "id": 1283293,
    "user": 1234,
    "create_time": 1514764800,
    "finish_time": 1514764900,
    "trade_id": 13566,
    "status": "finished",
    "finish_as": "cancelled",
    "reason": "",
    "order_type": "close-long-order"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpricetriggeredorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listpricetriggeredorders-responses)

| Status | Meaning                                                                    | Description                 | Schema                                                              |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-auto-orders-2) Cancel all auto orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders-2](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders-2)

> Code samples

`DELETE /futures/{settle}/price_orders`

_Cancel all auto orders_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorderlist-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorderlist-parameters)

| Name     | In    | Type   | Required | Description                                             |
| -------- | ----- | ------ | -------- | ------------------------------------------------------- |
| contract | query | string | false    | Futures contract, return related data only if specified |
| settle   | path  | string | true     | Settle currency                                         |

#### [#](#enumerated-values-83) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "initial": {
      "contract": "BTC_USDT",
      "size": 100,
      "price": "5.03"
    },
    "trigger": {
      "strategy_type": 0,
      "price_type": 0,
      "price": "3000",
      "rule": 1,
      "expiration": 86400
    },
    "id": 1283293,
    "user": 1234,
    "create_time": 1514764800,
    "finish_time": 1514764900,
    "trade_id": 13566,
    "status": "finished",
    "finish_as": "cancelled",
    "reason": "",
    "order_type": "close-long-order"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorderlist-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorderlist-responses)

| Status | Meaning                                                                    | Description                                                                         | Schema                                                              |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation request accepted and processed, success determined by order list | \[[FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-auto-order-details-2) Query single auto order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details-2](https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details-2)

> Code samples

`GET /futures/{settle}/price_orders/{order_id}`

_Query single auto order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getpricetriggeredorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-84) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "initial": {
    "contract": "BTC_USDT",
    "size": 100,
    "price": "5.03"
  },
  "trigger": {
    "strategy_type": 0,
    "price_type": 0,
    "price": "3000",
    "rule": 1,
    "expiration": 86400
  },
  "id": 1283293,
  "user": 1234,
  "create_time": 1514764800,
  "finish_time": 1514764900,
  "trade_id": 13566,
  "status": "finished",
  "finish_as": "cancelled",
  "reason": "",
  "order_type": "close-long-order"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getpricetriggeredorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                          |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-auto-order-2) Cancel single auto order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order-2](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order-2)

> Code samples

`DELETE /futures/{settle}/price_orders/{order_id}`

_Cancel single auto order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-85) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | btc   |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "initial": {
    "contract": "BTC_USDT",
    "size": 100,
    "price": "5.03"
  },
  "trigger": {
    "strategy_type": 0,
    "price_type": 0,
    "price": "3000",
    "rule": 1,
    "expiration": 86400
  },
  "id": 1283293,
  "user": 1234,
  "create_time": 1514764800,
  "finish_time": 1514764900,
  "trade_id": 13566,
  "status": "finished",
  "finish_as": "cancelled",
  "reason": "",
  "order_type": "close-long-order"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggeredorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                          |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret
