# [#](#delivery) Delivery

Delivery contract

## [#](#query-all-futures-contracts-2) Query all futures contracts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-all-futures-contracts-2](https://www.gate.io/docs/developers/apiv4/en/#query-all-futures-contracts-2)

> Code samples

`GET /delivery/{settle}/contracts`

_Query all futures contracts_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverycontracts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverycontracts-parameters)

| Name   | In   | Type   | Required | Description     |
| ------ | ---- | ------ | -------- | --------------- |
| settle | path | string | true     | Settle currency |

#### [#](#enumerated-values-86) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | usdt  |

> Example responses

> 200 Response

```
[
  {
    "name": "BTC_USDT_20200814",
    "underlying": "BTC_USDT",
    "cycle": "WEEKLY",
    "type": "direct",
    "quanto_multiplier": "0.0001",
    "mark_type": "index",
    "last_price": "9017",
    "mark_price": "9019",
    "index_price": "9005.3",
    "basis_rate": "0.185095",
    "basis_value": "13.7",
    "basis_impact_value": "100000",
    "settle_price": "0",
    "settle_price_interval": 60,
    "settle_price_duration": 1800,
    "settle_fee_rate": "0.0015",
    "expire_time": 1593763200,
    "order_price_round": "0.1",
    "mark_price_round": "0.1",
    "leverage_min": "1",
    "leverage_max": "100",
    "maintenance_rate": "1000000",
    "risk_limit_base": "140.726652109199",
    "risk_limit_step": "1000000",
    "risk_limit_max": "8000000",
    "maker_fee_rate": "-0.00025",
    "taker_fee_rate": "0.00075",
    "ref_discount_rate": "0",
    "ref_rebate_rate": "0.2",
    "order_price_deviate": "0.5",
    "order_size_min": 1,
    "order_size_max": 1000000,
    "orders_limit": 50,
    "orderbook_id": 63,
    "trade_id": 26,
    "trade_size": 435,
    "position_size": 130,
    "config_change_time": 1593158867,
    "in_delisting": false
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverycontracts-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverycontracts-responses)

| Status | Meaning                                                                    | Description                 | Schema                                          |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ----------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[DeliveryContract](#schemadeliverycontract)\] |

This operation does not require authentication

## [#](#query-single-contract-information-2) Query single contract information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-contract-information-2](https://www.gate.io/docs/developers/apiv4/en/#query-single-contract-information-2)

> Code samples

`GET /delivery/{settle}/contracts/{contract}`

_Query single contract information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdeliverycontract-parameters](https://www.gate.io/docs/developers/apiv4/en/#getdeliverycontract-parameters)

| Name     | In   | Type   | Required | Description      |
| -------- | ---- | ------ | -------- | ---------------- |
| settle   | path | string | true     | Settle currency  |
| contract | path | string | true     | Futures contract |

#### [#](#enumerated-values-87) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | usdt  |

> Example responses

> 200 Response

```
{
  "name": "BTC_USDT_20200814",
  "underlying": "BTC_USDT",
  "cycle": "WEEKLY",
  "type": "direct",
  "quanto_multiplier": "0.0001",
  "mark_type": "index",
  "last_price": "9017",
  "mark_price": "9019",
  "index_price": "9005.3",
  "basis_rate": "0.185095",
  "basis_value": "13.7",
  "basis_impact_value": "100000",
  "settle_price": "0",
  "settle_price_interval": 60,
  "settle_price_duration": 1800,
  "settle_fee_rate": "0.0015",
  "expire_time": 1593763200,
  "order_price_round": "0.1",
  "mark_price_round": "0.1",
  "leverage_min": "1",
  "leverage_max": "100",
  "maintenance_rate": "1000000",
  "risk_limit_base": "140.726652109199",
  "risk_limit_step": "1000000",
  "risk_limit_max": "8000000",
  "maker_fee_rate": "-0.00025",
  "taker_fee_rate": "0.00075",
  "ref_discount_rate": "0",
  "ref_rebate_rate": "0.2",
  "order_price_deviate": "0.5",
  "order_size_min": 1,
  "order_size_max": 1000000,
  "orders_limit": 50,
  "orderbook_id": 63,
  "trade_id": 26,
  "trade_size": 435,
  "position_size": 130,
  "config_change_time": 1593158867,
  "in_delisting": false
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdeliverycontract-responses](https://www.gate.io/docs/developers/apiv4/en/#getdeliverycontract-responses)

| Status | Meaning                                                                    | Description          | Schema                                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Contract information | [DeliveryContract](#schemadeliverycontract) |

This operation does not require authentication

## [#](#query-futures-market-depth-information-2) Query futures market depth information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-depth-information-2](https://www.gate.io/docs/developers/apiv4/en/#query-futures-market-depth-information-2)

> Code samples

`GET /delivery/{settle}/order_book`

_Query futures market depth information_

Bids will be sorted by price from high to low, while asks sorted reversely

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-parameters)

| Name     | In    | Type    | Required | Description                                                                                   |
| -------- | ----- | ------- | -------- | --------------------------------------------------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                                                               |
| contract | query | string  | true     | Futures contract                                                                              |
| interval | query | string  | false    | Price precision for depth aggregation, 0 means no aggregation, defaults to 0 if not specified |
| limit    | query | integer | false    | Number of depth levels                                                                        |
| with_id  | query | boolean | false    | Whether to return depth update ID. This ID increments by 1 each time depth changes            |

#### [#](#enumerated-values-88) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | usdt  |
| interval  | 0     |
| interval  | 0.1   |
| interval  | 0.01  |

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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-responses)

