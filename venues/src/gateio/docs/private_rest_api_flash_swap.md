# [#](#flash-swap) Flash_swap

Flash exchange

## [#](#list-all-supported-currency-pairs-in-flash-swap) List All Supported Currency Pairs In Flash Swap

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-supported-currency-pairs-in-flash-swap](https://www.gate.io/docs/developers/apiv4/en/#list-all-supported-currency-pairs-in-flash-swap)

> Code samples

`GET /flash_swap/currency_pairs`

_List All Supported Currency Pairs In Flash Swap_

`BTC_GT` represents selling BTC and buying GT. The limits for each currency may
vary across different currency pairs.

It is not necessary that two currencies that can be swapped instantaneously can
be exchanged with each other. For example, it is possible to sell BTC and buy
GT, but it does not necessarily mean that GT can be sold to buy BTC.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-parameters](https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-parameters)

| Name     | In    | Type           | Required | Description                                                                |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------- |
| currency | query | string         | false    | Query by specified currency name                                           |
| page     | query | integer(int32) | false    | Page number                                                                |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 1000, minimum: 1, maximum: 1000 |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "BTC_USDT",
    "sell_currency": "BTC",
    "buy_currency": "USDT",
    "sell_min_amount": "0.00001",
    "sell_max_amount": "100",
    "buy_min_amount": "10",
    "buy_max_amount": "10000000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-responses](https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listflashswapcurrencypair-responseschema)

Status Code **200**

| Name               | Type   | Description                                                          |
| ------------------ | ------ | -------------------------------------------------------------------- |
| » _None_           | object | List all supported currencies in flash swap                          |
| »» currency_pair   | string | Currency pair, `BTC_USDT` represents selling `BTC` and buying `USDT` |
| »» sell_currency   | string | Currency to sell                                                     |
| »» buy_currency    | string | Currency to buy                                                      |
| »» sell_min_amount | string | Minimum sell quantity                                                |
| »» sell_max_amount | string | Maximum sell quantity                                                |
| »» buy_min_amount  | string | Minimum buy quantity                                                 |
| »» buy_max_amount  | string | Maximum buy quantity                                                 |

This operation does not require authentication

## [#](#create-a-flash-swap-order) Create a flash swap order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-a-flash-swap-order](https://www.gate.io/docs/developers/apiv4/en/#create-a-flash-swap-order)

> Code samples

`POST /flash_swap/orders`

_Create a flash swap order_

Initiate a flash swap preview in advance because order creation requires a
preview result

> Body parameter

```
{
  "preview_id": "4564564",
  "sell_currency": "BTC",
  "sell_amount": "0.1",
  "buy_currency": "USDT",
  "buy_amount": "10"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-parameters](https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-parameters)

| Name            | In   | Type   | Required | Description                                                                                                                                           |
| --------------- | ---- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| body            | body | object | true     | none                                                                                                                                                  |
| » preview_id    | body | string | true     | Preview result ID                                                                                                                                     |
| » sell_currency | body | string | true     | Name of the asset to be sold, obtained from the interface GET /flash_swap/currency_pairs: Query the list of all trading pairs supporting flash swap   |
| » sell_amount   | body | string | true     | Amount to sell (based on the preview result)                                                                                                          |
| » buy_currency  | body | string | true     | Name of the asset to be bought, obtained from the interface GET /flash_swap/currency_pairs: Query the list of all trading pairs supporting flash swap |
| » buy_amount    | body | string | true     | Amount to buy (based on the preview result)                                                                                                           |

> Example responses

> 201 Response

```
{
  "id": 54646,
  "create_time": 1651116876378,
  "update_time": 1651116876378,
  "user_id": 11135567,
  "sell_currency": "BTC",
  "sell_amount": "0.01",
  "buy_currency": "USDT",
  "buy_amount": "10",
  "price": "100",
  "status": 1
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-responses](https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-responses)

| Status | Meaning                                                                         | Description                           | Schema |
| ------ | ------------------------------------------------------------------------------- | ------------------------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Flash swap order created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createflashswaporder-responseschema)

Status Code **201**

_Flash swap order_

| Name            | Type           | Description                              |
| --------------- | -------------- | ---------------------------------------- |
| » id            | integer(int64) | Flash swap order ID                      |
| » create_time   | integer(int64) | Creation time of order (in milliseconds) |
| » user_id       | integer(int64) | User ID                                  |
| » sell_currency | string         | Currency to sell                         |
| » sell_amount   | string         | Amount to sell                           |
| » buy_currency  | string         | Currency to buy                          |
| » buy_amount    | string         | Amount to buy                            |
| » price         | string         | Price                                    |
| » status        | integer        | Flash swap order status                  |

`1` - success  
`2` - failure |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-flash-swap-order-list) Query flash swap order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-flash-swap-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-flash-swap-order-list)

> Code samples

`GET /flash_swap/orders`

_Query flash swap order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-parameters)

| Name          | In    | Type           | Required | Description                                                 |
| ------------- | ----- | -------------- | -------- | ----------------------------------------------------------- |
| status        | query | integer        | false    | Flash swap order status                                     |
| sell_currency | query | string         | false    | Asset name to sell                                          |
| buy_currency  | query | string         | false    | Asset name to buy                                           |
| reverse       | query | boolean        | false    | Sort by ID in ascending or descending order, default `true` |
| limit         | query | integer        | false    | Maximum number of records returned in a single list         |
| page          | query | integer(int32) | false    | Page number                                                 |

#### [#](#detailed-descriptions-21) Detailed descriptions

**status**: Flash swap order status

`1` - success `2` - failed

**sell_currency**: Asset name to sell

