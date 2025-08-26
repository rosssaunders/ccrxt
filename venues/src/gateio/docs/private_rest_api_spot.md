# [#](#spot) Spot

Spot trading

## [#](#query-all-currency-information) Query all currency information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-all-currency-information](https://www.gate.io/docs/developers/apiv4/en/#query-all-currency-information)

> Code samples

`GET /spot/currencies`

_Query all currency information_

When a currency corresponds to multiple chains, you can query the information of
multiple chains through the `chains` field, such as the charging and recharge
status, identification, etc. of the chain

> Example responses

> 200 Response

```
[
  {
    "currency": "GT",
    "name": "GateToken",
    "delisted": false,
    "withdraw_disabled": false,
    "withdraw_delayed": false,
    "deposit_disabled": false,
    "trade_disabled": false,
    "chain": "GT",
    "chains": [
      {
        "name": "GT",
        "addr": "",
        "withdraw_disabled": false,
        "withdraw_delayed": false,
        "deposit_disabled": false
      },
      {
        "name": "ETH",
        "withdraw_disabled": false,
        "withdraw_delayed": false,
        "deposit_disabled": false,
        "addr": "0xE66747a101bFF2dBA3697199DCcE5b743b454759"
      },
      {
        "name": "GTEVM",
        "withdraw_disabled": false,
        "withdraw_delayed": false,
        "deposit_disabled": false,
        "addr": ""
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencies-responses](https://www.gate.io/docs/developers/apiv4/en/#listcurrencies-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencies-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcurrencies-responseschema)

Status Code **200**

| Name                  | Type    | Description                                                                     |
| --------------------- | ------- | ------------------------------------------------------------------------------- |
| _None_                | array   | none                                                                            |
| » currency            | string  | Currency symbol                                                                 |
| » name                | string  | Currency name                                                                   |
| » delisted            | boolean | Whether currency is de-listed                                                   |
| » withdraw_disabled   | boolean | Whether withdrawal is suspended (deprecated)                                    |
| » withdraw_delayed    | boolean | Whether withdrawal has delay (deprecated)                                       |
| » deposit_disabled    | boolean | Whether deposit is suspended (deprecated)                                       |
| » trade_disabled      | boolean | Whether currency's trading is disabled                                          |
| » fixed_rate          | string  | Fixed fee rate. Only for fixed rate currencies, not valid for normal currencies |
| » chain               | string  | The main chain corresponding to the coin                                        |
| » is_private          | boolean | Is it a privacy currency?                                                       |
| » chains              | array   | All links corresponding to coins                                                |
| »» SpotCurrencyChain  | object  | none                                                                            |
| »»» name              | string  | Blockchain name                                                                 |
| »»» addr              | string  | token address                                                                   |
| »»» withdraw_disabled | boolean | Whether currency's withdrawal is disabled                                       |
| »»» withdraw_delayed  | boolean | Whether currency's withdrawal is delayed                                        |
| »»» deposit_disabled  | boolean | Whether currency's deposit is disabled                                          |

This operation does not require authentication

## [#](#query-single-currency-information) Query single currency information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-currency-information](https://www.gate.io/docs/developers/apiv4/en/#query-single-currency-information)

> Code samples

`GET /spot/currencies/{currency}`

_Query single currency information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrency-parameters](https://www.gate.io/docs/developers/apiv4/en/#getcurrency-parameters)

| Name     | In   | Type   | Required | Description   |
| -------- | ---- | ------ | -------- | ------------- |
| currency | path | string | true     | Currency name |

> Example responses

> 200 Response

```
{
  "currency": "GT",
  "name": "GateToken",
  "delisted": false,
  "withdraw_disabled": false,
  "withdraw_delayed": false,
  "deposit_disabled": false,
  "trade_disabled": false,
  "chain": "GT",
  "chains": [
    {
      "name": "GT",
      "addr": "",
      "withdraw_disabled": false,
      "withdraw_delayed": false,
      "deposit_disabled": false
    },
    {
      "name": "ETH",
      "withdraw_disabled": false,
      "withdraw_delayed": false,
      "deposit_disabled": false,
      "addr": "0xE66747a101bFF2dBA3697199DCcE5b743b454759"
    },
    {
      "name": "GTEVM",
      "withdraw_disabled": false,
      "withdraw_delayed": false,
      "deposit_disabled": false,
      "addr": ""
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrency-responses](https://www.gate.io/docs/developers/apiv4/en/#getcurrency-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrency-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getcurrency-responseschema)

Status Code **200**

| Name                  | Type    | Description                                                                     |
| --------------------- | ------- | ------------------------------------------------------------------------------- |
| » currency            | string  | Currency symbol                                                                 |
| » name                | string  | Currency name                                                                   |
| » delisted            | boolean | Whether currency is de-listed                                                   |
| » withdraw_disabled   | boolean | Whether withdrawal is suspended (deprecated)                                    |
| » withdraw_delayed    | boolean | Whether withdrawal has delay (deprecated)                                       |
| » deposit_disabled    | boolean | Whether deposit is suspended (deprecated)                                       |
| » trade_disabled      | boolean | Whether currency's trading is disabled                                          |
| » fixed_rate          | string  | Fixed fee rate. Only for fixed rate currencies, not valid for normal currencies |
| » chain               | string  | The main chain corresponding to the coin                                        |
| » is_private          | boolean | Is it a privacy currency?                                                       |
| » chains              | array   | All links corresponding to coins                                                |
| »» SpotCurrencyChain  | object  | none                                                                            |
| »»» name              | string  | Blockchain name                                                                 |
| »»» addr              | string  | token address                                                                   |
| »»» withdraw_disabled | boolean | Whether currency's withdrawal is disabled                                       |
| »»» withdraw_delayed  | boolean | Whether currency's withdrawal is delayed                                        |
| »»» deposit_disabled  | boolean | Whether currency's deposit is disabled                                          |

This operation does not require authentication

## [#](#query-all-supported-currency-pairs) Query all supported currency pairs

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-all-supported-currency-pairs](https://www.gate.io/docs/developers/apiv4/en/#query-all-supported-currency-pairs)

> Code samples

`GET /spot/currency_pairs`

_Query all supported currency pairs_

> Example responses

> 200 Response

```
[
  {
    "id": "ETH_USDT",
    "base": "ETH",
    "base_name": "Ethereum",
    "quote": "USDT",
    "quote_name": "Tether",
    "fee": "0.2",
    "min_base_amount": "0.001",
    "min_quote_amount": "1.0",
    "max_base_amount": "10000",
    "max_quote_amount": "10000000",
    "amount_precision": 3,
    "precision": 6,
    "trade_status": "tradable",
    "sell_start": 1516378650,
    "buy_start": 1516378650,
    "delisting_time": 0,
    "trade_url": "https://www.gate.io/trade/ETH_USDT",
    "st_tag": false
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencypairs-responses](https://www.gate.io/docs/developers/apiv4/en/#listcurrencypairs-responses)

| Status | Meaning                                                                    | Description                  | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | All currency pairs retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencypairs-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcurrencypairs-responseschema)

Status Code **200**

| Name                | Type    | Description                                                      |
| ------------------- | ------- | ---------------------------------------------------------------- |
| _None_              | array   | \[Spot currency pair\]                                           |
| » _None_            | object  | Spot currency pair                                               |
| »» id               | string  | Trading pair                                                     |
| »» base             | string  | Base currency                                                    |
| »» base_name        | string  | Base currency name                                               |
| »» quote            | string  | Quote currency                                                   |
| »» quote_name       | string  | Quote currency name                                              |
| »» fee              | string  | Trading fee rate                                                 |
| »» min_base_amount  | string  | Minimum amount of base currency to trade, `null` means no limit  |
| »» min_quote_amount | string  | Minimum amount of quote currency to trade, `null` means no limit |
| »» max_base_amount  | string  | Maximum amount of base currency to trade, `null` means no limit  |
| »» max_quote_amount | string  | Maximum amount of quote currency to trade, `null` means no limit |
| »» amount_precision | integer | Amount scale                                                     |
| »» precision        | integer | Price scale                                                      |
| »» trade_status     | string  | Trading status                                                   |

\- untradable: cannot be traded  
\- buyable: can be bought  
\- sellable: can be sold  
\- tradable: can be bought and sold | | »» sell_start | integer(int64) | Sell
start unix timestamp in seconds | | »» buy_start | integer(int64) | Buy start
unix timestamp in seconds | | »» delisting_time | integer(int64) | Expected time
to remove the shelves, Unix timestamp in seconds | | »» type | string | Trading
pair type, normal: normal, premarket: pre-market | | »» trade_url | string |
Transaction link | | »» st_tag | boolean | Whether the trading pair is in ST
risk assessment, false - No, true - Yes |

#### [#](#enumerated-values-10) Enumerated Values

| Property     | Value      |
| ------------ | ---------- |
| trade_status | untradable |
| trade_status | buyable    |
| trade_status | sellable   |
| trade_status | tradable   |

This operation does not require authentication

## [#](#query-single-currency-pair-details) Query single currency pair details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-currency-pair-details](https://www.gate.io/docs/developers/apiv4/en/#query-single-currency-pair-details)

> Code samples

`GET /spot/currency_pairs/{currency_pair}`

_Query single currency pair details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-parameters](https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-parameters)

| Name          | In   | Type   | Required | Description  |
| ------------- | ---- | ------ | -------- | ------------ |
| currency_pair | path | string | true     | Trading pair |

> Example responses

> 200 Response

```
{
  "id": "ETH_USDT",
  "base": "ETH",
  "base_name": "Ethereum",
  "quote": "USDT",
  "quote_name": "Tether",
  "fee": "0.2",
  "min_base_amount": "0.001",
  "min_quote_amount": "1.0",
  "max_base_amount": "10000",
  "max_quote_amount": "10000000",
  "amount_precision": 3,
  "precision": 6,
  "trade_status": "tradable",
  "sell_start": 1516378650,
  "buy_start": 1516378650,
  "delisting_time": 0,
  "trade_url": "https://www.gate.io/trade/ETH_USDT",
  "st_tag": false
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-responses](https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getcurrencypair-responseschema)

Status Code **200**

_Spot currency pair_

| Name               | Type    | Description                                                      |
| ------------------ | ------- | ---------------------------------------------------------------- |
| » id               | string  | Trading pair                                                     |
| » base             | string  | Base currency                                                    |
| » base_name        | string  | Base currency name                                               |
| » quote            | string  | Quote currency                                                   |
| » quote_name       | string  | Quote currency name                                              |
| » fee              | string  | Trading fee rate                                                 |
| » min_base_amount  | string  | Minimum amount of base currency to trade, `null` means no limit  |
| » min_quote_amount | string  | Minimum amount of quote currency to trade, `null` means no limit |
| » max_base_amount  | string  | Maximum amount of base currency to trade, `null` means no limit  |
| » max_quote_amount | string  | Maximum amount of quote currency to trade, `null` means no limit |
| » amount_precision | integer | Amount scale                                                     |
| » precision        | integer | Price scale                                                      |
| » trade_status     | string  | Trading status                                                   |

\- untradable: cannot be traded  
\- buyable: can be bought  
\- sellable: can be sold  
\- tradable: can be bought and sold | | » sell_start | integer(int64) | Sell
start unix timestamp in seconds | | » buy_start | integer(int64) | Buy start
unix timestamp in seconds | | » delisting_time | integer(int64) | Expected time
to remove the shelves, Unix timestamp in seconds | | » type | string | Trading
pair type, normal: normal, premarket: pre-market | | » trade_url | string |
Transaction link | | » st_tag | boolean | Whether the trading pair is in ST risk
assessment, false - No, true - Yes |

#### [#](#enumerated-values-11) Enumerated Values

| Property     | Value      |
| ------------ | ---------- |
| trade_status | untradable |
| trade_status | buyable    |
| trade_status | sellable   |
| trade_status | tradable   |

This operation does not require authentication

## [#](#get-currency-pair-ticker-information) Get currency pair ticker information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-currency-pair-ticker-information](https://www.gate.io/docs/developers/apiv4/en/#get-currency-pair-ticker-information)

> Code samples

`GET /spot/tickers`

_Get currency pair ticker information_

If `currency_pair` is specified, only query that currency pair; otherwise return
all information

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtickers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listtickers-parameters)

| Name          | In    | Type   | Required | Description  |
| ------------- | ----- | ------ | -------- | ------------ |
| currency_pair | query | string | false    | Trading pair |
| timezone      | query | string | false    | Timezone     |

#### [#](#enumerated-values-12) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| timezone  | utc0  |
| timezone  | utc8  |
| timezone  | all   |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "BTC3L_USDT",
    "last": "2.46140352",
    "lowest_ask": "2.477",
    "highest_bid": "2.4606821",
    "change_percentage": "-8.91",
    "change_utc0": "-8.91",
    "change_utc8": "-8.91",
    "base_volume": "656614.0845820589",
    "quote_volume": "1602221.66468375534639404191",
    "high_24h": "2.7431",
    "low_24h": "1.9863",
    "etf_net_value": "2.46316141",
    "etf_pre_net_value": "2.43201848",
    "etf_pre_timestamp": 1611244800,
    "etf_leverage": "2.2803019447281203"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtickers-responses](https://www.gate.io/docs/developers/apiv4/en/#listtickers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtickers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listtickers-responseschema)

Status Code **200**

| Name                | Type           | Description                                                                                                            |
| ------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| » currency_pair     | string         | Trading pair                                                                                                           |
| » last              | string         | Last trading price                                                                                                     |
| » lowest_ask        | string         | Recent lowest ask                                                                                                      |
| » lowest_size       | string         | Latest seller's lowest price quantity; not available for batch queries; available for single queries, empty if no data |
| » highest_bid       | string         | Recent highest bid                                                                                                     |
| » highest_size      | string         | Latest buyer's highest price quantity; not available for batch queries; available for single queries, empty if no data |
| » change_percentage | string         | 24h price change percentage (negative for decrease, e.g., -7.45)                                                       |
| » change_utc0       | string         | UTC+0 timezone, 24h price change percentage, negative for decline (e.g., -7.45)                                        |
| » change_utc8       | string         | UTC+8 timezone, 24h price change percentage, negative for decline (e.g., -7.45)                                        |
| » base_volume       | string         | Base currency trading volume in the last 24h                                                                           |
| » quote_volume      | string         | Quote currency trading volume in the last 24h                                                                          |
| » high_24h          | string         | 24h High                                                                                                               |
| » low_24h           | string         | 24h Low                                                                                                                |
| » etf_net_value     | string         | ETF net value                                                                                                          |
| » etf_pre_net_value | string         | null                                                                                                                   | ETF net value at previous rebalancing point |
| » etf_pre_timestamp | integer(int64) | null                                                                                                                   | ETF previous rebalancing time               |
| » etf_leverage      | string         | null                                                                                                                   | ETF current leverage                        |

This operation does not require authentication

## [#](#get-market-depth-information) Get market depth information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-market-depth-information](https://www.gate.io/docs/developers/apiv4/en/#get-market-depth-information)

> Code samples

`GET /spot/order_book`

_Get market depth information_

Market depth buy orders are sorted by price from high to low, sell orders are
sorted from low to high

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorderbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listorderbook-parameters)

| Name          | In    | Type    | Required | Description                                                                                   |
| ------------- | ----- | ------- | -------- | --------------------------------------------------------------------------------------------- |
| currency_pair | query | string  | true     | Trading pair                                                                                  |
| interval      | query | string  | false    | Price precision for depth aggregation, 0 means no aggregation, defaults to 0 if not specified |
| limit         | query | integer | false    | Number of depth levels                                                                        |
| with_id       | query | boolean | false    | Return order book update ID                                                                   |

> Example responses

> 200 Response

```
{
  "id": 123456,
  "current": 1623898993123,
  "update": 1623898993121,
  "asks": [
    [
      "1.52",
      "1.151"
    ],
    [
      "1.53",
      "1.218"
    ]
  ],
  "bids": [
    [
      "1.17",
      "201.863"
    ],
    [
      "1.16",
      "725.464"
    ]
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorderbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listorderbook-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorderbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listorderbook-responseschema)

Status Code **200**

| Name      | Type           | Description                                                                                                    |
| --------- | -------------- | -------------------------------------------------------------------------------------------------------------- |
| » id      | integer(int64) | Order book ID, which is updated whenever the order book is changed. Valid only when `with_id` is set to `true` |
| » current | integer(int64) | The timestamp of the response data being generated (in milliseconds)                                           |
| » update  | integer(int64) | The timestamp of when the orderbook last changed (in milliseconds)                                             |
| » asks    | array          | Ask Depth                                                                                                      |
| »» _None_ | array          | Price and Quantity Pair                                                                                        |
| » bids    | array          | Bid Depth                                                                                                      |
| »» _None_ | array          | Price and Quantity Pair                                                                                        |

This operation does not require authentication

## [#](#query-market-transaction-records) Query market transaction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-market-transaction-records](https://www.gate.io/docs/developers/apiv4/en/#query-market-transaction-records)

> Code samples

`GET /spot/trades`

_Query market transaction records_

Supports querying by time range using `from` and `to` parameters or pagination
based on `last_id`. By default, queries the last 30 days.

Pagination based on `last_id` is no longer recommended. If `last_id` is
specified, the time range query parameters will be ignored.

When using limit&page pagination to retrieve data, the maximum number of pages
is 100,000, that is, limit \* (page - 1) <= 100,000.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listtrades-parameters)

| Name          | In    | Type           | Required | Description                                                                                    |
| ------------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------- |
| currency_pair | query | string         | true     | Trading pair                                                                                   |
| limit         | query | integer(int32) | false    | Maximum number of items returned in list. Default: 100, minimum: 1, maximum: 1000              |
| last_id       | query | string         | false    | Specify the currency name to query in batches, and support up to 100 pass parameters at a time |
| reverse       | query | boolean        | false    | Whether to retrieve data less than `last_id`. Default returns records greater than `last_id`.  |
| from          | query | integer(int64) | false    | Start timestamp for the query                                                                  |
| to            | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                         |
| page          | query | integer(int32) | false    | Page number                                                                                    |

#### [#](#detailed-descriptions-10) Detailed descriptions

**reverse**: Whether to retrieve data less than `last_id`. Default returns
records greater than `last_id`.

Set to `true` to trace back market trade records, `false` to get latest trades.

No effect when `last_id` is not set.

> Example responses

> 200 Response

```
[
  {
    "id": "1232893232",
    "create_time": "1548000000",
    "create_time_ms": "1548000000123.456",
    "order_id": "4128442423",
    "side": "buy",
    "role": "maker",
    "amount": "0.15",
    "price": "0.03",
    "fee": "0.0005",
    "fee_currency": "ETH",
    "point_fee": "0",
    "gt_fee": "0",
    "sequence_id": "588018",
    "text": "t-test"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listtrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listtrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listtrades-responseschema)

Status Code **200**

| Name                                                     | Type   | Description                                                    |
| -------------------------------------------------------- | ------ | -------------------------------------------------------------- |
| _None_                                                   | array  | none                                                           |
| » id                                                     | string | Fill ID                                                        |
| » create_time                                            | string | Fill Time                                                      |
| » create_time_ms                                         | string | Trading time, with millisecond precision                       |
| » currency_pair                                          | string | Currency pair                                                  |
| » side                                                   | string | Buy or sell order                                              |
| » role                                                   | string | Trade role, not returned in public endpoints                   |
| » amount                                                 | string | Trade amount                                                   |
| » price                                                  | string | Order price                                                    |
| » order_id                                               | string | Related order ID, not returned in public endpoints             |
| » fee                                                    | string | Fee deducted, not returned in public endpoints                 |
| » fee_currency                                           | string | Fee currency unit, not returned in public endpoints            |
| » point_fee                                              | string | Points used to deduct fee, not returned in public endpoints    |
| » gt_fee                                                 | string | GT used to deduct fee, not returned in public endpoints        |
| » amend_text                                             | string | The custom data that the user remarked when amending the order |
| » sequence_id                                            | string | Consecutive trade ID within a single market.                   |
| Used to track and identify trades in the specific market |
| » text                                                   | string | User-defined information, not returned in public endpoints     |

#### [#](#enumerated-values-13) Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | buy   |
| side     | sell  |
| role     | taker |
| role     | maker |

This operation does not require authentication

## [#](#market-k-line-chart) Market K-line chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#market-k-line-chart](https://www.gate.io/docs/developers/apiv4/en/#market-k-line-chart)

> Code samples

`GET /spot/candlesticks`

_Market K-line chart_

Maximum of 1000 points can be returned in a query. Be sure not to exceed the
limit when specifying from, to and interval

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-parameters)

| Name          | In    | Type           | Required | Description                                                                                                                                              |
| ------------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency_pair | query | string         | true     | Trading pair                                                                                                                                             |
| limit         | query | integer        | false    | Maximum number of recent data points to return. `limit` conflicts with `from` and `to`. If either `from` or `to` is specified, request will be rejected. |
| from          | query | integer(int64) | false    | Start time of candlesticks, formatted in Unix timestamp in seconds. Default to`to - 100 * interval` if not specified                                     |
| to            | query | integer(int64) | false    | Specify the end time of the K-line chart, defaults to current time if not specified, note that the time format is Unix timestamp with second precision   |
| interval      | query | string         | false    | Time interval between data points. Note that `30d` represents a calendar month, not aligned to 30 days                                                   |

#### [#](#enumerated-values-14) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| interval  | 1s    |
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
| interval  | 30d   |

> Example responses

> 200 Response

```
[
  [
    "1539852480",
    "971519.677",
    "0.0021724",
    "0.0021922",
    "0.0021724",
    "0.0021737",
    "true"
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-responses](https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-responses)

| Status | Meaning                                                                    | Description      | Schema         |
| ------ | -------------------------------------------------------------------------- | ---------------- | -------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[\[string\]\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcandlesticks-responseschema)

Status Code **200**

| Name     | Type  | Description                                                     |
| -------- | ----- | --------------------------------------------------------------- |
| » _None_ | array | Candlestick data for each time granularity, from left to right: |

\- Unix timestamp with second precision  
\- Trading volume in quote currency  
\- Closing price  
\- Highest price  
\- Lowest price  
\- Opening price  
\- Trading volume in base currency  
\- Whether window is closed; true means this candlestick data segment is
complete, false means not yet complete |

This operation does not require authentication

## [#](#query-account-fee-rates) Query account fee rates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-account-fee-rates](https://www.gate.io/docs/developers/apiv4/en/#query-account-fee-rates)

> Code samples

`GET /spot/fee`

_Query account fee rates_

This API is deprecated. The new fee query API is `/wallet/fee`

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfee-parameters](https://www.gate.io/docs/developers/apiv4/en/#getfee-parameters)

| Name          | In    | Type   | Required | Description                                              |
| ------------- | ----- | ------ | -------- | -------------------------------------------------------- |
| currency_pair | query | string | false    | Specify currency pair to get more accurate fee settings. |

#### [#](#detailed-descriptions-11) Detailed descriptions

**currency_pair**: Specify currency pair to get more accurate fee settings.

This field is optional. Usually fee settings are the same for all currency
pairs.

> Example responses

> 200 Response

```
{
  "user_id": 10001,
  "taker_fee": "0.002",
  "maker_fee": "0.002",
  "gt_discount": false,
  "gt_taker_fee": "0",
  "gt_maker_fee": "0",
  "loan_fee": "0.18",
  "point_type": "1",
  "currency_pair": "BTC_USDT",
  "debit_fee": 3
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfee-responses](https://www.gate.io/docs/developers/apiv4/en/#getfee-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getfee-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getfee-responseschema)

Status Code **200**

| Name            | Type           | Description                                                                          |
| --------------- | -------------- | ------------------------------------------------------------------------------------ |
| » user_id       | integer(int64) | User ID                                                                              |
| » taker_fee     | string         | taker fee rate                                                                       |
| » maker_fee     | string         | maker fee rate                                                                       |
| » gt_discount   | boolean        | Whether GT deduction discount is enabled                                             |
| » gt_taker_fee  | string         | Taker fee rate if using GT deduction. It will be 0 if GT deduction is disabled       |
| » gt_maker_fee  | string         | Maker fee rate with GT deduction. Returns 0 if GT deduction is disabled              |
| » loan_fee      | string         | Loan fee rate of margin lending                                                      |
| » point_type    | string         | Point card type: 0 - Original version, 1 - New version since 202009                  |
| » currency_pair | string         | Trading pair                                                                         |
| » debit_fee     | integer        | Deduction types for rates, 1 - GT deduction, 2 - Point card deduction, 3 - VIP rates |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-query-account-fee-rates) Batch query account fee rates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-query-account-fee-rates](https://www.gate.io/docs/developers/apiv4/en/#batch-query-account-fee-rates)

> Code samples

`GET /spot/batch_fee`

_Batch query account fee rates_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-parameters](https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-parameters)

