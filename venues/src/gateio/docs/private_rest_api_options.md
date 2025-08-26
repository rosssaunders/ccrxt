# [#](#options) Options

Options API

## [#](#list-all-underlying-assets) List all underlying assets

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-underlying-assets](https://www.gate.io/docs/developers/apiv4/en/#list-all-underlying-assets)

> Code samples

`GET /options/underlyings`

_List all underlying assets_

> Example responses

> 200 Response

```
[
  {
    "name": "BTC_USDT",
    "index_price": "70000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyings-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyings-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyings-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyings-responseschema)

Status Code **200**

| Name          | Type   | Description                       |
| ------------- | ------ | --------------------------------- |
| » name        | string | Underlying name                   |
| » index_price | string | Spot index price (quote currency) |

This operation does not require authentication

## [#](#list-all-expiration-dates) List all expiration dates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-expiration-dates](https://www.gate.io/docs/developers/apiv4/en/#list-all-expiration-dates)

> Code samples

`GET /options/expirations`

_List all expiration dates_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-parameters)

| Name       | In    | Type   | Required | Description                                          |
| ---------- | ----- | ------ | -------- | ---------------------------------------------------- |
| underlying | query | string | true     | Underlying (Obtained by listing underlying endpoint) |

> Example responses

> 200 Response

```
[
  1637913600
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-responses)

| Status | Meaning                                                                    | Description                                    | Schema      |
| ------ | -------------------------------------------------------------------------- | ---------------------------------------------- | ----------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List expiration dates for specified underlying | \[integer\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsexpirations-responseschema)

Status Code **200**

| Name     | Type           | Description                       |
| -------- | -------------- | --------------------------------- |
| » _None_ | integer(int64) | Unix timestamp of expiration date |

This operation does not require authentication

## [#](#list-all-contracts-for-specified-underlying-and-expiration-date) List all contracts for specified underlying and expiration date

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-contracts-for-specified-underlying-and-expiration-date](https://www.gate.io/docs/developers/apiv4/en/#list-all-contracts-for-specified-underlying-and-expiration-date)

> Code samples

`GET /options/contracts`

_List all contracts for specified underlying and expiration date_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| expiration | query | integer(int64) | false    | Unix timestamp of expiration date                    |

> Example responses

> 200 Response

```
[
  {
    "name": "BTC_USDT-20211130-65000-C",
    "tag": "WEEK",
    "create_time": 1636702700,
    "expiration_time": 1637913600,
    "is_call": true,
    "strike_price": "65000",
    "last_price": "13000",
    "mark_price": "14010",
    "orderbook_id": 9,
    "trade_id": 1,
    "trade_size": 10,
    "position_size": 10,
    "underlying": "BTC_USDT",
    "underlying_price": "70000",
    "multiplier": "0.0001",
    "order_price_round": "0.1",
    "mark_price_round": "0.1",
    "maker_fee_rate": "0.0004",
    "taker_fee_rate": "0.0004",
    "price_limit_fee_rate": "0.1",
    "ref_discount_rate": "0",
    "ref_rebate_rate": "0",
    "order_price_deviate": "0.5",
    "order_size_min": 1,
    "order_size_max": 100000,
    "orders_limit": 50
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionscontracts-responseschema)

Status Code **200**

| Name                   | Type           | Description                                                                                                                                         |
| ---------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| _None_                 | array          | \[Options contract details\]                                                                                                                        |
| » _None_               | object         | Options contract details                                                                                                                            |
| »» name                | string         | Options contract name                                                                                                                               |
| »» tag                 | string         | Tag                                                                                                                                                 |
| »» create_time         | number(double) | Creation time                                                                                                                                       |
| »» expiration_time     | number(double) | Expiration time                                                                                                                                     |
| »» is_call             | boolean        | `true` means call options, `false` means put options                                                                                                |
| »» multiplier          | string         | Multiplier used in converting from invoicing to settlement currency                                                                                 |
| »» underlying          | string         | Underlying                                                                                                                                          |
| »» underlying_price    | string         | Underlying price (quote currency)                                                                                                                   |
| »» last_price          | string         | Last trading price                                                                                                                                  |
| »» mark_price          | string         | Current mark price (quote currency)                                                                                                                 |
| »» index_price         | string         | Current index price (quote currency)                                                                                                                |
| »» maker_fee_rate      | string         | Maker fee rate, negative values indicate rebates                                                                                                    |
| »» taker_fee_rate      | string         | Taker fee rate                                                                                                                                      |
| »» order_price_round   | string         | Minimum order price increment                                                                                                                       |
| »» mark_price_round    | string         | Minimum mark price increment                                                                                                                        |
| »» order_size_min      | integer(int64) | Minimum order size allowed by the contract                                                                                                          |
| »» order_size_max      | integer(int64) | Maximum order size allowed by the contract                                                                                                          |
| »» order_price_deviate | string         | The positive and negative offset allowed between the order price and the current mark price, that `order_price` must meet the following conditions: |

order_price is within the range of mark_price +/- order_price_deviate \*
underlying_price and does not distinguish between buy and sell orders | | »»
ref_discount_rate | string | Trading fee discount for referred users | | »»
ref_rebate_rate | string | Commission rate for referrers | | »» orderbook_id |
integer(int64) | Orderbook update ID | | »» trade_id | integer(int64) | Current
trade ID | | »» trade_size | integer(int64) | Historical cumulative trading
volume | | »» position_size | integer(int64) | Current total long position size
| | »» orders_limit | integer | Maximum number of pending orders |

This operation does not require authentication

## [#](#query-specified-contract-details) Query specified contract details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-specified-contract-details](https://www.gate.io/docs/developers/apiv4/en/#query-specified-contract-details)

> Code samples

`GET /options/contracts/{contract}`

_Query specified contract details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-parameters](https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-parameters)

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| contract | path | string | true     | none        |

> Example responses

> 200 Response

```
{
  "name": "BTC_USDT-20211130-65000-C",
  "tag": "WEEK",
  "create_time": 1636702700,
  "expiration_time": 1637913600,
  "is_call": true,
  "strike_price": "65000",
  "last_price": "13000",
  "mark_price": "14010",
  "orderbook_id": 9,
  "trade_id": 1,
  "trade_size": 10,
  "position_size": 10,
  "underlying": "BTC_USDT",
  "underlying_price": "70000",
  "multiplier": "0.0001",
  "order_price_round": "0.1",
  "mark_price_round": "0.1",
  "maker_fee_rate": "0.0004",
  "taker_fee_rate": "0.0004",
  "price_limit_fee_rate": "0.1",
  "ref_discount_rate": "0",
  "ref_rebate_rate": "0",
  "order_price_deviate": "0.5",
  "order_size_min": 1,
  "order_size_max": 100000,
  "orders_limit": 50
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-responses](https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getoptionscontract-responseschema)

Status Code **200**

_Options contract details_

| Name                  | Type           | Description                                                                                                                                         |
| --------------------- | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| » name                | string         | Options contract name                                                                                                                               |
| » tag                 | string         | Tag                                                                                                                                                 |
| » create_time         | number(double) | Creation time                                                                                                                                       |
| » expiration_time     | number(double) | Expiration time                                                                                                                                     |
| » is_call             | boolean        | `true` means call options, `false` means put options                                                                                                |
| » multiplier          | string         | Multiplier used in converting from invoicing to settlement currency                                                                                 |
| » underlying          | string         | Underlying                                                                                                                                          |
| » underlying_price    | string         | Underlying price (quote currency)                                                                                                                   |
| » last_price          | string         | Last trading price                                                                                                                                  |
| » mark_price          | string         | Current mark price (quote currency)                                                                                                                 |
| » index_price         | string         | Current index price (quote currency)                                                                                                                |
| » maker_fee_rate      | string         | Maker fee rate, negative values indicate rebates                                                                                                    |
| » taker_fee_rate      | string         | Taker fee rate                                                                                                                                      |
| » order_price_round   | string         | Minimum order price increment                                                                                                                       |
| » mark_price_round    | string         | Minimum mark price increment                                                                                                                        |
| » order_size_min      | integer(int64) | Minimum order size allowed by the contract                                                                                                          |
| » order_size_max      | integer(int64) | Maximum order size allowed by the contract                                                                                                          |
| » order_price_deviate | string         | The positive and negative offset allowed between the order price and the current mark price, that `order_price` must meet the following conditions: |

order_price is within the range of mark_price +/- order_price_deviate \*
underlying_price and does not distinguish between buy and sell orders | | »
ref_discount_rate | string | Trading fee discount for referred users | | »
ref_rebate_rate | string | Commission rate for referrers | | » orderbook_id |
integer(int64) | Orderbook update ID | | » trade_id | integer(int64) | Current
trade ID | | » trade_size | integer(int64) | Historical cumulative trading
volume | | » position_size | integer(int64) | Current total long position size |
| » orders_limit | integer | Maximum number of pending orders |

This operation does not require authentication

## [#](#list-settlement-history) List settlement history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-settlement-history](https://www.gate.io/docs/developers/apiv4/en/#list-settlement-history)

> Code samples

`GET /options/settlements`

_List settlement history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| limit      | query | integer        | false    | Maximum number of records returned in a single list  |
| offset     | query | integer        | false    | List offset, starting from 0                         |
| from       | query | integer(int64) | false    | Start timestamp                                      |
| to         | query | integer(int64) | false    | Termination Timestamp                                |

#### [#](#detailed-descriptions-43) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

> Example responses

> 200 Response

```
[
  {
    "time": 1598839200,
    "profit": "312.35",
    "fee": "0.3284",
    "settle_price": "11687.65",
    "contract": "BTC-WEEKLY-200824-11000-P",
    "strike_price": "12000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionssettlements-responseschema)

Status Code **200**

| Name           | Type           | Description                                     |
| -------------- | -------------- | ----------------------------------------------- |
| _None_         | array          | none                                            |
| » time         | number(double) | Last configuration update time                  |
| » contract     | string         | Options contract name                           |
| » profit       | string         | Settlement profit per contract (quote currency) |
| » fee          | string         | Settlement fee per contract (quote currency)    |
| » strike_price | string         | Strike price (quote currency)                   |
| » settle_price | string         | Settlement price (quote currency)               |

This operation does not require authentication

## [#](#get-specified-contract-settlement-information) Get specified contract settlement information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-specified-contract-settlement-information](https://www.gate.io/docs/developers/apiv4/en/#get-specified-contract-settlement-information)

> Code samples

`GET /options/settlements/{contract}`

_Get specified contract settlement information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-parameters](https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| contract   | path  | string         | true     | none                                                 |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| at         | query | integer(int64) | true     | none                                                 |

> Example responses

> 200 Response

```
{
  "time": 1598839200,
  "profit": "312.35",
  "fee": "0.3284",
  "settle_price": "11687.65",
  "contract": "BTC-WEEKLY-200824-11000-P",
  "strike_price": "12000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-responses](https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getoptionssettlement-responseschema)

Status Code **200**

| Name           | Type           | Description                                     |
| -------------- | -------------- | ----------------------------------------------- |
| » time         | number(double) | Last configuration update time                  |
| » contract     | string         | Options contract name                           |
| » profit       | string         | Settlement profit per contract (quote currency) |
| » fee          | string         | Settlement fee per contract (quote currency)    |
| » strike_price | string         | Strike price (quote currency)                   |
| » settle_price | string         | Settlement price (quote currency)               |

This operation does not require authentication

## [#](#query-personal-settlement-records) Query personal settlement records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-settlement-records](https://www.gate.io/docs/developers/apiv4/en/#query-personal-settlement-records)

> Code samples

`GET /options/my_settlements`

_Query personal settlement records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| contract   | query | string         | false    | Options contract name                                |
| limit      | query | integer        | false    | Maximum number of records returned in a single list  |
| offset     | query | integer        | false    | List offset, starting from 0                         |
| from       | query | integer(int64) | false    | Start timestamp                                      |
| to         | query | integer(int64) | false    | Termination Timestamp                                |

#### [#](#detailed-descriptions-44) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

> Example responses

> 200 Response

```
[
  {
    "size": -1,
    "settle_profit": "0",
    "contract": "BTC_USDT-20220624-26000-C",
    "strike_price": "26000",
    "time": 1656057600,
    "settle_price": "20917.461281337048",
    "underlying": "BTC_USDT",
    "realised_pnl": "-0.00116042",
    "fee": "0"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-responses](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionssettlements-responseschema)

Status Code **200**

| Name            | Type           | Description                                                                                                           |
| --------------- | -------------- | --------------------------------------------------------------------------------------------------------------------- |
| » time          | number(double) | Settlement time                                                                                                       |
| » underlying    | string         | Underlying                                                                                                            |
| » contract      | string         | Options contract name                                                                                                 |
| » strike_price  | string         | Strike price (quote currency)                                                                                         |
| » settle_price  | string         | Settlement price (quote currency)                                                                                     |
| » size          | integer(int64) | Settlement size                                                                                                       |
| » settle_profit | string         | Settlement profit (quote currency)                                                                                    |
| » fee           | string         | Settlement fee (quote currency)                                                                                       |
| » realised_pnl  | string         | Accumulated profit and loss from opening positions, including premium, fees, settlement profit, etc. (quote currency) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-options-contract-order-book) Query options contract order book

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-options-contract-order-book](https://www.gate.io/docs/developers/apiv4/en/#query-options-contract-order-book)

> Code samples

`GET /options/order_book`

_Query options contract order book_

Bids will be sorted by price from high to low, while asks sorted reversely

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-parameters)

| Name     | In    | Type    | Required | Description                                                                                   |
| -------- | ----- | ------- | -------- | --------------------------------------------------------------------------------------------- |
| contract | query | string  | true     | Options contract name                                                                         |
| interval | query | string  | false    | Price precision for depth aggregation, 0 means no aggregation, defaults to 0 if not specified |
| limit    | query | integer | false    | Number of depth levels                                                                        |
| with_id  | query | boolean | false    | Whether to return depth update ID. This ID increments by 1 each time depth changes            |

#### [#](#enumerated-values-118) Enumerated Values

| Parameter | Value |
| --------- | ----- |
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
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-responses)

| Status | Meaning                                                                    | Description            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Depth query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorderbook-responseschema)

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

## [#](#query-options-market-ticker-information) Query options market ticker information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-options-market-ticker-information](https://www.gate.io/docs/developers/apiv4/en/#query-options-market-ticker-information)

> Code samples

`GET /options/tickers`

_Query options market ticker information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-parameters)

| Name       | In    | Type   | Required | Description                                          |
| ---------- | ----- | ------ | -------- | ---------------------------------------------------- |
| underlying | query | string | true     | Underlying (Obtained by listing underlying endpoint) |

> Example responses

> 200 Response

```
[
  {
    "name": "BTC_USDT-20211130-65000-C",
    "last_price": "13000",
    "mark_price": "14010",
    "position_size": 10,
    "ask1_size": 0,
    "ask1_price": "0",
    "bid1_size": 1,
    "bid1_price": "11",
    "vega": "41.41202",
    "theta": "-120.1506",
    "rho": "6.52485",
    "gamma": "0.00004",
    "delta": "0.33505",
    "mark_iv": "0.123",
    "bid_iv": "0.023",
    "ask_iv": "0.342",
    "leverage": "13"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionstickers-responseschema)

Status Code **200**

| Name             | Type           | Description                                                       |
| ---------------- | -------------- | ----------------------------------------------------------------- |
| » _None_         | object         | Options contract details                                          |
| »» name          | string         | Options contract name                                             |
| »» last_price    | string         | Last trade price (quote currency)                                 |
| »» mark_price    | string         | Current mark price (quote currency)                               |
| »» index_price   | string         | Current index price (quote currency)                              |
| »» ask1_size     | integer(int64) | Best ask size                                                     |
| »» ask1_price    | string         | Best ask price                                                    |
| »» bid1_size     | integer(int64) | Best bid size                                                     |
| »» bid1_price    | string         | Best bid price                                                    |
| »» position_size | integer(int64) | Current total long position size                                  |
| »» mark_iv       | string         | Implied volatility                                                |
| »» bid_iv        | string         | Bid side implied volatility                                       |
| »» ask_iv        | string         | Ask side implied volatility                                       |
| »» leverage      | string         | Current leverage. Formula: underlying_price / mark_price \* delta |
| »» delta         | string         | Greek letter delta                                                |
| »» gamma         | string         | Greek letter gamma                                                |
| »» vega          | string         | Greek letter vega                                                 |
| »» theta         | string         | Greek letter theta                                                |
| »» rho           | string         | Rho                                                               |

This operation does not require authentication

## [#](#query-underlying-ticker-information) Query underlying ticker information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-underlying-ticker-information](https://www.gate.io/docs/developers/apiv4/en/#query-underlying-ticker-information)

> Code samples

`GET /options/underlying/tickers/{underlying}`

_Query underlying ticker information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-parameters)

| Name       | In   | Type   | Required | Description |
| ---------- | ---- | ------ | -------- | ----------- |
| underlying | path | string | true     | Underlying  |

> Example responses

> 200 Response

```
{
  "trade_put": 33505,
  "trade_call": 123,
  "index_price": "76543.3"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingtickers-responseschema)

Status Code **200**

_Options underlying detail_

| Name          | Type           | Description                                  |
| ------------- | -------------- | -------------------------------------------- |
| » trade_put   | integer(int64) | Total put options trades amount in last 24h  |
| » trade_call  | integer(int64) | Total call options trades amount in last 24h |
| » index_price | string         | Index price (quote currency)                 |

This operation does not require authentication

## [#](#options-contract-market-candlestick-chart) Options contract market candlestick chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#options-contract-market-candlestick-chart](https://www.gate.io/docs/developers/apiv4/en/#options-contract-market-candlestick-chart)

> Code samples

`GET /options/candlesticks`

_Options contract market candlestick chart_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-parameters)

| Name     | In    | Type           | Required | Description                                         |
| -------- | ----- | -------------- | -------- | --------------------------------------------------- |
| contract | query | string         | true     | Options contract name                               |
| limit    | query | integer        | false    | Maximum number of records returned in a single list |
| from     | query | integer(int64) | false    | Start timestamp                                     |
| to       | query | integer(int64) | false    | Termination Timestamp                               |
| interval | query | string         | false    | Time interval between data points                   |

#### [#](#detailed-descriptions-45) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-119) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| interval  | 1m    |
| interval  | 5m    |
| interval  | 15m   |
| interval  | 30m   |
| interval  | 1h    |

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
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionscandlesticks-responseschema)

Status Code **200**

| Name     | Type           | Description                                                                 |
| -------- | -------------- | --------------------------------------------------------------------------- |
| » _None_ | object         | data point in every timestamp                                               |
| »» t     | number(double) | Unix timestamp in seconds                                                   |
| »» v     | integer(int64) | size volume (contract size). Only returned if `contract` is not prefixed    |
| »» c     | string         | Close price (quote currency, unit: underlying corresponding option price)   |
| »» h     | string         | Highest price (quote currency, unit: underlying corresponding option price) |
| »» l     | string         | Lowest price (quote currency, unit: underlying corresponding option price)  |
| »» o     | string         | Open price (quote currency, unit: underlying corresponding option price)    |

This operation does not require authentication

## [#](#underlying-index-price-candlestick-chart) Underlying index price candlestick chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#underlying-index-price-candlestick-chart](https://www.gate.io/docs/developers/apiv4/en/#underlying-index-price-candlestick-chart)

> Code samples

`GET /options/underlying/candlesticks`

_Underlying index price candlestick chart_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| limit      | query | integer        | false    | Maximum number of records returned in a single list  |
| from       | query | integer(int64) | false    | Start timestamp                                      |
| to         | query | integer(int64) | false    | Termination Timestamp                                |
| interval   | query | string         | false    | Time interval between data points                    |

#### [#](#detailed-descriptions-46) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-120) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| interval  | 1m    |
| interval  | 5m    |
| interval  | 15m   |
| interval  | 30m   |
| interval  | 1h    |

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
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsunderlyingcandlesticks-responseschema)

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

## [#](#market-trade-records) Market trade records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#market-trade-records](https://www.gate.io/docs/developers/apiv4/en/#market-trade-records)

> Code samples

`GET /options/trades`

_Market trade records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-parameters)

| Name     | In    | Type           | Required | Description                                         |
| -------- | ----- | -------------- | -------- | --------------------------------------------------- |
| contract | query | string         | false    | Options contract name                               |
| type     | query | string(P)      | false    | `C` for call, `P` for put                           |
| limit    | query | integer        | false    | Maximum number of records returned in a single list |
| offset   | query | integer        | false    | List offset, starting from 0                        |
| from     | query | integer(int64) | false    | Start timestamp                                     |
| to       | query | integer(int64) | false    | Termination Timestamp                               |

#### [#](#detailed-descriptions-47) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

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
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionstrades-responseschema)

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

## [#](#query-account-information) Query account information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-account-information](https://www.gate.io/docs/developers/apiv4/en/#query-account-information)

> Code samples

`GET /options/accounts`

_Query account information_

> Example responses

> 200 Response

```
{
  "user": 666,
  "currency": "USDT",
  "short_enabled": true,
  "mmp_enabled": false,
  "liq_triggered": false,
  "margin_mode": 0,
  "total": "1650.443022",
  "position_value": "-40.1136",
  "equity": "1610.329422",
  "unrealised_pnl": "-0.7811",
  "init_margin": "0",
  "maint_margin": "135.541485",
  "order_margin": "139.74496",
  "ask_order_margin": "139.74496",
  "bid_order_margin": "0",
  "available": "1514.901537",
  "point": "0",
  "orders_limit": 10,
  "position_notional_limit": 1000000
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccount-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccount-responseschema)

Status Code **200**

| Name                                                                   | Type           | Description                                                                                 |
| ---------------------------------------------------------------------- | -------------- | ------------------------------------------------------------------------------------------- |
| » user                                                                 | integer(int64) | User ID                                                                                     |
| » total                                                                | string         | Account Balance                                                                             |
| » position_value                                                       | string         | Position value, long position value is positive, short position value is negative           |
| » equity                                                               | string         | Account equity, the sum of account balance and position value                               |
| » short_enabled                                                        | boolean        | If the account is allowed to short                                                          |
| » mmp_enabled                                                          | boolean        | Whether to enable MMP                                                                       |
| » liq_triggered                                                        | boolean        | Whether to trigger position liquidation                                                     |
| » margin_mode                                                          | integer(int32) | ｜ Margin模式：                                                                             |
| \- 0：经典SpotMargin模式 - 1：跨CurrencyMargin模式 - 2：组合Margin模式 |
| » unrealised_pnl                                                       | string         | Unrealized PNL                                                                              |
| » init_margin                                                          | string         | Initial position margin                                                                     |
| » maint_margin                                                         | string         | Position maintenance margin                                                                 |
| » order_margin                                                         | string         | Order margin of unfinished orders                                                           |
| » ask_order_margin                                                     | string         | Margin for outstanding sell orders                                                          |
| » bid_order_margin                                                     | string         | Margin for outstanding buy orders                                                           |
| » available                                                            | string         | Available balance to transfer out or trade                                                  |
| » point                                                                | string         | Point card amount                                                                           |
| » currency                                                             | string         | Settlement currency                                                                         |
| » orders_limit                                                         | integer(int32) | Maximum number of outstanding orders                                                        |
| » position_notional_limit                                              | integer(int64) | Notional value upper limit, including the nominal value of positions and outstanding orders |

#### [#](#enumerated-values-121) Enumerated Values

| Property    | Value |
| ----------- | ----- |
| margin_mode | 0     |
| margin_mode | 1     |
| margin_mode | 2     |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-account-change-history) Query account change history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-account-change-history](https://www.gate.io/docs/developers/apiv4/en/#query-account-change-history)

> Code samples

`GET /options/account_book`

_Query account change history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-parameters)

