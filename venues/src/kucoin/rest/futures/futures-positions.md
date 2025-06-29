---
title: futures v1.0.0
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

<h1 id="futures">futures v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

positions

<h1 id="futures-default">Default</h1>

## Get Margin Mode

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v2/position/getMarginMode?symbol=type,string", {
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

r = requests.get('/api/v2/position/getMarginMode', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v2/position/getMarginMode`

This interface can query the margin mode of the current symbol.

<h3 id="get-margin-mode-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "marginMode": {
          "type": "string",
          "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).",
          "enum": ["ISOLATED", "CROSS"],
          "x-api-enum": [
            {
              "value": "ISOLATED",
              "name": "ISOLATED",
              "description": "isolated margin"
            },
            {
              "value": "CROSS",
              "name": "CROSS",
              "description": "cross margin"
            }
          ]
        }
      },
      "required": ["symbol", "marginMode"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-margin-mode-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-margin-mode-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type   | Required | Restrictions | Description                                                                                                        |
| ------------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code        | string | true     | none         | none                                                                                                               |
| » data        | object | true     | none         | none                                                                                                               |
| »» symbol     | string | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» marginMode | string | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                            |

#### Enumerated Values

| Property   | Value    |
| ---------- | -------- |
| marginMode | ISOLATED |
| marginMode | CROSS    |

<aside class="success">
This operation does not require authentication
</aside>

## Switch Margin Mode

<a id="opId002"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": [
        "XBTUSDTM",
        "XBTUSDCM",
        "ETHUSDTM"
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Modified margin model: ISOLATED (isolated), CROSS (cross margin).",
      "enum": [
        "ISOLATED",
        "CROSS"
      ],
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin Mode"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin MOde"
        }
      ]
    }
  },
  "required": [
    "symbol",
    "marginMode"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v2/position/changeMarginMode',
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

r = requests.post('/api/v2/position/changeMarginMode', headers = headers)

print(r.json())

```

`POST /api/v2/position/changeMarginMode`

This interface can modify the margin mode of the current symbol.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": ["XBTUSDTM", "XBTUSDCM", "ETHUSDTM"]
    },
    "marginMode": {
      "type": "string",
      "description": "Modified margin model: ISOLATED (isolated), CROSS (cross margin).",
      "enum": ["ISOLATED", "CROSS"],
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin Mode"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin MOde"
        }
      ]
    }
  },
  "required": ["symbol", "marginMode"]
}
```

<h3 id="switch-margin-mode-parameters">Parameters</h3>

| Name         | In   | Type   | Required | Description                                                                                                        |
| ------------ | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| body         | body | object | false    | none                                                                                                               |
| » symbol     | body | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| » marginMode | body | string | true     | Modified margin model: ISOLATED (isolated), CROSS (cross margin).                                                  |

#### Enumerated Values

| Parameter    | Value    |
| ------------ | -------- |
| » marginMode | ISOLATED |
| » marginMode | CROSS    |

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
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "marginMode": {
          "type": "string",
          "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).",
          "enum": ["ISOLATED", "CROSS"],
          "x-api-enum": [
            {
              "value": "ISOLATED",
              "name": "ISOLATED",
              "description": "isolated margin"
            },
            {
              "value": "CROSS",
              "name": "CROSS",
              "description": "cross margin"
            }
          ]
        }
      },
      "required": ["symbol", "marginMode"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="switch-margin-mode-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="switch-margin-mode-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type   | Required | Restrictions | Description                                                                                                        |
| ------------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code        | string | true     | none         | none                                                                                                               |
| » data        | object | true     | none         | none                                                                                                               |
| »» symbol     | string | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» marginMode | string | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                            |

#### Enumerated Values

| Property   | Value    |
| ---------- | -------- |
| marginMode | ISOLATED |
| marginMode | CROSS    |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Switch Margin Mode

<a id="opId003"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "marginMode": {
      "type": "string",
      "description": "Modified margin model: ISOLATED (isolated), CROSS (cross margin).",
      "enum": [
        "ISOLATED",
        "CROSS"
      ],
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin Mode"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin MOde"
        }
      ]
    },
    "symbols": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "Symbol list of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    }
  },
  "required": [
    "marginMode",
    "symbols"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v2/position/batchChangeMarginMode',
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

r = requests.post('/api/v2/position/batchChangeMarginMode', headers = headers)

print(r.json())

```

`POST /api/v2/position/batchChangeMarginMode`

Batch modify the margin mode of the symbols.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "marginMode": {
      "type": "string",
      "description": "Modified margin model: ISOLATED (isolated), CROSS (cross margin).",
      "enum": ["ISOLATED", "CROSS"],
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin Mode"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin MOde"
        }
      ]
    },
    "symbols": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "Symbol list of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    }
  },
  "required": ["marginMode", "symbols"]
}
```

<h3 id="batch-switch-margin-mode-parameters">Parameters</h3>

| Name         | In   | Type     | Required | Description                                                                                                             |
| ------------ | ---- | -------- | -------- | ----------------------------------------------------------------------------------------------------------------------- |
| body         | body | object   | false    | none                                                                                                                    |
| » marginMode | body | string   | true     | Modified margin model: ISOLATED (isolated), CROSS (cross margin).                                                       |
| » symbols    | body | [string] | true     | Symbol list of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

#### Enumerated Values

| Parameter    | Value    |
| ------------ | -------- |
| » marginMode | ISOLATED |
| » marginMode | CROSS    |

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
        "marginMode": {
          "type": "object",
          "properties": {},
          "description": "Target Margin Model, Symbols that failed to be modified will also be included",
          "additionalProperties": {
            "type": "string"
          }
        },
        "errors": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "code": {
                "type": "string",
                "description": "Error code"
              },
              "msg": {
                "type": "string",
                "description": "Error message"
              },
              "symbol": {
                "type": "string",
                "description": "Symbol"
              }
            }
          },
          "description": "Symbol which modification failed"
        }
      },
      "required": ["marginMode", "errors"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-switch-margin-mode-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-switch-margin-mode-responseschema">Response Schema</h3>

Status Code **200**

| Name                         | Type     | Required | Restrictions | Description                                                                   |
| ---------------------------- | -------- | -------- | ------------ | ----------------------------------------------------------------------------- |
| » code                       | string   | true     | none         | none                                                                          |
| » data                       | object   | true     | none         | none                                                                          |
| »» marginMode                | object   | true     | none         | Target Margin Model, Symbols that failed to be modified will also be included |
| »»» **additionalProperties** | string   | false    | none         | none                                                                          |
| »» errors                    | [object] | true     | none         | Symbol which modification failed                                              |
| »»» code                     | string   | false    | none         | Error code                                                                    |
| »»» msg                      | string   | false    | none         | Error message                                                                 |
| »»» symbol                   | string   | false    | none         | Symbol                                                                        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Max Open Size

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v2/getMaxOpenSize?symbol=XBTUSDTM,XBTUSDM,ETHUSDTM&price=type,string&leverage=type,integer",
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

r = requests.get('/api/v2/getMaxOpenSize', params={
  'symbol': [
  "XBTUSDTM",
  "XBTUSDM",
  "ETHUSDTM"
],  'price': {
  "type": "string"
},  'leverage': {
  "type": "integer"
}
}, headers = headers)

print(r.json())

```

`GET /api/v2/getMaxOpenSize`

Get Maximum Open Position Size.

<h3 id="get-max-open-size-parameters">Parameters</h3>

| Name     | In    | Type    | Required | Description                                                                                                        |
| -------- | ----- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol   | query | string  | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| price    | query | string  | true     | Order Price                                                                                                        |
| leverage | query | integer | true     | Leverage                                                                                                           |