| Name           | In    | Type   | Required | Description                           |
| -------------- | ----- | ------ | -------- | ------------------------------------- |
| currency_pairs | query | string | true     | Maximum 50 currency pairs per request |

> Example responses

> 200 Response

```
{
  "BTC_USDT": {
    "user_id": 10001,
    "taker_fee": "0.002",
    "maker_fee": "0.002",
    "gt_discount": false,
    "gt_taker_fee": "0",
    "gt_maker_fee": "0",
    "loan_fee": "0.18",
    "point_type": "1",
    "currency_pair": "BTC_USDT",
    "debit_fee": 3
  },
  "GT_USDT": {
    "user_id": 10001,
    "taker_fee": "0.002",
    "maker_fee": "0.002",
    "gt_discount": false,
    "gt_taker_fee": "0",
    "gt_maker_fee": "0",
    "loan_fee": "0.18",
    "point_type": "1",
    "currency_pair": "GT_USDT",
    "debit_fee": 3
  },
  "ETH_USDT": {
    "user_id": 10001,
    "taker_fee": "0.002",
    "maker_fee": "0.002",
    "gt_discount": false,
    "gt_taker_fee": "0",
    "gt_maker_fee": "0",
    "loan_fee": "0.18",
    "point_type": "1",
    "currency_pair": "ETH_USDT",
    "debit_fee": 3
  }
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-responses](https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getbatchspotfee-responseschema)

Status Code **200**

| Name                       | Type           | Description                                                                          |
| -------------------------- | -------------- | ------------------------------------------------------------------------------------ |
| » **additionalProperties** | object         | none                                                                                 |
| »» user_id                 | integer(int64) | User ID                                                                              |
| »» taker_fee               | string         | taker fee rate                                                                       |
| »» maker_fee               | string         | maker fee rate                                                                       |
| »» gt_discount             | boolean        | Whether GT deduction discount is enabled                                             |
| »» gt_taker_fee            | string         | Taker fee rate if using GT deduction. It will be 0 if GT deduction is disabled       |
| »» gt_maker_fee            | string         | Maker fee rate with GT deduction. Returns 0 if GT deduction is disabled              |
| »» loan_fee                | string         | Loan fee rate of margin lending                                                      |
| »» point_type              | string         | Point card type: 0 - Original version, 1 - New version since 202009                  |
| »» currency_pair           | string         | Trading pair                                                                         |
| »» debit_fee               | integer        | Deduction types for rates, 1 - GT deduction, 2 - Point card deduction, 3 - VIP rates |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-spot-trading-accounts) List spot trading accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-spot-trading-accounts](https://www.gate.io/docs/developers/apiv4/en/#list-spot-trading-accounts)

> Code samples

`GET /spot/accounts`

_List spot trading accounts_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | false    | Query by specified currency name |

> Example responses

> 200 Response

```
[
  {
    "currency": "ETH",
    "available": "968.8",
    "locked": "0",
    "update_id": 98
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listspotaccounts-responseschema)

Status Code **200**

| Name        | Type           | Description                    |
| ----------- | -------------- | ------------------------------ |
| » currency  | string         | Currency detail                |
| » available | string         | Available amount               |
| » locked    | string         | Locked amount, used in trading |
| » update_id | integer(int64) | Version number                 |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-spot-account-transaction-history) Query spot account transaction history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-spot-account-transaction-history](https://www.gate.io/docs/developers/apiv4/en/#query-spot-account-transaction-history)

> Code samples

`GET /spot/account_book`

_Query spot account transaction history_

Record query time range cannot exceed 30 days.

When using limit&page pagination to retrieve data, the maximum number of pages
is 100,000, that is, limit \* (page - 1) <= 100,000.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-parameters)

