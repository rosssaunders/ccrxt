---
title: KuCoin Futures REST API v1.0.0
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

<h1 id="kucoin-futures-rest-api">KuCoin Futures REST API v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

market

<h1 id="kucoin-futures-rest-api-default">Default</h1>

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

<a id="opId017"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/fills", {
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

r = requests.get('/api/v1/fills', headers = headers)

print(r.json())

```

`GET /api/v1/fills`

Get a list of recent fills. If you need to get your recent trade history with
low latency, please query endpoint Get List of Orders Completed in 24h. The
requested data is not real-time.

<h3 id="get-trade-history-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                          |
| ----------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| orderId     | query | string         | false    | List fills for a specific order only (if you specify orderId, other parameters can be ignored)                                                       |
| symbol      | query | string         | false    | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                   |
| side        | query | string         | false    | Order side                                                                                                                                           |
| type        | query | string         | false    | Order Type                                                                                                                                           |
| tradeTypes  | query | string         | false    | Transaction type: trade, adl, liquid, settlement. Supports querying multiple types at the same time, separated by commas. Query all types when empty |
| startAt     | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                            |
| endAt       | query | integer(int64) | false    | End time (milliseconds)                                                                                                                              |
| currentPage | query | integer        | false    | Current request page. The default currentPage is 1                                                                                                   |
| pageSize    | query | integer        | false    | pageSize, The default pageSize is 50; the maximum cannot exceed 1000                                                                                 |

#### Enumerated Values

| Parameter | Value       |
| --------- | ----------- |
| side      | buy         |
| side      | sell        |
| type      | limit       |
| type      | market      |
| type      | limit_stop  |
| type      | market_stop |

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
          "type": "integer"
        },
        "pageSize": {
          "type": "integer"
        },
        "totalNum": {
          "type": "integer"
        },
        "totalPage": {
          "type": "integer"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "tradeId": {
                "type": "string",
                "description": "Trade ID\n"
              },
              "orderId": {
                "type": "string",
                "description": "Order ID\n"
              },
              "side": {
                "type": "string",
                "description": "Transaction side",
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
              "liquidity": {
                "type": "string",
                "description": "Liquidity-taker or -maker",
                "enum": ["taker", "maker"],
                "x-api-enum": [
                  {
                    "value": "taker",
                    "name": "taker",
                    "description": "taker"
                  },
                  {
                    "value": "maker",
                    "name": "maker",
                    "description": "maker"
                  }
                ]
              },
              "forceTaker": {
                "type": "boolean",
                "description": "Whether to force processing as a taker\n"
              },
              "price": {
                "type": "string",
                "description": "Filled price"
              },
              "size": {
                "type": "integer",
                "description": "Filled amount"
              },
              "value": {
                "type": "string",
                "description": "Order value"
              },
              "openFeePay": {
                "type": "string",
                "description": "Opening transaction fee"
              },
              "closeFeePay": {
                "type": "string",
                "description": "Closing transaction fee"
              },
              "stop": {
                "type": "string",
                "description": "A mark to the stop order type",
                "enum": ["down", "up", ""],
                "x-api-enum": [
                  {
                    "value": "down",
                    "name": "down",
                    "description": "Triggers when the price reaches or goes below the stopPrice."
                  },
                  {
                    "value": "up",
                    "name": "up",
                    "description": "Triggers when the price reaches or goes above the stopPrice."
                  },
                  {
                    "value": "",
                    "name": "None",
                    "description": "Not a stop order"
                  }
                ]
              },
              "feeRate": {
                "type": "string",
                "description": "Fee Rate"
              },
              "fixFee": {
                "type": "string",
                "description": "Fixed fees (Deprecated field, no actual use of the value field)"
              },
              "feeCurrency": {
                "type": "string",
                "description": "Charging currency"
              },
              "tradeTime": {
                "type": "integer",
                "description": "Trade time in nanoseconds",
                "format": "int64"
              },
              "subTradeType": {
                "type": "string",
                "description": "Deprecated field, no actual use of the value field"
              },
              "marginMode": {
                "type": "string",
                "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).",
                "enum": ["ISOLATED", "CROSS"],
                "x-api-enum": [
                  {
                    "value": "ISOLATED",
                    "name": "ISOLATED",
                    "description": "Isolated margin"
                  },
                  {
                    "value": "CROSS",
                    "name": "CROSS",
                    "description": "Cross margin"
                  }
                ]
              },
              "settleCurrency": {
                "type": "string",
                "description": "Settle currency"
              },
              "displayType": {
                "type": "string",
                "description": "Order type",
                "enum": ["limit", "market", "limit_stop", "market_stop"],
                "x-api-enum": [
                  {
                    "value": "limit",
                    "name": "limit",
                    "description": "limit order"
                  },
                  {
                    "value": "market",
                    "name": "market",
                    "description": "market order"
                  },
                  {
                    "value": "limit_stop",
                    "name": "limit_stop",
                    "description": "stop limit order"
                  },
                  {
                    "value": "market_stop",
                    "name": "market_stop",
                    "description": "stop market order"
                  }
                ]
              },
              "fee": {
                "type": "string",
                "description": "Trading fee"
              },
              "orderType": {
                "type": "string",
                "description": "Order type",
                "enum": ["market", "limit"],
                "x-api-enum": [
                  {
                    "value": "market",
                    "name": "market",
                    "description": "market"
                  },
                  {
                    "value": "limit",
                    "name": "limit",
                    "description": "limit"
                  }
                ]
              },
              "tradeType": {
                "type": "string",
                "description": "Trade type (trade, liquid, adl or settlement)\n",
                "enum": ["trade", "liquid", "adl", "settlement"],
                "x-api-enum": [
                  {
                    "value": "trade",
                    "name": "trade",
                    "description": "trade"
                  },
                  {
                    "value": "liquid",
                    "name": "liquid",
                    "description": "liquid"
                  },
                  {
                    "value": "adl",
                    "name": "adl",
                    "description": "adl"
                  },
                  {
                    "value": "settlement",
                    "name": "settlement",
                    "description": "settlement"
                  }
                ]
              },
              "createdAt": {
                "type": "integer",
                "description": "Order creation time\n",
                "format": "int64"
              },
              "openFeeTaxPay": {
                "type": "string",
                "description": "Opening tax fee (Only KYC users in some regions have this parameter)"
              },
              "closeFeeTaxPay": {
                "type": "string",
                "description": "Close tax fee (Only KYC users in some regions have this parameter)"
              }
            },
            "required": [
              "symbol",
              "tradeId",
              "orderId",
              "side",
              "liquidity",
              "forceTaker",
              "price",
              "size",
              "value",
              "openFeePay",
              "closeFeePay",
              "stop",
              "feeRate",
              "fixFee",
              "feeCurrency",
              "tradeTime",
              "subTradeType",
              "marginMode",
              "openFeeTaxPay",
              "closeFeeTaxPay",
              "displayType",
              "fee",
              "settleCurrency",
              "orderType",
              "tradeType",
              "createdAt"
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

<h3 id="get-trade-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-trade-history-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                        |
| ------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code             | string         | true     | none         | none                                                                                                               |
| » data             | object         | true     | none         | none                                                                                                               |
| »» currentPage     | integer        | true     | none         | none                                                                                                               |
| »» pageSize        | integer        | true     | none         | none                                                                                                               |
| »» totalNum        | integer        | true     | none         | none                                                                                                               |
| »» totalPage       | integer        | true     | none         | none                                                                                                               |
| »» items           | [object]       | true     | none         | none                                                                                                               |
| »»» symbol         | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »»» tradeId        | string         | true     | none         | Trade ID                                                                                                           |
| »»» orderId        | string         | true     | none         | Order ID                                                                                                           |
| »»» side           | string         | true     | none         | Transaction side                                                                                                   |
| »»» liquidity      | string         | true     | none         | Liquidity-taker or -maker                                                                                          |
| »»» forceTaker     | boolean        | true     | none         | Whether to force processing as a taker                                                                             |
| »»» price          | string         | true     | none         | Filled price                                                                                                       |
| »»» size           | integer        | true     | none         | Filled amount                                                                                                      |
| »»» value          | string         | true     | none         | Order value                                                                                                        |
| »»» openFeePay     | string         | true     | none         | Opening transaction fee                                                                                            |
| »»» closeFeePay    | string         | true     | none         | Closing transaction fee                                                                                            |
| »»» stop           | string         | true     | none         | A mark to the stop order type                                                                                      |
| »»» feeRate        | string         | true     | none         | Fee Rate                                                                                                           |
| »»» fixFee         | string         | true     | none         | Fixed fees (Deprecated field, no actual use of the value field)                                                    |
| »»» feeCurrency    | string         | true     | none         | Charging currency                                                                                                  |
| »»» tradeTime      | integer(int64) | true     | none         | Trade time in nanoseconds                                                                                          |
| »»» subTradeType   | string         | true     | none         | Deprecated field, no actual use of the value field                                                                 |
| »»» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                            |
| »»» settleCurrency | string         | true     | none         | Settle currency                                                                                                    |
| »»» displayType    | string         | true     | none         | Order type                                                                                                         |
| »»» fee            | string         | true     | none         | Trading fee                                                                                                        |
| »»» orderType      | string         | true     | none         | Order type                                                                                                         |
| »»» tradeType      | string         | true     | none         | Trade type (trade, liquid, adl or settlement)                                                                      |
| »»» createdAt      | integer(int64) | true     | none         | Order creation time                                                                                                |
| »»» openFeeTaxPay  | string         | true     | none         | Opening tax fee (Only KYC users in some regions have this parameter)                                               |
| »»» closeFeeTaxPay | string         | true     | none         | Close tax fee (Only KYC users in some regions have this parameter)                                                 |

#### Enumerated Values

| Property    | Value       |
| ----------- | ----------- |
| side        | buy         |
| side        | sell        |
| liquidity   | taker       |
| liquidity   | maker       |
| stop        | down        |
| stop        | up          |
| stop        |             |
| marginMode  | ISOLATED    |
| marginMode  | CROSS       |
| displayType | limit       |
| displayType | market      |
| displayType | limit_stop  |
| displayType | market_stop |
| orderType   | market      |
| orderType   | limit       |
| tradeType   | trade       |
| tradeType   | liquid      |
| tradeType   | adl         |
| tradeType   | settlement  |

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

## Add Order

<a id="opId001"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Unique order ID created by users to identify their orders. The maximum length cannot exceed 40, e.g. UUID only allows numbers, characters, underline(_), and separator (-).",
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
      "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": [
        "XBTUSDTM"
      ]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [
        3
      ]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order",
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
      ],
      "example": [
        "limit"
      ]
    },
    "remark": {
      "type": "string",
      "description": "Remark for the order: Length cannot exceed 100 utf8 characters"
    },
    "stop": {
      "type": "string",
      "description": "Either 'down' or 'up'.  If stop is used, parameter stopPrice and stopPriceType also need to be provided.",
      "enum": [
        "down",
        "up"
      ],
      "x-api-enum": [
        {
          "value": "down",
          "name": "down",
          "description": "Triggers when the price reaches or goes below the stopPrice."
        },
        {
          "value": "up",
          "name": "up",
          "description": "Triggers when the price reaches or goes above the stopPrice."
        }
      ]
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
      "enum": [
        "TP",
        "MP",
        "IP"
      ],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
        }
      ]
    },
    "stopPrice": {
      "type": "string",
      "description": "Needs to be defined if stop is specified. "
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to force-hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will force-freeze a certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a way that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. DC not currently supported.",
      "enum": [
        "CN",
        "CO",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": [
        "ISOLATED",
        "CROSS"
      ],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin"
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": [
        "0.1"
      ]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (lot), must be a positive integer. The quantity unit of coin-swap contracts is size (lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": [
        "GTC",
        "IOC"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "Order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "Being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, post only flag, invalid when timeInForce is IOC. When postOnly is true, choosing hidden or iceberg is not allowed. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fees, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden is chosen, choosing postOnly is not allowed.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg is chosen, choosing postOnly is not allowed.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, the maximum visible size of an iceberg order. Please place order in size (lots). The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**. Order size (base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size (lot), which is not supported."
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**. Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size (lot), which is not supported."
    }
  },
  "required": [
    "clientOid",
    "symbol",
    "side",
    "leverage"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/orders',
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

r = requests.post('/api/v1/orders', headers = headers)

print(r.json())

```

`POST /api/v1/orders`