| Status | Meaning                                                                    | Description            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Depth query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorderbook-responseschema)

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

## [#](#futures-market-transaction-records-2) Futures market transaction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-transaction-records-2](https://www.gate.io/docs/developers/apiv4/en/#futures-market-transaction-records-2)

> Code samples

`GET /delivery/{settle}/trades`

_Futures market transaction records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                                                                                                                                                                          |
| contract | query | string         | true     | Futures contract                                                                                                                                                                                         |
| limit    | query | integer        | false    | Maximum number of records returned in a single list                                                                                                                                                      |
| last_id  | query | string         | false    | Use the ID of the last record in the previous list as the starting point for the next list.This field is no longer supported. For new requests, please use the fromand tofields to specify the time rang |
| from     | query | integer(int64) | false    | Specify starting time in Unix seconds. If not specified, `to` and `limit` will be used to limit response items.                                                                                          |
| to       | query | integer(int64) | false    | Specify end time in Unix seconds, default to current time.                                                                                                                                               |

#### [#](#detailed-descriptions-39) Detailed descriptions

**from**: Specify starting time in Unix seconds. If not specified, `to` and
`limit` will be used to limit response items. If items between `from` and `to`
are more than `limit`, only `limit` number will be returned.

#### [#](#enumerated-values-89) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytrades-responseschema)

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

## [#](#futures-market-k-line-chart-2) Futures market K-line chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-k-line-chart-2](https://www.gate.io/docs/developers/apiv4/en/#futures-market-k-line-chart-2)

> Code samples

`GET /delivery/{settle}/candlesticks`

_Futures market K-line chart_

Return specified contract candlesticks. If prefix `contract` with `mark_`, the
contract's mark price candlesticks are returned; if prefix with `index_`, index
price candlesticks will be returned.