| Name   | In    | Type           | Required | Description                                         |
| ------ | ----- | -------------- | -------- | --------------------------------------------------- |
| limit  | query | integer        | false    | Maximum number of records returned in a single list |
| offset | query | integer        | false    | List offset, starting from 0                        |
| from   | query | integer(int64) | false    | Start timestamp                                     |
| to     | query | integer(int64) | false    | Termination Timestamp                               |
| type   | query | string         | false    | Change types:                                       |

#### [#](#detailed-descriptions-48) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

**type**: Change types:

- dnw: Deposit & Withdrawal
- prem: Trading premium
- fee: Trading fee
- refr: Referrer rebate
- set: Settlement P&L

#### [#](#enumerated-values-122) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| type      | dnw   |
| type      | prem  |
| type      | fee   |
| type      | refr  |
| type      | set   |

> Example responses

> 200 Response

```
[
  {
    "time": 1636426005,
    "change": "-0.16",
    "balance": "7378.189",
    "text": "BTC_USDT-20211216-5000-P:25",
    "type": "fee"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsaccountbook-responseschema)

Status Code **200**

| Name      | Type           | Description                               |
| --------- | -------------- | ----------------------------------------- |
| » time    | number(double) | Change time                               |
| » change  | string         | Amount changed (USDT)                     |
| » balance | string         | Account total balance after change (USDT) |
| » type    | string         | Changing Type:                            |

\- dnw: Deposit & Withdraw  
\- prem: Trading premium  
\- fee: Trading fee  
\- refr: Referrer rebate  
\- point_dnw: point_fee: POINT Trading fee  
\- point_refr: POINT Referrer rebate | | » text | string | Note |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-user-s-positions-of-specified-underlying) List user's positions of specified underlying

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-user-s-positions-of-specified-underlying](https://www.gate.io/docs/developers/apiv4/en/#list-user-s-positions-of-specified-underlying)

> Code samples

`GET /options/positions`

_List user's positions of specified underlying_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-parameters)

| Name       | In    | Type   | Required | Description |
| ---------- | ----- | ------ | -------- | ----------- |
| underlying | query | string | false    | Underlying  |

> Example responses

> 200 Response

```
[
  {
    "user": 11027586,
    "underlying": "BTC_USDT",
    "underlying_price": "70000",
    "contract": "BTC_USDT-20211216-5000-P",
    "size": 10,
    "entry_price": "1234",
    "realised_pnl": "120",
    "mark_price": "6000",
    "mark_iv": "0.9638",
    "unrealised_pnl": "-320",
    "pending_orders": 1,
    "close_order": {
      "id": 232323,
      "price": "5779",
      "is_liq": false
    },
    "delta": "-0.0046",
    "gamma": "0",
    "vega": "2.87656",
    "theta": "-1.00247"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositions-responseschema)

Status Code **200**

| Name                | Type           | Description                                 |
| ------------------- | -------------- | ------------------------------------------- | ------------------------------------------------------------ |
| _None_              | array          | \[Options contract position details\]       |
| » _None_            | object         | Options contract position details           |
| »» user             | integer        | User ID                                     |
| »» underlying       | string         | Underlying                                  |
| »» underlying_price | string         | Underlying price (quote currency)           |
| »» contract         | string         | Options contract name                       |
| »» size             | integer(int64) | Position size (contract quantity)           |
| »» entry_price      | string         | Entry size (quote currency)                 |
| »» mark_price       | string         | Current mark price (quote currency)         |
| »» mark_iv          | string         | Implied volatility                          |
| »» realised_pnl     | string         | Realized PnL                                |
| »» unrealised_pnl   | string         | Unrealized PNL                              |
| »» pending_orders   | integer        | Current pending order quantity              |
| »» close_order      | object         | null                                        | Current close order information, or `null` if no close order |
| »»» id              | integer(int64) | Order ID                                    |
| »»» price           | string         | Order price (quote currency)                |
| »»» is_liq          | boolean        | Whether the close order is from liquidation |
| »» delta            | string         | Greek letter delta                          |
| »» gamma            | string         | Greek letter gamma                          |
| »» vega             | string         | Greek letter vega                           |
| »» theta            | string         | Greek letter theta                          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-specified-contract-position) Get specified contract position

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-specified-contract-position](https://www.gate.io/docs/developers/apiv4/en/#get-specified-contract-position)

> Code samples

`GET /options/positions/{contract}`

_Get specified contract position_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-parameters](https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-parameters)

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| contract | path | string | true     | none        |

