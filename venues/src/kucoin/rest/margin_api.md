---
title: KuCoin Margin REST API v1.0.0
language_tabs:
  - javascript: JavaScript
  - python: Python
toc_footers: []
includes: []
search: true
highlight_theme: darkula
headingLevel: 2
---

<!-- Generator: Widdershins v4.0.1 -->

<h1 id="kucoin-margin-rest-api">KuCoin Margin REST API v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

market

<h1 id="kucoin-margin-rest-api-default">Default</h1>

## Get Symbols - Cross Margin

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/margin/symbols", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/margin/symbols', headers = headers)

print(r.json())

```

`GET /api/v3/margin/symbols`

This endpoint allows querying the configuration of cross margin symbol.

<h3 id="get-symbols---cross-margin-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                       |
| ------ | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------- |
| symbol | query | string | false    | If not provided, all cross margin symbol will be queried. If provided, only the specified symbol will be queried. |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "timestamp": {
          "type": "integer",
          "format": "int64"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "symbol"
              },
              "name": {
                "type": "string",
                "description": "Symbol name"
              },
              "enableTrading": {
                "type": "boolean",
                "description": "Whether trading is enabled: True for enabled; false for disabled"
              },
              "market": {
                "type": "string",
                "description": "Trading market"
              },
              "baseCurrency": {
                "type": "string",
                "description": "Base currency, e.g. BTC."
              },
              "quoteCurrency": {
                "type": "string",
                "description": "Quote currency, e.g. USDT."
              },
              "baseIncrement": {
                "type": "string",
                "description": "Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001."
              },
              "baseMinSize": {
                "type": "string",
                "description": "The minimum order quantity required to place an order."
              },
              "quoteIncrement": {
                "type": "string",
                "description": "Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001."
              },
              "quoteMinSize": {
                "type": "string",
                "description": "The minimum order funds required to place a market order."
              },
              "baseMaxSize": {
                "type": "string",
                "description": "The maximum order size required to place an order."
              },
              "quoteMaxSize": {
                "type": "string",
                "description": "The maximum order funds required to place a market order."
              },
              "priceIncrement": {
                "type": "string",
                "description": "Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001.\n\nSpecifies the min. order price as well as the price increment.This also applies to quote currency."
              },
              "feeCurrency": {
                "type": "string",
                "description": "The currency of charged fees."
              },
              "priceLimitRate": {
                "type": "string",
                "description": "Threshold for price protection"
              },
              "minFunds": {
                "type": "string",
                "description": "The minimum trading amounts"
              }
            },
            "required": [
              "symbol",
              "name",
              "enableTrading",
              "market",
              "baseCurrency",
              "quoteCurrency",
              "baseIncrement",
              "baseMinSize",
              "quoteIncrement",
              "quoteMinSize",
              "baseMaxSize",
              "quoteMaxSize",
              "priceIncrement",
              "feeCurrency",
              "priceLimitRate",
              "minFunds"
            ]
          }
        }
      },
      "required": ["timestamp", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-symbols---cross-margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-symbols---cross-margin-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                         |
| ------------------ | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| »» timestamp       | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                              |
| »»» name           | string         | true     | none         | Symbol name                                                                                                                                                                                                                                                                                                                         |
| »»» enableTrading  | boolean        | true     | none         | Whether trading is enabled: True for enabled; false for disabled                                                                                                                                                                                                                                                                    |
| »»» market         | string         | true     | none         | Trading market                                                                                                                                                                                                                                                                                                                      |
| »»» baseCurrency   | string         | true     | none         | Base currency, e.g. BTC.                                                                                                                                                                                                                                                                                                            |
| »»» quoteCurrency  | string         | true     | none         | Quote currency, e.g. USDT.                                                                                                                                                                                                                                                                                                          |
| »»» baseIncrement  | string         | true     | none         | Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001.               |
| »»» baseMinSize    | string         | true     | none         | The minimum order quantity required to place an order.                                                                                                                                                                                                                                                                              |
| »»» quoteIncrement | string         | true     | none         | Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001.                         |
| »»» quoteMinSize   | string         | true     | none         | The minimum order funds required to place a market order.                                                                                                                                                                                                                                                                           |
| »»» baseMaxSize    | string         | true     | none         | The maximum order size required to place an order.                                                                                                                                                                                                                                                                                  |
| »»» quoteMaxSize   | string         | true     | none         | The maximum order funds required to place a market order.                                                                                                                                                                                                                                                                           |
| »»» priceIncrement | string         | true     | none         | Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001.<br><br>Specifies the min. order price as well as the price increment.This also applies to quote currency. |
| »»» feeCurrency    | string         | true     | none         | The currency of charged fees.                                                                                                                                                                                                                                                                                                       |
| »»» priceLimitRate | string         | true     | none         | Threshold for price protection                                                                                                                                                                                                                                                                                                      |
| »»» minFunds       | string         | true     | none         | The minimum trading amounts                                                                                                                                                                                                                                                                                                         |

<aside class="success">
This operation does not require authentication
</aside>

## Get ETF Info

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/etf/info", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/etf/info', headers = headers)

print(r.json())

```

`GET /api/v3/etf/info`

This interface returns leveraged token information.

<h3 id="get-etf-info-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description                                  |
| -------- | ----- | ------ | -------- | -------------------------------------------- |
| currency | query | string | false    | ETF Currency: If empty, query all currencies |

#### Detailed descriptions