| Name     | In    | Type           | Required | Description                                                                                                                            |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| currency | query | string         | false    | Query by specified currency name                                                                                                       |
| from     | query | integer(int64) | false    | Start timestamp for the query                                                                                                          |
| to       | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                                                                 |
| page     | query | integer(int32) | false    | Page number                                                                                                                            |
| limit    | query | integer        | false    | Maximum number of records returned in a single list                                                                                    |
| type     | query | string         | false    | Query by specified account change type. If not specified, all change types will be included.                                           |
| code     | query | string         | false    | Specify account change code for query. If not specified, all change types are included. This parameter has higher priority than `type` |

> Example responses

> 200 Response

```
[
  {
    "id": "123456",
    "time": 1547633726123,
    "currency": "BTC",
    "change": "1.03",
    "balance": "4.59316525194",
    "type": "margin_in",
    "text": "3815099"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listspotaccountbook-responseschema)

Status Code **200**

| Name       | Type           | Description                                                                               |
| ---------- | -------------- | ----------------------------------------------------------------------------------------- |
| » id       | string         | Balance change record ID                                                                  |
| » time     | integer(int64) | The timestamp of the change (in milliseconds)                                             |
| » currency | string         | Currency changed                                                                          |
| » change   | string         | Amount changed. Positive value means transferring in, while negative out                  |
| » balance  | string         | Balance after change                                                                      |
| » type     | string         | Account book type. Please refer to [account book type](#accountbook-type) for more detail |
| » code     | string         | Account change code, see \[Asset Record Code\] (Asset Record Code)                        |
| » text     | string         | Additional information                                                                    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-place-orders) Batch place orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-place-orders](https://www.gate.io/docs/developers/apiv4/en/#batch-place-orders)

> Code samples

`POST /spot/batch_orders`

_Batch place orders_

Batch order requirements:

1.  Custom order field `text` is required
2.  At most 4 trading pairs, maximum 10 orders each, are allowed in one request
3.  No mixture of spot orders and margin orders, i.e. `account` must be
    identical for all orders

> Body parameter

```
[
  {
    "text": "t-abc123",
    "currency_pair": "BTC_USDT",
    "type": "limit",
    "account": "unified",
    "side": "buy",
    "amount": "0.001",
    "price": "65000",
    "time_in_force": "gtc",
    "iceberg": "0"
  }
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-parameters)

| Name           | In     | Type   | Required | Description                                                                                                                                      |
| -------------- | ------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array  | true     | none                                                                                                                                             |

> Example responses

> 200 Response

```
[
  {
    "order_id": "12332324",
    "amend_text": "t-123456",
    "text": "t-123456",
    "succeeded": true,
    "label": "",
    "message": "",
    "id": "12332324",
    "create_time": "1548000000",
    "update_time": "1548000100",
    "create_time_ms": 1548000000123,
    "update_time_ms": 1548000100123,
    "currency_pair": "ETC_BTC",
    "status": "cancelled",
    "type": "limit",
    "account": "spot",
    "side": "buy",
    "amount": "1",
    "price": "5.00032",
    "time_in_force": "gtc",
    "iceberg": "0",
    "left": "0.5",
    "filled_amount": "1.242",
    "filled_total": "2.50016",
    "avg_deal_price": "5.00032",
    "fee": "0.005",
    "fee_currency": "ETH",
    "point_fee": "0",
    "gt_fee": "0",
    "gt_discount": false,
    "rebated_fee": "0",
    "rebated_fee_currency": "BTC",
    "stp_act": "cn",
    "finish_as": "stp",
    "stp_id": 10240
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-responses](https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Request execution completed | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createbatchorders-responseschema)

Status Code **200**

| Name          | Type   | Description                                                                                                          |
| ------------- | ------ | -------------------------------------------------------------------------------------------------------------------- |
| _None_        | array  | \[Batch order details\]                                                                                              |
| » _None_      | object | Batch order details                                                                                                  |
| »» order_id   | string | Order ID                                                                                                             |
| »» amend_text | string | The custom data that the user remarked when amending the order                                                       |
| »» text       | string | Order custom information. Users can set custom ID with this field. Custom fields must meet the following conditions: |

1\. Must start with `t-`  
2\. Excluding `t-`, length cannot exceed 28 bytes  
3\. Can only contain numbers, letters, underscore(\_), hyphen(-) or dot(.) | |
»» succeeded | boolean | Request execution result | | »» label | string | Error
label, if any, otherwise an empty string | | »» message | string | Detailed
error message, if any, otherwise an empty string | | »» id | string | Order ID |
| »» create_time | string | Creation time of order | | »» update_time | string |
Last modification time of order | | »» create_time_ms | integer(int64) |
Creation time of order (in milliseconds) | | »» update_time_ms | integer(int64)
| Last modification time of order (in milliseconds) | | »» status | string |
Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | »» currency_pair | string | Currency pair | | »»
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | »» account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | »» side |
string | Buy or sell order | | »» amount | string | Trade amount | | »» price |
string | Order price | | »» time_in_force | string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none | | »» iceberg | string |
Amount to display for the iceberg order. Null or 0 for normal orders. Hiding all
amount is not supported | | »» auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »»
left | string | Amount left to fill | | »» filled_amount | string | Amount
filled | | »» fill_price | string | Total filled in quote currency. Deprecated
in favor of `filled_total` | | »» filled_total | string | Total filled in quote
currency | | »» avg_deal_price | string | Average fill price | | »» fee | string
| Fee deducted | | »» fee_currency | string | Fee currency unit | | »» point_fee
| string | Points used to deduct fee | | »» gt_fee | string | GT used to deduct
fee | | »» gt_discount | boolean | Whether GT fee deduction is enabled | | »»
rebated_fee | string | Rebated fee | | »» rebated_fee_currency | string |
Rebated fee currency unit | | »» stp_id | integer | Orders between users in the
same `stp_id` group are not allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | »» stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevetion strategies

1\. After users join the `STP Group`, he can pass `stp_act` to limit the user's
self-trade prevetion strategy. If `stp_act` is not passed, the default is `cn`
strategy。  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter。  
3\. If the user did not use 'stp_act' when placing the order, 'stp_act' will
return '-'

\- cn: Cancel newest, Cancel new orders and keep old ones  
\- co: Cancel oldest, new ones  
\- cb: Cancel both, Both old and new orders will be cancelled | | »» finish_as |
string | How the order was finished.

\- open: processing  
\- filled: filled totally  
\- cancelled: manually cancelled  
\- ioc: time in force is `IOC`, finish immediately  
\- stp: cancelled because self trade prevention |

#### [#](#enumerated-values-15) Enumerated Values

| Property      | Value        |
| ------------- | ------------ |
| status        | open         |
| status        | closed       |
| status        | cancelled    |
| type          | limit        |
| type          | market       |
| account       | spot         |
| account       | margin       |
| account       | cross_margin |
| account       | unified      |
| side          | buy          |
| side          | sell         |
| time_in_force | gtc          |
| time_in_force | ioc          |
| time_in_force | poc          |
| time_in_force | fok          |
| stp_act       | cn           |
| stp_act       | co           |
| stp_act       | cb           |
| stp_act       | \-           |
| finish_as     | open         |
| finish_as     | filled       |
| finish_as     | cancelled    |
| finish_as     | ioc          |
| finish_as     | stp          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-all-open-orders) List all open orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-open-orders](https://www.gate.io/docs/developers/apiv4/en/#list-all-open-orders)

> Code samples

`GET /spot/open_orders`

_List all open orders_

Query the current order list of all trading pairs. Please note that the paging
parameter controls the number of pending orders in each trading pair. There is
no paging control trading pairs. All trading pairs with pending orders will be
returned.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-parameters)

| Name    | In    | Type           | Required | Description                                                          |
| ------- | ----- | -------------- | -------- | -------------------------------------------------------------------- |
| page    | query | integer(int32) | false    | Page number                                                          |
| limit   | query | integer        | false    | Maximum number of records returned in one page in each currency pair |
| account | query | string         | false    | Specify query account                                                |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "ETH_BTC",
    "total": 1,
    "orders": [
      {
        "id": "12332324",
        "text": "t-123456",
        "create_time": "1548000000",
        "update_time": "1548000100",
        "currency_pair": "ETH_BTC",
        "status": "open",
        "type": "limit",
        "account": "spot",
        "side": "buy",
        "amount": "1",
        "price": "5.00032",
        "time_in_force": "gtc",
        "left": "0.5",
        "filled_total": "2.50016",
        "fee": "0.005",
        "fee_currency": "ETH",
        "point_fee": "0",
        "gt_fee": "0",
        "gt_discount": false,
        "rebated_fee": "0",
        "rebated_fee_currency": "BTC"
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listallopenorders-responseschema)

Status Code **200**

| Name            | Type    | Description                                                           |
| --------------- | ------- | --------------------------------------------------------------------- |
| » currency_pair | string  | Trading pair                                                          |
| » total         | integer | Total number of open orders for this trading pair on the current page |
| » orders        | array   | none                                                                  |
| »» _None_       | object  | Spot order details                                                    |
| »»» id          | string  | Order ID                                                              |
| »»» text        | string  | User defined information. If not empty, must follow the rules below:  |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | »»» amend_text | string | The custom data that the user
remarked when amending the order | | »»» create_time | string | Creation time of
order | | »»» update_time | string | Last modification time of order | | »»»
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | |
»»» update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | »»» status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | »»» currency_pair | string | Currency pair | | »»»
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | »»» account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | »»» side |
string | Buy or sell order | | »»» amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | »»» price
| string | Trading price, required when `type`\=`limit` | | »»» time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | »»» iceberg |
string | Amount to display for the iceberg order. Null or 0 for normal orders.
Hiding all amount is not supported | | »»» auto_repay | boolean | Enable or
disable automatic repayment for automatic borrow loan generated by cross margin
order. Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »»»
left | string | Amount left to fill | | »»» filled_amount | string | Amount
filled | | »»» fill_price | string | Total filled in quote currency. Deprecated
in favor of `filled_total` | | »»» filled_total | string | Total filled in quote
currency | | »»» avg_deal_price | string | Average fill price | | »»» fee |
string | Fee deducted | | »»» fee_currency | string | Fee currency unit | | »»»
point_fee | string | Points used to deduct fee | | »»» gt_fee | string | GT used
to deduct fee | | »»» gt_maker_fee | string | GT amount used to deduct maker fee
| | »»» gt_taker_fee | string | GT amount used to deduct taker fee | | »»»
gt_discount | boolean | Whether GT fee deduction is enabled | | »»» rebated_fee
| string | Rebated fee | | »»» rebated_fee_currency | string | Rebated fee
currency unit | | »»» stp_id | integer | Orders between users in the same
`stp_id` group are not allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | »»» stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | »»» finish_as
| string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | »»» fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-16) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#close-position-when-cross-currency-is-disabled) Close position when cross-currency is disabled

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#close-position-when-cross-currency-is-disabled](https://www.gate.io/docs/developers/apiv4/en/#close-position-when-cross-currency-is-disabled)

> Code samples

`POST /spot/cross_liquidate_orders`

_Close position when cross-currency is disabled_

Currently, only cross-margin accounts are supported to place buy orders for
disabled currencies. Maximum buy quantity = (unpaid principal and interest -
currency balance - the amount of the currency in pending orders) / 0.998

> Body parameter

```
{
  "currency_pair": "GT_USDT",
  "amount": "12",
  "price": "10.15",
  "text": "t-34535"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-parameters)