> Example responses

> 200 Response

```
{
  "user": 11027586,
  "underlying": "BTC_USDT",
  "underlying_price": "70000",
  "contract": "BTC_USDT-20211216-5000-P",
  "size": 10,
  "entry_price": "1234",
  "realised_pnl": "120",
  "mark_price": "6000",
  "mark_iv": "0.9638",
  "unrealised_pnl": "-320",
  "pending_orders": 1,
  "close_order": {
    "id": 232323,
    "price": "5779",
    "is_liq": false
  },
  "delta": "-0.0046",
  "gamma": "0",
  "vega": "2.87656",
  "theta": "-1.00247"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-responses](https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getoptionsposition-responseschema)

Status Code **200**

_Options contract position details_

| Name               | Type           | Description                                 |
| ------------------ | -------------- | ------------------------------------------- | ------------------------------------------------------------ |
| » user             | integer        | User ID                                     |
| » underlying       | string         | Underlying                                  |
| » underlying_price | string         | Underlying price (quote currency)           |
| » contract         | string         | Options contract name                       |
| » size             | integer(int64) | Position size (contract quantity)           |
| » entry_price      | string         | Entry size (quote currency)                 |
| » mark_price       | string         | Current mark price (quote currency)         |
| » mark_iv          | string         | Implied volatility                          |
| » realised_pnl     | string         | Realized PnL                                |
| » unrealised_pnl   | string         | Unrealized PNL                              |
| » pending_orders   | integer        | Current pending order quantity              |
| » close_order      | object         | null                                        | Current close order information, or `null` if no close order |
| »» id              | integer(int64) | Order ID                                    |
| »» price           | string         | Order price (quote currency)                |
| »» is_liq          | boolean        | Whether the close order is from liquidation |
| » delta            | string         | Greek letter delta                          |
| » gamma            | string         | Greek letter gamma                          |
| » vega             | string         | Greek letter vega                           |
| » theta            | string         | Greek letter theta                          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-user-s-liquidation-history-of-specified-underlying) List user's liquidation history of specified underlying

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-user-s-liquidation-history-of-specified-underlying](https://www.gate.io/docs/developers/apiv4/en/#list-user-s-liquidation-history-of-specified-underlying)

> Code samples

`GET /options/position_close`

_List user's liquidation history of specified underlying_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-parameters)

| Name       | In    | Type   | Required | Description                                          |
| ---------- | ----- | ------ | -------- | ---------------------------------------------------- |
| underlying | query | string | true     | Underlying (Obtained by listing underlying endpoint) |
| contract   | query | string | false    | Options contract name                                |

> Example responses

> 200 Response

```
[
  {
    "time": 1631764800,
    "pnl": "-42914.291",
    "settle_size": "-10001",
    "side": "short",
    "contract": "BTC_USDT-20210916-5000-C",
    "text": "settled"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionspositionclose-responseschema)

Status Code **200**

| Name       | Type           | Description           |
| ---------- | -------------- | --------------------- |
| » time     | number(double) | Position close time   |
| » contract | string         | Options contract name |
| » side     | string         | Position side         |

\- `long`: Long position  
\- `short`: Short position | | » pnl | string | PnL | | » text | string | Source
of close order. See `order.text` field for specific values | | » settle_size |
string | Settlement size |

#### [#](#enumerated-values-123) Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | long  |
| side     | short |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#create-an-options-order) Create an options order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-an-options-order](https://www.gate.io/docs/developers/apiv4/en/#create-an-options-order)

> Code samples

`POST /options/orders`

_Create an options order_

> Body parameter

```
{
  "size": -1,
  "iceberg": 0,
  "contract": "BTC_USDT-20210916-5000-C",
  "text": "-",
  "tif": "gtc",
  "price": "100"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-parameters)

| Name          | In   | Type           | Required | Description                                                                                                  |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | true     | none                                                                                                         |
| » contract    | body | string         | true     | Options identifier                                                                                           |
| » size        | body | integer(int64) | true     | Required. Trading quantity. Positive for buy, negative for sell. Set to 0 for close position orders.         |
| » iceberg     | body | integer(int64) | false    | Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden portions are charged taker fees. |
| » price       | body | string         | false    | Order price. Price of 0 with `tif` set as `ioc` represents market order (quote currency)                     |
| » close       | body | boolean        | false    | Set as `true` to close the position, with `size` set to 0                                                    |
| » reduce_only | body | boolean        | false    | Set as `true` to be reduce-only order                                                                        |
| » mmp         | body | boolean        | false    | When set to true, it is an MMP order                                                                         |
| » tif         | body | string         | false    | Time in force strategy. Market orders currently only support IOC mode                                        |
| » text        | body | string         | false    | User defined information. If not empty, must follow the rules below:                                         |

#### [#](#detailed-descriptions-49) Detailed descriptions

**» tif**: Time in force strategy. Market orders currently only support IOC mode

- gtc: Good Till Cancelled
- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only
- poc: Pending Or Cancelled, passive order, maker only

**» text**: User defined information. If not empty, must follow the rules below:

1.  prefixed with `t-`
2.  no longer than 28 bytes without `t-` prefix
3.  can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.) Besides
    user defined information, reserved contents are listed below, denoting how
    the order is created:

- web: from web
- api: from API
- app: from mobile phones
- auto_deleveraging: from ADL
- liquidation: from liquidation
- insurance: from insurance

#### [#](#enumerated-values-124) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| » tif     | gtc   |
| » tif     | ioc   |
| » tif     | poc   |

> Example responses

> 201 Response

```
{
  "status": "finished",
  "size": -1,
  "id": 2,
  "iceberg": 0,
  "is_liq": false,
  "is_close": false,
  "is_mmp": false,
  "contract": "BTC_USDT-20210916-5000-C",
  "text": "-",
  "fill_price": "100",
  "finish_as": "filled",
  "left": 0,
  "tif": "gtc",
  "is_reduce_only": false,
  "create_time": 1631763361,
  "finish_time": 1631763397,
  "price": "100"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-responses](https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-responses)

| Status | Meaning                                                                         | Description  | Schema |
| ------ | ------------------------------------------------------------------------------- | ------------ | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Order detail | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createoptionsorder-responseschema)