Maximum of 2000 points are returned in one query. Be sure not to exceed the
limit when specifying `from`, `to` and `interval`

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| settle   | path  | string         | true     | Settle currency                                                                                                                                          |
| contract | query | string         | true     | Futures contract                                                                                                                                         |
| from     | query | integer(int64) | false    | Start time of candlesticks, formatted in Unix timestamp in seconds. Default to`to - 100 * interval` if not specified                                     |
| to       | query | integer(int64) | false    | Specify the end time of the K-line chart, defaults to current time if not specified, note that the time format is Unix timestamp with second precision   |
| limit    | query | integer        | false    | Maximum number of recent data points to return. `limit` conflicts with `from` and `to`. If either `from` or `to` is specified, request will be rejected. |
| interval | query | string         | false    | Time interval between data points, note that 1w represents a natural week, 7d time is aligned with Unix initial time                                     |

#### [#](#enumerated-values-90) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | usdt  |
| interval  | 10s   |
| interval  | 30s   |
| interval  | 1m    |
| interval  | 5m    |
| interval  | 15m   |
| interval  | 30m   |
| interval  | 1h    |
| interval  | 2h    |
| interval  | 4h    |
| interval  | 6h    |
| interval  | 8h    |
| interval  | 12h   |
| interval  | 1d    |
| interval  | 7d    |
| interval  | 1w    |
| interval  | 30d   |

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
    "o": "1.032"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliverycandlesticks-responseschema)

Status Code **200**

| Name     | Type           | Description                                                              |
| -------- | -------------- | ------------------------------------------------------------------------ |
| » _None_ | object         | data point in every timestamp                                            |
| »» t     | number(double) | Unix timestamp in seconds                                                |
| »» v     | integer(int64) | size volume (contract size). Only returned if `contract` is not prefixed |
| »» c     | string         | Close price (quote currency)                                             |
| »» h     | string         | Highest price (quote currency)                                           |
| »» l     | string         | Lowest price (quote currency)                                            |
| »» o     | string         | Open price (quote currency)                                              |

This operation does not require authentication

## [#](#get-all-futures-trading-statistics-2) Get all futures trading statistics

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-all-futures-trading-statistics-2](https://www.gate.io/docs/developers/apiv4/en/#get-all-futures-trading-statistics-2)

> Code samples

`GET /delivery/{settle}/tickers`

_Get all futures trading statistics_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-parameters)

| Name     | In    | Type   | Required | Description      |
| -------- | ----- | ------ | -------- | ---------------- |
| settle   | path  | string | true     | Settle currency  |
| contract | query | string | false    | Futures contract |

#### [#](#enumerated-values-91) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliverytickers-responseschema)

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
| » basis_rate              | string | Basis rate                                                                                                             |
| » basis_value             | string | Basis value                                                                                                            |
| » lowest_ask              | string | Recent lowest ask                                                                                                      |
| » lowest_size             | string | The latest seller's lowest price order quantity                                                                        |
| » highest_bid             | string | Recent highest bid                                                                                                     |
| » highest_size            | string | The latest buyer's highest price order volume                                                                          |

This operation does not require authentication

## [#](#futures-market-insurance-fund-history-2) Futures market insurance fund history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#futures-market-insurance-fund-history-2](https://www.gate.io/docs/developers/apiv4/en/#futures-market-insurance-fund-history-2)

> Code samples

`GET /delivery/{settle}/insurance`

_Futures market insurance fund history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-parameters)

| Name   | In    | Type    | Required | Description                                         |
| ------ | ----- | ------- | -------- | --------------------------------------------------- |
| settle | path  | string  | true     | Settle currency                                     |
| limit  | query | integer | false    | Maximum number of records returned in a single list |

#### [#](#enumerated-values-92) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryinsuranceledger-responseschema)

Status Code **200**

| Name   | Type           | Description               |
| ------ | -------------- | ------------------------- |
| _None_ | array          | none                      |
| » t    | integer(int64) | Unix timestamp in seconds |
| » b    | string         | Insurance balance         |

This operation does not require authentication

## [#](#get-futures-account-2) Get futures account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-futures-account-2](https://www.gate.io/docs/developers/apiv4/en/#get-futures-account-2)

> Code samples

`GET /delivery/{settle}/accounts`