- Retrieved from API `GET /flash_swap/currencies` for supported flash swap
  currencies

**buy_currency**: Asset name to buy

- Retrieved from API `GET /flash_swap/currencies` for supported flash swap
  currencies

**reverse**: Sort by ID in ascending or descending order, default `true`

- `true`: ID descending order (most recent data first)
- `false`: ID ascending order (oldest data first)

> Example responses

> 200 Response

```
[
  {
    "id": 54646,
    "create_time": 1651116876378,
    "update_time": 1651116876378,
    "user_id": 11135567,
    "sell_currency": "BTC",
    "sell_amount": "0.01",
    "buy_currency": "USDT",
    "buy_amount": "10",
    "price": "100",
    "status": 1
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-responses](https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listflashswaporders-responseschema)

Status Code **200**

| Name             | Type           | Description                              |
| ---------------- | -------------- | ---------------------------------------- |
| _None_           | array          | \[Flash swap order\]                     |
| » _None_         | object         | Flash swap order                         |
| »» id            | integer(int64) | Flash swap order ID                      |
| »» create_time   | integer(int64) | Creation time of order (in milliseconds) |
| »» user_id       | integer(int64) | User ID                                  |
| »» sell_currency | string         | Currency to sell                         |
| »» sell_amount   | string         | Amount to sell                           |
| »» buy_currency  | string         | Currency to buy                          |
| »» buy_amount    | string         | Amount to buy                            |
| »» price         | string         | Price                                    |
| »» status        | integer        | Flash swap order status                  |

`1` - success  
`2` - failure |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-flash-swap-order) Query single flash swap order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-flash-swap-order](https://www.gate.io/docs/developers/apiv4/en/#query-single-flash-swap-order)

> Code samples

`GET /flash_swap/orders/{order_id}`

_Query single flash swap order_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-parameters](https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-parameters)

| Name     | In   | Type    | Required | Description         |
| -------- | ---- | ------- | -------- | ------------------- |
| order_id | path | integer | true     | Flash swap order ID |

> Example responses

> 200 Response

```
{
  "id": 54646,
  "create_time": 1651116876378,
  "update_time": 1651116876378,
  "user_id": 11135567,
  "sell_currency": "BTC",
  "sell_amount": "0.01",
  "buy_currency": "USDT",
  "buy_amount": "10",
  "price": "100",
  "status": 1
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-responses](https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getflashswaporder-responseschema)

Status Code **200**

_Flash swap order_

| Name            | Type           | Description                              |
| --------------- | -------------- | ---------------------------------------- |
| » id            | integer(int64) | Flash swap order ID                      |
| » create_time   | integer(int64) | Creation time of order (in milliseconds) |
| » user_id       | integer(int64) | User ID                                  |
| » sell_currency | string         | Currency to sell                         |
| » sell_amount   | string         | Amount to sell                           |
| » buy_currency  | string         | Currency to buy                          |
| » buy_amount    | string         | Amount to buy                            |
| » price         | string         | Price                                    |
| » status        | integer        | Flash swap order status                  |

`1` - success  
`2` - failure |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#flash-swap-order-preview) Flash swap order preview

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#flash-swap-order-preview](https://www.gate.io/docs/developers/apiv4/en/#flash-swap-order-preview)

> Code samples

`POST /flash_swap/orders/preview`

_Flash swap order preview_

> Body parameter

```
{
  "sell_currency": "BTC",
  "sell_amount": "0.1",
  "buy_currency": "USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-parameters](https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-parameters)

| Name            | In   | Type   | Required | Description                                                                                                                                                    |
| --------------- | ---- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body            | body | object | true     | none                                                                                                                                                           |
| » sell_currency | body | string | true     | The name of the asset being sold, as obtained from the "GET /flash_swap/currency_pairs" API, which retrieves a list of supported flash swap currency pairs     |
| » sell_amount   | body | string | false    | Amount to sell.                                                                                                                                                |
| » buy_currency  | body | string | true     | The name of the asset being purchased, as obtained from the "GET /flash_swap/currency_pairs" API, which provides a list of supported flash swap currency pairs |
| » buy_amount    | body | string | false    | Amount to buy.                                                                                                                                                 |

#### [#](#detailed-descriptions-22) Detailed descriptions

**» sell_amount**: Amount to sell. It is required to choose one parameter
between `sell_amount` and `buy_amount`

**» buy_amount**: Amount to buy. It is required to choose one parameter between
`sell_amount` and `buy_amount`

> Example responses

> 200 Response

```
{
  "preview_id": "3453434",
  "sell_currency": "BTC",
  "sell_amount": "0.1",
  "buy_currency": "USDT",
  "buy_amount": "10",
  "price": "100"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-responses](https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-responses)

| Status | Meaning                                                                    | Description                         | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Flash swap order preview successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#previewflashswaporder-responseschema)

Status Code **200**

_Flash swap order preview_

| Name                                                                                                             | Type   | Description                  |
| ---------------------------------------------------------------------------------------------------------------- | ------ | ---------------------------- |
| » preview_id                                                                                                     | string | Preview result ID            |
| » sell_currency                                                                                                  | string | Name of the sold asset,      |
| Refer to the interface Query the list of currencies supported for flash swap GET /flash_swap/currenciesto obtain |
| » sell_amount                                                                                                    | string | Amount to sell               |
| » buy_currency                                                                                                   | string | Name of the purchased asset, |
| Refer to the interface Query the list of currencies supported for flash swap GET /flash_swap/currenciesto obtain |
| » buy_amount                                                                                                     | string | Amount to buy                |
| » price                                                                                                          | string | Price                        |

WARNING

To perform this operation, you must be authenticated by API key and secret