| Name            | In   | Type   | Required | Description                                                                                                          |
| --------------- | ---- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------- |
| body            | body | object | true     | none                                                                                                                 |
| » text          | body | string | false    | Order custom information. Users can set custom ID with this field. Custom fields must meet the following conditions: |
| » currency_pair | body | string | true     | Currency pair                                                                                                        |
| » amount        | body | string | true     | Trade amount                                                                                                         |
| » price         | body | string | true     | Order price                                                                                                          |
| » action_mode   | body | string | false    | Processing mode:                                                                                                     |

#### [#](#detailed-descriptions-12) Detailed descriptions

**» text**: Order custom information. Users can set custom ID with this field.
Custom fields must meet the following conditions:

1.  Must start with `t-`
2.  Excluding `t-`, length cannot exceed 28 bytes
3.  Can only contain numbers, letters, underscore(\_), hyphen(-) or dot(.)

**» action_mode**: Processing mode:

Different fields are returned when placing an order based on action_mode. This
field is only valid during the request and is not included in the response
`ACK`: Asynchronous mode, only returns key order fields `RESULT`: No liquidation
information `FULL`: Full mode (default)

> Example responses

> 201 Response

```
{
  "id": "1852454420",
  "text": "t-abc123",
  "amend_text": "-",
  "create_time": "1710488334",
  "update_time": "1710488334",
  "create_time_ms": 1710488334073,
  "update_time_ms": 1710488334074,
  "status": "closed",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "left": "0",
  "filled_amount": "0.001",
  "fill_price": "63.4693",
  "filled_total": "63.4693",
  "avg_deal_price": "63469.3",
  "fee": "0.00000022",
  "fee_currency": "BTC",
  "point_fee": "0",
  "gt_fee": "0",
  "gt_maker_fee": "0",
  "gt_taker_fee": "0",
  "gt_discount": false,
  "rebated_fee": "0",
  "rebated_fee_currency": "USDT",
  "finish_as": "filled"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-responses)

| Status | Meaning                                                                         | Description                | Schema |
| ------ | ------------------------------------------------------------------------------- | -------------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createcrossliquidateorder-responseschema)

Status Code **201**

_Spot order details_

| Name   | Type   | Description                                                          |
| ------ | ------ | -------------------------------------------------------------------- |
| » id   | string | Order ID                                                             |
| » text | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | » amend_text | string | The custom data that the user
remarked when amending the order | | » create_time | string | Creation time of
order | | » update_time | string | Last modification time of order | | »
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | | »
update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | » status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | » currency_pair | string | Currency pair | | »
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | » account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | » side |
string | Buy or sell order | | » amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | » price |
string | Trading price, required when `type`\=`limit` | | » time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | » iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | » auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »
left | string | Amount left to fill | | » filled_amount | string | Amount filled
| | » fill_price | string | Total filled in quote currency. Deprecated in favor
of `filled_total` | | » filled_total | string | Total filled in quote currency |
| » avg_deal_price | string | Average fill price | | » fee | string | Fee
deducted | | » fee_currency | string | Fee currency unit | | » point_fee |
string | Points used to deduct fee | | » gt_fee | string | GT used to deduct fee
| | » gt_maker_fee | string | GT amount used to deduct maker fee | | »
gt_taker_fee | string | GT amount used to deduct taker fee | | » gt_discount |
boolean | Whether GT fee deduction is enabled | | » rebated_fee | string |
Rebated fee | | » rebated_fee_currency | string | Rebated fee currency unit | |
» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | » stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | » finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | » fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-17) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#create-an-order) Create an order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-an-order](https://www.gate.io/docs/developers/apiv4/en/#create-an-order)

> Code samples

`POST /spot/orders`

_Create an order_

Supports spot, margin, leverage, and cross-margin leverage orders. Use different
accounts through the `account` field. Default is `spot`, which means using the
spot account to place orders. If the user has a `unified` account, the default
is to place orders with the unified account.

When using leveraged account trading (i.e., when `account` is set to `margin`),
you can set `auto_borrow` to `true`. In case of insufficient account balance,
the system will automatically execute `POST /margin/uni/loans` to borrow the
insufficient amount. Whether assets obtained after leveraged order execution are
automatically used to repay borrowing orders of the isolated margin account
depends on the automatic repayment settings of the user's isolated margin
account. Account automatic repayment settings can be queried and set through
`/margin/auto_repay`.

When using unified account trading (i.e., when `account` is set to `unified`),
`auto_borrow` can also be enabled to realize automatic borrowing of insufficient
amounts. However, unlike the isolated margin account, whether unified account
orders are automatically repaid depends on the `auto_repay` setting when placing
the order. This setting only applies to the current order, meaning only assets
obtained after order execution will be used to repay borrowing orders of the
cross-margin account. Unified account ordering currently supports enabling both
`auto_borrow` and `auto_repay` simultaneously.

Auto repayment will be triggered when the order ends, i.e., when `status` is
`cancelled` or `closed`.

**Order Status**

The order status in pending orders is `open`, which remains `open` until all
quantity is filled. If fully filled, the order ends and status becomes `closed`.
If the order is cancelled before all transactions are completed, regardless of
partial fills, the status will become `cancelled`.

**Iceberg Orders**

`iceberg` is used to set the displayed quantity of iceberg orders and does not
support complete hiding. Note that hidden portions are charged according to the
taker's fee rate.

**Self-Trade Prevention**

Set `stp_act` to determine the self-trade prevention strategy to use

> Body parameter

```
{
  "text": "t-abc123",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createorder-parameters)

| Name            | In     | Type    | Required | Description                                                                                                                                      |
| --------------- | ------ | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime  | header | string  | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body            | body   | object  | true     | none                                                                                                                                             |
| » text          | body   | string  | false    | User defined information. If not empty, must follow the rules below:                                                                             |
| » currency_pair | body   | string  | true     | Currency pair                                                                                                                                    |
| » type          | body   | string  | false    | Order Type                                                                                                                                       |
| » account       | body   | string  | false    | Account type, spot - spot account, margin - leveraged account, unified - unified account                                                         |
| » side          | body   | string  | true     | Buy or sell order                                                                                                                                |
| » amount        | body   | string  | true     | Trading quantity                                                                                                                                 |
| » price         | body   | string  | false    | Trading price, required when `type`\=`limit`                                                                                                     |
| » time_in_force | body   | string  | false    | Time in force                                                                                                                                    |
| » iceberg       | body   | string  | false    | Amount to display for the iceberg order. Null or 0 for normal orders. Hiding all amount is not supported                                         |
| » auto_borrow   | body   | boolean | false    | Used in margin or cross margin trading to allow automatic loan of insufficient amount if balance is not enough                                   |
| » auto_repay    | body   | boolean | false    | Enable or disable automatic repayment for automatic borrow loan generated by cross margin order. Default is disabled. Note that:                 |
| » stp_act       | body   | string  | false    | Self-Trading Prevention Action. Users can use this field to set self-trade prevention strategies                                                 |
| » fee_discount  | body   | string  | false    | Fee rate discount                                                                                                                                |
| » action_mode   | body   | string  | false    | Processing Mode:                                                                                                                                 |

#### [#](#detailed-descriptions-13) Detailed descriptions

**» text**: User defined information. If not empty, must follow the rules below:

1.  prefixed with `t-`
2.  no longer than 28 bytes without `t-` prefix
3.  can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

- 101: from android
- 102: from IOS
- 103: from IPAD
- 104: from webapp
- 3: from web
- 2: from apiv2
- apiv4: from apiv4

**» type**: Order Type

- limit : Limit Order
- market : Market Order

**» amount**: Trading quantity When `type` is `limit`, it refers to the base
currency (the currency being traded), such as `BTC` in `BTC_USDT` When `type` is
`market`, it refers to different currencies based on the side:

- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`
- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC`

**» time_in_force**: Time in force

- gtc: GoodTillCancelled
- ioc: ImmediateOrCancelled, taker only
- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
  fee
- fok: FillOrKill, fill either completely or none Only `ioc` and `fok` are
  supported when `type`\=`market`

**» auto_repay**: Enable or disable automatic repayment for automatic borrow
loan generated by cross margin order. Default is disabled. Note that:

1.  This field is only effective for cross margin orders. Margin account does
    not support setting auto repayment for orders.
2.  `auto_borrow` and `auto_repay` can be both set to true in one order

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

**» action_mode**: Processing Mode: When placing an order, different fields are
returned based on action_mode. This field is only valid during the request and
is not included in the response result ACK: Asynchronous mode, only returns key
order fields RESULT: No clearing information FULL: Full mode (default)

#### [#](#enumerated-values-18) Enumerated Values

| Parameter       | Value  |
| --------------- | ------ |
| » type          | limit  |
| » type          | market |
| » side          | buy    |
| » side          | sell   |
| » time_in_force | gtc    |
| » time_in_force | ioc    |
| » time_in_force | poc    |
| » time_in_force | fok    |
| » stp_act       | cn     |
| » stp_act       | co     |
| » stp_act       | cb     |
| » stp_act       | \-     |

> Example responses

> ACK response body example

```
{
  "id": "12332324",
  "text": "t-123456",
  "amend_text": "test2"
}
```

> RESULT response body example

```
{
  "id": "12332324",
  "text": "t-123456",
  "create_time": "1548000000",
  "update_time": "1548000100",
  "create_time_ms": 1548000000123,
  "update_time_ms": 1548000100123,
  "currency_pair": "ETH_BTC",
  "status": "cancelled",
  "type": "limit",
  "account": "spot",
  "side": "buy",
  "iceberg": "0",
  "amount": "1",
  "price": "5.00032",
  "time_in_force": "gtc",
  "auto_borrow": false,
  "left": "0.5",
  "filled_total": "2.50016",
  "avg_deal_price": "5.00032",
  "stp_act": "cn",
  "finish_as": "stp",
  "stp_id": 10240
}
```

> FULL response body example