_Get futures account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-parameters)

| Name   | In   | Type   | Required | Description     |
| ------ | ---- | ------ | -------- | --------------- |
| settle | path | string | true     | Settle currency |

#### [#](#enumerated-values-93) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccounts-responseschema)

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

## [#](#query-futures-account-change-history-2) Query futures account change history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-account-change-history-2](https://www.gate.io/docs/developers/apiv4/en/#query-futures-account-change-history-2)

> Code samples

`GET /delivery/{settle}/account_book`

_Query futures account change history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-parameters)

| Name   | In    | Type           | Required | Description                                         |
| ------ | ----- | -------------- | -------- | --------------------------------------------------- |
| settle | path  | string         | true     | Settle currency                                     |
| limit  | query | integer        | false    | Maximum number of records returned in a single list |
| from   | query | integer(int64) | false    | Start timestamp                                     |
| to     | query | integer(int64) | false    | Termination Timestamp                               |
| type   | query | string         | false    | Changing Type:                                      |

#### [#](#detailed-descriptions-40) Detailed descriptions

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

#### [#](#enumerated-values-94) Enumerated Values

| Parameter | Value      |
| --------- | ---------- |
| settle    | usdt       |
| type      | dnw        |
| type      | pnl        |
| type      | fee        |
| type      | refr       |
| type      | fund       |
| type      | point_dnw  |
| type      | point_fee  |
| type      | point_refr |

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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryaccountbook-responseschema)

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

#### [#](#enumerated-values-95) Enumerated Values

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

## [#](#get-user-position-list-2) Get user position list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-user-position-list-2](https://www.gate.io/docs/developers/apiv4/en/#get-user-position-list-2)

> Code samples

`GET /delivery/{settle}/positions`

_Get user position list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositions-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositions-parameters)

| Name   | In   | Type   | Required | Description     |
| ------ | ---- | ------ | -------- | --------------- |
| settle | path | string | true     | Settle currency |

#### [#](#enumerated-values-96) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositions-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositions-responses)

| Status | Meaning                                                                    | Description                 | Schema                          |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[Position](#schemaposition)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-single-position-information-2) Get single position information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-single-position-information-2](https://www.gate.io/docs/developers/apiv4/en/#get-single-position-information-2)

> Code samples

`GET /delivery/{settle}/positions/{contract}`

_Get single position information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdeliveryposition-parameters](https://www.gate.io/docs/developers/apiv4/en/#getdeliveryposition-parameters)

| Name     | In   | Type   | Required | Description      |
| -------- | ---- | ------ | -------- | ---------------- |
| settle   | path | string | true     | Settle currency  |
| contract | path | string | true     | Futures contract |

#### [#](#enumerated-values-97) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#getdeliveryposition-responses](https://www.gate.io/docs/developers/apiv4/en/#getdeliveryposition-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-margin-2) Update position margin

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-margin-2](https://www.gate.io/docs/developers/apiv4/en/#update-position-margin-2)

> Code samples

`POST /delivery/{settle}/positions/{contract}/margin`

_Update position margin_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionmargin-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionmargin-parameters)

| Name     | In    | Type   | Required | Description                                                                |
| -------- | ----- | ------ | -------- | -------------------------------------------------------------------------- |
| settle   | path  | string | true     | Settle currency                                                            |
| contract | path  | string | true     | Futures contract                                                           |
| change   | query | string | true     | Margin change amount, positive number increases, negative number decreases |

#### [#](#enumerated-values-98) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionmargin-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionmargin-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-leverage-2) Update position leverage

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage-2](https://www.gate.io/docs/developers/apiv4/en/#update-position-leverage-2)

> Code samples

`POST /delivery/{settle}/positions/{contract}/leverage`

_Update position leverage_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionleverage-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionleverage-parameters)