**currency**: ETF Currency: If empty, query all currencies

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "currency": {
            "type": "string",
            "description": "ETF Currency"
          },
          "netAsset": {
            "type": "string",
            "description": "Net worth"
          },
          "targetLeverage": {
            "type": "string",
            "description": "Target leverage"
          },
          "actualLeverage": {
            "type": "string",
            "description": "Actual leverage"
          },
          "issuedSize": {
            "type": "string",
            "description": "The amount of currency issued"
          },
          "basket": {
            "type": "string",
            "description": "Basket information"
          }
        },
        "required": [
          "currency",
          "netAsset",
          "targetLeverage",
          "actualLeverage",
          "issuedSize",
          "basket"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-etf-info-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-etf-info-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type     | Required | Restrictions | Description                   |
| ----------------- | -------- | -------- | ------------ | ----------------------------- |
| » code            | string   | true     | none         | none                          |
| » data            | [object] | true     | none         | none                          |
| »» currency       | string   | true     | none         | ETF Currency                  |
| »» netAsset       | string   | true     | none         | Net worth                     |
| »» targetLeverage | string   | true     | none         | Target leverage               |
| »» actualLeverage | string   | true     | none         | Actual leverage               |
| »» issuedSize     | string   | true     | none         | The amount of currency issued |
| »» basket         | string   | true     | none         | Basket information            |

<aside class="success">
This operation does not require authentication
</aside>

## Get Mark Price Detail

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/mark-price/{symbol}/current", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v1/mark-price/{symbol}/current', headers = headers)

print(r.json())

```

`GET /api/v1/mark-price/{symbol}/current`

This endpoint returns the current Mark price for specified margin trading pairs.

<h3 id="get-mark-price-detail-parameters">Parameters</h3>

| Name   | In   | Type   | Required | Description |
| ------ | ---- | ------ | -------- | ----------- |
| symbol | path | string | true     | symbol      |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "symbol": {
          "type": "string",
          "description": "symbol"
        },
        "timePoint": {
          "type": "integer",
          "description": "Timestamp (milliseconds)",
          "format": "int64"
        },
        "value": {
          "type": "number",
          "description": "Mark price"
        }
      },
      "required": ["symbol", "timePoint", "value"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-mark-price-detail-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-mark-price-detail-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type           | Required | Restrictions | Description              |
| ------------ | -------------- | -------- | ------------ | ------------------------ |
| » code       | string         | true     | none         | none                     |
| » data       | object         | true     | none         | none                     |
| »» symbol    | string         | true     | none         | symbol                   |
| »» timePoint | integer(int64) | true     | none         | Timestamp (milliseconds) |
| »» value     | number         | true     | none         | Mark price               |

<aside class="success">
This operation does not require authentication
</aside>

## Get Margin Config

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/margin/config", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v1/margin/config', headers = headers)

print(r.json())

```

`GET /api/v1/margin/config`

Request the configure info of the cross margin via this endpoint.

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "currencyList": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Available currencies for margin trade"
        },
        "maxLeverage": {
          "type": "integer",
          "description": "Max. leverage available"
        },
        "warningDebtRatio": {
          "type": "string",
          "description": "The warning debt ratio of the forced liquidation"
        },
        "liqDebtRatio": {
          "type": "string",
          "description": "The debt ratio of the forced liquidation"
        }
      },
      "required": [
        "currencyList",
        "maxLeverage",
        "warningDebtRatio",
        "liqDebtRatio"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-margin-config-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-margin-config-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type     | Required | Restrictions | Description                                      |
| ------------------- | -------- | -------- | ------------ | ------------------------------------------------ |
| » code              | string   | true     | none         | none                                             |
| » data              | object   | true     | none         | none                                             |
| »» currencyList     | [string] | true     | none         | Available currencies for margin trade            |
| »» maxLeverage      | integer  | true     | none         | Max. leverage available                          |
| »» warningDebtRatio | string   | true     | none         | The warning debt ratio of the forced liquidation |
| »» liqDebtRatio     | string   | true     | none         | The debt ratio of the forced liquidation         |

<aside class="success">
This operation does not require authentication
</aside>

## Get Mark Price List

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/mark-price/all-symbols", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/mark-price/all-symbols', headers = headers)

print(r.json())

```

`GET /api/v3/mark-price/all-symbols`

This endpoint returns the current Mark price for all margin trading pairs.

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "symbol"
          },
          "timePoint": {
            "type": "integer",
            "description": "Timestamp (milliseconds)",
            "format": "int64"
          },
          "value": {
            "type": "number",
            "description": "Mark price"
          }
        },
        "required": ["symbol", "timePoint", "value"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-mark-price-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-mark-price-list-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type           | Required | Restrictions | Description              |
| ------------ | -------------- | -------- | ------------ | ------------------------ |
| » code       | string         | true     | none         | none                     |
| » data       | [object]       | true     | none         | none                     |
| »» symbol    | string         | true     | none         | symbol                   |
| »» timePoint | integer(int64) | true     | none         | Timestamp (milliseconds) |
| »» value     | number         | true     | none         | Mark price               |

<aside class="success">
This operation does not require authentication
</aside>

## Get Symbols - Isolated Margin

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/isolated/symbols", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v1/isolated/symbols', headers = headers)

print(r.json())

```

`GET /api/v1/isolated/symbols`

This endpoint allows querying the configuration of isolated margin symbol.

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "symbol"
          },
          "symbolName": {
            "type": "string",
            "description": "Symbol name"
          },
          "baseCurrency": {
            "type": "string",
            "description": "Base currency, e.g. BTC."
          },
          "quoteCurrency": {
            "type": "string",
            "description": "Quote currency, e.g. USDT."
          },
          "maxLeverage": {
            "type": "integer",
            "description": "Max. leverage of this symbol"
          },
          "flDebtRatio": {
            "type": "string"
          },
          "tradeEnable": {
            "type": "boolean"
          },
          "autoRenewMaxDebtRatio": {
            "type": "string"
          },
          "baseBorrowEnable": {
            "type": "boolean"
          },
          "quoteBorrowEnable": {
            "type": "boolean"
          },
          "baseTransferInEnable": {
            "type": "boolean"
          },
          "quoteTransferInEnable": {
            "type": "boolean"
          },
          "baseBorrowCoefficient": {
            "type": "string"
          },
          "quoteBorrowCoefficient": {
            "type": "string"
          }
        },
        "required": [
          "symbol",
          "symbolName",
          "baseCurrency",
          "quoteCurrency",
          "maxLeverage",
          "flDebtRatio",
          "tradeEnable",
          "autoRenewMaxDebtRatio",
          "baseBorrowEnable",
          "quoteBorrowEnable",
          "baseTransferInEnable",
          "quoteTransferInEnable",
          "baseBorrowCoefficient",
          "quoteBorrowCoefficient"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-symbols---isolated-margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-symbols---isolated-margin-responseschema">Response Schema</h3>

Status Code **200**

| Name                      | Type     | Required | Restrictions | Description                  |
| ------------------------- | -------- | -------- | ------------ | ---------------------------- |
| » code                    | string   | true     | none         | none                         |
| » data                    | [object] | true     | none         | none                         |
| »» symbol                 | string   | true     | none         | symbol                       |
| »» symbolName             | string   | true     | none         | Symbol name                  |
| »» baseCurrency           | string   | true     | none         | Base currency, e.g. BTC.     |
| »» quoteCurrency          | string   | true     | none         | Quote currency, e.g. USDT.   |
| »» maxLeverage            | integer  | true     | none         | Max. leverage of this symbol |
| »» flDebtRatio            | string   | true     | none         | none                         |
| »» tradeEnable            | boolean  | true     | none         | none                         |
| »» autoRenewMaxDebtRatio  | string   | true     | none         | none                         |
| »» baseBorrowEnable       | boolean  | true     | none         | none                         |
| »» quoteBorrowEnable      | boolean  | true     | none         | none                         |
| »» baseTransferInEnable   | boolean  | true     | none         | none                         |
| »» quoteTransferInEnable  | boolean  | true     | none         | none                         |
| »» baseBorrowCoefficient  | string   | true     | none         | none                         |
| »» quoteBorrowCoefficient | string   | true     | none         | none                         |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order

<a id="opId001"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": [
        "buy",
        "sell"
      ],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": [
        "buy"
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
      "enum": [
        "DC",
        "CO",
        "CN",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": [
        "GTC",
        "GTT",
        "IOC",
        "FOK"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "[Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/hf/margin/order',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/hf/margin/order', headers = headers)

print(r.json())

```

`POST /api/v3/hf/margin/order`

Place order in the Cross-margin or Isolated-margin trading system. You can place
two major types of order: Limit and market. Orders can only be placed if your
account has sufficient funds. Once an order is placed, your funds will be put on
hold for the duration of the order. The amount of funds on hold depends on the
order type and parameters specified.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": ["buy", "sell"],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": ["buy"]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": ["limit", "market"],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
      "enum": ["DC", "CO", "CN", "CB"],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": ["GTC", "GTT", "IOC", "FOK"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "[Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » isIsolated  | body | boolean        | false    | True - isolated margin; false - cross margin. Default is false                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |

#### Detailed descriptions

**» clientOid**: Client Order ID: The ClientOid field is a unique ID created by
the user (we recommend using a UUID), and can only contain numbers, letters,
underscores (\_), and hyphens (-). This field is returned when order information
is obtained. You can use clientOid to tag your orders. ClientOid is different
from the order ID created by the service provider. Please do not initiate
requests using the same clientOid. The maximum length for the ClientOid is 40
characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: Specify if the order is a 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price; you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order.

When **type** is limited, size refers to the amount of trading targets (the
asset name written in front) for the trading pair. The Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value  |
| ------------- | ------ |
| » side        | buy    |
| » side        | sell   |
| » type        | limit  |
| » type        | market |
| » stp         | DC     |
| » stp         | CO     |
| » stp         | CN     |
| » stp         | CB     |
| » timeInForce | GTC    |
| » timeInForce | GTT    |
| » timeInForce | IOC    |
| » timeInForce | FOK    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderId": {
          "type": "string",
          "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
        },
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        }
      },
      "required": ["orderId", "borrowSize", "clientOid", "loanApplyId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | The user self-defined order ID.                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order Test

<a id="opId002"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": [
        "buy",
        "sell"
      ],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": [
        "buy"
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
      "enum": [
        "DC",
        "CO",
        "CN",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": [
        "GTC",
        "GTT",
        "IOC",
        "FOK"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "[Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/hf/margin/order/test',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/hf/margin/order/test', headers = headers)

print(r.json())

```

`POST /api/v3/hf/margin/order/test`

Order test endpoint: This endpoint’s request and return parameters are identical
to the order endpoint, and can be used to verify whether the signature is
correct, among other operations. After placing an order, the order will not
enter the matching system, and the order cannot be queried.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": ["buy", "sell"],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": ["buy"]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": ["limit", "market"],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
      "enum": ["DC", "CO", "CN", "CB"],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": ["GTC", "GTT", "IOC", "FOK"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "[Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-test-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » isIsolated  | body | boolean        | false    | True - isolated margin; false - cross margin. Default is false                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |

#### Detailed descriptions

**» clientOid**: Client Order ID: The ClientOid field is a unique ID created by
the user (we recommend using a UUID), and can only contain numbers, letters,
underscores (\_), and hyphens (-). This field is returned when order information
is obtained. You can use clientOid to tag your orders. ClientOid is different
from the order ID created by the service provider. Please do not initiate
requests using the same clientOid. The maximum length for the ClientOid is 40
characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: Specify if the order is a 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price; you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order.

When **type** is limited, size refers to the amount of trading targets (the
asset name written in front) for the trading pair. The Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value  |
| ------------- | ------ |
| » side        | buy    |
| » side        | sell   |
| » type        | limit  |
| » type        | market |
| » stp         | DC     |
| » stp         | CO     |
| » stp         | CN     |
| » stp         | CB     |
| » timeInForce | GTC    |
| » timeInForce | GTT    |
| » timeInForce | IOC    |
| » timeInForce | FOK    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderId": {
          "type": "string",
          "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
        },
        "loanApplyId": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "number",
          "description": "ID of the borrowing response. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        }
      },
      "required": ["orderId", "borrowSize", "clientOid", "loanApplyId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-test-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-test-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | number | true     | none         | ID of the borrowing response. The field is returned only after placing the order under the mode of Auto-Borrow.               |
| »» clientOid   | string | true     | none         | The user self-defined order ID.                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By OrderId

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/hf/margin/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
  method: "DELETE",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/api/v3/hf/margin/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders/{orderId}`

This endpoint can be used to cancel a margin order by orderId. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-orderid-parameters">Parameters</h3>

| Name    | In    | Type   | Required | Description                                         |
| ------- | ----- | ------ | -------- | --------------------------------------------------- |
| symbol  | query | string | true     | symbol                                              |
| orderId | path  | string | true     | The unique order id generated by the trading system |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderId": {
          "type": "string",
          "description": "Order id"
        }
      },
      "required": ["orderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description |
| ---------- | ------ | -------- | ------------ | ----------- |
| » code     | string | true     | none         | none        |
| » data     | object | true     | none         | none        |
| »» orderId | string | true     | none         | Order id    |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order By OrderId

<a id="opId010"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/hf/margin/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/{orderId}`

This endpoint can be used to obtain information for a single Margin order using
the order ID. After the user successfully places an order, the order is in the
Active state, and the user can use inOrderBook to determine whether the order
has entered the order. Canceled or fully filled orders are marked as completed
Done status.

<h3 id="get-order-by-orderid-parameters">Parameters</h3>

| Name    | In    | Type   | Required | Description                                         |
| ------- | ----- | ------ | -------- | --------------------------------------------------- |
| symbol  | query | string | true     | symbol                                              |
| orderId | path  | string | true     | The unique order id generated by the trading system |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "The unique order id generated by the trading system"
        },
        "symbol": {
          "type": "string",
          "description": "symbol",
          "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
        },
        "opType": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "description": "Specify if the order is a 'limit' order or 'market' order. ",
          "enum": ["limit", "market"],
          "x-api-enum": [
            {
              "value": "limit",
              "name": "",
              "description": ""
            },
            {
              "value": "market",
              "name": "",
              "description": ""
            }
          ]
        },
        "side": {
          "type": "string",
          "description": "Buy or sell",
          "enum": ["buy", "sell"],
          "x-api-enum": [
            {
              "value": "buy",
              "name": "",
              "description": ""
            },
            {
              "value": "sell",
              "name": "",
              "description": ""
            }
          ]
        },
        "price": {
          "type": "string",
          "description": "Order Price"
        },
        "size": {
          "type": "string",
          "description": "Order Size"
        },
        "funds": {
          "type": "string",
          "description": "Order Funds"
        },
        "dealSize": {
          "type": "string",
          "description": "Number of filled transactions"
        },
        "dealFunds": {
          "type": "string",
          "description": "Funds of filled transactions"
        },
        "fee": {
          "type": "string",
          "description": "[Handling fees](https://www.kucoin.com/docs-new/api-5327739)"
        },
        "feeCurrency": {
          "type": "string",
          "description": "Currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
          "enum": ["DC", "CO", "CN", "CB"],
          "x-api-enum": [
            {
              "value": "DC",
              "name": "",
              "description": ""
            },
            {
              "value": "CO",
              "name": "",
              "description": ""
            },
            {
              "value": "CN",
              "name": "",
              "description": ""
            },
            {
              "value": "CB",
              "name": "",
              "description": ""
            }
          ]
        },
        "stop": {
          "type": "string"
        },
        "stopTriggered": {
          "type": "boolean"
        },
        "stopPrice": {
          "type": "string"
        },
        "timeInForce": {
          "type": "string",
          "description": "Time in force",
          "enum": ["GTC", "GTT", "IOC", "FOK"],
          "x-api-enum": [
            {
              "value": "GTC",
              "name": "",
              "description": ""
            },
            {
              "value": "GTT",
              "name": "",
              "description": ""
            },
            {
              "value": "IOC",
              "name": "",
              "description": ""
            },
            {
              "value": "FOK",
              "name": "",
              "description": ""
            }
          ]
        },
        "postOnly": {
          "type": "boolean",
          "description": "Whether it’s a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether it’s a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether it’s a iceberg order."
        },
        "visibleSize": {
          "type": "string",
          "description": "Visible size of iceberg order in order book."
        },
        "cancelAfter": {
          "type": "integer",
          "description": "A GTT timeInForce that expires in n seconds"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string",
          "description": "Client Order Id, unique identifier created by the user"
        },
        "remark": {
          "type": "string",
          "description": "Order placement remarks"
        },
        "tags": {
          "type": "string",
          "description": "Order tag"
        },
        "cancelExist": {
          "type": "boolean",
          "description": "Whether there is a cancellation record for the order."
        },
        "createdAt": {
          "type": "integer",
          "format": "int64"
        },
        "lastUpdatedAt": {
          "type": "integer",
          "format": "int64"
        },
        "tradeType": {
          "type": "string",
          "description": "Trade type, redundancy param"
        },
        "inOrderBook": {
          "type": "boolean",
          "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
        },
        "cancelledSize": {
          "type": "string",
          "description": "Number of canceled transactions"
        },
        "cancelledFunds": {
          "type": "string",
          "description": "Funds of canceled transactions"
        },
        "remainSize": {
          "type": "string",
          "description": "Number of remain transactions"
        },
        "remainFunds": {
          "type": "string",
          "description": "Funds of remain transactions"
        },
        "tax": {
          "type": "string",
          "description": "Users in some regions have this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order is active; false-The status of the order is done"
        }
      },
      "required": [
        "id",
        "symbol",
        "opType",
        "type",
        "side",
        "price",
        "size",
        "funds",
        "dealSize",
        "dealFunds",
        "fee",
        "feeCurrency",
        "stopTriggered",
        "stopPrice",
        "timeInForce",
        "postOnly",
        "hidden",
        "iceberg",
        "visibleSize",
        "cancelAfter",
        "channel",
        "clientOid",
        "remark",
        "cancelExist",
        "createdAt",
        "lastUpdatedAt",
        "tradeType",
        "inOrderBook",
        "cancelledSize",
        "cancelledFunds",
        "remainSize",
        "remainFunds",
        "tax",
        "active"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | object         | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                              |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | true     | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| type        | limit  |
| type        | market |
| side        | buy    |
| side        | sell   |
| stp         | DC     |
| stp         | CO     |
| stp         | CN     |
| stp         | CB     |
| timeInForce | GTC    |
| timeInForce | GTT    |
| timeInForce | IOC    |
| timeInForce | FOK    |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By ClientOid

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
  {
    method: "DELETE",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/api/v3/hf/margin/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders/client-order/{clientOid}`

This endpoint can be used to cancel a margin order by clientOid. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id, unique identifier created by the user |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "clientOid": {
          "type": "string",
          "description": "Client Order Id, unique identifier created by the user"
        }
      },
      "required": ["clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type   | Required | Restrictions | Description                                            |
| ------------ | ------ | -------- | ------------ | ------------------------------------------------------ |
| » code       | string | true     | none         | none                                                   |
| » data       | object | true     | none         | none                                                   |
| »» clientOid | string | true     | none         | Client Order Id, unique identifier created by the user |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order By ClientOid

<a id="opId011"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/client-order/{clientOid}`

This endpoint can be used to obtain information for a single Margin order using
the client order ID. After the user successfully places an order, the order is
in the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id, unique identifier created by the user |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "The unique order id generated by the trading system"
        },
        "symbol": {
          "type": "string",
          "description": "symbol",
          "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
        },
        "opType": {
          "type": "string"
        },
        "type": {
          "type": "string",
          "description": "Specify if the order is a 'limit' order or 'market' order. ",
          "enum": ["limit", "market"],
          "x-api-enum": [
            {
              "value": "limit",
              "name": "",
              "description": ""
            },
            {
              "value": "market",
              "name": "",
              "description": ""
            }
          ]
        },
        "side": {
          "type": "string",
          "description": "Buy or sell",
          "enum": ["buy", "sell"],
          "x-api-enum": [
            {
              "value": "buy",
              "name": "",
              "description": ""
            },
            {
              "value": "sell",
              "name": "",
              "description": ""
            }
          ]
        },
        "price": {
          "type": "string",
          "description": "Order Price"
        },
        "size": {
          "type": "string",
          "description": "Order Size"
        },
        "funds": {
          "type": "string",
          "description": "Order Funds"
        },
        "dealSize": {
          "type": "string",
          "description": "Number of filled transactions"
        },
        "dealFunds": {
          "type": "string",
          "description": "Funds of filled transactions"
        },
        "fee": {
          "type": "string",
          "description": "[Handling fees](https://www.kucoin.com/docs-new/api-5327739)"
        },
        "feeCurrency": {
          "type": "string",
          "description": "Currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
          "enum": ["DC", "CO", "CN", "CB"],
          "x-api-enum": [
            {
              "value": "DC",
              "name": "",
              "description": ""
            },
            {
              "value": "CO",
              "name": "",
              "description": ""
            },
            {
              "value": "CN",
              "name": "",
              "description": ""
            },
            {
              "value": "CB",
              "name": "",
              "description": ""
            }
          ]
        },
        "stop": {
          "type": "string"
        },
        "stopTriggered": {
          "type": "boolean"
        },
        "stopPrice": {
          "type": "string"
        },
        "timeInForce": {
          "type": "string",
          "description": "Time in force",
          "enum": ["GTC", "GTT", "IOC", "FOK"],
          "x-api-enum": [
            {
              "value": "GTC",
              "name": "",
              "description": ""
            },
            {
              "value": "GTT",
              "name": "",
              "description": ""
            },
            {
              "value": "IOC",
              "name": "",
              "description": ""
            },
            {
              "value": "FOK",
              "name": "",
              "description": ""
            }
          ]
        },
        "postOnly": {
          "type": "boolean",
          "description": "Whether it’s a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether it’s a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether it’s a iceberg order."
        },
        "visibleSize": {
          "type": "string",
          "description": "Visible size of iceberg order in order book."
        },
        "cancelAfter": {
          "type": "integer",
          "description": "A GTT timeInForce that expires in n seconds"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string",
          "description": "Client Order Id, unique identifier created by the user"
        },
        "remark": {
          "type": "string",
          "description": "Order placement remarks"
        },
        "tags": {
          "type": "string",
          "description": "Order tag"
        },
        "cancelExist": {
          "type": "boolean",
          "description": "Whether there is a cancellation record for the order."
        },
        "createdAt": {
          "type": "integer",
          "format": "int64"
        },
        "lastUpdatedAt": {
          "type": "integer",
          "format": "int64"
        },
        "tradeType": {
          "type": "string",
          "description": "Trade type, redundancy param"
        },
        "inOrderBook": {
          "type": "boolean",
          "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
        },
        "cancelledSize": {
          "type": "string",
          "description": "Number of canceled transactions"
        },
        "cancelledFunds": {
          "type": "string",
          "description": "Funds of canceled transactions"
        },
        "remainSize": {
          "type": "string",
          "description": "Number of remain transactions"
        },
        "remainFunds": {
          "type": "string",
          "description": "Funds of remain transactions"
        },
        "tax": {
          "type": "string",
          "description": "Users in some regions have this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order is active; false-The status of the order is done"
        }
      },
      "required": [
        "id",
        "symbol",
        "opType",
        "type",
        "side",
        "price",
        "size",
        "funds",
        "dealSize",
        "dealFunds",
        "fee",
        "feeCurrency",
        "stopTriggered",
        "stopPrice",
        "timeInForce",
        "postOnly",
        "hidden",
        "iceberg",
        "visibleSize",
        "cancelAfter",
        "channel",
        "clientOid",
        "cancelExist",
        "createdAt",
        "lastUpdatedAt",
        "tradeType",
        "inOrderBook",
        "cancelledSize",
        "cancelledFunds",
        "remainSize",
        "remainFunds",
        "tax",
        "active"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | object         | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                              |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | false    | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| type        | limit  |
| type        | market |
| side        | buy    |
| side        | sell   |
| stp         | DC     |
| stp         | CO     |
| stp         | CN     |
| stp         | CB     |
| timeInForce | GTC    |
| timeInForce | GTT    |
| timeInForce | IOC    |
| timeInForce | FOK    |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel All Orders By Symbol

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "DELETE",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.delete('/api/v3/hf/margin/orders', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders`

This interface can cancel all open Margin orders by symbol. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to Websocket.

<h3 id="cancel-all-orders-by-symbol-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                        |
| --------- | ----- | ------ | -------- | -------------------------------------------------------------------------------------------------- |
| symbol    | query | string | true     | symbol                                                                                             |
| tradeType | query | string | true     | Transaction type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "string"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-all-orders-by-symbol-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-all-orders-by-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description |
| ------ | ------ | -------- | ------------ | ----------- |
| » code | string | true     | none         | none        |
| » data | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Symbols With Open Order

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/order/active/symbols?tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/order/active/symbols', params={
  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/order/active/symbols`

This interface can query all Margin symbols that have active orders.

<h3 id="get-symbols-with-open-order-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                        |
| --------- | ----- | ------ | -------- | ------------------------------------------------------------------ |
| tradeType | query | string | true     | Cross Margin: MARGIN_TRADE, Isolated Margin: MARGIN_ISOLATED_TRADE |

#### Detailed descriptions

**tradeType**: Cross Margin: MARGIN_TRADE, Isolated Margin:
MARGIN_ISOLATED_TRADE

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "symbolSize": {
          "type": "integer",
          "description": "Symbol Size"
        },
        "symbols": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The symbol that has active orders"
        }
      },
      "required": ["symbolSize", "symbols"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-symbols-with-open-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-symbols-with-open-order-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type     | Required | Restrictions | Description                       |
| ------------- | -------- | -------- | ------------ | --------------------------------- |
| » code        | string   | true     | none         | none                              |
| » data        | object   | true     | none         | none                              |
| »» symbolSize | integer  | true     | none         | Symbol Size                       |
| »» symbols    | [string] | true     | none         | The symbol that has active orders |

<aside class="success">
This operation does not require authentication
</aside>

## Get Open Orders

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/active?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/orders/active', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/active`

This interface is to obtain all Margin active order lists, and the return value
of the active order interface is the paged data of all uncompleted order lists.
The returned data is sorted in descending order according to the latest update
time of the order. After the user successfully places an order, the order is in
the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-open-orders-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                                  |
| --------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                                                                       |
| tradeType | query | string | true     | Order type: MARGIN_TRADE - cross margin trading order, MARGIN_ISOLATED_TRADE - isolated margin trading order |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "The unique order id generated by the trading system"
          },
          "symbol": {
            "type": "string",
            "description": "symbol",
            "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
          },
          "opType": {
            "type": "string"
          },
          "type": {
            "type": "string",
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
            "enum": ["limit", "market"],
            "x-api-enum": [
              {
                "value": "limit",
                "name": "",
                "description": ""
              },
              {
                "value": "market",
                "name": "",
                "description": ""
              }
            ]
          },
          "side": {
            "type": "string",
            "description": "Buy or sell",
            "enum": ["buy", "sell"],
            "x-api-enum": [
              {
                "value": "buy",
                "name": "",
                "description": ""
              },
              {
                "value": "sell",
                "name": "",
                "description": ""
              }
            ]
          },
          "price": {
            "type": "string",
            "description": "Order Price"
          },
          "size": {
            "type": "string",
            "description": "Order Size"
          },
          "funds": {
            "type": "string",
            "description": "Order Funds"
          },
          "dealSize": {
            "type": "string",
            "description": "Number of filled transactions"
          },
          "dealFunds": {
            "type": "string",
            "description": "Funds of filled transactions"
          },
          "fee": {
            "type": "string",
            "description": "Trading fee"
          },
          "feeCurrency": {
            "type": "string",
            "description": "Currency used to calculate trading fee"
          },
          "stp": {
            "type": "string",
            "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
            "enum": ["DC", "CO", "CN", "CB"],
            "x-api-enum": [
              {
                "value": "DC",
                "name": "",
                "description": ""
              },
              {
                "value": "CO",
                "name": "",
                "description": ""
              },
              {
                "value": "CN",
                "name": "",
                "description": ""
              },
              {
                "value": "CB",
                "name": "",
                "description": ""
              }
            ]
          },
          "stop": {
            "type": "string"
          },
          "stopTriggered": {
            "type": "boolean"
          },
          "stopPrice": {
            "type": "string"
          },
          "timeInForce": {
            "type": "string",
            "description": "Time in force",
            "enum": ["GTC", "GTT", "IOC", "FOK"],
            "x-api-enum": [
              {
                "value": "GTC",
                "name": "",
                "description": ""
              },
              {
                "value": "GTT",
                "name": "",
                "description": ""
              },
              {
                "value": "IOC",
                "name": "",
                "description": ""
              },
              {
                "value": "FOK",
                "name": "",
                "description": ""
              }
            ]
          },
          "postOnly": {
            "type": "boolean",
            "description": "Whether it’s a postOnly order."
          },
          "hidden": {
            "type": "boolean",
            "description": "Whether it’s a hidden order."
          },
          "iceberg": {
            "type": "boolean",
            "description": "Whether it’s a iceberg order."
          },
          "visibleSize": {
            "type": "string",
            "description": "Visible size of iceberg order in order book."
          },
          "cancelAfter": {
            "type": "integer",
            "description": "A GTT timeInForce that expires in n seconds"
          },
          "channel": {
            "type": "string"
          },
          "clientOid": {
            "type": "string",
            "description": "Client Order Id, unique identifier created by the user"
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks"
          },
          "tags": {
            "type": "string",
            "description": "Order tag"
          },
          "cancelExist": {
            "type": "boolean",
            "description": "Whether there is a cancellation record for the order."
          },
          "createdAt": {
            "type": "integer",
            "format": "int64"
          },
          "lastUpdatedAt": {
            "type": "integer",
            "format": "int64"
          },
          "tradeType": {
            "type": "string",
            "description": "Trade type, redundancy param"
          },
          "inOrderBook": {
            "type": "boolean",
            "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
          },
          "cancelledSize": {
            "type": "string",
            "description": "Number of canceled transactions"
          },
          "cancelledFunds": {
            "type": "string",
            "description": "Funds of canceled transactions"
          },
          "remainSize": {
            "type": "string",
            "description": "Number of remain transactions"
          },
          "remainFunds": {
            "type": "string",
            "description": "Funds of remain transactions"
          },
          "tax": {
            "type": "string",
            "description": "Users in some regions have this field"
          },
          "active": {
            "type": "boolean",
            "description": "Order status: true-The status of the order is active; false-The status of the order is done"
          }
        },
        "required": [
          "stopPrice",
          "id",
          "symbol",
          "opType",
          "type",
          "side",
          "price",
          "size",
          "funds",
          "dealSize",
          "dealFunds",
          "fee",
          "feeCurrency",
          "stopTriggered",
          "timeInForce",
          "postOnly",
          "hidden",
          "iceberg",
          "visibleSize",
          "cancelAfter",
          "channel",
          "clientOid",
          "cancelExist",
          "tradeType",
          "inOrderBook",
          "cancelledSize",
          "cancelledFunds",
          "remainSize",
          "remainFunds",
          "active",
          "createdAt",
          "lastUpdatedAt",
          "tax"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-open-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-open-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | [object]       | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | Trading fee                                                                                                               |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | false    | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| type        | limit  |
| type        | market |
| side        | buy    |
| side        | sell   |
| stp         | DC     |
| stp         | CO     |
| stp         | CN     |
| stp         | CB     |
| timeInForce | GTC    |
| timeInForce | GTT    |
| timeInForce | IOC    |
| timeInForce | FOK    |

<aside class="success">
This operation does not require authentication
</aside>

## Get Closed Orders

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/done?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/orders/done', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/done`

This interface is to obtain all Margin closed order lists, and the return value
of the active order interface is the paged data of all uncompleted order lists.
The returned data is sorted in descending order according to the latest update
time of the order. After the user successfully places an order, the order is in
the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-closed-orders-parameters">Parameters</h3>

| Name      | In    | Type           | Required | Description                                                                                               |
| --------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| symbol    | query | string         | true     | symbol                                                                                                    |
| tradeType | query | string         | true     | Transaction type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade        |
| side      | query | string         | false    | Specify if the order is to 'buy' or 'sell'.                                                               |
| type      | query | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                |
| lastId    | query | integer(int64) | false    | The ID of the last set of data from the previous data batch. By default, the latest information is given. |
| limit     | query | integer        | false    | Default20, Max100                                                                                         |
| startAt   | query | integer(int64) | false    | Start time (milliseconds)                                                                                 |
| endAt     | query | integer(int64) | false    | End time (milliseconds)                                                                                   |

#### Detailed descriptions

**lastId**: The ID of the last set of data from the previous data batch. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId, which can be used as a query
parameter to look up new data from the next page.

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "lastId": {
          "type": "integer",
          "description": "The ID of the last set of data from the previous data batch. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page.",
          "format": "int64"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string",
                "description": "The unique order id generated by the trading system"
              },
              "symbol": {
                "type": "string",
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "opType": {
                "type": "string"
              },
              "type": {
                "type": "string",
                "description": "Specify if the order is a 'limit' order or 'market' order. ",
                "enum": ["limit", "market"],
                "x-api-enum": [
                  {
                    "value": "limit",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "market",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "side": {
                "type": "string",
                "description": "Buy or sell"
              },
              "price": {
                "type": "string",
                "description": "Order Price"
              },
              "size": {
                "type": "string",
                "description": "Order Size"
              },
              "funds": {
                "type": "string",
                "description": "Order Funds"
              },
              "dealSize": {
                "type": "string",
                "description": "Number of filled transactions"
              },
              "dealFunds": {
                "type": "string",
                "description": "Funds of filled transactions"
              },
              "fee": {
                "type": "string",
                "description": "[Handling fees](https://www.kucoin.com/docs-new/api-5327739)"
              },
              "feeCurrency": {
                "type": "string",
                "description": "Currency used to calculate trading fee"
              },
              "stp": {
                "type": "string",
                "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
                "enum": ["DC", "CO", "CN", "CB"],
                "x-api-enum": [
                  {
                    "value": "DC",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "CO",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "CN",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "CB",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "stop": {
                "type": "string"
              },
              "stopTriggered": {
                "type": "boolean"
              },
              "stopPrice": {
                "type": "string"
              },
              "timeInForce": {
                "type": "string",
                "description": "Time in force",
                "enum": ["GTC", "GTT", "IOC", "FOK"],
                "x-api-enum": [
                  {
                    "value": "GTC",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "GTT",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "IOC",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "FOK",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "postOnly": {
                "type": "boolean",
                "description": "Whether it’s a postOnly order."
              },
              "hidden": {
                "type": "boolean",
                "description": "Whether it’s a hidden order."
              },
              "iceberg": {
                "type": "boolean",
                "description": "Whether it’s a iceberg order."
              },
              "visibleSize": {
                "type": "string",
                "description": "Visible size of iceberg order in order book."
              },
              "cancelAfter": {
                "type": "integer",
                "description": "A GTT timeInForce that expires in n seconds"
              },
              "channel": {
                "type": "string"
              },
              "clientOid": {
                "type": "string",
                "description": "Client Order Id, unique identifier created by the user"
              },
              "remark": {
                "type": "string",
                "description": "Order placement remarks"
              },
              "tags": {
                "type": "string",
                "description": "Order tag"
              },
              "cancelExist": {
                "type": "boolean",
                "description": "Whether there is a cancellation record for the order."
              },
              "createdAt": {
                "type": "integer",
                "format": "int64"
              },
              "lastUpdatedAt": {
                "type": "integer",
                "format": "int64"
              },
              "tradeType": {
                "type": "string",
                "description": "Trade type, redundancy param"
              },
              "inOrderBook": {
                "type": "boolean",
                "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
              },
              "cancelledSize": {
                "type": "string",
                "description": "Number of canceled transactions"
              },
              "cancelledFunds": {
                "type": "string",
                "description": "Funds of canceled transactions"
              },
              "remainSize": {
                "type": "string",
                "description": "Number of remain transactions"
              },
              "remainFunds": {
                "type": "string",
                "description": "Funds of remain transactions"
              },
              "tax": {
                "type": "string",
                "description": "Users in some regions have this field"
              },
              "active": {
                "type": "boolean",
                "description": "Order status: true-The status of the order is active; false-The status of the order is done"
              }
            },
            "required": [
              "id",
              "symbol",
              "opType",
              "type",
              "side",
              "price",
              "size",
              "funds",
              "dealSize",
              "dealFunds",
              "fee",
              "feeCurrency",
              "stopTriggered",
              "stopPrice",
              "timeInForce",
              "postOnly",
              "hidden",
              "iceberg",
              "visibleSize",
              "cancelAfter",
              "channel",
              "clientOid",
              "cancelExist",
              "createdAt",
              "lastUpdatedAt",
              "tradeType",
              "inOrderBook",
              "cancelledSize",
              "cancelledFunds",
              "remainSize",
              "remainFunds",
              "tax",
              "active"
            ]
          }
        }
      },
      "required": ["lastId", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-closed-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-closed-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» lastId          | integer(int64) | true     | none         | The ID of the last set of data from the previous data batch. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page. |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» id             | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                       |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                    |
| »»» opType         | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                               |
| »»» price          | string         | true     | none         | Order Price                                                                                                                                                                                                                                                                                                                                               |
| »»» size           | string         | true     | none         | Order Size                                                                                                                                                                                                                                                                                                                                                |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                               |
| »»» dealSize       | string         | true     | none         | Number of filled transactions                                                                                                                                                                                                                                                                                                                             |
| »»» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                                                                                                                                                                                                                                                              |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                              |
| »»» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                    |
| »»» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                 |
| »»» stop           | string         | false    | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» stopTriggered  | boolean        | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» stopPrice      | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» timeInForce    | string         | true     | none         | Time in force                                                                                                                                                                                                                                                                                                                                             |
| »»» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                                                                                                                                                                                                                                                            |
| »»» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                                                                                                                                                                                                                                                              |
| »»» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                                                                                                                                                                                                                                                             |
| »»» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                                                                                                                                                                                                                                                              |
| »»» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                                                                                                                                                                                                                                                               |
| »»» channel        | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                                                                                                                                                                                                                                                    |
| »»» remark         | string         | false    | none         | Order placement remarks                                                                                                                                                                                                                                                                                                                                   |
| »»» tags           | string         | false    | none         | Order tag                                                                                                                                                                                                                                                                                                                                                 |
| »»» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                                                                                                                                                                                                                                                     |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                              |
| »»» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                                                                                                                                                                                                                                                              |
| »»» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                                                                                                                                                                                                                                                           |
| »»» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                                                                                                                                                                                                                                                            |
| »»» remainSize     | string         | true     | none         | Number of remain transactions                                                                                                                                                                                                                                                                                                                             |
| »»» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                                                                                                                                                                                                                                                              |
| »»» tax            | string         | true     | none         | Users in some regions have this field                                                                                                                                                                                                                                                                                                                     |
| »»» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                                                                                                                                                                                                                                                               |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| type        | limit  |
| type        | market |
| stp         | DC     |
| stp         | CO     |
| stp         | CN     |
| stp         | CB     |
| timeInForce | GTC    |
| timeInForce | GTT    |
| timeInForce | IOC    |
| timeInForce | FOK    |

<aside class="success">
This operation does not require authentication
</aside>

## Get Trade History

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/fills?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/hf/margin/fills', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/fills`

This endpoint can be used to obtain a list of the latest Margin transaction
details. The returned data is sorted in descending order according to the latest
update time of the order.

<h3 id="get-trade-history-parameters">Parameters</h3>

| Name      | In    | Type           | Required | Description                                                                                               |
| --------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| symbol    | query | string         | true     | symbol                                                                                                    |
| tradeType | query | string         | true     | Trade type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade              |
| orderId   | query | string         | false    | The unique order id generated by the trading system                                                       |
| side      | query | string         | false    | Specify if the order is to 'buy' or 'sell'.                                                               |
| type      | query | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                |
| lastId    | query | integer(int64) | false    | The ID of the last set of data from the previous data batch. By default, the latest information is given. |
| limit     | query | integer        | false    | Default20, Max100                                                                                         |
| startAt   | query | integer(int64) | false    | Start time (milliseconds)                                                                                 |
| endAt     | query | integer(int64) | false    | End time (milliseconds)                                                                                   |

#### Detailed descriptions

**orderId**: The unique order id generated by the trading system (If orderId is
specified, please ignore the other query parameters)

**lastId**: The ID of the last set of data from the previous data batch. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId, which can be used as a query
parameter to look up new data from the next page.

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "integer",
                "description": "ID of transaction detail",
                "format": "int64"
              },
              "symbol": {
                "type": "string",
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "tradeId": {
                "type": "integer",
                "description": "Trade ID, symbol latitude increment",
                "format": "int64"
              },
              "orderId": {
                "type": "string",
                "description": "The unique order id generated by the trading system"
              },
              "counterOrderId": {
                "type": "string",
                "description": "Counterparty order ID"
              },
              "side": {
                "type": "string",
                "description": "Buy or sell",
                "enum": ["buy", "sell"],
                "x-api-enum": [
                  {
                    "value": "buy",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "sell",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "liquidity": {
                "type": "string",
                "description": "Liquidity type: taker or maker",
                "enum": ["taker", "maker"],
                "x-api-enum": [
                  {
                    "value": "taker",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "maker",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "forceTaker": {
                "type": "boolean"
              },
              "price": {
                "type": "string",
                "description": "Order Price"
              },
              "size": {
                "type": "string",
                "description": "Order Size"
              },
              "funds": {
                "type": "string",
                "description": "Order Funds"
              },
              "fee": {
                "type": "string",
                "description": "[Handling fees](https://www.kucoin.com/docs-new/api-5327739)"
              },
              "feeRate": {
                "type": "string",
                "description": "Fee rate\n"
              },
              "feeCurrency": {
                "type": "string",
                "description": "Currency used to calculate trading fee"
              },
              "stop": {
                "type": "string",
                "description": "Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty"
              },
              "tradeType": {
                "type": "string",
                "description": "Trade type, redundancy param"
              },
              "tax": {
                "type": "string",
                "description": "Users in some regions have this field"
              },
              "taxRate": {
                "type": "string",
                "description": "Tax Rate: Users in some regions must query this field"
              },
              "type": {
                "type": "string",
                "description": "Specify if the order is a 'limit' order or 'market' order. ",
                "enum": ["limit", "market"],
                "x-api-enum": [
                  {
                    "value": "limit",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "market",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "createdAt": {
                "type": "integer",
                "format": "int64"
              }
            },
            "required": [
              "id",
              "symbol",
              "tradeId",
              "orderId",
              "counterOrderId",
              "side",
              "liquidity",
              "forceTaker",
              "price",
              "size",
              "funds",
              "fee",
              "feeRate",
              "feeCurrency",
              "stop",
              "tradeType",
              "tax",
              "taxRate",
              "type",
              "createdAt"
            ]
          }
        },
        "lastId": {
          "type": "integer",
          "description": "The ID of the last set of data from the previous data batch. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page.",
          "format": "int64"
        }
      },
      "required": ["items", "lastId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-trade-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-trade-history-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» id             | integer(int64) | true     | none         | ID of transaction detail                                                                                                                                                                                                                                                                                                                                  |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                    |
| »»» tradeId        | integer(int64) | true     | none         | Trade ID, symbol latitude increment                                                                                                                                                                                                                                                                                                                       |
| »»» orderId        | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                       |
| »»» counterOrderId | string         | true     | none         | Counterparty order ID                                                                                                                                                                                                                                                                                                                                     |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                               |
| »»» liquidity      | string         | true     | none         | Liquidity type: taker or maker                                                                                                                                                                                                                                                                                                                            |
| »»» forceTaker     | boolean        | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» price          | string         | true     | none         | Order Price                                                                                                                                                                                                                                                                                                                                               |
| »»» size           | string         | true     | none         | Order Size                                                                                                                                                                                                                                                                                                                                                |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                               |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                              |
| »»» feeRate        | string         | true     | none         | Fee rate                                                                                                                                                                                                                                                                                                                                                  |
| »»» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                    |
| »»» stop           | string         | true     | none         | Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty                                                                                                                                                                                                                                         |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                              |
| »»» tax            | string         | true     | none         | Users in some regions have this field                                                                                                                                                                                                                                                                                                                     |
| »»» taxRate        | string         | true     | none         | Tax Rate: Users in some regions must query this field                                                                                                                                                                                                                                                                                                     |
| »»» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» lastId          | integer(int64) | true     | none         | The ID of the last set of data from the previous data batch. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page. |

#### Enumerated Values

| Property  | Value  |
| --------- | ------ |
| side      | buy    |
| side      | sell   |
| liquidity | taker  |
| liquidity | maker  |
| type      | limit  |
| type      | market |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order - V1

<a id="opId012"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": [
        "buy",
        "sell"
      ],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": [
        "buy"
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC",
      "enum": [
        "DC",
        "CO",
        "CN",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading",
      "enum": [
        "GTC",
        "GTT",
        "IOC",
        "FOK"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "Hidden or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in iceberg orders",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": [
        "cross",
        "isolated"
      ],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/margin/order',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v1/margin/order', headers = headers)

print(r.json())

```

`POST /api/v1/margin/order`

Place order in the Cross-margin or Isolated-margin trading system. You can place
two major types of order: Limit and market. Orders can only be placed if your
account has sufficient funds. Once an order is placed, your funds will be put on
hold for the duration of the order. The amount of funds on hold depends on the
order type and parameters specified.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": ["buy", "sell"],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": ["buy"]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": ["limit", "market"],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC",
      "enum": ["DC", "CO", "CN", "CB"],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading",
      "enum": ["GTC", "GTT", "IOC", "FOK"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "Hidden or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in iceberg orders",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": ["cross", "isolated"],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order---v1-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                       |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                          |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                     |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |
| » marginModel | body | string         | false    | The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.                                                                                                                                                                                                                                                                                                                                                                 |

#### Detailed descriptions

**» clientOid**: Client Order ID: The ClientOid field is a unique ID created by
the user (we recommend using a UUID), and can only contain numbers, letters,
underscores (\_), and hyphens (-). This field is returned when order information
is obtained. You can use clientOid to tag your orders. ClientOid is different
from the order ID created by the service provider. Please do not initiate
requests using the same clientOid. The maximum length for the ClientOid is 40
characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: Specify if the order is a 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price; you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order.

When **type** is limited, size refers to the amount of trading targets (the
asset name written in front) for the trading pair. The Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value    |
| ------------- | -------- |
| » side        | buy      |
| » side        | sell     |
| » type        | limit    |
| » type        | market   |
| » stp         | DC       |
| » stp         | CO       |
| » stp         | CN       |
| » stp         | CB       |
| » timeInForce | GTC      |
| » timeInForce | GTT      |
| » timeInForce | IOC      |
| » timeInForce | FOK      |
| » marginModel | cross    |
| » marginModel | isolated |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderId": {
          "type": "string",
          "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
        },
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "This return value is invalid"
        }
      },
      "required": ["orderId", "loanApplyId", "borrowSize", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order---v1-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order---v1-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | This return value is invalid                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order Test - V1

<a id="opId013"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": [
        "buy",
        "sell"
      ],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": [
        "buy"
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC",
      "enum": [
        "DC",
        "CO",
        "CN",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": [
        "GTC",
        "GTT",
        "IOC",
        "FOK"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "Hidden or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in iceberg orders",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": [
        "cross",
        "isolated"
      ],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/margin/order/test',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v1/margin/order/test', headers = headers)

print(r.json())

```

`POST /api/v1/margin/order/test`

Order test endpoint: This endpoint’s request and return parameters are identical
to the order endpoint, and can be used to verify whether the signature is
correct, among other operations. After placing an order, the order will not
enter the matching system, and the order cannot be queried.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "Specify if the order is to 'buy' or 'sell'.",
      "enum": ["buy", "sell"],
      "x-api-enum": [
        {
          "value": "buy",
          "name": "",
          "description": ""
        },
        {
          "value": "sell",
          "name": "",
          "description": ""
        }
      ],
      "example": ["buy"]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price; you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": ["limit", "market"],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "",
          "description": ""
        },
        {
          "value": "market",
          "name": "",
          "description": ""
        }
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC",
      "enum": ["DC", "CO", "CN", "CB"],
      "x-api-enum": [
        {
          "value": "DC",
          "name": "",
          "description": ""
        },
        {
          "value": "CO",
          "name": "",
          "description": ""
        },
        {
          "value": "CN",
          "name": "",
          "description": ""
        },
        {
          "value": "CB",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Specify price for order\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order.\n\nWhen **type** is limited, size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
      "enum": ["GTC", "GTT", "IOC", "FOK"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "",
          "description": ""
        },
        {
          "value": "GTT",
          "name": "",
          "description": ""
        },
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "default": false,
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK"
    },
    "hidden": {
      "type": "boolean",
      "description": "Hidden or not (not shown in order book)",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Whether or not only visible portions of orders are shown in iceberg orders",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Maximum visible quantity in iceberg orders"
    },
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": ["cross", "isolated"],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-test---v1-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                     |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |
| » marginModel | body | string         | false    | The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.                                                                                                                                                                                                                                                                                                                                                                 |

#### Detailed descriptions

**» clientOid**: Client Order ID: The ClientOid field is a unique ID created by
the user (we recommend using a UUID), and can only contain numbers, letters,
underscores (\_), and hyphens (-). This field is returned when order information
is obtained. You can use clientOid to tag your orders. ClientOid is different
from the order ID created by the service provider. Please do not initiate
requests using the same clientOid. The maximum length for the ClientOid is 40
characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: Specify if the order is a 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price; you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order.

When **type** is limited, size refers to the amount of trading targets (the
asset name written in front) for the trading pair. The Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value    |
| ------------- | -------- |
| » side        | buy      |
| » side        | sell     |
| » type        | limit    |
| » type        | market   |
| » stp         | DC       |
| » stp         | CO       |
| » stp         | CN       |
| » stp         | CB       |
| » timeInForce | GTC      |
| » timeInForce | GTT      |
| » timeInForce | IOC      |
| » timeInForce | FOK      |
| » marginModel | cross    |
| » marginModel | isolated |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderId": {
          "type": "string",
          "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
        },
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "This return value is invalid"
        }
      },
      "required": ["orderId", "loanApplyId", "borrowSize", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-test---v1-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-test---v1-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | This return value is invalid                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

## Get Loan Market

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/project/list", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/project/list', headers = headers)

print(r.json())

```

`GET /api/v3/project/list`

This API endpoint is used to get the information about the currencies available
for lending.

<h3 id="get-loan-market-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description |
| -------- | ----- | ------ | -------- | ----------- |
| currency | query | string | false    | currency    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "currency": {
            "type": "string",
            "description": "Currency"
          },
          "purchaseEnable": {
            "type": "boolean",
            "description": "Whether purchase is supported."
          },
          "redeemEnable": {
            "type": "boolean",
            "description": "Whether redeem is supported."
          },
          "increment": {
            "type": "string",
            "description": "Increment precision for purchase and redemption"
          },
          "minPurchaseSize": {
            "type": "string",
            "description": "Minimum purchase amount"
          },
          "minInterestRate": {
            "type": "string",
            "description": "Minimum lending rate"
          },
          "maxInterestRate": {
            "type": "string",
            "description": "Maximum lending rate"
          },
          "interestIncrement": {
            "type": "string",
            "description": "Increment precision for interest; default is 0.0001",
            "default": "0.0001"
          },
          "maxPurchaseSize": {
            "type": "string",
            "description": "Maximum purchase amount"
          },
          "marketInterestRate": {
            "type": "string",
            "description": "Latest market lending rate"
          },
          "autoPurchaseEnable": {
            "type": "boolean",
            "description": "Whether to allow automatic purchase: True: on; false: off"
          }
        }
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-loan-market-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-loan-market-responseschema">Response Schema</h3>

Status Code **200**

| Name                  | Type     | Required | Restrictions | Description                                               |
| --------------------- | -------- | -------- | ------------ | --------------------------------------------------------- |
| » code                | string   | true     | none         | none                                                      |
| » data                | [object] | true     | none         | none                                                      |
| »» currency           | string   | false    | none         | Currency                                                  |
| »» purchaseEnable     | boolean  | false    | none         | Whether purchase is supported.                            |
| »» redeemEnable       | boolean  | false    | none         | Whether redeem is supported.                              |
| »» increment          | string   | false    | none         | Increment precision for purchase and redemption           |
| »» minPurchaseSize    | string   | false    | none         | Minimum purchase amount                                   |
| »» minInterestRate    | string   | false    | none         | Minimum lending rate                                      |
| »» maxInterestRate    | string   | false    | none         | Maximum lending rate                                      |
| »» interestIncrement  | string   | false    | none         | Increment precision for interest; default is 0.0001       |
| »» maxPurchaseSize    | string   | false    | none         | Maximum purchase amount                                   |
| »» marketInterestRate | string   | false    | none         | Latest market lending rate                                |
| »» autoPurchaseEnable | boolean  | false    | none         | Whether to allow automatic purchase: True: on; false: off |

<aside class="success">
This operation does not require authentication
</aside>

## Get Loan Market Interest Rate

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/project/marketInterestRate?currency=BTC,ETH,KCS", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/project/marketInterestRate', params={
  'currency': [
  "BTC",
  "ETH",
  "KCS"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/project/marketInterestRate`

This API endpoint is used to get the interest rates of the margin lending market
over the past 7 days.

<h3 id="get-loan-market-interest-rate-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description |
| -------- | ----- | ------ | -------- | ----------- |
| currency | query | string | true     | currency    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "time": {
            "type": "string",
            "description": "Time: YYYYMMDDHH00"
          },
          "marketInterestRate": {
            "type": "string",
            "description": "Market lending rate"
          }
        },
        "required": ["time", "marketInterestRate"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-loan-market-interest-rate-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-loan-market-interest-rate-responseschema">Response Schema</h3>

Status Code **200**

| Name                  | Type     | Required | Restrictions | Description         |
| --------------------- | -------- | -------- | ------------ | ------------------- |
| » code                | string   | true     | none         | none                |
| » data                | [object] | true     | none         | none                |
| »» time               | string   | true     | none         | Time: YYYYMMDDHH00  |
| »» marketInterestRate | string   | true     | none         | Market lending rate |

<aside class="success">
This operation does not require authentication
</aside>

## Purchase

<a id="opId003"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": [
        "BTC",
        "ETH",
        "USDT"
      ]
    },
    "size": {
      "type": "string",
      "description": "Purchase amount"
    },
    "interestRate": {
      "type": "string",
      "description": "Purchase interest rate"
    }
  },
  "required": [
    "currency",
    "size",
    "interestRate"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/purchase',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/purchase', headers = headers)

print(r.json())

```

`POST /api/v3/purchase`

Invest credit in the market and earn interest

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": ["BTC", "ETH", "USDT"]
    },
    "size": {
      "type": "string",
      "description": "Purchase amount"
    },
    "interestRate": {
      "type": "string",
      "description": "Purchase interest rate"
    }
  },
  "required": ["currency", "size", "interestRate"]
}
```

<h3 id="purchase-parameters">Parameters</h3>

| Name           | In   | Type   | Required | Description            |
| -------------- | ---- | ------ | -------- | ---------------------- |
| body           | body | object | false    | none                   |
| » currency     | body | string | true     | Currency               |
| » size         | body | string | true     | Purchase amount        |
| » interestRate | body | string | true     | Purchase interest rate |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderNo": {
          "type": "string",
          "description": "Purchase order ID"
        }
      },
      "required": ["orderNo"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="purchase-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="purchase-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description       |
| ---------- | ------ | -------- | ------------ | ----------------- |
| » code     | string | true     | none         | none              |
| » data     | object | true     | none         | none              |
| »» orderNo | string | true     | none         | Purchase order ID |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Purchase

<a id="opId004"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": [
        "BTC",
        "ETH",
        "USDT"
      ]
    },
    "interestRate": {
      "type": "string",
      "description": "Modified purchase interest rate"
    },
    "purchaseOrderNo": {
      "type": "string",
      "description": "Purchase order ID"
    }
  },
  "required": [
    "currency",
    "purchaseOrderNo",
    "interestRate"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/lend/purchase/update',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/lend/purchase/update', headers = headers)

print(r.json())

```

`POST /api/v3/lend/purchase/update`

This API endpoint is used to update the interest rates of subscription orders,
which will take effect at the beginning of the next hour. Please ensure that the
funds are in the main (funding) account.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": ["BTC", "ETH", "USDT"]
    },
    "interestRate": {
      "type": "string",
      "description": "Modified purchase interest rate"
    },
    "purchaseOrderNo": {
      "type": "string",
      "description": "Purchase order ID"
    }
  },
  "required": ["currency", "purchaseOrderNo", "interestRate"]
}
```

<h3 id="modify-purchase-parameters">Parameters</h3>

| Name              | In   | Type   | Required | Description                     |
| ----------------- | ---- | ------ | -------- | ------------------------------- |
| body              | body | object | false    | none                            |
| » currency        | body | string | true     | Currency                        |
| » interestRate    | body | string | true     | Modified purchase interest rate |
| » purchaseOrderNo | body | string | true     | Purchase order ID               |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "string"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-purchase-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-purchase-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description |
| ------ | ------ | -------- | ------------ | ----------- |
| » code | string | true     | none         | none        |
| » data | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Purchase Orders

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/purchase/orders?status=type,string,enum,DONE%2CPENDING,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/purchase/orders', params={
  'status': {
  "type": "string",
  "enum": [
    "DONE",
    "PENDING"
  ],
  "x-api-enum": [
    {
      "value": "DONE",
      "name": "DONE",
      "description": "completed"
    },
    {
      "value": "PENDING",
      "name": "PENDING",
      "description": "settling"
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/purchase/orders`

This API endpoint provides a pagination query for the purchase orders.

<h3 id="get-purchase-orders-parameters">Parameters</h3>

| Name            | In    | Type    | Required | Description                               |
| --------------- | ----- | ------- | -------- | ----------------------------------------- |
| status          | query | string  | true     | DONE-completed; PENDING-settling          |
| currency        | query | string  | false    | Currency                                  |
| purchaseOrderNo | query | string  | false    | Purchase order ID                         |
| currentPage     | query | integer | false    | Current page; default is 1                |
| pageSize        | query | integer | false    | Page size; 1<=pageSize<=50; default is 50 |

#### Enumerated Values

| Parameter | Value   |
| --------- | ------- |
| status    | DONE    |
| status    | PENDING |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "currentPage": {
          "type": "integer",
          "description": "Current Page"
        },
        "pageSize": {
          "type": "integer",
          "description": "Page Size"
        },
        "totalNum": {
          "type": "integer",
          "description": "Total Number"
        },
        "totalPage": {
          "type": "integer",
          "description": "Total Pages"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "currency": {
                "type": "string",
                "description": "Currency",
                "example": ["BTC", "ETH", "USDT"]
              },
              "purchaseOrderNo": {
                "type": "string",
                "description": "Purchase order ID"
              },
              "purchaseSize": {
                "type": "string",
                "description": "Total purchase size"
              },
              "matchSize": {
                "type": "string",
                "description": "Executed size"
              },
              "interestRate": {
                "type": "string",
                "description": "Target annualized interest rate"
              },
              "incomeSize": {
                "type": "string",
                "description": "Redeemed amount"
              },
              "applyTime": {
                "type": "integer",
                "description": "Time of purchase",
                "format": "int64"
              },
              "status": {
                "type": "string",
                "description": "Status: DONE-completed; PENDING-settling",
                "enum": ["DONE", "PENDING"],
                "x-api-enum": [
                  {
                    "value": "DONE",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "PENDING",
                    "name": "",
                    "description": ""
                  }
                ]
              }
            },
            "required": [
              "currency",
              "purchaseOrderNo",
              "purchaseSize",
              "matchSize",
              "interestRate",
              "incomeSize",
              "applyTime",
              "status"
            ]
          }
        }
      },
      "required": ["currentPage", "pageSize", "totalNum", "totalPage", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-purchase-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-purchase-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type           | Required | Restrictions | Description                              |
| ------------------- | -------------- | -------- | ------------ | ---------------------------------------- |
| » code              | string         | true     | none         | none                                     |
| » data              | object         | true     | none         | none                                     |
| »» currentPage      | integer        | true     | none         | Current Page                             |
| »» pageSize         | integer        | true     | none         | Page Size                                |
| »» totalNum         | integer        | true     | none         | Total Number                             |
| »» totalPage        | integer        | true     | none         | Total Pages                              |
| »» items            | [object]       | true     | none         | none                                     |
| »»» currency        | string         | true     | none         | Currency                                 |
| »»» purchaseOrderNo | string         | true     | none         | Purchase order ID                        |
| »»» purchaseSize    | string         | true     | none         | Total purchase size                      |
| »»» matchSize       | string         | true     | none         | Executed size                            |
| »»» interestRate    | string         | true     | none         | Target annualized interest rate          |
| »»» incomeSize      | string         | true     | none         | Redeemed amount                          |
| »»» applyTime       | integer(int64) | true     | none         | Time of purchase                         |
| »»» status          | string         | true     | none         | Status: DONE-completed; PENDING-settling |

#### Enumerated Values

| Property | Value   |
| -------- | ------- |
| status   | DONE    |
| status   | PENDING |

<aside class="success">
This operation does not require authentication
</aside>

## Redeem

<a id="opId006"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": [
        "BTC",
        "ETH",
        "USDT"
      ]
    },
    "size": {
      "type": "string",
      "description": "Redemption amount"
    },
    "purchaseOrderNo": {
      "type": "string",
      "description": "Purchase order ID"
    }
  },
  "required": [
    "currency",
    "size",
    "purchaseOrderNo"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/redeem',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/redeem', headers = headers)

print(r.json())

```

`POST /api/v3/redeem`

Redeem your loan order.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "Currency",
      "example": ["BTC", "ETH", "USDT"]
    },
    "size": {
      "type": "string",
      "description": "Redemption amount"
    },
    "purchaseOrderNo": {
      "type": "string",
      "description": "Purchase order ID"
    }
  },
  "required": ["currency", "size", "purchaseOrderNo"]
}
```

<h3 id="redeem-parameters">Parameters</h3>

| Name              | In   | Type   | Required | Description       |
| ----------------- | ---- | ------ | -------- | ----------------- |
| body              | body | object | false    | none              |
| » currency        | body | string | true     | Currency          |
| » size            | body | string | true     | Redemption amount |
| » purchaseOrderNo | body | string | true     | Purchase order ID |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderNo": {
          "type": "string",
          "description": "Redeem order ID"
        }
      },
      "required": ["orderNo"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="redeem-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="redeem-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description     |
| ---------- | ------ | -------- | ------------ | --------------- |
| » code     | string | true     | none         | none            |
| » data     | object | true     | none         | none            |
| »» orderNo | string | true     | none         | Redeem order ID |

<aside class="success">
This operation does not require authentication
</aside>

## Get Redeem Orders

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/redeem/orders?status=type,string,enum,DONE%2CPENDING,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
  {
    method: "GET",

    headers: headers
  }
)
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/redeem/orders', params={
  'status': {
  "type": "string",
  "enum": [
    "DONE",
    "PENDING"
  ],
  "x-api-enum": [
    {
      "value": "DONE",
      "name": "DONE",
      "description": "completed"
    },
    {
      "value": "PENDING",
      "name": "PENDING",
      "description": "settling"
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/redeem/orders`

This API endpoint provides pagination query for the redeem orders.

<h3 id="get-redeem-orders-parameters">Parameters</h3>

| Name          | In    | Type    | Required | Description                               |
| ------------- | ----- | ------- | -------- | ----------------------------------------- |
| status        | query | string  | true     | DONE-completed; PENDING-settling          |
| currency      | query | string  | false    | currency                                  |
| redeemOrderNo | query | string  | false    | Redeem order ID                           |
| currentPage   | query | integer | false    | Current page; default is 1                |
| pageSize      | query | integer | false    | Page size; 1<=pageSize<=50; default is 50 |

#### Enumerated Values

| Parameter | Value   |
| --------- | ------- |
| status    | DONE    |
| status    | PENDING |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "currentPage": {
          "type": "integer",
          "description": "Current Page"
        },
        "pageSize": {
          "type": "integer",
          "description": "Page Size"
        },
        "totalNum": {
          "type": "integer",
          "description": "Total Number"
        },
        "totalPage": {
          "type": "integer",
          "description": "Total Pages"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "currency": {
                "type": "string",
                "description": "Currency",
                "example": ["BTC", "ETH", "USDT"]
              },
              "purchaseOrderNo": {
                "type": "string",
                "description": "Purchase order ID"
              },
              "redeemOrderNo": {
                "type": "string",
                "description": "Redeem order ID"
              },
              "redeemSize": {
                "type": "string",
                "description": "Redemption size"
              },
              "receiptSize": {
                "type": "string",
                "description": "Redeemed size"
              },
              "applyTime": {
                "type": "string",
                "description": "Time of redeem"
              },
              "status": {
                "type": "string",
                "description": "Status: DONE-completed; PENDING-settling"
              }
            },
            "required": [
              "currency",
              "purchaseOrderNo",
              "redeemOrderNo",
              "redeemSize",
              "receiptSize",
              "applyTime",
              "status"
            ]
          }
        }
      },
      "required": ["currentPage", "pageSize", "totalNum", "totalPage", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-redeem-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-redeem-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type     | Required | Restrictions | Description                              |
| ------------------- | -------- | -------- | ------------ | ---------------------------------------- |
| » code              | string   | true     | none         | none                                     |
| » data              | object   | true     | none         | none                                     |
| »» currentPage      | integer  | true     | none         | Current Page                             |
| »» pageSize         | integer  | true     | none         | Page Size                                |
| »» totalNum         | integer  | true     | none         | Total Number                             |
| »» totalPage        | integer  | true     | none         | Total Pages                              |
| »» items            | [object] | true     | none         | none                                     |
| »»» currency        | string   | true     | none         | Currency                                 |
| »»» purchaseOrderNo | string   | true     | none         | Purchase order ID                        |
| »»» redeemOrderNo   | string   | true     | none         | Redeem order ID                          |
| »»» redeemSize      | string   | true     | none         | Redemption size                          |
| »»» receiptSize     | string   | true     | none         | Redeemed size                            |
| »»» applyTime       | string   | true     | none         | Time of redeem                           |
| »»» status          | string   | true     | none         | Status: DONE-completed; PENDING-settling |

<aside class="success">
This operation does not require authentication
</aside>

## Borrow

<a id="opId001"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "currency",
      "example": [
        "BTC",
        "ETH",
        "USDT"
      ]
    },
    "size": {
      "type": "number",
      "description": "Borrow amount"
    },
    "timeInForce": {
      "type": "string",
      "enum": [
        "IOC",
        "FOK"
      ],
      "description": "timeInForce: IOC, FOK",
      "x-api-enum": [
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "isHf": {
      "type": "boolean",
      "description": "true: high frequency borrowing, false: low frequency borrowing; default false",
      "default": false
    }
  },
  "required": [
    "currency",
    "size",
    "timeInForce"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/margin/borrow',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/margin/borrow', headers = headers)

print(r.json())

```

`POST /api/v3/margin/borrow`

This API endpoint is used to initiate an application for cross or isolated
margin borrowing.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "currency",
      "example": ["BTC", "ETH", "USDT"]
    },
    "size": {
      "type": "number",
      "description": "Borrow amount"
    },
    "timeInForce": {
      "type": "string",
      "enum": ["IOC", "FOK"],
      "description": "timeInForce: IOC, FOK",
      "x-api-enum": [
        {
          "value": "IOC",
          "name": "",
          "description": ""
        },
        {
          "value": "FOK",
          "name": "",
          "description": ""
        }
      ]
    },
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "isHf": {
      "type": "boolean",
      "description": "true: high frequency borrowing, false: low frequency borrowing; default false",
      "default": false
    }
  },
  "required": ["currency", "size", "timeInForce"]
}
```

<h3 id="borrow-parameters">Parameters</h3>

| Name          | In   | Type    | Required | Description                                                                   |
| ------------- | ---- | ------- | -------- | ----------------------------------------------------------------------------- |
| body          | body | object  | false    | none                                                                          |
| » currency    | body | string  | true     | currency                                                                      |
| » size        | body | number  | true     | Borrow amount                                                                 |
| » timeInForce | body | string  | true     | timeInForce: IOC, FOK                                                         |
| » symbol      | body | string  | false    | symbol, mandatory for isolated margin account                                 |
| » isIsolated  | body | boolean | false    | true-isolated, false-cross; default is false                                  |
| » isHf        | body | boolean | false    | true: high frequency borrowing, false: low frequency borrowing; default false |

#### Enumerated Values

| Parameter     | Value |
| ------------- | ----- |
| » timeInForce | IOC   |
| » timeInForce | FOK   |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "orderNo": {
          "type": "string",
          "description": "Borrow Order ID"
        },
        "actualSize": {
          "type": "string",
          "description": "Actual borrowed amount"
        }
      },
      "required": ["orderNo", "actualSize"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="borrow-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="borrow-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type   | Required | Restrictions | Description            |
| ------------- | ------ | -------- | ------------ | ---------------------- |
| » code        | string | true     | none         | none                   |
| » data        | object | true     | none         | none                   |
| »» orderNo    | string | true     | none         | Borrow Order ID        |
| »» actualSize | string | true     | none         | Actual borrowed amount |

<aside class="success">
This operation does not require authentication
</aside>

## Get Borrow History

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/margin/borrow?currency=BTC,ETH,KCS", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/margin/borrow', params={
  'currency': [
  "BTC",
  "ETH",
  "KCS"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/margin/borrow`

This API endpoint is used to get the borrowing orders for cross and isolated
margin accounts.

<h3 id="get-borrow-history-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                                            |
| ----------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency    | query | string         | true     | currency                                                                                                                                                               |
| isIsolated  | query | boolean        | false    | true-isolated, false-cross; default is false                                                                                                                           |
| symbol      | query | string         | false    | symbol, mandatory for isolated margin account                                                                                                                          |
| orderNo     | query | string         | false    | Borrow Order ID                                                                                                                                                        |
| startTime   | query | integer(int64) | false    | The start and end times are not restricted. If the start time is empty or less than 1680278400000, the default value is set to 1680278400000 (April 1, 2023, 00:00:00) |
| endTime     | query | integer(int64) | false    | End time                                                                                                                                                               |
| currentPage | query | integer        | false    | Current query page, with a starting value of 1. Default:1                                                                                                              |
| pageSize    | query | integer        | false    | Number of results per page. Default is 50, minimum is 10, maximum is 500                                                                                               |

#### Detailed descriptions

**currentPage**: Current query page, with a starting value of 1. Default:1

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "timestamp": {
          "type": "integer",
          "format": "int64"
        },
        "currentPage": {
          "type": "integer",
          "description": "current page"
        },
        "pageSize": {
          "type": "integer",
          "description": "page size"
        },
        "totalNum": {
          "type": "integer",
          "description": "total number"
        },
        "totalPage": {
          "type": "integer",
          "description": "total pages"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "orderNo": {
                "type": "string",
                "description": "Borrow Order ID"
              },
              "symbol": {
                "type": "string",
                "description": "Isolated Margin symbol; empty for cross margin",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "currency": {
                "type": "string",
                "description": "currency"
              },
              "size": {
                "type": "string",
                "description": "Initiated borrow amount"
              },
              "actualSize": {
                "type": "string",
                "description": "Actual borrow amount"
              },
              "status": {
                "type": "string",
                "description": "PENDING: Processing, SUCCESS: Successful, FAILED: Failed",
                "enum": ["PENDING", "SUCCESS", "FAILED"],
                "x-api-enum": [
                  {
                    "value": "PENDING",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "SUCCESS",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "FAILED",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "createdTime": {
                "type": "integer",
                "description": "Borrow time",
                "format": "int64"
              }
            },
            "required": [
              "orderNo",
              "symbol",
              "currency",
              "size",
              "actualSize",
              "status",
              "createdTime"
            ]
          }
        }
      },
      "required": [
        "timestamp",
        "currentPage",
        "pageSize",
        "totalNum",
        "totalPage",
        "items"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-borrow-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-borrow-history-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                              |
| --------------- | -------------- | -------- | ------------ | -------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                     |
| » data          | object         | true     | none         | none                                                     |
| »» timestamp    | integer(int64) | true     | none         | none                                                     |
| »» currentPage  | integer        | true     | none         | current page                                             |
| »» pageSize     | integer        | true     | none         | page size                                                |
| »» totalNum     | integer        | true     | none         | total number                                             |
| »» totalPage    | integer        | true     | none         | total pages                                              |
| »» items        | [object]       | true     | none         | none                                                     |
| »»» orderNo     | string         | true     | none         | Borrow Order ID                                          |
| »»» symbol      | string         | true     | none         | Isolated Margin symbol; empty for cross margin           |
| »»» currency    | string         | true     | none         | currency                                                 |
| »»» size        | string         | true     | none         | Initiated borrow amount                                  |
| »»» actualSize  | string         | true     | none         | Actual borrow amount                                     |
| »»» status      | string         | true     | none         | PENDING: Processing, SUCCESS: Successful, FAILED: Failed |
| »»» createdTime | integer(int64) | true     | none         | Borrow time                                              |

#### Enumerated Values

| Property | Value   |
| -------- | ------- |
| status   | PENDING |
| status   | SUCCESS |
| status   | FAILED  |

<aside class="success">
This operation does not require authentication
</aside>

## Repay

<a id="opId003"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "currency",
      "example": [
        "BTC",
        "ETH",
        "USDT"
      ]
    },
    "size": {
      "type": "number",
      "description": "Borrow amount"
    },
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "isHf": {
      "type": "boolean",
      "description": "true: high frequency borrowing, false: low frequency borrowing; default false",
      "default": false
    }
  },
  "required": [
    "currency",
    "size"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/margin/repay',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/margin/repay', headers = headers)

print(r.json())

```

`POST /api/v3/margin/repay`

This API endpoint is used to initiate an application for cross or isolated
margin repayment.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "currency": {
      "type": "string",
      "description": "currency",
      "example": ["BTC", "ETH", "USDT"]
    },
    "size": {
      "type": "number",
      "description": "Borrow amount"
    },
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "isHf": {
      "type": "boolean",
      "description": "true: high frequency borrowing, false: low frequency borrowing; default false",
      "default": false
    }
  },
  "required": ["currency", "size"]
}
```

<h3 id="repay-parameters">Parameters</h3>

| Name         | In   | Type    | Required | Description                                                                   |
| ------------ | ---- | ------- | -------- | ----------------------------------------------------------------------------- |
| body         | body | object  | false    | none                                                                          |
| » currency   | body | string  | true     | currency                                                                      |
| » size       | body | number  | true     | Borrow amount                                                                 |
| » symbol     | body | string  | false    | symbol, mandatory for isolated margin account                                 |
| » isIsolated | body | boolean | false    | true-isolated, false-cross; default is false                                  |
| » isHf       | body | boolean | false    | true: high frequency borrowing, false: low frequency borrowing; default false |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "timestamp": {
          "type": "integer",
          "format": "int64"
        },
        "orderNo": {
          "type": "string",
          "description": "Repay order ID"
        },
        "actualSize": {
          "type": "string",
          "description": "Actual repay amount"
        }
      },
      "required": ["timestamp", "orderNo", "actualSize"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="repay-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="repay-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type           | Required | Restrictions | Description         |
| ------------- | -------------- | -------- | ------------ | ------------------- |
| » code        | string         | true     | none         | none                |
| » data        | object         | true     | none         | none                |
| »» timestamp  | integer(int64) | true     | none         | none                |
| »» orderNo    | string         | true     | none         | Repay order ID      |
| »» actualSize | string         | true     | none         | Actual repay amount |

<aside class="success">
This operation does not require authentication
</aside>

## Get Repay History

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/margin/repay?currency=BTC,ETH,KCS", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/margin/repay', params={
  'currency': [
  "BTC",
  "ETH",
  "KCS"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/margin/repay`

This API endpoint is used to get the borrowing orders for cross and isolated
margin accounts.

<h3 id="get-repay-history-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                                            |
| ----------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency    | query | string         | true     | currency                                                                                                                                                               |
| isIsolated  | query | boolean        | false    | true-isolated, false-cross; default is false                                                                                                                           |
| symbol      | query | string         | false    | symbol, mandatory for isolated margin account                                                                                                                          |
| orderNo     | query | string         | false    | Repay order ID                                                                                                                                                         |
| startTime   | query | integer(int64) | false    | The start and end times are not restricted. If the start time is empty or less than 1680278400000, the default value is set to 1680278400000 (April 1, 2023, 00:00:00) |
| endTime     | query | integer(int64) | false    | End time                                                                                                                                                               |
| currentPage | query | integer        | false    | Current query page, with a starting value of 1. Default:1                                                                                                              |
| pageSize    | query | integer        | false    | Number of results per page. Default is 50, minimum is 10, maximum is 500                                                                                               |

#### Detailed descriptions

**currentPage**: Current query page, with a starting value of 1. Default:1

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "timestamp": {
          "type": "integer",
          "format": "int64"
        },
        "currentPage": {
          "type": "integer",
          "description": "current page"
        },
        "pageSize": {
          "type": "integer",
          "description": "page size"
        },
        "totalNum": {
          "type": "integer",
          "description": "total number"
        },
        "totalPage": {
          "type": "integer",
          "description": "total pages"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "orderNo": {
                "type": "string",
                "description": "Repay order ID"
              },
              "symbol": {
                "type": "string",
                "description": "Isolated Margin symbol; empty for cross margin"
              },
              "currency": {
                "type": "string",
                "description": "currency"
              },
              "size": {
                "type": "string",
                "description": "Amount of initiated repay"
              },
              "principal": {
                "type": "string",
                "description": "Amount of principal paid"
              },
              "interest": {
                "type": "string",
                "description": "Amount of interest paid"
              },
              "status": {
                "type": "string",
                "description": "PENDING: Processing, SUCCESS: Successful, FAILED: Failed",
                "enum": ["PENDING", "SUCCESS", "FAILED"],
                "x-api-enum": [
                  {
                    "value": "PENDING",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "SUCCESS",
                    "name": "",
                    "description": ""
                  },
                  {
                    "value": "FAILED",
                    "name": "",
                    "description": ""
                  }
                ]
              },
              "createdTime": {
                "type": "integer",
                "description": "Repayment time",
                "format": "int64"
              }
            },
            "required": [
              "symbol",
              "orderNo",
              "currency",
              "size",
              "principal",
              "interest",
              "status",
              "createdTime"
            ]
          }
        }
      },
      "required": [
        "timestamp",
        "currentPage",
        "pageSize",
        "totalNum",
        "totalPage",
        "items"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-repay-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-repay-history-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                              |
| --------------- | -------------- | -------- | ------------ | -------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                     |
| » data          | object         | true     | none         | none                                                     |
| »» timestamp    | integer(int64) | true     | none         | none                                                     |
| »» currentPage  | integer        | true     | none         | current page                                             |
| »» pageSize     | integer        | true     | none         | page size                                                |
| »» totalNum     | integer        | true     | none         | total number                                             |
| »» totalPage    | integer        | true     | none         | total pages                                              |
| »» items        | [object]       | true     | none         | none                                                     |
| »»» orderNo     | string         | true     | none         | Repay order ID                                           |
| »»» symbol      | string         | true     | none         | Isolated Margin symbol; empty for cross margin           |
| »»» currency    | string         | true     | none         | currency                                                 |
| »»» size        | string         | true     | none         | Amount of initiated repay                                |
| »»» principal   | string         | true     | none         | Amount of principal paid                                 |
| »»» interest    | string         | true     | none         | Amount of interest paid                                  |
| »»» status      | string         | true     | none         | PENDING: Processing, SUCCESS: Successful, FAILED: Failed |
| »»» createdTime | integer(int64) | true     | none         | Repayment time                                           |

#### Enumerated Values

| Property | Value   |
| -------- | ------- |
| status   | PENDING |
| status   | SUCCESS |
| status   | FAILED  |

<aside class="success">
This operation does not require authentication
</aside>

## Get Interest History.

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/margin/interest", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/margin/interest', headers = headers)

print(r.json())

```

`GET /api/v3/margin/interest`

Request the interest records of the cross/isolated margin lending via this
endpoint.

<h3 id="get-interest-history.-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                                            |
| ----------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency    | query | string         | false    | currency                                                                                                                                                               |
| isIsolated  | query | boolean        | false    | true-isolated, false-cross; default is false                                                                                                                           |
| symbol      | query | string         | false    | symbol, mandatory for isolated margin account                                                                                                                          |
| startTime   | query | integer(int64) | false    | The start and end times are not restricted. If the start time is empty or less than 1680278400000, the default value is set to 1680278400000 (April 1, 2023, 00:00:00) |
| endTime     | query | integer(int64) | false    | End time                                                                                                                                                               |
| currentPage | query | integer        | false    | Current query page, with a starting value of 1. Default:1                                                                                                              |
| pageSize    | query | integer        | false    | Number of results per page. Default is 50, minimum is 10, maximum is 500                                                                                               |

#### Detailed descriptions

**currentPage**: Current query page, with a starting value of 1. Default:1

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "object",
      "properties": {
        "timestamp": {
          "type": "integer",
          "format": "int64"
        },
        "currentPage": {
          "type": "integer",
          "description": "current page"
        },
        "pageSize": {
          "type": "integer",
          "description": "page size"
        },
        "totalNum": {
          "type": "integer",
          "description": "total number"
        },
        "totalPage": {
          "type": "integer",
          "description": "total pages"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "currency": {
                "type": "string",
                "description": "currency"
              },
              "dayRatio": {
                "type": "string",
                "description": "Daily interest rate"
              },
              "interestAmount": {
                "type": "string",
                "description": "Interest amount"
              },
              "createdTime": {
                "type": "integer",
                "description": "Interest Timestamp",
                "format": "int64"
              }
            },
            "required": [
              "currency",
              "dayRatio",
              "interestAmount",
              "createdTime"
            ]
          }
        }
      },
      "required": [
        "timestamp",
        "currentPage",
        "pageSize",
        "totalNum",
        "totalPage",
        "items"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-interest-history.-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-interest-history.-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description         |
| ------------------ | -------------- | -------- | ------------ | ------------------- |
| » code             | string         | true     | none         | none                |
| » data             | object         | true     | none         | none                |
| »» timestamp       | integer(int64) | true     | none         | none                |
| »» currentPage     | integer        | true     | none         | current page        |
| »» pageSize        | integer        | true     | none         | page size           |
| »» totalNum        | integer        | true     | none         | total number        |
| »» totalPage       | integer        | true     | none         | total pages         |
| »» items           | [object]       | true     | none         | none                |
| »»» currency       | string         | true     | none         | currency            |
| »»» dayRatio       | string         | true     | none         | Daily interest rate |
| »»» interestAmount | string         | true     | none         | Interest amount     |
| »»» createdTime    | integer(int64) | true     | none         | Interest Timestamp  |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Leverage

<a id="opId006"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "leverage": {
      "type": "string",
      "description": "New leverage multiplier. Must be greater than 1 and up to two decimal places, and cannot be less than the user's current debt leverage or greater than the system's maximum leverage"
    }
  },
  "required": [
    "leverage"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/position/update-user-leverage',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json'
}

r = requests.post('/api/v3/position/update-user-leverage', headers = headers)

print(r.json())

```

`POST /api/v3/position/update-user-leverage`

This endpoint allows modifying the leverage multiplier for cross margin or
isolated margin.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "symbol, mandatory for isolated margin account",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "isIsolated": {
      "type": "boolean",
      "description": "true-isolated, false-cross; default is false",
      "default": false
    },
    "leverage": {
      "type": "string",
      "description": "New leverage multiplier. Must be greater than 1 and up to two decimal places, and cannot be less than the user's current debt leverage or greater than the system's maximum leverage"
    }
  },
  "required": ["leverage"]
}
```

<h3 id="modify-leverage-parameters">Parameters</h3>

| Name         | In   | Type    | Required | Description                                                                                                                                                                          |
| ------------ | ---- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body         | body | object  | false    | none                                                                                                                                                                                 |
| » symbol     | body | string  | false    | symbol, mandatory for isolated margin account                                                                                                                                        |
| » isIsolated | body | boolean | false    | true-isolated, false-cross; default is false                                                                                                                                         |
| » leverage   | body | string  | true     | New leverage multiplier. Must be greater than 1 and up to two decimal places, and cannot be less than the user's current debt leverage or greater than the system's maximum leverage |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "string"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-leverage-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-leverage-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description |
| ------ | ------ | -------- | ------------ | ----------- |
| » code | string | true     | none         | none        |
| » data | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Margin Risk Limit

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/margin/currencies", {
  method: "GET",

  headers: headers
})
  .then(function (res) {
    return res.json()
  })
  .then(function (body) {
    console.log(body)
  })
```