Status Code **201**

_Options order details_

| Name          | Type           | Description                                        |
| ------------- | -------------- | -------------------------------------------------- |
| » id          | integer(int64) | Options order ID                                   |
| » user        | integer        | User ID                                            |
| » create_time | number(double) | Creation time of order                             |
| » finish_time | number(double) | Order finished time. Not returned if order is open |
| » finish_as   | string         | Order finish reason:                               |

\- filled: Fully filled  
\- cancelled: User cancelled  
\- liquidated: Cancelled due to liquidation  
\- ioc: Not immediately fully filled due to IOC time-in-force setting  
\- auto_deleveraged: Cancelled due to auto-deleveraging  
\- reduce_only: Cancelled due to position increase while reduce-only is set  
\- position_closed: Cancelled because the position was closed  
\- reduce_out: Only reduce positions by excluding hard-to-fill orders  
\- mmp_cancelled: Cancelled by MMP | | » status | string | Order status

\- `open`: Pending  
\- `finished`: Completed | | » contract | string | Options identifier | | » size
| integer(int64) | Required. Trading quantity. Positive for buy, negative for
sell. Set to 0 for close position orders. | | » iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | » price | string | Order price. Price of 0
with `tif` set as `ioc` represents market order (quote currency) | | » is_close
| boolean | Is the order to close position | | » is_reduce_only | boolean | Is
the order reduce-only | | » is_liq | boolean | Is the order for liquidation | |
» is_mmp | boolean | Whether it is an MMP order. Corresponds to `mmp` in the
request | | » tif | string | Time in force strategy. Market orders currently
only support IOC mode