| Name     | In    | Type   | Required | Description           |
| -------- | ----- | ------ | -------- | --------------------- |
| settle   | path  | string | true     | Settle currency       |
| contract | path  | string | true     | Futures contract      |
| leverage | query | string | true     | New position leverage |

#### [#](#enumerated-values-99) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionleverage-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionleverage-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-position-risk-limit-2) Update position risk limit

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit-2](https://www.gate.io/docs/developers/apiv4/en/#update-position-risk-limit-2)

> Code samples

`POST /delivery/{settle}/positions/{contract}/risk_limit`

_Update position risk limit_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionrisklimit-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionrisklimit-parameters)

| Name       | In    | Type   | Required | Description             |
| ---------- | ----- | ------ | -------- | ----------------------- |
| settle     | path  | string | true     | Settle currency         |
| contract   | path  | string | true     | Futures contract        |
| risk_limit | query | string | true     | New position risk limit |

#### [#](#enumerated-values-100) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionrisklimit-responses](https://www.gate.io/docs/developers/apiv4/en/#updatedeliverypositionrisklimit-responses)

| Status | Meaning                                                                    | Description          | Schema                      |
| ------ | -------------------------------------------------------------------------- | -------------------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Position information | [Position](#schemaposition) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#place-futures-order-2) Place futures order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-futures-order-2](https://www.gate.io/docs/developers/apiv4/en/#place-futures-order-2)

> Code samples

`POST /delivery/{settle}/orders`

_Place futures order_

Zero-fill orders cannot be retrieved 10 minutes after order cancellation

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
[https://www.gate.io/docs/developers/apiv4/en/#createdeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createdeliveryorder-parameters)

| Name          | In   | Type                                | Required | Description                                                                                                                                       |
| ------------- | ---- | ----------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| body          | body | [FuturesOrder](#schemafuturesorder) | true     | none                                                                                                                                              |
| » contract    | body | string                              | true     | Futures contract                                                                                                                                  |
| » size        | body | integer(int64)                      | true     | Required. Trading quantity. Positive for buy, negative for sell. Set to 0 for close position orders.                                              |
| » iceberg     | body | integer(int64)                      | false    | Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden portions are charged taker fees.                                      |
| » price       | body | string                              | false    | Order price. Price of 0 with `tif` set to `ioc` represents a market order.                                                                        |
| » close       | body | boolean                             | false    | Set as `true` to close the position, with `size` set to 0                                                                                         |
| » reduce_only | body | boolean                             | false    | Set as `true` to be reduce-only order                                                                                                             |
| » tif         | body | string                              | false    | Time in force                                                                                                                                     |
| » text        | body | string                              | false    | Custom order information. If not empty, must follow the rules below:                                                                              |
| » auto_size   | body | string                              | false    | Set side to close dual-mode position. `close_long` closes the long side; while `close_short` the short one. Note `size` also needs to be set to 0 |
| » stp_act     | body | string                              | false    | Self-Trading Prevention Action. Users can use this field to set self-trade prevention strategies                                                  |

|» limit_vip|body|integer(int64)|false|限价委托成交的对手单User
VIP 等级，当前下单仅会跟小于等于对手单User
VIP 等级的单成交，仅支持传递11~16，默认是0| |»
pid|body|integer(int64)|false|仓位ID| |settle|path|string|true|Settle currency|

#### [#](#detailed-descriptions-41) Detailed descriptions

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

#### [#](#enumerated-values-101) Enumerated Values

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
[https://www.gate.io/docs/developers/apiv4/en/#createdeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createdeliveryorder-responses)

| Status | Meaning                                                                         | Description   | Schema                              |
| ------ | ------------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-futures-order-list-2) Query futures order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list-2](https://www.gate.io/docs/developers/apiv4/en/#query-futures-order-list-2)

> Code samples

`GET /delivery/{settle}/orders`

_Query futures order list_

Zero-fill orders cannot be retrieved 10 minutes after order cancellation

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorders-parameters)

| Name        | In    | Type    | Required | Description                                                                                    |
| ----------- | ----- | ------- | -------- | ---------------------------------------------------------------------------------------------- |
| contract    | query | string  | false    | Futures contract                                                                               |
| status      | query | string  | true     | Query order list based on status                                                               |
| limit       | query | integer | false    | Maximum number of records returned in a single list                                            |
| offset      | query | integer | false    | List offset, starting from 0                                                                   |
| last_id     | query | string  | false    | Specify the currency name to query in batches, and support up to 100 pass parameters at a time |
| count_total | query | integer | false    | Whether to return total number matched, defaults to 0 (no return)                              |
| settle      | path  | string  | true     | Settle currency                                                                                |

#### [#](#enumerated-values-102) Enumerated Values

| Parameter   | Value    |
| ----------- | -------- |
| status      | open     |
| status      | finished |
| count_total | 0        |
| count_total | 1        |
| settle      | usdt     |

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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryorders-responses)

| Status | Meaning                                                                    | Description                 | Schema                                  |
| ------ | -------------------------------------------------------------------------- | --------------------------- | --------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[FuturesOrder](#schemafuturesorder)\] |

### [#](#response-headers-5) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers-5](https://www.gate.io/docs/developers/apiv4/en/#response-headers-5)

| Status | Header              | Type    | Format | Description                                                      |
| ------ | ------------------- | ------- | ------ | ---------------------------------------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination                                   |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination                                  |
| 200    | X-Pagination-Total  | integer |        | Total number matched, only returned if `count_total` is set to 1 |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-orders-with-open-status-2) Cancel all orders with 'open' status

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status-2](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status-2)