```python
import requests
headers = {
  'Accept': 'application/json'
}

r = requests.get('/api/v3/margin/currencies', headers = headers)

print(r.json())

```

`GET /api/v3/margin/currencies`

Request Configure and Risk limit info of the margin via this endpoint.

<h3 id="get-margin-risk-limit-parameters">Parameters</h3>

| Name       | In    | Type    | Required | Description                                             |
| ---------- | ----- | ------- | -------- | ------------------------------------------------------- |
| isIsolated | query | boolean | false    | True-isolated, false-cross                              |
| currency   | query | string  | false    | Currency: This field is only required for cross margin  |
| symbol     | query | string  | false    | Symbol: This field is only required for isolated margin |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "timestamp": {
            "type": "integer",
            "description": "Time stamp",
            "format": "int64"
          },
          "currency": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Currency"
          },
          "borrowMaxAmount": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed."
          },
          "buyMaxAmount": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Maximum buy amount"
          },
          "holdMaxAmount": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Maximum holding amount"
          },
          "borrowCoefficient": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, [Borrow Coefficient](https://www.kucoin.com/land/price-protect)"
          },
          "marginCoefficient": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, [Margin Coefficient](https://www.kucoin.com/land/price-protect)"
          },
          "precision": {
            "type": "integer",
            "description": "CROSS MARGIN RESPONSES, Currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001"
          },
          "borrowMinAmount": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Minimum personal borrow amount"
          },
          "borrowMinUnit": {
            "type": "string",
            "description": "CROSS MARGIN RESPONSES, Minimum unit for borrowing; the borrowed amount must be an integer multiple of this value"
          },
          "borrowEnabled": {
            "type": "boolean",
            "description": "CROSS MARGIN RESPONSES, Whether to support borrowing"
          },
          "symbol": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Symbol"
          },
          "baseMaxBorrowAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Base maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed."
          },
          "quoteMaxBorrowAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Quote maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed."
          },
          "baseMaxBuyAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Base maximum buy amount\n"
          },
          "quoteMaxBuyAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Quote maximum buy amount"
          },
          "baseMaxHoldAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Base maximum holding amount\n"
          },
          "quoteMaxHoldAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Quote maximum holding amount\n"
          },
          "basePrecision": {
            "type": "integer",
            "description": "ISOLATED MARGIN RESPONSES, Base currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001"
          },
          "quotePrecision": {
            "type": "integer",
            "description": "ISOLATED MARGIN RESPONSES, Quote currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001\n"
          },
          "baseBorrowMinAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Base minimum personal borrow amount\n"
          },
          "quoteBorrowMinAmount": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Quote minimum personal borrow amount"
          },
          "baseBorrowMinUnit": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Base minimum unit for borrowing, the borrowed amount must be an integer multiple of this value"
          },
          "quoteBorrowMinUnit": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, Quote minimum unit for borrowing, the borrowed amount must be an integer multiple of this value"
          },
          "baseBorrowEnabled": {
            "type": "boolean",
            "description": "ISOLATED MARGIN RESPONSES, Base whether to support borrowing\n"
          },
          "quoteBorrowEnabled": {
            "type": "boolean",
            "description": "ISOLATED MARGIN RESPONSES, Quote whether to support borrowing\n"
          },
          "baseBorrowCoefficient": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, [Base Borrow Coefficient](https://www.kucoin.com/land/price-protect)"
          },
          "quoteBorrowCoefficient": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, [Quote Borrow Coefficient](https://www.kucoin.com/land/price-protect)"
          },
          "baseMarginCoefficient": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, [Base Margin Coefficient](https://www.kucoin.com/land/price-protect)"
          },
          "quoteMarginCoefficient": {
            "type": "string",
            "description": "ISOLATED MARGIN RESPONSES, [Quote Margin Coefficient](https://www.kucoin.com/land/price-protect)"
          }
        }
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-margin-risk-limit-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-margin-risk-limit-responseschema">Response Schema</h3>