```
{
  "id": "1852454420",
  "text": "t-abc123",
  "amend_text": "-",
  "create_time": "1710488334",
  "update_time": "1710488334",
  "create_time_ms": 1710488334073,
  "update_time_ms": 1710488334074,
  "status": "closed",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "left": "0",
  "filled_amount": "0.001",
  "fill_price": "63.4693",
  "filled_total": "63.4693",
  "avg_deal_price": "63469.3",
  "fee": "0.00000022",
  "fee_currency": "BTC",
  "point_fee": "0",
  "gt_fee": "0",
  "gt_maker_fee": "0",
  "gt_taker_fee": "0",
  "gt_discount": false,
  "rebated_fee": "0",
  "rebated_fee_currency": "USDT",
  "finish_as": "filled"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createorder-responses)

| Status | Meaning                                                                         | Description   | Schema |
| ------ | ------------------------------------------------------------------------------- | ------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order created | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createorder-responseschema)

Status Code **201**

_Spot order details_

| Name   | Type   | Description                                                          |
| ------ | ------ | -------------------------------------------------------------------- |
| » id   | string | Order ID                                                             |
| » text | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | » amend_text | string | The custom data that the user
remarked when amending the order | | » create_time | string | Creation time of
order | | » update_time | string | Last modification time of order | | »
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | | »
update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | » status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | » currency_pair | string | Currency pair | | »
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | » account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | » side |
string | Buy or sell order | | » amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | » price |
string | Trading price, required when `type`\=`limit` | | » time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | » iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | » auto_borrow | boolean | Used in margin or
cross margin trading to allow automatic loan of insufficient amount if balance
is not enough | | » auto_repay | boolean | Enable or disable automatic repayment
for automatic borrow loan generated by cross margin order. Default is disabled.
Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »
left | string | Amount left to fill | | » filled_amount | string | Amount filled
| | » fill_price | string | Total filled in quote currency. Deprecated in favor
of `filled_total` | | » filled_total | string | Total filled in quote currency |
| » avg_deal_price | string | Average fill price | | » fee | string | Fee
deducted | | » fee_currency | string | Fee currency unit | | » point_fee |
string | Points used to deduct fee | | » gt_fee | string | GT used to deduct fee
| | » gt_maker_fee | string | GT amount used to deduct maker fee | | »
gt_taker_fee | string | GT amount used to deduct taker fee | | » gt_discount |
boolean | Whether GT fee deduction is enabled | | » rebated_fee | string |
Rebated fee | | » rebated_fee_currency | string | Rebated fee currency unit | |
» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | » stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | » finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | » fee_discount | string | Fee rate discount | | »
action_mode | string | Processing Mode:  
When placing an order, different fields are returned based on action_mode. This
field is only valid during the request and is not included in the response
result  
ACK: Asynchronous mode, only returns key order fields  
RESULT: No clearing information  
FULL: Full mode (default) |

#### [#](#enumerated-values-19) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-orders) List orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-orders](https://www.gate.io/docs/developers/apiv4/en/#list-orders)

> Code samples

`GET /spot/orders`

_List orders_

`status` is set to `open`, that is, when querying the pending order list, only
`page` and `limit` paging control are supported. `limit` The maximum setting is
only 100. Does not support `side` and `from`, `to` parameters for querying by
time range.

`status` when querying historical orders, in addition to paging query, it also
supports `from` and `to` query by time range. In addition, it also supports
setting the `side` parameter to filter unilateral history.

The parameters for time range filtering are all processed according to the
**end** time of the order.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listorders-parameters)

| Name          | In    | Type           | Required | Description                                                                                |
| ------------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------ |
| currency_pair | query | string         | true     | Query by specified currency pair. Required for open orders, optional for filled orders     |
| status        | query | string         | true     | List orders based on status                                                                |
| page          | query | integer(int32) | false    | Page number                                                                                |
| limit         | query | integer        | false    | Maximum number of records to be returned. If `status` is `open`, maximum of `limit` is 100 |
| account       | query | string         | false    | Specify query account                                                                      |
| from          | query | integer(int64) | false    | Start timestamp for the query                                                              |
| to            | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                     |
| side          | query | string         | false    | Specify all bids or all asks, both included if not specified                               |

#### [#](#detailed-descriptions-14) Detailed descriptions

**status**: List orders based on status

`open` - order is waiting to be filled `finished` - order has been filled or
cancelled

> Example responses

> 200 Response

```
[
  {
    "id": "1852454420",
    "text": "t-abc123",
    "amend_text": "-",
    "create_time": "1710488334",
    "update_time": "1710488334",
    "create_time_ms": 1710488334073,
    "update_time_ms": 1710488334074,
    "status": "closed",
    "currency_pair": "BTC_USDT",
    "type": "limit",
    "account": "unified",
    "side": "buy",
    "amount": "0.001",
    "price": "65000",
    "time_in_force": "gtc",
    "iceberg": "0",
    "left": "0",
    "filled_amount": "0.001",
    "fill_price": "63.4693",
    "filled_total": "63.4693",
    "avg_deal_price": "63469.3",
    "fee": "0.00000022",
    "fee_currency": "BTC",
    "point_fee": "0",
    "gt_fee": "0",
    "gt_maker_fee": "0",
    "gt_taker_fee": "0",
    "gt_discount": false,
    "rebated_fee": "0",
    "rebated_fee_currency": "USDT",
    "finish_as": "filled"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listorders-responseschema)

Status Code **200**

| Name     | Type   | Description                                                          |
| -------- | ------ | -------------------------------------------------------------------- |
| _None_   | array  | \[Spot order details\]                                               |
| » _None_ | object | Spot order details                                                   |
| »» id    | string | Order ID                                                             |
| »» text  | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | »» amend_text | string | The custom data that the user
remarked when amending the order | | »» create_time | string | Creation time of
order | | »» update_time | string | Last modification time of order | | »»
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | |
»» update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | »» status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | »» currency_pair | string | Currency pair | | »»
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | »» account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | »» side |
string | Buy or sell order | | »» amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | »» price |
string | Trading price, required when `type`\=`limit` | | »» time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | »» iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | »» auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »»
left | string | Amount left to fill | | »» filled_amount | string | Amount
filled | | »» fill_price | string | Total filled in quote currency. Deprecated
in favor of `filled_total` | | »» filled_total | string | Total filled in quote
currency | | »» avg_deal_price | string | Average fill price | | »» fee | string
| Fee deducted | | »» fee_currency | string | Fee currency unit | | »» point_fee
| string | Points used to deduct fee | | »» gt_fee | string | GT used to deduct
fee | | »» gt_maker_fee | string | GT amount used to deduct maker fee | | »»
gt_taker_fee | string | GT amount used to deduct taker fee | | »» gt_discount |
boolean | Whether GT fee deduction is enabled | | »» rebated_fee | string |
Rebated fee | | »» rebated_fee_currency | string | Rebated fee currency unit | |
»» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | »» stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | »» finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | »» fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-20) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-open-orders-in-specified-currency-pair) Cancel all `open` orders in specified currency pair

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-open-orders-in-specified-currency-pair](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-open-orders-in-specified-currency-pair)

> Code samples

`DELETE /spot/orders`

_Cancel all `open` orders in specified currency pair_

When the `account` parameter is not specified, all pending orders including
spot, unified account, and isolated margin will be cancelled. When
`currency_pair` is not specified, all trading pair pending orders will be
cancelled. You can specify a particular account to cancel all pending orders
under that account

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelorders-parameters)

| Name           | In     | Type   | Required | Description                                                                                                                                      |
| -------------- | ------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| currency_pair  | query  | string | false    | Trading pair                                                                                                                                     |
| side           | query  | string | false    | Specify all bids or all asks, both included if not specified                                                                                     |
| account        | query  | string | false    | Specify account type                                                                                                                             |
| action_mode    | query  | string | false    | Processing Mode                                                                                                                                  |
| x-gate-exptime | header | string | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |

#### [#](#detailed-descriptions-15) Detailed descriptions

**account**: Specify account type

Classic account: All are included if not specified Unified account: Specify
`unified`

**action_mode**: Processing Mode

When placing an order, different fields are returned based on the action_mode

- `ACK`: Asynchronous mode, returns only key order fields
- `RESULT`: No clearing information
- `FULL`: Full mode (default)

> Example responses

> 200 Response

```
[
  {
    "id": "1852454420",
    "text": "t-abc123",
    "amend_text": "-",
    "succeeded": true,
    "create_time": "1710488334",
    "update_time": "1710488334",
    "create_time_ms": 1710488334073,
    "update_time_ms": 1710488334074,
    "status": "closed",
    "currency_pair": "BTC_USDT",
    "type": "limit",
    "account": "unified",
    "side": "buy",
    "amount": "0.001",
    "price": "65000",
    "time_in_force": "gtc",
    "iceberg": "0",
    "left": "0",
    "filled_amount": "0.001",
    "fill_price": "63.4693",
    "filled_total": "63.4693",
    "avg_deal_price": "63469.3",
    "fee": "0.00000022",
    "fee_currency": "BTC",
    "point_fee": "0",
    "gt_fee": "0",
    "gt_maker_fee": "0",
    "gt_taker_fee": "0",
    "gt_discount": false,
    "rebated_fee": "0",
    "rebated_fee_currency": "USDT",
    "finish_as": "filled"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorders-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelorders-responses)

| Status | Meaning                                                                    | Description                                                                         | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation request accepted and processed, success determined by order list | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#cancelorders-responseschema)

Status Code **200**

| Name     | Type   | Description                                                          |
| -------- | ------ | -------------------------------------------------------------------- |
| » _None_ | object | Spot order details                                                   |
| »» id    | string | Order ID                                                             |
| »» text  | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | »» amend_text | string | The custom data that the user
remarked when amending the order | | »» succeeded | boolean | Request execution
result | | »» label | string | Error label, if any, otherwise an empty string |
| »» message | string | Detailed error message, if any, otherwise an empty
string | | »» create_time | string | Creation time of order | | »» update_time |
string | Last modification time of order | | »» create_time_ms | integer(int64)
| Creation time of order (in milliseconds) | | »» update_time_ms |
integer(int64) | Last modification time of order (in milliseconds) | | »» status
| string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | »» currency_pair | string | Currency pair | | »»
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | »» account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | »» side |
string | Buy or sell order | | »» amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | »» price |
string | Trading price, required when `type`\=`limit` | | »» time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | »» iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | »» auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »»
left | string | Amount left to fill | | »» filled_amount | string | Amount
filled | | »» fill_price | string | Total filled in quote currency. Deprecated
in favor of `filled_total` | | »» filled_total | string | Total filled in quote
currency | | »» avg_deal_price | string | Average fill price | | »» fee | string
| Fee deducted | | »» fee_currency | string | Fee currency unit | | »» point_fee
| string | Points used to deduct fee | | »» gt_fee | string | GT used to deduct
fee | | »» gt_maker_fee | string | GT amount used to deduct maker fee | | »»
gt_taker_fee | string | GT amount used to deduct taker fee | | »» gt_discount |
boolean | Whether GT fee deduction is enabled | | »» rebated_fee | string |
Rebated fee | | »» rebated_fee_currency | string | Rebated fee currency unit | |
»» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | »» stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | »» finish_as |
string | How the order was finished.

\- open: processing  
\- filled: filled totally  
\- cancelled: manually cancelled  
\- ioc: time in force is `IOC`, finish immediately  
\- stp: cancelled because self trade prevention | | »» fee_discount | string |
Fee rate discount |

#### [#](#enumerated-values-21) Enumerated Values

| Property      | Value     |
| ------------- | --------- |
| status        | open      |
| status        | closed    |
| status        | cancelled |
| type          | limit     |
| type          | market    |
| side          | buy       |
| side          | sell      |
| time_in_force | gtc       |
| time_in_force | ioc       |
| time_in_force | poc       |
| time_in_force | fok       |
| stp_act       | cn        |
| stp_act       | co        |
| stp_act       | cb        |
| stp_act       | \-        |
| finish_as     | open      |
| finish_as     | filled    |
| finish_as     | cancelled |
| finish_as     | ioc       |
| finish_as     | stp       |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-batch-orders-by-specified-id-list) Cancel batch orders by specified ID list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-batch-orders-by-specified-id-list](https://www.gate.io/docs/developers/apiv4/en/#cancel-batch-orders-by-specified-id-list)

> Code samples

`POST /spot/cancel_batch_orders`

_Cancel batch orders by specified ID list_

Multiple currency pairs can be specified, but maximum 20 orders are allowed per
request

> Body parameter

```
[
  {
    "currency_pair": "BTC_USDT",
    "id": "123456"
  }
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-parameters)

| Name           | In     | Type            | Required | Description                                                                                                                                      |
| -------------- | ------ | --------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string          | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array\[object\] | true     | none                                                                                                                                             |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "BTC_USDT",
    "id": "123456",
    "text": "123456",
    "succeeded": true,
    "label": null,
    "message": null
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-responses)

| Status | Meaning                                                                    | Description                  | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation completed | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#cancelbatchorders-responseschema)

Status Code **200**

| Name                | Type    | Description                                                     |
| ------------------- | ------- | --------------------------------------------------------------- |
| » CancelOrderResult | object  | Order cancellation result                                       |
| »» currency_pair    | string  | Order currency pair                                             |
| »» id               | string  | Order ID                                                        |
| »» text             | string  | Custom order information                                        |
| »» succeeded        | boolean | Whether cancellation succeeded                                  |
| »» label            | string  | Error label when failed to cancel the order; emtpy if succeeded |
| »» message          | string  | Error description when cancellation fails, empty if successful  |
| »» account          | string  | Default is empty (deprecated)                                   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-order-details) Query single order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details)

> Code samples

`GET /spot/orders/{order_id}`