> Code samples

`DELETE /delivery/{settle}/orders`

_Cancel all orders with 'open' status_

Zero-fill orders cannot be retrieved 10 minutes after order cancellation

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorders-parameters)

| Name     | In    | Type   | Required | Description                                                  |
| -------- | ----- | ------ | -------- | ------------------------------------------------------------ |
| contract | query | string | true     | Futures contract                                             |
| side     | query | string | false    | Specify all bids or all asks, both included if not specified |
| settle   | path  | string | true     | Settle currency                                              |

#### [#](#enumerated-values-103) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| side      | ask   |
| side      | bid   |
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
[https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorders-responses](https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorders-responses)

| Status | Meaning                                                                    | Description                   | Schema                                  |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | --------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation successful | \[[FuturesOrder](#schemafuturesorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-order-details-3) Query single order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-3](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-3)

> Code samples

`GET /delivery/{settle}/orders/{order_id}`

_Query single order details_

Zero-fill orders cannot be retrieved 10 minutes after order cancellation

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getdeliveryorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-104) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#getdeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getdeliveryorder-responses)

| Status | Meaning                                                                    | Description   | Schema                              |
| ------ | -------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-order-3) Cancel single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-3](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-3)

> Code samples

`DELETE /delivery/{settle}/orders/{order_id}`

_Cancel single order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-105) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#canceldeliveryorder-responses)

| Status | Meaning                                                                    | Description   | Schema                              |
| ------ | -------------------------------------------------------------------------- | ------------- | ----------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details | [FuturesOrder](#schemafuturesorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-records-3) Query personal trading records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-3](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-3)

> Code samples

`GET /delivery/{settle}/my_trades`

_Query personal trading records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-parameters)

| Name        | In    | Type           | Required | Description                                                                                    |
| ----------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------- |
| settle      | path  | string         | true     | Settle currency                                                                                |
| contract    | query | string         | false    | Futures contract                                                                               |
| order       | query | integer(int64) | false    | Futures order ID, return related data only if specified                                        |
| limit       | query | integer        | false    | Maximum number of records returned in a single list                                            |
| offset      | query | integer        | false    | List offset, starting from 0                                                                   |
| last_id     | query | string         | false    | Specify the currency name to query in batches, and support up to 100 pass parameters at a time |
| count_total | query | integer        | false    | Whether to return total number matched, defaults to 0 (no return)                              |