Status Code **200**

| Name                      | Type           | Required | Restrictions | Description                                                                                                                                                                                                                       |
| ------------------------- | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code                    | string         | true     | none         | none                                                                                                                                                                                                                              |
| » data                    | [object]       | true     | none         | none                                                                                                                                                                                                                              |
| »» timestamp              | integer(int64) | false    | none         | Time stamp                                                                                                                                                                                                                        |
| »» currency               | string         | false    | none         | CROSS MARGIN RESPONSES, Currency                                                                                                                                                                                                  |
| »» borrowMaxAmount        | string         | false    | none         | CROSS MARGIN RESPONSES, Maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed.                                                                                              |
| »» buyMaxAmount           | string         | false    | none         | CROSS MARGIN RESPONSES, Maximum buy amount                                                                                                                                                                                        |
| »» holdMaxAmount          | string         | false    | none         | CROSS MARGIN RESPONSES, Maximum holding amount                                                                                                                                                                                    |
| »» borrowCoefficient      | string         | false    | none         | CROSS MARGIN RESPONSES, [Borrow Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                           |
| »» marginCoefficient      | string         | false    | none         | CROSS MARGIN RESPONSES, [Margin Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                           |
| »» precision              | integer        | false    | none         | CROSS MARGIN RESPONSES, Currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001          |
| »» borrowMinAmount        | string         | false    | none         | CROSS MARGIN RESPONSES, Minimum personal borrow amount                                                                                                                                                                            |
| »» borrowMinUnit          | string         | false    | none         | CROSS MARGIN RESPONSES, Minimum unit for borrowing; the borrowed amount must be an integer multiple of this value                                                                                                                 |
| »» borrowEnabled          | boolean        | false    | none         | CROSS MARGIN RESPONSES, Whether to support borrowing                                                                                                                                                                              |
| »» symbol                 | string         | false    | none         | ISOLATED MARGIN RESPONSES, Symbol                                                                                                                                                                                                 |
| »» baseMaxBorrowAmount    | string         | false    | none         | ISOLATED MARGIN RESPONSES, Base maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed.                                                                                      |
| »» quoteMaxBorrowAmount   | string         | false    | none         | ISOLATED MARGIN RESPONSES, Quote maximum personal borrow amount. If the platform has no borrowing amount, this value will still be displayed.                                                                                     |
| »» baseMaxBuyAmount       | string         | false    | none         | ISOLATED MARGIN RESPONSES, Base maximum buy amount                                                                                                                                                                                |
| »» quoteMaxBuyAmount      | string         | false    | none         | ISOLATED MARGIN RESPONSES, Quote maximum buy amount                                                                                                                                                                               |
| »» baseMaxHoldAmount      | string         | false    | none         | ISOLATED MARGIN RESPONSES, Base maximum holding amount                                                                                                                                                                            |
| »» quoteMaxHoldAmount     | string         | false    | none         | ISOLATED MARGIN RESPONSES, Quote maximum holding amount                                                                                                                                                                           |
| »» basePrecision          | integer        | false    | none         | ISOLATED MARGIN RESPONSES, Base currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001  |
| »» quotePrecision         | integer        | false    | none         | ISOLATED MARGIN RESPONSES, Quote currency precision. The minimum repayment amount of a single transaction should be >= currency precision. For example, the precision of ETH is 8, and the minimum repayment amount is 0.00000001 |
| »» baseBorrowMinAmount    | string         | false    | none         | ISOLATED MARGIN RESPONSES, Base minimum personal borrow amount                                                                                                                                                                    |
| »» quoteBorrowMinAmount   | string         | false    | none         | ISOLATED MARGIN RESPONSES, Quote minimum personal borrow amount                                                                                                                                                                   |
| »» baseBorrowMinUnit      | string         | false    | none         | ISOLATED MARGIN RESPONSES, Base minimum unit for borrowing, the borrowed amount must be an integer multiple of this value                                                                                                         |
| »» quoteBorrowMinUnit     | string         | false    | none         | ISOLATED MARGIN RESPONSES, Quote minimum unit for borrowing, the borrowed amount must be an integer multiple of this value                                                                                                        |
| »» baseBorrowEnabled      | boolean        | false    | none         | ISOLATED MARGIN RESPONSES, Base whether to support borrowing                                                                                                                                                                      |
| »» quoteBorrowEnabled     | boolean        | false    | none         | ISOLATED MARGIN RESPONSES, Quote whether to support borrowing                                                                                                                                                                     |
| »» baseBorrowCoefficient  | string         | false    | none         | ISOLATED MARGIN RESPONSES, [Base Borrow Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                   |
| »» quoteBorrowCoefficient | string         | false    | none         | ISOLATED MARGIN RESPONSES, [Quote Borrow Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                  |
| »» baseMarginCoefficient  | string         | false    | none         | ISOLATED MARGIN RESPONSES, [Base Margin Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                   |
| »» quoteMarginCoefficient | string         | false    | none         | ISOLATED MARGIN RESPONSES, [Quote Margin Coefficient](https://www.kucoin.com/land/price-protect)                                                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