Place order in the futures trading system. You can place two major types of
order: Limit and market. Orders can only be placed if your account has
sufficient funds. Once an order is placed, your funds will be put on hold for
the duration of the order. The amount of funds on hold depends on the order type
and parameters specified.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Unique order ID created by users to identify their orders. The maximum length cannot exceed 40, e.g. UUID only allows numbers, characters, underline(_), and separator (-).",
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
      "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": ["XBTUSDTM"]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [3]
    },
    "type": {
      "type": "string",
      "description": "Specify if the order is a 'limit' order or 'market' order",
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
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "Remark for the order: Length cannot exceed 100 utf8 characters"
    },
    "stop": {
      "type": "string",
      "description": "Either 'down' or 'up'.  If stop is used, parameter stopPrice and stopPriceType also need to be provided.",
      "enum": ["down", "up"],
      "x-api-enum": [
        {
          "value": "down",
          "name": "down",
          "description": "Triggers when the price reaches or goes below the stopPrice."
        },
        {
          "value": "up",
          "name": "up",
          "description": "Triggers when the price reaches or goes above the stopPrice."
        }
      ]
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
      "enum": ["TP", "MP", "IP"],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
        }
      ]
    },
    "stopPrice": {
      "type": "string",
      "description": "Needs to be defined if stop is specified. "
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to force-hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will force-freeze a certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a way that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. DC not currently supported.",
      "enum": ["CN", "CO", "CB"],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": ["ISOLATED", "CROSS"],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "ISOLATED",
          "description": "Isolated Margin"
        },
        {
          "value": "CROSS",
          "name": "CROSS",
          "description": "Cross Margin"
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": ["0.1"]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (lot), must be a positive integer. The quantity unit of coin-swap contracts is size (lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": ["GTC", "IOC"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "Order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "Being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, post only flag, invalid when timeInForce is IOC. When postOnly is true, choosing hidden or iceberg is not allowed. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fees, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden is chosen, choosing postOnly is not allowed.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg is chosen, choosing postOnly is not allowed.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, the maximum visible size of an iceberg order. Please place order in size (lots). The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**. Order size (base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size (lot), which is not supported."
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**. Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size (lot), which is not supported."
    }
  },
  "required": ["clientOid", "symbol", "side", "leverage"]
}
```

<h3 id="add-order-parameters">Parameters</h3>

| Name            | In   | Type    | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| --------------- | ---- | ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body            | body | object  | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » clientOid     | body | string  | true     | Unique order ID created by users to identify their orders. The maximum length cannot exceed 40, e.g. UUID only allows numbers, characters, underline(\_), and separator (-).                                                                                                                                                                                                                                                                                                                                                |
| » side          | body | string  | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » symbol        | body | string  | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                                                                                                                                                                                                                                                                          |
| » leverage      | body | integer | true     | Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.                                                                                                                                                                                                                                                                                                                                                                                                  |
| » type          | body | string  | false    | Specify if the order is a 'limit' order or 'market' order                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| » remark        | body | string  | false    | Remark for the order: Length cannot exceed 100 utf8 characters                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » stop          | body | string  | false    | Either 'down' or 'up'. If stop is used, parameter stopPrice and stopPriceType also need to be provided.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stopPriceType | body | string  | false    | Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » stopPrice     | body | string  | false    | Needs to be defined if stop is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| » reduceOnly    | body | boolean | false    | A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.                                                                                                                                                                                                                             |
| » closeOrder    | body | boolean | false    | A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.                                                                                                                                                                                                                                                |
| » forceHold     | body | boolean | false    | A mark to force-hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will force-freeze a certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a way that not enough funds are frozen for the order. |
| » stp           | body | string  | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. DC not currently supported.                                                                                                                                                                                                                                                                                                                                                                               |
| » marginMode    | body | string  | false    | Margin mode: ISOLATED, CROSS, default: ISOLATED                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » price         | body | string  | false    | Required for type is 'limit' order, indicating the operating price                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » size          | body | integer | false    | **Choose one of size, qty, valueQty**, Order size (lot), must be a positive integer. The quantity unit of coin-swap contracts is size (lot), and other units are not supported.                                                                                                                                                                                                                                                                                                                                             |
| » timeInForce   | body | string  | false    | Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC                                                                                                                                                                                                                                                                                                                                                                   |
| » postOnly      | body | boolean | false    | Optional for type is 'limit' order, post only flag, invalid when timeInForce is IOC. When postOnly is true, choosing hidden or iceberg is not allowed. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fees, the order will be fully rejected.                                                                                                                                                                |
| » hidden        | body | boolean | false    | Optional for type is 'limit' order, orders not displaying in order book. When hidden is chosen, choosing postOnly is not allowed.                                                                                                                                                                                                                                                                                                                                                                                           |
| » iceberg       | body | boolean | false    | Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg is chosen, choosing postOnly is not allowed.                                                                                                                                                                                                                                                                                                                                                             |
| » visibleSize   | body | string  | false    | Optional for type is 'limit' order, the maximum visible size of an iceberg order. Please place order in size (lots). The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified.                                                                                                                                                                                                                                                                                   |
| » qty           | body | string  | false    | **Choose one of size, qty, valueQty**. Order size (base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size (lot), which is not supported.                                                                                                                                                                                                                                                                                                                               |
| » valueQty      | body | string  | false    | **Choose one of size, qty, valueQty**. Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size (lot), which is not supported.                                                                                                                                                                                                                                                                                                                                               |

#### Enumerated Values

| Parameter       | Value    |
| --------------- | -------- |
| » side          | buy      |
| » side          | sell     |
| » type          | limit    |
| » type          | market   |
| » stop          | down     |
| » stop          | up       |
| » stopPriceType | TP       |
| » stopPriceType | MP       |
| » stopPriceType | IP       |
| » stp           | CN       |
| » stp           | CO       |
| » stp           | CB       |
| » marginMode    | ISOLATED |
| » marginMode    | CROSS    |
| » timeInForce   | GTC      |
| » timeInForce   | IOC      |

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
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        }
      },
      "required": ["orderId", "clientOid"]
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

| Name         | Type   | Required | Restrictions | Description                                                                                                                   |
| ------------ | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code       | string | true     | none         | none                                                                                                                          |
| » data       | object | true     | none         | none                                                                                                                          |
| »» orderId   | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» clientOid | string | true     | none         | The user self-defined order ID.                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order List

<a id="opId012"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/orders", {
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

r = requests.get('/api/v1/orders', headers = headers)

print(r.json())

```

`GET /api/v1/orders`

List your current orders.

<h3 id="get-order-list-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                        |
| ----------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| status      | query | string         | false    | active or done, done as default. Only list orders for a specific status                                            |
| symbol      | query | string         | false    | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| side        | query | string         | false    | buy or sell                                                                                                        |
| type        | query | string         | false    | Order Type                                                                                                         |
| startAt     | query | integer(int64) | false    | Start time (milisecond)                                                                                            |
| endAt       | query | integer(int64) | false    | End time (milisecond)                                                                                              |
| currentPage | query | integer        | false    | Current request page, The default currentPage is 1                                                                 |
| pageSize    | query | integer        | false    | pageSize, The default pageSize is 50, The maximum cannot exceed 1000                                               |

#### Enumerated Values

| Parameter | Value       |
| --------- | ----------- |
| status    | active      |
| status    | done        |
| side      | buy         |
| side      | sell        |
| type      | limit       |
| type      | market      |
| type      | limit_stop  |
| type      | market_stop |
| type      | oco_limit   |
| type      | oco_stop    |

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
          "description": "Current request page, The default currentPage is 1"
        },
        "pageSize": {
          "type": "integer",
          "description": "pageSize, The default pageSize is 50, The maximum cannot exceed 1000"
        },
        "totalNum": {
          "type": "integer"
        },
        "totalPage": {
          "type": "integer"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string",
                "description": "Order ID"
              },
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "type": {
                "type": "string",
                "description": "Order type, market order or limit order\n"
              },
              "side": {
                "type": "string",
                "description": "Transaction side"
              },
              "price": {
                "type": "string",
                "description": "Order price"
              },
              "size": {
                "type": "integer",
                "description": "Order quantity"
              },
              "value": {
                "type": "string",
                "description": "Order value"
              },
              "dealValue": {
                "type": "string",
                "description": "Executed size of funds\n"
              },
              "dealSize": {
                "type": "integer",
                "description": "Executed quantity"
              },
              "stp": {
                "type": "string",
                "description": "self trade prevention"
              },
              "stop": {
                "type": "string",
                "description": "A mark to the stop order type",
                "enum": ["down", "up", ""],
                "x-api-enum": [
                  {
                    "value": "down",
                    "name": "down",
                    "description": "Triggers when the price reaches or goes below the stopPrice."
                  },
                  {
                    "value": "up",
                    "name": "up",
                    "description": "Triggers when the price reaches or goes above the stopPrice."
                  },
                  {
                    "value": "",
                    "name": "None",
                    "description": "Not a stop order"
                  }
                ]
              },
              "stopPriceType": {
                "type": "string",
                "description": "Trigger price type of stop orders",
                "enum": ["TP", "MP", "IP", ""],
                "x-api-enum": [
                  {
                    "value": "TP",
                    "name": "trade price",
                    "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
                  },
                  {
                    "value": "MP",
                    "name": "mark price",
                    "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
                  },
                  {
                    "value": "IP",
                    "name": "index price",
                    "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
                  },
                  {
                    "value": "",
                    "name": "None",
                    "description": "Not a stop order"
                  }
                ]
              },
              "stopTriggered": {
                "type": "boolean",
                "description": "Mark to show whether the stop order is triggered"
              },
              "stopPrice": {
                "type": "integer",
                "description": "Trigger price of stop orders"
              },
              "timeInForce": {
                "type": "string",
                "description": "Time in force policy type"
              },
              "postOnly": {
                "type": "boolean",
                "description": "Mark of post only"
              },
              "hidden": {
                "type": "boolean",
                "description": "Mark of the hidden order"
              },
              "iceberg": {
                "type": "boolean",
                "description": "Mark of the iceberg order"
              },
              "leverage": {
                "type": "string",
                "description": "Leverage of the order"
              },
              "forceHold": {
                "type": "boolean",
                "description": "A mark to forcely hold the funds for an order"
              },
              "closeOrder": {
                "type": "boolean",
                "description": "A mark to close the position"
              },
              "visibleSize": {
                "type": "integer",
                "description": "Visible size of the iceberg order"
              },
              "clientOid": {
                "type": "string",
                "description": "Unique order id created by users to identify their orders"
              },
              "remark": {
                "type": "string",
                "description": "Remark of the order"
              },
              "tags": {
                "type": "string",
                "description": "tag order source"
              },
              "isActive": {
                "type": "boolean",
                "description": "Mark of the active orders"
              },
              "cancelExist": {
                "type": "boolean",
                "description": "Mark of the canceled orders"
              },
              "createdAt": {
                "type": "integer",
                "description": "Time the order created",
                "format": "int64"
              },
              "updatedAt": {
                "type": "integer",
                "description": "last update time",
                "format": "int64"
              },
              "endAt": {
                "type": "integer",
                "description": "End time",
                "format": "int64"
              },
              "orderTime": {
                "type": "integer",
                "description": "Order create time in nanosecond",
                "format": "int64"
              },
              "settleCurrency": {
                "type": "string",
                "description": "settlement currency"
              },
              "marginMode": {
                "type": "string",
                "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin)."
              },
              "avgDealPrice": {
                "type": "string",
                "description": "Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots * multiplier"
              },
              "status": {
                "type": "string",
                "description": "order status: “open” or “done”"
              },
              "filledSize": {
                "type": "integer",
                "description": "Value of the executed orders"
              },
              "filledValue": {
                "type": "string",
                "description": "Executed order quantity"
              },
              "reduceOnly": {
                "type": "boolean",
                "description": "A mark to reduce the position size only"
              }
            },
            "required": [
              "id",
              "symbol",
              "type",
              "side",
              "price",
              "size",
              "value",
              "dealValue",
              "dealSize",
              "stp",
              "stop",
              "stopPriceType",
              "stopTriggered",
              "stopPrice",
              "timeInForce",
              "postOnly",
              "hidden",
              "iceberg",
              "leverage",
              "forceHold",
              "closeOrder",
              "visibleSize",
              "clientOid",
              "remark",
              "tags",
              "isActive",
              "cancelExist",
              "createdAt",
              "updatedAt",
              "endAt",
              "orderTime",
              "settleCurrency",
              "marginMode",
              "avgDealPrice",
              "status",
              "filledSize",
              "filledValue",
              "reduceOnly"
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

<h3 id="get-order-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-order-list-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                |
| ------------------ | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                       |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» currentPage     | integer        | true     | none         | Current request page, The default currentPage is 1                                                                                                                                                                                                                         |
| »» pageSize        | integer        | true     | none         | pageSize, The default pageSize is 50, The maximum cannot exceed 1000                                                                                                                                                                                                       |
| »» totalNum        | integer        | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» totalPage       | integer        | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                       |
| »»» id             | string         | true     | none         | Order ID                                                                                                                                                                                                                                                                   |
| »»» symbol         | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                         |
| »»» type           | string         | true     | none         | Order type, market order or limit order                                                                                                                                                                                                                                    |
| »»» side           | string         | true     | none         | Transaction side                                                                                                                                                                                                                                                           |
| »»» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                |
| »»» size           | integer        | true     | none         | Order quantity                                                                                                                                                                                                                                                             |
| »»» value          | string         | true     | none         | Order value                                                                                                                                                                                                                                                                |
| »»» dealValue      | string         | true     | none         | Executed size of funds                                                                                                                                                                                                                                                     |
| »»» dealSize       | integer        | true     | none         | Executed quantity                                                                                                                                                                                                                                                          |
| »»» stp            | string         | true     | none         | self trade prevention                                                                                                                                                                                                                                                      |
| »»» stop           | string         | true     | none         | A mark to the stop order type                                                                                                                                                                                                                                              |
| »»» stopPriceType  | string         | true     | none         | Trigger price type of stop orders                                                                                                                                                                                                                                          |
| »»» stopTriggered  | boolean        | true     | none         | Mark to show whether the stop order is triggered                                                                                                                                                                                                                           |
| »»» stopPrice      | integer        | true     | none         | Trigger price of stop orders                                                                                                                                                                                                                                               |
| »»» timeInForce    | string         | true     | none         | Time in force policy type                                                                                                                                                                                                                                                  |
| »»» postOnly       | boolean        | true     | none         | Mark of post only                                                                                                                                                                                                                                                          |
| »»» hidden         | boolean        | true     | none         | Mark of the hidden order                                                                                                                                                                                                                                                   |
| »»» iceberg        | boolean        | true     | none         | Mark of the iceberg order                                                                                                                                                                                                                                                  |
| »»» leverage       | string         | true     | none         | Leverage of the order                                                                                                                                                                                                                                                      |
| »»» forceHold      | boolean        | true     | none         | A mark to forcely hold the funds for an order                                                                                                                                                                                                                              |
| »»» closeOrder     | boolean        | true     | none         | A mark to close the position                                                                                                                                                                                                                                               |
| »»» visibleSize    | integer        | true     | none         | Visible size of the iceberg order                                                                                                                                                                                                                                          |
| »»» clientOid      | string         | true     | none         | Unique order id created by users to identify their orders                                                                                                                                                                                                                  |
| »»» remark         | string         | true     | none         | Remark of the order                                                                                                                                                                                                                                                        |
| »»» tags           | string         | true     | none         | tag order source                                                                                                                                                                                                                                                           |
| »»» isActive       | boolean        | true     | none         | Mark of the active orders                                                                                                                                                                                                                                                  |
| »»» cancelExist    | boolean        | true     | none         | Mark of the canceled orders                                                                                                                                                                                                                                                |
| »»» createdAt      | integer(int64) | true     | none         | Time the order created                                                                                                                                                                                                                                                     |
| »»» updatedAt      | integer(int64) | true     | none         | last update time                                                                                                                                                                                                                                                           |
| »»» endAt          | integer(int64) | true     | none         | End time                                                                                                                                                                                                                                                                   |
| »»» orderTime      | integer(int64) | true     | none         | Order create time in nanosecond                                                                                                                                                                                                                                            |
| »»» settleCurrency | string         | true     | none         | settlement currency                                                                                                                                                                                                                                                        |
| »»» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                                                                                                                                                                                    |
| »»» avgDealPrice   | string         | true     | none         | Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots \* multiplier |
| »»» status         | string         | true     | none         | order status: “open” or “done”                                                                                                                                                                                                                                             |
| »»» filledSize     | integer        | true     | none         | Value of the executed orders                                                                                                                                                                                                                                               |
| »»» filledValue    | string         | true     | none         | Executed order quantity                                                                                                                                                                                                                                                    |
| »»» reduceOnly     | boolean        | true     | none         | A mark to reduce the position size only                                                                                                                                                                                                                                    |

#### Enumerated Values

| Property      | Value |
| ------------- | ----- |
| stop          | down  |
| stop          | up    |
| stop          |       |
| stopPriceType | TP    |
| stopPriceType | MP    |
| stopPriceType | IP    |
| stopPriceType |       |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel All Orders - V1

<a id="opId018"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/orders", {
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

r = requests.delete('/api/v1/orders', headers = headers)

print(r.json())

```

`DELETE /api/v1/orders`

Cancel all open orders (excluding stop orders). The response is a list of
orderIDs of the canceled orders.

<h3 id="cancel-all-orders---v1-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                                                                                                                       |
| ------ | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol | query | string | false    | To cancel all limit orders for a specific contract only, unless otherwise specified, all limit orders will be deleted. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
        "cancelledOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Unique ID of the canceled order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-all-orders---v1-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-all-orders---v1-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                     |
| -------------------- | -------- | -------- | ------------ | ------------------------------- |
| » code               | string   | true     | none         | none                            |
| » data               | object   | true     | none         | none                            |
| »» cancelledOrderIds | [string] | true     | none         | Unique ID of the canceled order |

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
      "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "specify if the order is to 'buy' or 'sell'",
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
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": [
        "XBTUSDTM"
      ]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [
        3
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order",
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
      ],
      "example": [
        "limit"
      ]
    },
    "remark": {
      "type": "string",
      "description": "remark for the order, length cannot exceed 100 utf8 characters"
    },
    "stop": {
      "type": "string",
      "description": "Either 'down' or 'up'.  If stop is used,parameter stopPrice and stopPriceType also need to be provieded.",
      "enum": [
        "down",
        "up"
      ],
      "x-api-enum": [
        {
          "value": "down",
          "name": "down",
          "description": "Triggers when the price reaches or goes below the stopPrice."
        },
        {
          "value": "up",
          "name": "up",
          "description": "Triggers when the price reaches or goes above the stopPrice"
        }
      ]
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
      "enum": [
        "TP",
        "MP",
        "IP"
      ],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
        }
      ]
    },
    "stopPrice": {
      "type": "string",
      "description": "Need to be defined if stop is specified. "
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
      "enum": [
        "CN",
        "CO",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": [
        "ISOLATED",
        "CROSS"
      ],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "",
          "description": ""
        },
        {
          "value": "CROSS",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": [
        "0.1"
      ]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": [
        "GTC",
        "IOC"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
    }
  },
  "required": [
    "clientOid",
    "symbol",
    "type",
    "side",
    "leverage"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/orders/test',
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

r = requests.post('/api/v1/orders/test', headers = headers)

print(r.json())

```

`POST /api/v1/orders/test`

Place order to the futures trading system just for validation

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "specify if the order is to 'buy' or 'sell'",
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
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": ["XBTUSDTM"]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [3]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order",
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
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "remark for the order, length cannot exceed 100 utf8 characters"
    },
    "stop": {
      "type": "string",
      "description": "Either 'down' or 'up'.  If stop is used,parameter stopPrice and stopPriceType also need to be provieded.",
      "enum": ["down", "up"],
      "x-api-enum": [
        {
          "value": "down",
          "name": "down",
          "description": "Triggers when the price reaches or goes below the stopPrice."
        },
        {
          "value": "up",
          "name": "up",
          "description": "Triggers when the price reaches or goes above the stopPrice"
        }
      ]
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
      "enum": ["TP", "MP", "IP"],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
        }
      ]
    },
    "stopPrice": {
      "type": "string",
      "description": "Need to be defined if stop is specified. "
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
      "enum": ["CN", "CO", "CB"],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": ["ISOLATED", "CROSS"],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "",
          "description": ""
        },
        {
          "value": "CROSS",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": ["0.1"]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": ["GTC", "IOC"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
    }
  },
  "required": ["clientOid", "symbol", "type", "side", "leverage"]
}
```

<h3 id="add-order-test-parameters">Parameters</h3>

| Name            | In   | Type    | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------- | ---- | ------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body            | body | object  | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| » clientOid     | body | string  | true     | Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(\_), and separator(-)                                                                                                                                                                                                                                                                                                                                                            |
| » side          | body | string  | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » symbol        | body | string  | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » leverage      | body | integer | true     | Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.                                                                                                                                                                                                                                                                                                                                                                                                             |
| » type          | body | string  | true     | specify if the order is an 'limit' order or 'market' order                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » remark        | body | string  | false    | remark for the order, length cannot exceed 100 utf8 characters                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » stop          | body | string  | false    | Either 'down' or 'up'. If stop is used,parameter stopPrice and stopPriceType also need to be provieded.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » stopPriceType | body | string  | false    | Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » stopPrice     | body | string  | false    | Need to be defined if stop is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » reduceOnly    | body | boolean | false    | A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.                                                                                                                                                                                                                                        |
| » closeOrder    | body | boolean | false    | A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.                                                                                                                                                                                                                                                           |
| » forceHold     | body | boolean | false    | A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order. |
| » stp           | body | string  | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.                                                                                                                                                                                                                                                                                                                                                                                      |
| » marginMode    | body | string  | false    | Margin mode: ISOLATED, CROSS, default: ISOLATED                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » price         | body | string  | false    | Required for type is 'limit' order, indicating the operating price                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » size          | body | integer | false    | **Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported.                                                                                                                                                                                                                                                                                                                                                         |
| » timeInForce   | body | string  | false    | Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC                                                                                                                                                                                                                                                                                                                                                                              |
| » postOnly      | body | boolean | false    | Optional for type is 'limit' order, post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.                                                                                                                                                                                 |
| » hidden        | body | boolean | false    | Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.                                                                                                                                                                                                                                                                                                                                                                                                               |
| » iceberg       | body | boolean | false    | Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.                                                                                                                                                                                                                                                                                                                                                                                 |
| » visibleSize   | body | string  | false    | Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified.                                                                                                                                                                                                                                                                                              |
| » qty           | body | string  | false    | **Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported                                                                                                                                                                                                                                                                                                                                            |
| » valueQty      | body | string  | false    | **Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported                                                                                                                                                                                                                                                                                                                                                            |

#### Enumerated Values

| Parameter       | Value    |
| --------------- | -------- |
| » side          | buy      |
| » side          | sell     |
| » type          | limit    |
| » type          | market   |
| » stop          | down     |
| » stop          | up       |
| » stopPriceType | TP       |
| » stopPriceType | MP       |
| » stopPriceType | IP       |
| » stp           | CN       |
| » stp           | CO       |
| » stp           | CB       |
| » marginMode    | ISOLATED |
| » marginMode    | CROSS    |
| » timeInForce   | GTC      |
| » timeInForce   | IOC      |

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
          "description": "The unique order id generated by the trading system,which can be used later for further actions such as canceling the order."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order id."
        }
      },
      "required": ["orderId", "clientOid"]
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

| Name         | Type   | Required | Restrictions | Description                                                                                                                  |
| ------------ | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code       | string | true     | none         | none                                                                                                                         |
| » data       | object | true     | none         | none                                                                                                                         |
| »» orderId   | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |
| »» clientOid | string | true     | none         | The user self-defined order id.                                                                                              |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Add Orders

<a id="opId003"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "clientOid": {
        "type": "string",
        "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
        "example": [
          "5c52e11203aa677f33e493fb"
        ]
      },
      "side": {
        "type": "string",
        "description": "specify if the order is to 'buy' or 'sell'",
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
        "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-221752070) ",
        "example": [
          "XBTUSDTM"
        ]
      },
      "leverage": {
        "type": "integer",
        "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
        "example": [
          3
        ]
      },
      "type": {
        "type": "string",
        "description": "specify if the order is an 'limit' order or 'market' order",
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
        ],
        "example": [
          "limit"
        ]
      },
      "remark": {
        "type": "string",
        "description": "remark for the order, length cannot exceed 100 utf8 characters"
      },
      "stop": {
        "type": "string",
        "description": "Either 'down' or 'up'.  If stop is used,parameter stopPrice and stopPriceType also need to be provieded.",
        "enum": [
          "down",
          "up"
        ],
        "x-api-enum": [
          {
            "value": "down",
            "name": "down",
            "description": "Triggers when the price reaches or goes below the stopPrice."
          },
          {
            "value": "up",
            "name": "up",
            "description": "Triggers when the price reaches or goes above the stopPrice"
          }
        ]
      },
      "stopPriceType": {
        "type": "string",
        "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
        "enum": [
          "TP",
          "MP",
          "IP"
        ],
        "x-api-enum": [
          {
            "value": "TP",
            "name": "trade price",
            "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
          },
          {
            "value": "MP",
            "name": "mark price",
            "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
          },
          {
            "value": "IP",
            "name": "index price",
            "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
          }
        ]
      },
      "stopPrice": {
        "type": "string",
        "description": "Need to be defined if stop is specified. "
      },
      "reduceOnly": {
        "type": "boolean",
        "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
        "default": false
      },
      "closeOrder": {
        "type": "boolean",
        "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
        "default": false
      },
      "forceHold": {
        "type": "boolean",
        "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
        "default": false
      },
      "stp": {
        "type": "string",
        "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
        "enum": [
          "CN",
          "CO",
          "CB"
        ],
        "x-api-enum": [
          {
            "value": "CN",
            "name": "CN",
            "description": "Cancel new, Cancel the new order"
          },
          {
            "value": "CO",
            "name": "CO",
            "description": "Cancel old, Cancel the old order"
          },
          {
            "value": "CB",
            "name": "CB",
            "description": "Cancel both, Cancel both sides"
          }
        ]
      },
      "marginMode": {
        "type": "string",
        "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
        "enum": [
          "ISOLATED",
          "CROSS"
        ],
        "default": "ISOLATED",
        "x-api-enum": [
          {
            "value": "ISOLATED",
            "name": "",
            "description": ""
          },
          {
            "value": "CROSS",
            "name": "",
            "description": ""
          }
        ]
      },
      "price": {
        "type": "string",
        "description": "Required for type is 'limit' order, indicating the operating price",
        "example": [
          "0.1"
        ]
      },
      "size": {
        "type": "integer",
        "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
      },
      "timeInForce": {
        "type": "string",
        "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
        "enum": [
          "GTC",
          "IOC"
        ],
        "default": "GTC",
        "x-api-enum": [
          {
            "value": "GTC",
            "name": "Good Till Canceled",
            "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
          },
          {
            "value": "IOC",
            "name": "Immediate Or Cancel",
            "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
          }
        ]
      },
      "postOnly": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
        "default": false
      },
      "hidden": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
        "default": false
      },
      "iceberg": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
        "default": false
      },
      "visibleSize": {
        "type": "string",
        "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported."
      },
      "qty": {
        "type": "string",
        "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
      },
      "valueQty": {
        "type": "string",
        "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
      }
    },
    "required": [
      "clientOid",
      "symbol",
      "side",
      "leverage"
    ]
  }
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/orders/multi',
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

r = requests.post('/api/v1/orders/multi', headers = headers)

print(r.json())

```