_Query single order details_

By default, queries orders for spot, unified account, and isolated margin
accounts.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getorder-parameters)

| Name          | In    | Type   | Required | Description                                                                                                                                                  |
| ------------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| order_id      | path  | string | true     | The order ID returned when the order was successfully created or the custom ID specified by the user's creation (i.e. the `text` field).                     |
| currency_pair | query | string | true     | Specify the trading pair to query. This field is required when querying pending order records. This field can be omitted when querying filled order records. |
| account       | query | string | false    | Specify query account                                                                                                                                        |

#### [#](#detailed-descriptions-16) Detailed descriptions

**order_id**: The order ID returned when the order was successfully created or
the custom ID specified by the user's creation (i.e. the `text` field).
Operations based on custom IDs can only be checked in pending orders. Only order
ID can be used after the order is finished (transaction/cancel)

> Example responses

> 200 Response

```
{
  "id": "1852454420",
  "text": "t-abc123",
  "amend_text": "-",
  "create_time": "1710488334",
  "update_time": "1710488334",
  "create_time_ms": 1710488334073,
  "update_time_ms": 1710488334074,
  "status": "closed",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "left": "0",
  "filled_amount": "0.001",
  "fill_price": "63.4693",
  "filled_total": "63.4693",
  "avg_deal_price": "63469.3",
  "fee": "0.00000022",
  "fee_currency": "BTC",
  "point_fee": "0",
  "gt_fee": "0",
  "gt_maker_fee": "0",
  "gt_taker_fee": "0",
  "gt_discount": false,
  "rebated_fee": "0",
  "rebated_fee_currency": "USDT",
  "finish_as": "filled"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getorder-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Detail retrieved | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getorder-responseschema)

Status Code **200**

_Spot order details_

| Name   | Type   | Description                                                          |
| ------ | ------ | -------------------------------------------------------------------- |
| » id   | string | Order ID                                                             |
| » text | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | » amend_text | string | The custom data that the user
remarked when amending the order | | » create_time | string | Creation time of
order | | » update_time | string | Last modification time of order | | »
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | | »
update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | » status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | » currency_pair | string | Currency pair | | »
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | » account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | » side |
string | Buy or sell order | | » amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | » price |
string | Trading price, required when `type`\=`limit` | | » time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | » iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | » auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »
left | string | Amount left to fill | | » filled_amount | string | Amount filled
| | » fill_price | string | Total filled in quote currency. Deprecated in favor
of `filled_total` | | » filled_total | string | Total filled in quote currency |
| » avg_deal_price | string | Average fill price | | » fee | string | Fee
deducted | | » fee_currency | string | Fee currency unit | | » point_fee |
string | Points used to deduct fee | | » gt_fee | string | GT used to deduct fee
| | » gt_maker_fee | string | GT amount used to deduct maker fee | | »
gt_taker_fee | string | GT amount used to deduct taker fee | | » gt_discount |
boolean | Whether GT fee deduction is enabled | | » rebated_fee | string |
Rebated fee | | » rebated_fee_currency | string | Rebated fee currency unit | |
» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | » stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | » finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | » fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-22) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#amend-single-order) Amend single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amend-single-order](https://www.gate.io/docs/developers/apiv4/en/#amend-single-order)

> Code samples

`PATCH /spot/orders/{order_id}`

_Amend single order_

Modify orders in spot, unified account and isolated margin account by default.

Currently both request body and query support currency_pair and account
parameters, but request body has higher priority.

currency_pair must be filled in one of the request body or query parameters.

About rate limit: Order modification and order creation share the same rate
limit rules.

About matching priority: Only reducing the quantity does not affect the matching
priority. Modifying the price or increasing the quantity will adjust the
priority to the end of the new price level.

Note: Modifying the quantity to be less than the filled quantity will trigger a
cancellation and isolated margin account by default.

Currently both request body and query support currency_pair and account
parameters, but request body has higher priority.

currency_pair must be filled in one of the request body or query parameters.

About rate limit: Order modification and order creation share the same rate
limit rules.

About matching priority: Only reducing the quantity does not affect the matching
priority. Modifying the price or increasing the quantity will adjust the
priority to the end of the new price level.

Note: Modifying the quantity to be less than the filled quantity will trigger a
cancellation operation.

> Body parameter

```
{
  "currency_pair": "BTC_USDT",
  "account": "spot",
  "amount": "1"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#amendorder-parameters)

| Name            | In     | Type   | Required | Description                                                                                                                                      |
| --------------- | ------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| order_id        | path   | string | true     | The order ID returned when the order was successfully created or the custom ID specified by the user's creation (i.e. the `text` field).         |
| currency_pair   | query  | string | false    | Trading pair                                                                                                                                     |
| account         | query  | string | false    | Specify query account                                                                                                                            |
| x-gate-exptime  | header | string | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body            | body   | object | true     | none                                                                                                                                             |
| » currency_pair | body   | string | false    | Trading pair                                                                                                                                     |
| » account       | body   | string | false    | Specify query account                                                                                                                            |
| » amount        | body   | string | false    | Trading quantity. Either `amount` or `price` must be specified                                                                                   |
| » price         | body   | string | false    | Trading price. Either `amount` or `price` must be specified                                                                                      |
| » amend_text    | body   | string | false    | Custom info during order amendment                                                                                                               |
| » action_mode   | body   | string | false    | Processing Mode:                                                                                                                                 |

#### [#](#detailed-descriptions-17) Detailed descriptions

**order_id**: The order ID returned when the order was successfully created or
the custom ID specified by the user's creation (i.e. the `text` field).
Operations based on custom IDs can only be checked in pending orders. Only order
ID can be used after the order is finished (transaction/cancel)

**» action_mode**: Processing Mode: When placing an order, different fields are
returned based on action_mode. This field is only valid during the request and
is not included in the response result ACK: Asynchronous mode, only returns key
order fields RESULT: No clearing information FULL: Full mode (default)

> Example responses

> 200 Response

```
{
  "id": "1852454420",
  "text": "t-abc123",
  "amend_text": "-",
  "create_time": "1710488334",
  "update_time": "1710488334",
  "create_time_ms": 1710488334073,
  "update_time_ms": 1710488334074,
  "status": "closed",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "left": "0",
  "filled_amount": "0.001",
  "fill_price": "63.4693",
  "filled_total": "63.4693",
  "avg_deal_price": "63469.3",
  "fee": "0.00000022",
  "fee_currency": "BTC",
  "point_fee": "0",
  "gt_fee": "0",
  "gt_maker_fee": "0",
  "gt_taker_fee": "0",
  "gt_discount": false,
  "rebated_fee": "0",
  "rebated_fee_currency": "USDT",
  "finish_as": "filled"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendorder-responses](https://www.gate.io/docs/developers/apiv4/en/#amendorder-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Updated successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#amendorder-responseschema)

Status Code **200**

_Spot order details_

| Name   | Type   | Description                                                          |
| ------ | ------ | -------------------------------------------------------------------- |
| » id   | string | Order ID                                                             |
| » text | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | » amend_text | string | The custom data that the user
remarked when amending the order | | » create_time | string | Creation time of
order | | » update_time | string | Last modification time of order | | »
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | | »
update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | » status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | » currency_pair | string | Currency pair | | »
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | » account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | » side |
string | Buy or sell order | | » amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | » price |
string | Trading price, required when `type`\=`limit` | | » time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | » iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | » auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »
left | string | Amount left to fill | | » filled_amount | string | Amount filled
| | » fill_price | string | Total filled in quote currency. Deprecated in favor
of `filled_total` | | » filled_total | string | Total filled in quote currency |
| » avg_deal_price | string | Average fill price | | » fee | string | Fee
deducted | | » fee_currency | string | Fee currency unit | | » point_fee |
string | Points used to deduct fee | | » gt_fee | string | GT used to deduct fee
| | » gt_maker_fee | string | GT amount used to deduct maker fee | | »
gt_taker_fee | string | GT amount used to deduct taker fee | | » gt_discount |
boolean | Whether GT fee deduction is enabled | | » rebated_fee | string |
Rebated fee | | » rebated_fee_currency | string | Rebated fee currency unit | |
» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | » stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | » finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | » fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-23) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-order) Cancel single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order)

> Code samples

`DELETE /spot/orders/{order_id}`

_Cancel single order_

By default, orders for spot, unified accounts and leveraged accounts are
revoked.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelorder-parameters)

| Name           | In     | Type   | Required | Description                                                                                                                                      |
| -------------- | ------ | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| order_id       | path   | string | true     | The order ID returned when the order was successfully created or the custom ID specified by the user's creation (i.e. the `text` field).         |
| currency_pair  | query  | string | true     | Trading pair                                                                                                                                     |
| account        | query  | string | false    | Specify query account                                                                                                                            |
| action_mode    | query  | string | false    | Processing Mode                                                                                                                                  |
| x-gate-exptime | header | string | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |

#### [#](#detailed-descriptions-18) Detailed descriptions

**order_id**: The order ID returned when the order was successfully created or
the custom ID specified by the user's creation (i.e. the `text` field).
Operations based on custom IDs can only be checked in pending orders. Only order
ID can be used after the order is finished (transaction/cancel)

**action_mode**: Processing Mode

When placing an order, different fields are returned based on the action_mode

- `ACK`: Asynchronous mode, returns only key order fields
- `RESULT`: No clearing information
- `FULL`: Full mode (default)

> Example responses

> 200 Response

```
{
  "id": "1852454420",
  "text": "t-abc123",
  "amend_text": "-",
  "create_time": "1710488334",
  "update_time": "1710488334",
  "create_time_ms": 1710488334073,
  "update_time_ms": 1710488334074,
  "status": "closed",
  "currency_pair": "BTC_USDT",
  "type": "limit",
  "account": "unified",
  "side": "buy",
  "amount": "0.001",
  "price": "65000",
  "time_in_force": "gtc",
  "iceberg": "0",
  "left": "0",
  "filled_amount": "0.001",
  "fill_price": "63.4693",
  "filled_total": "63.4693",
  "avg_deal_price": "63469.3",
  "fee": "0.00000022",
  "fee_currency": "BTC",
  "point_fee": "0",
  "gt_fee": "0",
  "gt_maker_fee": "0",
  "gt_taker_fee": "0",
  "gt_discount": false,
  "rebated_fee": "0",
  "rebated_fee_currency": "USDT",
  "finish_as": "filled"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorder-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelorder-responses)

| Status | Meaning                                                                    | Description     | Schema |
| ------ | -------------------------------------------------------------------------- | --------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order cancelled | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#cancelorder-responseschema)

Status Code **200**

_Spot order details_

| Name   | Type   | Description                                                          |
| ------ | ------ | -------------------------------------------------------------------- |
| » id   | string | Order ID                                                             |
| » text | string | User defined information. If not empty, must follow the rules below: |

1\. prefixed with `t-`  
2\. no longer than 28 bytes without `t-` prefix  
3\. can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.)

Besides user defined information, reserved contents are listed below, denoting
how the order is created:

\- 101: from android  
\- 102: from IOS  
\- 103: from IPAD  
\- 104: from webapp  
\- 3: from web  
\- 2: from apiv2  
\- apiv4: from apiv4 | | » amend_text | string | The custom data that the user
remarked when amending the order | | » create_time | string | Creation time of
order | | » update_time | string | Last modification time of order | | »
create_time_ms | integer(int64) | Creation time of order (in milliseconds) | | »
update_time_ms | integer(int64) | Last modification time of order (in
milliseconds) | | » status | string | Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | » currency_pair | string | Currency pair | | »
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | » account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | » side |
string | Buy or sell order | | » amount | string | Trading quantity  
When `type` is `limit`, it refers to the base currency (the currency being
traded), such as `BTC` in `BTC_USDT`  
When `type` is `market`, it refers to different currencies based on the side:  
\- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`  
\- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC` | | » price |
string | Trading price, required when `type`\=`limit` | | » time_in_force |
string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none  
Only `ioc` and `fok` are supported when `type`\=`market` | | » iceberg | string
| Amount to display for the iceberg order. Null or 0 for normal orders. Hiding
all amount is not supported | | » auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »
left | string | Amount left to fill | | » filled_amount | string | Amount filled
| | » fill_price | string | Total filled in quote currency. Deprecated in favor
of `filled_total` | | » filled_total | string | Total filled in quote currency |
| » avg_deal_price | string | Average fill price | | » fee | string | Fee
deducted | | » fee_currency | string | Fee currency unit | | » point_fee |
string | Points used to deduct fee | | » gt_fee | string | GT used to deduct fee
| | » gt_maker_fee | string | GT amount used to deduct maker fee | | »
gt_taker_fee | string | GT amount used to deduct taker fee | | » gt_discount |
boolean | Whether GT fee deduction is enabled | | » rebated_fee | string |
Rebated fee | | » rebated_fee_currency | string | Rebated fee currency unit | |
» stp_id | integer | Orders between users in the same `stp_id` group are not
allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | » stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevention strategies