\- gtc: Good Till Cancelled  
\- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only  
\- poc: Pending Or Cancelled, passive order, maker only | | » left |
integer(int64) | Unfilled quantity | | » fill_price | string | Fill price | | »
text | string | User defined information. If not empty, must follow the rules
below:

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
\- insurance: from insurance | | » tkfr | string | Taker fee | | » mkfr | string
| Maker fee | | » refu | integer | Referrer user ID | | » refr | string |
Referrer rebate |

#### [#](#enumerated-values-125) Enumerated Values

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
| finish_as | mmp_cancelled    |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-options-orders) List options orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-options-orders](https://www.gate.io/docs/developers/apiv4/en/#list-options-orders)

> Code samples

`GET /options/orders`

_List options orders_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-parameters)

| Name       | In    | Type           | Required | Description                                         |
| ---------- | ----- | -------------- | -------- | --------------------------------------------------- |
| contract   | query | string         | false    | Options contract name                               |
| underlying | query | string         | false    | Underlying                                          |
| status     | query | string         | true     | Query order list based on status                    |
| limit      | query | integer        | false    | Maximum number of records returned in a single list |
| offset     | query | integer        | false    | List offset, starting from 0                        |
| from       | query | integer(int64) | false    | Start timestamp                                     |
| to         | query | integer(int64) | false    | Termination Timestamp                               |

#### [#](#detailed-descriptions-50) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-126) Enumerated Values

| Parameter | Value    |
| --------- | -------- |
| status    | open     |
| status    | finished |

> Example responses

> 200 Response