`POST /api/v1/orders/multi`

Place multiple order to the futures trading system, you can place two major
types of orders: limit and market. Orders can only be placed if your account has
sufficient funds. Once an order is placed, your funds will be put on hold for
the duration of the order. The amount of funds on hold depends on the order type
and parameters specified. You can place up to 20 orders at one time, including
limit orders, market orders, and stop orders Please be noted that the system
would hold the fees from the orders entered the orderbook in advance.

> Body parameter

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "clientOid": {
        "type": "string",
        "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
        "example": ["5c52e11203aa677f33e493fb"]
      },
      "side": {
        "type": "string",
        "description": "specify if the order is to 'buy' or 'sell'",
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
        "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-221752070) ",
        "example": ["XBTUSDTM"]
      },
      "leverage": {
        "type": "integer",
        "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
        "example": [3]
      },
      "type": {
        "type": "string",
        "description": "specify if the order is an 'limit' order or 'market' order",
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
        ],
        "example": ["limit"]
      },
      "remark": {
        "type": "string",
        "description": "remark for the order, length cannot exceed 100 utf8 characters"
      },
      "stop": {
        "type": "string",
        "description": "Either 'down' or 'up'.  If stop is used,parameter stopPrice and stopPriceType also need to be provieded.",
        "enum": ["down", "up"],
        "x-api-enum": [
          {
            "value": "down",
            "name": "down",
            "description": "Triggers when the price reaches or goes below the stopPrice."
          },
          {
            "value": "up",
            "name": "up",
            "description": "Triggers when the price reaches or goes above the stopPrice"
          }
        ]
      },
      "stopPriceType": {
        "type": "string",
        "description": "Either 'TP', 'IP' or 'MP', Need to be defined if stop is specified.",
        "enum": ["TP", "MP", "IP"],
        "x-api-enum": [
          {
            "value": "TP",
            "name": "trade price",
            "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
          },
          {
            "value": "MP",
            "name": "mark price",
            "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
          },
          {
            "value": "IP",
            "name": "index price",
            "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
          }
        ]
      },
      "stopPrice": {
        "type": "string",
        "description": "Need to be defined if stop is specified. "
      },
      "reduceOnly": {
        "type": "boolean",
        "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
        "default": false
      },
      "closeOrder": {
        "type": "boolean",
        "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
        "default": false
      },
      "forceHold": {
        "type": "boolean",
        "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
        "default": false
      },
      "stp": {
        "type": "string",
        "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
        "enum": ["CN", "CO", "CB"],
        "x-api-enum": [
          {
            "value": "CN",
            "name": "CN",
            "description": "Cancel new, Cancel the new order"
          },
          {
            "value": "CO",
            "name": "CO",
            "description": "Cancel old, Cancel the old order"
          },
          {
            "value": "CB",
            "name": "CB",
            "description": "Cancel both, Cancel both sides"
          }
        ]
      },
      "marginMode": {
        "type": "string",
        "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
        "enum": ["ISOLATED", "CROSS"],
        "default": "ISOLATED",
        "x-api-enum": [
          {
            "value": "ISOLATED",
            "name": "",
            "description": ""
          },
          {
            "value": "CROSS",
            "name": "",
            "description": ""
          }
        ]
      },
      "price": {
        "type": "string",
        "description": "Required for type is 'limit' order, indicating the operating price",
        "example": ["0.1"]
      },
      "size": {
        "type": "integer",
        "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
      },
      "timeInForce": {
        "type": "string",
        "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
        "enum": ["GTC", "IOC"],
        "default": "GTC",
        "x-api-enum": [
          {
            "value": "GTC",
            "name": "Good Till Canceled",
            "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
          },
          {
            "value": "IOC",
            "name": "Immediate Or Cancel",
            "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
          }
        ]
      },
      "postOnly": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
        "default": false
      },
      "hidden": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
        "default": false
      },
      "iceberg": {
        "type": "boolean",
        "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
        "default": false
      },
      "visibleSize": {
        "type": "string",
        "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported."
      },
      "qty": {
        "type": "string",
        "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
      },
      "valueQty": {
        "type": "string",
        "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
      }
    },
    "required": ["clientOid", "symbol", "side", "leverage"]
  }
}
```

<h3 id="batch-add-orders-parameters">Parameters</h3>

| Name | In   | Type          | Required | Description |
| ---- | ---- | ------------- | -------- | ----------- |
| body | body | array[object] | false    | none        |

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
          "orderId": {
            "type": "string",
            "description": "The unique order id generated by the trading system,which can be used later for further actions such as canceling the order."
          },
          "clientOid": {
            "type": "string",
            "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)"
          },
          "symbol": {
            "type": "string",
            "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
          },
          "code": {
            "type": "string"
          },
          "msg": {
            "type": "string"
          }
        },
        "required": ["orderId", "clientOid", "symbol", "code", "msg"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-add-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-add-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type     | Required | Restrictions | Description                                                                                                                                                                 |
| ------------ | -------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code       | string   | true     | none         | none                                                                                                                                                                        |
| » data       | [object] | true     | none         | none                                                                                                                                                                        |
| »» orderId   | string   | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order.                                                |
| »» clientOid | string   | true     | none         | Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(\_), and separator(-) |
| »» symbol    | string   | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                          |
| »» code      | string   | true     | none         | none                                                                                                                                                                        |
| »» msg       | string   | true     | none         | none                                                                                                                                                                        |

<aside class="success">
This operation does not require authentication
</aside>

## Add Take Profit And Stop Loss Order

<a id="opId004"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
      "example": [
        "5c52e11203aa677f33e493fb"
      ]
    },
    "side": {
      "type": "string",
      "description": "specify if the order is to 'buy' or 'sell'",
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
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": [
        "XBTUSDTM"
      ]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [
        3
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order",
      "enum": [
        "limit",
        "market"
      ],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "limit",
          "description": "limit order"
        },
        {
          "value": "market",
          "name": "market",
          "description": "market order"
        }
      ],
      "example": [
        "limit"
      ]
    },
    "remark": {
      "type": "string",
      "description": "remark for the order, length cannot exceed 100 utf8 characters"
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP'",
      "enum": [
        "TP",
        "MP",
        "IP"
      ],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
        }
      ]
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
      "enum": [
        "CN",
        "CO",
        "CB"
      ],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": [
        "ISOLATED",
        "CROSS"
      ],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "",
          "description": ""
        },
        {
          "value": "CROSS",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": [
        "0.1"
      ]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": [
        "GTC",
        "IOC"
      ],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "triggerStopUpPrice": {
      "type": "string",
      "description": "Take profit price"
    },
    "triggerStopDownPrice": {
      "type": "string",
      "description": "Stop loss price"
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
    }
  },
  "required": [
    "clientOid",
    "symbol",
    "type",
    "side",
    "leverage"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/st-orders',
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

r = requests.post('/api/v1/st-orders', headers = headers)

print(r.json())

```

