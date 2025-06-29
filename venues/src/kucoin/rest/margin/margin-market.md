---
title: margin v1.0.0
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

<h1 id="margin">margin v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

market

<h1 id="margin-default">Default</h1>

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

# Schemas