```
[
  {
    "status": "finished",
    "size": -1,
    "id": 2,
    "iceberg": 0,
    "is_liq": false,
    "is_close": false,
    "is_mmp": false,
    "contract": "BTC_USDT-20210916-5000-C",
    "text": "-",
    "fill_price": "100",
    "finish_as": "filled",
    "left": 0,
    "tif": "gtc",
    "is_reduce_only": false,
    "create_time": 1631763361,
    "finish_time": 1631763397,
    "price": "100"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listoptionsorders-responseschema)

Status Code **200**

| Name           | Type           | Description                                        |
| -------------- | -------------- | -------------------------------------------------- |
| _None_         | array          | \[Options order details\]                          |
| » _None_       | object         | Options order details                              |
| »» id          | integer(int64) | Options order ID                                   |
| »» user        | integer        | User ID                                            |
| »» create_time | number(double) | Creation time of order                             |
| »» finish_time | number(double) | Order finished time. Not returned if order is open |
| »» finish_as   | string         | Order finish reason:                               |

\- filled: Fully filled  
\- cancelled: User cancelled  
\- liquidated: Cancelled due to liquidation  
\- ioc: Not immediately fully filled due to IOC time-in-force setting  
\- auto_deleveraged: Cancelled due to auto-deleveraging  
\- reduce_only: Cancelled due to position increase while reduce-only is set  
\- position_closed: Cancelled because the position was closed  
\- reduce_out: Only reduce positions by excluding hard-to-fill orders  
\- mmp_cancelled: Cancelled by MMP | | »» status | string | Order status

\- `open`: Pending  
\- `finished`: Completed | | »» contract | string | Options identifier | | »»
size | integer(int64) | Required. Trading quantity. Positive for buy, negative
for sell. Set to 0 for close position orders. | | »» iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | »» price | string | Order price. Price of 0
with `tif` set as `ioc` represents market order (quote currency) | | »» is_close
| boolean | Is the order to close position | | »» is_reduce_only | boolean | Is
the order reduce-only | | »» is_liq | boolean | Is the order for liquidation | |
»» is_mmp | boolean | Whether it is an MMP order. Corresponds to `mmp` in the
request | | »» tif | string | Time in force strategy. Market orders currently
only support IOC mode

\- gtc: Good Till Cancelled  
\- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only  
\- poc: Pending Or Cancelled, passive order, maker only | | »» left |
integer(int64) | Unfilled quantity | | »» fill_price | string | Fill price | |
»» text | string | User defined information. If not empty, must follow the rules
below:

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
string | Maker fee | | »» refu | integer | Referrer user ID | | »» refr | string
| Referrer rebate |

#### [#](#enumerated-values-127) Enumerated Values

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
| finish_as | mmp_cancelled    |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-all-orders-with-open-status-3) Cancel all orders with 'open' status

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status-3](https://www.gate.io/docs/developers/apiv4/en/#cancel-all-orders-with-open-status-3)

> Code samples

`DELETE /options/orders`

_Cancel all orders with 'open' status_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-parameters)

| Name       | In    | Type   | Required | Description                                                  |
| ---------- | ----- | ------ | -------- | ------------------------------------------------------------ |
| contract   | query | string | false    | Options contract name                                        |
| underlying | query | string | false    | Underlying                                                   |
| side       | query | string | false    | Specify all bids or all asks, both included if not specified |

#### [#](#enumerated-values-128) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| side      | ask   |
| side      | bid   |

> Example responses

> 200 Response

```
[
  {
    "status": "finished",
    "size": -1,
    "id": 2,
    "iceberg": 0,
    "is_liq": false,
    "is_close": false,
    "is_mmp": false,
    "contract": "BTC_USDT-20210916-5000-C",
    "text": "-",
    "fill_price": "100",
    "finish_as": "filled",
    "left": 0,
    "tif": "gtc",
    "is_reduce_only": false,
    "create_time": 1631763361,
    "finish_time": 1631763397,
    "price": "100"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-responses](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-responses)

| Status | Meaning                                                                    | Description                   | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Batch cancellation successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorders-responseschema)

Status Code **200**

| Name           | Type           | Description                                        |
| -------------- | -------------- | -------------------------------------------------- |
| _None_         | array          | \[Options order details\]                          |
| » _None_       | object         | Options order details                              |
| »» id          | integer(int64) | Options order ID                                   |
| »» user        | integer        | User ID                                            |
| »» create_time | number(double) | Creation time of order                             |
| »» finish_time | number(double) | Order finished time. Not returned if order is open |
| »» finish_as   | string         | Order finish reason:                               |

\- filled: Fully filled  
\- cancelled: User cancelled  
\- liquidated: Cancelled due to liquidation  
\- ioc: Not immediately fully filled due to IOC time-in-force setting  
\- auto_deleveraged: Cancelled due to auto-deleveraging  
\- reduce_only: Cancelled due to position increase while reduce-only is set  
\- position_closed: Cancelled because the position was closed  
\- reduce_out: Only reduce positions by excluding hard-to-fill orders  
\- mmp_cancelled: Cancelled by MMP | | »» status | string | Order status

\- `open`: Pending  
\- `finished`: Completed | | »» contract | string | Options identifier | | »»
size | integer(int64) | Required. Trading quantity. Positive for buy, negative
for sell. Set to 0 for close position orders. | | »» iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | »» price | string | Order price. Price of 0
with `tif` set as `ioc` represents market order (quote currency) | | »» is_close
| boolean | Is the order to close position | | »» is_reduce_only | boolean | Is
the order reduce-only | | »» is_liq | boolean | Is the order for liquidation | |
»» is_mmp | boolean | Whether it is an MMP order. Corresponds to `mmp` in the
request | | »» tif | string | Time in force strategy. Market orders currently
only support IOC mode

\- gtc: Good Till Cancelled  
\- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only  
\- poc: Pending Or Cancelled, passive order, maker only | | »» left |
integer(int64) | Unfilled quantity | | »» fill_price | string | Fill price | |
»» text | string | User defined information. If not empty, must follow the rules
below:

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
string | Maker fee | | »» refu | integer | Referrer user ID | | »» refr | string
| Referrer rebate |

#### [#](#enumerated-values-129) Enumerated Values

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
| finish_as | mmp_cancelled    |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-order-details-4) Query single order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-4](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-4)

> Code samples

`GET /options/orders/{order_id}`

_Query single order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-parameters)

| Name     | In   | Type           | Required | Description                                          |
| -------- | ---- | -------------- | -------- | ---------------------------------------------------- |
| order_id | path | integer(int64) | true     | Order ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "status": "finished",
  "size": -1,
  "id": 2,
  "iceberg": 0,
  "is_liq": false,
  "is_close": false,
  "is_mmp": false,
  "contract": "BTC_USDT-20210916-5000-C",
  "text": "-",
  "fill_price": "100",
  "finish_as": "filled",
  "left": 0,
  "tif": "gtc",
  "is_reduce_only": false,
  "create_time": 1631763361,
  "finish_time": 1631763397,
  "price": "100"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-responses](https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-responses)

| Status | Meaning                                                                    | Description  | Schema |
| ------ | -------------------------------------------------------------------------- | ------------ | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order detail | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getoptionsorder-responseschema)

Status Code **200**

_Options order details_

| Name          | Type           | Description                                        |
| ------------- | -------------- | -------------------------------------------------- |
| » id          | integer(int64) | Options order ID                                   |
| » user        | integer        | User ID                                            |
| » create_time | number(double) | Creation time of order                             |
| » finish_time | number(double) | Order finished time. Not returned if order is open |
| » finish_as   | string         | Order finish reason:                               |