`POST /api/v1/st-orders`

Place take profit and stop loss order supports both take-profit and stop-loss
functions, and other functions are exactly the same as the place order
interface.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(_), and separator(-)",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "side": {
      "type": "string",
      "description": "specify if the order is to 'buy' or 'sell'",
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
      "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) ",
      "example": ["XBTUSDTM"]
    },
    "leverage": {
      "type": "integer",
      "description": "Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.",
      "example": [3]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order",
      "enum": ["limit", "market"],
      "default": "limit",
      "x-api-enum": [
        {
          "value": "limit",
          "name": "limit",
          "description": "limit order"
        },
        {
          "value": "market",
          "name": "market",
          "description": "market order"
        }
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "remark for the order, length cannot exceed 100 utf8 characters"
    },
    "stopPriceType": {
      "type": "string",
      "description": "Either 'TP', 'IP' or 'MP'",
      "enum": ["TP", "MP", "IP"],
      "x-api-enum": [
        {
          "value": "TP",
          "name": "trade price",
          "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
        },
        {
          "value": "MP",
          "name": "mark price",
          "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
        },
        {
          "value": "IP",
          "name": "index price",
          "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
        }
      ]
    },
    "reduceOnly": {
      "type": "boolean",
      "description": "A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.",
      "default": false
    },
    "closeOrder": {
      "type": "boolean",
      "description": "A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.",
      "default": false
    },
    "forceHold": {
      "type": "boolean",
      "description": "A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order.",
      "default": false
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
      "enum": ["CN", "CO", "CB"],
      "x-api-enum": [
        {
          "value": "CN",
          "name": "CN",
          "description": "Cancel new, Cancel the new order"
        },
        {
          "value": "CO",
          "name": "CO",
          "description": "Cancel old, Cancel the old order"
        },
        {
          "value": "CB",
          "name": "CB",
          "description": "Cancel both, Cancel both sides"
        }
      ]
    },
    "marginMode": {
      "type": "string",
      "description": "Margin mode: ISOLATED, CROSS, default: ISOLATED",
      "enum": ["ISOLATED", "CROSS"],
      "default": "ISOLATED",
      "x-api-enum": [
        {
          "value": "ISOLATED",
          "name": "",
          "description": ""
        },
        {
          "value": "CROSS",
          "name": "",
          "description": ""
        }
      ]
    },
    "price": {
      "type": "string",
      "description": "Required for type is 'limit' order, indicating the operating price",
      "example": ["0.1"]
    },
    "size": {
      "type": "integer",
      "description": "**Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported."
    },
    "timeInForce": {
      "type": "string",
      "description": "Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC",
      "enum": ["GTC", "IOC"],
      "default": "GTC",
      "x-api-enum": [
        {
          "value": "GTC",
          "name": "Good Till Canceled",
          "description": "order remains open on the order book until canceled. This is the default type if the field is left empty."
        },
        {
          "value": "IOC",
          "name": "Immediate Or Cancel",
          "description": "being matched or not, the remaining size of the order will be instantly canceled instead of entering the order book."
        }
      ]
    },
    "postOnly": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order,  post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.",
      "default": false
    },
    "hidden": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.",
      "default": false
    },
    "iceberg": {
      "type": "boolean",
      "description": "Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.",
      "default": false
    },
    "visibleSize": {
      "type": "string",
      "description": "Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified."
    },
    "triggerStopUpPrice": {
      "type": "string",
      "description": "Take profit price"
    },
    "triggerStopDownPrice": {
      "type": "string",
      "description": "Stop loss price"
    },
    "qty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported"
    },
    "valueQty": {
      "type": "string",
      "description": "**Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported"
    }
  },
  "required": ["clientOid", "symbol", "type", "side", "leverage"]
}
```

<h3 id="add-take-profit-and-stop-loss-order-parameters">Parameters</h3>

| Name                   | In   | Type    | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ---------------------- | ---- | ------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body                   | body | object  | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| » clientOid            | body | string  | true     | Unique order id created by users to identify their orders, the maximum length cannot exceed 40, e.g. UUID, Only allows numbers, characters, underline(\_), and separator(-)                                                                                                                                                                                                                                                                                                                                                            |
| » side                 | body | string  | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » symbol               | body | string  | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » leverage             | body | integer | true     | Used to calculate the margin to be frozen for the order. If you are to close the position, this parameter is not required.                                                                                                                                                                                                                                                                                                                                                                                                             |
| » type                 | body | string  | true     | specify if the order is an 'limit' order or 'market' order                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » remark               | body | string  | false    | remark for the order, length cannot exceed 100 utf8 characters                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » stopPriceType        | body | string  | false    | Either 'TP', 'IP' or 'MP'                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » reduceOnly           | body | boolean | false    | A mark to reduce the position size only. Set to false by default. Need to set the position size when reduceOnly is true. If set to true, only the orders reducing the position size will be executed. If the reduce-only order size exceeds the position size, the extra size will be canceled.                                                                                                                                                                                                                                        |
| » closeOrder           | body | boolean | false    | A mark to close the position. Set to false by default. If closeOrder is set to true, the system will close the position and the position size will become 0. Side, Size and Leverage fields can be left empty and the system will determine the side and size automatically.                                                                                                                                                                                                                                                           |
| » forceHold            | body | boolean | false    | A mark to forcely hold the funds for an order, even though it's an order to reduce the position size. This helps the order stay on the order book and not get canceled when the position size changes. Set to false by default. The system will forcely freeze certain amount of funds for this order, including orders whose direction is opposite to the current positions. This feature is to ensure that the order won’t be canceled by the matching engine in such a circumstance that not enough funds are frozen for the order. |
| » stp                  | body | string  | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.                                                                                                                                                                                                                                                                                                                                                                                      |
| » marginMode           | body | string  | false    | Margin mode: ISOLATED, CROSS, default: ISOLATED                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » price                | body | string  | false    | Required for type is 'limit' order, indicating the operating price                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » size                 | body | integer | false    | **Choose one of size, qty, valueQty**, Order size (Lot), must be a positive integer. The quantity unit of coin-swap contracts is size(lot), and other units are not supported.                                                                                                                                                                                                                                                                                                                                                         |
| » timeInForce          | body | string  | false    | Optional for type is 'limit' order, [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading, default is GTC                                                                                                                                                                                                                                                                                                                                                                              |
| » postOnly             | body | boolean | false    | Optional for type is 'limit' order, post only flag, invalid when timeInForce is IOC. When postOnly is true, not allowed choose hidden or iceberg. The post-only flag ensures that the trader always pays the maker fee and provides liquidity to the order book. If any part of the order is going to pay taker fee, the order will be fully rejected.                                                                                                                                                                                 |
| » hidden               | body | boolean | false    | Optional for type is 'limit' order, orders not displaying in order book. When hidden chose, not allowed choose postOnly.                                                                                                                                                                                                                                                                                                                                                                                                               |
| » iceberg              | body | boolean | false    | Optional for type is 'limit' order, Only visible portion of the order is displayed in the order book. When iceberg chose, not allowed choose postOnly.                                                                                                                                                                                                                                                                                                                                                                                 |
| » visibleSize          | body | string  | false    | Optional for type is 'limit' order, The maximum visible size of an iceberg order. please place order in size (lots), The units of qty (base currency) and valueQty (value) are not supported. Need to be defined if iceberg is specified.                                                                                                                                                                                                                                                                                              |
| » triggerStopUpPrice   | body | string  | false    | Take profit price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| » triggerStopDownPrice | body | string  | false    | Stop loss price                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » qty                  | body | string  | false    | **Choose one of size, qty, valueQty**, Order size (Base currency) must be an integer multiple of the multiplier. The unit of the quantity of coin-swap is size(lot), which is not supported                                                                                                                                                                                                                                                                                                                                            |
| » valueQty             | body | string  | false    | **Choose one of size, qty, valueQty**, Order size (Value), USDS-Swap correspond to USDT or USDC. The unit of the quantity of coin-swap is size(lot), which is not supported                                                                                                                                                                                                                                                                                                                                                            |

#### Enumerated Values

| Parameter       | Value    |
| --------------- | -------- |
| » side          | buy      |
| » side          | sell     |
| » type          | limit    |
| » type          | market   |
| » stopPriceType | TP       |
| » stopPriceType | MP       |
| » stopPriceType | IP       |
| » stp           | CN       |
| » stp           | CO       |
| » stp           | CB       |
| » marginMode    | ISOLATED |
| » marginMode    | CROSS    |
| » timeInForce   | GTC      |
| » timeInForce   | IOC      |

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
          "description": "The unique order id generated by the trading system,which can be used later for further actions such as canceling the order."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order id."
        }
      },
      "required": ["orderId", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-take-profit-and-stop-loss-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-take-profit-and-stop-loss-order-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type   | Required | Restrictions | Description                                                                                                                  |
| ------------ | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code       | string | true     | none         | none                                                                                                                         |
| » data       | object | true     | none         | none                                                                                                                         |
| »» orderId   | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |
| »» clientOid | string | true     | none         | The user self-defined order id.                                                                                              |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By OrderId

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/orders/{orderId}", {
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

r = requests.delete('/api/v1/orders/{orderId}', headers = headers)

print(r.json())

```

`DELETE /api/v1/orders/{orderId}`

Cancel order by system generated orderId.

<h3 id="cancel-order-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description |
| ------- | ---- | ------ | -------- | ----------- |
| orderId | path | string | true     | none        |

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
        "cancelledOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The orderId that is to be canceled"
        }
      },
      "required": ["cancelledOrderIds"]
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

| Name                 | Type     | Required | Restrictions | Description                        |
| -------------------- | -------- | -------- | ------------ | ---------------------------------- |
| » code               | string   | true     | none         | none                               |
| » data               | object   | true     | none         | none                               |
| »» cancelledOrderIds | [string] | true     | none         | The orderId that is to be canceled |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By ClientOid

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/orders/client-order/{clientOid}?symbol=type,string", {
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

r = requests.delete('/api/v1/orders/client-order/{clientOid}', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/orders/client-order/{clientOid}`

Cancel order by client defined orderId.

<h3 id="cancel-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                                        |
| --------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol    | query | string | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| clientOid | path  | string | true     | client order id                                                                                                    |

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
          "type": "string"
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

| Name         | Type   | Required | Restrictions | Description |
| ------------ | ------ | -------- | ------------ | ----------- |
| » code       | string | true     | none         | none        |
| » data       | object | true     | none         | none        |
| »» clientOid | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Cancel Orders

<a id="opId007"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "orderIdsList": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "the list of orderId"
    },
    "clientOidsList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
          },
          "clientOid": {
            "type": "string"
          }
        },
        "required": [
          "symbol",
          "clientOid"
        ]
      },
      "description": "the list of client orderId"
    }
  },
  "required": [
    "orderIdsList",
    "clientOidsList"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/orders/multi-cancel',
{
  method: 'DELETE',
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

r = requests.delete('/api/v1/orders/multi-cancel', headers = headers)

print(r.json())

```

`DELETE /api/v1/orders/multi-cancel`

Cancel a bach of orders by client defined orderId or system generated orderId

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "orderIdsList": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "description": "the list of orderId"
    },
    "clientOidsList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "symbol": {
            "type": "string",
            "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
          },
          "clientOid": {
            "type": "string"
          }
        },
        "required": ["symbol", "clientOid"]
      },
      "description": "the list of client orderId"
    }
  },
  "required": ["orderIdsList", "clientOidsList"]
}
```

<h3 id="batch-cancel-orders-parameters">Parameters</h3>

| Name             | In   | Type     | Required | Description                                                                                                        |
| ---------------- | ---- | -------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| body             | body | object   | false    | none                                                                                                               |
| » orderIdsList   | body | [string] | true     | the list of orderId                                                                                                |
| » clientOidsList | body | [object] | true     | the list of client orderId                                                                                         |
| »» symbol        | body | string   | true     | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» clientOid     | body | string   | true     | none                                                                                                               |

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
          "orderId": {
            "type": "string"
          },
          "clientOid": {
            "type": "string"
          },
          "code": {
            "type": "string"
          },
          "msg": {
            "type": "string"
          }
        },
        "required": ["orderId", "clientOid", "code", "msg"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-cancel-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-cancel-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type     | Required | Restrictions | Description |
| ------------ | -------- | -------- | ------------ | ----------- |
| » code       | string   | true     | none         | none        |
| » data       | [object] | true     | none         | none        |
| »» orderId   | string   | true     | none         | none        |
| »» clientOid | string   | true     | none         | none        |
| »» code      | string   | true     | none         | none        |
| »» msg       | string   | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel All Orders

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/orders?symbol=type,string", {
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

r = requests.delete('/api/v3/orders', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/orders`

Cancel all open orders (excluding stop orders). The response is a list of
orderIDs of the canceled orders.

<h3 id="cancel-all-orders-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                                                    |
| ------ | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol | query | string | true     | Cancel all limit orders for a specific symbol only, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
        "cancelledOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Unique ID of the cancelled order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-all-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-all-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                      |
| -------------------- | -------- | -------- | ------------ | -------------------------------- |
| » code               | string   | true     | none         | none                             |
| » data               | object   | true     | none         | none                             |
| »» cancelledOrderIds | [string] | true     | none         | Unique ID of the cancelled order |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel All Stop orders

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stopOrders", {
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

r = requests.delete('/api/v1/stopOrders', headers = headers)

print(r.json())

```

`DELETE /api/v1/stopOrders`

Cancel all untriggered stop orders. The response is a list of orderIDs of the
canceled stop orders. To cancel triggered stop orders, please use 'Cancel
Multiple Futures Limit orders'.

<h3 id="cancel-all-stop-orders-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                                                                                                              |
| ------ | ----- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol | query | string | false    | Cancel all limit orders for a specific contract only, If not specified, all the limit orders will be deleted, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
        "cancelledOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "Unique ID of the cancelled order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-all-stop-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-all-stop-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                      |
| -------------------- | -------- | -------- | ------------ | -------------------------------- |
| » code               | string   | true     | none         | none                             |
| » data               | object   | true     | none         | none                             |
| »» cancelledOrderIds | [string] | true     | none         | Unique ID of the cancelled order |

<aside class="success">
This operation does not require authentication
</aside>

## Get Stop Order List

<a id="opId014"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stopOrders", {
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

r = requests.get('/api/v1/stopOrders', headers = headers)

print(r.json())

```

`GET /api/v1/stopOrders`

Get the un-triggered stop orders list. Stop orders that have been triggered can
be queried through the general order interface

