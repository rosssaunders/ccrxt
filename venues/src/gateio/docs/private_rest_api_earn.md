# [#](#earn) Earn

Earn service

## [#](#eth2-swap) ETH2 swap

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#eth2-swap](https://www.gate.io/docs/developers/apiv4/en/#eth2-swap)

> Code samples

`POST /earn/staking/eth2/swap`

_ETH2 swap_

> Body parameter

```
{
  "side": "1",
  "amount": "1.5"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#swapeth2-parameters](https://www.gate.io/docs/developers/apiv4/en/#swapeth2-parameters)

| Name     | In   | Type   | Required | Description                                                |
| -------- | ---- | ------ | -------- | ---------------------------------------------------------- |
| body     | body | object | true     | none                                                       |
| » side   | body | string | true     | 1-Forward Swap (ETH -> ETH2), 2-Reverse Swap (ETH2 -> ETH) |
| » amount | body | string | true     | Swap Amount                                                |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#swapeth2-responses](https://www.gate.io/docs/developers/apiv4/en/#swapeth2-responses)

| Status | Meaning                                                                    | Description     | Schema |
| ------ | -------------------------------------------------------------------------- | --------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Swap successful | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#eth2-historical-return-rate-query) ETH2 historical return rate query

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#eth2-historical-return-rate-query](https://www.gate.io/docs/developers/apiv4/en/#eth2-historical-return-rate-query)

> Code samples

`GET /earn/staking/eth2/rate_records`

_ETH2 historical return rate query_

Query ETH earnings rate records for the last 31 days

> Example responses

> 200 Response

```
[
  {
    "date_time": 1690348815,
    "date": "2023-07-26",
    "rate": "60.00"
  },
  {
    "date_time": 1690435215,
    "date": "2023-07-27",
    "rate": "20.00"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#ratelisteth2-responses](https://www.gate.io/docs/developers/apiv4/en/#ratelisteth2-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#ratelisteth2-responseschema](https://www.gate.io/docs/developers/apiv4/en/#ratelisteth2-responseschema)

Status Code **200**

| Name        | Type           | Description     |
| ----------- | -------------- | --------------- |
| » date_time | integer(int64) | Date Timestamp  |
| » date      | string         | Date            |
| » rate      | string         | Percentage Rate |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#dual-investment-product-list) Dual Investment product list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#dual-investment-product-list](https://www.gate.io/docs/developers/apiv4/en/#dual-investment-product-list)

> Code samples

`GET /earn/dual/investment_plan`

_Dual Investment product list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-parameters)

| Name    | In    | Type           | Required | Description          |
| ------- | ----- | -------------- | -------- | -------------------- |
| plan_id | query | integer(int64) | false    | Financial project ID |

> Example responses

> 200 Response

```
[
  {
    "id": 272,
    "instrument_name": "DOGE-17NOV23-0.067-P",
    "type": "put",
    "invest_currency": "USDT",
    "exercise_currency": "DOGE",
    "exercise_price": 0.067,
    "delivery_time": 1700208000,
    "min_copies": 1,
    "max_copies": 1000,
    "start_time": 1697685172,
    "end_time": 1697685172,
    "status": "ONGOING",
    "apy_display": "0.0114000000",
    "per_value": "1"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-responses](https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdualinvestmentplans-responseschema)

Status Code **200**

| Name                | Type           | Description      |
| ------------------- | -------------- | ---------------- |
| » id                | integer(int32) | Product ID       |
| » instrument_name   | string         | Product Name     |
| » invest_currency   | string         | Investment Token |
| » exercise_currency | string         | Strike Token     |
| » exercise_price    | number(double) | Strike price     |
| » delivery_time     | integer(int32) | Settlement time  |
| » min_copies        | integer(int32) | Minimum Units    |
| » max_copies        | integer(int32) | Maximum Units    |
| » per_value         | string         | Value Per Unit   |
| » apy_display       | string         | Annual Yield     |
| » start_time        | integer(int32) | Start Time       |
| » end_time          | integer(int32) | End time         |
| » status            | string         | Status:          |

`NOTSTARTED`\-Not Started  
`ONGOING`\-In Progress  
`ENDED`\-Ended |

This operation does not require authentication

## [#](#dual-investment-order-list) Dual Investment order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#dual-investment-order-list](https://www.gate.io/docs/developers/apiv4/en/#dual-investment-order-list)

> Code samples

`GET /earn/dual/orders`

_Dual Investment order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdualorders-parameters)

| Name  | In    | Type           | Required | Description                                         |
| ----- | ----- | -------------- | -------- | --------------------------------------------------- |
| from  | query | integer(int64) | false    | Start settlement time                               |
| to    | query | integer(int64) | false    | End settlement time                                 |
| page  | query | integer(int32) | false    | Page number                                         |
| limit | query | integer        | false    | Maximum number of records returned in a single list |

> Example responses

> 200 Response

```
[
  {
    "id": 373,
    "plan_id": 176,
    "copies": "1.0000000000",
    "invest_amount": "0.0000000000",
    "settlement_amount": "0.0000000000",
    "create_time": 1697685172,
    "complete_time": 1697685172,
    "status": "CANCELED",
    "invest_currency": "USDT",
    "exercise_currency": "BTC",
    "settlement_currency": "",
    "exercise_price": "24500.0000000000",
    "settlement_price": "0.0000000000",
    "delivery_time": 1697685172,
    "apy_display": "0.6800000000",
    "apy_settlement": "0.0000000000",
    "text": "t-custom-text"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listdualorders-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdualorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdualorders-responseschema)

Status Code **200**

| Name                | Type           | Description         |
| ------------------- | -------------- | ------------------- |
| » id                | integer(int32) | Order ID            |
| » plan_id           | integer(int32) | Product ID          |
| » copies            | string         | Units               |
| » invest_amount     | string         | Investment Quantity |
| » settlement_amount | string         | Settlement Quantity |
| » create_time       | integer(int32) | Creation time       |
| » complete_time     | integer(int32) | Completed Time      |
| » status            | string         | Status:             |

`INIT`\-Created  
`SETTLEMENT_SUCCESS`\-Settlement Success  
`SETTLEMENT_PROCESSING`\-Settlement Processing  
`CANCELED`\-Canceled  
`FAILED`\-Failed | | » invest_currency | string | Investment Token | | »
exercise_currency | string | Strike Token | | » exercise_price | string | Strike
price | | » settlement_price | string | Settlement price | | »
settlement_currency | string | Settlement currency | | » apy_display | string |
Annual Yield | | » apy_settlement | string | Settlement Annual Yield | | »
delivery_time | integer(int32) | Settlement time | | » text | string | Custom
order information |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#place-dual-investment-order) Place Dual Investment order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-dual-investment-order](https://www.gate.io/docs/developers/apiv4/en/#place-dual-investment-order)

> Code samples

`POST /earn/dual/orders`

_Place Dual Investment order_

> Body parameter

```
{
  "plan_id": "176",
  "amount": "1",
  "text": "t-custom-text"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#placedualorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#placedualorder-parameters)

| Name      | In   | Type   | Required | Description                                                                                                          |
| --------- | ---- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------- |
| body      | body | object | true     | none                                                                                                                 |
| » plan_id | body | string | true     | Product ID                                                                                                           |
| » amount  | body | string | true     | Subscription amount, mutually exclusive with copies field                                                            |
| » text    | body | string | false    | Order custom information. Users can set custom ID with this field. Custom fields must meet the following conditions: |

#### [#](#detailed-descriptions-56) Detailed descriptions

**» text**: Order custom information. Users can set custom ID with this field.
Custom fields must meet the following conditions:

1.  Must start with `t-`
2.  Excluding `t-`, length cannot exceed 28 bytes
3.  Can only contain numbers, letters, underscore(\_), hyphen(-) or dot(.)

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#placedualorder-responses](https://www.gate.io/docs/developers/apiv4/en/#placedualorder-responses)

| Status | Meaning                                                                    | Description               | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order placed successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#structured-product-list) Structured Product List

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#structured-product-list](https://www.gate.io/docs/developers/apiv4/en/#structured-product-list)

> Code samples

`GET /earn/structured/products`

_Structured Product List_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-parameters](https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-parameters)

| Name   | In    | Type           | Required | Description                                         |
| ------ | ----- | -------------- | -------- | --------------------------------------------------- |
| type   | query | string         | false    | Product Type (Default empty to query all)           |
| status | query | string         | true     | Status (Default empty to query all)                 |
| page   | query | integer(int32) | false    | Page number                                         |
| limit  | query | integer        | false    | Maximum number of records returned in a single list |

#### [#](#detailed-descriptions-57) Detailed descriptions

**type**: Product Type (Default empty to query all)

`SharkFin2.0`\-Shark Fin `BullishSharkFin`\-Bullish Treasure
`BearishSharkFin`\-Bearish Treasure `DoubleNoTouch`\-Volatility Treasure
`RangeAccrual`\-Range Smart Yield `SnowBall`\-Snowball

**status**: Status (Default empty to query all)

`in_process`\-In progress `will_begin`\-Not started `wait_settlement`\-Pending
settlement `done`\-Completed

> Example responses

> 200 Response

```
[
  {
    "id": 3700,
    "type": "BullishSharkFin",
    "name_en": "Bullish Sharkfin_USDT",
    "investment_period": 7,
    "min_annual_rate": "0.50",
    "mid_annual_rate": "7.50",
    "max_annual_rate": "13.00",
    "watch_market": "BTC_USDT",
    "investment_coin": "USDT",
    "start_time": 1698224400,
    "end_time": 1700902800,
    "status": "in_process"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-responses](https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#liststructuredproducts-responseschema)

Status Code **200**

| Name     | Type           | Description           |
| -------- | -------------- | --------------------- |
| » _None_ | object         | Structured Investment |
| »» id    | integer(int32) | Product ID            |
| »» type  | string         | Product Type:         |

`SharkFin2.0`\-Shark Fin 2.0  
`BullishSharkFin`\-Bullish Shark Fin  
`BearishSharkFin`\-Bearish Shark Fin  
`DoubleNoTouch`\-Double No-Touch  
`RangeAccrual`\-Range Accrual  
`SnowBall`\-Snow Ball | | »» name_en | string | Product Name | | »»
investment_coin | string | Investment Token | | »» investment_period | string |
Investment Period | | »» min_annual_rate | string | Minimum Annual Rate | | »»
mid_annual_rate | string | Intermediate Annual Rate | | »» max_annual_rate |
string | Maximum Annual Rate | | »» watch_market | string | Underlying Market |
| »» start_time | integer(int32) | Start Time | | »» end_time | integer(int32) |
End time | | »» status | string | Status:

`in_process`\-in progress  
`will_begin`\-will begin  
`wait_settlement`\-waiting for settlement  
`done`\-done |

This operation does not require authentication

## [#](#structured-product-order-list) Structured Product Order List

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#structured-product-order-list](https://www.gate.io/docs/developers/apiv4/en/#structured-product-order-list)

> Code samples

`GET /earn/structured/orders`

_Structured Product Order List_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-parameters)

| Name  | In    | Type           | Required | Description                                         |
| ----- | ----- | -------------- | -------- | --------------------------------------------------- |
| from  | query | integer(int64) | false    | Start timestamp                                     |
| to    | query | integer(int64) | false    | Termination Timestamp                               |
| page  | query | integer(int32) | false    | Page number                                         |
| limit | query | integer        | false    | Maximum number of records returned in a single list |

#### [#](#detailed-descriptions-58) Detailed descriptions

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
    "id": 35,
    "pid": "3691",
    "lock_coin": "ETH",
    "amount": "20",
    "status": "SUCCESS",
    "income": "0.000000",
    "create_time": 1697685172
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-responses](https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#liststructuredorders-responseschema)

Status Code **200**

| Name         | Type           | Description      |
| ------------ | -------------- | ---------------- |
| » _None_     | object         | Structured order |
| »» id        | integer(int32) | Order ID         |
| »» pid       | string         | Product ID       |
| »» lock_coin | string         | Locked coin      |
| »» amount    | string         | Locked amount    |
| »» status    | string         | Status:          |

SUCCESS - SUCCESS  
FAILED - FAILED  
DONE - DONE | | »» income | string | Income | | »» create_time | integer(int32)
| Created time |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#place-structured-product-order) Place Structured Product Order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-structured-product-order](https://www.gate.io/docs/developers/apiv4/en/#place-structured-product-order)

> Code samples

`POST /earn/structured/orders`

_Place Structured Product Order_

> Body parameter

```
{
  "pid": "1",
  "amount": "0.5"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#placestructuredorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#placestructuredorder-parameters)

| Name     | In   | Type   | Required | Description  |
| -------- | ---- | ------ | -------- | ------------ |
| body     | body | object | true     | none         |
| » pid    | body | string | false    | Product ID   |
| » amount | body | string | false    | Buy Quantity |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#placestructuredorder-responses](https://www.gate.io/docs/developers/apiv4/en/#placestructuredorder-responses)

| Status | Meaning                                                                    | Description               | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order placed successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#staking-coins) Staking coins

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#staking-coins](https://www.gate.io/docs/developers/apiv4/en/#staking-coins)

> Code samples

`GET /earn/staking/coins`

_Staking coins_

> Body parameter

```
{
  "coin": "string",
  "cointype": "string"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#findcoin-parameters](https://www.gate.io/docs/developers/apiv4/en/#findcoin-parameters)

| Name       | In   | Type   | Required | Description                           |
| ---------- | ---- | ------ | -------- | ------------------------------------- |
| body       | body | object | true     | none                                  |
| » coin     | body | string | false    | Currency                              |
| » cointype | body | string | false    | Token Type: swap-Voucher, lock-Locked |

> Example responses

> 200 Response

```
[
  "GT",
  "SOL",
  "USDT",
  "ALEO",
  "DOT",
  "TRX",
  "ADA"
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#findcoin-responses](https://www.gate.io/docs/developers/apiv4/en/#findcoin-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[string\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#findcoin-responseschema](https://www.gate.io/docs/developers/apiv4/en/#findcoin-responseschema)

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#on-chain-token-swap-for-earned-coins) On-chain token swap for earned coins

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#on-chain-token-swap-for-earned-coins](https://www.gate.io/docs/developers/apiv4/en/#on-chain-token-swap-for-earned-coins)

> Code samples

`POST /earn/staking/swap`

_On-chain token swap for earned coins_

> Body parameter

```
{
  "coin": "GT",
  "side": "0",
  "amount": "1.5"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-parameters](https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-parameters)

| Name     | In   | Type           | Required | Description                          |
| -------- | ---- | -------------- | -------- | ------------------------------------ |
| body     | body | object         | true     | none                                 |
| » coin   | body | string         | true     | Currency                             |
| » side   | body | string         | true     | 0 - Stake 1 - Redeem                 |
| » amount | body | string         | true     | Amount                               |
| » pid    | body | integer(int32) | false    | DeFi-type Mining Protocol Identifier |

> Example responses

> 200 Response

```
{
  "id": 21000,
  "uid": 12345,
  "coin": "GT",
  "type": 0,
  "exchange_rate": "1.00000000",
  "amount": "2",
  "pid": 1,
  "status": 1,
  "createStamp": 1752200661
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-responses](https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-responses)

| Status | Meaning                                                                    | Description     | Schema |
| ------ | -------------------------------------------------------------------------- | --------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Swap successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-responseschema](https://www.gate.io/docs/developers/apiv4/en/#swapstakingcoin-responseschema)

Status Code **200**

| Name              | Type    | Description                 |
| ----------------- | ------- | --------------------------- |
| » id              | integer | Order ID                    |
| » pid             | integer | Product ID                  |
| » uid             | integer | User ID                     |
| » coin            | string  | Currency                    |
| » type            | integer | Type 0-Staking 1-Redemption |
| » subtype         | string  | 子Type                      |
| » amount          | string  | Amount                      |
| » exchange_rate   | string  | Exchange ratio              |
| » exchange_amount | string  | Redemption Amount           |
| » updateStamp     | integer | 更新Timestamp               |
| » createStamp     | integer | Transaction timestamp       |
| » status          | integer | status 1-success            |
| » protocol_type   | integer | DEFI Protocol Type          |
| » client_order_id | string  | Reference ID                |
| » source          | string  | Order Origin                |

WARNING

To perform this operation, you must be authenticated by API key and secret