1\. After users join the `STP Group`, they can pass `stp_act` to limit the
user's self-trade prevention strategy. If `stp_act` is not passed, the default
is `cn` strategy.  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter.  
3\. If the user did not use `stp_act` when placing the order, `stp_act` will
return '-'

\- cn: Cancel newest, cancel new orders and keep old ones  
\- co: Cancel oldest, cancel old orders and keep new ones  
\- cb: Cancel both, both old and new orders will be cancelled | | » finish_as |
string | Order completion statuses include:

\- open: Awaiting processing  
\- filled: Fully filled  
\- cancelled: Cancelled by user  
\- liquidate_cancelled: Cancelled due to liquidation  
\- small: Order quantity too small  
\- depth_not_enough: Cancelled due to insufficient market depth  
\- trader_not_enough: Cancelled due to insufficient counterparty  
\- ioc: Not immediately filled because tif is set to ioc  
\- poc: Not met the order poc  
\- fok: Not fully filled immediately because tif is set to fok  
\- stp: Cancelled due to self-trade prevention  
\- unknown: Unknown | | » fee_discount | string | Fee rate discount |

#### [#](#enumerated-values-24) Enumerated Values

| Property      | Value               |
| ------------- | ------------------- |
| status        | open                |
| status        | closed              |
| status        | cancelled           |
| type          | limit               |
| type          | market              |
| side          | buy                 |
| side          | sell                |
| time_in_force | gtc                 |
| time_in_force | ioc                 |
| time_in_force | poc                 |
| time_in_force | fok                 |
| stp_act       | cn                  |
| stp_act       | co                  |
| stp_act       | cb                  |
| stp_act       | \-                  |
| finish_as     | open                |
| finish_as     | filled              |
| finish_as     | cancelled           |
| finish_as     | liquidate_cancelled |
| finish_as     | depth_not_enough    |
| finish_as     | trader_not_enough   |
| finish_as     | small               |
| finish_as     | ioc                 |
| finish_as     | poc                 |
| finish_as     | fok                 |
| finish_as     | stp                 |
| finish_as     | unknown             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-records) Query personal trading records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records)

> Code samples

`GET /spot/my_trades`

_Query personal trading records_

By default query of transaction records for spot, unified account and
warehouse-by-site leverage accounts.

The history within a specified time range can be queried by specifying `from` or
(and) `to`.

- If no time parameters are specified, only data for the last 7 days can be
  obtained.
- If only any parameter of `from` or `to` is specified, only 7-day data from the
  start (or end) of the specified time is returned.
- The range not allowed to exceed 30 days.

The parameters of the time range filter are processed according to the order end
time.

The maximum number of pages when searching data using limit&page paging function
is 100,0, that is, limit \* (page - 1) <= 100,0.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmytrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmytrades-parameters)

| Name          | In    | Type           | Required | Description                                                                                      |
| ------------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------ |
| currency_pair | query | string         | false    | Retrieve results with specified currency pair                                                    |
| limit         | query | integer        | false    | Maximum number of items returned in list. Default: 100, minimum: 1, maximum: 1000                |
| page          | query | integer(int32) | false    | Page number                                                                                      |
| order_id      | query | string         | false    | Filter trades with specified order ID. `currency_pair` is also required if this field is present |
| account       | query | string         | false    | Specify query account                                                                            |
| from          | query | integer(int64) | false    | Start timestamp for the query                                                                    |
| to            | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                           |

> Example responses

> 200 Response

```
[
  {
    "id": "1232893232",
    "create_time": "1548000000",
    "create_time_ms": "1548000000123.456",
    "order_id": "4128442423",
    "side": "buy",
    "role": "maker",
    "amount": "0.15",
    "price": "0.03",
    "fee": "0.0005",
    "fee_currency": "ETH",
    "point_fee": "0",
    "gt_fee": "0",
    "sequence_id": "588018",
    "text": "t-test"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmytrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listmytrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmytrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmytrades-responseschema)

Status Code **200**

| Name                                                     | Type   | Description                                                    |
| -------------------------------------------------------- | ------ | -------------------------------------------------------------- |
| _None_                                                   | array  | none                                                           |
| » id                                                     | string | Fill ID                                                        |
| » create_time                                            | string | Fill Time                                                      |
| » create_time_ms                                         | string | Trading time, with millisecond precision                       |
| » currency_pair                                          | string | Currency pair                                                  |
| » side                                                   | string | Buy or sell order                                              |
| » role                                                   | string | Trade role, not returned in public endpoints                   |
| » amount                                                 | string | Trade amount                                                   |
| » price                                                  | string | Order price                                                    |
| » order_id                                               | string | Related order ID, not returned in public endpoints             |
| » fee                                                    | string | Fee deducted, not returned in public endpoints                 |
| » fee_currency                                           | string | Fee currency unit, not returned in public endpoints            |
| » point_fee                                              | string | Points used to deduct fee, not returned in public endpoints    |
| » gt_fee                                                 | string | GT used to deduct fee, not returned in public endpoints        |
| » amend_text                                             | string | The custom data that the user remarked when amending the order |
| » sequence_id                                            | string | Consecutive trade ID within a single market.                   |
| Used to track and identify trades in the specific market |
| » text                                                   | string | User-defined information, not returned in public endpoints     |

#### [#](#enumerated-values-25) Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | buy   |
| side     | sell  |
| role     | taker |
| role     | maker |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-server-current-time) Get server current time

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-server-current-time](https://www.gate.io/docs/developers/apiv4/en/#get-server-current-time)

> Code samples

`GET /spot/time`

_Get server current time_

> Example responses

> 200 Response

```
{
  "server_time": 1597026383085
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsystemtime-responses](https://www.gate.io/docs/developers/apiv4/en/#getsystemtime-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsystemtime-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getsystemtime-responseschema)

Status Code **200**

_SystemTime_

| Name          | Type           | Description             |
| ------------- | -------------- | ----------------------- |
| » server_time | integer(int64) | Server current time(ms) |

This operation does not require authentication

## [#](#countdown-cancel-orders) Countdown cancel orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders](https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders)

> Code samples

`POST /spot/countdown_cancel_all`

_Countdown cancel orders_

Spot order heartbeat detection. If there is no "cancel existing countdown" or
"set new countdown" when the user-set `timeout` time is reached, the related
`spot pending orders` will be automatically cancelled. This interface can be
called repeatedly to set a new countdown or cancel the countdown. Usage example:
Repeat this interface at 30s intervals, setting the countdown `timeout` to
`30 (seconds)` each time. If this interface is not called again within 30
seconds, all pending orders on the `market` you specified will be automatically
cancelled. If no `market` is specified, all market cancelled. If the `timeout`
is set to 0 within 30 seconds, the countdown timer will be terminated and the
automatic order cancellation function will be cancelled.

> Body parameter

```
{
  "timeout": 30,
  "currency_pair": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-parameters](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-parameters)

| Name            | In   | Type           | Required | Description               |
| --------------- | ---- | -------------- | -------- | ------------------------- |
| body            | body | object         | true     | none                      |
| » timeout       | body | integer(int32) | true     | Countdown time in seconds |
| » currency_pair | body | string         | false    | Currency pair             |

#### [#](#detailed-descriptions-19) Detailed descriptions

**» timeout**: Countdown time in seconds At least 5 seconds, 0 means cancel
countdown

> Example responses

> 200 Response

```
{
  "triggerTime": "1660039145000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-responses](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-responses)

| Status | Meaning                                                                    | Description                | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Countdown set successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-responseschema](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelallspot-responseschema)

Status Code **200**

_triggerTime_

| Name          | Type           | Description                                    |
| ------------- | -------------- | ---------------------------------------------- |
| » triggerTime | integer(int64) | Timestamp when countdown ends, in milliseconds |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-modification-of-orders) Batch modification of orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-modification-of-orders](https://www.gate.io/docs/developers/apiv4/en/#batch-modification-of-orders)

> Code samples

`POST /spot/amend_batch_orders`

_Batch modification of orders_

Modify orders in spot, unified account and isolated margin account by default.
Modify uncompleted orders, up to 5 orders can be modified at a time. Request
parameters should be passed in array format. If there are order modification
failures during the batch modification process, the modification of the next
order will continue to be executed, and the execution will return with the
corresponding order failure information. The call order of batch modification
orders is consistent with the order list order. The return content order of
batch modification orders is consistent with the order list order.

> Body parameter

```
[
  {
    "order_id": "121212",
    "currency_pair": "BTC_USDT",
    "account": "spot",
    "amount": "1",
    "amend_text": "test"
  }
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-parameters)

| Name           | In     | Type            | Required | Description                                                                                                                                      |
| -------------- | ------ | --------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| x-gate-exptime | header | string          | false    | Specify the expiration time (milliseconds); if the GATE receives the request time greater than the expiration time, the request will be rejected |
| body           | body   | array\[object\] | true     | none                                                                                                                                             |

> Example responses

> 200 Response

```
[
  {
    "order_id": "12332324",
    "amend_text": "t-123456",
    "text": "t-123456",
    "succeeded": true,
    "label": "",
    "message": "",
    "id": "12332324",
    "create_time": "1548000000",
    "update_time": "1548000100",
    "create_time_ms": 1548000000123,
    "update_time_ms": 1548000100123,
    "currency_pair": "ETC_BTC",
    "status": "cancelled",
    "type": "limit",
    "account": "spot",
    "side": "buy",
    "amount": "1",
    "price": "5.00032",
    "time_in_force": "gtc",
    "iceberg": "0",
    "left": "0.5",
    "filled_amount": "1.242",
    "filled_total": "2.50016",
    "avg_deal_price": "5.00032",
    "fee": "0.005",
    "fee_currency": "ETH",
    "point_fee": "0",
    "gt_fee": "0",
    "gt_discount": false,
    "rebated_fee": "0",
    "rebated_fee_currency": "BTC",
    "stp_act": "cn",
    "finish_as": "stp",
    "stp_id": 10240
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-responses](https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-responses)

| Status | Meaning                                                                    | Description                              | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order modification executed successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#amendbatchorders-responseschema)

Status Code **200**

| Name          | Type   | Description                                                                                                          |
| ------------- | ------ | -------------------------------------------------------------------------------------------------------------------- |
| _None_        | array  | \[Batch order details\]                                                                                              |
| » _None_      | object | Batch order details                                                                                                  |
| »» order_id   | string | Order ID                                                                                                             |
| »» amend_text | string | The custom data that the user remarked when amending the order                                                       |
| »» text       | string | Order custom information. Users can set custom ID with this field. Custom fields must meet the following conditions: |

1\. Must start with `t-`  
2\. Excluding `t-`, length cannot exceed 28 bytes  
3\. Can only contain numbers, letters, underscore(\_), hyphen(-) or dot(.) | |
»» succeeded | boolean | Request execution result | | »» label | string | Error
label, if any, otherwise an empty string | | »» message | string | Detailed
error message, if any, otherwise an empty string | | »» id | string | Order ID |
| »» create_time | string | Creation time of order | | »» update_time | string |
Last modification time of order | | »» create_time_ms | integer(int64) |
Creation time of order (in milliseconds) | | »» update_time_ms | integer(int64)
| Last modification time of order (in milliseconds) | | »» status | string |
Order status

\- `open`: to be filled  
\- `closed`: filled  
\- `cancelled`: cancelled | | »» currency_pair | string | Currency pair | | »»
type | string | Order Type

\- limit : Limit Order  
\- market : Market Order | | »» account | string | Account type, spot - spot
account, margin - leveraged account, unified - unified account | | »» side |
string | Buy or sell order | | »» amount | string | Trade amount | | »» price |
string | Order price | | »» time_in_force | string | Time in force

\- gtc: GoodTillCancelled  
\- ioc: ImmediateOrCancelled, taker only  
\- poc: PendingOrCancelled, makes a post-only order that always enjoys a maker
fee  
\- fok: FillOrKill, fill either completely or none | | »» iceberg | string |
Amount to display for the iceberg order. Null or 0 for normal orders. Hiding all
amount is not supported | | »» auto_repay | boolean | Enable or disable
automatic repayment for automatic borrow loan generated by cross margin order.
Default is disabled. Note that:

1\. This field is only effective for cross margin orders. Margin account does
not support setting auto repayment for orders.  
2\. `auto_borrow` and `auto_repay` can be both set to true in one order | | »»
left | string | Amount left to fill | | »» filled_amount | string | Amount
filled | | »» fill_price | string | Total filled in quote currency. Deprecated
in favor of `filled_total` | | »» filled_total | string | Total filled in quote
currency | | »» avg_deal_price | string | Average fill price | | »» fee | string
| Fee deducted | | »» fee_currency | string | Fee currency unit | | »» point_fee
| string | Points used to deduct fee | | »» gt_fee | string | GT used to deduct
fee | | »» gt_discount | boolean | Whether GT fee deduction is enabled | | »»
rebated_fee | string | Rebated fee | | »» rebated_fee_currency | string |
Rebated fee currency unit | | »» stp_id | integer | Orders between users in the
same `stp_id` group are not allowed to be self-traded

1\. If the `stp_id` of two orders being matched is non-zero and equal, they will
not be executed. Instead, the corresponding strategy will be executed based on
the `stp_act` of the taker.  
2\. `stp_id` returns `0` by default for orders that have not been set for
`STP group` | | »» stp_act | string | Self-Trading Prevention Action. Users can
use this field to set self-trade prevetion strategies

1\. After users join the `STP Group`, he can pass `stp_act` to limit the user's
self-trade prevetion strategy. If `stp_act` is not passed, the default is `cn`
strategy。  
2\. When the user does not join the `STP group`, an error will be returned when
passing the `stp_act` parameter。  
3\. If the user did not use 'stp_act' when placing the order, 'stp_act' will
return '-'

\- cn: Cancel newest, Cancel new orders and keep old ones  
\- co: Cancel oldest, new ones  
\- cb: Cancel both, Both old and new orders will be cancelled | | »» finish_as |
string | How the order was finished.

\- open: processing  
\- filled: filled totally  
\- cancelled: manually cancelled  
\- ioc: time in force is `IOC`, finish immediately  
\- stp: cancelled because self trade prevention |

#### [#](#enumerated-values-26) Enumerated Values

| Property      | Value        |
| ------------- | ------------ |
| status        | open         |
| status        | closed       |
| status        | cancelled    |
| type          | limit        |
| type          | market       |
| account       | spot         |
| account       | margin       |
| account       | cross_margin |
| account       | unified      |
| side          | buy          |
| side          | sell         |
| time_in_force | gtc          |
| time_in_force | ioc          |
| time_in_force | poc          |
| time_in_force | fok          |
| stp_act       | cn           |
| stp_act       | co           |
| stp_act       | cb           |
| stp_act       | \-           |
| finish_as     | open         |
| finish_as     | filled       |
| finish_as     | cancelled    |
| finish_as     | ioc          |
| finish_as     | stp          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-spot-insurance-fund-historical-data) Query spot insurance fund historical data

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-spot-insurance-fund-historical-data](https://www.gate.io/docs/developers/apiv4/en/#query-spot-insurance-fund-historical-data)

> Code samples

`GET /spot/insurance_history`

_Query spot insurance fund historical data_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-parameters](https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-parameters)