#### Detailed descriptions

**price**: Order Price

**leverage**: Leverage

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
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "maxBuyOpenSize": {
          "type": "integer",
          "description": "Maximum buy size (unit: lot)\n"
        },
        "maxSellOpenSize": {
          "type": "integer",
          "description": "Maximum buy size (unit: lot)\n"
        }
      },
      "required": ["symbol", "maxBuyOpenSize", "maxSellOpenSize"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-max-open-size-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-max-open-size-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type    | Required | Restrictions | Description                                                                                                        |
| ------------------ | ------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code             | string  | true     | none         | none                                                                                                               |
| » data             | object  | true     | none         | none                                                                                                               |
| »» symbol          | string  | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» maxBuyOpenSize  | integer | true     | none         | Maximum buy size (unit: lot)                                                                                       |
| »» maxSellOpenSize | integer | true     | none         | Maximum buy size (unit: lot)                                                                                       |

<aside class="success">
This operation does not require authentication
</aside>

## Get Position Details

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/position?symbol=type,string", {
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

r = requests.get('/api/v1/position', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/position`

Get the position details of a specified position.

<h3 id="get-position-details-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
          "description": "Position ID\n"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
          "example": ["XBTUSDTM", "XBTUSDM", "ETHUSDTM"]
        },
        "crossMode": {
          "type": "boolean",
          "description": "Whether it is cross margin."
        },
        "delevPercentage": {
          "type": "number",
          "description": "ADL ranking percentile\n"
        },
        "openingTimestamp": {
          "type": "integer",
          "description": "First opening time",
          "format": "int64"
        },
        "currentTimestamp": {
          "type": "integer",
          "description": "Current timestamp\n",
          "format": "int64"
        },
        "currentQty": {
          "type": "integer",
          "description": "Current postion quantity\n"
        },
        "currentCost": {
          "type": "number",
          "description": "Current postion value\n"
        },
        "currentComm": {
          "type": "number",
          "description": "Current commission\n"
        },
        "unrealisedCost": {
          "type": "number",
          "description": "Unrealised value\n"
        },
        "realisedGrossCost": {
          "type": "number",
          "description": "Accumulated realised gross profit value\n"
        },
        "realisedCost": {
          "type": "number",
          "description": "Current realised position value\n"
        },
        "isOpen": {
          "type": "boolean",
          "description": "Opened position or not\n"
        },
        "markPrice": {
          "type": "number",
          "description": "Mark price\n"
        },
        "markValue": {
          "type": "number",
          "description": "Mark Value\n"
        },
        "posCost": {
          "type": "number",
          "description": "Position value\n"
        },
        "posInit": {
          "type": "number",
          "description": "Inital margin Cross = opening value/cross leverage; isolated = accumulation of initial margin for each transaction\n"
        },
        "posMargin": {
          "type": "number",
          "description": "Bankruptcy cost Cross = mark value * imr; Isolated = position margin (accumulation of initial margin, additional margin, generated funding fees, etc.)\n"
        },
        "realisedGrossPnl": {
          "type": "number",
          "description": "Accumulated realised gross profit value\n"
        },
        "realisedPnl": {
          "type": "number",
          "description": "Realised profit and loss\n"
        },
        "unrealisedPnl": {
          "type": "number",
          "description": "Unrealised profit and loss\n"
        },
        "unrealisedPnlPcnt": {
          "type": "number",
          "description": "Profit-loss ratio of the position\n"
        },
        "unrealisedRoePcnt": {
          "type": "number",
          "description": "Rate of return on investment\n"
        },
        "avgEntryPrice": {
          "type": "number",
          "description": "Average entry price\n"
        },
        "liquidationPrice": {
          "type": "number",
          "description": "Liquidation price For Cross Margin, you can refer to the liquidationPrice, and the liquidation is based on the risk rate.\n"
        },
        "bankruptPrice": {
          "type": "number",
          "description": "Bankruptcy price For Cross Margin, you can refer to the bankruptPrice, and the liquidation is based on the risk rate.\n"
        },
        "settleCurrency": {
          "type": "string",
          "description": "Currency used to clear and settle the trades\n"
        },
        "isInverse": {
          "type": "boolean",
          "description": "Reverse contract or not\n"
        },
        "marginMode": {
          "type": "string",
          "description": "Margin Mode: CROSS，ISOLATED\n",
          "enum": ["CROSS", "ISOLATED"],
          "x-api-enum": [
            {
              "value": "CROSS",
              "name": "CROSS",
              "description": "cross margin"
            },
            {
              "value": "ISOLATED",
              "name": "ISOLATED",
              "description": "isolated margin"
            }
          ]
        },
        "positionSide": {
          "type": "string",
          "description": "Position Side\n",
          "enum": ["BOTH"],
          "x-api-enum": [
            {
              "value": "BOTH",
              "name": "BOTH",
              "description": "One-way position"
            }
          ]
        },
        "leverage": {
          "type": "number",
          "description": "Leverage"
        },
        "autoDeposit": {
          "type": "boolean",
          "description": "Auto deposit margin or not **Only applicable to Isolated Margin**\n"
        },
        "maintMarginReq": {
          "type": "number",
          "description": "Maintenance margin requirement\n"
        },
        "riskLimit": {
          "type": "integer",
          "description": "Risk limit **Only applicable to Isolated Margin**\n"
        },
        "realLeverage": {
          "type": "number",
          "description": "Leverage of the order **Only applicable to Isolated Margin**\n"
        },
        "posCross": {
          "type": "number",
          "description": "added margin **Only applicable to Isolated Margin**\n"
        },
        "posCrossMargin": {
          "type": "integer",
          "description": "Additional margin calls (automatic, manual, adjusted risk limits) **Only applicable to Isolated Margin**\n"
        },
        "posComm": {
          "type": "number",
          "description": "Bankruptcy cost **Only applicable to Isolated Margin**\n"
        },
        "posCommCommon": {
          "type": "number",
          "description": "Part of bankruptcy cost (positioning, add margin) **Only applicable to Isolated Margin**\n"
        },
        "posLoss": {
          "type": "number",
          "description": "Funding fees paid out **Only applicable to Isolated Margin**\n"
        },
        "posFunding": {
          "type": "number",
          "description": "The current remaining unsettled funding fee for the position **Only applicable to Isolated Margin**\n"
        },
        "posMaint": {
          "type": "number",
          "description": "Maintenance margin\n"
        },
        "maintMargin": {
          "type": "number",
          "description": "Position margin **Only applicable to Isolated Margin**\n"
        },
        "maintainMargin": {
          "type": "number",
          "description": "Maintenance margin rate **Only applicable to Isolated Margin**\n"
        }
      },
      "required": [
        "id",
        "symbol",
        "crossMode",
        "delevPercentage",
        "openingTimestamp",
        "currentTimestamp",
        "currentQty",
        "currentCost",
        "currentComm",
        "unrealisedCost",
        "realisedGrossCost",
        "realisedCost",
        "isOpen",
        "markPrice",
        "markValue",
        "posCost",
        "posInit",
        "posMargin",
        "realisedGrossPnl",
        "realisedPnl",
        "unrealisedPnl",
        "unrealisedPnlPcnt",
        "unrealisedRoePcnt",
        "avgEntryPrice",
        "liquidationPrice",
        "bankruptPrice",
        "settleCurrency",
        "isInverse",
        "marginMode",
        "positionSide",
        "leverage"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-position-details-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-position-details-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type           | Required | Restrictions | Description                                                                                                                                             |
| -------------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code               | string         | true     | none         | none                                                                                                                                                    |
| » data               | object         | true     | none         | none                                                                                                                                                    |
| »» id                | string         | true     | none         | Position ID                                                                                                                                             |
| »» symbol            | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                      |
| »» crossMode         | boolean        | true     | none         | Whether it is cross margin.                                                                                                                             |
| »» delevPercentage   | number         | true     | none         | ADL ranking percentile                                                                                                                                  |
| »» openingTimestamp  | integer(int64) | true     | none         | First opening time                                                                                                                                      |
| »» currentTimestamp  | integer(int64) | true     | none         | Current timestamp                                                                                                                                       |
| »» currentQty        | integer        | true     | none         | Current postion quantity                                                                                                                                |
| »» currentCost       | number         | true     | none         | Current postion value                                                                                                                                   |
| »» currentComm       | number         | true     | none         | Current commission                                                                                                                                      |
| »» unrealisedCost    | number         | true     | none         | Unrealised value                                                                                                                                        |
| »» realisedGrossCost | number         | true     | none         | Accumulated realised gross profit value                                                                                                                 |
| »» realisedCost      | number         | true     | none         | Current realised position value                                                                                                                         |
| »» isOpen            | boolean        | true     | none         | Opened position or not                                                                                                                                  |
| »» markPrice         | number         | true     | none         | Mark price                                                                                                                                              |
| »» markValue         | number         | true     | none         | Mark Value                                                                                                                                              |
| »» posCost           | number         | true     | none         | Position value                                                                                                                                          |
| »» posInit           | number         | true     | none         | Inital margin Cross = opening value/cross leverage; isolated = accumulation of initial margin for each transaction                                      |
| »» posMargin         | number         | true     | none         | Bankruptcy cost Cross = mark value \* imr; Isolated = position margin (accumulation of initial margin, additional margin, generated funding fees, etc.) |
| »» realisedGrossPnl  | number         | true     | none         | Accumulated realised gross profit value                                                                                                                 |
| »» realisedPnl       | number         | true     | none         | Realised profit and loss                                                                                                                                |
| »» unrealisedPnl     | number         | true     | none         | Unrealised profit and loss                                                                                                                              |
| »» unrealisedPnlPcnt | number         | true     | none         | Profit-loss ratio of the position                                                                                                                       |
| »» unrealisedRoePcnt | number         | true     | none         | Rate of return on investment                                                                                                                            |
| »» avgEntryPrice     | number         | true     | none         | Average entry price                                                                                                                                     |
| »» liquidationPrice  | number         | true     | none         | Liquidation price For Cross Margin, you can refer to the liquidationPrice, and the liquidation is based on the risk rate.                               |
| »» bankruptPrice     | number         | true     | none         | Bankruptcy price For Cross Margin, you can refer to the bankruptPrice, and the liquidation is based on the risk rate.                                   |
| »» settleCurrency    | string         | true     | none         | Currency used to clear and settle the trades                                                                                                            |
| »» isInverse         | boolean        | true     | none         | Reverse contract or not                                                                                                                                 |
| »» marginMode        | string         | true     | none         | Margin Mode: CROSS，ISOLATED                                                                                                                            |
| »» positionSide      | string         | true     | none         | Position Side                                                                                                                                           |
| »» leverage          | number         | true     | none         | Leverage                                                                                                                                                |
| »» autoDeposit       | boolean        | false    | none         | Auto deposit margin or not **Only applicable to Isolated Margin**                                                                                       |
| »» maintMarginReq    | number         | false    | none         | Maintenance margin requirement                                                                                                                          |
| »» riskLimit         | integer        | false    | none         | Risk limit **Only applicable to Isolated Margin**                                                                                                       |
| »» realLeverage      | number         | false    | none         | Leverage of the order **Only applicable to Isolated Margin**                                                                                            |
| »» posCross          | number         | false    | none         | added margin **Only applicable to Isolated Margin**                                                                                                     |
| »» posCrossMargin    | integer        | false    | none         | Additional margin calls (automatic, manual, adjusted risk limits) **Only applicable to Isolated Margin**                                                |
| »» posComm           | number         | false    | none         | Bankruptcy cost **Only applicable to Isolated Margin**                                                                                                  |
| »» posCommCommon     | number         | false    | none         | Part of bankruptcy cost (positioning, add margin) **Only applicable to Isolated Margin**                                                                |
| »» posLoss           | number         | false    | none         | Funding fees paid out **Only applicable to Isolated Margin**                                                                                            |
| »» posFunding        | number         | false    | none         | The current remaining unsettled funding fee for the position **Only applicable to Isolated Margin**                                                     |
| »» posMaint          | number         | false    | none         | Maintenance margin                                                                                                                                      |
| »» maintMargin       | number         | false    | none         | Position margin **Only applicable to Isolated Margin**                                                                                                  |
| »» maintainMargin    | number         | false    | none         | Maintenance margin rate **Only applicable to Isolated Margin**                                                                                          |

#### Enumerated Values

| Property     | Value    |
| ------------ | -------- |
| marginMode   | CROSS    |
| marginMode   | ISOLATED |
| positionSide | BOTH     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Position List

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/positions", {
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

r = requests.get('/api/v1/positions', headers = headers)

print(r.json())

```

`GET /api/v1/positions`

Get the position details of a specified position.

<h3 id="get-position-list-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description                                                                                                                                         |
| -------- | ----- | ------ | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency | query | string | false    | Quote currency code, Please refer to [rootSymbol](https://www.kucoin.com/docs-new/api-221752070) , such as USDT,XBT. Query all positions when empty |

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
            "description": "Position ID\n"
          },
          "symbol": {
            "type": "string",
            "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
            "example": ["XBTUSDTM", "XBTUSDM", "ETHUSDTM"]
          },
          "crossMode": {
            "type": "boolean",
            "description": "Whether it is cross margin."
          },
          "delevPercentage": {
            "type": "number",
            "description": "ADL ranking percentile\n"
          },
          "openingTimestamp": {
            "type": "integer",
            "description": "First opening time",
            "format": "int64"
          },
          "currentTimestamp": {
            "type": "integer",
            "description": "Current timestamp\n",
            "format": "int64"
          },
          "currentQty": {
            "type": "integer",
            "description": "Current postion quantity\n"
          },
          "currentCost": {
            "type": "number",
            "description": "Current postion value\n"
          },
          "currentComm": {
            "type": "number",
            "description": "Current commission\n"
          },
          "unrealisedCost": {
            "type": "number",
            "description": "Unrealised value\n"
          },
          "realisedGrossCost": {
            "type": "number",
            "description": "Accumulated realised gross profit value\n"
          },
          "realisedCost": {
            "type": "number",
            "description": "Current realised position value\n"
          },
          "isOpen": {
            "type": "boolean",
            "description": "Opened position or not\n"
          },
          "markPrice": {
            "type": "number",
            "description": "Mark price\n"
          },
          "markValue": {
            "type": "number",
            "description": "Mark Value\n"
          },
          "posCost": {
            "type": "number",
            "description": "Position value\n"
          },
          "posInit": {
            "type": "number",
            "description": "Inital margin Cross = opening value/cross leverage; isolated = accumulation of initial margin for each transaction\n"
          },
          "posMargin": {
            "type": "number",
            "description": "Bankruptcy cost Cross = mark value * imr; Isolated = position margin (accumulation of initial margin, additional margin, generated funding fees, etc.)\n"
          },
          "realisedGrossPnl": {
            "type": "number",
            "description": "Accumulated realised gross profit value\n"
          },
          "realisedPnl": {
            "type": "number",
            "description": "Realised profit and loss\n"
          },
          "unrealisedPnl": {
            "type": "number",
            "description": "Unrealised profit and loss\n"
          },
          "unrealisedPnlPcnt": {
            "type": "number",
            "description": "Profit-loss ratio of the position\n"
          },
          "unrealisedRoePcnt": {
            "type": "number",
            "description": "Rate of return on investment\n"
          },
          "avgEntryPrice": {
            "type": "number",
            "description": "Average entry price\n"
          },
          "liquidationPrice": {
            "type": "number",
            "description": "Liquidation price For Cross Margin, you can refer to the liquidationPrice, and the liquidation is based on the risk rate.\n"
          },
          "bankruptPrice": {
            "type": "number",
            "description": "Bankruptcy price For Cross Margin, you can refer to the bankruptPrice, and the liquidation is based on the risk rate.\n"
          },
          "settleCurrency": {
            "type": "string",
            "description": "Currency used to clear and settle the trades\n"
          },
          "isInverse": {
            "type": "boolean",
            "description": "Reverse contract or not\n"
          },
          "marginMode": {
            "type": "string",
            "description": "Margin Mode: CROSS，ISOLATED\n",
            "enum": ["CROSS", "ISOLATED"],
            "x-api-enum": [
              {
                "value": "CROSS",
                "name": "CROSS",
                "description": "cross margin"
              },
              {
                "value": "ISOLATED",
                "name": "ISOLATED",
                "description": "isolated margin"
              }
            ]
          },
          "positionSide": {
            "type": "string",
            "description": "Position Side\n",
            "enum": ["BOTH"],
            "x-api-enum": [
              {
                "value": "BOTH",
                "name": "BOTH",
                "description": "One-way position"
              }
            ]
          },
          "leverage": {
            "type": "number",
            "description": "Leverage"
          },
          "autoDeposit": {
            "type": "boolean",
            "description": "Auto deposit margin or not **Only applicable to Isolated Margin**\n"
          },
          "maintMarginReq": {
            "type": "number",
            "description": "Maintenance margin requirement\n"
          },
          "riskLimit": {
            "type": "number",
            "description": "Risk limit **Only applicable to Isolated Margin**\n"
          },
          "realLeverage": {
            "type": "number",
            "description": "Leverage of the order **Only applicable to Isolated Margin**\n"
          },
          "posCross": {
            "type": "number",
            "description": "added margin **Only applicable to Isolated Margin**\n"
          },
          "posCrossMargin": {
            "type": "number",
            "description": "Additional margin calls (automatic, manual, adjusted risk limits) **Only applicable to Isolated Margin**\n"
          },
          "posComm": {
            "type": "number",
            "description": "Bankruptcy cost **Only applicable to Isolated Margin**\n"
          },
          "posCommCommon": {
            "type": "number",
            "description": "Part of bankruptcy cost (positioning, add margin) **Only applicable to Isolated Margin**\n"
          },
          "posLoss": {
            "type": "number",
            "description": "Funding fees paid out **Only applicable to Isolated Margin**\n"
          },
          "posFunding": {
            "type": "number",
            "description": "The current remaining unsettled funding fee for the position **Only applicable to Isolated Margin**\n"
          },
          "posMaint": {
            "type": "number",
            "description": "Maintenance margin"
          },
          "maintMargin": {
            "type": "number",
            "description": "Position margin **Only applicable to Isolated Margin**\n"
          },
          "maintainMargin": {
            "type": "number",
            "description": "Maintenance margin rate **Only applicable to Isolated Margin**\n"
          }
        },
        "required": [
          "id",
          "symbol",
          "crossMode",
          "delevPercentage",
          "openingTimestamp",
          "currentTimestamp",
          "currentQty",
          "currentCost",
          "currentComm",
          "unrealisedCost",
          "realisedGrossCost",
          "realisedCost",
          "isOpen",
          "markPrice",
          "markValue",
          "posCost",
          "posInit",
          "posMargin",
          "realisedGrossPnl",
          "realisedPnl",
          "unrealisedPnl",
          "unrealisedPnlPcnt",
          "unrealisedRoePcnt",
          "avgEntryPrice",
          "liquidationPrice",
          "bankruptPrice",
          "settleCurrency",
          "isInverse",
          "marginMode",
          "positionSide",
          "leverage"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-position-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-position-list-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type           | Required | Restrictions | Description                                                                                                                                             |
| -------------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code               | string         | true     | none         | none                                                                                                                                                    |
| » data               | [object]       | true     | none         | none                                                                                                                                                    |
| »» id                | string         | true     | none         | Position ID                                                                                                                                             |
| »» symbol            | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                      |
| »» crossMode         | boolean        | true     | none         | Whether it is cross margin.                                                                                                                             |
| »» delevPercentage   | number         | true     | none         | ADL ranking percentile                                                                                                                                  |
| »» openingTimestamp  | integer(int64) | true     | none         | First opening time                                                                                                                                      |
| »» currentTimestamp  | integer(int64) | true     | none         | Current timestamp                                                                                                                                       |
| »» currentQty        | integer        | true     | none         | Current postion quantity                                                                                                                                |
| »» currentCost       | number         | true     | none         | Current postion value                                                                                                                                   |
| »» currentComm       | number         | true     | none         | Current commission                                                                                                                                      |
| »» unrealisedCost    | number         | true     | none         | Unrealised value                                                                                                                                        |
| »» realisedGrossCost | number         | true     | none         | Accumulated realised gross profit value                                                                                                                 |
| »» realisedCost      | number         | true     | none         | Current realised position value                                                                                                                         |
| »» isOpen            | boolean        | true     | none         | Opened position or not                                                                                                                                  |
| »» markPrice         | number         | true     | none         | Mark price                                                                                                                                              |
| »» markValue         | number         | true     | none         | Mark Value                                                                                                                                              |
| »» posCost           | number         | true     | none         | Position value                                                                                                                                          |
| »» posInit           | number         | true     | none         | Inital margin Cross = opening value/cross leverage; isolated = accumulation of initial margin for each transaction                                      |
| »» posMargin         | number         | true     | none         | Bankruptcy cost Cross = mark value \* imr; Isolated = position margin (accumulation of initial margin, additional margin, generated funding fees, etc.) |
| »» realisedGrossPnl  | number         | true     | none         | Accumulated realised gross profit value                                                                                                                 |
| »» realisedPnl       | number         | true     | none         | Realised profit and loss                                                                                                                                |
| »» unrealisedPnl     | number         | true     | none         | Unrealised profit and loss                                                                                                                              |
| »» unrealisedPnlPcnt | number         | true     | none         | Profit-loss ratio of the position                                                                                                                       |
| »» unrealisedRoePcnt | number         | true     | none         | Rate of return on investment                                                                                                                            |
| »» avgEntryPrice     | number         | true     | none         | Average entry price                                                                                                                                     |
| »» liquidationPrice  | number         | true     | none         | Liquidation price For Cross Margin, you can refer to the liquidationPrice, and the liquidation is based on the risk rate.                               |
| »» bankruptPrice     | number         | true     | none         | Bankruptcy price For Cross Margin, you can refer to the bankruptPrice, and the liquidation is based on the risk rate.                                   |
| »» settleCurrency    | string         | true     | none         | Currency used to clear and settle the trades                                                                                                            |
| »» isInverse         | boolean        | true     | none         | Reverse contract or not                                                                                                                                 |
| »» marginMode        | string         | true     | none         | Margin Mode: CROSS，ISOLATED                                                                                                                            |
| »» positionSide      | string         | true     | none         | Position Side                                                                                                                                           |
| »» leverage          | number         | true     | none         | Leverage                                                                                                                                                |
| »» autoDeposit       | boolean        | false    | none         | Auto deposit margin or not **Only applicable to Isolated Margin**                                                                                       |
| »» maintMarginReq    | number         | false    | none         | Maintenance margin requirement                                                                                                                          |
| »» riskLimit         | number         | false    | none         | Risk limit **Only applicable to Isolated Margin**                                                                                                       |
| »» realLeverage      | number         | false    | none         | Leverage of the order **Only applicable to Isolated Margin**                                                                                            |
| »» posCross          | number         | false    | none         | added margin **Only applicable to Isolated Margin**                                                                                                     |
| »» posCrossMargin    | number         | false    | none         | Additional margin calls (automatic, manual, adjusted risk limits) **Only applicable to Isolated Margin**                                                |
| »» posComm           | number         | false    | none         | Bankruptcy cost **Only applicable to Isolated Margin**                                                                                                  |
| »» posCommCommon     | number         | false    | none         | Part of bankruptcy cost (positioning, add margin) **Only applicable to Isolated Margin**                                                                |
| »» posLoss           | number         | false    | none         | Funding fees paid out **Only applicable to Isolated Margin**                                                                                            |
| »» posFunding        | number         | false    | none         | The current remaining unsettled funding fee for the position **Only applicable to Isolated Margin**                                                     |
| »» posMaint          | number         | false    | none         | Maintenance margin                                                                                                                                      |
| »» maintMargin       | number         | false    | none         | Position margin **Only applicable to Isolated Margin**                                                                                                  |
| »» maintainMargin    | number         | false    | none         | Maintenance margin rate **Only applicable to Isolated Margin**                                                                                          |

#### Enumerated Values

| Property     | Value    |
| ------------ | -------- |
| marginMode   | CROSS    |
| marginMode   | ISOLATED |
| positionSide | BOTH     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Positions History

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/history-positions", {
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

r = requests.get('/api/v1/history-positions', headers = headers)

print(r.json())

```

`GET /api/v1/history-positions`

This interface can query position history information records.

<h3 id="get-positions-history-parameters">Parameters</h3>

| Name   | In    | Type           | Required | Description                                                                                                        |
| ------ | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string         | false    | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| from   | query | integer(int64) | false    | Closing start time(ms)                                                                                             |
| to     | query | integer(int64) | false    | Closing end time(ms)                                                                                               |
| limit  | query | integer        | false    | Number of requests per page, max 200, default 10                                                                   |
| pageId | query | integer        | false    | Current page number, default 1                                                                                     |

#### Detailed descriptions

**from**: Closing start time(ms)

**to**: Closing end time(ms)

**limit**: Number of requests per page, max 200, default 10

**pageId**: Current page number, default 1

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
          "description": "Current page number\n"
        },
        "pageSize": {
          "type": "integer",
          "description": "Number of results per page\n"
        },
        "totalNum": {
          "type": "integer",
          "description": "Total number of results\n"
        },
        "totalPage": {
          "type": "integer",
          "description": "Total number of pages\n"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "closeId": {
                "type": "string",
                "description": "Close ID\n"
              },
              "userId": {
                "type": "string",
                "description": "User ID"
              },
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "settleCurrency": {
                "type": "string",
                "description": "Currency used to settle trades\n"
              },
              "leverage": {
                "type": "string",
                "description": "Leverage applied to the order\n"
              },
              "type": {
                "type": "string",
                "description": "Type of closure\n"
              },
              "pnl": {
                "type": "string",
                "description": "Net profit and loss (after deducting fees and funding costs)\n"
              },
              "realisedGrossCost": {
                "type": "string",
                "description": "Accumulated realised gross profit value\n"
              },
              "withdrawPnl": {
                "type": "string",
                "description": "Accumulated realised profit withdrawn from the position\n"
              },
              "tradeFee": {
                "type": "string",
                "description": "Accumulated trading fees\n"
              },
              "fundingFee": {
                "type": "string",
                "description": "Accumulated funding fees\n"
              },
              "openTime": {
                "type": "integer",
                "description": "Time when the position was opened\n",
                "format": "int64"
              },
              "closeTime": {
                "type": "integer",
                "description": "Time when the position was closed (default sorted in descending order)\n",
                "format": "int64"
              },
              "openPrice": {
                "type": "string",
                "description": "Opening price of the position\n"
              },
              "closePrice": {
                "type": "string",
                "description": "Closing price of the position\n"
              },
              "marginMode": {
                "type": "string",
                "description": "Margin Mode: CROSS，ISOLATED",
                "enum": ["CROSS", "ISOLATED"],
                "x-api-enum": [
                  {
                    "value": "CROSS",
                    "name": "CROSS",
                    "description": "cross margin"
                  },
                  {
                    "value": "ISOLATED",
                    "name": "ISOLATED",
                    "description": "isolated margin"
                  }
                ]
              },
              "realisedGrossCostNew": {
                "type": "string"
              },
              "tax": {
                "type": "string",
                "description": "Tax"
              },
              "roe": {
                "type": "string"
              },
              "liquidAmount": {
                "type": "string"
              },
              "liquidPrice": {
                "type": "string"
              },
              "side": {
                "type": "string",
                "description": "Position side"
              }
            },
            "required": [
              "closeId",
              "userId",
              "symbol",
              "settleCurrency",
              "leverage",
              "type",
              "pnl",
              "realisedGrossCost",
              "withdrawPnl",
              "tradeFee",
              "fundingFee",
              "openTime",
              "closeTime",
              "openPrice",
              "closePrice",
              "marginMode",
              "realisedGrossCostNew",
              "tax",
              "liquidAmount",
              "liquidPrice",
              "side"
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

<h3 id="get-positions-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-positions-history-responseschema">Response Schema</h3>

Status Code **200**

| Name                     | Type           | Required | Restrictions | Description                                                                                                        |
| ------------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code                   | string         | true     | none         | none                                                                                                               |
| » data                   | object         | true     | none         | none                                                                                                               |
| »» currentPage           | integer        | true     | none         | Current page number                                                                                                |
| »» pageSize              | integer        | true     | none         | Number of results per page                                                                                         |
| »» totalNum              | integer        | true     | none         | Total number of results                                                                                            |
| »» totalPage             | integer        | true     | none         | Total number of pages                                                                                              |
| »» items                 | [object]       | true     | none         | none                                                                                                               |
| »»» closeId              | string         | true     | none         | Close ID                                                                                                           |
| »»» userId               | string         | true     | none         | User ID                                                                                                            |
| »»» symbol               | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »»» settleCurrency       | string         | true     | none         | Currency used to settle trades                                                                                     |
| »»» leverage             | string         | true     | none         | Leverage applied to the order                                                                                      |
| »»» type                 | string         | true     | none         | Type of closure                                                                                                    |
| »»» pnl                  | string         | true     | none         | Net profit and loss (after deducting fees and funding costs)                                                       |
| »»» realisedGrossCost    | string         | true     | none         | Accumulated realised gross profit value                                                                            |
| »»» withdrawPnl          | string         | true     | none         | Accumulated realised profit withdrawn from the position                                                            |
| »»» tradeFee             | string         | true     | none         | Accumulated trading fees                                                                                           |
| »»» fundingFee           | string         | true     | none         | Accumulated funding fees                                                                                           |
| »»» openTime             | integer(int64) | true     | none         | Time when the position was opened                                                                                  |
| »»» closeTime            | integer(int64) | true     | none         | Time when the position was closed (default sorted in descending order)                                             |
| »»» openPrice            | string         | true     | none         | Opening price of the position                                                                                      |
| »»» closePrice           | string         | true     | none         | Closing price of the position                                                                                      |
| »»» marginMode           | string         | true     | none         | Margin Mode: CROSS，ISOLATED                                                                                       |
| »»» realisedGrossCostNew | string         | true     | none         | none                                                                                                               |
| »»» tax                  | string         | true     | none         | Tax                                                                                                                |
| »»» roe                  | string         | false    | none         | none                                                                                                               |
| »»» liquidAmount         | string         | true     | none         | none                                                                                                               |
| »»» liquidPrice          | string         | true     | none         | none                                                                                                               |
| »»» side                 | string         | true     | none         | Position side                                                                                                      |

#### Enumerated Values

| Property   | Value    |
| ---------- | -------- |
| marginMode | CROSS    |
| marginMode | ISOLATED |

<aside class="success">
This operation does not require authentication
</aside>

## Get Max Withdraw Margin

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/margin/maxWithdrawMargin?symbol=type,string", {
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

r = requests.get('/api/v1/margin/maxWithdrawMargin', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/margin/maxWithdrawMargin`

This interface can query the maximum amount of margin that the current position
supports withdrawal.

<h3 id="get-max-withdraw-margin-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
      "type": "string",
      "description": "The size of the position that can be deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins\n"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-max-withdraw-margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-max-withdraw-margin-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description                                                                                                                                                             |
| ------ | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code | string | true     | none         | none                                                                                                                                                                    |
| » data | string | true     | none         | The size of the position that can be deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins |

<aside class="success">
This operation does not require authentication
</aside>

## Get Cross Margin Leverage

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v2/getCrossUserLeverage?symbol=type,string", {
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

r = requests.get('/api/v2/getCrossUserLeverage', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v2/getCrossUserLeverage`

This interface can query the current symbol’s cross-margin leverage multiple.

<h3 id="get-cross-margin-leverage-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "leverage": {
          "type": "string",
          "description": "Leverage multiple"
        }
      },
      "required": ["symbol", "leverage"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-cross-margin-leverage-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-cross-margin-leverage-responseschema">Response Schema</h3>

Status Code **200**

| Name        | Type   | Required | Restrictions | Description                                                                                                        |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code      | string | true     | none         | none                                                                                                               |
| » data      | object | true     | none         | none                                                                                                               |
| »» symbol   | string | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» leverage | string | true     | none         | Leverage multiple                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Cross Margin Leverage

<a id="opId010"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "leverage": {
      "type": "string",
      "description": "Leverage multiple"
    }
  },
  "required": [
    "symbol",
    "leverage"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v2/changeCrossUserLeverage',
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

r = requests.post('/api/v2/changeCrossUserLeverage', headers = headers)

print(r.json())

```

`POST /api/v2/changeCrossUserLeverage`

This interface can modify the current symbol’s cross-margin leverage multiple.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "leverage": {
      "type": "string",
      "description": "Leverage multiple"
    }
  },
  "required": ["symbol", "leverage"]
}
```

<h3 id="modify-cross-margin-leverage-parameters">Parameters</h3>

| Name       | In   | Type   | Required | Description                                                                                                        |
| ---------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| body       | body | object | false    | none                                                                                                               |
| » symbol   | body | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| » leverage | body | string | true     | Leverage multiple                                                                                                  |

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
      "type": "boolean"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-cross-margin-leverage-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-cross-margin-leverage-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type    | Required | Restrictions | Description |
| ------ | ------- | -------- | ------------ | ----------- |
| » code | string  | true     | none         | none        |
| » data | boolean | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Add Isolated Margin

<a id="opId011"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "margin": {
      "type": "number",
      "description": "Margin amount (min. margin amount≥0.00001667XBT）"
    },
    "bizNo": {
      "type": "string",
      "description": "A unique ID generated by the user, to ensure the operation is processed by the system only once, The maximum length cannot exceed 36"
    }
  },
  "required": [
    "symbol",
    "margin",
    "bizNo"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/position/margin/deposit-margin',
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

r = requests.post('/api/v1/position/margin/deposit-margin', headers = headers)

print(r.json())

```

`POST /api/v1/position/margin/deposit-margin`

Add Isolated Margin Manually.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "margin": {
      "type": "number",
      "description": "Margin amount (min. margin amount≥0.00001667XBT）"
    },
    "bizNo": {
      "type": "string",
      "description": "A unique ID generated by the user, to ensure the operation is processed by the system only once, The maximum length cannot exceed 36"
    }
  },
  "required": ["symbol", "margin", "bizNo"]
}
```

<h3 id="add-isolated-margin-parameters">Parameters</h3>

| Name     | In   | Type   | Required | Description                                                                                                                          |
| -------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| body     | body | object | false    | none                                                                                                                                 |
| » symbol | body | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                   |
| » margin | body | number | true     | Margin amount (min. margin amount≥0.00001667XBT）                                                                                    |
| » bizNo  | body | string | true     | A unique ID generated by the user, to ensure the operation is processed by the system only once, The maximum length cannot exceed 36 |

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
          "description": "Position ID"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "autoDeposit": {
          "type": "boolean",
          "description": "Auto deposit margin or not"
        },
        "maintMarginReq": {
          "type": "number",
          "description": "Maintenance margin requirement\n\n"
        },
        "riskLimit": {
          "type": "integer",
          "description": "Risk limit\n"
        },
        "realLeverage": {
          "type": "number",
          "description": "Leverage o the order"
        },
        "crossMode": {
          "type": "boolean",
          "description": "Cross mode or not"
        },
        "delevPercentage": {
          "type": "number",
          "description": "ADL ranking percentile"
        },
        "openingTimestamp": {
          "type": "integer",
          "description": "Open time",
          "format": "int64"
        },
        "currentTimestamp": {
          "type": "integer",
          "format": "int64",
          "description": "Current timestamp\n"
        },
        "currentQty": {
          "type": "integer",
          "description": "Current postion quantity"
        },
        "currentCost": {
          "type": "number",
          "description": "Current postion value"
        },
        "currentComm": {
          "type": "number",
          "description": "Current commission"
        },
        "unrealisedCost": {
          "type": "number",
          "description": "Unrealised value"
        },
        "realisedGrossCost": {
          "type": "number",
          "description": "Accumulated realised gross profit value"
        },
        "realisedCost": {
          "type": "number",
          "description": "Current realised position value"
        },
        "isOpen": {
          "type": "boolean",
          "description": "Opened position or not"
        },
        "markPrice": {
          "type": "number",
          "description": "Mark price"
        },
        "markValue": {
          "type": "number",
          "description": "Mark value\n"
        },
        "posCost": {
          "type": "number",
          "description": "Position value"
        },
        "posCross": {
          "type": "number",
          "description": "added margin"
        },
        "posInit": {
          "type": "number",
          "description": "Leverage margin"
        },
        "posComm": {
          "type": "number",
          "description": "Bankruptcy cost"
        },
        "posLoss": {
          "type": "number",
          "description": "Funding fees paid out"
        },
        "posMargin": {
          "type": "number",
          "description": "Position margin"
        },
        "posMaint": {
          "type": "number",
          "description": "Maintenance margin"
        },
        "maintMargin": {
          "type": "number",
          "description": "Position margin"
        },
        "realisedGrossPnl": {
          "type": "number",
          "description": "Accumulated realised gross profit value"
        },
        "realisedPnl": {
          "type": "number",
          "description": "Realised profit and loss"
        },
        "unrealisedPnl": {
          "type": "number",
          "description": "Unrealised profit and loss"
        },
        "unrealisedPnlPcnt": {
          "type": "number",
          "description": "Profit-loss ratio of the position"
        },
        "unrealisedRoePcnt": {
          "type": "number",
          "description": "Rate of return on investment"
        },
        "avgEntryPrice": {
          "type": "number",
          "description": "Average entry price"
        },
        "liquidationPrice": {
          "type": "number",
          "description": "Liquidation price"
        },
        "bankruptPrice": {
          "type": "number",
          "description": "Bankruptcy price"
        },
        "userId": {
          "type": "integer",
          "description": "userId"
        },
        "settleCurrency": {
          "type": "string",
          "description": "Currency used to clear and settle the trades"
        }
      },
      "required": [
        "id",
        "symbol",
        "autoDeposit",
        "maintMarginReq",
        "riskLimit",
        "realLeverage",
        "crossMode",
        "delevPercentage",
        "openingTimestamp",
        "currentTimestamp",
        "currentQty",
        "currentCost",
        "currentComm",
        "unrealisedCost",
        "realisedGrossCost",
        "realisedCost",
        "isOpen",
        "markPrice",
        "markValue",
        "posCost",
        "posCross",
        "posInit",
        "posComm",
        "posLoss",
        "posMargin",
        "posMaint",
        "maintMargin",
        "realisedGrossPnl",
        "realisedPnl",
        "unrealisedPnl",
        "unrealisedPnlPcnt",
        "unrealisedRoePcnt",
        "avgEntryPrice",
        "liquidationPrice",
        "bankruptPrice",
        "userId",
        "settleCurrency"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-isolated-margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-isolated-margin-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type           | Required | Restrictions | Description                                                                                                        |
| -------------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code               | string         | true     | none         | none                                                                                                               |
| » data               | object         | true     | none         | none                                                                                                               |
| »» id                | string         | true     | none         | Position ID                                                                                                        |
| »» symbol            | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» autoDeposit       | boolean        | true     | none         | Auto deposit margin or not                                                                                         |
| »» maintMarginReq    | number         | true     | none         | Maintenance margin requirement                                                                                     |
| »» riskLimit         | integer        | true     | none         | Risk limit                                                                                                         |
| »» realLeverage      | number         | true     | none         | Leverage o the order                                                                                               |
| »» crossMode         | boolean        | true     | none         | Cross mode or not                                                                                                  |
| »» delevPercentage   | number         | true     | none         | ADL ranking percentile                                                                                             |
| »» openingTimestamp  | integer(int64) | true     | none         | Open time                                                                                                          |
| »» currentTimestamp  | integer(int64) | true     | none         | Current timestamp                                                                                                  |
| »» currentQty        | integer        | true     | none         | Current postion quantity                                                                                           |
| »» currentCost       | number         | true     | none         | Current postion value                                                                                              |
| »» currentComm       | number         | true     | none         | Current commission                                                                                                 |
| »» unrealisedCost    | number         | true     | none         | Unrealised value                                                                                                   |
| »» realisedGrossCost | number         | true     | none         | Accumulated realised gross profit value                                                                            |
| »» realisedCost      | number         | true     | none         | Current realised position value                                                                                    |
| »» isOpen            | boolean        | true     | none         | Opened position or not                                                                                             |
| »» markPrice         | number         | true     | none         | Mark price                                                                                                         |
| »» markValue         | number         | true     | none         | Mark value                                                                                                         |
| »» posCost           | number         | true     | none         | Position value                                                                                                     |
| »» posCross          | number         | true     | none         | added margin                                                                                                       |
| »» posInit           | number         | true     | none         | Leverage margin                                                                                                    |
| »» posComm           | number         | true     | none         | Bankruptcy cost                                                                                                    |
| »» posLoss           | number         | true     | none         | Funding fees paid out                                                                                              |
| »» posMargin         | number         | true     | none         | Position margin                                                                                                    |
| »» posMaint          | number         | true     | none         | Maintenance margin                                                                                                 |
| »» maintMargin       | number         | true     | none         | Position margin                                                                                                    |
| »» realisedGrossPnl  | number         | true     | none         | Accumulated realised gross profit value                                                                            |
| »» realisedPnl       | number         | true     | none         | Realised profit and loss                                                                                           |
| »» unrealisedPnl     | number         | true     | none         | Unrealised profit and loss                                                                                         |
| »» unrealisedPnlPcnt | number         | true     | none         | Profit-loss ratio of the position                                                                                  |
| »» unrealisedRoePcnt | number         | true     | none         | Rate of return on investment                                                                                       |
| »» avgEntryPrice     | number         | true     | none         | Average entry price                                                                                                |
| »» liquidationPrice  | number         | true     | none         | Liquidation price                                                                                                  |
| »» bankruptPrice     | number         | true     | none         | Bankruptcy price                                                                                                   |
| »» userId            | integer        | true     | none         | userId                                                                                                             |
| »» settleCurrency    | string         | true     | none         | Currency used to clear and settle the trades                                                                       |

<aside class="success">
This operation does not require authentication
</aside>

## Remove Isolated Margin

<a id="opId012"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "withdrawAmount": {
      "type": "string",
      "description": "The size of the position that can be deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins\n"
    }
  },
  "required": [
    "symbol",
    "withdrawAmount"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/margin/withdrawMargin',
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

r = requests.post('/api/v1/margin/withdrawMargin', headers = headers)

print(r.json())

```

`POST /api/v1/margin/withdrawMargin`

Remove Isolated Margin Manually.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "withdrawAmount": {
      "type": "string",
      "description": "The size of the position that can be deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins\n"
    }
  },
  "required": ["symbol", "withdrawAmount"]
}
```

<h3 id="remove-isolated-margin-parameters">Parameters</h3>

| Name             | In   | Type   | Required | Description                                                                                                                                                             |
| ---------------- | ---- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body             | body | object | false    | none                                                                                                                                                                    |
| » symbol         | body | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                      |
| » withdrawAmount | body | string | true     | The size of the position that can be deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins |

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
      "type": "string",
      "description": "The size of the position deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins\n"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="remove-isolated-margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="remove-isolated-margin-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description                                                                                                                                                 |
| ------ | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code | string | true     | none         | none                                                                                                                                                        |
| » data | string | true     | none         | The size of the position deposited. If it is USDT-margin, it represents the amount of USDT. If it is coin-margin, this value represents the number of coins |

<aside class="success">
This operation does not require authentication
</aside>

## Get Cross Margin Risk Limit

<a id="opId013"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v2/batchGetCrossOrderLimit?symbol=XBTUSDTM,XBTUSDTM%2CETHUSDTM", {
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

r = requests.get('/api/v2/batchGetCrossOrderLimit', params={
  'symbol': [
  "XBTUSDTM",
  "XBTUSDTM,ETHUSDTM"
]
}, headers = headers)

print(r.json())

```

`GET /api/v2/batchGetCrossOrderLimit`

Batch get cross margin risk limit.

<h3 id="get-cross-margin-risk-limit-parameters">Parameters</h3>

| Name        | In    | Type    | Required | Description                                                                                                                                                                          |
| ----------- | ----- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| symbol      | query | string  | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220), (You may add up to 50 symbols. Use a halfwidth comma to each IP) |
| totalMargin | query | string  | false    | The position opening amount, in the contract's settlement currency.                                                                                                                  |
| leverage    | query | integer | false    | Calculates the max position size at the specified leverage.                                                                                                                          |

#### Detailed descriptions

**totalMargin**: The position opening amount, in the contract's settlement
currency. Defaults to 10,000 in margin currency for max position calculation.
For USDT/USDC, it's 10,000 USD; for others, it's 10,000 divided by the token's
USDT price.

**leverage**: Calculates the max position size at the specified leverage.
Defaults to the symbol’s max cross leverage.

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
            "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)"
          },
          "maxOpenSize": {
            "type": "integer",
            "description": "Maximum amount of open position(Unit is **lots**)\n"
          },
          "maxOpenValue": {
            "type": "string",
            "description": "Maximum value of open position(Unit is **quoteCcy**)\n"
          },
          "totalMargin": {
            "type": "string",
            "description": "Margin amount used for max position calculation."
          },
          "price": {
            "type": "string",
            "description": "Price used for max position calculation. Defaults to latest transaction price"
          },
          "leverage": {
            "type": "string",
            "description": "Leverage used for max position calculation."
          },
          "mmr": {
            "type": "string",
            "description": "Maintenance Margin Rate"
          },
          "imr": {
            "type": "string",
            "description": "Initial Margin Rate"
          },
          "currency": {
            "type": "string",
            "description": "Margin Currency"
          }
        },
        "required": [
          "symbol",
          "maxOpenSize",
          "maxOpenValue",
          "totalMargin",
          "price",
          "leverage",
          "mmr",
          "imr",
          "currency"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-cross-margin-risk-limit-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-cross-margin-risk-limit-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type     | Required | Restrictions | Description                                                                                                        |
| --------------- | -------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code          | string   | true     | none         | none                                                                                                               |
| » data          | [object] | true     | none         | none                                                                                                               |
| »» symbol       | string   | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» maxOpenSize  | integer  | true     | none         | Maximum amount of open position(Unit is **lots**)                                                                  |
| »» maxOpenValue | string   | true     | none         | Maximum value of open position(Unit is **quoteCcy**)                                                               |
| »» totalMargin  | string   | true     | none         | Margin amount used for max position calculation.                                                                   |
| »» price        | string   | true     | none         | Price used for max position calculation. Defaults to latest transaction price                                      |
| »» leverage     | string   | true     | none         | Leverage used for max position calculation.                                                                        |
| »» mmr          | string   | true     | none         | Maintenance Margin Rate                                                                                            |
| »» imr          | string   | true     | none         | Initial Margin Rate                                                                                                |
| »» currency     | string   | true     | none         | Margin Currency                                                                                                    |

<aside class="success">
This operation does not require authentication
</aside>

## Get Isolated Margin Risk Limit

<a id="opId014"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/contracts/risk-limit/{symbol}", {
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

r = requests.get('/api/v1/contracts/risk-limit/{symbol}', headers = headers)

print(r.json())

```

`GET /api/v1/contracts/risk-limit/{symbol}`

This interface can be used to obtain information about risk limit level of a
specific contract (only valid for Isolated Margin).

<h3 id="get-isolated-margin-risk-limit-parameters">Parameters</h3>

| Name   | In   | Type   | Required | Description                                                                                                        |
| ------ | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | path | string | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
            "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
          },
          "level": {
            "type": "integer",
            "description": "Level\n"
          },
          "maxRiskLimit": {
            "type": "integer",
            "description": "Upper limit USDT (included)\n"
          },
          "minRiskLimit": {
            "type": "integer",
            "description": "Lower limit USDT\n"
          },
          "maxLeverage": {
            "type": "integer",
            "description": "Max. leverage\n"
          },
          "initialMargin": {
            "type": "number",
            "description": "Initial margin rate\n"
          },
          "maintainMargin": {
            "type": "number",
            "description": "Maintenance margin rate\n"
          }
        },
        "required": [
          "symbol",
          "level",
          "maxRiskLimit",
          "minRiskLimit",
          "maxLeverage",
          "initialMargin",
          "maintainMargin"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-isolated-margin-risk-limit-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-isolated-margin-risk-limit-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type     | Required | Restrictions | Description                                                                                                        |
| ----------------- | -------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code            | string   | true     | none         | none                                                                                                               |
| » data            | [object] | true     | none         | none                                                                                                               |
| »» symbol         | string   | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» level          | integer  | true     | none         | Level                                                                                                              |
| »» maxRiskLimit   | integer  | true     | none         | Upper limit USDT (included)                                                                                        |
| »» minRiskLimit   | integer  | true     | none         | Lower limit USDT                                                                                                   |
| »» maxLeverage    | integer  | true     | none         | Max. leverage                                                                                                      |
| »» initialMargin  | number   | true     | none         | Initial margin rate                                                                                                |
| »» maintainMargin | number   | true     | none         | Maintenance margin rate                                                                                            |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Isolated Margin Risk Limit

<a id="opId015"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "level": {
      "type": "integer",
      "description": "Level"
    }
  },
  "required": [
    "symbol",
    "level"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/position/risk-limit-level/change',
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

r = requests.post('/api/v1/position/risk-limit-level/change', headers = headers)

print(r.json())

```

`POST /api/v1/position/risk-limit-level/change`

This interface can be used to obtain information about risk limit level of a
specific contract (only valid for Isolated Margin).

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
    },
    "level": {
      "type": "integer",
      "description": "Level"
    }
  },
  "required": ["symbol", "level"]
}
```

<h3 id="modify-isolated-margin-risk-limit-parameters">Parameters</h3>

| Name     | In   | Type    | Required | Description                                                                                                        |
| -------- | ---- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| body     | body | object  | false    | none                                                                                                               |
| » symbol | body | string  | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| » level  | body | integer | true     | Level                                                                                                              |

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
      "type": "boolean",
      "description": "Adjusting the level will result in the cancellation of any open orders. The response will indicate only whether the adjustment request was successfully submitted.\n"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-isolated-margin-risk-limit-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-isolated-margin-risk-limit-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type    | Required | Restrictions | Description                                                                                                                                                        |
| ------ | ------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| » code | string  | true     | none         | none                                                                                                                                                               |
| » data | boolean | true     | none         | Adjusting the level will result in the cancellation of any open orders. The response will indicate only whether the adjustment request was successfully submitted. |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Isolated Margin Auto-Deposit Status

<a id="opId016"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract\n",
      "example": [
        "XBTUSDTM",
        "ETHUSDTM"
      ]
    },
    "status": {
      "type": "boolean",
      "description": "Status"
    }
  },
  "required": [
    "symbol",
    "status"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/position/margin/auto-deposit-status',
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

r = requests.post('/api/v1/position/margin/auto-deposit-status', headers = headers)

print(r.json())

```

`POST /api/v1/position/margin/auto-deposit-status`

This endpoint is only applicable to isolated margin and is no longer
recommended. It is recommended to use cross margin instead.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "type": "string",
      "description": "Symbol of the contract\n",
      "example": ["XBTUSDTM", "ETHUSDTM"]
    },
    "status": {
      "type": "boolean",
      "description": "Status"
    }
  },
  "required": ["symbol", "status"]
}
```

<h3 id="modify-isolated-margin-auto-deposit-status-parameters">Parameters</h3>

| Name     | In   | Type    | Required | Description            |
| -------- | ---- | ------- | -------- | ---------------------- |
| body     | body | object  | false    | none                   |
| » symbol | body | string  | true     | Symbol of the contract |
| » status | body | boolean | true     | Status                 |

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
      "type": "boolean"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-isolated-margin-auto-deposit-status-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-isolated-margin-auto-deposit-status-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type    | Required | Restrictions | Description |
| ------ | ------- | -------- | ------------ | ----------- |
| » code | string  | true     | none         | none        |
| » data | boolean | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