\- filled: Fully filled  
\- cancelled: User cancelled  
\- liquidated: Cancelled due to liquidation  
\- ioc: Not immediately fully filled due to IOC time-in-force setting  
\- auto_deleveraged: Cancelled due to auto-deleveraging  
\- reduce_only: Cancelled due to position increase while reduce-only is set  
\- position_closed: Cancelled because the position was closed  
\- reduce_out: Only reduce positions by excluding hard-to-fill orders  
\- mmp_cancelled: Cancelled by MMP | | » status | string | Order status

\- `open`: Pending  
\- `finished`: Completed | | » contract | string | Options identifier | | » size
| integer(int64) | Required. Trading quantity. Positive for buy, negative for
sell. Set to 0 for close position orders. | | » iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | » price | string | Order price. Price of 0
with `tif` set as `ioc` represents market order (quote currency) | | » is_close
| boolean | Is the order to close position | | » is_reduce_only | boolean | Is
the order reduce-only | | » is_liq | boolean | Is the order for liquidation | |
» is_mmp | boolean | Whether it is an MMP order. Corresponds to `mmp` in the
request | | » tif | string | Time in force strategy. Market orders currently
only support IOC mode

\- gtc: Good Till Cancelled  
\- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only  
\- poc: Pending Or Cancelled, passive order, maker only | | » left |
integer(int64) | Unfilled quantity | | » fill_price | string | Fill price | | »
text | string | User defined information. If not empty, must follow the rules
below:

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
\- insurance: from insurance | | » tkfr | string | Taker fee | | » mkfr | string
| Maker fee | | » refu | integer | Referrer user ID | | » refr | string |
Referrer rebate |

#### [#](#enumerated-values-130) Enumerated Values

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
| finish_as | mmp_cancelled    |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-single-order-4) Cancel single order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-4](https://www.gate.io/docs/developers/apiv4/en/#cancel-single-order-4)

> Code samples

`DELETE /options/orders/{order_id}`

_Cancel single order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-parameters)

| Name     | In   | Type           | Required | Description                                          |
| -------- | ---- | -------------- | -------- | ---------------------------------------------------- |
| order_id | path | integer(int64) | true     | Order ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "status": "finished",
  "size": -1,
  "id": 2,
  "iceberg": 0,
  "is_liq": false,
  "is_close": false,
  "is_mmp": false,
  "contract": "BTC_USDT-20210916-5000-C",
  "text": "-",
  "fill_price": "100",
  "finish_as": "filled",
  "left": 0,
  "tif": "gtc",
  "is_reduce_only": false,
  "create_time": 1631763361,
  "finish_time": 1631763397,
  "price": "100"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-responses](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-responses)

| Status | Meaning                                                                    | Description  | Schema |
| ------ | -------------------------------------------------------------------------- | ------------ | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order detail | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#canceloptionsorder-responseschema)

Status Code **200**

_Options order details_

| Name          | Type           | Description                                        |
| ------------- | -------------- | -------------------------------------------------- |
| » id          | integer(int64) | Options order ID                                   |
| » user        | integer        | User ID                                            |
| » create_time | number(double) | Creation time of order                             |
| » finish_time | number(double) | Order finished time. Not returned if order is open |
| » finish_as   | string         | Order finish reason:                               |

\- filled: Fully filled  
\- cancelled: User cancelled  
\- liquidated: Cancelled due to liquidation  
\- ioc: Not immediately fully filled due to IOC time-in-force setting  
\- auto_deleveraged: Cancelled due to auto-deleveraging  
\- reduce_only: Cancelled due to position increase while reduce-only is set  
\- position_closed: Cancelled because the position was closed  
\- reduce_out: Only reduce positions by excluding hard-to-fill orders  
\- mmp_cancelled: Cancelled by MMP | | » status | string | Order status

\- `open`: Pending  
\- `finished`: Completed | | » contract | string | Options identifier | | » size
| integer(int64) | Required. Trading quantity. Positive for buy, negative for
sell. Set to 0 for close position orders. | | » iceberg | integer(int64) |
Display size for iceberg orders. 0 for non-iceberg orders. Note that hidden
portions are charged taker fees. | | » price | string | Order price. Price of 0
with `tif` set as `ioc` represents market order (quote currency) | | » is_close
| boolean | Is the order to close position | | » is_reduce_only | boolean | Is
the order reduce-only | | » is_liq | boolean | Is the order for liquidation | |
» is_mmp | boolean | Whether it is an MMP order. Corresponds to `mmp` in the
request | | » tif | string | Time in force strategy. Market orders currently
only support IOC mode

\- gtc: Good Till Cancelled  
\- ioc: Immediate Or Cancelled, execute immediately or cancel, taker only  
\- poc: Pending Or Cancelled, passive order, maker only | | » left |
integer(int64) | Unfilled quantity | | » fill_price | string | Fill price | | »
text | string | User defined information. If not empty, must follow the rules
below:

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
\- insurance: from insurance | | » tkfr | string | Taker fee | | » mkfr | string
| Maker fee | | » refu | integer | Referrer user ID | | » refr | string |
Referrer rebate |