#### [#](#enumerated-values-106) Enumerated Values

| Parameter   | Value |
| ----------- | ----- |
| settle      | usdt  |
| count_total | 0     |
| count_total | 1     |

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
[https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-responses](https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmydeliverytrades-responseschema)

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

#### [#](#enumerated-values-107) Enumerated Values

| Property | Value |
| -------- | ----- |
| role     | taker |
| role     | maker |

### [#](#response-headers-6) Response Headers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#response-headers-6](https://www.gate.io/docs/developers/apiv4/en/#response-headers-6)

| Status | Header              | Type    | Format | Description                                                      |
| ------ | ------------------- | ------- | ------ | ---------------------------------------------------------------- |
| 200    | X-Pagination-Limit  | integer |        | Limit specified for pagination                                   |
| 200    | X-Pagination-Offset | integer |        | Offset specified for pagination                                  |
| 200    | X-Pagination-Total  | integer |        | Total number matched, only returned if `count_total` is set to 1 |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-position-close-history-2) Query position close history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-position-close-history-2](https://www.gate.io/docs/developers/apiv4/en/#query-position-close-history-2)

> Code samples

`GET /delivery/{settle}/position_close`

_Query position close history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-parameters)

| Name     | In    | Type    | Required | Description                                         |
| -------- | ----- | ------- | -------- | --------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                     |
| contract | query | string  | false    | Futures contract                                    |
| limit    | query | integer | false    | Maximum number of records returned in a single list |

#### [#](#enumerated-values-108) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliverypositionclose-responseschema)

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

#### [#](#enumerated-values-109) Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | long  |
| side     | short |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-liquidation-history-2) Query liquidation history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-history-2](https://www.gate.io/docs/developers/apiv4/en/#query-liquidation-history-2)

> Code samples

`GET /delivery/{settle}/liquidates`

_Query liquidation history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-parameters)

| Name     | In    | Type    | Required | Description                                         |
| -------- | ----- | ------- | -------- | --------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                     |
| contract | query | string  | false    | Futures contract                                    |
| limit    | query | integer | false    | Maximum number of records returned in a single list |
| at       | query | integer | false    | Specify liquidation timestamp                       |

#### [#](#enumerated-values-110) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryliquidates-responseschema)

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

## [#](#query-settlement-records) Query settlement records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-settlement-records](https://www.gate.io/docs/developers/apiv4/en/#query-settlement-records)

> Code samples

`GET /delivery/{settle}/settlements`

_Query settlement records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-parameters)

| Name     | In    | Type    | Required | Description                                         |
| -------- | ----- | ------- | -------- | --------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                     |
| contract | query | string  | false    | Futures contract                                    |
| limit    | query | integer | false    | Maximum number of records returned in a single list |
| at       | query | integer | false    | Specify settlement timestamp                        |

#### [#](#enumerated-values-111) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
    "settle_price": "3421.54",
    "profit": "-6.87498",
    "fee": "0.03079386"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliverysettlements-responseschema)

Status Code **200**

| Name           | Type           | Description         |
| -------------- | -------------- | ------------------- |
| » time         | integer(int64) | Liquidation time    |
| » contract     | string         | Futures contract    |
| » leverage     | string         | Position leverage   |
| » size         | integer(int64) | Position size       |
| » margin       | string         | Position margin     |
| » entry_price  | string         | Average entry price |
| » settle_price | string         | Settled price       |
| » profit       | string         | Profit              |
| » fee          | string         | Fee deducted        |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-risk-limit-tiers-2) Query risk limit tiers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-tiers-2](https://www.gate.io/docs/developers/apiv4/en/#query-risk-limit-tiers-2)

> Code samples

`GET /delivery/{settle}/risk_limit_tiers`

_Query risk limit tiers_

When the 'contract' parameter is not passed, the default is to query the risk
limits for the top 100 markets.'Limit' and 'offset' correspond to pagination
queries at the market level, not to the length of the returned array. This only
takes effect empty.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-parameters)