| Name     | In    | Type           | Required | Description                                                                 |
| -------- | ----- | -------------- | -------- | --------------------------------------------------------------------------- |
| business | query | string         | true     | Leverage business, margin - position by position; unified - unified account |
| currency | query | string         | true     | Currency                                                                    |
| page     | query | integer(int32) | false    | Page number                                                                 |
| limit    | query | integer        | false    | The maximum number of items returned in the list, the default value is 30   |
| from     | query | integer(int64) | true     | Start timestamp in seconds                                                  |
| to       | query | integer(int64) | true     | End timestamp in seconds                                                    |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "balance": "1021.21",
    "time": 1727054547
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-responses](https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getspotinsurancehistory-responseschema)

Status Code **200**

| Name       | Type           | Description                            |
| ---------- | -------------- | -------------------------------------- |
| » currency | string         | Currency                               |
| » balance  | string         | Balance                                |
| » time     | integer(int64) | Creation time, timestamp, milliseconds |

This operation does not require authentication

## [#](#create-price-triggered-order) Create price-triggered order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order](https://www.gate.io/docs/developers/apiv4/en/#create-price-triggered-order)

> Code samples

`POST /spot/price_orders`

_Create price-triggered order_

> Body parameter

```
{
  "trigger": {
    "price": "100",
    "rule": ">=",
    "expiration": 3600
  },
  "put": {
    "type": "limit",
    "side": "buy",
    "price": "2.15",
    "amount": "2.00000000",
    "account": "normal",
    "time_in_force": "gtc",
    "text": "api"
  },
  "market": "GT_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-parameters)

| Name             | In   | Type                                                      | Required | Description                                                                              |
| ---------------- | ---- | --------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------- |
| body             | body | [SpotPriceTriggeredOrder](#schemaspotpricetriggeredorder) | true     | none                                                                                     |
| » trigger        | body | object                                                    | true     | none                                                                                     |
| »» price         | body | string                                                    | true     | Trigger price                                                                            |
| »» rule          | body | string                                                    | true     | Price trigger condition                                                                  |
| »» expiration    | body | integer                                                   | true     | Maximum wait time for trigger condition (in seconds). Order will be cancelled if timeout |
| » put            | body | object                                                    | true     | none                                                                                     |
| »» type          | body | string                                                    | false    | Order type，default to `limit`                                                           |
| »» side          | body | string                                                    | true     | Order side                                                                               |
| »» price         | body | string                                                    | true     | Order price                                                                              |
| »» amount        | body | string                                                    | true     | Trading quantity                                                                         |
| »» account       | body | string                                                    | true     | Trading account type. Unified account must be set to `unified`                           |
| »» time_in_force | body | string                                                    | false    | time_in_force                                                                            |
| »» auto_borrow   | body | boolean                                                   | false    | Whether to borrow coins automatically                                                    |
| »» auto_repay    | body | boolean                                                   | false    | Whether to repay the loan automatically                                                  |
| »» text          | body | string                                                    | false    | The source of the order, including:                                                      |
| » market         | body | string                                                    | true     | Market                                                                                   |

#### [#](#detailed-descriptions-20) Detailed descriptions

**»» rule**: Price trigger condition

- `>=`: triggered when market price is greater than or equal to `price`
- `<=`: triggered when market price is less than or equal to `price`

**»» type**: Order type，default to `limit`

- limit : Limit Order
- market : Market Order

**»» side**: Order side

- buy: buy side
- sell: sell side

**»» amount**: Trading quantity When `type` is `limit`, it refers to the base
currency (the currency being traded), such as `BTC` in `BTC_USDT` When `type` is
`market`, it refers to different currencies based on the side:

- `side`: `buy` refers to quote currency, `BTC_USDT` means `USDT`
- `side`: `sell` refers to base currency, `BTC_USDT` means `BTC`

**»» account**: Trading account type. Unified account must be set to `unified`

- normal: spot trading
- margin: margin trading
- unified: unified account

**»» time_in_force**: time_in_force

- gtc: GoodTillCancelled
- ioc: ImmediateOrCancelled, taker only

**»» text**: The source of the order, including:

- web: Web
- api: API call
- app: Mobile app

#### [#](#enumerated-values-27) Enumerated Values

| Parameter        | Value   |
| ---------------- | ------- |
| »» rule          | \>=     |
| »» rule          | <=      |
| »» type          | limit   |
| »» type          | market  |
| »» side          | buy     |
| »» side          | sell    |
| »» account       | normal  |
| »» account       | margin  |
| »» account       | unified |
| »» time_in_force | gtc     |
| »» time_in_force | ioc     |

> Example responses

> 201 Response

```
{
  "id": 1432329
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-responses)

| Status | Meaning                                                                         | Description                | Schema |
| ------ | ------------------------------------------------------------------------------- | -------------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createspotpricetriggeredorder-responseschema)

Status Code **201**

_TriggerOrderResponse_

| Name | Type           | Description   |
| ---- | -------------- | ------------- |
| » id | integer(int64) | Auto order ID |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-running-auto-order-list) Query running auto order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-running-auto-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-running-auto-order-list)

> Code samples

`GET /spot/price_orders`

_Query running auto order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotpricetriggeredorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listspotpricetriggeredorders-parameters)

| Name    | In    | Type    | Required | Description                                                    |
| ------- | ----- | ------- | -------- | -------------------------------------------------------------- |
| status  | query | string  | true     | Query order list based on status                               |
| market  | query | string  | false    | Trading market                                                 |
| account | query | string  | false    | Trading account type. Unified account must be set to `unified` |
| limit   | query | integer | false    | Maximum number of records returned in a single list            |
| offset  | query | integer | false    | List offset, starting from 0                                   |

#### [#](#enumerated-values-28) Enumerated Values

| Parameter | Value    |
| --------- | -------- |
| status    | open     |
| status    | finished |
| account   | normal   |
| account   | margin   |
| account   | unified  |

> Example responses

> 200 Response

```
[
  {
    "trigger": {
      "price": "100",
      "rule": ">=",
      "expiration": 3600
    },
    "put": {
      "type": "limit",
      "side": "buy",
      "price": "2.15",
      "amount": "2.00000000",
      "account": "normal",
      "time_in_force": "gtc",
      "text": "api"
    },
    "id": 1283293,
    "user": 1234,
    "market": "GT_USDT",
    "ctime": 1616397800,
    "ftime": 1616397801,
    "fired_order_id": 0,
    "status": "",
    "reason": ""
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listspotpricetriggeredorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listspotpricetriggeredorders-responses)

| Status | Meaning                                                                    | Description                 | Schema                                                        |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[SpotPriceTriggeredOrder](#schemaspotpricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-auto-orders) Cancel all auto orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-auto-orders)

> Code samples

`DELETE /spot/price_orders`

_Cancel all auto orders_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorderlist-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorderlist-parameters)

| Name    | In    | Type   | Required | Description                                                    |
| ------- | ----- | ------ | -------- | -------------------------------------------------------------- |
| market  | query | string | false    | Trading market                                                 |
| account | query | string | false    | Trading account type. Unified account must be set to `unified` |

#### [#](#enumerated-values-29) Enumerated Values

| Parameter | Value   |
| --------- | ------- |
| account   | normal  |
| account   | margin  |
| account   | unified |

> Example responses

> 200 Response

```
[
  {
    "trigger": {
      "price": "100",
      "rule": ">=",
      "expiration": 3600
    },
    "put": {
      "type": "limit",
      "side": "buy",
      "price": "2.15",
      "amount": "2.00000000",
      "account": "normal",
      "time_in_force": "gtc",
      "text": "api"
    },
    "id": 1283293,
    "user": 1234,
    "market": "GT_USDT",
    "ctime": 1616397800,
    "ftime": 1616397801,
    "fired_order_id": 0,
    "status": "",
    "reason": ""
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorderlist-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorderlist-responses)

| Status | Meaning                                                                    | Description                                                                         | Schema                                                        |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation request accepted and processed, success determined by order list | \[[SpotPriceTriggeredOrder](#schemaspotpricetriggeredorder)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-auto-order-details) Query single auto order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details](https://www.gate.io/docs/developers/apiv4/en/#query-single-auto-order-details)

> Code samples

`GET /spot/price_orders/{order_id}`

_Query single auto order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getspotpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getspotpricetriggeredorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| order_id | path | string | true     | ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "trigger": {
    "price": "100",
    "rule": ">=",
    "expiration": 3600
  },
  "put": {
    "type": "limit",
    "side": "buy",
    "price": "2.15",
    "amount": "2.00000000",
    "account": "normal",
    "time_in_force": "gtc",
    "text": "api"
  },
  "id": 1283293,
  "user": 1234,
  "market": "GT_USDT",
  "ctime": 1616397800,
  "ftime": 1616397801,
  "fired_order_id": 0,
  "status": "",
  "reason": ""
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getspotpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getspotpricetriggeredorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                    |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [SpotPriceTriggeredOrder](#schemaspotpricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-auto-order) Cancel single auto order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-auto-order)

> Code samples

`DELETE /spot/price_orders/{order_id}`

_Cancel single auto order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorder-parameters)

| Name     | In   | Type   | Required | Description                                    |
| -------- | ---- | ------ | -------- | ---------------------------------------------- |
| order_id | path | string | true     | ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "trigger": {
    "price": "100",
    "rule": ">=",
    "expiration": 3600
  },
  "put": {
    "type": "limit",
    "side": "buy",
    "price": "2.15",
    "amount": "2.00000000",
    "account": "normal",
    "time_in_force": "gtc",
    "text": "api"
  },
  "id": 1283293,
  "user": 1234,
  "market": "GT_USDT",
  "ctime": 1616397800,
  "ftime": 1616397801,
  "fired_order_id": 0,
  "status": "",
  "reason": ""
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelspotpricetriggeredorder-responses)

| Status | Meaning                                                                    | Description        | Schema                                                    |
| ------ | -------------------------------------------------------------------------- | ------------------ | --------------------------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Auto order details | [SpotPriceTriggeredOrder](#schemaspotpricetriggeredorder) |

WARNING

To perform this operation, you must be authenticated by API key and secret