#### [#](#enumerated-values-131) Enumerated Values

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
| finish_as | mmp_cancelled    |
| status    | open             |
| status    | finished         |
| tif       | gtc              |
| tif       | ioc              |
| tif       | poc              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#countdown-cancel-orders-3) Countdown cancel orders

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders-3](https://www.gate.io/docs/developers/apiv4/en/#countdown-cancel-orders-3)

> Code samples

`POST /options/countdown_cancel_all`

_Countdown cancel orders_

Option order heartbeat detection, when the `timeout` time set by the user is
reached, if the existing countdown is not canceled or a new countdown is set,
the related `option pending order` will be automatically canceled. This
interface can be called repeatedly to set a new countdown or cancel the
countdown. Usage example: Repeat this interface at intervals of 30 seconds, with
each countdown `timeout` set to 30 (seconds). If this interface is not called
again within 30 seconds, all pending orders on the `underlying` `contract` you
specified will be automatically cancelled. If `underlying` `contract` is not
specified, user will be automatically cancelled If `timeout` is set to 0 within
30 seconds, the countdown timer will expire and the automatic order cancellation
function will be cancelled.

> Body parameter

```
{
  "timeout": 30,
  "contract": "BTC_USDT-20241001-46000-C",
  "underlying": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-parameters](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-parameters)

| Name         | In   | Type           | Required | Description               |
| ------------ | ---- | -------------- | -------- | ------------------------- |
| body         | body | object         | true     | none                      |
| » timeout    | body | integer(int32) | true     | Countdown time in seconds |
| » contract   | body | string         | false    | Options contract name     |
| » underlying | body | string         | false    | Underlying                |

#### [#](#detailed-descriptions-51) Detailed descriptions

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
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-responses](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-responses)

| Status | Meaning                                                                    | Description                | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Countdown set successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-responseschema](https://www.gate.io/docs/developers/apiv4/en/#countdowncancelalloptions-responseschema)

Status Code **200**

_triggerTime_

| Name          | Type           | Description                                    |
| ------------- | -------------- | ---------------------------------------------- |
| » triggerTime | integer(int64) | Timestamp when countdown ends, in milliseconds |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-records-4) Query personal trading records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-4](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-records-4)

> Code samples

`GET /options/my_trades`

_Query personal trading records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-parameters)

| Name       | In    | Type           | Required | Description                                          |
| ---------- | ----- | -------------- | -------- | ---------------------------------------------------- |
| underlying | query | string         | true     | Underlying (Obtained by listing underlying endpoint) |
| contract   | query | string         | false    | Options contract name                                |
| limit      | query | integer        | false    | Maximum number of records returned in a single list  |
| offset     | query | integer        | false    | List offset, starting from 0                         |
| from       | query | integer(int64) | false    | Start timestamp                                      |
| to         | query | integer(int64) | false    | Termination Timestamp                                |

#### [#](#detailed-descriptions-52) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

> Example responses

> 200 Response

```
[
  {
    "underlying_price": "48000",
    "size": 1,
    "contract": "BTC_USDT-20210916-5000-C",
    "id": 1,
    "role": "taker",
    "create_time": 1631763397,
    "order_id": 4,
    "price": "100"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-responses](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmyoptionstrades-responseschema)

Status Code **200**

| Name               | Type           | Description                              |
| ------------------ | -------------- | ---------------------------------------- |
| » id               | integer(int64) | Fill ID                                  |
| » create_time      | number(double) | Fill Time                                |
| » contract         | string         | Options contract name                    |
| » order_id         | integer        | Related order ID                         |
| » size             | integer(int64) | Trading size                             |
| » price            | string         | Trade price (quote currency)             |
| » underlying_price | string         | Underlying price (quote currency)        |
| » role             | string         | Trade role. taker - taker, maker - maker |

#### [#](#enumerated-values-132) Enumerated Values

| Property | Value |
| -------- | ----- |
| role     | taker |
| role     | maker |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#mmp-settings) MMP Settings

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#mmp-settings](https://www.gate.io/docs/developers/apiv4/en/#mmp-settings)

> Code samples

`POST /options/mmp`

_MMP Settings_

> Body parameter

```
{
  "underlying": "BTC_USDT",
  "window": 5000,
  "frozen_period": 200,
  "qty_limit": "10",
  "delta_limit": "10"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-parameters](https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-parameters)

| Name            | In   | Type           | Required | Description                                                                               |
| --------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------------------- |
| body            | body | object         | true     | none                                                                                      |
| » underlying    | body | string         | true     | Underlying                                                                                |
| » window        | body | integer(int32) | true     | Time window (milliseconds), between 1-5000, 0 means disable MMP                           |
| » frozen_period | body | integer(int32) | true     | Freeze duration (milliseconds), 0 means always frozen, need to call reset API to unfreeze |
| » qty_limit     | body | string         | true     | Trading volume upper limit (positive number, up to 2 decimal places)                      |
| » delta_limit   | body | string         | true     | Upper limit of net delta value (positive number, up to 2 decimal places)                  |

> Example responses

> 200 Response

```
{
  "underlying": "BTC_USDT",
  "window": 5000,
  "frozen_period": 200,
  "qty_limit": "10",
  "delta_limit": "10",
  "trigger_time_ms": 0,
  "frozen_until_ms": 0
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-responses](https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-responses)

| Status | Meaning                                                                    | Description     | Schema |
| ------ | -------------------------------------------------------------------------- | --------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | MMP Information | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-responseschema](https://www.gate.io/docs/developers/apiv4/en/#setoptionsmmp-responseschema)

Status Code **200**

_MMP Settings_

| Name              | Type           | Description                                                                                                                          |
| ----------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| » underlying      | string         | Underlying                                                                                                                           |
| » window          | integer(int32) | Time window (milliseconds), between 1-5000, 0 means disable MMP                                                                      |
| » frozen_period   | integer(int32) | Freeze duration (milliseconds), 0 means always frozen, need to call reset API to unfreeze                                            |
| » qty_limit       | string         | Trading volume upper limit (positive number, up to 2 decimal places)                                                                 |
| » delta_limit     | string         | Upper limit of net delta value (positive number, up to 2 decimal places)                                                             |
| » trigger_time_ms | integer(int64) | Trigger freeze time (milliseconds), 0 means no freeze is triggered                                                                   |
| » frozen_until_ms | integer(int64) | Unfreeze time (milliseconds). If the freeze duration is not configured, there will be no unfreeze time after the freeze is triggered |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#mmp-query) MMP Query.

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#mmp-query](https://www.gate.io/docs/developers/apiv4/en/#mmp-query)

> Code samples

`GET /options/mmp`

_MMP Query._

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-parameters](https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-parameters)

| Name       | In    | Type   | Required | Description |
| ---------- | ----- | ------ | -------- | ----------- |
| underlying | query | string | false    | Underlying  |

> Example responses

> 200 Response

```
[
  {
    "underlying": "BTC_USDT",
    "window": 5000,
    "frozen_period": 200,
    "qty_limit": "10",
    "delta_limit": "10",
    "trigger_time_ms": 0,
    "frozen_until_ms": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-responses](https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getoptionsmmp-responseschema)

Status Code **200**

| Name               | Type           | Description                                                                                                                          |
| ------------------ | -------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| _None_             | array          | \[MMP Settings\]                                                                                                                     |
| » _None_           | object         | MMP Settings                                                                                                                         |
| »» underlying      | string         | Underlying                                                                                                                           |
| »» window          | integer(int32) | Time window (milliseconds), between 1-5000, 0 means disable MMP                                                                      |
| »» frozen_period   | integer(int32) | Freeze duration (milliseconds), 0 means always frozen, need to call reset API to unfreeze                                            |
| »» qty_limit       | string         | Trading volume upper limit (positive number, up to 2 decimal places)                                                                 |
| »» delta_limit     | string         | Upper limit of net delta value (positive number, up to 2 decimal places)                                                             |
| »» trigger_time_ms | integer(int64) | Trigger freeze time (milliseconds), 0 means no freeze is triggered                                                                   |
| »» frozen_until_ms | integer(int64) | Unfreeze time (milliseconds). If the freeze duration is not configured, there will be no unfreeze time after the freeze is triggered |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#mmp-reset) MMP Reset

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#mmp-reset](https://www.gate.io/docs/developers/apiv4/en/#mmp-reset)

> Code samples

`POST /options/mmp/reset`

_MMP Reset_

> Body parameter

```
{
  "underlying": "BTC_USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-parameters](https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-parameters)

| Name         | In   | Type   | Required | Description |
| ------------ | ---- | ------ | -------- | ----------- |
| body         | body | object | true     | none        |
| » underlying | body | string | true     | Underlying  |

> Example responses

> 200 Response

```
{
  "underlying": "BTC_USDT",
  "window": 5000,
  "frozen_period": 200,
  "qty_limit": "10",
  "delta_limit": "10",
  "trigger_time_ms": 0,
  "frozen_until_ms": 0
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-responses](https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-responses)

| Status | Meaning                                                                    | Description     | Schema |
| ------ | -------------------------------------------------------------------------- | --------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | MMP Information | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-responseschema](https://www.gate.io/docs/developers/apiv4/en/#resetoptionsmmp-responseschema)

Status Code **200**

_MMP Settings_

| Name              | Type           | Description                                                                                                                          |
| ----------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| » underlying      | string         | Underlying                                                                                                                           |
| » window          | integer(int32) | Time window (milliseconds), between 1-5000, 0 means disable MMP                                                                      |
| » frozen_period   | integer(int32) | Freeze duration (milliseconds), 0 means always frozen, need to call reset API to unfreeze                                            |
| » qty_limit       | string         | Trading volume upper limit (positive number, up to 2 decimal places)                                                                 |
| » delta_limit     | string         | Upper limit of net delta value (positive number, up to 2 decimal places)                                                             |
| » trigger_time_ms | integer(int64) | Trigger freeze time (milliseconds), 0 means no freeze is triggered                                                                   |
| » frozen_until_ms | integer(int64) | Unfreeze time (milliseconds). If the freeze duration is not configured, there will be no unfreeze time after the freeze is triggered |

WARNING

To perform this operation, you must be authenticated by API key and secret