<h3 id="get-stop-order-list-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                        |
| ----------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol      | query | string         | false    | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| side        | query | string         | false    | buy or sell                                                                                                        |
| type        | query | string         | false    | limit, market                                                                                                      |
| startAt     | query | integer(int64) | false    | Start time (milisecond)                                                                                            |
| endAt       | query | integer(int64) | false    | End time (milisecond)                                                                                              |
| currentPage | query | integer        | false    | Current request page, The default currentPage is 1                                                                 |
| pageSize    | query | integer        | false    | pageSize, The default pageSize is 50, The maximum cannot exceed 1000                                               |

#### Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| side      | buy    |
| side      | sell   |
| type      | limit  |
| type      | market |

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
          "description": "Current request page, The default currentPage is 1"
        },
        "pageSize": {
          "type": "integer",
          "description": "pageSize, The default pageSize is 50, The maximum cannot exceed 1000"
        },
        "totalNum": {
          "type": "integer"
        },
        "totalPage": {
          "type": "integer"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string",
                "description": "Order ID"
              },
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "type": {
                "type": "string",
                "description": "Order type, market order or limit order\n"
              },
              "side": {
                "type": "string",
                "description": "Transaction side"
              },
              "price": {
                "type": "string",
                "description": "Order price"
              },
              "size": {
                "type": "integer",
                "description": "Order quantity"
              },
              "value": {
                "type": "string",
                "description": "Order value"
              },
              "dealValue": {
                "type": "string",
                "description": "Executed size of funds\n"
              },
              "dealSize": {
                "type": "integer",
                "description": "Executed quantity"
              },
              "stp": {
                "type": "string",
                "description": "self trade prevention"
              },
              "stop": {
                "type": "string",
                "description": "A mark to the stop order type",
                "enum": ["down", "up", ""],
                "x-api-enum": [
                  {
                    "value": "down",
                    "name": "down",
                    "description": "Triggers when the price reaches or goes below the stopPrice."
                  },
                  {
                    "value": "up",
                    "name": "up",
                    "description": "Triggers when the price reaches or goes above the stopPrice."
                  },
                  {
                    "value": "",
                    "name": "None",
                    "description": "Not a stop order"
                  }
                ]
              },
              "stopPriceType": {
                "type": "string",
                "description": "Trigger price type of stop orders",
                "enum": ["TP", "MP", "IP", ""],
                "x-api-enum": [
                  {
                    "value": "TP",
                    "name": "trade price",
                    "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
                  },
                  {
                    "value": "MP",
                    "name": "mark price",
                    "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
                  },
                  {
                    "value": "IP",
                    "name": "index price",
                    "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
                  },
                  {
                    "value": "",
                    "name": "None",
                    "description": "Not a stop order"
                  }
                ]
              },
              "stopTriggered": {
                "type": "boolean",
                "description": "Mark to show whether the stop order is triggered"
              },
              "stopPrice": {
                "type": "string",
                "description": "Trigger price of stop orders"
              },
              "timeInForce": {
                "type": "string",
                "description": "Time in force policy type"
              },
              "postOnly": {
                "type": "boolean",
                "description": "Mark of post only"
              },
              "hidden": {
                "type": "boolean",
                "description": "Mark of the hidden order"
              },
              "iceberg": {
                "type": "boolean",
                "description": "Mark of the iceberg order"
              },
              "leverage": {
                "type": "string",
                "description": "Leverage of the order"
              },
              "forceHold": {
                "type": "boolean",
                "description": "A mark to forcely hold the funds for an order"
              },
              "closeOrder": {
                "type": "boolean",
                "description": "A mark to close the position"
              },
              "visibleSize": {
                "type": "integer",
                "description": "Visible size of the iceberg order"
              },
              "clientOid": {
                "type": "string",
                "description": "Unique order id created by users to identify their orders"
              },
              "remark": {
                "type": "string",
                "description": "Remark of the order"
              },
              "tags": {
                "type": "string",
                "description": "tag order source"
              },
              "isActive": {
                "type": "boolean",
                "description": "Mark of the active orders"
              },
              "cancelExist": {
                "type": "boolean",
                "description": "Mark of the canceled orders"
              },
              "createdAt": {
                "type": "integer",
                "description": "Time the order created",
                "format": "int64"
              },
              "updatedAt": {
                "type": "integer",
                "description": "last update time",
                "format": "int64"
              },
              "endAt": {
                "type": "integer",
                "description": "End time",
                "format": "int64"
              },
              "orderTime": {
                "type": "integer",
                "description": "Order create time in nanosecond",
                "format": "int64"
              },
              "settleCurrency": {
                "type": "string",
                "description": "settlement currency"
              },
              "marginMode": {
                "type": "string",
                "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin)."
              },
              "avgDealPrice": {
                "type": "string",
                "description": "Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots * multiplier"
              },
              "filledSize": {
                "type": "integer",
                "description": "Value of the executed orders"
              },
              "filledValue": {
                "type": "string",
                "description": "Executed order quantity"
              },
              "status": {
                "type": "string",
                "description": "order status: “open” or “done”"
              },
              "reduceOnly": {
                "type": "boolean",
                "description": "A mark to reduce the position size only"
              }
            },
            "required": [
              "id",
              "symbol",
              "type",
              "side",
              "price",
              "size",
              "value",
              "dealValue",
              "dealSize",
              "stp",
              "stop",
              "stopPriceType",
              "stopTriggered",
              "stopPrice",
              "timeInForce",
              "postOnly",
              "hidden",
              "iceberg",
              "leverage",
              "forceHold",
              "closeOrder",
              "visibleSize",
              "clientOid",
              "remark",
              "tags",
              "isActive",
              "cancelExist",
              "createdAt",
              "updatedAt",
              "endAt",
              "orderTime",
              "settleCurrency",
              "marginMode",
              "avgDealPrice",
              "filledSize",
              "filledValue",
              "status",
              "reduceOnly"
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

<h3 id="get-stop-order-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-stop-order-list-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                |
| ------------------ | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                       |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» currentPage     | integer        | true     | none         | Current request page, The default currentPage is 1                                                                                                                                                                                                                         |
| »» pageSize        | integer        | true     | none         | pageSize, The default pageSize is 50, The maximum cannot exceed 1000                                                                                                                                                                                                       |
| »» totalNum        | integer        | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» totalPage       | integer        | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                       |
| »»» id             | string         | true     | none         | Order ID                                                                                                                                                                                                                                                                   |
| »»» symbol         | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                         |
| »»» type           | string         | true     | none         | Order type, market order or limit order                                                                                                                                                                                                                                    |
| »»» side           | string         | true     | none         | Transaction side                                                                                                                                                                                                                                                           |
| »»» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                |
| »»» size           | integer        | true     | none         | Order quantity                                                                                                                                                                                                                                                             |
| »»» value          | string         | true     | none         | Order value                                                                                                                                                                                                                                                                |
| »»» dealValue      | string         | true     | none         | Executed size of funds                                                                                                                                                                                                                                                     |
| »»» dealSize       | integer        | true     | none         | Executed quantity                                                                                                                                                                                                                                                          |
| »»» stp            | string         | true     | none         | self trade prevention                                                                                                                                                                                                                                                      |
| »»» stop           | string         | true     | none         | A mark to the stop order type                                                                                                                                                                                                                                              |
| »»» stopPriceType  | string         | true     | none         | Trigger price type of stop orders                                                                                                                                                                                                                                          |
| »»» stopTriggered  | boolean        | true     | none         | Mark to show whether the stop order is triggered                                                                                                                                                                                                                           |
| »»» stopPrice      | string         | true     | none         | Trigger price of stop orders                                                                                                                                                                                                                                               |
| »»» timeInForce    | string         | true     | none         | Time in force policy type                                                                                                                                                                                                                                                  |
| »»» postOnly       | boolean        | true     | none         | Mark of post only                                                                                                                                                                                                                                                          |
| »»» hidden         | boolean        | true     | none         | Mark of the hidden order                                                                                                                                                                                                                                                   |
| »»» iceberg        | boolean        | true     | none         | Mark of the iceberg order                                                                                                                                                                                                                                                  |
| »»» leverage       | string         | true     | none         | Leverage of the order                                                                                                                                                                                                                                                      |
| »»» forceHold      | boolean        | true     | none         | A mark to forcely hold the funds for an order                                                                                                                                                                                                                              |
| »»» closeOrder     | boolean        | true     | none         | A mark to close the position                                                                                                                                                                                                                                               |
| »»» visibleSize    | integer        | true     | none         | Visible size of the iceberg order                                                                                                                                                                                                                                          |
| »»» clientOid      | string         | true     | none         | Unique order id created by users to identify their orders                                                                                                                                                                                                                  |
| »»» remark         | string         | true     | none         | Remark of the order                                                                                                                                                                                                                                                        |
| »»» tags           | string         | true     | none         | tag order source                                                                                                                                                                                                                                                           |
| »»» isActive       | boolean        | true     | none         | Mark of the active orders                                                                                                                                                                                                                                                  |
| »»» cancelExist    | boolean        | true     | none         | Mark of the canceled orders                                                                                                                                                                                                                                                |
| »»» createdAt      | integer(int64) | true     | none         | Time the order created                                                                                                                                                                                                                                                     |
| »»» updatedAt      | integer(int64) | true     | none         | last update time                                                                                                                                                                                                                                                           |
| »»» endAt          | integer(int64) | true     | none         | End time                                                                                                                                                                                                                                                                   |
| »»» orderTime      | integer(int64) | true     | none         | Order create time in nanosecond                                                                                                                                                                                                                                            |
| »»» settleCurrency | string         | true     | none         | settlement currency                                                                                                                                                                                                                                                        |
| »»» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                                                                                                                                                                                    |
| »»» avgDealPrice   | string         | true     | none         | Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots \* multiplier |
| »»» filledSize     | integer        | true     | none         | Value of the executed orders                                                                                                                                                                                                                                               |
| »»» filledValue    | string         | true     | none         | Executed order quantity                                                                                                                                                                                                                                                    |
| »»» status         | string         | true     | none         | order status: “open” or “done”                                                                                                                                                                                                                                             |
| »»» reduceOnly     | boolean        | true     | none         | A mark to reduce the position size only                                                                                                                                                                                                                                    |

#### Enumerated Values

| Property      | Value |
| ------------- | ----- |
| stop          | down  |
| stop          | up    |
| stop          |       |
| stopPriceType | TP    |
| stopPriceType | MP    |
| stopPriceType | IP    |
| stopPriceType |       |

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

fetch("/api/v1/orders/{order-id}", {
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

r = requests.get('/api/v1/orders/{order-id}', headers = headers)

print(r.json())

```

`GET /api/v1/orders/{order-id}`

Get a single order by order id (including a stop order).

<h3 id="get-order-by-orderid-parameters">Parameters</h3>

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| order-id | path | string | true     | none        |

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
          "description": "Order ID"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "type": {
          "type": "string",
          "description": "Order type, market order or limit order",
          "enum": ["market", "limit"],
          "x-api-enum": [
            {
              "value": "market",
              "name": "market",
              "description": "Market Order"
            },
            {
              "value": "limit",
              "name": "limit",
              "description": "Limit Order"
            }
          ]
        },
        "side": {
          "type": "string",
          "description": "Transaction side",
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
        "price": {
          "type": "string",
          "description": "Order price"
        },
        "size": {
          "type": "integer",
          "description": "Order quantity"
        },
        "value": {
          "type": "string",
          "description": "Order value\n"
        },
        "dealValue": {
          "type": "string",
          "description": "Executed size of funds\n"
        },
        "dealSize": {
          "type": "integer",
          "description": "Executed quantity\n"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.",
          "enum": ["", "CN", "CO", "CB"],
          "x-api-enum": [
            {
              "value": "",
              "name": "NULL",
              "description": "Null means not stp order"
            },
            {
              "value": "CN",
              "name": "CN",
              "description": "Cancel new, Cancel the new order"
            },
            {
              "value": "CO",
              "name": "CO",
              "description": "Cancel old, Cancel the old order"
            },
            {
              "value": "CB",
              "name": "CB",
              "description": "Cancel both, Cancel both sides"
            }
          ]
        },
        "stop": {
          "type": "string",
          "description": "A mark to the stop order type",
          "enum": ["down", "up", ""],
          "x-api-enum": [
            {
              "value": "down",
              "name": "down",
              "description": "Triggers when the price reaches or goes below the stopPrice."
            },
            {
              "value": "up",
              "name": "up",
              "description": "Triggers when the price reaches or goes above the stopPrice."
            },
            {
              "value": "",
              "name": "None",
              "description": "Not a stop order"
            }
          ]
        },
        "stopPriceType": {
          "type": "string",
          "description": "Trigger price type of stop orders",
          "enum": ["TP", "MP", "IP", ""],
          "x-api-enum": [
            {
              "value": "TP",
              "name": "trade price",
              "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
            },
            {
              "value": "MP",
              "name": "mark price",
              "description": "MP for mark price, The mark price can be obtained through relevant OPEN API for index services"
            },
            {
              "value": "IP",
              "name": "index price",
              "description": "IP for index price, The index price can be obtained through relevant OPEN API for index services"
            },
            {
              "value": "",
              "name": "None",
              "description": "Not a stop order"
            }
          ]
        },
        "stopTriggered": {
          "type": "boolean",
          "description": "Mark to show whether the stop order is triggered"
        },
        "stopPrice": {
          "type": "number",
          "description": "Trigger price of stop orders"
        },
        "timeInForce": {
          "type": "string",
          "description": "Time in force policy type\n"
        },
        "postOnly": {
          "type": "boolean",
          "description": "Mark of post only\n"
        },
        "hidden": {
          "type": "boolean",
          "description": "Mark of the hidden order\n"
        },
        "iceberg": {
          "type": "boolean",
          "description": "Mark of the iceberg order\n"
        },
        "leverage": {
          "type": "string",
          "description": "Leverage of the order\n"
        },
        "forceHold": {
          "type": "boolean",
          "description": "A mark to forcely hold the funds for an order\n"
        },
        "closeOrder": {
          "type": "boolean",
          "description": "A mark to close the position\n"
        },
        "visibleSize": {
          "type": "integer",
          "description": "Visible size of the iceberg order\n"
        },
        "clientOid": {
          "type": "string",
          "description": "Unique order id created by users to identify their orders\n"
        },
        "remark": {
          "type": "string",
          "description": "Remark"
        },
        "tags": {
          "type": "string",
          "description": "tag order source\n"
        },
        "isActive": {
          "type": "boolean",
          "description": "Mark of the active orders\n"
        },
        "cancelExist": {
          "type": "boolean",
          "description": "Mark of the canceled orders\n"
        },
        "createdAt": {
          "type": "integer",
          "description": "Time the order created\n",
          "format": "int64"
        },
        "updatedAt": {
          "type": "integer",
          "description": "last update time\n",
          "format": "int64"
        },
        "endAt": {
          "type": "integer",
          "description": "Order Endtime",
          "format": "int64"
        },
        "orderTime": {
          "type": "integer",
          "description": "Order create time in nanosecond\n",
          "format": "int64"
        },
        "settleCurrency": {
          "type": "string",
          "description": "settlement currency\n"
        },
        "marginMode": {
          "type": "string",
          "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).\n",
          "enum": ["CROSS", "ISOLATED"],
          "x-api-enum": [
            {
              "value": "CROSS",
              "name": "CROSS",
              "description": "Cross margin"
            },
            {
              "value": "ISOLATED",
              "name": "ISOLATED",
              "description": "Isolated margin"
            }
          ]
        },
        "avgDealPrice": {
          "type": "string",
          "description": "Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots * multiplier\n"
        },
        "filledSize": {
          "type": "integer",
          "description": "Value of the executed orders\n"
        },
        "filledValue": {
          "type": "string",
          "description": "Executed order quantity\n"
        },
        "status": {
          "type": "string",
          "description": "order status: “open” or “done”\n",
          "enum": ["open", "done"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "open",
              "description": "open order"
            },
            {
              "value": "done",
              "name": "done",
              "description": "done order"
            }
          ]
        },
        "reduceOnly": {
          "type": "boolean",
          "description": "A mark to reduce the position size only\n"
        }
      },
      "required": [
        "id",
        "symbol",
        "type",
        "side",
        "price",
        "size",
        "value",
        "dealValue",
        "dealSize",
        "stp",
        "stop",
        "stopPriceType",
        "stopTriggered",
        "stopPrice",
        "timeInForce",
        "postOnly",
        "hidden",
        "iceberg",
        "leverage",
        "forceHold",
        "closeOrder",
        "visibleSize",
        "clientOid",
        "remark",
        "tags",
        "isActive",
        "cancelExist",
        "createdAt",
        "updatedAt",
        "orderTime",
        "settleCurrency",
        "marginMode",
        "avgDealPrice",
        "filledSize",
        "filledValue",
        "status",
        "reduceOnly"
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

| Name              | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                |
| ----------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                                                                                                                                                                       |
| » data            | object         | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» id             | string         | true     | none         | Order ID                                                                                                                                                                                                                                                                   |
| »» symbol         | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                         |
| »» type           | string         | true     | none         | Order type, market order or limit order                                                                                                                                                                                                                                    |
| »» side           | string         | true     | none         | Transaction side                                                                                                                                                                                                                                                           |
| »» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                |
| »» size           | integer        | true     | none         | Order quantity                                                                                                                                                                                                                                                             |
| »» value          | string         | true     | none         | Order value                                                                                                                                                                                                                                                                |
| »» dealValue      | string         | true     | none         | Executed size of funds                                                                                                                                                                                                                                                     |
| »» dealSize       | integer        | true     | none         | Executed quantity                                                                                                                                                                                                                                                          |
| »» stp            | string         | true     | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. Not supported DC at the moment.                                                                                                                          |
| »» stop           | string         | true     | none         | A mark to the stop order type                                                                                                                                                                                                                                              |
| »» stopPriceType  | string         | true     | none         | Trigger price type of stop orders                                                                                                                                                                                                                                          |
| »» stopTriggered  | boolean        | true     | none         | Mark to show whether the stop order is triggered                                                                                                                                                                                                                           |
| »» stopPrice      | number         | true     | none         | Trigger price of stop orders                                                                                                                                                                                                                                               |
| »» timeInForce    | string         | true     | none         | Time in force policy type                                                                                                                                                                                                                                                  |
| »» postOnly       | boolean        | true     | none         | Mark of post only                                                                                                                                                                                                                                                          |
| »» hidden         | boolean        | true     | none         | Mark of the hidden order                                                                                                                                                                                                                                                   |
| »» iceberg        | boolean        | true     | none         | Mark of the iceberg order                                                                                                                                                                                                                                                  |
| »» leverage       | string         | true     | none         | Leverage of the order                                                                                                                                                                                                                                                      |
| »» forceHold      | boolean        | true     | none         | A mark to forcely hold the funds for an order                                                                                                                                                                                                                              |
| »» closeOrder     | boolean        | true     | none         | A mark to close the position                                                                                                                                                                                                                                               |
| »» visibleSize    | integer        | true     | none         | Visible size of the iceberg order                                                                                                                                                                                                                                          |
| »» clientOid      | string         | true     | none         | Unique order id created by users to identify their orders                                                                                                                                                                                                                  |
| »» remark         | string         | true     | none         | Remark                                                                                                                                                                                                                                                                     |
| »» tags           | string         | true     | none         | tag order source                                                                                                                                                                                                                                                           |
| »» isActive       | boolean        | true     | none         | Mark of the active orders                                                                                                                                                                                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Mark of the canceled orders                                                                                                                                                                                                                                                |
| »» createdAt      | integer(int64) | true     | none         | Time the order created                                                                                                                                                                                                                                                     |
| »» updatedAt      | integer(int64) | true     | none         | last update time                                                                                                                                                                                                                                                           |
| »» endAt          | integer(int64) | false    | none         | Order Endtime                                                                                                                                                                                                                                                              |
| »» orderTime      | integer(int64) | true     | none         | Order create time in nanosecond                                                                                                                                                                                                                                            |
| »» settleCurrency | string         | true     | none         | settlement currency                                                                                                                                                                                                                                                        |
| »» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                                                                                                                                                                                    |
| »» avgDealPrice   | string         | true     | none         | Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots \* multiplier |
| »» filledSize     | integer        | true     | none         | Value of the executed orders                                                                                                                                                                                                                                               |
| »» filledValue    | string         | true     | none         | Executed order quantity                                                                                                                                                                                                                                                    |
| »» status         | string         | true     | none         | order status: “open” or “done”                                                                                                                                                                                                                                             |
| »» reduceOnly     | boolean        | true     | none         | A mark to reduce the position size only                                                                                                                                                                                                                                    |

#### Enumerated Values

| Property      | Value    |
| ------------- | -------- |
| type          | market   |
| type          | limit    |
| side          | buy      |
| side          | sell     |
| stp           |          |
| stp           | CN       |
| stp           | CO       |
| stp           | CB       |
| stop          | down     |
| stop          | up       |
| stop          |          |
| stopPriceType | TP       |
| stopPriceType | MP       |
| stopPriceType | IP       |
| stopPriceType |          |
| marginMode    | CROSS    |
| marginMode    | ISOLATED |
| status        | open     |
| status        | done     |

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

fetch("/api/v1/orders/byClientOid?clientOid=type,string", {
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

r = requests.get('/api/v1/orders/byClientOid', params={
  'clientOid': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/orders/byClientOid`

Get a single order by client order ID (including a stop order).

<h3 id="get-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                     |
| --------- | ----- | ------ | -------- | ------------------------------- |
| clientOid | query | string | true     | The user self-defined order ID. |

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
          "description": "Order ID"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
        },
        "type": {
          "type": "string",
          "description": "Order type, market order or limit order",
          "enum": ["market", "limit"],
          "x-api-enum": [
            {
              "value": "market",
              "name": "market",
              "description": "Market Order"
            },
            {
              "value": "limit",
              "name": "limit",
              "description": "Limit Order"
            }
          ]
        },
        "side": {
          "type": "string",
          "description": "Transaction side",
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
        "price": {
          "type": "string",
          "description": "Order Price"
        },
        "size": {
          "type": "integer",
          "description": "Order quantity"
        },
        "value": {
          "type": "string",
          "description": "Order value\n"
        },
        "dealValue": {
          "type": "string",
          "description": "Executed size of funds\n"
        },
        "dealSize": {
          "type": "integer",
          "description": "Executed quantity\n"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. DC not currently supported.",
          "enum": ["", "CN", "CO", "CB"],
          "x-api-enum": [
            {
              "value": "",
              "name": "NULL",
              "description": "None"
            },
            {
              "value": "CN",
              "name": "CN",
              "description": "Cancel new, Cancel the new order"
            },
            {
              "value": "CO",
              "name": "CO",
              "description": "Cancel old, Cancel the old order"
            },
            {
              "value": "CB",
              "name": "CB",
              "description": "Cancel both, Cancel both sides"
            }
          ]
        },
        "stop": {
          "type": "string",
          "description": "A mark to the stop order type",
          "enum": ["down", "up", ""],
          "x-api-enum": [
            {
              "value": "down",
              "name": "down",
              "description": "Triggers when the price reaches or goes below the stopPrice."
            },
            {
              "value": "up",
              "name": "up",
              "description": "Triggers when the price reaches or goes above the stopPrice."
            },
            {
              "value": "",
              "name": "None",
              "description": "Not a stop order"
            }
          ]
        },
        "stopPriceType": {
          "type": "string",
          "description": "Trigger price type of stop orders",
          "enum": ["TP", "MP", "IP", ""],
          "x-api-enum": [
            {
              "value": "TP",
              "name": "trade price",
              "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
            },
            {
              "value": "MP",
              "name": "mark price",
              "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
            },
            {
              "value": "IP",
              "name": "index price",
              "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
            },
            {
              "value": "",
              "name": "None",
              "description": "Not a stop order"
            }
          ]
        },
        "stopTriggered": {
          "type": "boolean",
          "description": "Mark to show whether the stop order is triggered"
        },
        "stopPrice": {
          "type": "number",
          "description": "Trigger price of stop orders"
        },
        "timeInForce": {
          "type": "string",
          "description": "Time in force policy type\n"
        },
        "postOnly": {
          "type": "boolean",
          "description": "Mark of post only\n"
        },
        "hidden": {
          "type": "boolean",
          "description": "Mark of the hidden order\n"
        },
        "iceberg": {
          "type": "boolean",
          "description": "Mark of the iceberg order\n"
        },
        "leverage": {
          "type": "string",
          "description": "Leverage of the order\n"
        },
        "forceHold": {
          "type": "boolean",
          "description": "A mark to force-hold the funds for an order\n"
        },
        "closeOrder": {
          "type": "boolean",
          "description": "A mark to close the position\n"
        },
        "visibleSize": {
          "type": "integer",
          "description": "Visible size of the iceberg order\n"
        },
        "clientOid": {
          "type": "string",
          "description": "Unique order ID created by users to identify their orders\n"
        },
        "remark": {
          "type": "string",
          "description": "Remark"
        },
        "tags": {
          "type": "string",
          "description": "Tag order source\n"
        },
        "isActive": {
          "type": "boolean",
          "description": "Mark of the active orders\n"
        },
        "cancelExist": {
          "type": "boolean",
          "description": "Mark of the canceled orders\n"
        },
        "createdAt": {
          "type": "integer",
          "description": "Order creation time\n",
          "format": "int64"
        },
        "updatedAt": {
          "type": "integer",
          "description": "Last update time\n",
          "format": "int64"
        },
        "endAt": {
          "type": "integer",
          "description": "Order Endtime",
          "format": "int64"
        },
        "orderTime": {
          "type": "integer",
          "description": "Order creation time in nanoseconds\n",
          "format": "int64"
        },
        "settleCurrency": {
          "type": "string",
          "description": "Settlement currency\n"
        },
        "marginMode": {
          "type": "string",
          "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).\n",
          "enum": ["CROSS", "ISOLATED"],
          "x-api-enum": [
            {
              "value": "CROSS",
              "name": "CROSS",
              "description": "Cross margin"
            },
            {
              "value": "ISOLATED",
              "name": "ISOLATED",
              "description": "Isolated margin"
            }
          ]
        },
        "avgDealPrice": {
          "type": "string",
          "description": "Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity); reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots * multiplier\n"
        },
        "filledSize": {
          "type": "integer",
          "description": "Value of the executed orders\n"
        },
        "filledValue": {
          "type": "string",
          "description": "Executed order quantity\n"
        },
        "status": {
          "type": "string",
          "description": "order status: “open” or “done”\n",
          "enum": ["open", "done"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "open",
              "description": "open order"
            },
            {
              "value": "done",
              "name": "done",
              "description": "done order"
            }
          ]
        },
        "reduceOnly": {
          "type": "boolean",
          "description": "A mark to reduce the position size only\n"
        }
      },
      "required": [
        "id",
        "symbol",
        "type",
        "side",
        "price",
        "size",
        "value",
        "dealValue",
        "dealSize",
        "stp",
        "stop",
        "stopPriceType",
        "stopTriggered",
        "stopPrice",
        "timeInForce",
        "postOnly",
        "hidden",
        "iceberg",
        "leverage",
        "forceHold",
        "closeOrder",
        "visibleSize",
        "clientOid",
        "remark",
        "tags",
        "isActive",
        "cancelExist",
        "createdAt",
        "updatedAt",
        "endAt",
        "orderTime",
        "settleCurrency",
        "marginMode",
        "avgDealPrice",
        "filledSize",
        "filledValue",
        "status",
        "reduceOnly"
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

| Name              | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                |
| ----------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                                                                                                                                                                       |
| » data            | object         | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» id             | string         | true     | none         | Order ID                                                                                                                                                                                                                                                                   |
| »» symbol         | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                         |
| »» type           | string         | true     | none         | Order type, market order or limit order                                                                                                                                                                                                                                    |
| »» side           | string         | true     | none         | Transaction side                                                                                                                                                                                                                                                           |
| »» price          | string         | true     | none         | Order Price                                                                                                                                                                                                                                                                |
| »» size           | integer        | true     | none         | Order quantity                                                                                                                                                                                                                                                             |
| »» value          | string         | true     | none         | Order value                                                                                                                                                                                                                                                                |
| »» dealValue      | string         | true     | none         | Executed size of funds                                                                                                                                                                                                                                                     |
| »» dealSize       | integer        | true     | none         | Executed quantity                                                                                                                                                                                                                                                          |
| »» stp            | string         | true     | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB. DC not currently supported.                                                                                                                              |
| »» stop           | string         | true     | none         | A mark to the stop order type                                                                                                                                                                                                                                              |
| »» stopPriceType  | string         | true     | none         | Trigger price type of stop orders                                                                                                                                                                                                                                          |
| »» stopTriggered  | boolean        | true     | none         | Mark to show whether the stop order is triggered                                                                                                                                                                                                                           |
| »» stopPrice      | number         | true     | none         | Trigger price of stop orders                                                                                                                                                                                                                                               |
| »» timeInForce    | string         | true     | none         | Time in force policy type                                                                                                                                                                                                                                                  |
| »» postOnly       | boolean        | true     | none         | Mark of post only                                                                                                                                                                                                                                                          |
| »» hidden         | boolean        | true     | none         | Mark of the hidden order                                                                                                                                                                                                                                                   |
| »» iceberg        | boolean        | true     | none         | Mark of the iceberg order                                                                                                                                                                                                                                                  |
| »» leverage       | string         | true     | none         | Leverage of the order                                                                                                                                                                                                                                                      |
| »» forceHold      | boolean        | true     | none         | A mark to force-hold the funds for an order                                                                                                                                                                                                                                |
| »» closeOrder     | boolean        | true     | none         | A mark to close the position                                                                                                                                                                                                                                               |
| »» visibleSize    | integer        | true     | none         | Visible size of the iceberg order                                                                                                                                                                                                                                          |
| »» clientOid      | string         | true     | none         | Unique order ID created by users to identify their orders                                                                                                                                                                                                                  |
| »» remark         | string         | true     | none         | Remark                                                                                                                                                                                                                                                                     |
| »» tags           | string         | true     | none         | Tag order source                                                                                                                                                                                                                                                           |
| »» isActive       | boolean        | true     | none         | Mark of the active orders                                                                                                                                                                                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Mark of the canceled orders                                                                                                                                                                                                                                                |
| »» createdAt      | integer(int64) | true     | none         | Order creation time                                                                                                                                                                                                                                                        |
| »» updatedAt      | integer(int64) | true     | none         | Last update time                                                                                                                                                                                                                                                           |
| »» endAt          | integer(int64) | true     | none         | Order Endtime                                                                                                                                                                                                                                                              |
| »» orderTime      | integer(int64) | true     | none         | Order creation time in nanoseconds                                                                                                                                                                                                                                         |
| »» settleCurrency | string         | true     | none         | Settlement currency                                                                                                                                                                                                                                                        |
| »» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                                                                                                                                                                                    |
| »» avgDealPrice   | string         | true     | none         | Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity); reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots \* multiplier |
| »» filledSize     | integer        | true     | none         | Value of the executed orders                                                                                                                                                                                                                                               |
| »» filledValue    | string         | true     | none         | Executed order quantity                                                                                                                                                                                                                                                    |
| »» status         | string         | true     | none         | order status: “open” or “done”                                                                                                                                                                                                                                             |
| »» reduceOnly     | boolean        | true     | none         | A mark to reduce the position size only                                                                                                                                                                                                                                    |

#### Enumerated Values

| Property      | Value    |
| ------------- | -------- |
| type          | market   |
| type          | limit    |
| side          | buy      |
| side          | sell     |
| stp           |          |
| stp           | CN       |
| stp           | CO       |
| stp           | CB       |
| stop          | down     |
| stop          | up       |
| stop          |          |
| stopPriceType | TP       |
| stopPriceType | MP       |
| stopPriceType | IP       |
| stopPriceType |          |
| marginMode    | CROSS    |
| marginMode    | ISOLATED |
| status        | open     |
| status        | done     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Recent Closed Orders

<a id="opId013"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/recentDoneOrders", {
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

r = requests.get('/api/v1/recentDoneOrders', headers = headers)

print(r.json())

```

`GET /api/v1/recentDoneOrders`

Get a list of recent 1000 closed orders in the last 24 hours. If you need to get
your recent traded order history with low latency, you may query this endpoint.

<h3 id="get-recent-closed-orders-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | false    | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
            "description": "Order ID"
          },
          "symbol": {
            "type": "string",
            "description": "Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
          },
          "type": {
            "type": "string",
            "description": "Order type, market order or limit order"
          },
          "side": {
            "type": "string",
            "description": "Transaction side"
          },
          "price": {
            "type": "string",
            "description": "Order price"
          },
          "size": {
            "type": "integer",
            "description": "Order quantity"
          },
          "value": {
            "type": "string",
            "description": "Order value\n"
          },
          "dealValue": {
            "type": "string",
            "description": "Executed size of funds\n"
          },
          "dealSize": {
            "type": "integer",
            "description": "Executed quantity\n"
          },
          "stp": {
            "type": "string",
            "description": "self trade prevention"
          },
          "stop": {
            "type": "string",
            "description": "A mark to the stop order type",
            "enum": ["down", "up", ""],
            "x-api-enum": [
              {
                "value": "down",
                "name": "down",
                "description": "Triggers when the price reaches or goes below the stopPrice."
              },
              {
                "value": "up",
                "name": "up",
                "description": "Triggers when the price reaches or goes above the stopPrice."
              },
              {
                "value": "",
                "name": "None",
                "description": "Not a stop order"
              }
            ]
          },
          "stopPriceType": {
            "type": "string",
            "description": "Trigger price type of stop orders",
            "enum": ["TP", "MP", "IP", ""],
            "x-api-enum": [
              {
                "value": "TP",
                "name": "trade price",
                "description": "TP for trade price, The last trade price is the last price at which an order was filled. This price can be found in the latest match message."
              },
              {
                "value": "MP",
                "name": "mark price",
                "description": "MP for mark price. The mark price can be obtained through relevant OPEN API for index services."
              },
              {
                "value": "IP",
                "name": "index price",
                "description": "IP for index price. The index price can be obtained through relevant OPEN API for index services."
              },
              {
                "value": "",
                "name": "None",
                "description": "Not a stop order"
              }
            ]
          },
          "stopTriggered": {
            "type": "boolean",
            "description": "Mark to show whether the stop order is triggered"
          },
          "stopPrice": {
            "type": "integer",
            "description": "Trigger price of stop orders\n"
          },
          "timeInForce": {
            "type": "string",
            "description": "Time in force policy type\n"
          },
          "postOnly": {
            "type": "boolean",
            "description": "Mark of post only\n"
          },
          "hidden": {
            "type": "boolean",
            "description": "Mark of the hidden order\n"
          },
          "iceberg": {
            "type": "boolean",
            "description": "Mark of the iceberg order\n"
          },
          "leverage": {
            "type": "string",
            "description": "Leverage of the order\n"
          },
          "forceHold": {
            "type": "boolean",
            "description": "A mark to forcely hold the funds for an order\n"
          },
          "closeOrder": {
            "type": "boolean",
            "description": "A mark to close the position\n"
          },
          "visibleSize": {
            "type": "integer",
            "description": "Visible size of the iceberg order\n"
          },
          "clientOid": {
            "type": "string",
            "description": "Unique order id created by users to identify their orders\n"
          },
          "remark": {
            "type": "string",
            "description": "Remark of the order\n"
          },
          "tags": {
            "type": "string",
            "description": "tag order source\n"
          },
          "isActive": {
            "type": "boolean",
            "description": "Mark of the active orders\n"
          },
          "cancelExist": {
            "type": "boolean",
            "description": "Mark of the canceled orders\n"
          },
          "createdAt": {
            "type": "integer",
            "description": "Time the order created\n",
            "format": "int64"
          },
          "updatedAt": {
            "type": "integer",
            "description": "last update time\n",
            "format": "int64"
          },
          "endAt": {
            "type": "integer",
            "description": "End time\n",
            "format": "int64"
          },
          "orderTime": {
            "type": "integer",
            "description": "Order create time in nanosecond\n",
            "format": "int64"
          },
          "settleCurrency": {
            "type": "string",
            "description": "settlement currency\n"
          },
          "marginMode": {
            "type": "string",
            "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).\n"
          },
          "avgDealPrice": {
            "type": "string",
            "description": "Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots * multiplier\n"
          },
          "filledSize": {
            "type": "integer",
            "description": "Value of the executed orders\n"
          },
          "filledValue": {
            "type": "string",
            "description": "Executed order quantity\n"
          },
          "status": {
            "type": "string",
            "description": "order status: “open” or “done”\n"
          },
          "reduceOnly": {
            "type": "boolean",
            "description": "A mark to reduce the position size only\n"
          }
        },
        "required": [
          "id",
          "symbol",
          "type",
          "side",
          "price",
          "size",
          "value",
          "dealValue",
          "dealSize",
          "stp",
          "stop",
          "stopPriceType",
          "stopTriggered",
          "stopPrice",
          "timeInForce",
          "postOnly",
          "hidden",
          "iceberg",
          "leverage",
          "forceHold",
          "closeOrder",
          "visibleSize",
          "clientOid",
          "remark",
          "tags",
          "isActive",
          "cancelExist",
          "createdAt",
          "updatedAt",
          "endAt",
          "orderTime",
          "settleCurrency",
          "marginMode",
          "avgDealPrice",
          "filledSize",
          "filledValue",
          "status",
          "reduceOnly"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-recent-closed-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-recent-closed-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                |
| ----------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                                                                                                                                                                       |
| » data            | [object]       | true     | none         | none                                                                                                                                                                                                                                                                       |
| »» id             | string         | true     | none         | Order ID                                                                                                                                                                                                                                                                   |
| »» symbol         | string         | true     | none         | Symbol of the contract, Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                                                                                                                                         |
| »» type           | string         | true     | none         | Order type, market order or limit order                                                                                                                                                                                                                                    |
| »» side           | string         | true     | none         | Transaction side                                                                                                                                                                                                                                                           |
| »» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                |
| »» size           | integer        | true     | none         | Order quantity                                                                                                                                                                                                                                                             |
| »» value          | string         | true     | none         | Order value                                                                                                                                                                                                                                                                |
| »» dealValue      | string         | true     | none         | Executed size of funds                                                                                                                                                                                                                                                     |
| »» dealSize       | integer        | true     | none         | Executed quantity                                                                                                                                                                                                                                                          |
| »» stp            | string         | true     | none         | self trade prevention                                                                                                                                                                                                                                                      |
| »» stop           | string         | true     | none         | A mark to the stop order type                                                                                                                                                                                                                                              |
| »» stopPriceType  | string         | true     | none         | Trigger price type of stop orders                                                                                                                                                                                                                                          |
| »» stopTriggered  | boolean        | true     | none         | Mark to show whether the stop order is triggered                                                                                                                                                                                                                           |
| »» stopPrice      | integer        | true     | none         | Trigger price of stop orders                                                                                                                                                                                                                                               |
| »» timeInForce    | string         | true     | none         | Time in force policy type                                                                                                                                                                                                                                                  |
| »» postOnly       | boolean        | true     | none         | Mark of post only                                                                                                                                                                                                                                                          |
| »» hidden         | boolean        | true     | none         | Mark of the hidden order                                                                                                                                                                                                                                                   |
| »» iceberg        | boolean        | true     | none         | Mark of the iceberg order                                                                                                                                                                                                                                                  |
| »» leverage       | string         | true     | none         | Leverage of the order                                                                                                                                                                                                                                                      |
| »» forceHold      | boolean        | true     | none         | A mark to forcely hold the funds for an order                                                                                                                                                                                                                              |
| »» closeOrder     | boolean        | true     | none         | A mark to close the position                                                                                                                                                                                                                                               |
| »» visibleSize    | integer        | true     | none         | Visible size of the iceberg order                                                                                                                                                                                                                                          |
| »» clientOid      | string         | true     | none         | Unique order id created by users to identify their orders                                                                                                                                                                                                                  |
| »» remark         | string         | true     | none         | Remark of the order                                                                                                                                                                                                                                                        |
| »» tags           | string         | true     | none         | tag order source                                                                                                                                                                                                                                                           |
| »» isActive       | boolean        | true     | none         | Mark of the active orders                                                                                                                                                                                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Mark of the canceled orders                                                                                                                                                                                                                                                |
| »» createdAt      | integer(int64) | true     | none         | Time the order created                                                                                                                                                                                                                                                     |
| »» updatedAt      | integer(int64) | true     | none         | last update time                                                                                                                                                                                                                                                           |
| »» endAt          | integer(int64) | true     | none         | End time                                                                                                                                                                                                                                                                   |
| »» orderTime      | integer(int64) | true     | none         | Order create time in nanosecond                                                                                                                                                                                                                                            |
| »» settleCurrency | string         | true     | none         | settlement currency                                                                                                                                                                                                                                                        |
| »» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                                                                                                                                                                                    |
| »» avgDealPrice   | string         | true     | none         | Average transaction price, forward contract average transaction price = sum (transaction value) / sum (transaction quantity), reverse contract average transaction price = sum (transaction quantity) / sum (transaction value). Transaction quantity = lots \* multiplier |
| »» filledSize     | integer        | true     | none         | Value of the executed orders                                                                                                                                                                                                                                               |
| »» filledValue    | string         | true     | none         | Executed order quantity                                                                                                                                                                                                                                                    |
| »» status         | string         | true     | none         | order status: “open” or “done”                                                                                                                                                                                                                                             |
| »» reduceOnly     | boolean        | true     | none         | A mark to reduce the position size only                                                                                                                                                                                                                                    |

#### Enumerated Values

| Property      | Value |
| ------------- | ----- |
| stop          | down  |
| stop          | up    |
| stop          |       |
| stopPriceType | TP    |
| stopPriceType | MP    |
| stopPriceType | IP    |
| stopPriceType |       |

<aside class="success">
This operation does not require authentication
</aside>

## Get Open Order Value

<a id="opId015"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/openOrderStatistics?symbol=type,string", {
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

r = requests.get('/api/v1/openOrderStatistics', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/openOrderStatistics`

You can query this endpoint to get the total number and value of all your active
orders.

<h3 id="get-open-order-value-parameters">Parameters</h3>

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
      "type": "object",
      "properties": {
        "openOrderBuySize": {
          "type": "integer",
          "description": "Total number of unexecuted buy orders\n"
        },
        "openOrderSellSize": {
          "type": "integer",
          "description": "Total number of unexecuted sell orders\n"
        },
        "openOrderBuyCost": {
          "type": "string",
          "description": "Value of all unexecuted buy orders\n"
        },
        "openOrderSellCost": {
          "type": "string",
          "description": "Value of all unexecuted sell orders\n"
        },
        "settleCurrency": {
          "type": "string",
          "description": "Settlement currency\n"
        }
      },
      "required": [
        "openOrderBuySize",
        "openOrderSellSize",
        "openOrderBuyCost",
        "openOrderSellCost",
        "settleCurrency"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-open-order-value-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-open-order-value-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type    | Required | Restrictions | Description                            |
| -------------------- | ------- | -------- | ------------ | -------------------------------------- |
| » code               | string  | true     | none         | none                                   |
| » data               | object  | true     | none         | none                                   |
| »» openOrderBuySize  | integer | true     | none         | Total number of unexecuted buy orders  |
| »» openOrderSellSize | integer | true     | none         | Total number of unexecuted sell orders |
| »» openOrderBuyCost  | string  | true     | none         | Value of all unexecuted buy orders     |
| »» openOrderSellCost | string  | true     | none         | Value of all unexecuted sell orders    |
| »» settleCurrency    | string  | true     | none         | Settlement currency                    |

<aside class="success">
This operation does not require authentication
</aside>

## Get Recent Trade History

<a id="opId016"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/recentFills", {
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

r = requests.get('/api/v1/recentFills', headers = headers)

print(r.json())

```

`GET /api/v1/recentFills`

Get a list of recent 1000 fills in the last 24 hours. If you need to get your
recently traded order history with low latency, you may query this endpoint.

<h3 id="get-recent-trade-history-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string | false    | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |

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
          "tradeId": {
            "type": "string",
            "description": "Trade ID\n"
          },
          "orderId": {
            "type": "string",
            "description": "Order ID\n"
          },
          "side": {
            "type": "string",
            "description": "Transaction side\n",
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
          "liquidity": {
            "type": "string",
            "description": "Liquidity-taker or -maker\n",
            "enum": ["taker", "maker"],
            "x-api-enum": [
              {
                "value": "taker",
                "name": "taker",
                "description": "taker"
              },
              {
                "value": "maker",
                "name": "maker",
                "description": "maker"
              }
            ]
          },
          "forceTaker": {
            "type": "boolean",
            "description": "Whether to force processing as a taker\n"
          },
          "price": {
            "type": "string",
            "description": "Filled price\n"
          },
          "size": {
            "type": "integer",
            "description": "Filled amount\n"
          },
          "value": {
            "type": "string",
            "description": "Order value\n"
          },
          "openFeePay": {
            "type": "string",
            "description": "Opening transaction fee\n"
          },
          "closeFeePay": {
            "type": "string",
            "description": "Closing transaction fee\n"
          },
          "stop": {
            "type": "string",
            "description": "A mark to the stop order type",
            "enum": ["down", "up", ""],
            "x-api-enum": [
              {
                "value": "down",
                "name": "down",
                "description": "Triggers when the price reaches or goes below the stopPrice."
              },
              {
                "value": "up",
                "name": "up",
                "description": "Triggers when the price reaches or goes above the stopPrice."
              },
              {
                "value": "",
                "name": "None",
                "description": "Not a stop order"
              }
            ]
          },
          "feeRate": {
            "type": "string",
            "description": "Fee Rate"
          },
          "fixFee": {
            "type": "string",
            "description": "Fixed fees (Deprecated field, no actual use of the value field)\n"
          },
          "feeCurrency": {
            "type": "string",
            "description": "Charging currency\n"
          },
          "tradeTime": {
            "type": "integer",
            "description": "Trade time in nanoseconds\n",
            "format": "int64"
          },
          "subTradeType": {
            "type": "string",
            "description": "Deprecated field, no actual use of the value field"
          },
          "marginMode": {
            "type": "string",
            "description": "Margin mode: ISOLATED (isolated), CROSS (cross margin).\n",
            "enum": ["ISOLATED", "CROSS"],
            "x-api-enum": [
              {
                "value": "ISOLATED",
                "name": "ISOLATED",
                "description": "Isolated Margin"
              },
              {
                "value": "CROSS",
                "name": "CROSS",
                "description": "Cross Margin"
              }
            ]
          },
          "displayType": {
            "type": "string",
            "enum": ["limit", "market", "limit_stop", "market_stop"],
            "description": "Order Type",
            "x-api-enum": [
              {
                "value": "limit",
                "name": "limit",
                "description": "Limit order"
              },
              {
                "value": "market",
                "name": "market",
                "description": "Market order"
              },
              {
                "value": "limit_stop",
                "name": "limit_stop",
                "description": "Stop limit order"
              },
              {
                "value": "market_stop",
                "name": "market_stop",
                "description": "Stop Market order"
              }
            ]
          },
          "fee": {
            "type": "string",
            "description": "Transaction fee\n"
          },
          "settleCurrency": {
            "type": "string",
            "description": "Settle Currency"
          },
          "orderType": {
            "type": "string",
            "description": "Order type\n",
            "enum": ["market", "limit"],
            "x-api-enum": [
              {
                "value": "market",
                "name": "market",
                "description": "market"
              },
              {
                "value": "limit",
                "name": "limit",
                "description": "limit"
              }
            ]
          },
          "tradeType": {
            "type": "string",
            "description": "Trade type (trade, liquid, cancel, adl or settlement)\n",
            "enum": ["trade", "cancel", "liquid", "adl", "settlement"],
            "x-api-enum": [
              {
                "value": "trade",
                "name": "trade",
                "description": "trade"
              },
              {
                "value": "cancel",
                "name": "cancel",
                "description": "Partially filled and canceled orders"
              },
              {
                "value": "liquid",
                "name": "liquid",
                "description": "liquid"
              },
              {
                "value": "adl",
                "name": "adl",
                "description": "adl"
              },
              {
                "value": "settlement",
                "name": "settlement",
                "description": "settlement"
              }
            ]
          },
          "createdAt": {
            "type": "integer",
            "description": "Order creation time\n",
            "format": "int64"
          }
        },
        "required": [
          "symbol",
          "tradeId",
          "orderId",
          "side",
          "liquidity",
          "forceTaker",
          "price",
          "size",
          "value",
          "openFeePay",
          "closeFeePay",
          "stop",
          "feeRate",
          "fixFee",
          "feeCurrency",
          "tradeTime",
          "subTradeType",
          "marginMode",
          "displayType",
          "fee",
          "settleCurrency",
          "orderType",
          "tradeType",
          "createdAt"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-recent-trade-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-recent-trade-history-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description                                                                                                        |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code            | string         | true     | none         | none                                                                                                               |
| » data            | [object]       | true     | none         | none                                                                                                               |
| »» symbol         | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» tradeId        | string         | true     | none         | Trade ID                                                                                                           |
| »» orderId        | string         | true     | none         | Order ID                                                                                                           |
| »» side           | string         | true     | none         | Transaction side                                                                                                   |
| »» liquidity      | string         | true     | none         | Liquidity-taker or -maker                                                                                          |
| »» forceTaker     | boolean        | true     | none         | Whether to force processing as a taker                                                                             |
| »» price          | string         | true     | none         | Filled price                                                                                                       |
| »» size           | integer        | true     | none         | Filled amount                                                                                                      |
| »» value          | string         | true     | none         | Order value                                                                                                        |
| »» openFeePay     | string         | true     | none         | Opening transaction fee                                                                                            |
| »» closeFeePay    | string         | true     | none         | Closing transaction fee                                                                                            |
| »» stop           | string         | true     | none         | A mark to the stop order type                                                                                      |
| »» feeRate        | string         | true     | none         | Fee Rate                                                                                                           |
| »» fixFee         | string         | true     | none         | Fixed fees (Deprecated field, no actual use of the value field)                                                    |
| »» feeCurrency    | string         | true     | none         | Charging currency                                                                                                  |
| »» tradeTime      | integer(int64) | true     | none         | Trade time in nanoseconds                                                                                          |
| »» subTradeType   | string         | true     | none         | Deprecated field, no actual use of the value field                                                                 |
| »» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                            |
| »» displayType    | string         | true     | none         | Order Type                                                                                                         |
| »» fee            | string         | true     | none         | Transaction fee                                                                                                    |
| »» settleCurrency | string         | true     | none         | Settle Currency                                                                                                    |
| »» orderType      | string         | true     | none         | Order type                                                                                                         |
| »» tradeType      | string         | true     | none         | Trade type (trade, liquid, cancel, adl or settlement)                                                              |
| »» createdAt      | integer(int64) | true     | none         | Order creation time                                                                                                |

#### Enumerated Values

| Property    | Value       |
| ----------- | ----------- |
| side        | buy         |
| side        | sell        |
| liquidity   | taker       |
| liquidity   | maker       |
| stop        | down        |
| stop        | up          |
| stop        |             |
| marginMode  | ISOLATED    |
| marginMode  | CROSS       |
| displayType | limit       |
| displayType | market      |
| displayType | limit_stop  |
| displayType | market_stop |
| orderType   | market      |
| orderType   | limit       |
| tradeType   | trade       |
| tradeType   | cancel      |
| tradeType   | liquid      |
| tradeType   | adl         |
| tradeType   | settlement  |

<aside class="success">
This operation does not require authentication
</aside>

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

## Get Current Funding Rate

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/funding-rate/{symbol}/current", {
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

r = requests.get('/api/v1/funding-rate/{symbol}/current', headers = headers)

print(r.json())

```

`GET /api/v1/funding-rate/{symbol}/current`

Get Current Funding Rate.

<h3 id="get-current-funding-rate-parameters">Parameters</h3>

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
          "description": "Funding Rate Symbol\n"
        },
        "granularity": {
          "type": "integer",
          "description": "Granularity (milliseconds)\n"
        },
        "timePoint": {
          "type": "integer",
          "description": "The funding rate settlement time point of the previous cycle\n(milliseconds)\nBefore going live, the system will pre-generate the first funding rate record to ensure the billing cycle can start immediately after the contract is launched.\n The timePoint field represents the time the funding rate data was generated, not the actual time it takes effect or is settled.\n The first actual settlement will occur at the designated settlement time (00:00 / 08:00 / 14:00) after the contract goes live.\n\n",
          "format": "int64"
        },
        "value": {
          "type": "number",
          "description": "Current cycle funding rate\n"
        },
        "predictedValue": {
          "type": "number",
          "description": "Predicted funding rate\n"
        },
        "fundingRateCap": {
          "type": "number",
          "description": "Maximum Funding Rate"
        },
        "fundingRateFloor": {
          "type": "number",
          "description": "Minimum Funding Rate"
        },
        "period": {
          "type": "integer",
          "enum": [1, 0],
          "description": "Indicates whether the current funding fee is charged within this cycle",
          "x-api-enum": [
            {
              "value": 1,
              "name": "1",
              "description": "Indicates that funding will be charged in the current cycle"
            },
            {
              "value": 0,
              "name": "0",
              "description": "Indicates a cross-cycle expense record that is not charged in the current cycle."
            }
          ]
        },
        "fundingTime": {
          "type": "integer",
          "description": "Indicates the next funding fee settlement time point, which can be used to synchronize periodic settlement timing.",
          "format": "int64"
        }
      },
      "required": [
        "symbol",
        "granularity",
        "timePoint",
        "value",
        "predictedValue",
        "fundingRateCap",
        "fundingRateFloor",
        "period",
        "fundingTime"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-current-funding-rate-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-current-funding-rate-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ------------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| » code              | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » data              | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» symbol           | string         | true     | none         | Funding Rate Symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| »» granularity      | integer        | true     | none         | Granularity (milliseconds)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| »» timePoint        | integer(int64) | true     | none         | The funding rate settlement time point of the previous cycle<br>(milliseconds)<br>Before going live, the system will pre-generate the first funding rate record to ensure the billing cycle can start immediately after the contract is launched.<br> The timePoint field represents the time the funding rate data was generated, not the actual time it takes effect or is settled.<br> The first actual settlement will occur at the designated settlement time (00:00 / 08:00 / 14:00) after the contract goes live. |
| »» value            | number         | true     | none         | Current cycle funding rate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| »» predictedValue   | number         | true     | none         | Predicted funding rate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| »» fundingRateCap   | number         | true     | none         | Maximum Funding Rate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» fundingRateFloor | number         | true     | none         | Minimum Funding Rate                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» period           | integer        | true     | none         | Indicates whether the current funding fee is charged within this cycle                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| »» fundingTime      | integer(int64) | true     | none         | Indicates the next funding fee settlement time point, which can be used to synchronize periodic settlement timing.                                                                                                                                                                                                                                                                                                                                                                                                       |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| period   | 1     |
| period   | 0     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Public Funding History

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/contract/funding-rates?symbol=type,string&from=type,integer,format,int64&to=type,integer,format,int64",
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

r = requests.get('/api/v1/contract/funding-rates', params={
  'symbol': {
  "type": "string"
},  'from': {
  "type": "integer",
  "format": "int64"
},  'to': {
  "type": "integer",
  "format": "int64"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/contract/funding-rates`

Query the funding rate at each settlement time point within a certain time range
of the corresponding contract.

<h3 id="get-public-funding-history-parameters">Parameters</h3>

| Name   | In    | Type           | Required | Description                                                                                                        |
| ------ | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------ |
| symbol | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| from   | query | integer(int64) | true     | Begin time (milliseconds)                                                                                          |
| to     | query | integer(int64) | true     | End time (milliseconds)                                                                                            |

#### Detailed descriptions

**from**: Begin time (milliseconds)

**to**: End time (milliseconds)

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
          "fundingRate": {
            "type": "number",
            "description": "Funding rate"
          },
          "timepoint": {
            "type": "integer",
            "description": "Time point (milliseconds)\n\n",
            "format": "int64"
          }
        },
        "required": ["symbol", "fundingRate", "timepoint"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-public-funding-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-public-funding-history-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                                                                                        |
| -------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code         | string         | true     | none         | none                                                                                                               |
| » data         | [object]       | true     | none         | none                                                                                                               |
| »» symbol      | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »» fundingRate | number         | true     | none         | Funding rate                                                                                                       |
| »» timepoint   | integer(int64) | true     | none         | Time point (milliseconds)                                                                                          |

<aside class="success">
This operation does not require authentication
</aside>

## Get Private Funding History

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/funding-history?symbol=type,string", {
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

r = requests.get('/api/v1/funding-history', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/funding-history`

Submit request to get the funding history.

<h3 id="get-private-funding-history-parameters">Parameters</h3>

| Name     | In    | Type           | Required | Description                                                                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol   | query | string         | true     | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220)                                       |
| startAt  | query | integer(int64) | false    | Begin time (milliseconds)                                                                                                                                |
| endAt    | query | integer(int64) | false    | End time (milliseconds)                                                                                                                                  |
| reverse  | query | boolean        | false    | This parameter functions to judge whether the lookup is forward or not. True means “yes” and False means “no”. This parameter is set as true by default. |
| offset   | query | integer        | false    | Start offset. The unique attribute of the last returned result of the last request. The data of the first page will be returned by default.              |
| forward  | query | boolean        | false    | This parameter functions to judge whether the lookup is forward or not. True means “yes” and False means “no”. This parameter is set as true by default. |
| maxCount | query | integer        | false    | Max. record count. The default record count is 10                                                                                                        |

#### Detailed descriptions

**startAt**: Begin time (milliseconds)

**endAt**: End time (milliseconds)

**reverse**: This parameter functions to judge whether the lookup is forward or
not. True means “yes” and False means “no”. This parameter is set as true by
default.

**offset**: Start offset. The unique attribute of the last returned result of
the last request. The data of the first page will be returned by default.

**forward**: This parameter functions to judge whether the lookup is forward or
not. True means “yes” and False means “no”. This parameter is set as true by
default.

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
              "id": {
                "type": "integer",
                "description": "ID",
                "format": "int64"
              },
              "symbol": {
                "type": "string",
                "description": "Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) "
              },
              "timePoint": {
                "type": "integer",
                "description": "Time point (milliseconds)\n",
                "format": "int64"
              },
              "fundingRate": {
                "type": "number",
                "description": "Funding rate\n"
              },
              "markPrice": {
                "type": "number",
                "description": "Mark price\n"
              },
              "positionQty": {
                "type": "integer",
                "description": "Position size"
              },
              "positionCost": {
                "type": "number",
                "description": "Position value at settlement period\n"
              },
              "funding": {
                "type": "number",
                "description": "Settled funding fees A positive number means that the user received the funding fee, and vice versa.\n"
              },
              "settleCurrency": {
                "type": "string",
                "description": "Settlement currency\n"
              },
              "context": {
                "type": "string",
                "description": "Context"
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
            "required": [
              "id",
              "symbol",
              "timePoint",
              "fundingRate",
              "markPrice",
              "positionQty",
              "positionCost",
              "funding",
              "settleCurrency",
              "context",
              "marginMode"
            ]
          }
        },
        "hasMore": {
          "type": "boolean",
          "description": "Whether there are more pages\n"
        }
      },
      "required": ["dataList", "hasMore"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-private-funding-history-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-private-funding-history-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                        |
| ------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code             | string         | true     | none         | none                                                                                                               |
| » data             | object         | true     | none         | none                                                                                                               |
| »» dataList        | [object]       | true     | none         | none                                                                                                               |
| »»» id             | integer(int64) | true     | none         | ID                                                                                                                 |
| »»» symbol         | string         | true     | none         | Symbol of the contract. Please refer to [Get Symbol endpoint: symbol](https://www.kucoin.com/docs-new/api-3470220) |
| »»» timePoint      | integer(int64) | true     | none         | Time point (milliseconds)                                                                                          |
| »»» fundingRate    | number         | true     | none         | Funding rate                                                                                                       |
| »»» markPrice      | number         | true     | none         | Mark price                                                                                                         |
| »»» positionQty    | integer        | true     | none         | Position size                                                                                                      |
| »»» positionCost   | number         | true     | none         | Position value at settlement period                                                                                |
| »»» funding        | number         | true     | none         | Settled funding fees A positive number means that the user received the funding fee, and vice versa.               |
| »»» settleCurrency | string         | true     | none         | Settlement currency                                                                                                |
| »»» context        | string         | true     | none         | Context                                                                                                            |
| »»» marginMode     | string         | true     | none         | Margin mode: ISOLATED (isolated), CROSS (cross margin).                                                            |
| »» hasMore         | boolean        | true     | none         | Whether there are more pages                                                                                       |

#### Enumerated Values

| Property   | Value    |
| ---------- | -------- |
| marginMode | ISOLATED |
| marginMode | CROSS    |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