| Name     | In    | Type    | Required | Description                                         |
| -------- | ----- | ------- | -------- | --------------------------------------------------- |
| settle   | path  | string  | true     | Settle currency                                     |
| contract | query | string  | false    | Futures contract                                    |
| limit    | query | integer | false    | Maximum number of records returned in a single list |
| offset   | query | integer | false    | List offset, starting from 0                        |

#### [#](#enumerated-values-112) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeliveryrisklimittiers-responseschema)

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

## [#](#create-price-triggered-order-3) Create price-triggered order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order-3)

> Code samples

`POST /delivery/{settle}/price_orders`

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
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-parameters)

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

#### [#](#detailed-descriptions-42) Detailed descriptions

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

#### [#](#enumerated-values-113) Enumerated Values

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
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-responses)

| Status | Meaning                                                                         | Description                | Schema |
| ------ | ------------------------------------------------------------------------------- | -------------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createpricetriggereddeliveryorder-responseschema)

Status Code **201**

_TriggerOrderResponse_

| Name | Type           | Description   |
| ---- | -------------- | ------------- |
| » id | integer(int64) | Auto order ID |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-auto-order-list-2) Query auto order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-auto-order-list-2](https://www.gate.io/docs/developers/apiv4/en/#query-auto-order-list-2)

> Code samples

`GET /delivery/{settle}/price_orders`

_Query auto order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpricetriggereddeliveryorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listpricetriggereddeliveryorders-parameters)

| Name     | In    | Type    | Required | Description                                             |
| -------- | ----- | ------- | -------- | ------------------------------------------------------- |
| status   | query | string  | true     | Query order list based on status                        |
| contract | query | string  | false    | Futures contract, return related data only if specified |
| limit    | query | integer | false    | Maximum number of records returned in a single list     |
| offset   | query | integer | false    | List offset, starting from 0                            |
| settle   | path  | string  | true     | Settle currency                                         |

#### [#](#enumerated-values-114) Enumerated Values

| Parameter | Value    |
| --------- | -------- |
| status    | open     |
| status    | finished |
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
[https://www.gate.io/docs/developers/apiv4/en/#listpricetriggereddeliveryorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listpricetriggereddeliveryorders-responses)

| Status | Meaning                                                                    | Description                 | Schema                                                              |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-auto-orders-3) Cancel all auto orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders-3](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders-3)

> Code samples

`DELETE /delivery/{settle}/price_orders`

_Cancel all auto orders_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorderlist-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorderlist-parameters)

| Name     | In    | Type   | Required | Description      |
| -------- | ----- | ------ | -------- | ---------------- |
| contract | query | string | true     | Futures contract |
| settle   | path  | string | true     | Settle currency  |

#### [#](#enumerated-values-115) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorderlist-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorderlist-responses)

| Status | Meaning                                                                    | Description                                                                         | Schema                                                              |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation request accepted and processed, success determined by order list | \[[FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-auto-order-details-3) Query single auto order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details-3](https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details-3)

> Code samples

`GET /delivery/{settle}/price_orders/{order_id}`

_Query single auto order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getpricetriggereddeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getpricetriggereddeliveryorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-116) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#getpricetriggereddeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getpricetriggereddeliveryorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                          |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-auto-order-3) Cancel single auto order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order-3](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order-3)

> Code samples

`DELETE /delivery/{settle}/price_orders/{order_id}`

_Cancel single auto order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| settle   | path | string | true     | Settle currency                                |
| order_id | path | string | true     | ID returned when order is successfully created |

#### [#](#enumerated-values-117) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorder-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelpricetriggereddeliveryorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                          |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [FuturesPriceTriggeredOrder](#schemafuturespricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret
