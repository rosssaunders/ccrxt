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

market

<h1 id="futures-default">Default</h1>

## Get Symbol

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/contracts/{symbol}", {
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

r = requests.get('/api/v1/contracts/{symbol}', headers = headers)

print(r.json())

```

`GET /api/v1/contracts/{symbol}`

Get information of specified contracts that can be traded. This API will return
a list of tradable contracts, including some key parameters of the contract such
as the symbol name, tick size, mark price, etc.

<h3 id="get-symbol-parameters">Parameters</h3>

| Name   | In   | Type   | Required | Description                            |
| ------ | ---- | ------ | -------- | -------------------------------------- |
| symbol | path | string | true     | Path Parameter. Symbol of the contract |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "200000 is for success, other is error"
    },
    "data": {
      "type": "object",
      "properties": {
        "symbol": {
          "type": "string",
          "description": "Symbol",
          "example": ["XBTUSDTM", "XBTUSDCM", "XBTUSDM"]
        },
        "rootSymbol": {
          "type": "string",
          "description": "Contract group",
          "example": ["USDT", "USDC", "XBT"]
        },
        "type": {
          "type": "string",
          "description": "Type of contract",
          "enum": ["FFWCSX", "FFICSX"],
          "x-api-enum": [
            {
              "value": "FFWCSX",
              "name": "FFWCSX",
              "description": "Standardized swap contracts, standard financial futures on swaps, expiration swap funding rates"
            },
            {
              "value": "FFICSX",
              "name": "FFICSX",
              "description": "Futures Contract"
            }
          ]
        },
        "firstOpenDate": {
          "type": "integer",
          "description": "First Open Date (milliseconds)",
          "format": "int64"
        },
        "expireDate": {
          "type": "integer",
          "description": "Expiration date (milliseconds) Null means it will never expire",
          "format": "int64"
        },
        "settleDate": {
          "type": "integer",
          "description": "Settlement date (milliseconds) Null indicates that automatic settlement is not supported",
          "format": "int64"
        },
        "baseCurrency": {
          "type": "string",
          "description": "Base currency"
        },
        "quoteCurrency": {
          "type": "string",
          "description": "Quote currency"
        },
        "settleCurrency": {
          "type": "string",
          "description": "Currency used to clear and settle the trades"
        },
        "maxOrderQty": {
          "type": "integer",
          "description": "Maximum order quantity"
        },
        "maxPrice": {
          "type": "number",
          "description": "Maximum order price"
        },
        "lotSize": {
          "type": "integer",
          "description": "Minimum lot size"
        },
        "tickSize": {
          "type": "number",
          "description": "Minimum price changes"
        },
        "indexPriceTickSize": {
          "type": "number",
          "description": "Index price of tick size"
        },
        "multiplier": {
          "type": "number",
          "description": "The basic unit of the contract API is lots. For the number of coins in each lot, please refer to the param multiplier. For example, for XBTUSDTM, multiplier=0.001, which corresponds to the value of each XBTUSDTM contract being 0.001 BTC. There is also a special case. All coin-swap contracts, such as each XBTUSDM contract, correspond to 1 USD."
        },
        "initialMargin": {
          "type": "number",
          "description": "Initial margin requirement"
        },
        "maintainMargin": {
          "type": "number",
          "description": "Maintenance margin requirement"
        },
        "maxRiskLimit": {
          "type": "integer",
          "description": "Maximum risk limit (unit: XBT)"
        },
        "minRiskLimit": {
          "type": "integer",
          "description": "Minimum risk limit (unit: XBT)"
        },
        "riskStep": {
          "type": "integer",
          "description": "Risk limit increment value (unit: XBT)"
        },
        "makerFeeRate": {
          "type": "number",
          "description": "Maker fee rate"
        },
        "takerFeeRate": {
          "type": "number",
          "description": "Taker fee rate"
        },
        "takerFixFee": {
          "type": "number",
          "description": "Deprecated param"
        },
        "makerFixFee": {
          "type": "number",
          "description": "Deprecated param"
        },
        "settlementFee": {
          "type": "number",
          "description": "Settlement fee"
        },
        "isDeleverage": {
          "type": "boolean",
          "description": "Enabled ADL or not"
        },
        "isQuanto": {
          "type": "boolean",
          "description": "Deprecated param"
        },
        "isInverse": {
          "type": "boolean",
          "description": "Whether it is a reverse contract"
        },
        "markMethod": {
          "type": "string",
          "description": "Marking method",
          "enum": ["FairPrice"],
          "x-api-enum": [
            {
              "value": "FairPrice",
              "name": "FairPrice",
              "description": "FairPrice"
            }
          ]
        },
        "fairMethod": {
          "type": "string",
          "description": "Fair price marking method; the Futures contract is null",
          "enum": ["FundingRate"],
          "x-api-enum": [
            {
              "value": "FundingRate",
              "name": "FundingRate",
              "description": "FundingRate"
            }
          ]
        },
        "fundingBaseSymbol": {
          "type": "string",
          "description": "Ticker symbol of the base currency"
        },
        "fundingQuoteSymbol": {
          "type": "string",
          "description": "Ticker symbol of the quote currency"
        },
        "fundingRateSymbol": {
          "type": "string",
          "description": "Funding rate symbol"
        },
        "indexSymbol": {
          "type": "string",
          "description": "Index symbol"
        },
        "settlementSymbol": {
          "type": "string",
          "description": "Settlement symbol"
        },
        "status": {
          "type": "string",
          "description": "Contract status",
          "enum": [
            "Init",
            "Open",
            "BeingSettled",
            "Settled",
            "Paused",
            "Closed",
            "CancelOnly"
          ],
          "x-api-enum": [
            {
              "value": "Init",
              "name": "Init",
              "description": "Initial"
            },
            {
              "value": "Open",
              "name": "Open",
              "description": "Online"
            },
            {
              "value": "BeingSettled",
              "name": "BeingSettled",
              "description": "Settling"
            },
            {
              "value": "Settled",
              "name": "Settled",
              "description": "Settled"
            },
            {
              "value": "Paused",
              "name": "Paused",
              "description": "Suspended"
            },
            {
              "value": "Closed",
              "name": "Closed",
              "description": "Offline"
            },
            {
              "value": "CancelOnly",
              "name": "CancelOnly",
              "description": "Orders can only be canceled"
            }
          ]
        },
        "fundingFeeRate": {
          "type": "number",
          "description": "Funding fee rate"
        },
        "predictedFundingFeeRate": {
          "type": "number",
          "description": "Predicted funding fee rate"
        },
        "fundingRateGranularity": {
          "type": "integer",
          "description": "Funding interval (milliseconds)"
        },
        "openInterest": {
          "type": "string",
          "description": "Open interest (unit: lots)"
        },
        "turnoverOf24h": {
          "type": "number",
          "description": "24-hour turnover"
        },
        "volumeOf24h": {
          "type": "number",
          "description": "24-hour volume"
        },
        "markPrice": {
          "type": "number",
          "description": "Mark price"
        },
        "indexPrice": {
          "type": "number",
          "description": "Index price"
        },
        "lastTradePrice": {
          "type": "number",
          "description": "Last trade price"
        },
        "nextFundingRateTime": {
          "type": "integer",
          "description": "Next funding rate time (milliseconds)"
        },
        "maxLeverage": {
          "type": "integer",
          "description": "Maximum leverage"
        },
        "sourceExchanges": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The contract index price source exchange"
        },
        "premiumsSymbol1M": {
          "type": "string",
          "description": "Premium index symbol (1 minute)"
        },
        "premiumsSymbol8H": {
          "type": "string",
          "description": "Premium index symbol (8 hours)"
        },
        "fundingBaseSymbol1M": {
          "type": "string",
          "description": "Base currency interest rate symbol (1 minute)"
        },
        "fundingQuoteSymbol1M": {
          "type": "string",
          "description": "Quote currency interest rate symbol (1 minute)"
        },
        "lowPrice": {
          "type": "number",
          "description": "24-hour lowest price"
        },
        "highPrice": {
          "type": "number",
          "description": "24-hour highest price"
        },
        "priceChgPct": {
          "type": "number",
          "description": "24-hour % price change "
        },
        "priceChg": {
          "type": "number",
          "description": "24-hour price change"
        },
        "k": {
          "type": "number"
        },
        "m": {
          "type": "number"
        },
        "f": {
          "type": "number"
        },
        "mmrLimit": {
          "type": "number"
        },
        "mmrLevConstant": {
          "type": "number"
        },
        "supportCross": {
          "type": "boolean",
          "description": "Whether support Cross Margin"
        },
        "buyLimit": {
          "type": "number",
          "description": "The current maximum buying price allowed"
        },
        "sellLimit": {
          "type": "number",
          "description": "The current minimum selling price allowed"
        }
      },
      "required": [
        "symbol",
        "rootSymbol",
        "type",
        "firstOpenDate",
        "expireDate",
        "settleDate",
        "baseCurrency",
        "quoteCurrency",
        "settleCurrency",
        "maxOrderQty",
        "maxPrice",
        "lotSize",
        "tickSize",
        "indexPriceTickSize",
        "multiplier",
        "initialMargin",
        "maintainMargin",
        "maxRiskLimit",
        "minRiskLimit",
        "riskStep",
        "makerFeeRate",
        "takerFeeRate",
        "takerFixFee",
        "makerFixFee",
        "settlementFee",
        "isDeleverage",
        "isQuanto",
        "isInverse",
        "markMethod",
        "fairMethod",
        "fundingBaseSymbol",
        "fundingQuoteSymbol",
        "fundingRateSymbol",
        "indexSymbol",
        "settlementSymbol",
        "status",
        "fundingFeeRate",
        "predictedFundingFeeRate",
        "fundingRateGranularity",
        "openInterest",
        "turnoverOf24h",
        "volumeOf24h",
        "markPrice",
        "indexPrice",
        "lastTradePrice",
        "nextFundingRateTime",
        "maxLeverage",
        "sourceExchanges",
        "premiumsSymbol1M",
        "premiumsSymbol8H",
        "fundingBaseSymbol1M",
        "fundingQuoteSymbol1M",
        "lowPrice",
        "highPrice",
        "priceChgPct",
        "priceChg",
        "k",
        "m",
        "f",
        "mmrLimit",
        "mmrLevConstant",
        "supportCross",
        "buyLimit",
        "sellLimit"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-symbol-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name                       | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                              |
| -------------------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code                     | string         | true     | none         | 200000 is for success, other is error                                                                                                                                                                                                                                                                                                                    |
| » data                     | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» symbol                  | string         | true     | none         | Symbol                                                                                                                                                                                                                                                                                                                                                   |
| »» rootSymbol              | string         | true     | none         | Contract group                                                                                                                                                                                                                                                                                                                                           |
| »» type                    | string         | true     | none         | Type of contract                                                                                                                                                                                                                                                                                                                                         |
| »» firstOpenDate           | integer(int64) | true     | none         | First Open Date (milliseconds)                                                                                                                                                                                                                                                                                                                           |
| »» expireDate              | integer(int64) | true     | none         | Expiration date (milliseconds) Null means it will never expire                                                                                                                                                                                                                                                                                           |
| »» settleDate              | integer(int64) | true     | none         | Settlement date (milliseconds) Null indicates that automatic settlement is not supported                                                                                                                                                                                                                                                                 |
| »» baseCurrency            | string         | true     | none         | Base currency                                                                                                                                                                                                                                                                                                                                            |
| »» quoteCurrency           | string         | true     | none         | Quote currency                                                                                                                                                                                                                                                                                                                                           |
| »» settleCurrency          | string         | true     | none         | Currency used to clear and settle the trades                                                                                                                                                                                                                                                                                                             |
| »» maxOrderQty             | integer        | true     | none         | Maximum order quantity                                                                                                                                                                                                                                                                                                                                   |
| »» maxPrice                | number         | true     | none         | Maximum order price                                                                                                                                                                                                                                                                                                                                      |
| »» lotSize                 | integer        | true     | none         | Minimum lot size                                                                                                                                                                                                                                                                                                                                         |
| »» tickSize                | number         | true     | none         | Minimum price changes                                                                                                                                                                                                                                                                                                                                    |
| »» indexPriceTickSize      | number         | true     | none         | Index price of tick size                                                                                                                                                                                                                                                                                                                                 |
| »» multiplier              | number         | true     | none         | The basic unit of the contract API is lots. For the number of coins in each lot, please refer to the param multiplier. For example, for XBTUSDTM, multiplier=0.001, which corresponds to the value of each XBTUSDTM contract being 0.001 BTC. There is also a special case. All coin-swap contracts, such as each XBTUSDM contract, correspond to 1 USD. |
| »» initialMargin           | number         | true     | none         | Initial margin requirement                                                                                                                                                                                                                                                                                                                               |
| »» maintainMargin          | number         | true     | none         | Maintenance margin requirement                                                                                                                                                                                                                                                                                                                           |
| »» maxRiskLimit            | integer        | true     | none         | Maximum risk limit (unit: XBT)                                                                                                                                                                                                                                                                                                                           |
| »» minRiskLimit            | integer        | true     | none         | Minimum risk limit (unit: XBT)                                                                                                                                                                                                                                                                                                                           |
| »» riskStep                | integer        | true     | none         | Risk limit increment value (unit: XBT)                                                                                                                                                                                                                                                                                                                   |
| »» makerFeeRate            | number         | true     | none         | Maker fee rate                                                                                                                                                                                                                                                                                                                                           |
| »» takerFeeRate            | number         | true     | none         | Taker fee rate                                                                                                                                                                                                                                                                                                                                           |
| »» takerFixFee             | number         | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» makerFixFee             | number         | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» settlementFee           | number         | true     | none         | Settlement fee                                                                                                                                                                                                                                                                                                                                           |
| »» isDeleverage            | boolean        | true     | none         | Enabled ADL or not                                                                                                                                                                                                                                                                                                                                       |
| »» isQuanto                | boolean        | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» isInverse               | boolean        | true     | none         | Whether it is a reverse contract                                                                                                                                                                                                                                                                                                                         |
| »» markMethod              | string         | true     | none         | Marking method                                                                                                                                                                                                                                                                                                                                           |
| »» fairMethod              | string         | true     | none         | Fair price marking method; the Futures contract is null                                                                                                                                                                                                                                                                                                  |
| »» fundingBaseSymbol       | string         | true     | none         | Ticker symbol of the base currency                                                                                                                                                                                                                                                                                                                       |
| »» fundingQuoteSymbol      | string         | true     | none         | Ticker symbol of the quote currency                                                                                                                                                                                                                                                                                                                      |
| »» fundingRateSymbol       | string         | true     | none         | Funding rate symbol                                                                                                                                                                                                                                                                                                                                      |
| »» indexSymbol             | string         | true     | none         | Index symbol                                                                                                                                                                                                                                                                                                                                             |
| »» settlementSymbol        | string         | true     | none         | Settlement symbol                                                                                                                                                                                                                                                                                                                                        |
| »» status                  | string         | true     | none         | Contract status                                                                                                                                                                                                                                                                                                                                          |
| »» fundingFeeRate          | number         | true     | none         | Funding fee rate                                                                                                                                                                                                                                                                                                                                         |
| »» predictedFundingFeeRate | number         | true     | none         | Predicted funding fee rate                                                                                                                                                                                                                                                                                                                               |
| »» fundingRateGranularity  | integer        | true     | none         | Funding interval (milliseconds)                                                                                                                                                                                                                                                                                                                          |
| »» openInterest            | string         | true     | none         | Open interest (unit: lots)                                                                                                                                                                                                                                                                                                                               |
| »» turnoverOf24h           | number         | true     | none         | 24-hour turnover                                                                                                                                                                                                                                                                                                                                         |
| »» volumeOf24h             | number         | true     | none         | 24-hour volume                                                                                                                                                                                                                                                                                                                                           |
| »» markPrice               | number         | true     | none         | Mark price                                                                                                                                                                                                                                                                                                                                               |
| »» indexPrice              | number         | true     | none         | Index price                                                                                                                                                                                                                                                                                                                                              |
| »» lastTradePrice          | number         | true     | none         | Last trade price                                                                                                                                                                                                                                                                                                                                         |
| »» nextFundingRateTime     | integer        | true     | none         | Next funding rate time (milliseconds)                                                                                                                                                                                                                                                                                                                    |
| »» maxLeverage             | integer        | true     | none         | Maximum leverage                                                                                                                                                                                                                                                                                                                                         |
| »» sourceExchanges         | [string]       | true     | none         | The contract index price source exchange                                                                                                                                                                                                                                                                                                                 |
| »» premiumsSymbol1M        | string         | true     | none         | Premium index symbol (1 minute)                                                                                                                                                                                                                                                                                                                          |
| »» premiumsSymbol8H        | string         | true     | none         | Premium index symbol (8 hours)                                                                                                                                                                                                                                                                                                                           |
| »» fundingBaseSymbol1M     | string         | true     | none         | Base currency interest rate symbol (1 minute)                                                                                                                                                                                                                                                                                                            |
| »» fundingQuoteSymbol1M    | string         | true     | none         | Quote currency interest rate symbol (1 minute)                                                                                                                                                                                                                                                                                                           |
| »» lowPrice                | number         | true     | none         | 24-hour lowest price                                                                                                                                                                                                                                                                                                                                     |
| »» highPrice               | number         | true     | none         | 24-hour highest price                                                                                                                                                                                                                                                                                                                                    |
| »» priceChgPct             | number         | true     | none         | 24-hour % price change                                                                                                                                                                                                                                                                                                                                   |
| »» priceChg                | number         | true     | none         | 24-hour price change                                                                                                                                                                                                                                                                                                                                     |
| »» k                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» m                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» f                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» mmrLimit                | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» mmrLevConstant          | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» supportCross            | boolean        | true     | none         | Whether support Cross Margin                                                                                                                                                                                                                                                                                                                             |
| »» buyLimit                | number         | true     | none         | The current maximum buying price allowed                                                                                                                                                                                                                                                                                                                 |
| »» sellLimit               | number         | true     | none         | The current minimum selling price allowed                                                                                                                                                                                                                                                                                                                |

#### Enumerated Values

| Property   | Value        |
| ---------- | ------------ |
| type       | FFWCSX       |
| type       | FFICSX       |
| markMethod | FairPrice    |
| fairMethod | FundingRate  |
| status     | Init         |
| status     | Open         |
| status     | BeingSettled |
| status     | Settled      |
| status     | Paused       |
| status     | Closed       |
| status     | CancelOnly   |

<aside class="success">
This operation does not require authentication
</aside>

## Get All Symbols

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/contracts/active", {
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

r = requests.get('/api/v1/contracts/active', headers = headers)

print(r.json())

```

`GET /api/v1/contracts/active`

Get detailed information of all contracts that can be traded. This API will
return a list of tradable contracts, including some key parameters of the
contract such as the symbol name, tick size, mark price, etc.

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "200000 is for success, other is error"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Symbol",
            "example": ["XBTUSDTM", "XBTUSDCM", "XBTUSDM"]
          },
          "rootSymbol": {
            "type": "string",
            "description": "Contract group",
            "example": ["USDT", "USDC", "XBT"]
          },
          "type": {
            "type": "string",
            "description": "Type of contract",
            "enum": ["FFWCSX", "FFICSX"],
            "x-api-enum": [
              {
                "value": "FFWCSX",
                "name": "FFWCSX",
                "description": "Standardized swap contracts, standard financial futures on swaps, expiration swap funding rates"
              },
              {
                "value": "FFICSX",
                "name": "FFICSX",
                "description": "Futures Contract"
              }
            ]
          },
          "firstOpenDate": {
            "type": "integer",
            "description": "First Open Date (milliseconds)",
            "format": "int64"
          },
          "expireDate": {
            "type": "integer",
            "description": "Expiration date (milliseconds) Null means it will never expire",
            "format": "int64"
          },
          "settleDate": {
            "type": "integer",
            "description": "Settlement date (milliseconds) Null indicates that automatic settlement is not supported",
            "format": "int64"
          },
          "baseCurrency": {
            "type": "string",
            "description": "Base currency"
          },
          "quoteCurrency": {
            "type": "string",
            "description": "Quote currency"
          },
          "settleCurrency": {
            "type": "string",
            "description": "Currency used to clear and settle the trades"
          },
          "maxOrderQty": {
            "type": "integer",
            "description": "Maximum order quantity"
          },
          "maxPrice": {
            "type": "number",
            "description": "Maximum order price"
          },
          "lotSize": {
            "type": "integer",
            "description": "Minimum lot size"
          },
          "tickSize": {
            "type": "number",
            "description": "Minimum price changes"
          },
          "indexPriceTickSize": {
            "type": "number",
            "description": "Index price of tick size"
          },
          "multiplier": {
            "type": "number",
            "description": "The basic unit of the contract API is lots. For the number of coins in each lot, please refer to the param multiplier. For example, for XBTUSDTM, multiplier=0.001, which corresponds to the value of each XBTUSDTM contract being 0.001 BTC. There is also a special case. All coin-swap contracts, such as each XBTUSDM contract, correspond to 1 USD."
          },
          "initialMargin": {
            "type": "number",
            "description": "Initial margin requirement"
          },
          "maintainMargin": {
            "type": "number",
            "description": "Maintenance margin requirement"
          },
          "maxRiskLimit": {
            "type": "integer",
            "description": "Maximum risk limit (unit: XBT)"
          },
          "minRiskLimit": {
            "type": "integer",
            "description": "Minimum risk limit (unit: XBT)"
          },
          "riskStep": {
            "type": "integer",
            "description": "Risk limit increment value (unit: XBT)"
          },
          "makerFeeRate": {
            "type": "number",
            "description": "Maker fee rate"
          },
          "takerFeeRate": {
            "type": "number",
            "description": "Taker fee rate"
          },
          "takerFixFee": {
            "type": "number",
            "description": "Deprecated param"
          },
          "makerFixFee": {
            "type": "number",
            "description": "Deprecated param"
          },
          "settlementFee": {
            "type": "number",
            "description": "Settlement fee"
          },
          "isDeleverage": {
            "type": "boolean",
            "description": "Enabled ADL or not"
          },
          "isQuanto": {
            "type": "boolean",
            "description": "Deprecated param"
          },
          "isInverse": {
            "type": "boolean",
            "description": "Whether it is a reverse contract"
          },
          "markMethod": {
            "type": "string",
            "description": "Marking method",
            "enum": ["FairPrice"],
            "x-api-enum": [
              {
                "value": "FairPrice",
                "name": "FairPrice",
                "description": "FairPrice"
              }
            ]
          },
          "fairMethod": {
            "type": "string",
            "description": "Fair price marking method; the Futures contract is null",
            "enum": ["FundingRate"],
            "x-api-enum": [
              {
                "value": "FundingRate",
                "name": "FundingRate",
                "description": "FundingRate"
              }
            ]
          },
          "fundingBaseSymbol": {
            "type": "string",
            "description": "Ticker symbol of the base currency"
          },
          "fundingQuoteSymbol": {
            "type": "string",
            "description": "Ticker symbol of the quote currency"
          },
          "fundingRateSymbol": {
            "type": "string",
            "description": "Funding rate symbol"
          },
          "indexSymbol": {
            "type": "string",
            "description": "Index symbol"
          },
          "settlementSymbol": {
            "type": "string",
            "description": "Settlement symbol"
          },
          "status": {
            "type": "string",
            "description": "Contract status",
            "enum": [
              "Init",
              "Open",
              "BeingSettled",
              "Settled",
              "Paused",
              "Closed",
              "CancelOnly"
            ],
            "x-api-enum": [
              {
                "value": "Init",
                "name": "Init",
                "description": "Initial"
              },
              {
                "value": "Open",
                "name": "Open",
                "description": "Online"
              },
              {
                "value": "BeingSettled",
                "name": "BeingSettled",
                "description": "Settling"
              },
              {
                "value": "Settled",
                "name": "Settled",
                "description": "Settled"
              },
              {
                "value": "Paused",
                "name": "Paused",
                "description": "Suspended"
              },
              {
                "value": "Closed",
                "name": "Closed",
                "description": "Offline"
              },
              {
                "value": "CancelOnly",
                "name": "CancelOnly",
                "description": "Orders can only be canceled"
              }
            ]
          },
          "fundingFeeRate": {
            "type": "number",
            "description": "Funding fee rate"
          },
          "predictedFundingFeeRate": {
            "type": "number",
            "description": "Predicted funding fee rate"
          },
          "fundingRateGranularity": {
            "type": "integer",
            "description": "Funding interval (milliseconds)"
          },
          "openInterest": {
            "type": "string",
            "description": "Open interest (unit: lots)"
          },
          "turnoverOf24h": {
            "type": "number",
            "description": "24-hour turnover"
          },
          "volumeOf24h": {
            "type": "number",
            "description": "24-hour volume"
          },
          "markPrice": {
            "type": "number",
            "description": "Mark price"
          },
          "indexPrice": {
            "type": "number",
            "description": "Index price"
          },
          "lastTradePrice": {
            "type": "number",
            "description": "Last trade price"
          },
          "nextFundingRateTime": {
            "type": "integer",
            "description": "Next funding rate time (milliseconds)"
          },
          "maxLeverage": {
            "type": "integer",
            "description": "Maximum leverage"
          },
          "sourceExchanges": {
            "type": "array",
            "items": {
              "type": "string"
            },
            "description": "The contract index price source exchange"
          },
          "premiumsSymbol1M": {
            "type": "string",
            "description": "Premium index symbol (1 minute)"
          },
          "premiumsSymbol8H": {
            "type": "string",
            "description": "Premium index symbol (8 hours)"
          },
          "fundingBaseSymbol1M": {
            "type": "string",
            "description": "Base currency interest rate symbol (1 minute)"
          },
          "fundingQuoteSymbol1M": {
            "type": "string",
            "description": "Quote currency interest rate symbol (1 minute)"
          },
          "lowPrice": {
            "type": "number",
            "description": "24-hour lowest price"
          },
          "highPrice": {
            "type": "number",
            "description": "24-hour highest price"
          },
          "priceChgPct": {
            "type": "number",
            "description": "24-hour % price change "
          },
          "priceChg": {
            "type": "number",
            "description": "24-hour price change"
          },
          "k": {
            "type": "number"
          },
          "m": {
            "type": "number"
          },
          "f": {
            "type": "number"
          },
          "mmrLimit": {
            "type": "number"
          },
          "mmrLevConstant": {
            "type": "number"
          },
          "supportCross": {
            "type": "boolean",
            "description": "Whether support Cross Margin"
          },
          "buyLimit": {
            "type": "number",
            "description": "The current maximum buying price allowed"
          },
          "sellLimit": {
            "type": "number",
            "description": "The current minimum selling price allowed"
          }
        },
        "required": [
          "symbol",
          "rootSymbol",
          "type",
          "firstOpenDate",
          "baseCurrency",
          "quoteCurrency",
          "settleCurrency",
          "maxOrderQty",
          "maxPrice",
          "lotSize",
          "tickSize",
          "indexPriceTickSize",
          "multiplier",
          "initialMargin",
          "maintainMargin",
          "maxRiskLimit",
          "minRiskLimit",
          "riskStep",
          "makerFeeRate",
          "takerFeeRate",
          "takerFixFee",
          "makerFixFee",
          "isDeleverage",
          "isQuanto",
          "isInverse",
          "markMethod",
          "indexSymbol",
          "settlementSymbol",
          "status",
          "fundingFeeRate",
          "predictedFundingFeeRate",
          "fundingRateGranularity",
          "openInterest",
          "turnoverOf24h",
          "volumeOf24h",
          "markPrice",
          "indexPrice",
          "lastTradePrice",
          "nextFundingRateTime",
          "maxLeverage",
          "sourceExchanges",
          "premiumsSymbol1M",
          "premiumsSymbol8H",
          "fundingBaseSymbol1M",
          "fundingQuoteSymbol1M",
          "lowPrice",
          "highPrice",
          "priceChgPct",
          "priceChg",
          "k",
          "m",
          "f",
          "mmrLimit",
          "mmrLevConstant",
          "supportCross",
          "buyLimit",
          "sellLimit"
        ]
      },
      "description": "List of all contracts"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-all-symbols-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-all-symbols-responseschema">Response Schema</h3>

Status Code **200**

| Name                       | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                              |
| -------------------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code                     | string         | true     | none         | 200000 is for success, other is error                                                                                                                                                                                                                                                                                                                    |
| » data                     | [object]       | true     | none         | List of all contracts                                                                                                                                                                                                                                                                                                                                    |
| »» symbol                  | string         | true     | none         | Symbol                                                                                                                                                                                                                                                                                                                                                   |
| »» rootSymbol              | string         | true     | none         | Contract group                                                                                                                                                                                                                                                                                                                                           |
| »» type                    | string         | true     | none         | Type of contract                                                                                                                                                                                                                                                                                                                                         |
| »» firstOpenDate           | integer(int64) | true     | none         | First Open Date (milliseconds)                                                                                                                                                                                                                                                                                                                           |
| »» expireDate              | integer(int64) | false    | none         | Expiration date (milliseconds) Null means it will never expire                                                                                                                                                                                                                                                                                           |
| »» settleDate              | integer(int64) | false    | none         | Settlement date (milliseconds) Null indicates that automatic settlement is not supported                                                                                                                                                                                                                                                                 |
| »» baseCurrency            | string         | true     | none         | Base currency                                                                                                                                                                                                                                                                                                                                            |
| »» quoteCurrency           | string         | true     | none         | Quote currency                                                                                                                                                                                                                                                                                                                                           |
| »» settleCurrency          | string         | true     | none         | Currency used to clear and settle the trades                                                                                                                                                                                                                                                                                                             |
| »» maxOrderQty             | integer        | true     | none         | Maximum order quantity                                                                                                                                                                                                                                                                                                                                   |
| »» maxPrice                | number         | true     | none         | Maximum order price                                                                                                                                                                                                                                                                                                                                      |
| »» lotSize                 | integer        | true     | none         | Minimum lot size                                                                                                                                                                                                                                                                                                                                         |
| »» tickSize                | number         | true     | none         | Minimum price changes                                                                                                                                                                                                                                                                                                                                    |
| »» indexPriceTickSize      | number         | true     | none         | Index price of tick size                                                                                                                                                                                                                                                                                                                                 |
| »» multiplier              | number         | true     | none         | The basic unit of the contract API is lots. For the number of coins in each lot, please refer to the param multiplier. For example, for XBTUSDTM, multiplier=0.001, which corresponds to the value of each XBTUSDTM contract being 0.001 BTC. There is also a special case. All coin-swap contracts, such as each XBTUSDM contract, correspond to 1 USD. |
| »» initialMargin           | number         | true     | none         | Initial margin requirement                                                                                                                                                                                                                                                                                                                               |
| »» maintainMargin          | number         | true     | none         | Maintenance margin requirement                                                                                                                                                                                                                                                                                                                           |
| »» maxRiskLimit            | integer        | true     | none         | Maximum risk limit (unit: XBT)                                                                                                                                                                                                                                                                                                                           |
| »» minRiskLimit            | integer        | true     | none         | Minimum risk limit (unit: XBT)                                                                                                                                                                                                                                                                                                                           |
| »» riskStep                | integer        | true     | none         | Risk limit increment value (unit: XBT)                                                                                                                                                                                                                                                                                                                   |
| »» makerFeeRate            | number         | true     | none         | Maker fee rate                                                                                                                                                                                                                                                                                                                                           |
| »» takerFeeRate            | number         | true     | none         | Taker fee rate                                                                                                                                                                                                                                                                                                                                           |
| »» takerFixFee             | number         | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» makerFixFee             | number         | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» settlementFee           | number         | false    | none         | Settlement fee                                                                                                                                                                                                                                                                                                                                           |
| »» isDeleverage            | boolean        | true     | none         | Enabled ADL or not                                                                                                                                                                                                                                                                                                                                       |
| »» isQuanto                | boolean        | true     | none         | Deprecated param                                                                                                                                                                                                                                                                                                                                         |
| »» isInverse               | boolean        | true     | none         | Whether it is a reverse contract                                                                                                                                                                                                                                                                                                                         |
| »» markMethod              | string         | true     | none         | Marking method                                                                                                                                                                                                                                                                                                                                           |
| »» fairMethod              | string         | false    | none         | Fair price marking method; the Futures contract is null                                                                                                                                                                                                                                                                                                  |
| »» fundingBaseSymbol       | string         | false    | none         | Ticker symbol of the base currency                                                                                                                                                                                                                                                                                                                       |
| »» fundingQuoteSymbol      | string         | false    | none         | Ticker symbol of the quote currency                                                                                                                                                                                                                                                                                                                      |
| »» fundingRateSymbol       | string         | false    | none         | Funding rate symbol                                                                                                                                                                                                                                                                                                                                      |
| »» indexSymbol             | string         | true     | none         | Index symbol                                                                                                                                                                                                                                                                                                                                             |
| »» settlementSymbol        | string         | true     | none         | Settlement symbol                                                                                                                                                                                                                                                                                                                                        |
| »» status                  | string         | true     | none         | Contract status                                                                                                                                                                                                                                                                                                                                          |
| »» fundingFeeRate          | number         | true     | none         | Funding fee rate                                                                                                                                                                                                                                                                                                                                         |
| »» predictedFundingFeeRate | number         | true     | none         | Predicted funding fee rate                                                                                                                                                                                                                                                                                                                               |
| »» fundingRateGranularity  | integer        | true     | none         | Funding interval (milliseconds)                                                                                                                                                                                                                                                                                                                          |
| »» openInterest            | string         | true     | none         | Open interest (unit: lots)                                                                                                                                                                                                                                                                                                                               |
| »» turnoverOf24h           | number         | true     | none         | 24-hour turnover                                                                                                                                                                                                                                                                                                                                         |
| »» volumeOf24h             | number         | true     | none         | 24-hour volume                                                                                                                                                                                                                                                                                                                                           |
| »» markPrice               | number         | true     | none         | Mark price                                                                                                                                                                                                                                                                                                                                               |
| »» indexPrice              | number         | true     | none         | Index price                                                                                                                                                                                                                                                                                                                                              |
| »» lastTradePrice          | number         | true     | none         | Last trade price                                                                                                                                                                                                                                                                                                                                         |
| »» nextFundingRateTime     | integer        | true     | none         | Next funding rate time (milliseconds)                                                                                                                                                                                                                                                                                                                    |
| »» maxLeverage             | integer        | true     | none         | Maximum leverage                                                                                                                                                                                                                                                                                                                                         |
| »» sourceExchanges         | [string]       | true     | none         | The contract index price source exchange                                                                                                                                                                                                                                                                                                                 |
| »» premiumsSymbol1M        | string         | true     | none         | Premium index symbol (1 minute)                                                                                                                                                                                                                                                                                                                          |
| »» premiumsSymbol8H        | string         | true     | none         | Premium index symbol (8 hours)                                                                                                                                                                                                                                                                                                                           |
| »» fundingBaseSymbol1M     | string         | true     | none         | Base currency interest rate symbol (1 minute)                                                                                                                                                                                                                                                                                                            |
| »» fundingQuoteSymbol1M    | string         | true     | none         | Quote currency interest rate symbol (1 minute)                                                                                                                                                                                                                                                                                                           |
| »» lowPrice                | number         | true     | none         | 24-hour lowest price                                                                                                                                                                                                                                                                                                                                     |
| »» highPrice               | number         | true     | none         | 24-hour highest price                                                                                                                                                                                                                                                                                                                                    |
| »» priceChgPct             | number         | true     | none         | 24-hour % price change                                                                                                                                                                                                                                                                                                                                   |
| »» priceChg                | number         | true     | none         | 24-hour price change                                                                                                                                                                                                                                                                                                                                     |
| »» k                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» m                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» f                       | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» mmrLimit                | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» mmrLevConstant          | number         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                     |
| »» supportCross            | boolean        | true     | none         | Whether support Cross Margin                                                                                                                                                                                                                                                                                                                             |
| »» buyLimit                | number         | true     | none         | The current maximum buying price allowed                                                                                                                                                                                                                                                                                                                 |
| »» sellLimit               | number         | true     | none         | The current minimum selling price allowed                                                                                                                                                                                                                                                                                                                |

#### Enumerated Values

| Property   | Value        |
| ---------- | ------------ |
| type       | FFWCSX       |
| type       | FFICSX       |
| markMethod | FairPrice    |
| fairMethod | FundingRate  |
| status     | Init         |
| status     | Open         |
| status     | BeingSettled |
| status     | Settled      |
| status     | Paused       |
| status     | Closed       |
| status     | CancelOnly   |

<aside class="success">
This operation does not require authentication
</aside>

## Get Ticker

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/ticker?symbol=XBTUSDTM,XBTUSDM,ETHUSDTM", {
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

r = requests.get('/api/v1/ticker', params={
  'symbol': [
  "XBTUSDTM",
  "XBTUSDM",
  "ETHUSDTM"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/ticker`

This endpoint returns "last traded price/size", "best bid/ask price/size" etc.
of a single symbol. These messages can also be obtained through Websocket.

<h3 id="get-ticker-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

#### Detailed descriptions

**symbol**: Symbol of the contract. Please refer to
[Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "200000 is for success, other is error"
    },
    "data": {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "description": "Sequence number, used to judge whether the messages pushed by Websocket are continuous.",
          "format": "int64"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "side": {
          "type": "string",
          "description": "Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book.",
          "enum": ["buy", "sell"],
          "x-api-enum": [
            {
              "value": "buy",
              "name": "buy",
              "description": "buy"
            },
            {
              "value": "sell",
              "name": "sell",
              "description": "sell"
            }
          ]
        },
        "size": {
          "type": "integer",
          "description": "Filled quantity"
        },
        "tradeId": {
          "type": "string",
          "description": "Transaction ID"
        },
        "price": {
          "type": "string",
          "description": "Filled price"
        },
        "bestBidPrice": {
          "type": "string",
          "description": "Best bid price"
        },
        "bestBidSize": {
          "type": "integer",
          "description": "Best bid size"
        },
        "bestAskPrice": {
          "type": "string",
          "description": "Best ask price"
        },
        "bestAskSize": {
          "type": "integer",
          "description": "Best ask size"
        },
        "ts": {
          "type": "integer",
          "description": "Filled time (nanoseconds)",
          "format": "int64"
        }
      },
      "required": [
        "sequence",
        "symbol",
        "side",
        "size",
        "tradeId",
        "price",
        "bestBidPrice",
        "bestBidSize",
        "bestAskPrice",
        "bestAskSize",
        "ts"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-ticker-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-ticker-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                                   |
| --------------- | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | 200000 is for success, other is error                                                                                                         |
| » data          | object         | true     | none         | none                                                                                                                                          |
| »» sequence     | integer(int64) | true     | none         | Sequence number, used to judge whether the messages pushed by Websocket are continuous.                                                       |
| »» symbol       | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                            |
| »» side         | string         | true     | none         | Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book. |
| »» size         | integer        | true     | none         | Filled quantity                                                                                                                               |
| »» tradeId      | string         | true     | none         | Transaction ID                                                                                                                                |
| »» price        | string         | true     | none         | Filled price                                                                                                                                  |
| »» bestBidPrice | string         | true     | none         | Best bid price                                                                                                                                |
| »» bestBidSize  | integer        | true     | none         | Best bid size                                                                                                                                 |
| »» bestAskPrice | string         | true     | none         | Best ask price                                                                                                                                |
| »» bestAskSize  | integer        | true     | none         | Best ask size                                                                                                                                 |
| »» ts           | integer(int64) | true     | none         | Filled time (nanoseconds)                                                                                                                     |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | buy   |
| side     | sell  |

<aside class="success">
This operation does not require authentication
</aside>

## Get All Tickers

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/allTickers", {
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

r = requests.get('/api/v1/allTickers', headers = headers)

print(r.json())

```

`GET /api/v1/allTickers`

This endpoint returns "last traded price/size", "best bid/ask price/size" etc.
of a single symbol. These messages can also be obtained through Websocket.

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "200000 is for success, other is error"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "sequence": {
            "type": "integer",
            "description": "Sequence number, used to judge whether the messages pushed by Websocket are continuous.",
            "format": "int64"
          },
          "symbol": {
            "type": "string",
            "description": "Symbol"
          },
          "side": {
            "type": "string",
            "description": "Trade direction",
            "enum": ["buy", "sell"],
            "x-api-enum": [
              {
                "value": "buy",
                "name": "BUY",
                "description": ""
              },
              {
                "value": "sell",
                "name": "SELL",
                "description": ""
              }
            ]
          },
          "size": {
            "type": "integer",
            "description": "Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book."
          },
          "tradeId": {
            "type": "string",
            "description": "Transaction ID"
          },
          "price": {
            "type": "string",
            "description": "Filled price"
          },
          "bestBidPrice": {
            "type": "string",
            "description": "Best bid price"
          },
          "bestBidSize": {
            "type": "integer",
            "description": "Best bid size"
          },
          "bestAskPrice": {
            "type": "string",
            "description": "Best ask price"
          },
          "bestAskSize": {
            "type": "integer",
            "description": "Best ask size"
          },
          "ts": {
            "type": "integer",
            "description": "Filled time (nanoseconds)",
            "format": "int64"
          }
        },
        "required": [
          "sequence",
          "symbol",
          "side",
          "tradeId",
          "price",
          "bestBidPrice",
          "bestBidSize",
          "bestAskPrice",
          "bestAskSize",
          "ts",
          "size"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-all-tickers-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-all-tickers-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                                   |
| --------------- | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | 200000 is for success, other is error                                                                                                         |
| » data          | [object]       | true     | none         | none                                                                                                                                          |
| »» sequence     | integer(int64) | true     | none         | Sequence number, used to judge whether the messages pushed by Websocket are continuous.                                                       |
| »» symbol       | string         | true     | none         | Symbol                                                                                                                                        |
| »» side         | string         | true     | none         | Trade direction                                                                                                                               |
| »» size         | integer        | true     | none         | Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book. |
| »» tradeId      | string         | true     | none         | Transaction ID                                                                                                                                |
| »» price        | string         | true     | none         | Filled price                                                                                                                                  |
| »» bestBidPrice | string         | true     | none         | Best bid price                                                                                                                                |
| »» bestBidSize  | integer        | true     | none         | Best bid size                                                                                                                                 |
| »» bestAskPrice | string         | true     | none         | Best ask price                                                                                                                                |
| »» bestAskSize  | integer        | true     | none         | Best ask size                                                                                                                                 |
| »» ts           | integer(int64) | true     | none         | Filled time (nanoseconds)                                                                                                                     |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | buy   |
| side     | sell  |

<aside class="success">
This operation does not require authentication
</aside>

## Get Full OrderBook

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/level2/snapshot", {
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

r = requests.get('/api/v1/level2/snapshot', headers = headers)

print(r.json())

```

`GET /api/v1/level2/snapshot`

Query for Full orderbook depth data (aggregated by price). It is generally used
by professional traders because it uses more server resources and traffic, and
we have strict access rate limit control. To maintain an up-to-date Order Book,
please use Websocket incremental feed after retrieving the OrderBook.

<h3 id="get-full-orderbook-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | false    | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

#### Detailed descriptions

**symbol**: Symbol of the contract. Please refer to
[Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "200000 is for success, other is error"
    },
    "data": {
      "type": "object",
      "properties": {
        "sequence": {
          "type": "integer",
          "description": "Sequence number",
          "format": "int64"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "bids": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "number",
              "description": "Price, Size"
            }
          },
          "description": "bids, from high to low"
        },
        "asks": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "number",
              "description": "Price, Size"
            }
          },
          "description": "asks, from low to high"
        },
        "ts": {
          "type": "integer",
          "format": "int64",
          "description": "Timestamp (nanoseconds)"
        }
      },
      "required": ["sequence", "symbol", "bids", "asks", "ts"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-full-orderbook-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-full-orderbook-responseschema">Response Schema</h3>

Status Code **200**

| Name        | Type           | Required | Restrictions | Description                                                                                                        |
| ----------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code      | string         | true     | none         | 200000 is for success, other is error                                                                              |
| » data      | object         | true     | none         | none                                                                                                               |
| »» sequence | integer(int64) | true     | none         | Sequence number                                                                                                    |
| »» symbol   | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» bids     | [array]        | true     | none         | bids, from high to low                                                                                             |
| »» asks     | [array]        | true     | none         | asks, from low to high                                                                                             |
| »» ts       | integer(int64) | true     | none         | Timestamp (nanoseconds)                                                                                            |

<aside class="success">
This operation does not require authentication
</aside>

## Get Part OrderBook

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/level2/depth{size}?symbol=type,string", {
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

r = requests.get('/api/v1/level2/depth{size}', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/level2/depth{size}`

Query for part orderbook depth data. (aggregated by price). It is recommended
that you request via this endpoint, as the system response will be faster and
consume less traffic.

<h3 id="get-part-orderbook-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| size   | path  | string | true     | Get the depth layer, optional value: 20, 100                                                                       |
| symbol | query | string | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
        "sequence": {
          "type": "integer",
          "description": "Sequence number",
          "format": "int64"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "bids": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "number",
              "description": "Price, Size"
            }
          },
          "description": "bids, from high to low"
        },
        "asks": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "number",
              "description": "Price, Size"
            }
          },
          "description": "asks, from low to high"
        },
        "ts": {
          "type": "integer",
          "description": "Timestamp (nanoseconds)",
          "format": "int64"
        }
      },
      "required": ["sequence", "symbol", "bids", "asks", "ts"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-part-orderbook-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-part-orderbook-responseschema">Response Schema</h3>

Status Code **200**

| Name        | Type           | Required | Restrictions | Description                                                                                                        |
| ----------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code      | string         | true     | none         | none                                                                                                               |
| » data      | object         | true     | none         | none                                                                                                               |
| »» sequence | integer(int64) | true     | none         | Sequence number                                                                                                    |
| »» symbol   | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» bids     | [array]        | true     | none         | bids, from high to low                                                                                             |
| »» asks     | [array]        | true     | none         | asks, from low to high                                                                                             |
| »» ts       | integer(int64) | true     | none         | Timestamp (nanoseconds)                                                                                            |

<aside class="success">
This operation does not require authentication
</aside>

## Get Trade History

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/trade/history?symbol=type,string", {
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

r = requests.get('/api/v1/trade/history', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/trade/history`

Request the trade history of the specified symbol via this endpoint. The
returned quantity is the last 100 transaction records.

<h3 id="get-trade-history-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
          "sequence": {
            "type": "integer",
            "description": "Sequence number",
            "format": "int64"
          },
          "contractId": {
            "type": "integer",
            "description": "Deprecated param"
          },
          "tradeId": {
            "type": "string",
            "description": "Transaction ID"
          },
          "makerOrderId": {
            "type": "string",
            "description": "Maker order ID"
          },
          "takerOrderId": {
            "type": "string",
            "description": "Taker order ID"
          },
          "ts": {
            "type": "integer",
            "description": "Filled timestamp (nanosecond)",
            "format": "int64"
          },
          "size": {
            "type": "integer",
            "description": "Filled amount"
          },
          "price": {
            "type": "string",
            "description": "Filled price"
          },
          "side": {
            "type": "string",
            "description": "Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book.",
            "enum": ["buy", "sell"],
            "x-api-enum": [
              {
                "value": "buy",
                "name": "buy",
                "description": "buy"
              },
              {
                "value": "sell",
                "name": "sell",
                "description": "sell"
              }
            ]
          }
        },
        "required": [
          "sequence",
          "contractId",
          "tradeId",
          "makerOrderId",
          "takerOrderId",
          "ts",
          "size",
          "price",
          "side"
        ]
      }
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

| Name            | Type           | Required | Restrictions | Description                                                                                                                                   |
| --------------- | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                                                                                                          |
| » data          | [object]       | true     | none         | none                                                                                                                                          |
| »» sequence     | integer(int64) | true     | none         | Sequence number                                                                                                                               |
| »» contractId   | integer        | true     | none         | Deprecated param                                                                                                                              |
| »» tradeId      | string         | true     | none         | Transaction ID                                                                                                                                |
| »» makerOrderId | string         | true     | none         | Maker order ID                                                                                                                                |
| »» takerOrderId | string         | true     | none         | Taker order ID                                                                                                                                |
| »» ts           | integer(int64) | true     | none         | Filled timestamp (nanosecond)                                                                                                                 |
| »» size         | integer        | true     | none         | Filled amount                                                                                                                                 |
| »» price        | string         | true     | none         | Filled price                                                                                                                                  |
| »» side         | string         | true     | none         | Filled side; the trade side indicates the taker order side. A taker order is the order that was matched with orders opened on the order book. |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | buy   |
| side     | sell  |

<aside class="success">
This operation does not require authentication
</aside>

## Get Klines

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/kline/query?symbol=XBTUSDTM,.KXBTUSDT,.XBTUSDTMPI,.XBTUSDTMPI8H&granularity=type,integer,enum,1%2C5%2C15%2C30%2C60%2C120%2C240%2C480%2C720%2C1440%2C10080,format,int64,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v1/kline/query', params={
  'symbol': [
  "XBTUSDTM",
  ".KXBTUSDT",
  ".XBTUSDTMPI",
  ".XBTUSDTMPI8H"
],  'granularity': {
  "type": "integer",
  "enum": [
    1,
    5,
    15,
    30,
    60,
    120,
    240,
    480,
    720,
    1440,
    10080
  ],
  "format": "int64",
  "x-api-enum": [
    {
      "value": 1,
      "name": "1",
      "description": "1min"
    },
    {
      "value": 5,
      "name": "5",
      "description": "5min"
    },
    {
      "value": 15,
      "name": "15",
      "description": "15min"
    },
    {
      "value": 30,
      "name": "30",
      "description": "30min"
    },
    {
      "value": 60,
      "name": "60",
      "description": "1hour"
    },
    {
      "value": 120,
      "name": "120",
      "description": "2hour"
    },
    {
      "value": 240,
      "name": "240",
      "description": "4hour"
    },
    {
      "value": 480,
      "name": "480",
      "description": "8hour"
    },
    {
      "value": 720,
      "name": "720",
      "description": "12hour"
    },
    {
      "value": 1440,
      "name": "1440",
      "description": "1day"
    },
    {
      "value": 10080,
      "name": "10080",
      "description": "1week"
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/kline/query`

Get the symbol’s candlestick chart. Data are returned in grouped buckets based
on requested type. For each query, the system will return at most 500 pieces of
data. To obtain more data, please page the data by time.

<h3 id="get-klines-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                                         |
| ----------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol, indexSymbol, premiumsSymbol1M, premiumsSymbol8H](https://www.kucoin.com/docs-new/api-3470220) |
| granularity | query | integer(int64) | true     | Type of candlestick patterns (minutes)                                                                                                                              |
| from        | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                                           |
| to          | query | integer(int64) | false    | End time (milliseconds)                                                                                                                                             |

#### Enumerated Values

| Parameter   | Value |
| ----------- | ----- |
| granularity | 1     |
| granularity | 5     |
| granularity | 15    |
| granularity | 30    |
| granularity | 60    |
| granularity | 120   |
| granularity | 240   |
| granularity | 480   |
| granularity | 720   |
| granularity | 1440  |
| granularity | 10080 |

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
        "type": "array",
        "items": {
          "type": "number",
          "description": "Start time of the candle cycle, opening price, highest price, lowest price, closing price, transaction volume"
        }
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-klines-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-klines-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type    | Required | Restrictions | Description |
| ------ | ------- | -------- | ------------ | ----------- |
| » code | string  | true     | none         | none        |
| » data | [array] | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Mark Price

<a id="opId009"></a>

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

Get the current mark price (Update snapshots once per second, real-time query).

<h3 id="get-mark-price-parameters">Parameters</h3>

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
      "type": "object",
      "properties": {
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "granularity": {
          "type": "integer",
          "description": "Granularity (milliseconds)"
        },
        "timePoint": {
          "type": "integer",
          "description": "Time point (milliseconds)",
          "format": "int64"
        },
        "value": {
          "type": "number",
          "description": "Mark price"
        },
        "indexPrice": {
          "type": "number",
          "description": "Index price"
        }
      },
      "required": ["symbol", "granularity", "timePoint", "value", "indexPrice"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-mark-price-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-mark-price-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                                                                                        |
| -------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code         | string         | true     | none         | none                                                                                                               |
| » data         | object         | true     | none         | none                                                                                                               |
| »» symbol      | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» granularity | integer        | true     | none         | Granularity (milliseconds)                                                                                         |
| »» timePoint   | integer(int64) | true     | none         | Time point (milliseconds)                                                                                          |
| »» value       | number         | true     | none         | Mark price                                                                                                         |
| »» indexPrice  | number         | true     | none         | Index price                                                                                                        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Spot Index Price

<a id="opId010"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/index/query?symbol=.KXBTUSDT,.BXBT", {
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

r = requests.get('/api/v1/index/query', params={
  'symbol': [
  ".KXBTUSDT",
  ".BXBT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/index/query`

Get Spot Index Price (Update snapshots once per second, and there is a 5s cache
when querying).

<h3 id="get-spot-index-price-parameters">Parameters</h3>

| Name     | In    | Type           | Required | Description                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol   | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: indexSymbol](https://www.kucoin.com/docs-new/api-3470220)                                  |
| startAt  | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                                |
| endAt    | query | integer(int64) | false    | End time (milliseconds)                                                                                                                                  |
| reverse  | query | boolean        | false    | This parameter functions to judge whether the lookup is reversed. True means “yes”. False means “no”. This parameter is set as True by default.          |
| offset   | query | integer(int64) | false    | Start offset. The unique attribute of the last returned result of the last request. The data of the first page will be returned by default.              |
| forward  | query | boolean        | false    | This parameter functions to judge whether the lookup is forward or not. True means “yes” and False means “no”. This parameter is set as true by default. |
| maxCount | query | integer(int64) | false    | Max. record count. The default record count is 10; the maximum length cannot exceed 100                                                                  |

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
        "dataList": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: indexSymbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "granularity": {
                "type": "integer",
                "description": "Granularity (milliseconds)"
              },
              "timePoint": {
                "type": "integer",
                "description": "Timestamp (milliseconds)",
                "format": "int64"
              },
              "value": {
                "type": "number",
                "description": "Index Value"
              },
              "decomposionList": {
                "type": "array",
                "items": {
                  "type": "object",
                  "properties": {
                    "exchange": {
                      "type": "string",
                      "description": "Exchange"
                    },
                    "price": {
                      "type": "number",
                      "description": "Price"
                    },
                    "weight": {
                      "type": "number",
                      "description": "Weight"
                    }
                  },
                  "required": ["exchange", "price", "weight"]
                },
                "description": "Component List"
              }
            },
            "required": [
              "symbol",
              "granularity",
              "timePoint",
              "value",
              "decomposionList"
            ]
          }
        },
        "hasMore": {
          "type": "boolean",
          "description": "Whether there are more pages"
        }
      },
      "required": ["dataList", "hasMore"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-spot-index-price-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-spot-index-price-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type           | Required | Restrictions | Description                                                                                                             |
| ------------------- | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------- |
| » code              | string         | true     | none         | none                                                                                                                    |
| » data              | object         | true     | none         | none                                                                                                                    |
| »» dataList         | [object]       | true     | none         | none                                                                                                                    |
| »»» symbol          | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: indexSymbol](https://www.kucoin.com/docs-new/api-3470220) |
| »»» granularity     | integer        | true     | none         | Granularity (milliseconds)                                                                                              |
| »»» timePoint       | integer(int64) | true     | none         | Timestamp (milliseconds)                                                                                                |
| »»» value           | number         | true     | none         | Index Value                                                                                                             |
| »»» decomposionList | [object]       | true     | none         | Component List                                                                                                          |
| »»»» exchange       | string         | true     | none         | Exchange                                                                                                                |
| »»»» price          | number         | true     | none         | Price                                                                                                                   |
| »»»» weight         | number         | true     | none         | Weight                                                                                                                  |
| »» hasMore          | boolean        | true     | none         | Whether there are more pages                                                                                            |

<aside class="success">
This operation does not require authentication
</aside>

## Get Interest Rate Index

<a id="opId011"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/interest/query?symbol=.XBTINT8H,.USDTINT8H,.XBTINT,.USDTINT", {
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

r = requests.get('/api/v1/interest/query', params={
  'symbol': [
  ".XBTINT8H",
  ".USDTINT8H",
  ".XBTINT",
  ".USDTINT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/interest/query`

Get interest rate Index (real-time query).

<h3 id="get-interest-rate-index-parameters">Parameters</h3>

| Name     | In    | Type           | Required | Description                                                                                                                                                                                  |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol   | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: fundingBaseSymbol, fundingQuoteSymbol, fundingBaseSymbol1M, fundingQuoteSymbol1M](https://www.kucoin.com/docs-new/api-3470220) |
| startAt  | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                                                                    |
| endAt    | query | integer(int64) | false    | End time (milliseconds)                                                                                                                                                                      |
| reverse  | query | boolean        | false    | This parameter functions to judge whether the lookup is reversed. True means “yes”. False means “no”. This parameter is set as True by default.                                              |
| offset   | query | integer(int64) | false    | Start offset. The unique attribute of the last returned result of the last request. The data of the first page will be returned by default.                                                  |
| forward  | query | boolean        | false    | This parameter functions to judge whether the lookup is forward or not. True means “yes” and False means “no”. This parameter is set as true by default.                                     |
| maxCount | query | integer(int64) | false    | Max. record count. The default record count is 10; the maximum length cannot exceed 100                                                                                                      |

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
        "dataList": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: fundingBaseSymbol, fundingQuoteSymbol, fundingBaseSymbol1M, fundingQuoteSymbol1M](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "granularity": {
                "type": "integer",
                "description": "Granularity (milliseconds)"
              },
              "timePoint": {
                "type": "integer",
                "description": "Timestamp (milliseconds)",
                "format": "int64"
              },
              "value": {
                "type": "number",
                "description": "Interest rate value"
              }
            },
            "required": ["symbol", "granularity", "timePoint", "value"]
          }
        },
        "hasMore": {
          "type": "boolean",
          "description": "Whether there are more pages"
        }
      },
      "required": ["dataList", "hasMore"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-interest-rate-index-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-interest-rate-index-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                                                                                  |
| --------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                                                                                                                                                         |
| » data          | object         | true     | none         | none                                                                                                                                                                                         |
| »» dataList     | [object]       | true     | none         | none                                                                                                                                                                                         |
| »»» symbol      | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: fundingBaseSymbol, fundingQuoteSymbol, fundingBaseSymbol1M, fundingQuoteSymbol1M](https://www.kucoin.com/docs-new/api-3470220) |
| »»» granularity | integer        | true     | none         | Granularity (milliseconds)                                                                                                                                                                   |
| »»» timePoint   | integer(int64) | true     | none         | Timestamp (milliseconds)                                                                                                                                                                     |
| »»» value       | number         | true     | none         | Interest rate value                                                                                                                                                                          |
| »» hasMore      | boolean        | true     | none         | Whether there are more pages                                                                                                                                                                 |

<aside class="success">
This operation does not require authentication
</aside>

## Get Premium Index

<a id="opId012"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/premium/query?symbol=.XBTUSDTMPI,.XBTUSDTMPI8H", {
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

r = requests.get('/api/v1/premium/query', params={
  'symbol': [
  ".XBTUSDTMPI",
  ".XBTUSDTMPI8H"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/premium/query`

Submit request to get premium index (Update snapshots once per second, real-time
query).

<h3 id="get-premium-index-parameters">Parameters</h3>

| Name     | In    | Type           | Required | Description                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol   | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: premiumsSymbol1M, premiumsSymbol8H](https://www.kucoin.com/docs-new/api-3470220)           |
| startAt  | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                                |
| endAt    | query | integer(int64) | false    | End time (milliseconds)                                                                                                                                  |
| reverse  | query | boolean        | false    | This parameter functions to judge whether the lookup is reversed. True means “yes”. False means “no”. This parameter is set as True by default.          |
| offset   | query | integer(int64) | false    | Start offset. The unique attribute of the last returned result of the last request. The data of the first page will be returned by default.              |
| forward  | query | boolean        | false    | This parameter functions to judge whether the lookup is forward or not. True means “yes” and False means “no”. This parameter is set as true by default. |
| maxCount | query | integer(int64) | false    | Max. record count. The default record count is 10; the maximum length cannot exceed 100                                                                  |

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
        "dataList": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: premiumsSymbol1M, premiumsSymbol8H](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "granularity": {
                "type": "integer",
                "description": "Granularity (milliseconds)"
              },
              "timePoint": {
                "type": "integer",
                "description": "Timestamp (milliseconds)",
                "format": "int64"
              },
              "value": {
                "type": "number",
                "description": "Premium index"
              }
            },
            "required": ["symbol", "granularity", "timePoint", "value"]
          }
        },
        "hasMore": {
          "type": "boolean",
          "description": "Whether there are more pages"
        }
      },
      "required": ["dataList", "hasMore"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-premium-index-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-premium-index-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                                    |
| --------------- | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                                                                                                           |
| » data          | object         | true     | none         | none                                                                                                                                           |
| »» dataList     | [object]       | true     | none         | none                                                                                                                                           |
| »»» symbol      | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: premiumsSymbol1M, premiumsSymbol8H](https://www.kucoin.com/docs-new/api-3470220) |
| »»» granularity | integer        | true     | none         | Granularity (milliseconds)                                                                                                                     |
| »»» timePoint   | integer(int64) | true     | none         | Timestamp (milliseconds)                                                                                                                       |
| »»» value       | number         | true     | none         | Premium index                                                                                                                                  |
| »» hasMore      | boolean        | true     | none         | Whether there are more pages                                                                                                                   |

<aside class="success">
This operation does not require authentication
</aside>

## Get 24hr stats

<a id="opId013"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/trade-statistics", {
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

r = requests.get('/api/v1/trade-statistics', headers = headers)

print(r.json())

```

`GET /api/v1/trade-statistics`

Get the statistics of the platform futures trading volume in the last 24 hours.

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
        "turnoverOf24h": {
          "type": "number",
          "description": "24-hour platform Futures trading volume. Unit is USD"
        }
      },
      "required": ["turnoverOf24h"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-24hr-stats-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-24hr-stats-responseschema">Response Schema</h3>

Status Code **200**

| Name             | Type   | Required | Restrictions | Description                                          |
| ---------------- | ------ | -------- | ------------ | ---------------------------------------------------- |
| » code           | string | true     | none         | none                                                 |
| » data           | object | true     | none         | none                                                 |
| »» turnoverOf24h | number | true     | none         | 24-hour platform Futures trading volume. Unit is USD |

<aside class="success">
This operation does not require authentication
</aside>

## Get Server Time

<a id="opId014"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/timestamp", {
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

r = requests.get('/api/v1/timestamp', headers = headers)

print(r.json())

```

`GET /api/v1/timestamp`

Get the API server time. This is the Unix timestamp.

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
      "type": "integer",
      "format": "int64",
      "description": "ServerTime (milliseconds)"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-server-time-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-server-time-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type           | Required | Restrictions | Description               |
| ------ | -------------- | -------- | ------------ | ------------------------- |
| » code | string         | true     | none         | none                      |
| » data | integer(int64) | true     | none         | ServerTime (milliseconds) |

<aside class="success">
This operation does not require authentication
</aside>

## Get Service Status

<a id="opId015"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/status", {
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

r = requests.get('/api/v1/status', headers = headers)

print(r.json())

```

`GET /api/v1/status`

Get the service status.

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
        "msg": {
          "type": "string"
        },
        "status": {
          "type": "string",
          "description": "Status of service: open: normal transaction; close: Stop Trading/Maintenance; cancelonly: can only cancel the order but not place order",
          "enum": ["open", "close", "cancelonly"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "OPEN",
              "description": "normal transaction"
            },
            {
              "value": "close",
              "name": "CLOSE",
              "description": "Stop Trading/Maintenance"
            },
            {
              "value": "cancelonly",
              "name": "CANCELONLY",
              "description": "can only cancel the order but not place order"
            }
          ]
        }
      },
      "required": ["msg", "status"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-service-status-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-service-status-responseschema">Response Schema</h3>

Status Code **200**

| Name      | Type   | Required | Restrictions | Description                                                                                                                             |
| --------- | ------ | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| » code    | string | true     | none         | none                                                                                                                                    |
| » data    | object | true     | none         | none                                                                                                                                    |
| »» msg    | string | true     | none         | none                                                                                                                                    |
| »» status | string | true     | none         | Status of service: open: normal transaction; close: Stop Trading/Maintenance; cancelonly: can only cancel the order but not place order |

#### Enumerated Values

| Property | Value      |
| -------- | ---------- |
| status   | open       |
| status   | close      |
| status   | cancelonly |

<aside class="success">
This operation does not require authentication
</aside>

## Get Public Token - Futures

<a id="opId016"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/bullet-public", {
  method: "POST",

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

r = requests.post('/api/v1/bullet-public', headers = headers)

print(r.json())

```

`POST /api/v1/bullet-public`

This interface can obtain the token required for Websocket to establish a
Futures connection. If you need use public channels (e.g. all public market
data), please make request as follows to obtain the server list and public token

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
        "token": {
          "type": "string",
          "description": "The token required to establish a Websocket connection"
        },
        "instanceServers": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "endpoint": {
                "type": "string",
                "description": "Websocket domain URL. It is recommended to use a dynamic URL, as the URL may change."
              },
              "encrypt": {
                "type": "boolean",
                "description": "Whether to encrypt. Currently only supports wss, not ws"
              },
              "protocol": {
                "type": "string",
                "description": "Network Protocol",
                "enum": ["websocket"],
                "x-api-enum": [
                  {
                    "value": "websocket",
                    "name": "Websocket",
                    "description": "Websocket"
                  }
                ]
              },
              "pingInterval": {
                "type": "integer",
                "description": "Recommended ping interval (milliseconds)"
              },
              "pingTimeout": {
                "type": "integer",
                "description": "Heartbeat timeout (milliseconds)"
              }
            },
            "required": [
              "endpoint",
              "encrypt",
              "protocol",
              "pingInterval",
              "pingTimeout"
            ]
          }
        }
      },
      "required": ["token", "instanceServers"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-public-token---futures-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-public-token---futures-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type     | Required | Restrictions | Description                                                                          |
| ------------------ | -------- | -------- | ------------ | ------------------------------------------------------------------------------------ |
| » code             | string   | true     | none         | none                                                                                 |
| » data             | object   | true     | none         | none                                                                                 |
| »» token           | string   | true     | none         | The token required to establish a Websocket connection                               |
| »» instanceServers | [object] | true     | none         | none                                                                                 |
| »»» endpoint       | string   | true     | none         | Websocket domain URL. It is recommended to use a dynamic URL, as the URL may change. |
| »»» encrypt        | boolean  | true     | none         | Whether to encrypt. Currently only supports wss, not ws                              |
| »»» protocol       | string   | true     | none         | Network Protocol                                                                     |
| »»» pingInterval   | integer  | true     | none         | Recommended ping interval (milliseconds)                                             |
| »»» pingTimeout    | integer  | true     | none         | Heartbeat timeout (milliseconds)                                                     |

#### Enumerated Values

| Property | Value     |
| -------- | --------- |
| protocol | websocket |

<aside class="success">
This operation does not require authentication
</aside>

## Get Private Token - Futures

<a id="opId017"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/bullet-private", {
  method: "POST",

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

r = requests.post('/api/v1/bullet-private', headers = headers)

print(r.json())

```

`POST /api/v1/bullet-private`

This interface can obtain the token required for Websocket to establish a
Futures private connection. If you need use private channels (e.g. account
balance notice), please make request as follows to obtain the server list and
private token

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
        "token": {
          "type": "string",
          "description": "The token required to establish a Websocket connection"
        },
        "instanceServers": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "endpoint": {
                "type": "string",
                "description": "Websocket domain URL. It is recommended to use a dynamic URL, as the URL may change."
              },
              "encrypt": {
                "type": "boolean",
                "description": "Whether to encrypt. Currently only supports wss, not ws"
              },
              "protocol": {
                "type": "string",
                "description": "Network Protocol",
                "enum": ["websocket"],
                "x-api-enum": [
                  {
                    "value": "websocket",
                    "name": "Websocket",
                    "description": "Websocket"
                  }
                ]
              },
              "pingInterval": {
                "type": "integer",
                "description": "Recommended ping interval (milliseconds)"
              },
              "pingTimeout": {
                "type": "integer",
                "description": "Heartbeat timeout (milliseconds)"
              }
            },
            "required": [
              "protocol",
              "pingInterval",
              "pingTimeout",
              "encrypt",
              "endpoint"
            ]
          }
        }
      },
      "required": ["token", "instanceServers"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-private-token---futures-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-private-token---futures-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type     | Required | Restrictions | Description                                                                          |
| ------------------ | -------- | -------- | ------------ | ------------------------------------------------------------------------------------ |
| » code             | string   | true     | none         | none                                                                                 |
| » data             | object   | true     | none         | none                                                                                 |
| »» token           | string   | true     | none         | The token required to establish a Websocket connection                               |
| »» instanceServers | [object] | true     | none         | none                                                                                 |
| »»» endpoint       | string   | true     | none         | Websocket domain URL. It is recommended to use a dynamic URL, as the URL may change. |
| »»» encrypt        | boolean  | true     | none         | Whether to encrypt. Currently only supports wss, not ws                              |
| »»» protocol       | string   | true     | none         | Network Protocol                                                                     |
| »»» pingInterval   | integer  | true     | none         | Recommended ping interval (milliseconds)                                             |
| »»» pingTimeout    | integer  | true     | none         | Heartbeat timeout (milliseconds)                                                     |

#### Enumerated Values

| Property | Value     |
| -------- | --------- |
| protocol | websocket |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
