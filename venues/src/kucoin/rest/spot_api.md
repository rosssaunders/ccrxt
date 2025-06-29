---
title: KuCoin Spot REST API v1.0.0
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

<h1 id="kucoin-spot-rest-api">KuCoin Spot REST API v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

market

<h1 id="kucoin-spot-rest-api-default">Default</h1>

## Get Announcements

<a id="opId001"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/announcements", {
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

r = requests.get('/api/v3/announcements', headers = headers)

print(r.json())

```

`GET /api/v3/announcements`

This interface can obtain the latest news announcements, and the default page
search is for announcements within a month.

<h3 id="get-announcements-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                          |
| ----------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currentPage | query | integer(int64) | false    | page number                                                                                                                                                                                                                                                                                                                                                          |
| pageSize    | query | integer(int64) | false    | page Size                                                                                                                                                                                                                                                                                                                                                            |
| annType     | query | string         | false    | Announcement types: latest-announcements , activities (latest activities), new-listings (new currency online), product-updates (product updates), vip (institutions and VIPs), maintenance-updates (system maintenance), product -updates (product news), delistings (currency offline), others, api-campaigns (API user activities), default : latest-announcements |
| lang        | query | string         | false    | Language type, the default is en_US, the specific value parameters are as follows                                                                                                                                                                                                                                                                                    |
| startTime   | query | integer(int64) | false    | Announcement online start time (milliseconds)                                                                                                                                                                                                                                                                                                                        |
| endTime     | query | integer(int64) | false    | Announcement online end time (milliseconds)                                                                                                                                                                                                                                                                                                                          |

#### Enumerated Values

| Parameter | Value                |
| --------- | -------------------- |
| annType   | latest-announcements |
| annType   | activities           |
| annType   | product-updates      |
| annType   | vip                  |
| annType   | maintenance-updates  |
| annType   | delistings           |
| annType   | others               |
| annType   | api-campaigns        |
| annType   | new-listings         |
| lang      | zh_HK                |
| lang      | ja_JP                |
| lang      | ko_KR                |
| lang      | en_US                |
| lang      | pl_PL                |
| lang      | es_ES                |
| lang      | fr_FR                |
| lang      | ar_AE                |
| lang      | it_IT                |
| lang      | id_ID                |
| lang      | nl_NL                |
| lang      | pt_PT                |
| lang      | vi_VN                |
| lang      | de_DE                |
| lang      | tr_TR                |
| lang      | ms_MY                |
| lang      | ru_RU                |
| lang      | th_TH                |
| lang      | hi_IN                |
| lang      | bn_BD                |
| lang      | fil_PH               |
| lang      | ur_PK                |

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
        "totalNum": {
          "type": "integer",
          "description": "Total Number"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "annId": {
                "type": "integer",
                "description": "Announcement ID"
              },
              "annTitle": {
                "type": "string",
                "description": "Announcement title"
              },
              "annType": {
                "type": "array",
                "items": {
                  "type": "string"
                },
                "description": "Announcement type"
              },
              "annDesc": {
                "type": "string",
                "description": "Announcement description"
              },
              "cTime": {
                "type": "integer",
                "description": "Announcement release time, Unix millisecond timestamp format",
                "format": "int64"
              },
              "language": {
                "type": "string",
                "description": "language type"
              },
              "annUrl": {
                "type": "string",
                "description": "Announcement link"
              }
            },
            "required": [
              "annId",
              "annTitle",
              "annType",
              "annDesc",
              "cTime",
              "language",
              "annUrl"
            ]
          }
        },
        "currentPage": {
          "type": "integer",
          "description": "Current page"
        },
        "pageSize": {
          "type": "integer",
          "description": "Page size"
        },
        "totalPage": {
          "type": "integer",
          "description": "Total Page"
        }
      },
      "required": ["totalNum", "items", "currentPage", "pageSize", "totalPage"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-announcements-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-announcements-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                                  |
| -------------- | -------------- | -------- | ------------ | ------------------------------------------------------------ |
| » code         | string         | true     | none         | none                                                         |
| » data         | object         | true     | none         | none                                                         |
| »» totalNum    | integer        | true     | none         | Total Number                                                 |
| »» items       | [object]       | true     | none         | none                                                         |
| »»» annId      | integer        | true     | none         | Announcement ID                                              |
| »»» annTitle   | string         | true     | none         | Announcement title                                           |
| »»» annType    | [string]       | true     | none         | Announcement type                                            |
| »»» annDesc    | string         | true     | none         | Announcement description                                     |
| »»» cTime      | integer(int64) | true     | none         | Announcement release time, Unix millisecond timestamp format |
| »»» language   | string         | true     | none         | language type                                                |
| »»» annUrl     | string         | true     | none         | Announcement link                                            |
| »» currentPage | integer        | true     | none         | Current page                                                 |
| »» pageSize    | integer        | true     | none         | Page size                                                    |
| »» totalPage   | integer        | true     | none         | Total Page                                                   |

<aside class="success">
This operation does not require authentication
</aside>

## Get Currency

<a id="opId002"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/currencies/{currency}", {
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

r = requests.get('/api/v3/currencies/{currency}', headers = headers)

print(r.json())

```

`GET /api/v3/currencies/{currency}`

Request the currency details of a specified currency via this endpoint.

<h3 id="get-currency-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description                                                                                                                                                                              |
| -------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| chain    | query | string | false    | Support for querying the chain of currency, e.g. the available values for USDT are OMNI, ERC20, TRC20. This only applies to multi-chain currencies; no need for single-chain currencies. |
| currency | path  | string | true     | Path parameter, Currency                                                                                                                                                                 |

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
        "currency": {
          "type": "string",
          "description": "A unique currency code that will never change"
        },
        "name": {
          "type": "string",
          "description": "Currency name; will change after renaming"
        },
        "fullName": {
          "type": "string",
          "description": "Full currency name; will change after renaming"
        },
        "precision": {
          "type": "integer",
          "description": "Currency precision"
        },
        "confirms": {
          "type": "integer",
          "description": "Number of block confirmations"
        },
        "contractAddress": {
          "type": "string",
          "description": "Contract address"
        },
        "isMarginEnabled": {
          "type": "boolean",
          "description": "Margin support or not"
        },
        "isDebitEnabled": {
          "type": "boolean",
          "description": "Debit support or not"
        },
        "chains": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "chainName": {
                "type": "string",
                "description": "Chain name of currency"
              },
              "withdrawalMinSize": {
                "type": "string",
                "description": "Minimum withdrawal amount"
              },
              "depositMinSize": {
                "type": "string",
                "description": "Minimum deposit amount"
              },
              "withdrawFeeRate": {
                "type": "string",
                "description": "Withdraw fee rate"
              },
              "withdrawalMinFee": {
                "type": "string",
                "description": "Minimum fees charged for withdrawal"
              },
              "isWithdrawEnabled": {
                "type": "boolean",
                "description": "Withdrawal support or not"
              },
              "isDepositEnabled": {
                "type": "boolean",
                "description": "Deposit support or not"
              },
              "confirms": {
                "type": "integer",
                "description": "Number of block confirmations"
              },
              "preConfirms": {
                "type": "integer",
                "description": "The number of blocks (confirmations) for advance on-chain verification"
              },
              "contractAddress": {
                "type": "string",
                "description": "Contract address"
              },
              "withdrawPrecision": {
                "type": "integer",
                "description": "Withdrawal precision bit, indicating the maximum supported length after the decimal point of the withdrawal amount"
              },
              "maxWithdraw": {
                "type": "number",
                "description": "Maximum amount of single withdrawal"
              },
              "maxDeposit": {
                "type": "string",
                "description": "Maximum amount of single deposit (only applicable to Lightning Network)"
              },
              "needTag": {
                "type": "boolean",
                "description": "Need for memo/tag or not"
              },
              "chainId": {
                "type": "string",
                "description": "Chain id of currency"
              }
            },
            "required": [
              "chainName",
              "withdrawalMinSize",
              "withdrawFeeRate",
              "withdrawalMinFee",
              "isWithdrawEnabled",
              "isDepositEnabled",
              "confirms",
              "preConfirms",
              "contractAddress",
              "withdrawPrecision",
              "needTag",
              "chainId"
            ]
          },
          "description": "Chain list"
        }
      },
      "required": [
        "currency",
        "name",
        "fullName",
        "precision",
        "isMarginEnabled",
        "isDebitEnabled",
        "chains"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-currency-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-currency-responseschema">Response Schema</h3>

Status Code **200**

| Name                  | Type     | Required | Restrictions | Description                                                                                                        |
| --------------------- | -------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code                | string   | true     | none         | none                                                                                                               |
| » data                | object   | true     | none         | none                                                                                                               |
| »» currency           | string   | true     | none         | A unique currency code that will never change                                                                      |
| »» name               | string   | true     | none         | Currency name; will change after renaming                                                                          |
| »» fullName           | string   | true     | none         | Full currency name; will change after renaming                                                                     |
| »» precision          | integer  | true     | none         | Currency precision                                                                                                 |
| »» confirms           | integer  | false    | none         | Number of block confirmations                                                                                      |
| »» contractAddress    | string   | false    | none         | Contract address                                                                                                   |
| »» isMarginEnabled    | boolean  | true     | none         | Margin support or not                                                                                              |
| »» isDebitEnabled     | boolean  | true     | none         | Debit support or not                                                                                               |
| »» chains             | [object] | true     | none         | Chain list                                                                                                         |
| »»» chainName         | string   | true     | none         | Chain name of currency                                                                                             |
| »»» withdrawalMinSize | string   | true     | none         | Minimum withdrawal amount                                                                                          |
| »»» depositMinSize    | string   | false    | none         | Minimum deposit amount                                                                                             |
| »»» withdrawFeeRate   | string   | true     | none         | Withdraw fee rate                                                                                                  |
| »»» withdrawalMinFee  | string   | true     | none         | Minimum fees charged for withdrawal                                                                                |
| »»» isWithdrawEnabled | boolean  | true     | none         | Withdrawal support or not                                                                                          |
| »»» isDepositEnabled  | boolean  | true     | none         | Deposit support or not                                                                                             |
| »»» confirms          | integer  | true     | none         | Number of block confirmations                                                                                      |
| »»» preConfirms       | integer  | true     | none         | The number of blocks (confirmations) for advance on-chain verification                                             |
| »»» contractAddress   | string   | true     | none         | Contract address                                                                                                   |
| »»» withdrawPrecision | integer  | true     | none         | Withdrawal precision bit, indicating the maximum supported length after the decimal point of the withdrawal amount |
| »»» maxWithdraw       | number   | false    | none         | Maximum amount of single withdrawal                                                                                |
| »»» maxDeposit        | string   | false    | none         | Maximum amount of single deposit (only applicable to Lightning Network)                                            |
| »»» needTag           | boolean  | true     | none         | Need for memo/tag or not                                                                                           |
| »»» chainId           | string   | true     | none         | Chain id of currency                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Get All Currencies

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/currencies", {
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

r = requests.get('/api/v3/currencies', headers = headers)

print(r.json())

```

`GET /api/v3/currencies`

Request a currency list via this endpoint. Not all currencies can currently be
used for trading.

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
            "description": "A unique currency code that will never change"
          },
          "name": {
            "type": "string",
            "description": "Currency name; will change after renaming"
          },
          "fullName": {
            "type": "string",
            "description": "Full currency name; will change after renaming"
          },
          "precision": {
            "type": "integer",
            "description": "Currency precision"
          },
          "confirms": {
            "type": "integer",
            "description": "Number of block confirmations"
          },
          "contractAddress": {
            "type": "string",
            "description": "Contract address"
          },
          "isMarginEnabled": {
            "type": "boolean",
            "description": "Margin support or not"
          },
          "isDebitEnabled": {
            "type": "boolean",
            "description": "Debit support or not"
          },
          "chains": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "chainName": {
                  "type": "string",
                  "description": "Chain name of currency"
                },
                "withdrawalMinSize": {
                  "type": "string",
                  "description": "Minimum withdrawal amount"
                },
                "depositMinSize": {
                  "type": "string",
                  "description": "Minimum deposit amount"
                },
                "withdrawFeeRate": {
                  "type": "string",
                  "description": "Withdraw fee rate"
                },
                "withdrawalMinFee": {
                  "type": "string",
                  "description": "Minimum fees charged for withdrawal"
                },
                "isWithdrawEnabled": {
                  "type": "boolean",
                  "description": "Withdrawal support or not"
                },
                "isDepositEnabled": {
                  "type": "boolean",
                  "description": "Deposit support or not"
                },
                "confirms": {
                  "type": "integer",
                  "description": "Number of block confirmations"
                },
                "preConfirms": {
                  "type": "integer",
                  "description": "The number of blocks (confirmations) for advance on-chain verification"
                },
                "contractAddress": {
                  "type": "string",
                  "description": "Contract address"
                },
                "withdrawPrecision": {
                  "type": "integer",
                  "description": "Withdrawal precision bit, indicating the maximum supported length after the decimal point of the withdrawal amount"
                },
                "maxWithdraw": {
                  "type": "string",
                  "description": "Maximum amount of single withdrawal"
                },
                "maxDeposit": {
                  "type": "string",
                  "description": "Maximum amount of single deposit (only applicable to Lightning Network)"
                },
                "needTag": {
                  "type": "boolean",
                  "description": "Need for memo/tag or not"
                },
                "chainId": {
                  "type": "string",
                  "description": "Chain id of currency"
                },
                "depositFeeRate": {
                  "type": "string",
                  "description": "Deposit fee rate (some currencies have this param; the default is empty)"
                },
                "withdrawMaxFee": {
                  "type": "string",
                  "description": "Withdraw max. fee (some currencies have this param; the default is empty)"
                },
                "depositTierFee": {
                  "type": "string"
                }
              },
              "required": [
                "chainName",
                "withdrawalMinSize",
                "withdrawalMinFee",
                "isWithdrawEnabled",
                "isDepositEnabled",
                "confirms",
                "preConfirms",
                "contractAddress",
                "withdrawPrecision",
                "needTag",
                "chainId",
                "withdrawFeeRate"
              ]
            },
            "description": "Chain list"
          }
        },
        "required": [
          "currency",
          "name",
          "fullName",
          "precision",
          "confirms",
          "contractAddress",
          "isMarginEnabled",
          "isDebitEnabled",
          "chains"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-all-currencies-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-all-currencies-responseschema">Response Schema</h3>

Status Code **200**

| Name                  | Type     | Required | Restrictions | Description                                                                                                        |
| --------------------- | -------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------ |
| » code                | string   | true     | none         | none                                                                                                               |
| » data                | [object] | true     | none         | none                                                                                                               |
| »» currency           | string   | true     | none         | A unique currency code that will never change                                                                      |
| »» name               | string   | true     | none         | Currency name; will change after renaming                                                                          |
| »» fullName           | string   | true     | none         | Full currency name; will change after renaming                                                                     |
| »» precision          | integer  | true     | none         | Currency precision                                                                                                 |
| »» confirms           | integer  | true     | none         | Number of block confirmations                                                                                      |
| »» contractAddress    | string   | true     | none         | Contract address                                                                                                   |
| »» isMarginEnabled    | boolean  | true     | none         | Margin support or not                                                                                              |
| »» isDebitEnabled     | boolean  | true     | none         | Debit support or not                                                                                               |
| »» chains             | [object] | true     | none         | Chain list                                                                                                         |
| »»» chainName         | string   | true     | none         | Chain name of currency                                                                                             |
| »»» withdrawalMinSize | string   | true     | none         | Minimum withdrawal amount                                                                                          |
| »»» depositMinSize    | string   | false    | none         | Minimum deposit amount                                                                                             |
| »»» withdrawFeeRate   | string   | true     | none         | Withdraw fee rate                                                                                                  |
| »»» withdrawalMinFee  | string   | true     | none         | Minimum fees charged for withdrawal                                                                                |
| »»» isWithdrawEnabled | boolean  | true     | none         | Withdrawal support or not                                                                                          |
| »»» isDepositEnabled  | boolean  | true     | none         | Deposit support or not                                                                                             |
| »»» confirms          | integer  | true     | none         | Number of block confirmations                                                                                      |
| »»» preConfirms       | integer  | true     | none         | The number of blocks (confirmations) for advance on-chain verification                                             |
| »»» contractAddress   | string   | true     | none         | Contract address                                                                                                   |
| »»» withdrawPrecision | integer  | true     | none         | Withdrawal precision bit, indicating the maximum supported length after the decimal point of the withdrawal amount |
| »»» maxWithdraw       | string   | false    | none         | Maximum amount of single withdrawal                                                                                |
| »»» maxDeposit        | string   | false    | none         | Maximum amount of single deposit (only applicable to Lightning Network)                                            |
| »»» needTag           | boolean  | true     | none         | Need for memo/tag or not                                                                                           |
| »»» chainId           | string   | true     | none         | Chain id of currency                                                                                               |
| »»» depositFeeRate    | string   | false    | none         | Deposit fee rate (some currencies have this param; the default is empty)                                           |
| »»» withdrawMaxFee    | string   | false    | none         | Withdraw max. fee (some currencies have this param; the default is empty)                                          |
| »»» depositTierFee    | string   | false    | none         | none                                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Get Symbol

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v2/symbols/{symbol}", {
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

r = requests.get('/api/v2/symbols/{symbol}', headers = headers)

print(r.json())

```

`GET /api/v2/symbols/{symbol}`

Request via this endpoint to get detail currency pairs for trading. If you want
to get the market information of the trading symbol, please use Get All Tickers.

<h3 id="get-symbol--parameters">Parameters</h3>

| Name   | In   | Type   | Required | Description            |
| ------ | ---- | ------ | -------- | ---------------------- |
| symbol | path | string | true     | Path parameter, Symbol |

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
          "description": "unique code of a symbol, it would not change after renaming",
          "example": ["BTC-USDT", "BCHSV-USDT"]
        },
        "name": {
          "type": "string",
          "description": "Name of trading pairs, it would change after renaming",
          "example": ["BTC-USDT", "BSV-USDT"]
        },
        "baseCurrency": {
          "type": "string",
          "description": "Base currency,e.g. BTC."
        },
        "quoteCurrency": {
          "type": "string",
          "description": "Quote currency,e.g. USDT."
        },
        "feeCurrency": {
          "type": "string",
          "description": "The currency of charged fees."
        },
        "market": {
          "type": "string",
          "description": "The trading market.",
          "example": ["USDS", "BTC", "ALTS"]
        },
        "baseMinSize": {
          "type": "string",
          "description": "The minimum order quantity requried to place an order."
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
        "baseIncrement": {
          "type": "string",
          "description": "Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001."
        },
        "quoteIncrement": {
          "type": "string",
          "description": "Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001."
        },
        "priceIncrement": {
          "type": "string",
          "description": "Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001."
        },
        "priceLimitRate": {
          "type": "string",
          "description": "Threshold for price portection"
        },
        "minFunds": {
          "type": "string",
          "description": "The minimum trading amounts"
        },
        "isMarginEnabled": {
          "type": "boolean",
          "description": "Available for margin or not."
        },
        "enableTrading": {
          "type": "boolean",
          "description": "Available for transaction or not."
        },
        "feeCategory": {
          "type": "integer",
          "description": "[Fee Type](https://www.kucoin.com/vip/privilege)",
          "enum": [1, 2, 3],
          "x-api-enum": [
            {
              "value": 1,
              "name": "classA",
              "description": "classA"
            },
            {
              "value": 2,
              "name": "classB",
              "description": "classB"
            },
            {
              "value": 3,
              "name": "classC",
              "description": "classC"
            }
          ]
        },
        "makerFeeCoefficient": {
          "type": "string",
          "description": "The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
        },
        "takerFeeCoefficient": {
          "type": "string",
          "description": "The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
        },
        "st": {
          "type": "boolean",
          "description": "Whether it is a [Special Treatment](https://www.kucoin.com/legal/special-treatment) symbol"
        },
        "callauctionIsEnabled": {
          "type": "boolean",
          "description": "The  [call auction](https://www.kucoin.com/support/40999744334105) status returns true/false"
        },
        "callauctionPriceFloor": {
          "type": "string",
          "description": "The lowest price declared in the call auction",
          "example": ["0.00001"]
        },
        "callauctionPriceCeiling": {
          "type": "string",
          "description": "The highest bid price in the call auction\n",
          "example": ["0.6"]
        },
        "callauctionFirstStageStartTime": {
          "type": "integer",
          "description": "The first phase of the call auction starts at (Allow add orders, allow cancel orders)",
          "format": "int64",
          "example": [1740470400000]
        },
        "callauctionSecondStageStartTime": {
          "type": "integer",
          "description": "The second phase of the call auction starts at (Allow add orders, don't allow cancel orders)",
          "format": "int64",
          "example": [1740473400000]
        },
        "callauctionThirdStageStartTime": {
          "type": "integer",
          "description": "The third phase of the call auction starts at (Don't allow add orders, don't allow cancel orders)",
          "format": "int64",
          "example": [1740473700000]
        },
        "tradingStartTime": {
          "type": "integer",
          "description": "Official opening time (end time of the third phase of call auction)",
          "format": "int64",
          "example": [1740474000000]
        }
      },
      "required": [
        "symbol",
        "name",
        "baseCurrency",
        "quoteCurrency",
        "feeCurrency",
        "market",
        "baseMinSize",
        "quoteMinSize",
        "baseMaxSize",
        "quoteMaxSize",
        "baseIncrement",
        "quoteIncrement",
        "priceIncrement",
        "priceLimitRate",
        "minFunds",
        "isMarginEnabled",
        "enableTrading",
        "feeCategory",
        "makerFeeCoefficient",
        "takerFeeCoefficient",
        "st",
        "callauctionIsEnabled"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-symbol--responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-symbol--responseschema">Response Schema</h3>

Status Code **200**

| Name                               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                           |
| ---------------------------------- | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code                             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                  |
| » data                             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                  |
| »» symbol                          | string         | true     | none         | unique code of a symbol, it would not change after renaming                                                                                                                                                                                                                                                           |
| »» name                            | string         | true     | none         | Name of trading pairs, it would change after renaming                                                                                                                                                                                                                                                                 |
| »» baseCurrency                    | string         | true     | none         | Base currency,e.g. BTC.                                                                                                                                                                                                                                                                                               |
| »» quoteCurrency                   | string         | true     | none         | Quote currency,e.g. USDT.                                                                                                                                                                                                                                                                                             |
| »» feeCurrency                     | string         | true     | none         | The currency of charged fees.                                                                                                                                                                                                                                                                                         |
| »» market                          | string         | true     | none         | The trading market.                                                                                                                                                                                                                                                                                                   |
| »» baseMinSize                     | string         | true     | none         | The minimum order quantity requried to place an order.                                                                                                                                                                                                                                                                |
| »» quoteMinSize                    | string         | true     | none         | The minimum order funds required to place a market order.                                                                                                                                                                                                                                                             |
| »» baseMaxSize                     | string         | true     | none         | The maximum order size required to place an order.                                                                                                                                                                                                                                                                    |
| »» quoteMaxSize                    | string         | true     | none         | The maximum order funds required to place a market order.                                                                                                                                                                                                                                                             |
| »» baseIncrement                   | string         | true     | none         | Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001. |
| »» quoteIncrement                  | string         | true     | none         | Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001.           |
| »» priceIncrement                  | string         | true     | none         | Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001.                                                                                             |
| »» priceLimitRate                  | string         | true     | none         | Threshold for price portection                                                                                                                                                                                                                                                                                        |
| »» minFunds                        | string         | true     | none         | The minimum trading amounts                                                                                                                                                                                                                                                                                           |
| »» isMarginEnabled                 | boolean        | true     | none         | Available for margin or not.                                                                                                                                                                                                                                                                                          |
| »» enableTrading                   | boolean        | true     | none         | Available for transaction or not.                                                                                                                                                                                                                                                                                     |
| »» feeCategory                     | integer        | true     | none         | [Fee Type](https://www.kucoin.com/vip/privilege)                                                                                                                                                                                                                                                                      |
| »» makerFeeCoefficient             | string         | true     | none         | The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee                                                                                                                                      |
| »» takerFeeCoefficient             | string         | true     | none         | The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee                                                                                                                                      |
| »» st                              | boolean        | true     | none         | Whether it is a [Special Treatment](https://www.kucoin.com/legal/special-treatment) symbol                                                                                                                                                                                                                            |
| »» callauctionIsEnabled            | boolean        | true     | none         | The [call auction](https://www.kucoin.com/support/40999744334105) status returns true/false                                                                                                                                                                                                                           |
| »» callauctionPriceFloor           | string         | false    | none         | The lowest price declared in the call auction                                                                                                                                                                                                                                                                         |
| »» callauctionPriceCeiling         | string         | false    | none         | The highest bid price in the call auction                                                                                                                                                                                                                                                                             |
| »» callauctionFirstStageStartTime  | integer(int64) | false    | none         | The first phase of the call auction starts at (Allow add orders, allow cancel orders)                                                                                                                                                                                                                                 |
| »» callauctionSecondStageStartTime | integer(int64) | false    | none         | The second phase of the call auction starts at (Allow add orders, don't allow cancel orders)                                                                                                                                                                                                                          |
| »» callauctionThirdStageStartTime  | integer(int64) | false    | none         | The third phase of the call auction starts at (Don't allow add orders, don't allow cancel orders)                                                                                                                                                                                                                     |
| »» tradingStartTime                | integer(int64) | false    | none         | Official opening time (end time of the third phase of call auction)                                                                                                                                                                                                                                                   |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| feeCategory | 1     |
| feeCategory | 2     |
| feeCategory | 3     |

<aside class="success">
This operation does not require authentication
</aside>

## Get All Symbols

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v2/symbols", {
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

r = requests.get('/api/v2/symbols', headers = headers)

print(r.json())

```

`GET /api/v2/symbols`

Request a list of available currency pairs for trading via this endpoint. If you
want to get the market information of the trading symbol, please use Get All
Tickers.

<h3 id="get-all-symbols-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description                                                         |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------- |
| market | query | string | false    | [The trading market](https://www.kucoin.com/docs-new/api-222921786) |

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
            "description": "Unique code of a symbol; it will not change after renaming",
            "example": ["BTC-USDT", "BCHSV-USDT"]
          },
          "name": {
            "type": "string",
            "description": "Name of trading pairs, it will change after renaming",
            "example": ["BTC-USDT", "BSV-USDT"]
          },
          "baseCurrency": {
            "type": "string",
            "description": "Base currency, e.g. BTC."
          },
          "quoteCurrency": {
            "type": "string",
            "description": "Quote currency, e.g. USDT."
          },
          "feeCurrency": {
            "type": "string",
            "description": "The currency of charged fees."
          },
          "market": {
            "type": "string",
            "description": "The trading market.",
            "example": ["USDS", "BTC", "ALTS"]
          },
          "baseMinSize": {
            "type": "string",
            "description": "The minimum order quantity required to place an order."
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
          "baseIncrement": {
            "type": "string",
            "description": "Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001."
          },
          "quoteIncrement": {
            "type": "string",
            "description": "Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001."
          },
          "priceIncrement": {
            "type": "string",
            "description": "Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001.\n\nSpecifies the min. order price as well as the price increment.This also applies to quote currency."
          },
          "priceLimitRate": {
            "type": "string",
            "description": "Threshold for price protection"
          },
          "minFunds": {
            "type": "string",
            "description": "The minimum trading amounts"
          },
          "isMarginEnabled": {
            "type": "boolean",
            "description": "Available for margin or not."
          },
          "enableTrading": {
            "type": "boolean",
            "description": "Available for transaction or not."
          },
          "feeCategory": {
            "type": "integer",
            "description": "[Fee Type](https://www.kucoin.com/vip/privilege)",
            "enum": [1, 2, 3],
            "x-api-enum": [
              {
                "value": 1,
                "name": "classA",
                "description": "classA"
              },
              {
                "value": 2,
                "name": "classB",
                "description": "classB"
              },
              {
                "value": 3,
                "name": "classC",
                "description": "classC"
              }
            ]
          },
          "makerFeeCoefficient": {
            "type": "string",
            "description": "The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
          },
          "takerFeeCoefficient": {
            "type": "string",
            "description": "The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
          },
          "st": {
            "type": "boolean",
            "description": "Whether it is a [Special Treatment](https://www.kucoin.com/legal/special-treatment) symbol"
          },
          "callauctionIsEnabled": {
            "type": "boolean",
            "description": "The [call auction](https://www.kucoin.com/support/40999744334105) status returns true/false"
          },
          "callauctionPriceFloor": {
            "type": "string",
            "description": "The lowest price declared in the call auction",
            "example": ["0.00001"]
          },
          "callauctionPriceCeiling": {
            "type": "string",
            "description": "The highest bid price in the call auction\n",
            "example": ["0.6"]
          },
          "callauctionFirstStageStartTime": {
            "type": "integer",
            "description": "The first phase of the call auction starts at (Allow add orders, allow cancel orders)",
            "format": "int64",
            "example": [1740470400000]
          },
          "callauctionSecondStageStartTime": {
            "type": "integer",
            "description": "The second phase of the call auction starts at (Allow add orders, don't allow cancel orders)",
            "format": "int64",
            "example": [1740473400000]
          },
          "callauctionThirdStageStartTime": {
            "type": "integer",
            "description": "The third phase of the call auction starts at (Don't allow add orders, don't allow cancel orders)",
            "format": "int64",
            "example": [1740473700000]
          },
          "tradingStartTime": {
            "type": "integer",
            "description": "Official opening time (end time of the third phase of call auction)",
            "format": "int64",
            "example": [1740474000000]
          }
        },
        "required": [
          "symbol",
          "name",
          "baseCurrency",
          "quoteCurrency",
          "feeCurrency",
          "market",
          "baseMinSize",
          "quoteMinSize",
          "baseMaxSize",
          "quoteMaxSize",
          "baseIncrement",
          "quoteIncrement",
          "priceIncrement",
          "priceLimitRate",
          "minFunds",
          "isMarginEnabled",
          "enableTrading",
          "feeCategory",
          "makerFeeCoefficient",
          "takerFeeCoefficient",
          "st",
          "callauctionIsEnabled"
        ]
      }
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

| Name                               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                         |
| ---------------------------------- | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code                             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| » data                             | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                |
| »» symbol                          | string         | true     | none         | Unique code of a symbol; it will not change after renaming                                                                                                                                                                                                                                                                          |
| »» name                            | string         | true     | none         | Name of trading pairs, it will change after renaming                                                                                                                                                                                                                                                                                |
| »» baseCurrency                    | string         | true     | none         | Base currency, e.g. BTC.                                                                                                                                                                                                                                                                                                            |
| »» quoteCurrency                   | string         | true     | none         | Quote currency, e.g. USDT.                                                                                                                                                                                                                                                                                                          |
| »» feeCurrency                     | string         | true     | none         | The currency of charged fees.                                                                                                                                                                                                                                                                                                       |
| »» market                          | string         | true     | none         | The trading market.                                                                                                                                                                                                                                                                                                                 |
| »» baseMinSize                     | string         | true     | none         | The minimum order quantity required to place an order.                                                                                                                                                                                                                                                                              |
| »» quoteMinSize                    | string         | true     | none         | The minimum order funds required to place a market order.                                                                                                                                                                                                                                                                           |
| »» baseMaxSize                     | string         | true     | none         | The maximum order size required to place an order.                                                                                                                                                                                                                                                                                  |
| »» quoteMaxSize                    | string         | true     | none         | The maximum order funds required to place a market order.                                                                                                                                                                                                                                                                           |
| »» baseIncrement                   | string         | true     | none         | Quantity increment: The quantity for an order must be a positive integer multiple of this increment. Here, the size refers to the quantity of the base currency for the order. For example, for the ETH-USDT trading pair, if the baseIncrement is 0.0000001, the order quantity can be 1.0000001 but not 1.00000001.               |
| »» quoteIncrement                  | string         | true     | none         | Quote increment: The funds for a market order must be a positive integer multiple of this increment. The funds refer to the quote currency amount. For example, for the ETH-USDT trading pair, if the quoteIncrement is 0.000001, the amount of USDT for the order can be 3000.000001 but not 3000.0000001.                         |
| »» priceIncrement                  | string         | true     | none         | Price increment: The price of an order must be a positive integer multiple of this increment. For example, for the ETH-USDT trading pair, if the priceIncrement is 0.01, the order price can be 3000.01 but not 3000.001.<br><br>Specifies the min. order price as well as the price increment.This also applies to quote currency. |
| »» priceLimitRate                  | string         | true     | none         | Threshold for price protection                                                                                                                                                                                                                                                                                                      |
| »» minFunds                        | string         | true     | none         | The minimum trading amounts                                                                                                                                                                                                                                                                                                         |
| »» isMarginEnabled                 | boolean        | true     | none         | Available for margin or not.                                                                                                                                                                                                                                                                                                        |
| »» enableTrading                   | boolean        | true     | none         | Available for transaction or not.                                                                                                                                                                                                                                                                                                   |
| »» feeCategory                     | integer        | true     | none         | [Fee Type](https://www.kucoin.com/vip/privilege)                                                                                                                                                                                                                                                                                    |
| »» makerFeeCoefficient             | string         | true     | none         | The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee                                                                                                                                                    |
| »» takerFeeCoefficient             | string         | true     | none         | The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee                                                                                                                                                    |
| »» st                              | boolean        | true     | none         | Whether it is a [Special Treatment](https://www.kucoin.com/legal/special-treatment) symbol                                                                                                                                                                                                                                          |
| »» callauctionIsEnabled            | boolean        | true     | none         | The [call auction](https://www.kucoin.com/support/40999744334105) status returns true/false                                                                                                                                                                                                                                         |
| »» callauctionPriceFloor           | string         | false    | none         | The lowest price declared in the call auction                                                                                                                                                                                                                                                                                       |
| »» callauctionPriceCeiling         | string         | false    | none         | The highest bid price in the call auction                                                                                                                                                                                                                                                                                           |
| »» callauctionFirstStageStartTime  | integer(int64) | false    | none         | The first phase of the call auction starts at (Allow add orders, allow cancel orders)                                                                                                                                                                                                                                               |
| »» callauctionSecondStageStartTime | integer(int64) | false    | none         | The second phase of the call auction starts at (Allow add orders, don't allow cancel orders)                                                                                                                                                                                                                                        |
| »» callauctionThirdStageStartTime  | integer(int64) | false    | none         | The third phase of the call auction starts at (Don't allow add orders, don't allow cancel orders)                                                                                                                                                                                                                                   |
| »» tradingStartTime                | integer(int64) | false    | none         | Official opening time (end time of the third phase of call auction)                                                                                                                                                                                                                                                                 |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| feeCategory | 1     |
| feeCategory | 2     |
| feeCategory | 3     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Ticker

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/orderbook/level1?symbol=type,string", {
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

r = requests.get('/api/v1/market/orderbook/level1', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/orderbook/level1`

Request via this endpoint to get Level 1 Market Data. The returned value
includes the best bid price and size, the best ask price and size as well as the
last traded price and the last traded size.

<h3 id="get-ticker-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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
        "time": {
          "type": "integer",
          "description": "timestamp",
          "format": "int64"
        },
        "sequence": {
          "type": "string",
          "description": "Sequence"
        },
        "price": {
          "type": "string",
          "description": "Last traded price"
        },
        "size": {
          "type": "string",
          "description": "Last traded size"
        },
        "bestBid": {
          "type": "string",
          "description": "Best bid price"
        },
        "bestBidSize": {
          "type": "string",
          "description": "Best bid size"
        },
        "bestAsk": {
          "type": "string",
          "description": "Best ask price"
        },
        "bestAskSize": {
          "type": "string",
          "description": "Best ask size"
        }
      },
      "required": [
        "sequence",
        "price",
        "size",
        "bestBid",
        "bestBidSize",
        "bestAsk",
        "bestAskSize",
        "time"
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

| Name           | Type           | Required | Restrictions | Description       |
| -------------- | -------------- | -------- | ------------ | ----------------- |
| » code         | string         | true     | none         | none              |
| » data         | object         | true     | none         | none              |
| »» time        | integer(int64) | true     | none         | timestamp         |
| »» sequence    | string         | true     | none         | Sequence          |
| »» price       | string         | true     | none         | Last traded price |
| »» size        | string         | true     | none         | Last traded size  |
| »» bestBid     | string         | true     | none         | Best bid price    |
| »» bestBidSize | string         | true     | none         | Best bid size     |
| »» bestAsk     | string         | true     | none         | Best ask price    |
| »» bestAskSize | string         | true     | none         | Best ask size     |

<aside class="success">
This operation does not require authentication
</aside>

## Get All Tickers

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/allTickers", {
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

r = requests.get('/api/v1/market/allTickers', headers = headers)

print(r.json())

```

`GET /api/v1/market/allTickers`

Request market tickers for all the trading pairs in the market (including 24h
volume); takes a snapshot every 2 seconds. On the rare occasion that we change
the currency name, if you still want the changed symbol name, you can use the
symbolName field instead of the symbol field via “Get all tickers” endpoint.

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
        "time": {
          "type": "integer",
          "description": "timestamp",
          "format": "int64"
        },
        "ticker": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "Symbol"
              },
              "symbolName": {
                "type": "string",
                "description": "Name of trading pairs, it will change after renaming"
              },
              "buy": {
                "type": "string",
                "description": "Best bid price"
              },
              "bestBidSize": {
                "type": "string",
                "description": "Best bid size"
              },
              "sell": {
                "type": "string",
                "description": "Best ask price"
              },
              "bestAskSize": {
                "type": "string",
                "description": "Best ask size"
              },
              "changeRate": {
                "type": "string",
                "description": "24h change rate"
              },
              "changePrice": {
                "type": "string",
                "description": "24h change price"
              },
              "high": {
                "type": "string",
                "description": "Highest price in 24h"
              },
              "low": {
                "type": "string",
                "description": "Lowest price in 24h"
              },
              "vol": {
                "type": "string",
                "description": "24h volume, executed based on base currency"
              },
              "volValue": {
                "type": "string",
                "description": "24h traded amount"
              },
              "last": {
                "type": "string",
                "description": "Last traded price"
              },
              "averagePrice": {
                "type": "string",
                "description": "Average trading price in the last 24 hours"
              },
              "takerFeeRate": {
                "type": "string",
                "description": "Basic Taker Fee"
              },
              "makerFeeRate": {
                "type": "string",
                "description": "Basic Maker Fee"
              },
              "takerCoefficient": {
                "type": "string",
                "description": "The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee",
                "enum": ["1", "0"],
                "x-api-enum": [
                  {
                    "value": "1",
                    "name": "1",
                    "description": "The taker fee coefficient is 1"
                  },
                  {
                    "value": "0",
                    "name": "",
                    "description": "No fee"
                  }
                ]
              },
              "makerCoefficient": {
                "type": "string",
                "description": "The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee",
                "enum": ["1", "0"],
                "x-api-enum": [
                  {
                    "value": "1",
                    "name": "1",
                    "description": "The maker fee coefficient is 1"
                  },
                  {
                    "value": "0",
                    "name": "",
                    "description": "No fee"
                  }
                ]
              }
            },
            "required": [
              "symbol",
              "symbolName",
              "buy",
              "bestBidSize",
              "sell",
              "bestAskSize",
              "changeRate",
              "changePrice",
              "high",
              "low",
              "vol",
              "volValue",
              "last",
              "averagePrice",
              "takerFeeRate",
              "makerFeeRate",
              "takerCoefficient",
              "makerCoefficient"
            ]
          }
        }
      },
      "required": ["time", "ticker"]
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

| Name                 | Type           | Required | Restrictions | Description                                                                                                                                                                      |
| -------------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code               | string         | true     | none         | none                                                                                                                                                                             |
| » data               | object         | true     | none         | none                                                                                                                                                                             |
| »» time              | integer(int64) | true     | none         | timestamp                                                                                                                                                                        |
| »» ticker            | [object]       | true     | none         | none                                                                                                                                                                             |
| »»» symbol           | string         | true     | none         | Symbol                                                                                                                                                                           |
| »»» symbolName       | string         | true     | none         | Name of trading pairs, it will change after renaming                                                                                                                             |
| »»» buy              | string         | true     | none         | Best bid price                                                                                                                                                                   |
| »»» bestBidSize      | string         | true     | none         | Best bid size                                                                                                                                                                    |
| »»» sell             | string         | true     | none         | Best ask price                                                                                                                                                                   |
| »»» bestAskSize      | string         | true     | none         | Best ask size                                                                                                                                                                    |
| »»» changeRate       | string         | true     | none         | 24h change rate                                                                                                                                                                  |
| »»» changePrice      | string         | true     | none         | 24h change price                                                                                                                                                                 |
| »»» high             | string         | true     | none         | Highest price in 24h                                                                                                                                                             |
| »»» low              | string         | true     | none         | Lowest price in 24h                                                                                                                                                              |
| »»» vol              | string         | true     | none         | 24h volume, executed based on base currency                                                                                                                                      |
| »»» volValue         | string         | true     | none         | 24h traded amount                                                                                                                                                                |
| »»» last             | string         | true     | none         | Last traded price                                                                                                                                                                |
| »»» averagePrice     | string         | true     | none         | Average trading price in the last 24 hours                                                                                                                                       |
| »»» takerFeeRate     | string         | true     | none         | Basic Taker Fee                                                                                                                                                                  |
| »»» makerFeeRate     | string         | true     | none         | Basic Maker Fee                                                                                                                                                                  |
| »»» takerCoefficient | string         | true     | none         | The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee |
| »»» makerCoefficient | string         | true     | none         | The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee |

#### Enumerated Values

| Property         | Value |
| ---------------- | ----- |
| takerCoefficient | 1     |
| takerCoefficient | 0     |
| makerCoefficient | 1     |
| makerCoefficient | 0     |

<aside class="success">
This operation does not require authentication
</aside>

## Get Trade History

<a id="opId020"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/fills?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/hf/fills', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/fills`

This endpoint can be used to obtain a list of the latest Spot transaction
details. The returned data is sorted in descending order according to the latest
update time of the order.

<h3 id="get-trade-history-parameters">Parameters</h3>

| Name    | In    | Type           | Required | Description                                                                                                  |
| ------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| symbol  | query | string         | true     | symbol                                                                                                       |
| orderId | query | string         | false    | The unique order id generated by the trading system                                                          |
| side    | query | string         | false    | specify if the order is to 'buy' or 'sell'                                                                   |
| type    | query | string         | false    | specify if the order is an 'limit' order or 'market' order.                                                  |
| lastId  | query | integer(int64) | false    | The id of the last set of data from the previous batch of data. By default, the latest information is given. |
| limit   | query | integer        | false    | Default20，Max100                                                                                            |
| startAt | query | integer(int64) | false    | Start time (milisecond)                                                                                      |
| endAt   | query | integer(int64) | false    | End time (milisecond)                                                                                        |

#### Detailed descriptions

**orderId**: The unique order id generated by the trading system (If orderId is
specified，please ignore the other query parameters)

**lastId**: The id of the last set of data from the previous batch of data. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId，which can be used as a query
parameter to look up new data from the next page.

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
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "integer",
                "description": "Id of transaction detail",
                "format": "int64"
              },
              "symbol": {
                "type": "string",
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "tradeId": {
                "type": "integer",
                "description": "Trade Id, symbol latitude increment",
                "format": "int64"
              },
              "orderId": {
                "type": "string",
                "description": "The unique order id generated by the trading system"
              },
              "counterOrderId": {
                "type": "string",
                "description": "Counterparty order Id"
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
                "description": "Order price"
              },
              "size": {
                "type": "string",
                "description": "Order size"
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
                "description": "currency used to calculate trading fee"
              },
              "stop": {
                "type": "string",
                "description": "Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty"
              },
              "tradeType": {
                "type": "string",
                "description": "Trade type, redundancy param"
              },
              "taxRate": {
                "type": "string",
                "description": "Tax Rate, Users in some regions need query this field"
              },
              "tax": {
                "type": "string",
                "description": "Users in some regions need query this field"
              },
              "type": {
                "type": "string",
                "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
              "taxRate",
              "tax",
              "type",
              "createdAt"
            ]
          }
        },
        "lastId": {
          "type": "integer",
          "description": "The id of the last set of data from the previous batch of data. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId，which can be used as a query parameter to look up new data from the next page.",
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

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                                  |
| ------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» id             | integer(int64) | true     | none         | Id of transaction detail                                                                                                                                                                                                                                                                                                                                     |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                       |
| »»» tradeId        | integer(int64) | true     | none         | Trade Id, symbol latitude increment                                                                                                                                                                                                                                                                                                                          |
| »»» orderId        | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                          |
| »»» counterOrderId | string         | true     | none         | Counterparty order Id                                                                                                                                                                                                                                                                                                                                        |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                                  |
| »»» liquidity      | string         | true     | none         | Liquidity type: taker or maker                                                                                                                                                                                                                                                                                                                               |
| »»» forceTaker     | boolean        | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                                                                                                  |
| »»» size           | string         | true     | none         | Order size                                                                                                                                                                                                                                                                                                                                                   |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                                  |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                                 |
| »»» feeRate        | string         | true     | none         | Fee rate                                                                                                                                                                                                                                                                                                                                                     |
| »»» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                       |
| »»» stop           | string         | true     | none         | Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty                                                                                                                                                                                                                                            |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                                 |
| »»» taxRate        | string         | true     | none         | Tax Rate, Users in some regions need query this field                                                                                                                                                                                                                                                                                                        |
| »»» tax            | string         | true     | none         | Users in some regions need query this field                                                                                                                                                                                                                                                                                                                  |
| »»» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                  |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »» lastId          | integer(int64) | true     | none         | The id of the last set of data from the previous batch of data. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId，which can be used as a query parameter to look up new data from the next page. |

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

## Get Klines

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/market/candles?symbol=type,string&type=type,string,enum,1min%2C3min%2C5min%2C15min%2C30min%2C1hour%2C2hour%2C4hour%2C6hour%2C8hour%2C12hour%2C1day%2C1week%2C1month,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v1/market/candles', params={
  'symbol': {
  "type": "string"
},  'type': {
  "type": "string",
  "enum": [
    "1min",
    "3min",
    "5min",
    "15min",
    "30min",
    "1hour",
    "2hour",
    "4hour",
    "6hour",
    "8hour",
    "12hour",
    "1day",
    "1week",
    "1month"
  ],
  "x-api-enum": [
    {
      "value": "1min",
      "name": "1min",
      "description": "1min"
    },
    {
      "value": "3min",
      "name": "3min",
      "description": "3min"
    },
    {
      "value": "5min",
      "name": "5min",
      "description": "5min"
    },
    {
      "value": "15min",
      "name": "15min",
      "description": "15min"
    },
    {
      "value": "30min",
      "name": "30min",
      "description": "30min"
    },
    {
      "value": "1hour",
      "name": "1hour",
      "description": "1hour"
    },
    {
      "value": "2hour",
      "name": "2hour",
      "description": "2hour"
    },
    {
      "value": "4hour",
      "name": "4hour",
      "description": "4hour"
    },
    {
      "value": "6hour",
      "name": "6hour",
      "description": "6hour"
    },
    {
      "value": "8hour",
      "name": "8hour",
      "description": "8hour"
    },
    {
      "value": "12hour",
      "name": "12hour",
      "description": "12hour"
    },
    {
      "value": "1day",
      "name": "1day",
      "description": "1day"
    },
    {
      "value": "1week",
      "name": "1week",
      "description": "1week"
    },
    {
      "value": "1month",
      "name": "1month",
      "description": "1month"
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/candles`

Get the Kline of the symbol. Data are returned in grouped buckets based on
requested type. For each query, the system would return at most 1500 pieces of
data. To obtain more data, please page the data by time.

<h3 id="get-klines-parameters">Parameters</h3>

| Name    | In    | Type           | Required | Description                                                                                                                  |
| ------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------- |
| symbol  | query | string         | true     | symbol                                                                                                                       |
| type    | query | string         | true     | Type of candlestick patterns: 1min, 3min, 5min, 15min, 30min, 1hour, 2hour, 4hour, 6hour, 8hour, 12hour, 1day, 1week, 1month |
| startAt | query | integer(int64) | false    | Start time (second), default is 0                                                                                            |
| endAt   | query | integer(int64) | false    | End time (second), default is 0                                                                                              |

#### Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| type      | 1min   |
| type      | 3min   |
| type      | 5min   |
| type      | 15min  |
| type      | 30min  |
| type      | 1hour  |
| type      | 2hour  |
| type      | 4hour  |
| type      | 6hour  |
| type      | 8hour  |
| type      | 12hour |
| type      | 1day   |
| type      | 1week  |
| type      | 1month |

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
          "type": "string",
          "description": "Start time of the candle cycle, opening price, closing price, highest price, Lowest price, Transaction amount, Transaction volume"
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

## Get Part OrderBook

<a id="opId010"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/orderbook/level2_{size}?symbol=type,string", {
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

r = requests.get('/api/v1/market/orderbook/level2_{size}', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/orderbook/level2_{size}`

Query for part orderbook depth data. (aggregated by price) You are recommended
to request via this endpoint as the system reponse would be faster and cosume
less traffic.

<h3 id="get-part-orderbook-parameters">Parameters</h3>

| Name   | In    | Type    | Required | Description                                  |
| ------ | ----- | ------- | -------- | -------------------------------------------- |
| symbol | query | string  | true     | symbol                                       |
| size   | path  | integer | true     | Get the depth layer, optional value: 20, 100 |

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
        "time": {
          "type": "integer",
          "description": "Timestamp(millisecond)",
          "format": "int64"
        },
        "sequence": {
          "type": "string",
          "description": "Sequence number"
        },
        "bids": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "string",
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
              "type": "string",
              "description": "Price, Size"
            }
          },
          "description": "asks, from low to high"
        }
      },
      "required": ["sequence", "bids", "asks", "time"]
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

| Name        | Type           | Required | Restrictions | Description            |
| ----------- | -------------- | -------- | ------------ | ---------------------- |
| » code      | string         | true     | none         | none                   |
| » data      | object         | true     | none         | none                   |
| »» time     | integer(int64) | true     | none         | Timestamp(millisecond) |
| »» sequence | string         | true     | none         | Sequence number        |
| »» bids     | [array]        | true     | none         | bids, from high to low |
| »» asks     | [array]        | true     | none         | asks, from low to high |

<aside class="success">
This operation does not require authentication
</aside>

## Get Full OrderBook

<a id="opId011"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/market/orderbook/level2?symbol=type,string", {
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

r = requests.get('/api/v3/market/orderbook/level2', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/market/orderbook/level2`

Query for Full orderbook depth data. (aggregated by price) It is generally used
by professional traders because it uses more server resources and traffic, and
we have strict access rate limit control. To maintain up-to-date Order Book,
please use Websocket incremental feed after retrieving the OrderBook.

<h3 id="get-full-orderbook-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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
        "time": {
          "type": "integer",
          "description": "Timestamp(millisecond)",
          "format": "int64"
        },
        "sequence": {
          "type": "string",
          "description": "Sequence number"
        },
        "bids": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "string",
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
              "type": "string",
              "description": "Price, Size"
            }
          },
          "description": "asks, from low to high"
        }
      },
      "required": ["time", "sequence", "bids", "asks"]
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

| Name        | Type           | Required | Restrictions | Description            |
| ----------- | -------------- | -------- | ------------ | ---------------------- |
| » code      | string         | true     | none         | none                   |
| » data      | object         | true     | none         | none                   |
| »» time     | integer(int64) | true     | none         | Timestamp(millisecond) |
| »» sequence | string         | true     | none         | Sequence number        |
| »» bids     | [array]        | true     | none         | bids, from high to low |
| »» asks     | [array]        | true     | none         | asks, from low to high |

<aside class="success">
This operation does not require authentication
</aside>

## Get Call Auction Part OrderBook

<a id="opId012"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/orderbook/callauction/level2_{size}?symbol=type,string", {
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

r = requests.get('/api/v1/market/orderbook/callauction/level2_{size}', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/orderbook/callauction/level2_{size}`

Query for call auction part orderbook depth data. (aggregated by price). It is
recommended that you request via this endpoint, as the system response will be
faster and consume less traffic.

<h3 id="get-call-auction-part-orderbook-parameters">Parameters</h3>

| Name   | In    | Type    | Required | Description                                  |
| ------ | ----- | ------- | -------- | -------------------------------------------- |
| symbol | query | string  | true     | symbol                                       |
| size   | path  | integer | true     | Get the depth layer, optional value: 20, 100 |

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
        "time": {
          "type": "integer",
          "description": "Timestamp (milliseconds)",
          "format": "int64"
        },
        "sequence": {
          "type": "string",
          "description": "Sequence number"
        },
        "bids": {
          "type": "array",
          "items": {
            "type": "array",
            "items": {
              "type": "string",
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
              "type": "string",
              "description": "Price, Size"
            }
          },
          "description": "asks, from low to high"
        }
      },
      "required": ["sequence", "bids", "asks", "time"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-call-auction-part-orderbook-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-call-auction-part-orderbook-responseschema">Response Schema</h3>

Status Code **200**

| Name        | Type           | Required | Restrictions | Description              |
| ----------- | -------------- | -------- | ------------ | ------------------------ |
| » code      | string         | true     | none         | none                     |
| » data      | object         | true     | none         | none                     |
| »» time     | integer(int64) | true     | none         | Timestamp (milliseconds) |
| »» sequence | string         | true     | none         | Sequence number          |
| »» bids     | [array]        | true     | none         | bids, from high to low   |
| »» asks     | [array]        | true     | none         | asks, from low to high   |

<aside class="success">
This operation does not require authentication
</aside>

## Get Call Auction Info

<a id="opId013"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/callauctionData?symbol=type,string", {
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

r = requests.get('/api/v1/market/callauctionData', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/callauctionData`

Get call auction data. This interface will return the following information for
the specified symbol during the call auction phase: estimated transaction price,
estimated transaction quantity, bid price range, and ask price range.

<h3 id="get-call-auction-info-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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
          "description": "Symbol",
          "example": ["BTC-USDT"]
        },
        "estimatedPrice": {
          "type": "string",
          "description": "Estimated price"
        },
        "estimatedSize": {
          "type": "string",
          "description": "Estimated size"
        },
        "sellOrderRangeLowPrice": {
          "type": "string",
          "description": "Sell ​​order minimum price"
        },
        "sellOrderRangeHighPrice": {
          "type": "string",
          "description": "Sell ​​order maximum price"
        },
        "buyOrderRangeLowPrice": {
          "type": "string",
          "description": "Buy order minimum price"
        },
        "buyOrderRangeHighPrice": {
          "type": "string",
          "description": "Buy ​​order maximum price"
        },
        "time": {
          "type": "integer",
          "description": "Timestamp (ms)",
          "format": "int64"
        }
      },
      "required": [
        "sellOrderRangeLowPrice",
        "sellOrderRangeHighPrice",
        "buyOrderRangeLowPrice",
        "buyOrderRangeHighPrice",
        "time",
        "symbol",
        "estimatedPrice",
        "estimatedSize"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-call-auction-info-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-call-auction-info-responseschema">Response Schema</h3>

Status Code **200**

| Name                       | Type           | Required | Restrictions | Description                |
| -------------------------- | -------------- | -------- | ------------ | -------------------------- |
| » code                     | string         | true     | none         | none                       |
| » data                     | object         | true     | none         | none                       |
| »» symbol                  | string         | true     | none         | Symbol                     |
| »» estimatedPrice          | string         | true     | none         | Estimated price            |
| »» estimatedSize           | string         | true     | none         | Estimated size             |
| »» sellOrderRangeLowPrice  | string         | true     | none         | Sell ​​order minimum price |
| »» sellOrderRangeHighPrice | string         | true     | none         | Sell ​​order maximum price |
| »» buyOrderRangeLowPrice   | string         | true     | none         | Buy order minimum price    |
| »» buyOrderRangeHighPrice  | string         | true     | none         | Buy ​​order maximum price  |
| »» time                    | integer(int64) | true     | none         | Timestamp (ms)             |

<aside class="success">
This operation does not require authentication
</aside>

## Get Fiat Price

<a id="opId014"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/prices", {
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

r = requests.get('/api/v1/prices', headers = headers)

print(r.json())

```

`GET /api/v1/prices`

Request the fiat price of the currencies for the available trading pairs via
this endpoint.

<h3 id="get-fiat-price-parameters">Parameters</h3>

| Name       | In    | Type   | Required | Description                                                                                                                         |
| ---------- | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| base       | query | string | false    | Ticker symbol of a base currency, e.g. USD, EUR. Default is USD                                                                     |
| currencies | query | string | false    | Comma-separated cryptocurrencies to be converted into fiat, e.g.: BTC,ETH, etc. Default to return the fiat price of all currencies. |

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
        "AGLD": {
          "type": "string"
        },
        "DFI": {
          "type": "string"
        },
        "PYTHUP": {
          "type": "string"
        },
        "ISLM": {
          "type": "string"
        },
        "NEAR": {
          "type": "string"
        },
        "AIOZ": {
          "type": "string"
        },
        "AUDIO": {
          "type": "string"
        },
        "BBL": {
          "type": "string"
        },
        "WLD": {
          "type": "string"
        },
        "HNT": {
          "type": "string"
        },
        "ETHFI": {
          "type": "string"
        },
        "DMAIL": {
          "type": "string"
        },
        "OPUP": {
          "type": "string"
        },
        "VET3S": {
          "type": "string"
        },
        "MANA3S": {
          "type": "string"
        },
        "TIDAL": {
          "type": "string"
        },
        "HALO": {
          "type": "string"
        },
        "OPUL": {
          "type": "string"
        },
        "MANA3L": {
          "type": "string"
        },
        "DGB": {
          "type": "string"
        },
        "AA": {
          "type": "string"
        },
        "BCH": {
          "type": "string"
        },
        "GMEE": {
          "type": "string"
        },
        "JST": {
          "type": "string"
        },
        "PBUX": {
          "type": "string"
        },
        "AR": {
          "type": "string"
        },
        "SEI": {
          "type": "string"
        },
        "PSTAKE": {
          "type": "string"
        },
        "LMWR": {
          "type": "string"
        },
        "UNFIDOWN": {
          "type": "string"
        },
        "BB": {
          "type": "string"
        },
        "JTO": {
          "type": "string"
        },
        "WEMIX": {
          "type": "string"
        },
        "G": {
          "type": "string"
        },
        "MARSH": {
          "type": "string"
        },
        "BN": {
          "type": "string"
        },
        "FLIP": {
          "type": "string"
        },
        "FLR": {
          "type": "string"
        },
        "BIGTIME": {
          "type": "string"
        },
        "FLY": {
          "type": "string"
        },
        "T": {
          "type": "string"
        },
        "W": {
          "type": "string"
        },
        "BDX": {
          "type": "string"
        },
        "BABYDOGE": {
          "type": "string"
        },
        "SFP": {
          "type": "string"
        },
        "DIA": {
          "type": "string"
        },
        "ISME": {
          "type": "string"
        },
        "LYM": {
          "type": "string"
        },
        "VET3L": {
          "type": "string"
        },
        "JUP": {
          "type": "string"
        },
        "LYX": {
          "type": "string"
        },
        "AIEPK": {
          "type": "string"
        },
        "SILLY": {
          "type": "string"
        },
        "SCPT": {
          "type": "string"
        },
        "WOO": {
          "type": "string"
        },
        "BLUR": {
          "type": "string"
        },
        "STRK": {
          "type": "string"
        },
        "BFC": {
          "type": "string"
        },
        "DC": {
          "type": "string"
        },
        "KARATE": {
          "type": "string"
        },
        "SUSHI3L": {
          "type": "string"
        },
        "NETVR": {
          "type": "string"
        },
        "WAVES": {
          "type": "string"
        },
        "LITH": {
          "type": "string"
        },
        "HAPI": {
          "type": "string"
        },
        "SUSHI3S": {
          "type": "string"
        },
        "CEEK": {
          "type": "string"
        },
        "FLOKI": {
          "type": "string"
        },
        "SHR": {
          "type": "string"
        },
        "SAND": {
          "type": "string"
        },
        "TURT": {
          "type": "string"
        },
        "UMA": {
          "type": "string"
        },
        "BEPRO": {
          "type": "string"
        },
        "SCRT": {
          "type": "string"
        },
        "TUSD": {
          "type": "string"
        },
        "COOKIE": {
          "type": "string"
        },
        "LRDS": {
          "type": "string"
        },
        "SIN": {
          "type": "string"
        },
        "OAS": {
          "type": "string"
        },
        "ROOT": {
          "type": "string"
        },
        "ADA3L": {
          "type": "string"
        },
        "TIAUP": {
          "type": "string"
        },
        "HTR": {
          "type": "string"
        },
        "UNB": {
          "type": "string"
        },
        "UNA": {
          "type": "string"
        },
        "HARD": {
          "type": "string"
        },
        "G3": {
          "type": "string"
        },
        "ADA3S": {
          "type": "string"
        },
        "MYRO": {
          "type": "string"
        },
        "HTX": {
          "type": "string"
        },
        "FT": {
          "type": "string"
        },
        "BTCDOWN": {
          "type": "string"
        },
        "UNI": {
          "type": "string"
        },
        "FX": {
          "type": "string"
        },
        "OBI": {
          "type": "string"
        },
        "UNO": {
          "type": "string"
        },
        "WRX": {
          "type": "string"
        },
        "TIADOWN": {
          "type": "string"
        },
        "ETHDOWN": {
          "type": "string"
        },
        "WELL": {
          "type": "string"
        },
        "SWFTC": {
          "type": "string"
        },
        "SKL": {
          "type": "string"
        },
        "UOS": {
          "type": "string"
        },
        "AIPAD": {
          "type": "string"
        },
        "BRETT": {
          "type": "string"
        },
        "SKY": {
          "type": "string"
        },
        "FRM": {
          "type": "string"
        },
        "VISION": {
          "type": "string"
        },
        "LENDS": {
          "type": "string"
        },
        "SLF": {
          "type": "string"
        },
        "BULL": {
          "type": "string"
        },
        "FLOW": {
          "type": "string"
        },
        "ODDZ": {
          "type": "string"
        },
        "SLN": {
          "type": "string"
        },
        "UPO": {
          "type": "string"
        },
        "SLP": {
          "type": "string"
        },
        "ID": {
          "type": "string"
        },
        "SLIM": {
          "type": "string"
        },
        "SPOT": {
          "type": "string"
        },
        "DOP": {
          "type": "string"
        },
        "ISSP": {
          "type": "string"
        },
        "UQC": {
          "type": "string"
        },
        "IO": {
          "type": "string"
        },
        "DOT": {
          "type": "string"
        },
        "1INCH": {
          "type": "string"
        },
        "SMH": {
          "type": "string"
        },
        "MAK": {
          "type": "string"
        },
        "TOKO": {
          "type": "string"
        },
        "TURBO": {
          "type": "string"
        },
        "UNFI": {
          "type": "string"
        },
        "MAN": {
          "type": "string"
        },
        "EVER": {
          "type": "string"
        },
        "FTM": {
          "type": "string"
        },
        "SHRAP": {
          "type": "string"
        },
        "MAV": {
          "type": "string"
        },
        "MAX": {
          "type": "string"
        },
        "DPR": {
          "type": "string"
        },
        "FTT": {
          "type": "string"
        },
        "ARKM": {
          "type": "string"
        },
        "ATOM": {
          "type": "string"
        },
        "PENDLE": {
          "type": "string"
        },
        "QUICK": {
          "type": "string"
        },
        "BLZ": {
          "type": "string"
        },
        "BOBA": {
          "type": "string"
        },
        "MBL": {
          "type": "string"
        },
        "OFN": {
          "type": "string"
        },
        "UNIO": {
          "type": "string"
        },
        "SNS": {
          "type": "string"
        },
        "SNX": {
          "type": "string"
        },
        "NXRA": {
          "type": "string"
        },
        "TAIKO": {
          "type": "string"
        },
        "AVAX3L": {
          "type": "string"
        },
        "L3": {
          "type": "string"
        },
        "API3": {
          "type": "string"
        },
        "XRP3S": {
          "type": "string"
        },
        "QKC": {
          "type": "string"
        },
        "AVAX3S": {
          "type": "string"
        },
        "ROSE": {
          "type": "string"
        },
        "SATS": {
          "type": "string"
        },
        "BMX": {
          "type": "string"
        },
        "PORTAL": {
          "type": "string"
        },
        "TOMI": {
          "type": "string"
        },
        "XRP3L": {
          "type": "string"
        },
        "SOL": {
          "type": "string"
        },
        "SON": {
          "type": "string"
        },
        "BNC": {
          "type": "string"
        },
        "SOCIAL": {
          "type": "string"
        },
        "CGPT": {
          "type": "string"
        },
        "CELR": {
          "type": "string"
        },
        "BNB": {
          "type": "string"
        },
        "OGN": {
          "type": "string"
        },
        "CELO": {
          "type": "string"
        },
        "AUCTION": {
          "type": "string"
        },
        "MANTA": {
          "type": "string"
        },
        "LAYER": {
          "type": "string"
        },
        "AERO": {
          "type": "string"
        },
        "CETUS": {
          "type": "string"
        },
        "LL": {
          "type": "string"
        },
        "SPA": {
          "type": "string"
        },
        "PYTHDOWN": {
          "type": "string"
        },
        "NEIROCTO": {
          "type": "string"
        },
        "UTK": {
          "type": "string"
        },
        "GMRX": {
          "type": "string"
        },
        "BOB": {
          "type": "string"
        },
        "HOTCROSS": {
          "type": "string"
        },
        "AERGO": {
          "type": "string"
        },
        "MOCA": {
          "type": "string"
        },
        "SQD": {
          "type": "string"
        },
        "MV": {
          "type": "string"
        },
        "BNB3L": {
          "type": "string"
        },
        "BNB3S": {
          "type": "string"
        },
        "GALAX3L": {
          "type": "string"
        },
        "KAI": {
          "type": "string"
        },
        "SQR": {
          "type": "string"
        },
        "GALAX3S": {
          "type": "string"
        },
        "EGLD": {
          "type": "string"
        },
        "ZBCN": {
          "type": "string"
        },
        "KAS": {
          "type": "string"
        },
        "MEW": {
          "type": "string"
        },
        "PUNDIX": {
          "type": "string"
        },
        "LOOKS": {
          "type": "string"
        },
        "FXS": {
          "type": "string"
        },
        "BOSON": {
          "type": "string"
        },
        "BRISE": {
          "type": "string"
        },
        "AEVO": {
          "type": "string"
        },
        "FLUX": {
          "type": "string"
        },
        "PRCL": {
          "type": "string"
        },
        "UNFIUP": {
          "type": "string"
        },
        "SEIDOWN": {
          "type": "string"
        },
        "DOAI": {
          "type": "string"
        },
        "QNT": {
          "type": "string"
        },
        "REDO": {
          "type": "string"
        },
        "STRIKE": {
          "type": "string"
        },
        "ETHW": {
          "type": "string"
        },
        "OM": {
          "type": "string"
        },
        "OP": {
          "type": "string"
        },
        "WHALE": {
          "type": "string"
        },
        "1CAT": {
          "type": "string"
        },
        "NEON": {
          "type": "string"
        },
        "GTAI": {
          "type": "string"
        },
        "SSV": {
          "type": "string"
        },
        "ETH2": {
          "type": "string"
        },
        "KCS": {
          "type": "string"
        },
        "ARPA": {
          "type": "string"
        },
        "ARTFI": {
          "type": "string"
        },
        "BRL": {
          "type": "string"
        },
        "ALEX": {
          "type": "string"
        },
        "STG": {
          "type": "string"
        },
        "SHIB": {
          "type": "string"
        },
        "IOTX": {
          "type": "string"
        },
        "OLE": {
          "type": "string"
        },
        "KDA": {
          "type": "string"
        },
        "CERE": {
          "type": "string"
        },
        "DOCK": {
          "type": "string"
        },
        "STX": {
          "type": "string"
        },
        "OLT": {
          "type": "string"
        },
        "QI": {
          "type": "string"
        },
        "SDAO": {
          "type": "string"
        },
        "BLAST": {
          "type": "string"
        },
        "LINK3S": {
          "type": "string"
        },
        "IOST": {
          "type": "string"
        },
        "SUI": {
          "type": "string"
        },
        "CAKE": {
          "type": "string"
        },
        "BSW": {
          "type": "string"
        },
        "OMG": {
          "type": "string"
        },
        "VOLT": {
          "type": "string"
        },
        "LINK3L": {
          "type": "string"
        },
        "GEEQ": {
          "type": "string"
        },
        "PYUSD": {
          "type": "string"
        },
        "SUN": {
          "type": "string"
        },
        "TOWER": {
          "type": "string"
        },
        "BTC": {
          "type": "string"
        },
        "IOTA": {
          "type": "string"
        },
        "REEF": {
          "type": "string"
        },
        "TRIAS": {
          "type": "string"
        },
        "KEY": {
          "type": "string"
        },
        "ETH3L": {
          "type": "string"
        },
        "BTT": {
          "type": "string"
        },
        "ONE": {
          "type": "string"
        },
        "RENDER": {
          "type": "string"
        },
        "ETH3S": {
          "type": "string"
        },
        "ANKR": {
          "type": "string"
        },
        "ALGO": {
          "type": "string"
        },
        "SYLO": {
          "type": "string"
        },
        "ZCX": {
          "type": "string"
        },
        "SD": {
          "type": "string"
        },
        "ONT": {
          "type": "string"
        },
        "MJT": {
          "type": "string"
        },
        "DYM": {
          "type": "string"
        },
        "DYP": {
          "type": "string"
        },
        "BAKEUP": {
          "type": "string"
        },
        "OOE": {
          "type": "string"
        },
        "ZELIX": {
          "type": "string"
        },
        "DOGE3L": {
          "type": "string"
        },
        "ARTY": {
          "type": "string"
        },
        "QORPO": {
          "type": "string"
        },
        "ICE": {
          "type": "string"
        },
        "NOTAI": {
          "type": "string"
        },
        "DOGE3S": {
          "type": "string"
        },
        "NAKA": {
          "type": "string"
        },
        "GALAX": {
          "type": "string"
        },
        "MKR": {
          "type": "string"
        },
        "DODO": {
          "type": "string"
        },
        "ICP": {
          "type": "string"
        },
        "ZEC": {
          "type": "string"
        },
        "ZEE": {
          "type": "string"
        },
        "ICX": {
          "type": "string"
        },
        "KMNO": {
          "type": "string"
        },
        "TT": {
          "type": "string"
        },
        "DOT3L": {
          "type": "string"
        },
        "XAI": {
          "type": "string"
        },
        "ZEN": {
          "type": "string"
        },
        "DOGE": {
          "type": "string"
        },
        "ALPHA": {
          "type": "string"
        },
        "DUSK": {
          "type": "string"
        },
        "DOT3S": {
          "type": "string"
        },
        "SXP": {
          "type": "string"
        },
        "HBAR": {
          "type": "string"
        },
        "SYNT": {
          "type": "string"
        },
        "ZEX": {
          "type": "string"
        },
        "BONDLY": {
          "type": "string"
        },
        "MLK": {
          "type": "string"
        },
        "KICKS": {
          "type": "string"
        },
        "PEPE": {
          "type": "string"
        },
        "OUSD": {
          "type": "string"
        },
        "LUNCDOWN": {
          "type": "string"
        },
        "DOGS": {
          "type": "string"
        },
        "REV3L": {
          "type": "string"
        },
        "CTSI": {
          "type": "string"
        },
        "C98": {
          "type": "string"
        },
        "OSMO": {
          "type": "string"
        },
        "NTRN": {
          "type": "string"
        },
        "CFX2S": {
          "type": "string"
        },
        "SYN": {
          "type": "string"
        },
        "VIDT": {
          "type": "string"
        },
        "SYS": {
          "type": "string"
        },
        "GAS": {
          "type": "string"
        },
        "BOME": {
          "type": "string"
        },
        "COMBO": {
          "type": "string"
        },
        "XCH": {
          "type": "string"
        },
        "VR": {
          "type": "string"
        },
        "CFX2L": {
          "type": "string"
        },
        "VSYS": {
          "type": "string"
        },
        "PANDORA": {
          "type": "string"
        },
        "THETA": {
          "type": "string"
        },
        "XCN": {
          "type": "string"
        },
        "NEXG": {
          "type": "string"
        },
        "MELOS": {
          "type": "string"
        },
        "XCV": {
          "type": "string"
        },
        "ORN": {
          "type": "string"
        },
        "WLKN": {
          "type": "string"
        },
        "AAVE": {
          "type": "string"
        },
        "MNT": {
          "type": "string"
        },
        "BONK": {
          "type": "string"
        },
        "PERP": {
          "type": "string"
        },
        "XDC": {
          "type": "string"
        },
        "MNW": {
          "type": "string"
        },
        "XDB": {
          "type": "string"
        },
        "BOND": {
          "type": "string"
        },
        "SUIA": {
          "type": "string"
        },
        "MOG": {
          "type": "string"
        },
        "SUTER": {
          "type": "string"
        },
        "TIME": {
          "type": "string"
        },
        "RACA": {
          "type": "string"
        },
        "BICO": {
          "type": "string"
        },
        "MON": {
          "type": "string"
        },
        "SWEAT": {
          "type": "string"
        },
        "MOXIE": {
          "type": "string"
        },
        "BABYBNB": {
          "type": "string"
        },
        "IGU": {
          "type": "string"
        },
        "HMSTR": {
          "type": "string"
        },
        "XEC": {
          "type": "string"
        },
        "MONI": {
          "type": "string"
        },
        "XR": {
          "type": "string"
        },
        "PEOPLE": {
          "type": "string"
        },
        "PUMLX": {
          "type": "string"
        },
        "ZIL": {
          "type": "string"
        },
        "WLDDOWN": {
          "type": "string"
        },
        "VAI": {
          "type": "string"
        },
        "XEN": {
          "type": "string"
        },
        "MPC": {
          "type": "string"
        },
        "XEM": {
          "type": "string"
        },
        "JASMY3S": {
          "type": "string"
        },
        "OTK": {
          "type": "string"
        },
        "TRAC": {
          "type": "string"
        },
        "DFYN": {
          "type": "string"
        },
        "BIDP": {
          "type": "string"
        },
        "JASMY3L": {
          "type": "string"
        },
        "INJDOWN": {
          "type": "string"
        },
        "KLV": {
          "type": "string"
        },
        "WAXL": {
          "type": "string"
        },
        "TRBDOWN": {
          "type": "string"
        },
        "BCH3L": {
          "type": "string"
        },
        "GMT3S": {
          "type": "string"
        },
        "KMD": {
          "type": "string"
        },
        "BCH3S": {
          "type": "string"
        },
        "ECOX": {
          "type": "string"
        },
        "AAVE3S": {
          "type": "string"
        },
        "GMT3L": {
          "type": "string"
        },
        "EPIK": {
          "type": "string"
        },
        "SUIP": {
          "type": "string"
        },
        "AAVE3L": {
          "type": "string"
        },
        "ZK": {
          "type": "string"
        },
        "ZKF": {
          "type": "string"
        },
        "OMNIA": {
          "type": "string"
        },
        "ZKJ": {
          "type": "string"
        },
        "ZKL": {
          "type": "string"
        },
        "GAFI": {
          "type": "string"
        },
        "CARV": {
          "type": "string"
        },
        "KNC": {
          "type": "string"
        },
        "CATS": {
          "type": "string"
        },
        "PROM": {
          "type": "string"
        },
        "ALEPH": {
          "type": "string"
        },
        "PONKE": {
          "type": "string"
        },
        "OVR": {
          "type": "string"
        },
        "CATI": {
          "type": "string"
        },
        "ORDER": {
          "type": "string"
        },
        "GFT": {
          "type": "string"
        },
        "BIFI": {
          "type": "string"
        },
        "GGC": {
          "type": "string"
        },
        "GGG": {
          "type": "string"
        },
        "DAPPX": {
          "type": "string"
        },
        "SUKU": {
          "type": "string"
        },
        "ULTI": {
          "type": "string"
        },
        "CREDI": {
          "type": "string"
        },
        "ERTHA": {
          "type": "string"
        },
        "FURY": {
          "type": "string"
        },
        "KARRAT": {
          "type": "string"
        },
        "MOBILE": {
          "type": "string"
        },
        "SIDUS": {
          "type": "string"
        },
        "NAVI": {
          "type": "string"
        },
        "TAO": {
          "type": "string"
        },
        "USDJ": {
          "type": "string"
        },
        "MTL": {
          "type": "string"
        },
        "VET": {
          "type": "string"
        },
        "FITFI": {
          "type": "string"
        },
        "USDT": {
          "type": "string"
        },
        "OXT": {
          "type": "string"
        },
        "CANDY": {
          "type": "string"
        },
        "USDP": {
          "type": "string"
        },
        "MTS": {
          "type": "string"
        },
        "TADA": {
          "type": "string"
        },
        "MTV": {
          "type": "string"
        },
        "NAVX": {
          "type": "string"
        },
        "ILV": {
          "type": "string"
        },
        "VINU": {
          "type": "string"
        },
        "GHX": {
          "type": "string"
        },
        "EDU": {
          "type": "string"
        },
        "HYVE": {
          "type": "string"
        },
        "BTC3L": {
          "type": "string"
        },
        "ANYONE": {
          "type": "string"
        },
        "BEAT": {
          "type": "string"
        },
        "KING": {
          "type": "string"
        },
        "CREAM": {
          "type": "string"
        },
        "CAS": {
          "type": "string"
        },
        "IMX": {
          "type": "string"
        },
        "CAT": {
          "type": "string"
        },
        "BTC3S": {
          "type": "string"
        },
        "USDE": {
          "type": "string"
        },
        "USDD": {
          "type": "string"
        },
        "CWAR": {
          "type": "string"
        },
        "USDC": {
          "type": "string"
        },
        "KRL": {
          "type": "string"
        },
        "INJ": {
          "type": "string"
        },
        "GAME": {
          "type": "string"
        },
        "TRIBL": {
          "type": "string"
        },
        "XLM": {
          "type": "string"
        },
        "TRBUP": {
          "type": "string"
        },
        "VRADOWN": {
          "type": "string"
        },
        "SUPER": {
          "type": "string"
        },
        "EIGEN": {
          "type": "string"
        },
        "IOI": {
          "type": "string"
        },
        "KSM": {
          "type": "string"
        },
        "CCD": {
          "type": "string"
        },
        "EGO": {
          "type": "string"
        },
        "EGP": {
          "type": "string"
        },
        "MXC": {
          "type": "string"
        },
        "TEL": {
          "type": "string"
        },
        "MOVR": {
          "type": "string"
        },
        "XMR": {
          "type": "string"
        },
        "MXM": {
          "type": "string"
        },
        "OORT": {
          "type": "string"
        },
        "GLM": {
          "type": "string"
        },
        "RAY": {
          "type": "string"
        },
        "XTAG": {
          "type": "string"
        },
        "GLQ": {
          "type": "string"
        },
        "CWEB": {
          "type": "string"
        },
        "REVU": {
          "type": "string"
        },
        "REVV": {
          "type": "string"
        },
        "ZRO": {
          "type": "string"
        },
        "XNL": {
          "type": "string"
        },
        "XNO": {
          "type": "string"
        },
        "SAROS": {
          "type": "string"
        },
        "KACE": {
          "type": "string"
        },
        "ZRX": {
          "type": "string"
        },
        "WLTH": {
          "type": "string"
        },
        "ATOM3L": {
          "type": "string"
        },
        "GMM": {
          "type": "string"
        },
        "BEER": {
          "type": "string"
        },
        "GMT": {
          "type": "string"
        },
        "HEART": {
          "type": "string"
        },
        "GMX": {
          "type": "string"
        },
        "ABBC": {
          "type": "string"
        },
        "OMNI": {
          "type": "string"
        },
        "ATOM3S": {
          "type": "string"
        },
        "IRL": {
          "type": "string"
        },
        "CFG": {
          "type": "string"
        },
        "WSDM": {
          "type": "string"
        },
        "GNS": {
          "type": "string"
        },
        "VANRY": {
          "type": "string"
        },
        "CFX": {
          "type": "string"
        },
        "GRAIL": {
          "type": "string"
        },
        "BEFI": {
          "type": "string"
        },
        "VELO": {
          "type": "string"
        },
        "XPR": {
          "type": "string"
        },
        "DOVI": {
          "type": "string"
        },
        "ACE": {
          "type": "string"
        },
        "ACH": {
          "type": "string"
        },
        "ISP": {
          "type": "string"
        },
        "XCAD": {
          "type": "string"
        },
        "MINA": {
          "type": "string"
        },
        "TIA": {
          "type": "string"
        },
        "DRIFT": {
          "type": "string"
        },
        "ACQ": {
          "type": "string"
        },
        "ACS": {
          "type": "string"
        },
        "MIND": {
          "type": "string"
        },
        "STORE": {
          "type": "string"
        },
        "REN": {
          "type": "string"
        },
        "ELA": {
          "type": "string"
        },
        "DREAMS": {
          "type": "string"
        },
        "ADA": {
          "type": "string"
        },
        "ELF": {
          "type": "string"
        },
        "REQ": {
          "type": "string"
        },
        "STORJ": {
          "type": "string"
        },
        "LADYS": {
          "type": "string"
        },
        "PAXG": {
          "type": "string"
        },
        "REZ": {
          "type": "string"
        },
        "XRD": {
          "type": "string"
        },
        "CHO": {
          "type": "string"
        },
        "CHR": {
          "type": "string"
        },
        "ADS": {
          "type": "string"
        },
        "CHZ": {
          "type": "string"
        },
        "ADX": {
          "type": "string"
        },
        "XRP": {
          "type": "string"
        },
        "JASMY": {
          "type": "string"
        },
        "KAGI": {
          "type": "string"
        },
        "FIDA": {
          "type": "string"
        },
        "PBR": {
          "type": "string"
        },
        "AEG": {
          "type": "string"
        },
        "H2O": {
          "type": "string"
        },
        "CHMB": {
          "type": "string"
        },
        "SAND3L": {
          "type": "string"
        },
        "PBX": {
          "type": "string"
        },
        "SOLVE": {
          "type": "string"
        },
        "DECHAT": {
          "type": "string"
        },
        "GARI": {
          "type": "string"
        },
        "SHIB2L": {
          "type": "string"
        },
        "SHIB2S": {
          "type": "string"
        },
        "ENA": {
          "type": "string"
        },
        "VEMP": {
          "type": "string"
        },
        "ENJ": {
          "type": "string"
        },
        "AFG": {
          "type": "string"
        },
        "RATS": {
          "type": "string"
        },
        "GRT": {
          "type": "string"
        },
        "FORWARD": {
          "type": "string"
        },
        "TFUEL": {
          "type": "string"
        },
        "ENS": {
          "type": "string"
        },
        "KASDOWN": {
          "type": "string"
        },
        "XTM": {
          "type": "string"
        },
        "DEGEN": {
          "type": "string"
        },
        "TLM": {
          "type": "string"
        },
        "DYDXDOWN": {
          "type": "string"
        },
        "CKB": {
          "type": "string"
        },
        "LUNC": {
          "type": "string"
        },
        "AURORA": {
          "type": "string"
        },
        "LUNA": {
          "type": "string"
        },
        "XTZ": {
          "type": "string"
        },
        "ELON": {
          "type": "string"
        },
        "DMTR": {
          "type": "string"
        },
        "EOS": {
          "type": "string"
        },
        "GST": {
          "type": "string"
        },
        "FORT": {
          "type": "string"
        },
        "FLAME": {
          "type": "string"
        },
        "PATEX": {
          "type": "string"
        },
        "DEEP": {
          "type": "string"
        },
        "ID3L": {
          "type": "string"
        },
        "GTC": {
          "type": "string"
        },
        "ID3S": {
          "type": "string"
        },
        "RIO": {
          "type": "string"
        },
        "CLH": {
          "type": "string"
        },
        "BURGER": {
          "type": "string"
        },
        "VRA": {
          "type": "string"
        },
        "SUNDOG": {
          "type": "string"
        },
        "GTT": {
          "type": "string"
        },
        "INJUP": {
          "type": "string"
        },
        "CPOOL": {
          "type": "string"
        },
        "EPX": {
          "type": "string"
        },
        "CLV": {
          "type": "string"
        },
        "FEAR": {
          "type": "string"
        },
        "MEME": {
          "type": "string"
        },
        "ROOBEE": {
          "type": "string"
        },
        "DEFI": {
          "type": "string"
        },
        "TOKEN": {
          "type": "string"
        },
        "GRAPE": {
          "type": "string"
        },
        "KASUP": {
          "type": "string"
        },
        "XWG": {
          "type": "string"
        },
        "SKEY": {
          "type": "string"
        },
        "SFUND": {
          "type": "string"
        },
        "EQX": {
          "type": "string"
        },
        "ORDIUP": {
          "type": "string"
        },
        "TON": {
          "type": "string"
        },
        "DEGO": {
          "type": "string"
        },
        "IZI": {
          "type": "string"
        },
        "ERG": {
          "type": "string"
        },
        "ERN": {
          "type": "string"
        },
        "VENOM": {
          "type": "string"
        },
        "VOXEL": {
          "type": "string"
        },
        "RLC": {
          "type": "string"
        },
        "PHA": {
          "type": "string"
        },
        "DYDXUP": {
          "type": "string"
        },
        "APE3S": {
          "type": "string"
        },
        "ORBS": {
          "type": "string"
        },
        "OPDOWN": {
          "type": "string"
        },
        "ESE": {
          "type": "string"
        },
        "APE3L": {
          "type": "string"
        },
        "HMND": {
          "type": "string"
        },
        "COQ": {
          "type": "string"
        },
        "AURY": {
          "type": "string"
        },
        "CULT": {
          "type": "string"
        },
        "AKT": {
          "type": "string"
        },
        "GLMR": {
          "type": "string"
        },
        "XYM": {
          "type": "string"
        },
        "ORAI": {
          "type": "string"
        },
        "XYO": {
          "type": "string"
        },
        "ETC": {
          "type": "string"
        },
        "LAI": {
          "type": "string"
        },
        "PIP": {
          "type": "string"
        },
        "ETH": {
          "type": "string"
        },
        "NEO": {
          "type": "string"
        },
        "RMV": {
          "type": "string"
        },
        "KLAY": {
          "type": "string"
        },
        "PIT": {
          "type": "string"
        },
        "TARA": {
          "type": "string"
        },
        "KALT": {
          "type": "string"
        },
        "PIX": {
          "type": "string"
        },
        "ETN": {
          "type": "string"
        },
        "CSIX": {
          "type": "string"
        },
        "TRADE": {
          "type": "string"
        },
        "MAVIA": {
          "type": "string"
        },
        "HIGH": {
          "type": "string"
        },
        "TRB": {
          "type": "string"
        },
        "ORDI": {
          "type": "string"
        },
        "TRVL": {
          "type": "string"
        },
        "AMB": {
          "type": "string"
        },
        "TRU": {
          "type": "string"
        },
        "LOGX": {
          "type": "string"
        },
        "FINC": {
          "type": "string"
        },
        "INFRA": {
          "type": "string"
        },
        "NATIX": {
          "type": "string"
        },
        "NFP": {
          "type": "string"
        },
        "TRY": {
          "type": "string"
        },
        "TRX": {
          "type": "string"
        },
        "LBP": {
          "type": "string"
        },
        "LBR": {
          "type": "string"
        },
        "EUL": {
          "type": "string"
        },
        "NFT": {
          "type": "string"
        },
        "SEIUP": {
          "type": "string"
        },
        "PUFFER": {
          "type": "string"
        },
        "EUR": {
          "type": "string"
        },
        "ORCA": {
          "type": "string"
        },
        "NEAR3L": {
          "type": "string"
        },
        "AMP": {
          "type": "string"
        },
        "XDEFI": {
          "type": "string"
        },
        "HIFI": {
          "type": "string"
        },
        "TRUF": {
          "type": "string"
        },
        "AITECH": {
          "type": "string"
        },
        "AMU": {
          "type": "string"
        },
        "USTC": {
          "type": "string"
        },
        "KNGL": {
          "type": "string"
        },
        "FOXY": {
          "type": "string"
        },
        "NGC": {
          "type": "string"
        },
        "TENET": {
          "type": "string"
        },
        "NEAR3S": {
          "type": "string"
        },
        "MAHA": {
          "type": "string"
        },
        "NGL": {
          "type": "string"
        },
        "TST": {
          "type": "string"
        },
        "HIPPO": {
          "type": "string"
        },
        "AXS3S": {
          "type": "string"
        },
        "CRO": {
          "type": "string"
        },
        "ZPAY": {
          "type": "string"
        },
        "MNDE": {
          "type": "string"
        },
        "CRV": {
          "type": "string"
        },
        "SWASH": {
          "type": "string"
        },
        "AXS3L": {
          "type": "string"
        },
        "VERSE": {
          "type": "string"
        },
        "RPK": {
          "type": "string"
        },
        "RPL": {
          "type": "string"
        },
        "AZERO": {
          "type": "string"
        },
        "SOUL": {
          "type": "string"
        },
        "VXV": {
          "type": "string"
        },
        "LDO": {
          "type": "string"
        },
        "MAGIC": {
          "type": "string"
        },
        "ALICE": {
          "type": "string"
        },
        "SEAM": {
          "type": "string"
        },
        "PLU": {
          "type": "string"
        },
        "AOG": {
          "type": "string"
        },
        "SMOLE": {
          "type": "string"
        },
        "EWT": {
          "type": "string"
        },
        "TSUGT": {
          "type": "string"
        },
        "PMG": {
          "type": "string"
        },
        "OPAI": {
          "type": "string"
        },
        "LOCUS": {
          "type": "string"
        },
        "CTA": {
          "type": "string"
        },
        "NIM": {
          "type": "string"
        },
        "CTC": {
          "type": "string"
        },
        "APE": {
          "type": "string"
        },
        "MERL": {
          "type": "string"
        },
        "JAM": {
          "type": "string"
        },
        "CTI": {
          "type": "string"
        },
        "APP": {
          "type": "string"
        },
        "APT": {
          "type": "string"
        },
        "WLDUP": {
          "type": "string"
        },
        "ZEND": {
          "type": "string"
        },
        "FIRE": {
          "type": "string"
        },
        "DENT": {
          "type": "string"
        },
        "PYTH": {
          "type": "string"
        },
        "LFT": {
          "type": "string"
        },
        "DPET": {
          "type": "string"
        },
        "ORDIDOWN": {
          "type": "string"
        },
        "KPOL": {
          "type": "string"
        },
        "ETHUP": {
          "type": "string"
        },
        "BAND": {
          "type": "string"
        },
        "POL": {
          "type": "string"
        },
        "ASTR": {
          "type": "string"
        },
        "NKN": {
          "type": "string"
        },
        "RSR": {
          "type": "string"
        },
        "DVPN": {
          "type": "string"
        },
        "TWT": {
          "type": "string"
        },
        "ARB": {
          "type": "string"
        },
        "CVC": {
          "type": "string"
        },
        "ARC": {
          "type": "string"
        },
        "XETA": {
          "type": "string"
        },
        "MTRG": {
          "type": "string"
        },
        "LOKA": {
          "type": "string"
        },
        "LPOOL": {
          "type": "string"
        },
        "TURBOS": {
          "type": "string"
        },
        "CVX": {
          "type": "string"
        },
        "ARX": {
          "type": "string"
        },
        "MPLX": {
          "type": "string"
        },
        "SUSHI": {
          "type": "string"
        },
        "NLK": {
          "type": "string"
        },
        "PEPE2": {
          "type": "string"
        },
        "WBTC": {
          "type": "string"
        },
        "SUI3L": {
          "type": "string"
        },
        "CWS": {
          "type": "string"
        },
        "SUI3S": {
          "type": "string"
        },
        "INSP": {
          "type": "string"
        },
        "MANA": {
          "type": "string"
        },
        "VRTX": {
          "type": "string"
        },
        "CSPR": {
          "type": "string"
        },
        "ATA": {
          "type": "string"
        },
        "OPEN": {
          "type": "string"
        },
        "HAI": {
          "type": "string"
        },
        "NMR": {
          "type": "string"
        },
        "ATH": {
          "type": "string"
        },
        "LIT": {
          "type": "string"
        },
        "TLOS": {
          "type": "string"
        },
        "TNSR": {
          "type": "string"
        },
        "CXT": {
          "type": "string"
        },
        "POLYX": {
          "type": "string"
        },
        "ZERO": {
          "type": "string"
        },
        "ROUTE": {
          "type": "string"
        },
        "LOOM": {
          "type": "string"
        },
        "PRE": {
          "type": "string"
        },
        "VRAUP": {
          "type": "string"
        },
        "HBB": {
          "type": "string"
        },
        "RVN": {
          "type": "string"
        },
        "PRQ": {
          "type": "string"
        },
        "ONDO": {
          "type": "string"
        },
        "PEPEDOWN": {
          "type": "string"
        },
        "WOOP": {
          "type": "string"
        },
        "LUNCUP": {
          "type": "string"
        },
        "KAVA": {
          "type": "string"
        },
        "LKI": {
          "type": "string"
        },
        "AVA": {
          "type": "string"
        },
        "NOM": {
          "type": "string"
        },
        "MAPO": {
          "type": "string"
        },
        "PEPEUP": {
          "type": "string"
        },
        "STRAX": {
          "type": "string"
        },
        "NOT": {
          "type": "string"
        },
        "ZERC": {
          "type": "string"
        },
        "BCUT": {
          "type": "string"
        },
        "MASA": {
          "type": "string"
        },
        "WAN": {
          "type": "string"
        },
        "WAT": {
          "type": "string"
        },
        "WAX": {
          "type": "string"
        },
        "MASK": {
          "type": "string"
        },
        "EOS3L": {
          "type": "string"
        },
        "IDEA": {
          "type": "string"
        },
        "EOS3S": {
          "type": "string"
        },
        "YFI": {
          "type": "string"
        },
        "MOODENG": {
          "type": "string"
        },
        "XCUR": {
          "type": "string"
        },
        "HYDRA": {
          "type": "string"
        },
        "POPCAT": {
          "type": "string"
        },
        "LQTY": {
          "type": "string"
        },
        "PIXEL": {
          "type": "string"
        },
        "LMR": {
          "type": "string"
        },
        "ZETA": {
          "type": "string"
        },
        "YGG": {
          "type": "string"
        },
        "AXS": {
          "type": "string"
        },
        "BCHSV": {
          "type": "string"
        },
        "NRN": {
          "type": "string"
        },
        "FTON": {
          "type": "string"
        },
        "COMP": {
          "type": "string"
        },
        "XPRT": {
          "type": "string"
        },
        "HFT": {
          "type": "string"
        },
        "UXLINK": {
          "type": "string"
        },
        "STAMP": {
          "type": "string"
        },
        "RUNE": {
          "type": "string"
        },
        "ZEUS": {
          "type": "string"
        },
        "LTC3L": {
          "type": "string"
        },
        "DAPP": {
          "type": "string"
        },
        "FORTH": {
          "type": "string"
        },
        "ALPINE": {
          "type": "string"
        },
        "SENSO": {
          "type": "string"
        },
        "LTC3S": {
          "type": "string"
        },
        "DEXE": {
          "type": "string"
        },
        "GOAL": {
          "type": "string"
        },
        "AVAX": {
          "type": "string"
        },
        "LISTA": {
          "type": "string"
        },
        "AMPL": {
          "type": "string"
        },
        "WORK": {
          "type": "string"
        },
        "BRWL": {
          "type": "string"
        },
        "BANANA": {
          "type": "string"
        },
        "PUSH": {
          "type": "string"
        },
        "WEN": {
          "type": "string"
        },
        "NEIRO": {
          "type": "string"
        },
        "BTCUP": {
          "type": "string"
        },
        "SOL3S": {
          "type": "string"
        },
        "BRAWL": {
          "type": "string"
        },
        "LAY3R": {
          "type": "string"
        },
        "LPT": {
          "type": "string"
        },
        "GODS": {
          "type": "string"
        },
        "SAND3S": {
          "type": "string"
        },
        "RDNT": {
          "type": "string"
        },
        "SOL3L": {
          "type": "string"
        },
        "NIBI": {
          "type": "string"
        },
        "NUM": {
          "type": "string"
        },
        "PYR": {
          "type": "string"
        },
        "DAG": {
          "type": "string"
        },
        "DAI": {
          "type": "string"
        },
        "HIP": {
          "type": "string"
        },
        "DAO": {
          "type": "string"
        },
        "AVAIL": {
          "type": "string"
        },
        "DAR": {
          "type": "string"
        },
        "FET": {
          "type": "string"
        },
        "FCON": {
          "type": "string"
        },
        "XAVA": {
          "type": "string"
        },
        "LRC": {
          "type": "string"
        },
        "UNI3S": {
          "type": "string"
        },
        "POKT": {
          "type": "string"
        },
        "DASH": {
          "type": "string"
        },
        "BAKEDOWN": {
          "type": "string"
        },
        "POLC": {
          "type": "string"
        },
        "CIRUS": {
          "type": "string"
        },
        "UNI3L": {
          "type": "string"
        },
        "NWC": {
          "type": "string"
        },
        "POLK": {
          "type": "string"
        },
        "LSD": {
          "type": "string"
        },
        "MARS4": {
          "type": "string"
        },
        "LSK": {
          "type": "string"
        },
        "BLOCK": {
          "type": "string"
        },
        "ANALOS": {
          "type": "string"
        },
        "SAFE": {
          "type": "string"
        },
        "DCK": {
          "type": "string"
        },
        "LSS": {
          "type": "string"
        },
        "DCR": {
          "type": "string"
        },
        "LIKE": {
          "type": "string"
        },
        "DATA": {
          "type": "string"
        },
        "WIF": {
          "type": "string"
        },
        "BLOK": {
          "type": "string"
        },
        "LTC": {
          "type": "string"
        },
        "METIS": {
          "type": "string"
        },
        "WIN": {
          "type": "string"
        },
        "HLG": {
          "type": "string"
        },
        "LTO": {
          "type": "string"
        },
        "DYDX": {
          "type": "string"
        },
        "ARB3S": {
          "type": "string"
        },
        "MUBI": {
          "type": "string"
        },
        "ARB3L": {
          "type": "string"
        },
        "RBTC1": {
          "type": "string"
        },
        "POND": {
          "type": "string"
        },
        "LINA": {
          "type": "string"
        },
        "MYRIA": {
          "type": "string"
        },
        "LINK": {
          "type": "string"
        },
        "QTUM": {
          "type": "string"
        },
        "TUNE": {
          "type": "string"
        },
        "UFO": {
          "type": "string"
        },
        "CYBER": {
          "type": "string"
        },
        "WILD": {
          "type": "string"
        },
        "POLS": {
          "type": "string"
        },
        "NYM": {
          "type": "string"
        },
        "FIL": {
          "type": "string"
        },
        "BAL": {
          "type": "string"
        },
        "SCA": {
          "type": "string"
        },
        "STND": {
          "type": "string"
        },
        "WMTX": {
          "type": "string"
        },
        "SCLP": {
          "type": "string"
        },
        "MANEKI": {
          "type": "string"
        },
        "BAT": {
          "type": "string"
        },
        "AKRO": {
          "type": "string"
        },
        "FTM3L": {
          "type": "string"
        },
        "BAX": {
          "type": "string"
        },
        "FTM3S": {
          "type": "string"
        },
        "COTI": {
          "type": "string"
        }
      },
      "required": [
        "AGLD",
        "DFI",
        "PYTHUP",
        "ISLM",
        "NEAR",
        "AIOZ",
        "AUDIO",
        "BBL",
        "WLD",
        "HNT",
        "ETHFI",
        "DMAIL",
        "OPUP",
        "VET3S",
        "MANA3S",
        "TIDAL",
        "HALO",
        "OPUL",
        "MANA3L",
        "DGB",
        "AA",
        "BCH",
        "GMEE",
        "JST",
        "PBUX",
        "AR",
        "SEI",
        "PSTAKE",
        "LMWR",
        "UNFIDOWN",
        "BB",
        "JTO",
        "WEMIX",
        "G",
        "MARSH",
        "BN",
        "FLIP",
        "FLR",
        "BIGTIME",
        "FLY",
        "T",
        "W",
        "BDX",
        "BABYDOGE",
        "SFP",
        "DIA",
        "ISME",
        "LYM",
        "VET3L",
        "JUP",
        "LYX",
        "AIEPK",
        "SILLY",
        "SCPT",
        "WOO",
        "BLUR",
        "STRK",
        "BFC",
        "DC",
        "KARATE",
        "SUSHI3L",
        "NETVR",
        "WAVES",
        "LITH",
        "HAPI",
        "SUSHI3S",
        "CEEK",
        "FLOKI",
        "SHR",
        "SAND",
        "TURT",
        "UMA",
        "BEPRO",
        "SCRT",
        "TUSD",
        "COOKIE",
        "LRDS",
        "SIN",
        "OAS",
        "ROOT",
        "ADA3L",
        "TIAUP",
        "HTR",
        "UNB",
        "UNA",
        "HARD",
        "G3",
        "ADA3S",
        "MYRO",
        "HTX",
        "FT",
        "BTCDOWN",
        "UNI",
        "FX",
        "OBI",
        "UNO",
        "WRX",
        "TIADOWN",
        "ETHDOWN",
        "WELL",
        "SWFTC",
        "SKL",
        "UOS",
        "AIPAD",
        "BRETT",
        "SKY",
        "FRM",
        "VISION",
        "LENDS",
        "SLF",
        "BULL",
        "FLOW",
        "ODDZ",
        "SLN",
        "UPO",
        "SLP",
        "ID",
        "SLIM",
        "SPOT",
        "DOP",
        "ISSP",
        "UQC",
        "IO",
        "DOT",
        "1INCH",
        "SMH",
        "MAK",
        "TOKO",
        "TURBO",
        "UNFI",
        "MAN",
        "EVER",
        "FTM",
        "SHRAP",
        "MAV",
        "MAX",
        "DPR",
        "FTT",
        "ARKM",
        "ATOM",
        "PENDLE",
        "QUICK",
        "BLZ",
        "BOBA",
        "MBL",
        "OFN",
        "UNIO",
        "SNS",
        "SNX",
        "NXRA",
        "TAIKO",
        "AVAX3L",
        "L3",
        "API3",
        "XRP3S",
        "QKC",
        "AVAX3S",
        "ROSE",
        "SATS",
        "BMX",
        "PORTAL",
        "TOMI",
        "XRP3L",
        "SOL",
        "SON",
        "BNC",
        "SOCIAL",
        "CGPT",
        "CELR",
        "BNB",
        "OGN",
        "CELO",
        "AUCTION",
        "MANTA",
        "LAYER",
        "AERO",
        "CETUS",
        "LL",
        "SPA",
        "PYTHDOWN",
        "NEIROCTO",
        "UTK",
        "GMRX",
        "BOB",
        "HOTCROSS",
        "AERGO",
        "MOCA",
        "SQD",
        "MV",
        "BNB3L",
        "BNB3S",
        "GALAX3L",
        "KAI",
        "SQR",
        "GALAX3S",
        "EGLD",
        "ZBCN",
        "KAS",
        "MEW",
        "PUNDIX",
        "LOOKS",
        "FXS",
        "BOSON",
        "BRISE",
        "AEVO",
        "FLUX",
        "PRCL",
        "UNFIUP",
        "SEIDOWN",
        "DOAI",
        "QNT",
        "REDO",
        "STRIKE",
        "ETHW",
        "OM",
        "OP",
        "WHALE",
        "1CAT",
        "NEON",
        "GTAI",
        "SSV",
        "ETH2",
        "KCS",
        "ARPA",
        "ARTFI",
        "BRL",
        "ALEX",
        "STG",
        "SHIB",
        "IOTX",
        "OLE",
        "KDA",
        "CERE",
        "DOCK",
        "STX",
        "OLT",
        "QI",
        "SDAO",
        "BLAST",
        "LINK3S",
        "IOST",
        "SUI",
        "CAKE",
        "BSW",
        "OMG",
        "VOLT",
        "LINK3L",
        "GEEQ",
        "PYUSD",
        "SUN",
        "TOWER",
        "BTC",
        "IOTA",
        "REEF",
        "TRIAS",
        "KEY",
        "ETH3L",
        "BTT",
        "ONE",
        "RENDER",
        "ETH3S",
        "ANKR",
        "ALGO",
        "SYLO",
        "ZCX",
        "SD",
        "ONT",
        "MJT",
        "DYM",
        "DYP",
        "BAKEUP",
        "OOE",
        "ZELIX",
        "DOGE3L",
        "ARTY",
        "QORPO",
        "ICE",
        "NOTAI",
        "DOGE3S",
        "NAKA",
        "GALAX",
        "MKR",
        "DODO",
        "ICP",
        "ZEC",
        "ZEE",
        "ICX",
        "KMNO",
        "TT",
        "DOT3L",
        "XAI",
        "ZEN",
        "DOGE",
        "ALPHA",
        "DUSK",
        "DOT3S",
        "SXP",
        "HBAR",
        "SYNT",
        "ZEX",
        "BONDLY",
        "MLK",
        "KICKS",
        "PEPE",
        "OUSD",
        "LUNCDOWN",
        "DOGS",
        "REV3L",
        "CTSI",
        "C98",
        "OSMO",
        "NTRN",
        "CFX2S",
        "SYN",
        "VIDT",
        "SYS",
        "GAS",
        "BOME",
        "COMBO",
        "XCH",
        "VR",
        "CFX2L",
        "VSYS",
        "PANDORA",
        "THETA",
        "XCN",
        "NEXG",
        "MELOS",
        "XCV",
        "ORN",
        "WLKN",
        "AAVE",
        "MNT",
        "BONK",
        "PERP",
        "XDC",
        "MNW",
        "XDB",
        "BOND",
        "SUIA",
        "MOG",
        "SUTER",
        "TIME",
        "RACA",
        "BICO",
        "MON",
        "SWEAT",
        "MOXIE",
        "BABYBNB",
        "IGU",
        "HMSTR",
        "XEC",
        "MONI",
        "XR",
        "PEOPLE",
        "PUMLX",
        "ZIL",
        "WLDDOWN",
        "VAI",
        "XEN",
        "MPC",
        "XEM",
        "JASMY3S",
        "OTK",
        "TRAC",
        "DFYN",
        "BIDP",
        "JASMY3L",
        "INJDOWN",
        "KLV",
        "WAXL",
        "TRBDOWN",
        "BCH3L",
        "GMT3S",
        "KMD",
        "BCH3S",
        "ECOX",
        "AAVE3S",
        "GMT3L",
        "EPIK",
        "SUIP",
        "AAVE3L",
        "ZK",
        "ZKF",
        "OMNIA",
        "ZKJ",
        "ZKL",
        "GAFI",
        "CARV",
        "KNC",
        "CATS",
        "PROM",
        "ALEPH",
        "PONKE",
        "OVR",
        "CATI",
        "ORDER",
        "GFT",
        "BIFI",
        "GGC",
        "GGG",
        "DAPPX",
        "SUKU",
        "ULTI",
        "CREDI",
        "ERTHA",
        "FURY",
        "KARRAT",
        "MOBILE",
        "SIDUS",
        "NAVI",
        "TAO",
        "USDJ",
        "MTL",
        "VET",
        "FITFI",
        "USDT",
        "OXT",
        "CANDY",
        "USDP",
        "MTS",
        "TADA",
        "MTV",
        "NAVX",
        "ILV",
        "VINU",
        "GHX",
        "EDU",
        "HYVE",
        "BTC3L",
        "ANYONE",
        "BEAT",
        "KING",
        "CREAM",
        "CAS",
        "IMX",
        "CAT",
        "BTC3S",
        "USDE",
        "USDD",
        "CWAR",
        "USDC",
        "KRL",
        "INJ",
        "GAME",
        "TRIBL",
        "XLM",
        "TRBUP",
        "VRADOWN",
        "SUPER",
        "EIGEN",
        "IOI",
        "KSM",
        "CCD",
        "EGO",
        "EGP",
        "MXC",
        "TEL",
        "MOVR",
        "XMR",
        "MXM",
        "OORT",
        "GLM",
        "RAY",
        "XTAG",
        "GLQ",
        "CWEB",
        "REVU",
        "REVV",
        "ZRO",
        "XNL",
        "XNO",
        "SAROS",
        "KACE",
        "ZRX",
        "WLTH",
        "ATOM3L",
        "GMM",
        "BEER",
        "GMT",
        "HEART",
        "GMX",
        "ABBC",
        "OMNI",
        "ATOM3S",
        "IRL",
        "CFG",
        "WSDM",
        "GNS",
        "VANRY",
        "CFX",
        "GRAIL",
        "BEFI",
        "VELO",
        "XPR",
        "DOVI",
        "ACE",
        "ACH",
        "ISP",
        "XCAD",
        "MINA",
        "TIA",
        "DRIFT",
        "ACQ",
        "ACS",
        "MIND",
        "STORE",
        "REN",
        "ELA",
        "DREAMS",
        "ADA",
        "ELF",
        "REQ",
        "STORJ",
        "LADYS",
        "PAXG",
        "REZ",
        "XRD",
        "CHO",
        "CHR",
        "ADS",
        "CHZ",
        "ADX",
        "XRP",
        "JASMY",
        "KAGI",
        "FIDA",
        "PBR",
        "AEG",
        "H2O",
        "CHMB",
        "SAND3L",
        "PBX",
        "SOLVE",
        "DECHAT",
        "GARI",
        "SHIB2L",
        "SHIB2S",
        "ENA",
        "VEMP",
        "ENJ",
        "AFG",
        "RATS",
        "GRT",
        "FORWARD",
        "TFUEL",
        "ENS",
        "KASDOWN",
        "XTM",
        "DEGEN",
        "TLM",
        "DYDXDOWN",
        "CKB",
        "LUNC",
        "AURORA",
        "LUNA",
        "XTZ",
        "ELON",
        "DMTR",
        "EOS",
        "GST",
        "FORT",
        "FLAME",
        "PATEX",
        "DEEP",
        "ID3L",
        "GTC",
        "ID3S",
        "RIO",
        "CLH",
        "BURGER",
        "VRA",
        "SUNDOG",
        "GTT",
        "INJUP",
        "CPOOL",
        "EPX",
        "CLV",
        "FEAR",
        "MEME",
        "ROOBEE",
        "DEFI",
        "TOKEN",
        "GRAPE",
        "KASUP",
        "XWG",
        "SKEY",
        "SFUND",
        "EQX",
        "ORDIUP",
        "TON",
        "DEGO",
        "IZI",
        "ERG",
        "ERN",
        "VENOM",
        "VOXEL",
        "RLC",
        "PHA",
        "DYDXUP",
        "APE3S",
        "ORBS",
        "OPDOWN",
        "ESE",
        "APE3L",
        "HMND",
        "COQ",
        "AURY",
        "CULT",
        "AKT",
        "GLMR",
        "XYM",
        "ORAI",
        "XYO",
        "ETC",
        "LAI",
        "PIP",
        "ETH",
        "NEO",
        "RMV",
        "KLAY",
        "PIT",
        "TARA",
        "KALT",
        "PIX",
        "ETN",
        "CSIX",
        "TRADE",
        "MAVIA",
        "HIGH",
        "TRB",
        "ORDI",
        "TRVL",
        "AMB",
        "TRU",
        "LOGX",
        "FINC",
        "INFRA",
        "NATIX",
        "NFP",
        "TRY",
        "TRX",
        "LBP",
        "LBR",
        "EUL",
        "NFT",
        "SEIUP",
        "PUFFER",
        "EUR",
        "ORCA",
        "NEAR3L",
        "AMP",
        "XDEFI",
        "HIFI",
        "TRUF",
        "AITECH",
        "AMU",
        "USTC",
        "KNGL",
        "FOXY",
        "NGC",
        "TENET",
        "NEAR3S",
        "MAHA",
        "NGL",
        "TST",
        "HIPPO",
        "AXS3S",
        "CRO",
        "ZPAY",
        "MNDE",
        "CRV",
        "SWASH",
        "AXS3L",
        "VERSE",
        "RPK",
        "RPL",
        "AZERO",
        "SOUL",
        "VXV",
        "LDO",
        "MAGIC",
        "ALICE",
        "SEAM",
        "PLU",
        "AOG",
        "SMOLE",
        "EWT",
        "TSUGT",
        "PMG",
        "OPAI",
        "LOCUS",
        "CTA",
        "NIM",
        "CTC",
        "APE",
        "MERL",
        "JAM",
        "CTI",
        "APP",
        "APT",
        "WLDUP",
        "ZEND",
        "FIRE",
        "DENT",
        "PYTH",
        "LFT",
        "DPET",
        "ORDIDOWN",
        "KPOL",
        "ETHUP",
        "BAND",
        "POL",
        "ASTR",
        "NKN",
        "RSR",
        "DVPN",
        "TWT",
        "ARB",
        "CVC",
        "ARC",
        "XETA",
        "MTRG",
        "LOKA",
        "LPOOL",
        "TURBOS",
        "CVX",
        "ARX",
        "MPLX",
        "SUSHI",
        "NLK",
        "PEPE2",
        "WBTC",
        "SUI3L",
        "CWS",
        "SUI3S",
        "INSP",
        "MANA",
        "VRTX",
        "CSPR",
        "ATA",
        "OPEN",
        "HAI",
        "NMR",
        "ATH",
        "LIT",
        "TLOS",
        "TNSR",
        "CXT",
        "POLYX",
        "ZERO",
        "ROUTE",
        "LOOM",
        "PRE",
        "VRAUP",
        "HBB",
        "RVN",
        "PRQ",
        "ONDO",
        "PEPEDOWN",
        "WOOP",
        "LUNCUP",
        "KAVA",
        "LKI",
        "AVA",
        "NOM",
        "MAPO",
        "PEPEUP",
        "STRAX",
        "NOT",
        "ZERC",
        "BCUT",
        "MASA",
        "WAN",
        "WAT",
        "WAX",
        "MASK",
        "EOS3L",
        "IDEA",
        "EOS3S",
        "YFI",
        "MOODENG",
        "XCUR",
        "HYDRA",
        "POPCAT",
        "LQTY",
        "PIXEL",
        "LMR",
        "ZETA",
        "YGG",
        "AXS",
        "BCHSV",
        "NRN",
        "FTON",
        "COMP",
        "XPRT",
        "HFT",
        "UXLINK",
        "STAMP",
        "RUNE",
        "ZEUS",
        "LTC3L",
        "DAPP",
        "FORTH",
        "ALPINE",
        "SENSO",
        "LTC3S",
        "DEXE",
        "GOAL",
        "AVAX",
        "LISTA",
        "AMPL",
        "WORK",
        "BRWL",
        "BANANA",
        "PUSH",
        "WEN",
        "NEIRO",
        "BTCUP",
        "SOL3S",
        "BRAWL",
        "LAY3R",
        "LPT",
        "GODS",
        "SAND3S",
        "RDNT",
        "SOL3L",
        "NIBI",
        "NUM",
        "PYR",
        "DAG",
        "DAI",
        "HIP",
        "DAO",
        "AVAIL",
        "DAR",
        "FET",
        "FCON",
        "XAVA",
        "LRC",
        "UNI3S",
        "POKT",
        "DASH",
        "BAKEDOWN",
        "POLC",
        "CIRUS",
        "UNI3L",
        "NWC",
        "POLK",
        "LSD",
        "MARS4",
        "LSK",
        "BLOCK",
        "ANALOS",
        "SAFE",
        "DCK",
        "LSS",
        "DCR",
        "LIKE",
        "DATA",
        "WIF",
        "BLOK",
        "LTC",
        "METIS",
        "WIN",
        "HLG",
        "LTO",
        "DYDX",
        "ARB3S",
        "MUBI",
        "ARB3L",
        "RBTC1",
        "POND",
        "LINA",
        "MYRIA",
        "LINK",
        "QTUM",
        "TUNE",
        "UFO",
        "CYBER",
        "WILD",
        "POLS",
        "NYM",
        "FIL",
        "BAL",
        "SCA",
        "STND",
        "WMTX",
        "SCLP",
        "MANEKI",
        "BAT",
        "AKRO",
        "FTM3L",
        "BAX",
        "FTM3S",
        "COTI"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-fiat-price-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-fiat-price-responseschema">Response Schema</h3>

Status Code **200**

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| » code      | string | true     | none         | none        |
| » data      | object | true     | none         | none        |
| »» AGLD     | string | true     | none         | none        |
| »» DFI      | string | true     | none         | none        |
| »» PYTHUP   | string | true     | none         | none        |
| »» ISLM     | string | true     | none         | none        |
| »» NEAR     | string | true     | none         | none        |
| »» AIOZ     | string | true     | none         | none        |
| »» AUDIO    | string | true     | none         | none        |
| »» BBL      | string | true     | none         | none        |
| »» WLD      | string | true     | none         | none        |
| »» HNT      | string | true     | none         | none        |
| »» ETHFI    | string | true     | none         | none        |
| »» DMAIL    | string | true     | none         | none        |
| »» OPUP     | string | true     | none         | none        |
| »» VET3S    | string | true     | none         | none        |
| »» MANA3S   | string | true     | none         | none        |
| »» TIDAL    | string | true     | none         | none        |
| »» HALO     | string | true     | none         | none        |
| »» OPUL     | string | true     | none         | none        |
| »» MANA3L   | string | true     | none         | none        |
| »» DGB      | string | true     | none         | none        |
| »» AA       | string | true     | none         | none        |
| »» BCH      | string | true     | none         | none        |
| »» GMEE     | string | true     | none         | none        |
| »» JST      | string | true     | none         | none        |
| »» PBUX     | string | true     | none         | none        |
| »» AR       | string | true     | none         | none        |
| »» SEI      | string | true     | none         | none        |
| »» PSTAKE   | string | true     | none         | none        |
| »» LMWR     | string | true     | none         | none        |
| »» UNFIDOWN | string | true     | none         | none        |
| »» BB       | string | true     | none         | none        |
| »» JTO      | string | true     | none         | none        |
| »» WEMIX    | string | true     | none         | none        |
| »» G        | string | true     | none         | none        |
| »» MARSH    | string | true     | none         | none        |
| »» BN       | string | true     | none         | none        |
| »» FLIP     | string | true     | none         | none        |
| »» FLR      | string | true     | none         | none        |
| »» BIGTIME  | string | true     | none         | none        |
| »» FLY      | string | true     | none         | none        |
| »» T        | string | true     | none         | none        |
| »» W        | string | true     | none         | none        |
| »» BDX      | string | true     | none         | none        |
| »» BABYDOGE | string | true     | none         | none        |
| »» SFP      | string | true     | none         | none        |
| »» DIA      | string | true     | none         | none        |
| »» ISME     | string | true     | none         | none        |
| »» LYM      | string | true     | none         | none        |
| »» VET3L    | string | true     | none         | none        |
| »» JUP      | string | true     | none         | none        |
| »» LYX      | string | true     | none         | none        |
| »» AIEPK    | string | true     | none         | none        |
| »» SILLY    | string | true     | none         | none        |
| »» SCPT     | string | true     | none         | none        |
| »» WOO      | string | true     | none         | none        |
| »» BLUR     | string | true     | none         | none        |
| »» STRK     | string | true     | none         | none        |
| »» BFC      | string | true     | none         | none        |
| »» DC       | string | true     | none         | none        |
| »» KARATE   | string | true     | none         | none        |
| »» SUSHI3L  | string | true     | none         | none        |
| »» NETVR    | string | true     | none         | none        |
| »» WAVES    | string | true     | none         | none        |
| »» LITH     | string | true     | none         | none        |
| »» HAPI     | string | true     | none         | none        |
| »» SUSHI3S  | string | true     | none         | none        |
| »» CEEK     | string | true     | none         | none        |
| »» FLOKI    | string | true     | none         | none        |
| »» SHR      | string | true     | none         | none        |
| »» SAND     | string | true     | none         | none        |
| »» TURT     | string | true     | none         | none        |
| »» UMA      | string | true     | none         | none        |
| »» BEPRO    | string | true     | none         | none        |
| »» SCRT     | string | true     | none         | none        |
| »» TUSD     | string | true     | none         | none        |
| »» COOKIE   | string | true     | none         | none        |
| »» LRDS     | string | true     | none         | none        |
| »» SIN      | string | true     | none         | none        |
| »» OAS      | string | true     | none         | none        |
| »» ROOT     | string | true     | none         | none        |
| »» ADA3L    | string | true     | none         | none        |
| »» TIAUP    | string | true     | none         | none        |
| »» HTR      | string | true     | none         | none        |
| »» UNB      | string | true     | none         | none        |
| »» UNA      | string | true     | none         | none        |
| »» HARD     | string | true     | none         | none        |
| »» G3       | string | true     | none         | none        |
| »» ADA3S    | string | true     | none         | none        |
| »» MYRO     | string | true     | none         | none        |
| »» HTX      | string | true     | none         | none        |
| »» FT       | string | true     | none         | none        |
| »» BTCDOWN  | string | true     | none         | none        |
| »» UNI      | string | true     | none         | none        |
| »» FX       | string | true     | none         | none        |
| »» OBI      | string | true     | none         | none        |
| »» UNO      | string | true     | none         | none        |
| »» WRX      | string | true     | none         | none        |
| »» TIADOWN  | string | true     | none         | none        |
| »» ETHDOWN  | string | true     | none         | none        |
| »» WELL     | string | true     | none         | none        |
| »» SWFTC    | string | true     | none         | none        |
| »» SKL      | string | true     | none         | none        |
| »» UOS      | string | true     | none         | none        |
| »» AIPAD    | string | true     | none         | none        |
| »» BRETT    | string | true     | none         | none        |
| »» SKY      | string | true     | none         | none        |
| »» FRM      | string | true     | none         | none        |
| »» VISION   | string | true     | none         | none        |
| »» LENDS    | string | true     | none         | none        |
| »» SLF      | string | true     | none         | none        |
| »» BULL     | string | true     | none         | none        |
| »» FLOW     | string | true     | none         | none        |
| »» ODDZ     | string | true     | none         | none        |
| »» SLN      | string | true     | none         | none        |
| »» UPO      | string | true     | none         | none        |
| »» SLP      | string | true     | none         | none        |
| »» ID       | string | true     | none         | none        |
| »» SLIM     | string | true     | none         | none        |
| »» SPOT     | string | true     | none         | none        |
| »» DOP      | string | true     | none         | none        |
| »» ISSP     | string | true     | none         | none        |
| »» UQC      | string | true     | none         | none        |
| »» IO       | string | true     | none         | none        |
| »» DOT      | string | true     | none         | none        |
| »» 1INCH    | string | true     | none         | none        |
| »» SMH      | string | true     | none         | none        |
| »» MAK      | string | true     | none         | none        |
| »» TOKO     | string | true     | none         | none        |
| »» TURBO    | string | true     | none         | none        |
| »» UNFI     | string | true     | none         | none        |
| »» MAN      | string | true     | none         | none        |
| »» EVER     | string | true     | none         | none        |
| »» FTM      | string | true     | none         | none        |
| »» SHRAP    | string | true     | none         | none        |
| »» MAV      | string | true     | none         | none        |
| »» MAX      | string | true     | none         | none        |
| »» DPR      | string | true     | none         | none        |
| »» FTT      | string | true     | none         | none        |
| »» ARKM     | string | true     | none         | none        |
| »» ATOM     | string | true     | none         | none        |
| »» PENDLE   | string | true     | none         | none        |
| »» QUICK    | string | true     | none         | none        |
| »» BLZ      | string | true     | none         | none        |
| »» BOBA     | string | true     | none         | none        |
| »» MBL      | string | true     | none         | none        |
| »» OFN      | string | true     | none         | none        |
| »» UNIO     | string | true     | none         | none        |
| »» SNS      | string | true     | none         | none        |
| »» SNX      | string | true     | none         | none        |
| »» NXRA     | string | true     | none         | none        |
| »» TAIKO    | string | true     | none         | none        |
| »» AVAX3L   | string | true     | none         | none        |
| »» L3       | string | true     | none         | none        |
| »» API3     | string | true     | none         | none        |
| »» XRP3S    | string | true     | none         | none        |
| »» QKC      | string | true     | none         | none        |
| »» AVAX3S   | string | true     | none         | none        |
| »» ROSE     | string | true     | none         | none        |
| »» SATS     | string | true     | none         | none        |
| »» BMX      | string | true     | none         | none        |
| »» PORTAL   | string | true     | none         | none        |
| »» TOMI     | string | true     | none         | none        |
| »» XRP3L    | string | true     | none         | none        |
| »» SOL      | string | true     | none         | none        |
| »» SON      | string | true     | none         | none        |
| »» BNC      | string | true     | none         | none        |
| »» SOCIAL   | string | true     | none         | none        |
| »» CGPT     | string | true     | none         | none        |
| »» CELR     | string | true     | none         | none        |
| »» BNB      | string | true     | none         | none        |
| »» OGN      | string | true     | none         | none        |
| »» CELO     | string | true     | none         | none        |
| »» AUCTION  | string | true     | none         | none        |
| »» MANTA    | string | true     | none         | none        |
| »» LAYER    | string | true     | none         | none        |
| »» AERO     | string | true     | none         | none        |
| »» CETUS    | string | true     | none         | none        |
| »» LL       | string | true     | none         | none        |
| »» SPA      | string | true     | none         | none        |
| »» PYTHDOWN | string | true     | none         | none        |
| »» NEIROCTO | string | true     | none         | none        |
| »» UTK      | string | true     | none         | none        |
| »» GMRX     | string | true     | none         | none        |
| »» BOB      | string | true     | none         | none        |
| »» HOTCROSS | string | true     | none         | none        |
| »» AERGO    | string | true     | none         | none        |
| »» MOCA     | string | true     | none         | none        |
| »» SQD      | string | true     | none         | none        |
| »» MV       | string | true     | none         | none        |
| »» BNB3L    | string | true     | none         | none        |
| »» BNB3S    | string | true     | none         | none        |
| »» GALAX3L  | string | true     | none         | none        |
| »» KAI      | string | true     | none         | none        |
| »» SQR      | string | true     | none         | none        |
| »» GALAX3S  | string | true     | none         | none        |
| »» EGLD     | string | true     | none         | none        |
| »» ZBCN     | string | true     | none         | none        |
| »» KAS      | string | true     | none         | none        |
| »» MEW      | string | true     | none         | none        |
| »» PUNDIX   | string | true     | none         | none        |
| »» LOOKS    | string | true     | none         | none        |
| »» FXS      | string | true     | none         | none        |
| »» BOSON    | string | true     | none         | none        |
| »» BRISE    | string | true     | none         | none        |
| »» AEVO     | string | true     | none         | none        |
| »» FLUX     | string | true     | none         | none        |
| »» PRCL     | string | true     | none         | none        |
| »» UNFIUP   | string | true     | none         | none        |
| »» SEIDOWN  | string | true     | none         | none        |
| »» DOAI     | string | true     | none         | none        |
| »» QNT      | string | true     | none         | none        |
| »» REDO     | string | true     | none         | none        |
| »» STRIKE   | string | true     | none         | none        |
| »» ETHW     | string | true     | none         | none        |
| »» OM       | string | true     | none         | none        |
| »» OP       | string | true     | none         | none        |
| »» WHALE    | string | true     | none         | none        |
| »» 1CAT     | string | true     | none         | none        |
| »» NEON     | string | true     | none         | none        |
| »» GTAI     | string | true     | none         | none        |
| »» SSV      | string | true     | none         | none        |
| »» ETH2     | string | true     | none         | none        |
| »» KCS      | string | true     | none         | none        |
| »» ARPA     | string | true     | none         | none        |
| »» ARTFI    | string | true     | none         | none        |
| »» BRL      | string | true     | none         | none        |
| »» ALEX     | string | true     | none         | none        |
| »» STG      | string | true     | none         | none        |
| »» SHIB     | string | true     | none         | none        |
| »» IOTX     | string | true     | none         | none        |
| »» OLE      | string | true     | none         | none        |
| »» KDA      | string | true     | none         | none        |
| »» CERE     | string | true     | none         | none        |
| »» DOCK     | string | true     | none         | none        |
| »» STX      | string | true     | none         | none        |
| »» OLT      | string | true     | none         | none        |
| »» QI       | string | true     | none         | none        |
| »» SDAO     | string | true     | none         | none        |
| »» BLAST    | string | true     | none         | none        |
| »» LINK3S   | string | true     | none         | none        |
| »» IOST     | string | true     | none         | none        |
| »» SUI      | string | true     | none         | none        |
| »» CAKE     | string | true     | none         | none        |
| »» BSW      | string | true     | none         | none        |
| »» OMG      | string | true     | none         | none        |
| »» VOLT     | string | true     | none         | none        |
| »» LINK3L   | string | true     | none         | none        |
| »» GEEQ     | string | true     | none         | none        |
| »» PYUSD    | string | true     | none         | none        |
| »» SUN      | string | true     | none         | none        |
| »» TOWER    | string | true     | none         | none        |
| »» BTC      | string | true     | none         | none        |
| »» IOTA     | string | true     | none         | none        |
| »» REEF     | string | true     | none         | none        |
| »» TRIAS    | string | true     | none         | none        |
| »» KEY      | string | true     | none         | none        |
| »» ETH3L    | string | true     | none         | none        |
| »» BTT      | string | true     | none         | none        |
| »» ONE      | string | true     | none         | none        |
| »» RENDER   | string | true     | none         | none        |
| »» ETH3S    | string | true     | none         | none        |
| »» ANKR     | string | true     | none         | none        |
| »» ALGO     | string | true     | none         | none        |
| »» SYLO     | string | true     | none         | none        |
| »» ZCX      | string | true     | none         | none        |
| »» SD       | string | true     | none         | none        |
| »» ONT      | string | true     | none         | none        |
| »» MJT      | string | true     | none         | none        |
| »» DYM      | string | true     | none         | none        |
| »» DYP      | string | true     | none         | none        |
| »» BAKEUP   | string | true     | none         | none        |
| »» OOE      | string | true     | none         | none        |
| »» ZELIX    | string | true     | none         | none        |
| »» DOGE3L   | string | true     | none         | none        |
| »» ARTY     | string | true     | none         | none        |
| »» QORPO    | string | true     | none         | none        |
| »» ICE      | string | true     | none         | none        |
| »» NOTAI    | string | true     | none         | none        |
| »» DOGE3S   | string | true     | none         | none        |
| »» NAKA     | string | true     | none         | none        |
| »» GALAX    | string | true     | none         | none        |
| »» MKR      | string | true     | none         | none        |
| »» DODO     | string | true     | none         | none        |
| »» ICP      | string | true     | none         | none        |
| »» ZEC      | string | true     | none         | none        |
| »» ZEE      | string | true     | none         | none        |
| »» ICX      | string | true     | none         | none        |
| »» KMNO     | string | true     | none         | none        |
| »» TT       | string | true     | none         | none        |
| »» DOT3L    | string | true     | none         | none        |
| »» XAI      | string | true     | none         | none        |
| »» ZEN      | string | true     | none         | none        |
| »» DOGE     | string | true     | none         | none        |
| »» ALPHA    | string | true     | none         | none        |
| »» DUSK     | string | true     | none         | none        |
| »» DOT3S    | string | true     | none         | none        |
| »» SXP      | string | true     | none         | none        |
| »» HBAR     | string | true     | none         | none        |
| »» SYNT     | string | true     | none         | none        |
| »» ZEX      | string | true     | none         | none        |
| »» BONDLY   | string | true     | none         | none        |
| »» MLK      | string | true     | none         | none        |
| »» KICKS    | string | true     | none         | none        |
| »» PEPE     | string | true     | none         | none        |
| »» OUSD     | string | true     | none         | none        |
| »» LUNCDOWN | string | true     | none         | none        |
| »» DOGS     | string | true     | none         | none        |
| »» REV3L    | string | true     | none         | none        |
| »» CTSI     | string | true     | none         | none        |
| »» C98      | string | true     | none         | none        |
| »» OSMO     | string | true     | none         | none        |
| »» NTRN     | string | true     | none         | none        |
| »» CFX2S    | string | true     | none         | none        |
| »» SYN      | string | true     | none         | none        |
| »» VIDT     | string | true     | none         | none        |
| »» SYS      | string | true     | none         | none        |
| »» GAS      | string | true     | none         | none        |
| »» BOME     | string | true     | none         | none        |
| »» COMBO    | string | true     | none         | none        |
| »» XCH      | string | true     | none         | none        |
| »» VR       | string | true     | none         | none        |
| »» CFX2L    | string | true     | none         | none        |
| »» VSYS     | string | true     | none         | none        |
| »» PANDORA  | string | true     | none         | none        |
| »» THETA    | string | true     | none         | none        |
| »» XCN      | string | true     | none         | none        |
| »» NEXG     | string | true     | none         | none        |
| »» MELOS    | string | true     | none         | none        |
| »» XCV      | string | true     | none         | none        |
| »» ORN      | string | true     | none         | none        |
| »» WLKN     | string | true     | none         | none        |
| »» AAVE     | string | true     | none         | none        |
| »» MNT      | string | true     | none         | none        |
| »» BONK     | string | true     | none         | none        |
| »» PERP     | string | true     | none         | none        |
| »» XDC      | string | true     | none         | none        |
| »» MNW      | string | true     | none         | none        |
| »» XDB      | string | true     | none         | none        |
| »» BOND     | string | true     | none         | none        |
| »» SUIA     | string | true     | none         | none        |
| »» MOG      | string | true     | none         | none        |
| »» SUTER    | string | true     | none         | none        |
| »» TIME     | string | true     | none         | none        |
| »» RACA     | string | true     | none         | none        |
| »» BICO     | string | true     | none         | none        |
| »» MON      | string | true     | none         | none        |
| »» SWEAT    | string | true     | none         | none        |
| »» MOXIE    | string | true     | none         | none        |
| »» BABYBNB  | string | true     | none         | none        |
| »» IGU      | string | true     | none         | none        |
| »» HMSTR    | string | true     | none         | none        |
| »» XEC      | string | true     | none         | none        |
| »» MONI     | string | true     | none         | none        |
| »» XR       | string | true     | none         | none        |
| »» PEOPLE   | string | true     | none         | none        |
| »» PUMLX    | string | true     | none         | none        |
| »» ZIL      | string | true     | none         | none        |
| »» WLDDOWN  | string | true     | none         | none        |
| »» VAI      | string | true     | none         | none        |
| »» XEN      | string | true     | none         | none        |
| »» MPC      | string | true     | none         | none        |
| »» XEM      | string | true     | none         | none        |
| »» JASMY3S  | string | true     | none         | none        |
| »» OTK      | string | true     | none         | none        |
| »» TRAC     | string | true     | none         | none        |
| »» DFYN     | string | true     | none         | none        |
| »» BIDP     | string | true     | none         | none        |
| »» JASMY3L  | string | true     | none         | none        |
| »» INJDOWN  | string | true     | none         | none        |
| »» KLV      | string | true     | none         | none        |
| »» WAXL     | string | true     | none         | none        |
| »» TRBDOWN  | string | true     | none         | none        |
| »» BCH3L    | string | true     | none         | none        |
| »» GMT3S    | string | true     | none         | none        |
| »» KMD      | string | true     | none         | none        |
| »» BCH3S    | string | true     | none         | none        |
| »» ECOX     | string | true     | none         | none        |
| »» AAVE3S   | string | true     | none         | none        |
| »» GMT3L    | string | true     | none         | none        |
| »» EPIK     | string | true     | none         | none        |
| »» SUIP     | string | true     | none         | none        |
| »» AAVE3L   | string | true     | none         | none        |
| »» ZK       | string | true     | none         | none        |
| »» ZKF      | string | true     | none         | none        |
| »» OMNIA    | string | true     | none         | none        |
| »» ZKJ      | string | true     | none         | none        |
| »» ZKL      | string | true     | none         | none        |
| »» GAFI     | string | true     | none         | none        |
| »» CARV     | string | true     | none         | none        |
| »» KNC      | string | true     | none         | none        |
| »» CATS     | string | true     | none         | none        |
| »» PROM     | string | true     | none         | none        |
| »» ALEPH    | string | true     | none         | none        |
| »» PONKE    | string | true     | none         | none        |
| »» OVR      | string | true     | none         | none        |
| »» CATI     | string | true     | none         | none        |
| »» ORDER    | string | true     | none         | none        |
| »» GFT      | string | true     | none         | none        |
| »» BIFI     | string | true     | none         | none        |
| »» GGC      | string | true     | none         | none        |
| »» GGG      | string | true     | none         | none        |
| »» DAPPX    | string | true     | none         | none        |
| »» SUKU     | string | true     | none         | none        |
| »» ULTI     | string | true     | none         | none        |
| »» CREDI    | string | true     | none         | none        |
| »» ERTHA    | string | true     | none         | none        |
| »» FURY     | string | true     | none         | none        |
| »» KARRAT   | string | true     | none         | none        |
| »» MOBILE   | string | true     | none         | none        |
| »» SIDUS    | string | true     | none         | none        |
| »» NAVI     | string | true     | none         | none        |
| »» TAO      | string | true     | none         | none        |
| »» USDJ     | string | true     | none         | none        |
| »» MTL      | string | true     | none         | none        |
| »» VET      | string | true     | none         | none        |
| »» FITFI    | string | true     | none         | none        |
| »» USDT     | string | true     | none         | none        |
| »» OXT      | string | true     | none         | none        |
| »» CANDY    | string | true     | none         | none        |
| »» USDP     | string | true     | none         | none        |
| »» MTS      | string | true     | none         | none        |
| »» TADA     | string | true     | none         | none        |
| »» MTV      | string | true     | none         | none        |
| »» NAVX     | string | true     | none         | none        |
| »» ILV      | string | true     | none         | none        |
| »» VINU     | string | true     | none         | none        |
| »» GHX      | string | true     | none         | none        |
| »» EDU      | string | true     | none         | none        |
| »» HYVE     | string | true     | none         | none        |
| »» BTC3L    | string | true     | none         | none        |
| »» ANYONE   | string | true     | none         | none        |
| »» BEAT     | string | true     | none         | none        |
| »» KING     | string | true     | none         | none        |
| »» CREAM    | string | true     | none         | none        |
| »» CAS      | string | true     | none         | none        |
| »» IMX      | string | true     | none         | none        |
| »» CAT      | string | true     | none         | none        |
| »» BTC3S    | string | true     | none         | none        |
| »» USDE     | string | true     | none         | none        |
| »» USDD     | string | true     | none         | none        |
| »» CWAR     | string | true     | none         | none        |
| »» USDC     | string | true     | none         | none        |
| »» KRL      | string | true     | none         | none        |
| »» INJ      | string | true     | none         | none        |
| »» GAME     | string | true     | none         | none        |
| »» TRIBL    | string | true     | none         | none        |
| »» XLM      | string | true     | none         | none        |
| »» TRBUP    | string | true     | none         | none        |
| »» VRADOWN  | string | true     | none         | none        |
| »» SUPER    | string | true     | none         | none        |
| »» EIGEN    | string | true     | none         | none        |
| »» IOI      | string | true     | none         | none        |
| »» KSM      | string | true     | none         | none        |
| »» CCD      | string | true     | none         | none        |
| »» EGO      | string | true     | none         | none        |
| »» EGP      | string | true     | none         | none        |
| »» MXC      | string | true     | none         | none        |
| »» TEL      | string | true     | none         | none        |
| »» MOVR     | string | true     | none         | none        |
| »» XMR      | string | true     | none         | none        |
| »» MXM      | string | true     | none         | none        |
| »» OORT     | string | true     | none         | none        |
| »» GLM      | string | true     | none         | none        |
| »» RAY      | string | true     | none         | none        |
| »» XTAG     | string | true     | none         | none        |
| »» GLQ      | string | true     | none         | none        |
| »» CWEB     | string | true     | none         | none        |
| »» REVU     | string | true     | none         | none        |
| »» REVV     | string | true     | none         | none        |
| »» ZRO      | string | true     | none         | none        |
| »» XNL      | string | true     | none         | none        |
| »» XNO      | string | true     | none         | none        |
| »» SAROS    | string | true     | none         | none        |
| »» KACE     | string | true     | none         | none        |
| »» ZRX      | string | true     | none         | none        |
| »» WLTH     | string | true     | none         | none        |
| »» ATOM3L   | string | true     | none         | none        |
| »» GMM      | string | true     | none         | none        |
| »» BEER     | string | true     | none         | none        |
| »» GMT      | string | true     | none         | none        |
| »» HEART    | string | true     | none         | none        |
| »» GMX      | string | true     | none         | none        |
| »» ABBC     | string | true     | none         | none        |
| »» OMNI     | string | true     | none         | none        |
| »» ATOM3S   | string | true     | none         | none        |
| »» IRL      | string | true     | none         | none        |
| »» CFG      | string | true     | none         | none        |
| »» WSDM     | string | true     | none         | none        |
| »» GNS      | string | true     | none         | none        |
| »» VANRY    | string | true     | none         | none        |
| »» CFX      | string | true     | none         | none        |
| »» GRAIL    | string | true     | none         | none        |
| »» BEFI     | string | true     | none         | none        |
| »» VELO     | string | true     | none         | none        |
| »» XPR      | string | true     | none         | none        |
| »» DOVI     | string | true     | none         | none        |
| »» ACE      | string | true     | none         | none        |
| »» ACH      | string | true     | none         | none        |
| »» ISP      | string | true     | none         | none        |
| »» XCAD     | string | true     | none         | none        |
| »» MINA     | string | true     | none         | none        |
| »» TIA      | string | true     | none         | none        |
| »» DRIFT    | string | true     | none         | none        |
| »» ACQ      | string | true     | none         | none        |
| »» ACS      | string | true     | none         | none        |
| »» MIND     | string | true     | none         | none        |
| »» STORE    | string | true     | none         | none        |
| »» REN      | string | true     | none         | none        |
| »» ELA      | string | true     | none         | none        |
| »» DREAMS   | string | true     | none         | none        |
| »» ADA      | string | true     | none         | none        |
| »» ELF      | string | true     | none         | none        |
| »» REQ      | string | true     | none         | none        |
| »» STORJ    | string | true     | none         | none        |
| »» LADYS    | string | true     | none         | none        |
| »» PAXG     | string | true     | none         | none        |
| »» REZ      | string | true     | none         | none        |
| »» XRD      | string | true     | none         | none        |
| »» CHO      | string | true     | none         | none        |
| »» CHR      | string | true     | none         | none        |
| »» ADS      | string | true     | none         | none        |
| »» CHZ      | string | true     | none         | none        |
| »» ADX      | string | true     | none         | none        |
| »» XRP      | string | true     | none         | none        |
| »» JASMY    | string | true     | none         | none        |
| »» KAGI     | string | true     | none         | none        |
| »» FIDA     | string | true     | none         | none        |
| »» PBR      | string | true     | none         | none        |
| »» AEG      | string | true     | none         | none        |
| »» H2O      | string | true     | none         | none        |
| »» CHMB     | string | true     | none         | none        |
| »» SAND3L   | string | true     | none         | none        |
| »» PBX      | string | true     | none         | none        |
| »» SOLVE    | string | true     | none         | none        |
| »» DECHAT   | string | true     | none         | none        |
| »» GARI     | string | true     | none         | none        |
| »» SHIB2L   | string | true     | none         | none        |
| »» SHIB2S   | string | true     | none         | none        |
| »» ENA      | string | true     | none         | none        |
| »» VEMP     | string | true     | none         | none        |
| »» ENJ      | string | true     | none         | none        |
| »» AFG      | string | true     | none         | none        |
| »» RATS     | string | true     | none         | none        |
| »» GRT      | string | true     | none         | none        |
| »» FORWARD  | string | true     | none         | none        |
| »» TFUEL    | string | true     | none         | none        |
| »» ENS      | string | true     | none         | none        |
| »» KASDOWN  | string | true     | none         | none        |
| »» XTM      | string | true     | none         | none        |
| »» DEGEN    | string | true     | none         | none        |
| »» TLM      | string | true     | none         | none        |
| »» DYDXDOWN | string | true     | none         | none        |
| »» CKB      | string | true     | none         | none        |
| »» LUNC     | string | true     | none         | none        |
| »» AURORA   | string | true     | none         | none        |
| »» LUNA     | string | true     | none         | none        |
| »» XTZ      | string | true     | none         | none        |
| »» ELON     | string | true     | none         | none        |
| »» DMTR     | string | true     | none         | none        |
| »» EOS      | string | true     | none         | none        |
| »» GST      | string | true     | none         | none        |
| »» FORT     | string | true     | none         | none        |
| »» FLAME    | string | true     | none         | none        |
| »» PATEX    | string | true     | none         | none        |
| »» DEEP     | string | true     | none         | none        |
| »» ID3L     | string | true     | none         | none        |
| »» GTC      | string | true     | none         | none        |
| »» ID3S     | string | true     | none         | none        |
| »» RIO      | string | true     | none         | none        |
| »» CLH      | string | true     | none         | none        |
| »» BURGER   | string | true     | none         | none        |
| »» VRA      | string | true     | none         | none        |
| »» SUNDOG   | string | true     | none         | none        |
| »» GTT      | string | true     | none         | none        |
| »» INJUP    | string | true     | none         | none        |
| »» CPOOL    | string | true     | none         | none        |
| »» EPX      | string | true     | none         | none        |
| »» CLV      | string | true     | none         | none        |
| »» FEAR     | string | true     | none         | none        |
| »» MEME     | string | true     | none         | none        |
| »» ROOBEE   | string | true     | none         | none        |
| »» DEFI     | string | true     | none         | none        |
| »» TOKEN    | string | true     | none         | none        |
| »» GRAPE    | string | true     | none         | none        |
| »» KASUP    | string | true     | none         | none        |
| »» XWG      | string | true     | none         | none        |
| »» SKEY     | string | true     | none         | none        |
| »» SFUND    | string | true     | none         | none        |
| »» EQX      | string | true     | none         | none        |
| »» ORDIUP   | string | true     | none         | none        |
| »» TON      | string | true     | none         | none        |
| »» DEGO     | string | true     | none         | none        |
| »» IZI      | string | true     | none         | none        |
| »» ERG      | string | true     | none         | none        |
| »» ERN      | string | true     | none         | none        |
| »» VENOM    | string | true     | none         | none        |
| »» VOXEL    | string | true     | none         | none        |
| »» RLC      | string | true     | none         | none        |
| »» PHA      | string | true     | none         | none        |
| »» DYDXUP   | string | true     | none         | none        |
| »» APE3S    | string | true     | none         | none        |
| »» ORBS     | string | true     | none         | none        |
| »» OPDOWN   | string | true     | none         | none        |
| »» ESE      | string | true     | none         | none        |
| »» APE3L    | string | true     | none         | none        |
| »» HMND     | string | true     | none         | none        |
| »» COQ      | string | true     | none         | none        |
| »» AURY     | string | true     | none         | none        |
| »» CULT     | string | true     | none         | none        |
| »» AKT      | string | true     | none         | none        |
| »» GLMR     | string | true     | none         | none        |
| »» XYM      | string | true     | none         | none        |
| »» ORAI     | string | true     | none         | none        |
| »» XYO      | string | true     | none         | none        |
| »» ETC      | string | true     | none         | none        |
| »» LAI      | string | true     | none         | none        |
| »» PIP      | string | true     | none         | none        |
| »» ETH      | string | true     | none         | none        |
| »» NEO      | string | true     | none         | none        |
| »» RMV      | string | true     | none         | none        |
| »» KLAY     | string | true     | none         | none        |
| »» PIT      | string | true     | none         | none        |
| »» TARA     | string | true     | none         | none        |
| »» KALT     | string | true     | none         | none        |
| »» PIX      | string | true     | none         | none        |
| »» ETN      | string | true     | none         | none        |
| »» CSIX     | string | true     | none         | none        |
| »» TRADE    | string | true     | none         | none        |
| »» MAVIA    | string | true     | none         | none        |
| »» HIGH     | string | true     | none         | none        |
| »» TRB      | string | true     | none         | none        |
| »» ORDI     | string | true     | none         | none        |
| »» TRVL     | string | true     | none         | none        |
| »» AMB      | string | true     | none         | none        |
| »» TRU      | string | true     | none         | none        |
| »» LOGX     | string | true     | none         | none        |
| »» FINC     | string | true     | none         | none        |
| »» INFRA    | string | true     | none         | none        |
| »» NATIX    | string | true     | none         | none        |
| »» NFP      | string | true     | none         | none        |
| »» TRY      | string | true     | none         | none        |
| »» TRX      | string | true     | none         | none        |
| »» LBP      | string | true     | none         | none        |
| »» LBR      | string | true     | none         | none        |
| »» EUL      | string | true     | none         | none        |
| »» NFT      | string | true     | none         | none        |
| »» SEIUP    | string | true     | none         | none        |
| »» PUFFER   | string | true     | none         | none        |
| »» EUR      | string | true     | none         | none        |
| »» ORCA     | string | true     | none         | none        |
| »» NEAR3L   | string | true     | none         | none        |
| »» AMP      | string | true     | none         | none        |
| »» XDEFI    | string | true     | none         | none        |
| »» HIFI     | string | true     | none         | none        |
| »» TRUF     | string | true     | none         | none        |
| »» AITECH   | string | true     | none         | none        |
| »» AMU      | string | true     | none         | none        |
| »» USTC     | string | true     | none         | none        |
| »» KNGL     | string | true     | none         | none        |
| »» FOXY     | string | true     | none         | none        |
| »» NGC      | string | true     | none         | none        |
| »» TENET    | string | true     | none         | none        |
| »» NEAR3S   | string | true     | none         | none        |
| »» MAHA     | string | true     | none         | none        |
| »» NGL      | string | true     | none         | none        |
| »» TST      | string | true     | none         | none        |
| »» HIPPO    | string | true     | none         | none        |
| »» AXS3S    | string | true     | none         | none        |
| »» CRO      | string | true     | none         | none        |
| »» ZPAY     | string | true     | none         | none        |
| »» MNDE     | string | true     | none         | none        |
| »» CRV      | string | true     | none         | none        |
| »» SWASH    | string | true     | none         | none        |
| »» AXS3L    | string | true     | none         | none        |
| »» VERSE    | string | true     | none         | none        |
| »» RPK      | string | true     | none         | none        |
| »» RPL      | string | true     | none         | none        |
| »» AZERO    | string | true     | none         | none        |
| »» SOUL     | string | true     | none         | none        |
| »» VXV      | string | true     | none         | none        |
| »» LDO      | string | true     | none         | none        |
| »» MAGIC    | string | true     | none         | none        |
| »» ALICE    | string | true     | none         | none        |
| »» SEAM     | string | true     | none         | none        |
| »» PLU      | string | true     | none         | none        |
| »» AOG      | string | true     | none         | none        |
| »» SMOLE    | string | true     | none         | none        |
| »» EWT      | string | true     | none         | none        |
| »» TSUGT    | string | true     | none         | none        |
| »» PMG      | string | true     | none         | none        |
| »» OPAI     | string | true     | none         | none        |
| »» LOCUS    | string | true     | none         | none        |
| »» CTA      | string | true     | none         | none        |
| »» NIM      | string | true     | none         | none        |
| »» CTC      | string | true     | none         | none        |
| »» APE      | string | true     | none         | none        |
| »» MERL     | string | true     | none         | none        |
| »» JAM      | string | true     | none         | none        |
| »» CTI      | string | true     | none         | none        |
| »» APP      | string | true     | none         | none        |
| »» APT      | string | true     | none         | none        |
| »» WLDUP    | string | true     | none         | none        |
| »» ZEND     | string | true     | none         | none        |
| »» FIRE     | string | true     | none         | none        |
| »» DENT     | string | true     | none         | none        |
| »» PYTH     | string | true     | none         | none        |
| »» LFT      | string | true     | none         | none        |
| »» DPET     | string | true     | none         | none        |
| »» ORDIDOWN | string | true     | none         | none        |
| »» KPOL     | string | true     | none         | none        |
| »» ETHUP    | string | true     | none         | none        |
| »» BAND     | string | true     | none         | none        |
| »» POL      | string | true     | none         | none        |
| »» ASTR     | string | true     | none         | none        |
| »» NKN      | string | true     | none         | none        |
| »» RSR      | string | true     | none         | none        |
| »» DVPN     | string | true     | none         | none        |
| »» TWT      | string | true     | none         | none        |
| »» ARB      | string | true     | none         | none        |
| »» CVC      | string | true     | none         | none        |
| »» ARC      | string | true     | none         | none        |
| »» XETA     | string | true     | none         | none        |
| »» MTRG     | string | true     | none         | none        |
| »» LOKA     | string | true     | none         | none        |
| »» LPOOL    | string | true     | none         | none        |
| »» TURBOS   | string | true     | none         | none        |
| »» CVX      | string | true     | none         | none        |
| »» ARX      | string | true     | none         | none        |
| »» MPLX     | string | true     | none         | none        |
| »» SUSHI    | string | true     | none         | none        |
| »» NLK      | string | true     | none         | none        |
| »» PEPE2    | string | true     | none         | none        |
| »» WBTC     | string | true     | none         | none        |
| »» SUI3L    | string | true     | none         | none        |
| »» CWS      | string | true     | none         | none        |
| »» SUI3S    | string | true     | none         | none        |
| »» INSP     | string | true     | none         | none        |
| »» MANA     | string | true     | none         | none        |
| »» VRTX     | string | true     | none         | none        |
| »» CSPR     | string | true     | none         | none        |
| »» ATA      | string | true     | none         | none        |
| »» OPEN     | string | true     | none         | none        |
| »» HAI      | string | true     | none         | none        |
| »» NMR      | string | true     | none         | none        |
| »» ATH      | string | true     | none         | none        |
| »» LIT      | string | true     | none         | none        |
| »» TLOS     | string | true     | none         | none        |
| »» TNSR     | string | true     | none         | none        |
| »» CXT      | string | true     | none         | none        |
| »» POLYX    | string | true     | none         | none        |
| »» ZERO     | string | true     | none         | none        |
| »» ROUTE    | string | true     | none         | none        |
| »» LOOM     | string | true     | none         | none        |
| »» PRE      | string | true     | none         | none        |
| »» VRAUP    | string | true     | none         | none        |
| »» HBB      | string | true     | none         | none        |
| »» RVN      | string | true     | none         | none        |
| »» PRQ      | string | true     | none         | none        |
| »» ONDO     | string | true     | none         | none        |
| »» PEPEDOWN | string | true     | none         | none        |
| »» WOOP     | string | true     | none         | none        |
| »» LUNCUP   | string | true     | none         | none        |
| »» KAVA     | string | true     | none         | none        |
| »» LKI      | string | true     | none         | none        |
| »» AVA      | string | true     | none         | none        |
| »» NOM      | string | true     | none         | none        |
| »» MAPO     | string | true     | none         | none        |
| »» PEPEUP   | string | true     | none         | none        |
| »» STRAX    | string | true     | none         | none        |
| »» NOT      | string | true     | none         | none        |
| »» ZERC     | string | true     | none         | none        |
| »» BCUT     | string | true     | none         | none        |
| »» MASA     | string | true     | none         | none        |
| »» WAN      | string | true     | none         | none        |
| »» WAT      | string | true     | none         | none        |
| »» WAX      | string | true     | none         | none        |
| »» MASK     | string | true     | none         | none        |
| »» EOS3L    | string | true     | none         | none        |
| »» IDEA     | string | true     | none         | none        |
| »» EOS3S    | string | true     | none         | none        |
| »» YFI      | string | true     | none         | none        |
| »» MOODENG  | string | true     | none         | none        |
| »» XCUR     | string | true     | none         | none        |
| »» HYDRA    | string | true     | none         | none        |
| »» POPCAT   | string | true     | none         | none        |
| »» LQTY     | string | true     | none         | none        |
| »» PIXEL    | string | true     | none         | none        |
| »» LMR      | string | true     | none         | none        |
| »» ZETA     | string | true     | none         | none        |
| »» YGG      | string | true     | none         | none        |
| »» AXS      | string | true     | none         | none        |
| »» BCHSV    | string | true     | none         | none        |
| »» NRN      | string | true     | none         | none        |
| »» FTON     | string | true     | none         | none        |
| »» COMP     | string | true     | none         | none        |
| »» XPRT     | string | true     | none         | none        |
| »» HFT      | string | true     | none         | none        |
| »» UXLINK   | string | true     | none         | none        |
| »» STAMP    | string | true     | none         | none        |
| »» RUNE     | string | true     | none         | none        |
| »» ZEUS     | string | true     | none         | none        |
| »» LTC3L    | string | true     | none         | none        |
| »» DAPP     | string | true     | none         | none        |
| »» FORTH    | string | true     | none         | none        |
| »» ALPINE   | string | true     | none         | none        |
| »» SENSO    | string | true     | none         | none        |
| »» LTC3S    | string | true     | none         | none        |
| »» DEXE     | string | true     | none         | none        |
| »» GOAL     | string | true     | none         | none        |
| »» AVAX     | string | true     | none         | none        |
| »» LISTA    | string | true     | none         | none        |
| »» AMPL     | string | true     | none         | none        |
| »» WORK     | string | true     | none         | none        |
| »» BRWL     | string | true     | none         | none        |
| »» BANANA   | string | true     | none         | none        |
| »» PUSH     | string | true     | none         | none        |
| »» WEN      | string | true     | none         | none        |
| »» NEIRO    | string | true     | none         | none        |
| »» BTCUP    | string | true     | none         | none        |
| »» SOL3S    | string | true     | none         | none        |
| »» BRAWL    | string | true     | none         | none        |
| »» LAY3R    | string | true     | none         | none        |
| »» LPT      | string | true     | none         | none        |
| »» GODS     | string | true     | none         | none        |
| »» SAND3S   | string | true     | none         | none        |
| »» RDNT     | string | true     | none         | none        |
| »» SOL3L    | string | true     | none         | none        |
| »» NIBI     | string | true     | none         | none        |
| »» NUM      | string | true     | none         | none        |
| »» PYR      | string | true     | none         | none        |
| »» DAG      | string | true     | none         | none        |
| »» DAI      | string | true     | none         | none        |
| »» HIP      | string | true     | none         | none        |
| »» DAO      | string | true     | none         | none        |
| »» AVAIL    | string | true     | none         | none        |
| »» DAR      | string | true     | none         | none        |
| »» FET      | string | true     | none         | none        |
| »» FCON     | string | true     | none         | none        |
| »» XAVA     | string | true     | none         | none        |
| »» LRC      | string | true     | none         | none        |
| »» UNI3S    | string | true     | none         | none        |
| »» POKT     | string | true     | none         | none        |
| »» DASH     | string | true     | none         | none        |
| »» BAKEDOWN | string | true     | none         | none        |
| »» POLC     | string | true     | none         | none        |
| »» CIRUS    | string | true     | none         | none        |
| »» UNI3L    | string | true     | none         | none        |
| »» NWC      | string | true     | none         | none        |
| »» POLK     | string | true     | none         | none        |
| »» LSD      | string | true     | none         | none        |
| »» MARS4    | string | true     | none         | none        |
| »» LSK      | string | true     | none         | none        |
| »» BLOCK    | string | true     | none         | none        |
| »» ANALOS   | string | true     | none         | none        |
| »» SAFE     | string | true     | none         | none        |
| »» DCK      | string | true     | none         | none        |
| »» LSS      | string | true     | none         | none        |
| »» DCR      | string | true     | none         | none        |
| »» LIKE     | string | true     | none         | none        |
| »» DATA     | string | true     | none         | none        |
| »» WIF      | string | true     | none         | none        |
| »» BLOK     | string | true     | none         | none        |
| »» LTC      | string | true     | none         | none        |
| »» METIS    | string | true     | none         | none        |
| »» WIN      | string | true     | none         | none        |
| »» HLG      | string | true     | none         | none        |
| »» LTO      | string | true     | none         | none        |
| »» DYDX     | string | true     | none         | none        |
| »» ARB3S    | string | true     | none         | none        |
| »» MUBI     | string | true     | none         | none        |
| »» ARB3L    | string | true     | none         | none        |
| »» RBTC1    | string | true     | none         | none        |
| »» POND     | string | true     | none         | none        |
| »» LINA     | string | true     | none         | none        |
| »» MYRIA    | string | true     | none         | none        |
| »» LINK     | string | true     | none         | none        |
| »» QTUM     | string | true     | none         | none        |
| »» TUNE     | string | true     | none         | none        |
| »» UFO      | string | true     | none         | none        |
| »» CYBER    | string | true     | none         | none        |
| »» WILD     | string | true     | none         | none        |
| »» POLS     | string | true     | none         | none        |
| »» NYM      | string | true     | none         | none        |
| »» FIL      | string | true     | none         | none        |
| »» BAL      | string | true     | none         | none        |
| »» SCA      | string | true     | none         | none        |
| »» STND     | string | true     | none         | none        |
| »» WMTX     | string | true     | none         | none        |
| »» SCLP     | string | true     | none         | none        |
| »» MANEKI   | string | true     | none         | none        |
| »» BAT      | string | true     | none         | none        |
| »» AKRO     | string | true     | none         | none        |
| »» FTM3L    | string | true     | none         | none        |
| »» BAX      | string | true     | none         | none        |
| »» FTM3S    | string | true     | none         | none        |
| »» COTI     | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get 24hr Stats

<a id="opId015"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/market/stats?symbol=type,string", {
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

r = requests.get('/api/v1/market/stats', params={
  'symbol': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/market/stats`

Request via this endpoint to get the statistics of the specified ticker in the
last 24 hours.

<h3 id="get-24hr-stats-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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
        "time": {
          "type": "integer",
          "description": "timestamp",
          "format": "int64"
        },
        "symbol": {
          "type": "string",
          "description": "Symbol"
        },
        "buy": {
          "type": "string",
          "description": "Best bid price\n"
        },
        "sell": {
          "type": "string",
          "description": "Best ask price"
        },
        "changeRate": {
          "type": "string",
          "description": "24h change rate"
        },
        "changePrice": {
          "type": "string",
          "description": "24h change price"
        },
        "high": {
          "type": "string",
          "description": "Highest price in 24h"
        },
        "low": {
          "type": "string",
          "description": "Lowest price in 24h"
        },
        "vol": {
          "type": "string",
          "description": "24h volume, executed based on base currency"
        },
        "volValue": {
          "type": "string",
          "description": "24h traded amount"
        },
        "last": {
          "type": "string",
          "description": "Last traded price"
        },
        "averagePrice": {
          "type": "string",
          "description": "Average trading price in the last 24 hours"
        },
        "takerFeeRate": {
          "type": "string",
          "description": "Basic Taker Fee"
        },
        "makerFeeRate": {
          "type": "string",
          "description": "Basic Maker Fee"
        },
        "takerCoefficient": {
          "type": "string",
          "description": "The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
        },
        "makerCoefficient": {
          "type": "string",
          "description": "The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee"
        }
      },
      "required": [
        "time",
        "symbol",
        "buy",
        "sell",
        "changeRate",
        "changePrice",
        "high",
        "low",
        "vol",
        "volValue",
        "last",
        "averagePrice",
        "takerFeeRate",
        "makerFeeRate",
        "takerCoefficient",
        "makerCoefficient"
      ]
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

| Name                | Type           | Required | Restrictions | Description                                                                                                                                                                      |
| ------------------- | -------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code              | string         | true     | none         | none                                                                                                                                                                             |
| » data              | object         | true     | none         | none                                                                                                                                                                             |
| »» time             | integer(int64) | true     | none         | timestamp                                                                                                                                                                        |
| »» symbol           | string         | true     | none         | Symbol                                                                                                                                                                           |
| »» buy              | string         | true     | none         | Best bid price                                                                                                                                                                   |
| »» sell             | string         | true     | none         | Best ask price                                                                                                                                                                   |
| »» changeRate       | string         | true     | none         | 24h change rate                                                                                                                                                                  |
| »» changePrice      | string         | true     | none         | 24h change price                                                                                                                                                                 |
| »» high             | string         | true     | none         | Highest price in 24h                                                                                                                                                             |
| »» low              | string         | true     | none         | Lowest price in 24h                                                                                                                                                              |
| »» vol              | string         | true     | none         | 24h volume, executed based on base currency                                                                                                                                      |
| »» volValue         | string         | true     | none         | 24h traded amount                                                                                                                                                                |
| »» last             | string         | true     | none         | Last traded price                                                                                                                                                                |
| »» averagePrice     | string         | true     | none         | Average trading price in the last 24 hours                                                                                                                                       |
| »» takerFeeRate     | string         | true     | none         | Basic Taker Fee                                                                                                                                                                  |
| »» makerFeeRate     | string         | true     | none         | Basic Maker Fee                                                                                                                                                                  |
| »» takerCoefficient | string         | true     | none         | The taker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee |
| »» makerCoefficient | string         | true     | none         | The maker fee coefficient. The actual fee needs to be multiplied by this coefficient to get the final fee. Most currencies have a coefficient of 1. If set to 0, it means no fee |

<aside class="success">
This operation does not require authentication
</aside>

## Get Market List

<a id="opId016"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/markets", {
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

r = requests.get('/api/v1/markets', headers = headers)

print(r.json())

```

`GET /api/v1/markets`

Request via this endpoint the transaction currency for the entire trading
market.

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
        "type": "string"
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-market-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-market-list-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type     | Required | Restrictions | Description |
| ------ | -------- | -------- | ------------ | ----------- |
| » code | string   | true     | none         | none        |
| » data | [string] | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Client IP Address

<a id="opId017"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/my-ip", {
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

r = requests.get('/api/v1/my-ip', headers = headers)

print(r.json())

```

`GET /api/v1/my-ip`

Get the server time.

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

<h3 id="get-client-ip-address-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-client-ip-address-responseschema">Response Schema</h3>

Status Code **200**

| Name   | Type   | Required | Restrictions | Description |
| ------ | ------ | -------- | ------------ | ----------- |
| » code | string | true     | none         | none        |
| » data | string | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Server Time

<a id="opId018"></a>

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

Get the server time.

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

<a id="opId019"></a>

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
        },
        "msg": {
          "type": "string",
          "description": "Remark for operation"
        }
      },
      "required": ["status", "msg"]
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
| »» status | string | true     | none         | Status of service: open: normal transaction; close: Stop Trading/Maintenance; cancelonly: can only cancel the order but not place order |
| »» msg    | string | true     | none         | Remark for operation                                                                                                                    |

#### Enumerated Values

| Property | Value      |
| -------- | ---------- |
| status   | open       |
| status   | close      |
| status   | cancelonly |

<aside class="success">
This operation does not require authentication
</aside>

## Get Public Token - Spot/Margin

<a id="opId020"></a>

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
Spot/Margin connection. If you need use public channels (e.g. all public market
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

<h3 id="get-public-token---spot/margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-public-token---spot/margin-responseschema">Response Schema</h3>

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

## Get Private Token - Spot/Margin

<a id="opId021"></a>

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
Spot/Margin private connection. If you need use private channels (e.g. account
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

<h3 id="get-private-token---spot/margin-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-private-token---spot/margin-responseschema">Response Schema</h3>

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
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
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
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds\n\nWhen placing a market order, the funds field refers to the funds for the priced asset (the asset name written latter) of the trading pair. The funds must be based on the quoteIncrement of the trading pair. The quoteIncrement represents the precision of the trading pair. The funds value for an order must be a multiple of quoteIncrement and must be between quoteMinSize and quoteMaxSize."
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [
        10,
        20,
        30
      ]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [
        1740711735178
      ]
    }
  },
  "required": [
    "symbol",
    "type",
    "side"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders',
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

r = requests.post('/api/v1/hf/orders', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders`

Place order to the Spot trading system, you can place two major types of orders:
limit and market. Orders can only be placed if your account has sufficient
funds. Once an order is placed, your funds will be put on hold for the duration
of the order. The amount of funds on hold depends on the order type and
parameters specified.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds\n\nWhen placing a market order, the funds field refers to the funds for the priced asset (the asset name written latter) of the trading pair. The funds must be based on the quoteIncrement of the trading pair. The quoteIncrement represents the precision of the trading pair. The funds value for an order must be a multiple of quoteIncrement and must be between quoteMinSize and quoteMaxSize."
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [10, 20, 30]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [1740711735178]
    }
  },
  "required": ["symbol", "type", "side"]
}
```

<h3 id="add-order-parameters">Parameters</h3>

| Name                 | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body                 | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » clientOid          | body | string         | false    | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side               | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » symbol             | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » type               | body | string         | true     | specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » remark             | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » stp                | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                            |
| » price              | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » size               | body | string         | false    | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » timeInForce        | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                               |
| » postOnly           | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                                 |
| » hidden             | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                         |
| » iceberg            | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                            |
| » visibleSize        | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » tags               | body | string         | false    | Order tag, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » cancelAfter        | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                                  |
| » funds              | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » allowMaxTimeWindow | body | integer(int64) | false    | Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.                                                                                                                                                                                                                                                                                                                            |
| » clientTimestamp    | body | integer(int64) | false    | Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.                                                                                                                                                                                                                                                                                                                                                                                                   |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: specify if the order is an 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price, you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

**» funds**: When **type** is market, select one out of two: size or funds

When placing a market order, the funds field refers to the funds for the priced
asset (the asset name written latter) of the trading pair. The funds must be
based on the quoteIncrement of the trading pair. The quoteIncrement represents
the precision of the trading pair. The funds value for an order must be a
multiple of quoteIncrement and must be between quoteMinSize and quoteMaxSize.

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

<h3 id="add-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-responseschema">Response Schema</h3>

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

## Cancel All Orders By Symbol

<a id="opId011"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.delete('/api/v1/hf/orders', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders`

This endpoint can cancel all spot orders for specific symbol. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to websocket.

<h3 id="cancel-all-orders-by-symbol-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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

## Add Order Sync

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
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [
        10,
        20,
        30
      ]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [
        1740711735178
      ]
    }
  },
  "required": [
    "symbol",
    "type",
    "side"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/sync',
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

r = requests.post('/api/v1/hf/orders/sync', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/sync`

Place order in the spot trading system. The difference between this interface
and "Add order" is that this interface will synchronously return the order
information after the order matching is completed. For higher latency
requirements, please select the "Add order" interface. If there is a requirement
for returning data integrity, please select this interface.

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
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [10, 20, 30]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [1740711735178]
    }
  },
  "required": ["symbol", "type", "side"]
}
```

<h3 id="add-order-sync-parameters">Parameters</h3>

| Name                 | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| -------------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body                 | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid          | body | string         | false    | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side               | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol             | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type               | body | string         | true     | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » remark             | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                            |
| » stp                | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                       |
| » price              | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size               | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce        | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                          |
| » postOnly           | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden             | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| » iceberg            | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| » visibleSize        | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » tags               | body | string         | false    | Order tag, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » cancelAfter        | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                             |
| » funds              | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » allowMaxTimeWindow | body | integer(int64) | false    | Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.                                                                                                                                                                                                                                                                                                                       |
| » clientTimestamp    | body | integer(int64) | false    | Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.                                                                                                                                                                                                                                                                                                                                                                                             |

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
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        },
        "orderTime": {
          "type": "integer",
          "format": "int64"
        },
        "originSize": {
          "type": "string",
          "description": "Original order size"
        },
        "dealSize": {
          "type": "string",
          "description": "Deal size"
        },
        "remainSize": {
          "type": "string",
          "description": "Remain size"
        },
        "canceledSize": {
          "type": "string",
          "description": "Cumulative canceled size"
        },
        "status": {
          "type": "string",
          "description": "Order Status. open: order is active; done: order has been completed",
          "enum": ["open", "done"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "",
              "description": ""
            },
            {
              "value": "done",
              "name": "",
              "description": ""
            }
          ]
        },
        "matchTime": {
          "type": "integer",
          "format": "int64"
        }
      },
      "required": [
        "orderId",
        "clientOid",
        "orderTime",
        "originSize",
        "dealSize",
        "remainSize",
        "canceledSize",
        "status",
        "matchTime"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-sync-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-sync-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                   |
| --------------- | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                                                                                          |
| » data          | object         | true     | none         | none                                                                                                                          |
| »» orderId      | string         | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» clientOid    | string         | true     | none         | The user self-defined order ID.                                                                                               |
| »» orderTime    | integer(int64) | true     | none         | none                                                                                                                          |
| »» originSize   | string         | true     | none         | Original order size                                                                                                           |
| »» dealSize     | string         | true     | none         | Deal size                                                                                                                     |
| »» remainSize   | string         | true     | none         | Remain size                                                                                                                   |
| »» canceledSize | string         | true     | none         | Cumulative canceled size                                                                                                      |
| »» status       | string         | true     | none         | Order Status. open: order is active; done: order has been completed                                                           |
| »» matchTime    | integer(int64) | true     | none         | none                                                                                                                          |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | open  |
| status   | done  |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order Test

<a id="opId003"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
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
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [
        10,
        20,
        30
      ]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [
        1740711735178
      ]
    }
  },
  "required": [
    "symbol",
    "type",
    "side"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/test',
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

r = requests.post('/api/v1/hf/orders/test', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/test`

Order test endpoint, the request parameters and return parameters of this
endpoint are exactly the same as the order endpoint, and can be used to verify
whether the signature is correct and other operations. After placing an order,
the order will not enter the matching system, and the order cannot be queried.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
    "tags": {
      "type": "string",
      "description": "Order tag, length cannot exceed 20 characters (ASCII)"
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "allowMaxTimeWindow": {
      "type": "integer",
      "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
      "format": "int64",
      "example": [10, 20, 30]
    },
    "clientTimestamp": {
      "type": "integer",
      "description": "Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.",
      "format": "int64",
      "example": [1740711735178]
    }
  },
  "required": ["symbol", "type", "side"]
}
```

<h3 id="add-order-test-parameters">Parameters</h3>

| Name                 | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body                 | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » clientOid          | body | string         | false    | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side               | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » symbol             | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » type               | body | string         | true     | specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » remark             | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » stp                | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                            |
| » price              | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » size               | body | string         | false    | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » timeInForce        | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                               |
| » postOnly           | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                                 |
| » hidden             | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                         |
| » iceberg            | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                            |
| » visibleSize        | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » tags               | body | string         | false    | Order tag, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » cancelAfter        | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                                  |
| » funds              | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » allowMaxTimeWindow | body | integer(int64) | false    | Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.                                                                                                                                                                                                                                                                                                                            |
| » clientTimestamp    | body | integer(int64) | false    | Equal to KC-API-TIMESTAMP, Need to be defined if allowMaxTimeWindow is specified.                                                                                                                                                                                                                                                                                                                                                                                                   |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: specify if the order is an 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price, you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
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

<a id="opId004"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n"
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
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
            "enum": [
              "limit"
            ],
            "x-api-enum": [
              {
                "value": "limit",
                "name": "",
                "description": ""
              }
            ]
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
            ]
          },
          "price": {
            "type": "string",
            "description": "Specify price for order"
          },
          "size": {
            "type": "string",
            "description": "Specify quantity for order.\n\nWhen **type** is limited, select one out of two: size or funds. Size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
          "cancelAfter": {
            "type": "integer",
            "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
            "format": "int64",
            "default": -1,
            "minimum": 0,
            "exclusiveMinimum": 0,
            "maximum": 2592000,
            "exclusiveMaximum": 2592000
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
          "tags": {
            "type": "string",
            "description": "Order tag, length cannot exceed 20 characters (ASCII)"
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
          },
          "funds": {
            "type": "string",
            "description": "When **type** is market, select one out of two: size or funds"
          },
          "clientTimestamp": {
            "type": "integer",
            "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
            "format": "int64",
            "example": [
              1740711735178
            ]
          },
          "allowMaxTimeWindow": {
            "type": "integer",
            "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
            "format": "int64",
            "example": [
              10,
              20,
              30
            ]
          }
        },
        "required": [
          "type",
          "symbol",
          "side",
          "price"
        ]
      },
      "description": "Order List"
    }
  },
  "required": [
    "orderList"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/multi',
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

r = requests.post('/api/v1/hf/orders/multi', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/multi`

This endpoint supports sequential batch order placement from a single endpoint.
A maximum of 5 orders can be placed simultaneously. The order types must be
limit orders of the same trading pair

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n"
          },
          "symbol": {
            "type": "string",
            "description": "symbol",
            "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
          },
          "type": {
            "type": "string",
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
            "enum": ["limit"],
            "x-api-enum": [
              {
                "value": "limit",
                "name": "",
                "description": ""
              }
            ]
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
            ]
          },
          "price": {
            "type": "string",
            "description": "Specify price for order"
          },
          "size": {
            "type": "string",
            "description": "Specify quantity for order.\n\nWhen **type** is limited, select one out of two: size or funds. Size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
          "cancelAfter": {
            "type": "integer",
            "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
            "format": "int64",
            "default": -1,
            "minimum": 0,
            "exclusiveMinimum": 0,
            "maximum": 2592000,
            "exclusiveMaximum": 2592000
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
          "tags": {
            "type": "string",
            "description": "Order tag, length cannot exceed 20 characters (ASCII)"
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
          },
          "funds": {
            "type": "string",
            "description": "When **type** is market, select one out of two: size or funds"
          },
          "clientTimestamp": {
            "type": "integer",
            "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
            "format": "int64",
            "example": [1740711735178]
          },
          "allowMaxTimeWindow": {
            "type": "integer",
            "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
            "format": "int64",
            "example": [10, 20, 30]
          }
        },
        "required": ["type", "symbol", "side", "price"]
      },
      "description": "Order List"
    }
  },
  "required": ["orderList"]
}
```

<h3 id="batch-add-orders-parameters">Parameters</h3>

| Name                  | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body                  | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » orderList           | body | [object]       | true     | Order List                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» clientOid          | body | string         | false    | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| »» symbol             | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| »» type               | body | string         | true     | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» timeInForce        | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                          |
| »» side               | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| »» price              | body | string         | true     | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| »» size               | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| »» stp                | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                       |
| »» cancelAfter        | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                             |
| »» postOnly           | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| »» hidden             | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| »» iceberg            | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| »» visibleSize        | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» tags               | body | string         | false    | Order tag, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» remark             | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                            |
| »» funds              | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| »» clientTimestamp    | body | integer(int64) | false    | Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.                                                                                                                                                                                                                                                                                                                                                                                             |
| »» allowMaxTimeWindow | body | integer(int64) | false    | Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.                                                                                                                                                                                                                                                                                                                       |

#### Detailed descriptions

**»» size**: Specify quantity for order.

When **type** is limited, select one out of two: size or funds. Size refers to
the amount of trading targets (the asset name written in front) for the trading
pair. The Size must be based on the baseIncrement of the trading pair. The
baseIncrement represents the precision for the trading pair. The size of an
order must be a positive-integer multiple of baseIncrement and must be between
baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter      | Value |
| -------------- | ----- |
| »» type        | limit |
| »» timeInForce | GTC   |
| »» timeInForce | GTT   |
| »» timeInForce | IOC   |
| »» timeInForce | FOK   |
| »» side        | buy   |
| »» side        | sell  |
| »» stp         | DC    |
| »» stp         | CO    |
| »» stp         | CN    |
| »» stp         | CB    |

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
            "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
          },
          "clientOid": {
            "type": "string",
            "description": "The user self-defined order ID."
          },
          "success": {
            "type": "boolean",
            "description": "Add order success/failure"
          },
          "failMsg": {
            "type": "string",
            "description": "Error message"
          }
        },
        "required": ["success"]
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

| Name         | Type     | Required | Restrictions | Description                                                                                                                   |
| ------------ | -------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code       | string   | true     | none         | none                                                                                                                          |
| » data       | [object] | true     | none         | none                                                                                                                          |
| »» orderId   | string   | false    | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» clientOid | string   | false    | none         | The user self-defined order ID.                                                                                               |
| »» success   | boolean  | true     | none         | Add order success/failure                                                                                                     |
| »» failMsg   | string   | false    | none         | Error message                                                                                                                 |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Add Orders Sync

<a id="opId005"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n"
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
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
            "enum": [
              "limit"
            ],
            "x-api-enum": [
              {
                "value": "limit",
                "name": "",
                "description": ""
              }
            ]
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
            ]
          },
          "price": {
            "type": "string",
            "description": "Specify price for order"
          },
          "size": {
            "type": "string",
            "description": "Specify quantity for order.\n\nWhen **type** is limited, select one out of two: size or funds. Size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
          "cancelAfter": {
            "type": "integer",
            "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
            "format": "int64",
            "default": -1,
            "minimum": 0,
            "exclusiveMinimum": 0,
            "maximum": 2592000,
            "exclusiveMaximum": 2592000
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
          "tags": {
            "type": "string",
            "description": "Order tag, length cannot exceed 20 characters (ASCII)"
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
          },
          "funds": {
            "type": "string",
            "description": "When **type** is market, select one out of two: size or funds"
          },
          "allowMaxTimeWindow": {
            "type": "integer",
            "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
            "format": "int64",
            "example": [
              10,
              20,
              30
            ]
          },
          "clientTimestamp": {
            "type": "integer",
            "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
            "format": "int64",
            "example": [
              1740711735178
            ]
          }
        },
        "required": [
          "type",
          "symbol",
          "side",
          "price"
        ]
      },
      "description": "Order List"
    }
  },
  "required": [
    "orderList"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/multi/sync',
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

r = requests.post('/api/v1/hf/orders/multi/sync', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/multi/sync`

This endpoint supports sequential batch order placement from a single endpoint.
A maximum of 5 orders can be placed simultaneously. The order types must be
limit orders of the same trading pair

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n"
          },
          "symbol": {
            "type": "string",
            "description": "symbol",
            "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
          },
          "type": {
            "type": "string",
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
            "enum": ["limit"],
            "x-api-enum": [
              {
                "value": "limit",
                "name": "",
                "description": ""
              }
            ]
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
            ]
          },
          "price": {
            "type": "string",
            "description": "Specify price for order"
          },
          "size": {
            "type": "string",
            "description": "Specify quantity for order.\n\nWhen **type** is limited, select one out of two: size or funds. Size refers to the amount of trading targets (the asset name written in front) for the trading pair. The Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
          "cancelAfter": {
            "type": "integer",
            "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
            "format": "int64",
            "default": -1,
            "minimum": 0,
            "exclusiveMinimum": 0,
            "maximum": 2592000,
            "exclusiveMaximum": 2592000
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
          "tags": {
            "type": "string",
            "description": "Order tag, length cannot exceed 20 characters (ASCII)"
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
          },
          "funds": {
            "type": "string",
            "description": "When **type** is market, select one out of two: size or funds"
          },
          "allowMaxTimeWindow": {
            "type": "integer",
            "description": "Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.",
            "format": "int64",
            "example": [10, 20, 30]
          },
          "clientTimestamp": {
            "type": "integer",
            "description": "Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.",
            "format": "int64",
            "example": [1740711735178]
          }
        },
        "required": ["type", "symbol", "side", "price"]
      },
      "description": "Order List"
    }
  },
  "required": ["orderList"]
}
```

<h3 id="batch-add-orders-sync-parameters">Parameters</h3>

| Name                  | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| --------------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body                  | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » orderList           | body | [object]       | true     | Order List                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» clientOid          | body | string         | false    | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| »» symbol             | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| »» type               | body | string         | true     | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» timeInForce        | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                          |
| »» side               | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| »» price              | body | string         | true     | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| »» size               | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| »» stp                | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                       |
| »» cancelAfter        | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                             |
| »» postOnly           | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| »» hidden             | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| »» iceberg            | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| »» visibleSize        | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| »» tags               | body | string         | false    | Order tag, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» remark             | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                            |
| »» funds              | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| »» allowMaxTimeWindow | body | integer(int64) | false    | Order failed after timeout of specified milliseconds, If clientTimestamp + allowMaxTimeWindow < Gateway received the message time, this order will fail.                                                                                                                                                                                                                                                                                                                       |
| »» clientTimestamp    | body | integer(int64) | false    | Equal to KC-API-TIMESTAMP. Needs to be defined if allowMaxTimeWindow is specified.                                                                                                                                                                                                                                                                                                                                                                                             |

#### Detailed descriptions

**»» size**: Specify quantity for order.

When **type** is limited, select one out of two: size or funds. Size refers to
the amount of trading targets (the asset name written in front) for the trading
pair. The Size must be based on the baseIncrement of the trading pair. The
baseIncrement represents the precision for the trading pair. The size of an
order must be a positive-integer multiple of baseIncrement and must be between
baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter      | Value |
| -------------- | ----- |
| »» type        | limit |
| »» timeInForce | GTC   |
| »» timeInForce | GTT   |
| »» timeInForce | IOC   |
| »» timeInForce | FOK   |
| »» side        | buy   |
| »» side        | sell  |
| »» stp         | DC    |
| »» stp         | CO    |
| »» stp         | CN    |
| »» stp         | CB    |

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
            "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
          },
          "clientOid": {
            "type": "string",
            "description": "The user self-defined order ID."
          },
          "orderTime": {
            "type": "integer",
            "format": "int64"
          },
          "originSize": {
            "type": "string",
            "description": "Original order size"
          },
          "dealSize": {
            "type": "string",
            "description": "Deal size"
          },
          "remainSize": {
            "type": "string",
            "description": "Remain size"
          },
          "canceledSize": {
            "type": "string",
            "description": "Cumulative canceled size"
          },
          "status": {
            "type": "string",
            "description": "Order Status. open: order is active; done: order has been completed",
            "enum": ["open", "done"],
            "x-api-enum": [
              {
                "value": "open",
                "name": "",
                "description": ""
              },
              {
                "value": "done",
                "name": "",
                "description": ""
              }
            ]
          },
          "matchTime": {
            "type": "integer",
            "format": "int64"
          },
          "success": {
            "type": "boolean",
            "description": "Add order success/failure"
          },
          "failMsg": {
            "type": "string",
            "description": "Error message"
          }
        },
        "required": ["success"]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-add-orders-sync-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-add-orders-sync-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type           | Required | Restrictions | Description                                                                                                                   |
| --------------- | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code          | string         | true     | none         | none                                                                                                                          |
| » data          | [object]       | true     | none         | none                                                                                                                          |
| »» orderId      | string         | false    | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» clientOid    | string         | false    | none         | The user self-defined order ID.                                                                                               |
| »» orderTime    | integer(int64) | false    | none         | none                                                                                                                          |
| »» originSize   | string         | false    | none         | Original order size                                                                                                           |
| »» dealSize     | string         | false    | none         | Deal size                                                                                                                     |
| »» remainSize   | string         | false    | none         | Remain size                                                                                                                   |
| »» canceledSize | string         | false    | none         | Cumulative canceled size                                                                                                      |
| »» status       | string         | false    | none         | Order Status. open: order is active; done: order has been completed                                                           |
| »» matchTime    | integer(int64) | false    | none         | none                                                                                                                          |
| »» success      | boolean        | true     | none         | Add order success/failure                                                                                                     |
| »» failMsg      | string         | false    | none         | Error message                                                                                                                 |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | open  |
| status   | done  |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By OrderId

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.delete('/api/v1/hf/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/{orderId}`

This endpoint can be used to cancel a spot order by orderId. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-orderid-parameters">Parameters</h3>

| Name    | In    | Type   | Required | Description                                         |
| ------- | ----- | ------ | -------- | --------------------------------------------------- |
| orderId | path  | string | true     | The unique order id generated by the trading system |
| symbol  | query | string | true     | symbol                                              |

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

<a id="opId014"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/hf/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/{orderId}`

This endpoint can be used to obtain information for a single Spot order using
the order id. After the user successfully places an order, the order is in
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
          "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
          "description": "Order price"
        },
        "size": {
          "type": "string",
          "description": "Order size"
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
          "description": "currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)",
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
          "description": "Whether its a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether its a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether its a iceberg order."
        },
        "visibleSize": {
          "type": "string",
          "description": "Visible size of iceberg order in order book."
        },
        "cancelAfter": {
          "type": "integer",
          "description": "A GTT timeInForce that expires in n seconds",
          "format": "int64"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string",
          "description": "Client Order Id，unique identifier created by the user"
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
          "description": "Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook"
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
          "description": "Users in some regions need query this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order isactive; false-The status of the order is done"
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

| Name              | Type           | Required | Restrictions | Description                                                                                |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------ |
| » code            | string         | true     | none         | none                                                                                       |
| » data            | object         | true     | none         | none                                                                                       |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                        |
| »» symbol         | string         | true     | none         | symbol                                                                                     |
| »» opType         | string         | true     | none         | none                                                                                       |
| »» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                |
| »» price          | string         | true     | none         | Order price                                                                                |
| »» size           | string         | true     | none         | Order size                                                                                 |
| »» funds          | string         | true     | none         | Order Funds                                                                                |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                              |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                               |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                               |
| »» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                     |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)                       |
| »» timeInForce    | string         | true     | none         | Time in force                                                                              |
| »» postOnly       | boolean        | true     | none         | Whether its a postOnly order.                                                              |
| »» hidden         | boolean        | true     | none         | Whether its a hidden order.                                                                |
| »» iceberg        | boolean        | true     | none         | Whether its a iceberg order.                                                               |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                               |
| »» cancelAfter    | integer(int64) | true     | none         | A GTT timeInForce that expires in n seconds                                                |
| »» channel        | string         | true     | none         | none                                                                                       |
| »» clientOid      | string         | true     | none         | Client Order Id，unique identifier created by the user                                     |
| »» remark         | string         | false    | none         | Order placement remarks                                                                    |
| »» tags           | string         | false    | none         | Order tag                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                      |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                       |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                       |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                               |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook  |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                            |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                             |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                              |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                               |
| »» tax            | string         | true     | none         | Users in some regions need query this field                                                |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order isactive; false-The status of the order is done |

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

## Cancel Order By OrderId Sync

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/sync/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.delete('/api/v1/hf/orders/sync/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/sync/{orderId}`

This endpoint can be used to cancel a spot order by orderId.

<h3 id="cancel-order-by-orderid-sync-parameters">Parameters</h3>

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
          "description": "order id"
        },
        "originSize": {
          "type": "string",
          "description": "original order size"
        },
        "dealSize": {
          "type": "string",
          "description": "deal size"
        },
        "remainSize": {
          "type": "string",
          "description": "remain size"
        },
        "canceledSize": {
          "type": "string",
          "description": "Cumulative canceled size"
        },
        "status": {
          "type": "string",
          "description": "Order Status. open：order is active; done：order has been completed",
          "enum": ["open", "done"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "open",
              "description": "order is active"
            },
            {
              "value": "done",
              "name": "done",
              "description": "order has been completed"
            }
          ]
        }
      },
      "required": [
        "orderId",
        "dealSize",
        "remainSize",
        "canceledSize",
        "status",
        "originSize"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-orderid-sync-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-orderid-sync-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type   | Required | Restrictions | Description                                                         |
| --------------- | ------ | -------- | ------------ | ------------------------------------------------------------------- |
| » code          | string | true     | none         | none                                                                |
| » data          | object | true     | none         | none                                                                |
| »» orderId      | string | true     | none         | order id                                                            |
| »» originSize   | string | true     | none         | original order size                                                 |
| »» dealSize     | string | true     | none         | deal size                                                           |
| »» remainSize   | string | true     | none         | remain size                                                         |
| »» canceledSize | string | true     | none         | Cumulative canceled size                                            |
| »» status       | string | true     | none         | Order Status. open：order is active; done：order has been completed |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | open  |
| status   | done  |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By ClientOid

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/hf/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
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

r = requests.delete('/api/v1/hf/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/client-order/{clientOid}`

This endpoint can be used to cancel a spot order by clientOid. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to websocket.

<h3 id="cancel-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| clientOid | path  | string | true     | Client Order Id，unique identifier created by the user |
| symbol    | query | string | true     | symbol                                                 |

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
          "description": "Client Order Id，unique identifier created by the user"
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
| »» clientOid | string | true     | none         | Client Order Id，unique identifier created by the user |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order By ClientOid

<a id="opId015"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/hf/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
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

r = requests.get('/api/v1/hf/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/client-order/{clientOid}`

This endpoint can be used to obtain information for a single Spot order using
the client order id. After the user successfully places an order, the order is
in Active state, and the user can use inOrderBook to determine whether the order
has entered the order. Canceled or fully filled orders are marked as completed
Done status.

<h3 id="get-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id，unique identifier created by the user |

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
          "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
          "description": "Order price"
        },
        "size": {
          "type": "string",
          "description": "Order size"
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
          "description": "currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)",
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
          "description": "Whether its a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether its a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether its a iceberg order."
        },
        "visibleSize": {
          "type": "string",
          "description": "Visible size of iceberg order in order book."
        },
        "cancelAfter": {
          "type": "integer",
          "description": "A GTT timeInForce that expires in n seconds",
          "format": "int64"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string",
          "description": "Client Order Id，unique identifier created by the user"
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
          "description": "Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook"
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
          "description": "Users in some regions need query this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order isactive; false-The status of the order is done"
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

| Name              | Type           | Required | Restrictions | Description                                                                                |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------ |
| » code            | string         | true     | none         | none                                                                                       |
| » data            | object         | true     | none         | none                                                                                       |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                        |
| »» symbol         | string         | true     | none         | symbol                                                                                     |
| »» opType         | string         | true     | none         | none                                                                                       |
| »» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                |
| »» price          | string         | true     | none         | Order price                                                                                |
| »» size           | string         | true     | none         | Order size                                                                                 |
| »» funds          | string         | true     | none         | Order Funds                                                                                |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                              |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                               |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                               |
| »» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                     |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)                       |
| »» timeInForce    | string         | true     | none         | Time in force                                                                              |
| »» postOnly       | boolean        | true     | none         | Whether its a postOnly order.                                                              |
| »» hidden         | boolean        | true     | none         | Whether its a hidden order.                                                                |
| »» iceberg        | boolean        | true     | none         | Whether its a iceberg order.                                                               |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                               |
| »» cancelAfter    | integer(int64) | true     | none         | A GTT timeInForce that expires in n seconds                                                |
| »» channel        | string         | true     | none         | none                                                                                       |
| »» clientOid      | string         | true     | none         | Client Order Id，unique identifier created by the user                                     |
| »» remark         | string         | false    | none         | Order placement remarks                                                                    |
| »» tags           | string         | false    | none         | Order tag                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                      |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                       |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                       |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                               |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook  |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                            |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                             |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                              |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                               |
| »» tax            | string         | true     | none         | Users in some regions need query this field                                                |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order isactive; false-The status of the order is done |

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

## Cancel Order By ClientOid Sync

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/hf/orders/sync/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
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

r = requests.delete('/api/v1/hf/orders/sync/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/sync/client-order/{clientOid}`

This endpoint can be used to cancel a spot order by orderId.

<h3 id="cancel-order-by-clientoid-sync-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id，unique identifier created by the user |

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
          "description": "Client Order Id，unique identifier created by the user"
        },
        "originSize": {
          "type": "string",
          "description": "original order size"
        },
        "dealSize": {
          "type": "string",
          "description": "deal size"
        },
        "remainSize": {
          "type": "string",
          "description": "remain size"
        },
        "canceledSize": {
          "type": "string",
          "description": "Cumulative canceled size"
        },
        "status": {
          "type": "string",
          "description": "Order Status. open：order is active; done：order has been completed",
          "enum": ["open", "done"],
          "x-api-enum": [
            {
              "value": "open",
              "name": "open",
              "description": "order is active"
            },
            {
              "value": "done",
              "name": "done",
              "description": "order has been completed"
            }
          ]
        }
      },
      "required": [
        "clientOid",
        "originSize",
        "dealSize",
        "remainSize",
        "canceledSize",
        "status"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-clientoid-sync-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-clientoid-sync-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type   | Required | Restrictions | Description                                                         |
| --------------- | ------ | -------- | ------------ | ------------------------------------------------------------------- |
| » code          | string | true     | none         | none                                                                |
| » data          | object | true     | none         | none                                                                |
| »» clientOid    | string | true     | none         | Client Order Id，unique identifier created by the user              |
| »» originSize   | string | true     | none         | original order size                                                 |
| »» dealSize     | string | true     | none         | deal size                                                           |
| »» remainSize   | string | true     | none         | remain size                                                         |
| »» canceledSize | string | true     | none         | Cumulative canceled size                                            |
| »» status       | string | true     | none         | Order Status. open：order is active; done：order has been completed |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | open  |
| status   | done  |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Partial Order

<a id="opId010"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v1/hf/orders/cancel/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT&cancelSize=type,string",
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

r = requests.delete('/api/v1/hf/orders/cancel/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'cancelSize': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/cancel/{orderId}`

This interface can cancel the specified quantity of the order according to the
orderId. The order execution order is: price first, time first, this interface
will not change the queue order

<h3 id="cancel-partial-order-parameters">Parameters</h3>

| Name       | In    | Type   | Required | Description                                         |
| ---------- | ----- | ------ | -------- | --------------------------------------------------- |
| orderId    | path  | string | true     | The unique order id generated by the trading system |
| symbol     | query | string | true     | symbol                                              |
| cancelSize | query | string | true     | The size you want cancel                            |

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
          "description": "order id"
        },
        "cancelSize": {
          "type": "string",
          "description": "The size you canceled"
        }
      },
      "required": ["orderId", "cancelSize"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-partial-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-partial-order-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type   | Required | Restrictions | Description           |
| ------------- | ------ | -------- | ------------ | --------------------- |
| » code        | string | true     | none         | none                  |
| » data        | object | true     | none         | none                  |
| »» orderId    | string | true     | none         | order id              |
| »» cancelSize | string | true     | none         | The size you canceled |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel All Orders

<a id="opId012"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/cancelAll", {
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

r = requests.delete('/api/v1/hf/orders/cancelAll', headers = headers)

print(r.json())

```

`DELETE /api/v1/hf/orders/cancelAll`

This endpoint can cancel all spot orders for all symbol. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to websocket.

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
        "succeedSymbols": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The Symbols Successfully cancelled"
        },
        "failedSymbols": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string",
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "error": {
                "type": "string",
                "description": "error message"
              }
            }
          },
          "description": "The Symbols Failed to cancel"
        }
      },
      "required": ["succeedSymbols", "failedSymbols"]
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

| Name              | Type     | Required | Restrictions | Description                        |
| ----------------- | -------- | -------- | ------------ | ---------------------------------- |
| » code            | string   | true     | none         | none                               |
| » data            | object   | true     | none         | none                               |
| »» succeedSymbols | [string] | true     | none         | The Symbols Successfully cancelled |
| »» failedSymbols  | [object] | true     | none         | The Symbols Failed to cancel       |
| »»» symbol        | string   | false    | none         | symbol                             |
| »»» error         | string   | false    | none         | error message                      |

<aside class="success">
This operation does not require authentication
</aside>

## Modify Order

<a id="opId013"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "One must be chose out of the old client order ID, orderId and clientOid",
      "example": [
        "5c52e11203aa677f33e493fb"
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
    "orderId": {
      "type": "string",
      "description": "One must be chosen out of the old order id, orderId and clientOid"
    },
    "newPrice": {
      "type": "string",
      "description": "One must be chosen out of the modified price of the new order, newPrice and newSize"
    },
    "newSize": {
      "type": "string",
      "description": "One must be chosen out of the modified size of the new order, newPrice and newSize"
    }
  },
  "required": [
    "symbol"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/alter',
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

r = requests.post('/api/v1/hf/orders/alter', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/alter`

This interface can modify the price and quantity of the order according to
orderId or clientOid. The implementation of this interface is: Cancel the order
and place a new order on the same trading pair, and return the modification
result to the client synchronously. When the quantity of the new order updated
by the user is less than the filled quantity of this order, the order will be
considered as completed, and the order will be canceled, and no new order will
be placed.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "One must be chose out of the old client order ID, orderId and clientOid",
      "example": ["5c52e11203aa677f33e493fb"]
    },
    "symbol": {
      "type": "string",
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "orderId": {
      "type": "string",
      "description": "One must be chosen out of the old order id, orderId and clientOid"
    },
    "newPrice": {
      "type": "string",
      "description": "One must be chosen out of the modified price of the new order, newPrice and newSize"
    },
    "newSize": {
      "type": "string",
      "description": "One must be chosen out of the modified size of the new order, newPrice and newSize"
    }
  },
  "required": ["symbol"]
}
```

<h3 id="modify-order-parameters">Parameters</h3>

| Name        | In   | Type   | Required | Description                                                                         |
| ----------- | ---- | ------ | -------- | ----------------------------------------------------------------------------------- |
| body        | body | object | false    | none                                                                                |
| » clientOid | body | string | false    | One must be chose out of the old client order ID, orderId and clientOid             |
| » symbol    | body | string | true     | symbol                                                                              |
| » orderId   | body | string | false    | One must be chosen out of the old order id, orderId and clientOid                   |
| » newPrice  | body | string | false    | One must be chosen out of the modified price of the new order, newPrice and newSize |
| » newSize   | body | string | false    | One must be chosen out of the modified size of the new order, newPrice and newSize  |

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
        "newOrderId": {
          "type": "string",
          "description": "The new order ID"
        },
        "clientOid": {
          "type": "string",
          "description": "The original client order ID"
        }
      },
      "required": ["newOrderId", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="modify-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="modify-order-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type   | Required | Restrictions | Description                  |
| ------------- | ------ | -------- | ------------ | ---------------------------- |
| » code        | string | true     | none         | none                         |
| » data        | object | true     | none         | none                         |
| »» newOrderId | string | true     | none         | The new order ID             |
| »» clientOid  | string | true     | none         | The original client order ID |

<aside class="success">
This operation does not require authentication
</aside>

## Get Symbols With Open Order

<a id="opId016"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/active/symbols", {
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

r = requests.get('/api/v1/hf/orders/active/symbols', headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/active/symbols`

This interface can query all spot symbol that has active orders

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
        "symbols": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The symbol that has active orders"
        }
      },
      "required": ["symbols"]
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

| Name       | Type     | Required | Restrictions | Description                       |
| ---------- | -------- | -------- | ------------ | --------------------------------- |
| » code     | string   | true     | none         | none                              |
| » data     | object   | true     | none         | none                              |
| »» symbols | [string] | true     | none         | The symbol that has active orders |

<aside class="success">
This operation does not require authentication
</aside>

## Get Open Orders

<a id="opId017"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/active?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/hf/orders/active', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/active`

This interface is to obtain all Spot active order lists, and the return value of
the active order interface is the paged data of all uncompleted order lists. The
returned data is sorted in descending order according to the latest update time
of the order. After the user successfully places an order, the order is in
Active state, and the user can use inOrderBook to determine whether the order
has entered the order. Canceled or fully filled orders are marked as completed
Done status.

<h3 id="get-open-orders-parameters">Parameters</h3>

| Name   | In    | Type   | Required | Description |
| ------ | ----- | ------ | -------- | ----------- |
| symbol | query | string | true     | symbol      |

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
            "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
            "description": "Order price"
          },
          "size": {
            "type": "string",
            "description": "Order size"
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
            "description": "currency used to calculate trading fee"
          },
          "stp": {
            "type": "string",
            "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)",
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
            "description": "Whether its a postOnly order."
          },
          "hidden": {
            "type": "boolean",
            "description": "Whether its a hidden order."
          },
          "iceberg": {
            "type": "boolean",
            "description": "Whether its a iceberg order."
          },
          "visibleSize": {
            "type": "string",
            "description": "Visible size of iceberg order in order book."
          },
          "cancelAfter": {
            "type": "integer",
            "description": "A GTT timeInForce that expires in n seconds",
            "format": "int64"
          },
          "channel": {
            "type": "string"
          },
          "clientOid": {
            "type": "string",
            "description": "Client Order Id，unique identifier created by the user"
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
            "description": "Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook"
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
            "description": "Users in some regions need query this field"
          },
          "active": {
            "type": "boolean",
            "description": "Order status: true-The status of the order isactive; false-The status of the order is done"
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

| Name              | Type           | Required | Restrictions | Description                                                                                |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------ |
| » code            | string         | true     | none         | none                                                                                       |
| » data            | [object]       | true     | none         | none                                                                                       |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                        |
| »» symbol         | string         | true     | none         | symbol                                                                                     |
| »» opType         | string         | true     | none         | none                                                                                       |
| »» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                |
| »» price          | string         | true     | none         | Order price                                                                                |
| »» size           | string         | true     | none         | Order size                                                                                 |
| »» funds          | string         | true     | none         | Order Funds                                                                                |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                              |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                               |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                               |
| »» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                     |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)                       |
| »» timeInForce    | string         | true     | none         | Time in force                                                                              |
| »» postOnly       | boolean        | true     | none         | Whether its a postOnly order.                                                              |
| »» hidden         | boolean        | true     | none         | Whether its a hidden order.                                                                |
| »» iceberg        | boolean        | true     | none         | Whether its a iceberg order.                                                               |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                               |
| »» cancelAfter    | integer(int64) | true     | none         | A GTT timeInForce that expires in n seconds                                                |
| »» channel        | string         | true     | none         | none                                                                                       |
| »» clientOid      | string         | true     | none         | Client Order Id，unique identifier created by the user                                     |
| »» remark         | string         | false    | none         | Order placement remarks                                                                    |
| »» tags           | string         | false    | none         | Order tag                                                                                  |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                      |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                       |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                       |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                               |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook  |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                            |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                             |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                              |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                               |
| »» tax            | string         | true     | none         | Users in some regions need query this field                                                |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order isactive; false-The status of the order is done |

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

## Get Open Orders By Page

<a id="opId018"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/active/page?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/hf/orders/active/page', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/active/page`

This interface is to obtain Spot active order (uncompleted order) lists by page.
The returned data is sorted in descending order according to the create time of
the order. After the user successfully places an order, the order is in Active
state, and the user can use inOrderBook to determine whether the order has
entered the order. Canceled or fully filled orders are marked as completed Done
status.

<h3 id="get-open-orders-by-page-parameters">Parameters</h3>

| Name     | In    | Type    | Required | Description   |
| -------- | ----- | ------- | -------- | ------------- |
| symbol   | query | string  | true     | Symbol        |
| pageNum  | query | integer | false    | Current page  |
| pageSize | query | integer | false    | Size per page |

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
                "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
                "description": "Order price"
              },
              "size": {
                "type": "string",
                "description": "Order size"
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
                "description": "currency used to calculate trading fee"
              },
              "stp": {
                "type": "string",
                "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)",
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
                "description": "Whether its a postOnly order."
              },
              "hidden": {
                "type": "boolean",
                "description": "Whether its a hidden order."
              },
              "iceberg": {
                "type": "boolean",
                "description": "Whether its a iceberg order."
              },
              "visibleSize": {
                "type": "string",
                "description": "Visible size of iceberg order in order book."
              },
              "cancelAfter": {
                "type": "integer",
                "description": "A GTT timeInForce that expires in n seconds",
                "format": "int64"
              },
              "channel": {
                "type": "string"
              },
              "clientOid": {
                "type": "string",
                "description": "Client Order Id，unique identifier created by the user"
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
                "description": "Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook"
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
                "description": "Users in some regions need query this field"
              },
              "active": {
                "type": "boolean",
                "description": "Order status: true-The status of the order isactive; false-The status of the order is done"
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
      "required": ["currentPage", "pageSize", "totalNum", "totalPage", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-open-orders-by-page-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-open-orders-by-page-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                |
| ------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------ |
| » code             | string         | true     | none         | none                                                                                       |
| » data             | object         | true     | none         | none                                                                                       |
| »» currentPage     | integer        | true     | none         | none                                                                                       |
| »» pageSize        | integer        | true     | none         | none                                                                                       |
| »» totalNum        | integer        | true     | none         | none                                                                                       |
| »» totalPage       | integer        | true     | none         | none                                                                                       |
| »» items           | [object]       | true     | none         | none                                                                                       |
| »»» id             | string         | true     | none         | The unique order id generated by the trading system                                        |
| »»» symbol         | string         | true     | none         | symbol                                                                                     |
| »»» opType         | string         | true     | none         | none                                                                                       |
| »»» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                |
| »»» side           | string         | true     | none         | Buy or sell                                                                                |
| »»» price          | string         | true     | none         | Order price                                                                                |
| »»» size           | string         | true     | none         | Order size                                                                                 |
| »»» funds          | string         | true     | none         | Order Funds                                                                                |
| »»» dealSize       | string         | true     | none         | Number of filled transactions                                                              |
| »»» dealFunds      | string         | true     | none         | Funds of filled transactions                                                               |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                               |
| »»» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                     |
| »»» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)                       |
| »»» timeInForce    | string         | true     | none         | Time in force                                                                              |
| »»» postOnly       | boolean        | true     | none         | Whether its a postOnly order.                                                              |
| »»» hidden         | boolean        | true     | none         | Whether its a hidden order.                                                                |
| »»» iceberg        | boolean        | true     | none         | Whether its a iceberg order.                                                               |
| »»» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                               |
| »»» cancelAfter    | integer(int64) | true     | none         | A GTT timeInForce that expires in n seconds                                                |
| »»» channel        | string         | true     | none         | none                                                                                       |
| »»» clientOid      | string         | true     | none         | Client Order Id，unique identifier created by the user                                     |
| »»» remark         | string         | false    | none         | Order placement remarks                                                                    |
| »»» tags           | string         | false    | none         | Order tag                                                                                  |
| »»» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                      |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                       |
| »»» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                       |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                               |
| »»» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook  |
| »»» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                            |
| »»» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                             |
| »»» remainSize     | string         | true     | none         | Number of remain transactions                                                              |
| »»» remainFunds    | string         | true     | none         | Funds of remain transactions                                                               |
| »»» tax            | string         | true     | none         | Users in some regions need query this field                                                |
| »»» active         | boolean        | true     | none         | Order status: true-The status of the order isactive; false-The status of the order is done |

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

<a id="opId019"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/done?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/hf/orders/done', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/done`

This interface is to obtain all Spot closed order lists, and the return value of
the active order interface is the paged data of all uncompleted order lists. The
returned data is sorted in descending order according to the latest update time
of the order. After the user successfully places an order, the order is in
Active state, and the user can use inOrderBook to determine whether the order
has entered the order. Canceled or fully filled orders are marked as completed
Done status.

<h3 id="get-closed-orders-parameters">Parameters</h3>

| Name    | In    | Type           | Required | Description                                                                                                  |
| ------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| symbol  | query | string         | true     | symbol                                                                                                       |
| side    | query | string         | false    | specify if the order is to 'buy' or 'sell'                                                                   |
| type    | query | string         | false    | specify if the order is an 'limit' order or 'market' order.                                                  |
| lastId  | query | integer(int64) | false    | The id of the last set of data from the previous batch of data. By default, the latest information is given. |
| limit   | query | integer        | false    | Default20，Max100                                                                                            |
| startAt | query | integer(int64) | false    | Start time (milisecond)                                                                                      |
| endAt   | query | integer(int64) | false    | End time (milisecond)                                                                                        |

#### Detailed descriptions

**lastId**: The id of the last set of data from the previous batch of data. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId，which can be used as a query
parameter to look up new data from the next page.

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
        "lastId": {
          "type": "integer",
          "description": "The id of the last set of data from the previous batch of data. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId，which can be used as a query parameter to look up new data from the next page.",
          "format": "int64",
          "minimum": 0,
          "exclusiveMinimum": 0
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
                "description": "Specify if the order is an 'limit' order or 'market' order. ",
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
                "description": "Order price"
              },
              "size": {
                "type": "string",
                "description": "Order size"
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
                "description": "currency used to calculate trading fee"
              },
              "stp": {
                "type": "string",
                "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)",
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
                "description": "Whether its a postOnly order."
              },
              "hidden": {
                "type": "boolean",
                "description": "Whether its a hidden order."
              },
              "iceberg": {
                "type": "boolean",
                "description": "Whether its a iceberg order."
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
                "description": "Client Order Id，unique identifier created by the user"
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
                "description": "Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook"
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
                "description": "Users in some regions need query this field"
              },
              "active": {
                "type": "boolean",
                "description": "Order status: true-The status of the order isactive; false-The status of the order is done"
              }
            },
            "required": [
              "feeCurrency",
              "id",
              "symbol",
              "opType",
              "type",
              "side",
              "price",
              "size",
              "funds",
              "dealSize",
              "fee",
              "timeInForce",
              "dealFunds",
              "clientOid",
              "remainSize",
              "remainFunds",
              "cancelledSize",
              "cancelledFunds",
              "lastUpdatedAt",
              "createdAt",
              "tax",
              "active",
              "cancelExist",
              "tradeType",
              "inOrderBook",
              "postOnly",
              "hidden",
              "iceberg",
              "visibleSize",
              "cancelAfter",
              "channel"
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

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                                  |
| ------------------ | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »» lastId          | integer(int64) | true     | none         | The id of the last set of data from the previous batch of data. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId，which can be used as a query parameter to look up new data from the next page. |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» id             | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                          |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                       |
| »»» opType         | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» type           | string         | true     | none         | Specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                  |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                                  |
| »»» price          | string         | true     | none         | Order price                                                                                                                                                                                                                                                                                                                                                  |
| »»» size           | string         | true     | none         | Order size                                                                                                                                                                                                                                                                                                                                                   |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                                  |
| »»» dealSize       | string         | true     | none         | Number of filled transactions                                                                                                                                                                                                                                                                                                                                |
| »»» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                                                                                                                                                                                                                                                                 |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                                 |
| »»» feeCurrency    | string         | true     | none         | currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                       |
| »»» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570)                                                                                                                                                                                                                                                                                         |
| »»» timeInForce    | string         | true     | none         | Time in force                                                                                                                                                                                                                                                                                                                                                |
| »»» postOnly       | boolean        | true     | none         | Whether its a postOnly order.                                                                                                                                                                                                                                                                                                                                |
| »»» hidden         | boolean        | true     | none         | Whether its a hidden order.                                                                                                                                                                                                                                                                                                                                  |
| »»» iceberg        | boolean        | true     | none         | Whether its a iceberg order.                                                                                                                                                                                                                                                                                                                                 |
| »»» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                                                                                                                                                                                                                                                                 |
| »»» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                                                                                                                                                                                                                                                                  |
| »»» channel        | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» clientOid      | string         | true     | none         | Client Order Id，unique identifier created by the user                                                                                                                                                                                                                                                                                                       |
| »»» remark         | string         | false    | none         | Order placement remarks                                                                                                                                                                                                                                                                                                                                      |
| »»» tags           | string         | false    | none         | Order tag                                                                                                                                                                                                                                                                                                                                                    |
| »»» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                                                                                                                                                                                                                                                        |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                         |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                                 |
| »»» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: true: enter the orderbook; false: not enter the orderbook                                                                                                                                                                                                                                                                    |
| »»» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                                                                                                                                                                                                                                                              |
| »»» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                                                                                                                                                                                                                                                               |
| »»» remainSize     | string         | true     | none         | Number of remain transactions                                                                                                                                                                                                                                                                                                                                |
| »»» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                                                                                                                                                                                                                                                                 |
| »»» tax            | string         | true     | none         | Users in some regions need query this field                                                                                                                                                                                                                                                                                                                  |
| »»» active         | boolean        | true     | none         | Order status: true-The status of the order isactive; false-The status of the order is done                                                                                                                                                                                                                                                                   |

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

## Get DCP

<a id="opId021"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/hf/orders/dead-cancel-all/query", {
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

r = requests.get('/api/v1/hf/orders/dead-cancel-all/query', headers = headers)

print(r.json())

```

`GET /api/v1/hf/orders/dead-cancel-all/query`

Get Disconnection Protect (Deadman Switch). Through this interface, you can
query the settings of automatic order cancellation.

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
        "timeout": {
          "type": "integer",
          "description": "Auto cancel order trigger setting time, the unit is second. Range: timeout=-1 (meaning unset) or 5 <= timeout <= 86400"
        },
        "symbols": {
          "type": "string",
          "description": "List of trading pairs. Separated by commas; empty means all trading pairs"
        },
        "currentTime": {
          "type": "integer",
          "description": "System current time (in seconds)",
          "format": "int64"
        },
        "triggerTime": {
          "type": "integer",
          "description": "Trigger cancellation time (in seconds)",
          "format": "int64"
        }
      },
      "description": "If the data is empty, it means that DCP is not set.",
      "additionalProperties": false
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-dcp-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-dcp-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                                                                                            |
| -------------- | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------- |
| » code         | string         | true     | none         | none                                                                                                                   |
| » data         | object         | true     | none         | If the data is empty, it means that DCP is not set.                                                                    |
| »» timeout     | integer        | false    | none         | Auto cancel order trigger setting time, the unit is second. Range: timeout=-1 (meaning unset) or 5 <= timeout <= 86400 |
| »» symbols     | string         | false    | none         | List of trading pairs. Separated by commas; empty means all trading pairs                                              |
| »» currentTime | integer(int64) | false    | none         | System current time (in seconds)                                                                                       |
| »» triggerTime | integer(int64) | false    | none         | Trigger cancellation time (in seconds)                                                                                 |

<aside class="success">
This operation does not require authentication
</aside>

## Set DCP

<a id="opId022"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "timeout": {
      "type": "integer",
      "description": "Auto cancel order trigger setting time, the unit is second. Range: timeout=-1 (meaning unset) or 5 <= timeout <= 86400. For example, timeout=5 means that the order will be automatically canceled if no user request is received for more than 5 seconds. When this parameter is changed, the previous setting will be overwritten."
    },
    "symbols": {
      "type": "string",
      "description": "List of trading pairs. When this parameter is not empty, separate it with commas and support up to 50 trading pairs. Empty means all trading pairs. When this parameter is changed, the previous setting will be overwritten.",
      "example": [
        "BTC-USDT,ETH-USDT"
      ]
    }
  },
  "required": [
    "timeout"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/hf/orders/dead-cancel-all',
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

r = requests.post('/api/v1/hf/orders/dead-cancel-all', headers = headers)

print(r.json())

```

`POST /api/v1/hf/orders/dead-cancel-all`

Set Disconnection Protect (Deadman Switch). Through this interface, call this
interface to automatically cancel all orders of the set trading pair after the
specified time. If this interface is not called again for renewal or
cancellation before the set time, the system will help the user to cancel the
order of the corresponding trading pair. Otherwise it will not.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "timeout": {
      "type": "integer",
      "description": "Auto cancel order trigger setting time, the unit is second. Range: timeout=-1 (meaning unset) or 5 <= timeout <= 86400. For example, timeout=5 means that the order will be automatically canceled if no user request is received for more than 5 seconds. When this parameter is changed, the previous setting will be overwritten."
    },
    "symbols": {
      "type": "string",
      "description": "List of trading pairs. When this parameter is not empty, separate it with commas and support up to 50 trading pairs. Empty means all trading pairs. When this parameter is changed, the previous setting will be overwritten.",
      "example": ["BTC-USDT,ETH-USDT"]
    }
  },
  "required": ["timeout"]
}
```

<h3 id="set-dcp-parameters">Parameters</h3>

| Name      | In   | Type    | Required | Description                                                                                                                                                                                                                                                                                                                          |
| --------- | ---- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body      | body | object  | false    | none                                                                                                                                                                                                                                                                                                                                 |
| » timeout | body | integer | true     | Auto cancel order trigger setting time, the unit is second. Range: timeout=-1 (meaning unset) or 5 <= timeout <= 86400. For example, timeout=5 means that the order will be automatically canceled if no user request is received for more than 5 seconds. When this parameter is changed, the previous setting will be overwritten. |
| » symbols | body | string  | false    | List of trading pairs. When this parameter is not empty, separate it with commas and support up to 50 trading pairs. Empty means all trading pairs. When this parameter is changed, the previous setting will be overwritten.                                                                                                        |

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
        "currentTime": {
          "type": "integer",
          "description": "System current time (in seconds)",
          "format": "int64"
        },
        "triggerTime": {
          "type": "integer",
          "description": "Trigger cancellation time (in seconds)",
          "format": "int64"
        }
      },
      "required": ["currentTime", "triggerTime"],
      "description": "If the data is empty, it means that DCP is not set."
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="set-dcp-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="set-dcp-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                         |
| -------------- | -------------- | -------- | ------------ | --------------------------------------------------- |
| » code         | string         | true     | none         | none                                                |
| » data         | object         | true     | none         | If the data is empty, it means that DCP is not set. |
| »» currentTime | integer(int64) | true     | none         | System current time (in seconds)                    |
| »» triggerTime | integer(int64) | true     | none         | Trigger cancellation time (in seconds)              |

<aside class="success">
This operation does not require authentication
</aside>

## Add Stop Order

<a id="opId023"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
      "enum": [
        "limit",
        "market"
      ],
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
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify price for order, not need for market order.\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading. Required for limit orders.",
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
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK if **type** is limit.",
      "default": false
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
      "description": "When **type** is limit, this is Maximum visible quantity in iceberg orders."
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "stopPrice": {
      "type": "string",
      "description": "The trigger price."
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). Default is TRADE"
    }
  },
  "required": [
    "symbol",
    "type",
    "side",
    "stopPrice"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v1/stop-order',
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

r = requests.post('/api/v1/stop-order', headers = headers)

print(r.json())

```

`POST /api/v1/stop-order`

Place stop order to the Spot trading system, you can place two major types of
orders: limit and market. Orders can only be placed if your account has
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
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ],
      "example": ["limit"]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify price for order, not need for market order.\n\nWhen placing a limit order, the price must be based on priceIncrement for the trading pair. The price increment (priceIncrement) is the price precision for the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement is 0.00001000. So the price for your orders cannot be less than 0.00001000 and must be a multiple of priceIncrement. Otherwise, the order will return an invalid priceIncrement error."
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
    },
    "timeInForce": {
      "type": "string",
      "description": "[Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading. Required for limit orders.",
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
      "description": "passive order labels, this is disabled when the order timing strategy is IOC or FOK if **type** is limit.",
      "default": false
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
      "description": "When **type** is limit, this is Maximum visible quantity in iceberg orders."
    },
    "cancelAfter": {
      "type": "integer",
      "description": "Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1\n",
      "format": "int64",
      "default": -1,
      "minimum": 0,
      "exclusiveMinimum": 0,
      "maximum": 2592000,
      "exclusiveMaximum": 2592000
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "stopPrice": {
      "type": "string",
      "description": "The trigger price."
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). Default is TRADE"
    }
  },
  "required": ["symbol", "type", "side", "stopPrice"]
}
```

<h3 id="add-stop-order-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » clientOid   | body | string         | false    | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » type        | body | string         | true     | specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » remark      | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                            |
| » price       | body | string         | false    | Specify price for order, not need for market order.                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » size        | body | string         | false    | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading. Required for limit orders.                                                                                                                                                                                                                                                                                                                                                   |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK if **type** is limit.                                                                                                                                                                                                                                                                                                                                                                           |
| » hidden      | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                         |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                            |
| » visibleSize | body | string         | false    | When **type** is limit, this is Maximum visible quantity in iceberg orders.                                                                                                                                                                                                                                                                                                                                                                                                         |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT, -1 means it will not be cancelled automatically, the default value is -1                                                                                                                                                                                                                                                                                                                                                  |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » stopPrice   | body | string         | true     | The trigger price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » tradeType   | body | string         | false    | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). Default is TRADE                                                                                                                                                                                                                                                                                                                                                         |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: specify if the order is an 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price, you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order, not need for market order.

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
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
          "description": "The unique order id generated by the trading system,which can be used later for further actions such as canceling the order."
        }
      },
      "required": ["orderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-stop-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-stop-order-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description                                                                                                                  |
| ---------- | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code     | string | true     | none         | none                                                                                                                         |
| » data     | object | true     | none         | none                                                                                                                         |
| »» orderId | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |

<aside class="success">
This operation does not require authentication
</aside>

## Get Stop Orders List

<a id="opId027"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order", {
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

r = requests.get('/api/v1/stop-order', headers = headers)

print(r.json())

```

`GET /api/v1/stop-order`

This interface is to obtain all Spot active stop order lists

<h3 id="get-stop-orders-list-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                 |
| ----------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------------------------- |
| symbol      | query | string         | false    | Only list orders for a specific symbol                                                                                      |
| side        | query | string         | false    | buy or sell                                                                                                                 |
| type        | query | string         | false    | limit, market                                                                                                               |
| tradeType   | query | string         | false    | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). Default is TRADE |
| startAt     | query | integer(int64) | false    | Start time (milisecond)                                                                                                     |
| endAt       | query | integer(int64) | false    | End time (milisecond)                                                                                                       |
| currentPage | query | integer        | false    | Current page                                                                                                                |
| orderIds    | query | string         | false    | Comma seperated order ID list                                                                                               |
| pageSize    | query | integer        | false    | Page size                                                                                                                   |
| stop        | query | string         | false    | Order type: stop: stop loss order, oco: oco order                                                                           |

#### Detailed descriptions

**currentPage**: Current page

#### Enumerated Values

| Parameter | Value  |
| --------- | ------ |
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
          "description": "current page id"
        },
        "pageSize": {
          "type": "integer"
        },
        "totalNum": {
          "type": "integer",
          "description": "the stop order count"
        },
        "totalPage": {
          "type": "integer",
          "description": "total page count of the list"
        },
        "items": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string",
                "description": "Order ID, the ID of an order."
              },
              "symbol": {
                "type": "string",
                "description": "Symbol name"
              },
              "userId": {
                "type": "string",
                "description": "User ID"
              },
              "status": {
                "type": "string",
                "description": "Order status, include NEW, TRIGGERED"
              },
              "type": {
                "type": "string",
                "description": "Order type",
                "enum": ["limit", "market"],
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
                  }
                ]
              },
              "side": {
                "type": "string",
                "description": "transaction direction,include buy and sell"
              },
              "price": {
                "type": "string",
                "description": "order price"
              },
              "size": {
                "type": "string",
                "description": "order quantity"
              },
              "funds": {
                "type": "string",
                "description": "order funds"
              },
              "stp": {
                "type": "string"
              },
              "timeInForce": {
                "type": "string",
                "description": "time InForce,include GTC,GTT,IOC,FOK"
              },
              "cancelAfter": {
                "type": "integer",
                "description": "cancel orders after n seconds，requires timeInForce to be GTT"
              },
              "postOnly": {
                "type": "boolean",
                "description": "postOnly"
              },
              "hidden": {
                "type": "boolean",
                "description": "hidden order"
              },
              "iceberg": {
                "type": "boolean",
                "description": "Iceberg order"
              },
              "visibleSize": {
                "type": "string",
                "description": "displayed quantity for iceberg order"
              },
              "channel": {
                "type": "string",
                "description": "order source"
              },
              "clientOid": {
                "type": "string",
                "description": "user-entered order unique mark"
              },
              "remark": {
                "type": "string",
                "description": "Remarks at stop order creation"
              },
              "tags": {
                "type": "string",
                "description": "tag order source"
              },
              "orderTime": {
                "type": "integer",
                "description": "Time of place a stop order, accurate to nanoseconds",
                "format": "int64"
              },
              "domainId": {
                "type": "string",
                "description": "domainId, e.g: kucoin"
              },
              "tradeSource": {
                "type": "string",
                "description": "trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）"
              },
              "tradeType": {
                "type": "string",
                "description": "The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin)."
              },
              "feeCurrency": {
                "type": "string",
                "description": "The currency of the fee"
              },
              "takerFeeRate": {
                "type": "string",
                "description": "Fee Rate of taker"
              },
              "makerFeeRate": {
                "type": "string",
                "description": "Fee Rate of maker"
              },
              "createdAt": {
                "type": "integer",
                "description": "order creation time",
                "format": "int64"
              },
              "stop": {
                "type": "string",
                "description": "Stop order type, include loss and entry"
              },
              "stopTriggerTime": {
                "type": "integer",
                "description": "The trigger time of the stop order",
                "format": "int64"
              },
              "stopPrice": {
                "type": "string",
                "description": "stop price"
              },
              "relatedNo": {
                "type": "string"
              },
              "limitPrice": {
                "type": "string"
              },
              "pop": {
                "type": "string"
              },
              "activateCondition": {
                "type": "string"
              }
            },
            "required": [
              "id",
              "symbol",
              "userId",
              "status",
              "type",
              "side",
              "price",
              "size",
              "timeInForce",
              "cancelAfter",
              "postOnly",
              "hidden",
              "iceberg",
              "channel",
              "clientOid",
              "remark",
              "orderTime",
              "domainId",
              "tradeSource",
              "tradeType",
              "feeCurrency",
              "takerFeeRate",
              "makerFeeRate",
              "createdAt",
              "stop",
              "stopPrice"
            ]
          },
          "description": "the list of stop orders"
        }
      },
      "required": ["currentPage", "pageSize", "totalNum", "totalPage", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-stop-orders-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-stop-orders-list-responseschema">Response Schema</h3>

Status Code **200**

| Name                  | Type           | Required | Restrictions | Description                                                                                                |
| --------------------- | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------- |
| » code                | string         | true     | none         | none                                                                                                       |
| » data                | object         | true     | none         | none                                                                                                       |
| »» currentPage        | integer        | true     | none         | current page id                                                                                            |
| »» pageSize           | integer        | true     | none         | none                                                                                                       |
| »» totalNum           | integer        | true     | none         | the stop order count                                                                                       |
| »» totalPage          | integer        | true     | none         | total page count of the list                                                                               |
| »» items              | [object]       | true     | none         | the list of stop orders                                                                                    |
| »»» id                | string         | true     | none         | Order ID, the ID of an order.                                                                              |
| »»» symbol            | string         | true     | none         | Symbol name                                                                                                |
| »»» userId            | string         | true     | none         | User ID                                                                                                    |
| »»» status            | string         | true     | none         | Order status, include NEW, TRIGGERED                                                                       |
| »»» type              | string         | true     | none         | Order type                                                                                                 |
| »»» side              | string         | true     | none         | transaction direction,include buy and sell                                                                 |
| »»» price             | string         | true     | none         | order price                                                                                                |
| »»» size              | string         | true     | none         | order quantity                                                                                             |
| »»» funds             | string         | false    | none         | order funds                                                                                                |
| »»» stp               | string         | false    | none         | none                                                                                                       |
| »»» timeInForce       | string         | true     | none         | time InForce,include GTC,GTT,IOC,FOK                                                                       |
| »»» cancelAfter       | integer        | true     | none         | cancel orders after n seconds，requires timeInForce to be GTT                                              |
| »»» postOnly          | boolean        | true     | none         | postOnly                                                                                                   |
| »»» hidden            | boolean        | true     | none         | hidden order                                                                                               |
| »»» iceberg           | boolean        | true     | none         | Iceberg order                                                                                              |
| »»» visibleSize       | string         | false    | none         | displayed quantity for iceberg order                                                                       |
| »»» channel           | string         | true     | none         | order source                                                                                               |
| »»» clientOid         | string         | true     | none         | user-entered order unique mark                                                                             |
| »»» remark            | string         | true     | none         | Remarks at stop order creation                                                                             |
| »»» tags              | string         | false    | none         | tag order source                                                                                           |
| »»» orderTime         | integer(int64) | true     | none         | Time of place a stop order, accurate to nanoseconds                                                        |
| »»» domainId          | string         | true     | none         | domainId, e.g: kucoin                                                                                      |
| »»» tradeSource       | string         | true     | none         | trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）                               |
| »»» tradeType         | string         | true     | none         | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). |
| »»» feeCurrency       | string         | true     | none         | The currency of the fee                                                                                    |
| »»» takerFeeRate      | string         | true     | none         | Fee Rate of taker                                                                                          |
| »»» makerFeeRate      | string         | true     | none         | Fee Rate of maker                                                                                          |
| »»» createdAt         | integer(int64) | true     | none         | order creation time                                                                                        |
| »»» stop              | string         | true     | none         | Stop order type, include loss and entry                                                                    |
| »»» stopTriggerTime   | integer(int64) | false    | none         | The trigger time of the stop order                                                                         |
| »»» stopPrice         | string         | true     | none         | stop price                                                                                                 |
| »»» relatedNo         | string         | false    | none         | none                                                                                                       |
| »»» limitPrice        | string         | false    | none         | none                                                                                                       |
| »»» pop               | string         | false    | none         | none                                                                                                       |
| »»» activateCondition | string         | false    | none         | none                                                                                                       |

#### Enumerated Values

| Property | Value  |
| -------- | ------ |
| type     | limit  |
| type     | market |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Stop Order By ClientOid

<a id="opId024"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order/cancelOrderByClientOid?clientOid=type,string", {
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

r = requests.delete('/api/v1/stop-order/cancelOrderByClientOid', params={
  'clientOid': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v1/stop-order/cancelOrderByClientOid`

This endpoint can be used to cancel a spot stop order by clientOid.

<h3 id="cancel-stop-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                               |
| --------- | ----- | ------ | -------- | --------------------------------------------------------- |
| symbol    | query | string | false    | symbol                                                    |
| clientOid | query | string | true     | Unique order id created by users to identify their orders |

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
          "description": "Client Order Id，unique identifier created by the user"
        },
        "cancelledOrderId": {
          "type": "string",
          "description": "Unique ID of the cancelled order"
        }
      },
      "required": ["clientOid", "cancelledOrderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-stop-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-stop-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type   | Required | Restrictions | Description                                            |
| ------------------- | ------ | -------- | ------------ | ------------------------------------------------------ |
| » code              | string | true     | none         | none                                                   |
| » data              | object | true     | none         | none                                                   |
| »» clientOid        | string | true     | none         | Client Order Id，unique identifier created by the user |
| »» cancelledOrderId | string | true     | none         | Unique ID of the cancelled order                       |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Stop Order By OrderId

<a id="opId025"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order/{orderId}", {
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

r = requests.delete('/api/v1/stop-order/{orderId}', headers = headers)

print(r.json())

```

`DELETE /api/v1/stop-order/{orderId}`

This endpoint can be used to cancel a spot stop order by orderId.

<h3 id="cancel-stop-order-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
          "description": "order ID array"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-stop-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-stop-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description    |
| -------------------- | -------- | -------- | ------------ | -------------- |
| » code               | string   | true     | none         | none           |
| » data               | object   | true     | none         | none           |
| »» cancelledOrderIds | [string] | true     | none         | order ID array |

<aside class="success">
This operation does not require authentication
</aside>

## Get Stop Order By OrderId

<a id="opId028"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order/{orderId}", {
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

r = requests.get('/api/v1/stop-order/{orderId}', headers = headers)

print(r.json())

```

`GET /api/v1/stop-order/{orderId}`

This interface is to obtain Spot stop order details by orderId

<h3 id="get-stop-order-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description  |
| ------- | ---- | ------ | -------- | ------------ |
| orderId | path | string | true     | The order id |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "data": {
      "type": "object",
      "properties": {
        "id": {
          "type": "string",
          "description": "Order ID, the ID of an order."
        },
        "symbol": {
          "type": "string",
          "description": "Symbol name"
        },
        "userId": {
          "type": "string",
          "description": "User ID"
        },
        "status": {
          "type": "string",
          "description": "Order status, include NEW, TRIGGERED"
        },
        "type": {
          "type": "string",
          "description": "Order type",
          "enum": ["limit", "market"],
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
            }
          ]
        },
        "side": {
          "type": "string",
          "description": "transaction direction,include buy and sell"
        },
        "price": {
          "type": "string",
          "description": "order price"
        },
        "size": {
          "type": "string",
          "description": "order quantity"
        },
        "funds": {
          "type": "string",
          "description": "order funds"
        },
        "stp": {
          "type": "string"
        },
        "timeInForce": {
          "type": "string",
          "description": "time InForce,include GTC,GTT,IOC,FOK"
        },
        "cancelAfter": {
          "type": "integer",
          "description": "cancel orders after n seconds，requires timeInForce to be GTT",
          "format": "int64"
        },
        "postOnly": {
          "type": "boolean",
          "description": "postOnly"
        },
        "hidden": {
          "type": "boolean",
          "description": "hidden order"
        },
        "iceberg": {
          "type": "boolean",
          "description": "Iceberg order"
        },
        "visibleSize": {
          "type": "string",
          "description": "displayed quantity for iceberg order"
        },
        "channel": {
          "type": "string",
          "description": "order source"
        },
        "clientOid": {
          "type": "string",
          "description": "user-entered order unique mark"
        },
        "remark": {
          "type": "string",
          "description": "Remarks at stop order creation"
        },
        "tags": {
          "type": "string",
          "description": "tag order source"
        },
        "domainId": {
          "type": "string",
          "description": "domainId, e.g: kucoin"
        },
        "tradeSource": {
          "type": "string",
          "description": "trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）"
        },
        "tradeType": {
          "type": "string",
          "description": "The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin)."
        },
        "feeCurrency": {
          "type": "string",
          "description": "The currency of the fee"
        },
        "takerFeeRate": {
          "type": "string",
          "description": "Fee Rate of taker"
        },
        "makerFeeRate": {
          "type": "string",
          "description": "Fee Rate of maker"
        },
        "createdAt": {
          "type": "integer",
          "description": "order creation time",
          "format": "int64"
        },
        "stop": {
          "type": "string",
          "description": "Stop order type, include loss and entry"
        },
        "stopTriggerTime": {
          "type": "integer",
          "description": "The trigger time of the stop order",
          "format": "int64"
        },
        "stopPrice": {
          "type": "string",
          "description": "stop price"
        },
        "orderTime": {
          "type": "integer",
          "description": "Time of place a stop order, accurate to nanoseconds",
          "format": "int64"
        }
      },
      "description": "the details"
    },
    "code": {
      "type": "string",
      "description": "return status code"
    }
  },
  "required": ["data", "code"]
}
```

<h3 id="get-stop-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-stop-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                |
| ------------------ | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------- |
| » data             | object         | true     | none         | the details                                                                                                |
| »» id              | string         | false    | none         | Order ID, the ID of an order.                                                                              |
| »» symbol          | string         | false    | none         | Symbol name                                                                                                |
| »» userId          | string         | false    | none         | User ID                                                                                                    |
| »» status          | string         | false    | none         | Order status, include NEW, TRIGGERED                                                                       |
| »» type            | string         | false    | none         | Order type                                                                                                 |
| »» side            | string         | false    | none         | transaction direction,include buy and sell                                                                 |
| »» price           | string         | false    | none         | order price                                                                                                |
| »» size            | string         | false    | none         | order quantity                                                                                             |
| »» funds           | string         | false    | none         | order funds                                                                                                |
| »» stp             | string         | false    | none         | none                                                                                                       |
| »» timeInForce     | string         | false    | none         | time InForce,include GTC,GTT,IOC,FOK                                                                       |
| »» cancelAfter     | integer(int64) | false    | none         | cancel orders after n seconds，requires timeInForce to be GTT                                              |
| »» postOnly        | boolean        | false    | none         | postOnly                                                                                                   |
| »» hidden          | boolean        | false    | none         | hidden order                                                                                               |
| »» iceberg         | boolean        | false    | none         | Iceberg order                                                                                              |
| »» visibleSize     | string         | false    | none         | displayed quantity for iceberg order                                                                       |
| »» channel         | string         | false    | none         | order source                                                                                               |
| »» clientOid       | string         | false    | none         | user-entered order unique mark                                                                             |
| »» remark          | string         | false    | none         | Remarks at stop order creation                                                                             |
| »» tags            | string         | false    | none         | tag order source                                                                                           |
| »» domainId        | string         | false    | none         | domainId, e.g: kucoin                                                                                      |
| »» tradeSource     | string         | false    | none         | trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）                               |
| »» tradeType       | string         | false    | none         | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). |
| »» feeCurrency     | string         | false    | none         | The currency of the fee                                                                                    |
| »» takerFeeRate    | string         | false    | none         | Fee Rate of taker                                                                                          |
| »» makerFeeRate    | string         | false    | none         | Fee Rate of maker                                                                                          |
| »» createdAt       | integer(int64) | false    | none         | order creation time                                                                                        |
| »» stop            | string         | false    | none         | Stop order type, include loss and entry                                                                    |
| »» stopTriggerTime | integer(int64) | false    | none         | The trigger time of the stop order                                                                         |
| »» stopPrice       | string         | false    | none         | stop price                                                                                                 |
| »» orderTime       | integer(int64) | false    | none         | Time of place a stop order, accurate to nanoseconds                                                        |
| » code             | string         | true     | none         | return status code                                                                                         |

#### Enumerated Values

| Property | Value  |
| -------- | ------ |
| type     | limit  |
| type     | market |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Cancel Stop Orders

<a id="opId026"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order/cancel", {
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

r = requests.delete('/api/v1/stop-order/cancel', headers = headers)

print(r.json())

```

`DELETE /api/v1/stop-order/cancel`

This endpoint can be used to cancel a spot stop orders by batch.

<h3 id="batch-cancel-stop-orders-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                                |
| --------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------- |
| symbol    | query | string | false    | Cancel the open order for the specified symbol                                                             |
| tradeType | query | string | false    | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). |
| orderIds  | query | string | false    | Comma seperated order IDs.                                                                                 |

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
          "description": "order id array"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-cancel-stop-orders-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-cancel-stop-orders-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description    |
| -------------------- | -------- | -------- | ------------ | -------------- |
| » code               | string   | true     | none         | none           |
| » data               | object   | true     | none         | none           |
| »» cancelledOrderIds | [string] | true     | none         | order id array |

<aside class="success">
This operation does not require authentication
</aside>

## Get Stop Order By ClientOid

<a id="opId029"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/stop-order/queryOrderByClientOid?clientOid=type,string", {
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

r = requests.get('/api/v1/stop-order/queryOrderByClientOid', params={
  'clientOid': {
  "type": "string"
}
}, headers = headers)

print(r.json())

```

`GET /api/v1/stop-order/queryOrderByClientOid`

This interface is to obtain Spot stop order details by orderId

<h3 id="get-stop-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description         |
| --------- | ----- | ------ | -------- | ------------------- |
| clientOid | query | string | true     | The client order id |
| symbol    | query | string | false    | symbol name         |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "string",
      "description": "the return code"
    },
    "data": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "Order ID, the ID of an order."
          },
          "symbol": {
            "type": "string",
            "description": "Symbol name"
          },
          "userId": {
            "type": "string",
            "description": "User ID"
          },
          "status": {
            "type": "string",
            "description": "Order status, include NEW, TRIGGERED"
          },
          "type": {
            "type": "string",
            "description": "Order type",
            "enum": ["limit", "market"],
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
              }
            ]
          },
          "side": {
            "type": "string",
            "description": "transaction direction,include buy and sell"
          },
          "price": {
            "type": "string",
            "description": "order price"
          },
          "size": {
            "type": "string",
            "description": "order quantity"
          },
          "funds": {
            "type": "string",
            "description": "order funds"
          },
          "stp": {
            "type": "string"
          },
          "timeInForce": {
            "type": "string",
            "description": "time InForce,include GTC,GTT,IOC,FOK"
          },
          "cancelAfter": {
            "type": "integer",
            "description": "cancel orders after n seconds，requires timeInForce to be GTT",
            "format": "int64"
          },
          "postOnly": {
            "type": "boolean",
            "description": "postOnly"
          },
          "hidden": {
            "type": "boolean",
            "description": "hidden order"
          },
          "iceberg": {
            "type": "boolean",
            "description": "Iceberg order"
          },
          "visibleSize": {
            "type": "string",
            "description": "displayed quantity for iceberg order"
          },
          "channel": {
            "type": "string",
            "description": "order source"
          },
          "clientOid": {
            "type": "string",
            "description": "user-entered order unique mark"
          },
          "remark": {
            "type": "string",
            "description": "Remarks at stop order creation"
          },
          "tags": {
            "type": "string",
            "description": "tag order source"
          },
          "domainId": {
            "type": "string",
            "description": "domainId, e.g: kucoin"
          },
          "tradeSource": {
            "type": "string",
            "description": "trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）"
          },
          "tradeType": {
            "type": "string",
            "description": "The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin)."
          },
          "feeCurrency": {
            "type": "string",
            "description": "The currency of the fee"
          },
          "takerFeeRate": {
            "type": "string",
            "description": "Fee Rate of taker"
          },
          "makerFeeRate": {
            "type": "string",
            "description": "Fee Rate of maker"
          },
          "createdAt": {
            "type": "integer",
            "description": "order creation time",
            "format": "int64"
          },
          "stop": {
            "type": "string",
            "description": "Stop order type, include loss and entry"
          },
          "stopTriggerTime": {
            "type": "integer",
            "description": "The trigger time of the stop order",
            "format": "int64"
          },
          "stopPrice": {
            "type": "string",
            "description": "stop price"
          },
          "orderTime": {
            "type": "integer",
            "description": "Time of place a stop order, accurate to nanoseconds",
            "format": "int64"
          }
        }
      },
      "description": "the return code"
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-stop-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-stop-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                |
| ------------------ | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | the return code                                                                                            |
| » data             | [object]       | true     | none         | the return code                                                                                            |
| »» id              | string         | false    | none         | Order ID, the ID of an order.                                                                              |
| »» symbol          | string         | false    | none         | Symbol name                                                                                                |
| »» userId          | string         | false    | none         | User ID                                                                                                    |
| »» status          | string         | false    | none         | Order status, include NEW, TRIGGERED                                                                       |
| »» type            | string         | false    | none         | Order type                                                                                                 |
| »» side            | string         | false    | none         | transaction direction,include buy and sell                                                                 |
| »» price           | string         | false    | none         | order price                                                                                                |
| »» size            | string         | false    | none         | order quantity                                                                                             |
| »» funds           | string         | false    | none         | order funds                                                                                                |
| »» stp             | string         | false    | none         | none                                                                                                       |
| »» timeInForce     | string         | false    | none         | time InForce,include GTC,GTT,IOC,FOK                                                                       |
| »» cancelAfter     | integer(int64) | false    | none         | cancel orders after n seconds，requires timeInForce to be GTT                                              |
| »» postOnly        | boolean        | false    | none         | postOnly                                                                                                   |
| »» hidden          | boolean        | false    | none         | hidden order                                                                                               |
| »» iceberg         | boolean        | false    | none         | Iceberg order                                                                                              |
| »» visibleSize     | string         | false    | none         | displayed quantity for iceberg order                                                                       |
| »» channel         | string         | false    | none         | order source                                                                                               |
| »» clientOid       | string         | false    | none         | user-entered order unique mark                                                                             |
| »» remark          | string         | false    | none         | Remarks at stop order creation                                                                             |
| »» tags            | string         | false    | none         | tag order source                                                                                           |
| »» domainId        | string         | false    | none         | domainId, e.g: kucoin                                                                                      |
| »» tradeSource     | string         | false    | none         | trade source: USER（Order by user）, MARGIN_SYSTEM（Order by margin system）                               |
| »» tradeType       | string         | false    | none         | The type of trading : TRADE（Spot）, MARGIN_TRADE (Cross Margin), MARGIN_ISOLATED_TRADE (Isolated Margin). |
| »» feeCurrency     | string         | false    | none         | The currency of the fee                                                                                    |
| »» takerFeeRate    | string         | false    | none         | Fee Rate of taker                                                                                          |
| »» makerFeeRate    | string         | false    | none         | Fee Rate of maker                                                                                          |
| »» createdAt       | integer(int64) | false    | none         | order creation time                                                                                        |
| »» stop            | string         | false    | none         | Stop order type, include loss and entry                                                                    |
| »» stopTriggerTime | integer(int64) | false    | none         | The trigger time of the stop order                                                                         |
| »» stopPrice       | string         | false    | none         | stop price                                                                                                 |
| »» orderTime       | integer(int64) | false    | none         | Time of place a stop order, accurate to nanoseconds                                                        |

#### Enumerated Values

| Property | Value  |
| -------- | ------ |
| type     | limit  |
| type     | market |

<aside class="success">
This operation does not require authentication
</aside>

## Add OCO Order

<a id="opId030"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
    },
    "price": {
      "type": "string",
      "description": "Specify price for order"
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order"
    },
    "stopPrice": {
      "type": "string",
      "description": "trigger price."
    },
    "limitPrice": {
      "type": "string",
      "description": "The limit order price after take-profit and stop-loss are triggered."
    },
    "tradeType": {
      "type": "string",
      "description": "Transaction Type, currently only supports TRADE (spot transactions), the default is TRADE",
      "enum": [
        "TRADE"
      ],
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot Trading"
        }
      ]
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid",
    "price",
    "size",
    "stopPrice",
    "limitPrice"
  ]
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json'
};

fetch('/api/v3/oco/order',
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

r = requests.post('/api/v3/oco/order', headers = headers)

print(r.json())

```

`POST /api/v3/oco/order`

Place OCO order to the Spot trading system

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
    },
    "price": {
      "type": "string",
      "description": "Specify price for order"
    },
    "size": {
      "type": "string",
      "description": "Specify quantity for order"
    },
    "stopPrice": {
      "type": "string",
      "description": "trigger price."
    },
    "limitPrice": {
      "type": "string",
      "description": "The limit order price after take-profit and stop-loss are triggered."
    },
    "tradeType": {
      "type": "string",
      "description": "Transaction Type, currently only supports TRADE (spot transactions), the default is TRADE",
      "enum": ["TRADE"],
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot Trading"
        }
      ]
    }
  },
  "required": [
    "symbol",
    "side",
    "clientOid",
    "price",
    "size",
    "stopPrice",
    "limitPrice"
  ]
}
```

<h3 id="add-oco-order-parameters">Parameters</h3>

| Name         | In   | Type   | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ------------ | ---- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body         | body | object | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » clientOid  | body | string | true     | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side       | body | string | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » symbol     | body | string | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » remark     | body | string | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » price      | body | string | true     | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » size       | body | string | true     | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| » stopPrice  | body | string | true     | trigger price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| » limitPrice | body | string | true     | The limit order price after take-profit and stop-loss are triggered.                                                                                                                                                                                                                                                                                                                                                                                                                |
| » tradeType  | body | string | false    | Transaction Type, currently only supports TRADE (spot transactions), the default is TRADE                                                                                                                                                                                                                                                                                                                                                                                           |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

#### Enumerated Values

| Parameter   | Value |
| ----------- | ----- |
| » side      | buy   |
| » side      | sell  |
| » tradeType | TRADE |

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
        }
      },
      "required": ["orderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-oco-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-oco-order-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description                                                                                                                  |
| ---------- | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code     | string | true     | none         | none                                                                                                                         |
| » data     | object | true     | none         | none                                                                                                                         |
| »» orderId | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel OCO Order By OrderId

<a id="opId031"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/order/{orderId}", {
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

r = requests.delete('/api/v3/oco/order/{orderId}', headers = headers)

print(r.json())

```

`DELETE /api/v3/oco/order/{orderId}`

This endpoint can be used to cancel a spot order by orderId. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to Websocket.

<h3 id="cancel-oco-order-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
          "description": "List of two order IDs related to the canceled OCO order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-oco-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-oco-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                                             |
| -------------------- | -------- | -------- | ------------ | ------------------------------------------------------- |
| » code               | string   | true     | none         | none                                                    |
| » data               | object   | true     | none         | none                                                    |
| »» cancelledOrderIds | [string] | true     | none         | List of two order IDs related to the canceled OCO order |

<aside class="success">
This operation does not require authentication
</aside>

## Get OCO Order By OrderId

<a id="opId034"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/order/{orderId}", {
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

r = requests.get('/api/v3/oco/order/{orderId}', headers = headers)

print(r.json())

```

`GET /api/v3/oco/order/{orderId}`

Request via this interface an OCO order information via the order ID.

<h3 id="get-oco-order-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
        "clientOid": {
          "type": "string",
          "description": "Client Order ID"
        },
        "orderId": {
          "type": "string",
          "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
        },
        "orderTime": {
          "type": "integer",
          "description": "Order placement time, milliseconds",
          "format": "int64"
        },
        "status": {
          "type": "string",
          "description": "Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELED: Canceled",
          "enum": ["NEW", "DONE", "TRIGGERED", "CANCELLED"],
          "x-api-enum": [
            {
              "value": "NEW",
              "name": "NEW",
              "description": "New"
            },
            {
              "value": "DONE",
              "name": "DONE",
              "description": "Completed"
            },
            {
              "value": "TRIGGERED",
              "name": "TRIGGERED",
              "description": "Triggered"
            },
            {
              "value": "CANCELLED",
              "name": "CANCELED",
              "description": "Canceled"
            }
          ]
        }
      },
      "required": ["orderId", "symbol", "clientOid", "orderTime", "status"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-oco-order-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-oco-order-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type           | Required | Restrictions | Description                                                                                                                   |
| ------------ | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code       | string         | true     | none         | none                                                                                                                          |
| » data       | object         | true     | none         | none                                                                                                                          |
| »» symbol    | string         | true     | none         | symbol                                                                                                                        |
| »» clientOid | string         | true     | none         | Client Order ID                                                                                                               |
| »» orderId   | string         | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» orderTime | integer(int64) | true     | none         | Order placement time, milliseconds                                                                                            |
| »» status    | string         | true     | none         | Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELED: Canceled                                             |

#### Enumerated Values

| Property | Value     |
| -------- | --------- |
| status   | NEW       |
| status   | DONE      |
| status   | TRIGGERED |
| status   | CANCELLED |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel OCO Order By ClientOid

<a id="opId032"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/client-order/{clientOid}", {
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

r = requests.delete('/api/v3/oco/client-order/{clientOid}', headers = headers)

print(r.json())

```

`DELETE /api/v3/oco/client-order/{clientOid}`

Request via this interface to cancel a stop order via the clientOid. You will
receive cancelledOrderIds field once the system has received the cancellation
request. The cancellation request will be processed by the matching engine in
sequence. To know if the request is processed (successfully or not), you may
check the order status or the update message from the pushes.

<h3 id="cancel-oco-order-by-clientoid-parameters">Parameters</h3>

| Name      | In   | Type   | Required | Description                                            |
| --------- | ---- | ------ | -------- | ------------------------------------------------------ |
| clientOid | path | string | true     | Client Order Id，unique identifier created by the user |

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
          "description": "List of two order IDs related to the canceled OCO order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-oco-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-oco-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                                             |
| -------------------- | -------- | -------- | ------------ | ------------------------------------------------------- |
| » code               | string   | true     | none         | none                                                    |
| » data               | object   | true     | none         | none                                                    |
| »» cancelledOrderIds | [string] | true     | none         | List of two order IDs related to the canceled OCO order |

<aside class="success">
This operation does not require authentication
</aside>

## Get OCO Order By ClientOid

<a id="opId035"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/client-order/{clientOid}", {
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

r = requests.get('/api/v3/oco/client-order/{clientOid}', headers = headers)

print(r.json())

```

`GET /api/v3/oco/client-order/{clientOid}`

Request via this interface to get a oco order information via the client order
ID.

<h3 id="get-oco-order-by-clientoid-parameters">Parameters</h3>

| Name      | In   | Type   | Required | Description                                            |
| --------- | ---- | ------ | -------- | ------------------------------------------------------ |
| clientOid | path | string | true     | Client Order Id，unique identifier created by the user |

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
        "clientOid": {
          "type": "string",
          "description": "Client Order Id"
        },
        "orderId": {
          "type": "string",
          "description": "The unique order id generated by the trading system,which can be used later for further actions such as canceling the order."
        },
        "orderTime": {
          "type": "integer",
          "description": "Order placement time, milliseconds",
          "format": "int64"
        },
        "status": {
          "type": "string",
          "description": "Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELLED: Cancelled"
        }
      },
      "required": ["orderId", "symbol", "clientOid", "orderTime", "status"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-oco-order-by-clientoid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-oco-order-by-clientoid-responseschema">Response Schema</h3>

Status Code **200**

| Name         | Type           | Required | Restrictions | Description                                                                                                                  |
| ------------ | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code       | string         | true     | none         | none                                                                                                                         |
| » data       | object         | true     | none         | none                                                                                                                         |
| »» symbol    | string         | true     | none         | symbol                                                                                                                       |
| »» clientOid | string         | true     | none         | Client Order Id                                                                                                              |
| »» orderId   | string         | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |
| »» orderTime | integer(int64) | true     | none         | Order placement time, milliseconds                                                                                           |
| »» status    | string         | true     | none         | Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELLED: Cancelled                                          |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Cancel OCO Order

<a id="opId033"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/orders", {
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

r = requests.delete('/api/v3/oco/orders', headers = headers)

print(r.json())

```

`DELETE /api/v3/oco/orders`

This interface can batch cancel OCO orders through orderIds. You will receive
canceledOrderIds field once the system has received the cancellation request.
The cancellation request will be processed by the matching engine in sequence.
To know if the request is processed (successfully or not), you may check the
order status or the update message from the pushes.

<h3 id="batch-cancel-oco-order-parameters">Parameters</h3>

| Name     | In    | Type   | Required | Description                                                                                                                         |
| -------- | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| orderIds | query | string | false    | Specify the order ID; there can be multiple orders, separated by commas. If not passed, all OCO orders will be canceled by default. |
| symbol   | query | string | false    | Trading pair. If not passed, the OCO orders of all symbols will be canceled by default.                                             |

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
          "description": "List of two order IDs related to the canceled OCO order"
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-cancel-oco-order-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-cancel-oco-order-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description                                             |
| -------------------- | -------- | -------- | ------------ | ------------------------------------------------------- |
| » code               | string   | true     | none         | none                                                    |
| » data               | object   | true     | none         | none                                                    |
| »» cancelledOrderIds | [string] | true     | none         | List of two order IDs related to the canceled OCO order |

<aside class="success">
This operation does not require authentication
</aside>

## Get OCO Order List

<a id="opId037"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/orders", {
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

r = requests.get('/api/v3/oco/orders', headers = headers)

print(r.json())

```

`GET /api/v3/oco/orders`

Request your current OCO order list via this endpoint. Items are paginated and
sorted to show the latest first. See the Pagination section for retrieving
additional entries after the first page.

<h3 id="get-oco-order-list-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                        |
| ----------- | ----- | -------------- | -------- | -------------------------------------------------- |
| symbol      | query | string         | false    | symbol                                             |
| startAt     | query | integer(int64) | false    | Start time (milliseconds)                          |
| endAt       | query | integer(int64) | false    | End time (milliseconds)                            |
| orderIds    | query | string         | false    | Specify orderId collection, up to 500 orders       |
| pageSize    | query | integer        | false    | Size per page, minimum value 10, maximum value 500 |
| currentPage | query | integer        | false    | Page number, minimum value 1                       |

#### Detailed descriptions

**orderIds**: Specify orderId collection, up to 500 orders

**currentPage**: Page number, minimum value 1

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
              "orderId": {
                "type": "string",
                "description": "The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order."
              },
              "symbol": {
                "type": "string",
                "description": "symbol"
              },
              "clientOid": {
                "type": "string",
                "description": "Client Order ID"
              },
              "orderTime": {
                "type": "integer",
                "description": "Order placement time, milliseconds",
                "format": "int64"
              },
              "status": {
                "type": "string",
                "description": "Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELED: Canceled",
                "enum": ["NEW", "DONE", "TRIGGERED", "CANCELLED"],
                "x-api-enum": [
                  {
                    "value": "NEW",
                    "name": "NEW",
                    "description": "New"
                  },
                  {
                    "value": "DONE",
                    "name": "DONE",
                    "description": "Completed"
                  },
                  {
                    "value": "TRIGGERED",
                    "name": "TRIGGERED",
                    "description": "Triggered"
                  },
                  {
                    "value": "CANCELLED",
                    "name": "CANCELED",
                    "description": "Canceled"
                  }
                ]
              }
            },
            "required": [
              "orderId",
              "symbol",
              "clientOid",
              "orderTime",
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

<h3 id="get-oco-order-list-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-oco-order-list-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type           | Required | Restrictions | Description                                                                                                                   |
| -------------- | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string         | true     | none         | none                                                                                                                          |
| » data         | object         | true     | none         | none                                                                                                                          |
| »» currentPage | integer        | true     | none         | none                                                                                                                          |
| »» pageSize    | integer        | true     | none         | none                                                                                                                          |
| »» totalNum    | integer        | true     | none         | none                                                                                                                          |
| »» totalPage   | integer        | true     | none         | none                                                                                                                          |
| »» items       | [object]       | true     | none         | none                                                                                                                          |
| »»» orderId    | string         | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »»» symbol     | string         | true     | none         | symbol                                                                                                                        |
| »»» clientOid  | string         | true     | none         | Client Order ID                                                                                                               |
| »»» orderTime  | integer(int64) | true     | none         | Order placement time, milliseconds                                                                                            |
| »»» status     | string         | true     | none         | Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELED: Canceled                                             |

#### Enumerated Values

| Property | Value     |
| -------- | --------- |
| status   | NEW       |
| status   | DONE      |
| status   | TRIGGERED |
| status   | CANCELLED |

<aside class="success">
This operation does not require authentication
</aside>

## Get OCO Order Detail By OrderId

<a id="opId036"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/oco/order/details/{orderId}", {
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

r = requests.get('/api/v3/oco/order/details/{orderId}', headers = headers)

print(r.json())

```

`GET /api/v3/oco/order/details/{orderId}`

Request via this interface to get a oco order detail via the order ID.

<h3 id="get-oco-order-detail-by-orderid-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
        "symbol": {
          "type": "string",
          "description": "symbol"
        },
        "clientOid": {
          "type": "string",
          "description": "Client Order Id"
        },
        "orderTime": {
          "type": "integer",
          "description": "Order placement time, milliseconds",
          "format": "int64"
        },
        "status": {
          "type": "string",
          "description": "Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELLED: Cancelled"
        },
        "orders": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "id": {
                "type": "string"
              },
              "symbol": {
                "type": "string"
              },
              "side": {
                "type": "string"
              },
              "price": {
                "type": "string"
              },
              "stopPrice": {
                "type": "string"
              },
              "size": {
                "type": "string"
              },
              "status": {
                "type": "string"
              }
            },
            "required": [
              "id",
              "symbol",
              "side",
              "price",
              "stopPrice",
              "size",
              "status"
            ]
          }
        }
      },
      "required": [
        "orderId",
        "symbol",
        "clientOid",
        "orderTime",
        "status",
        "orders"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-oco-order-detail-by-orderid-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-oco-order-detail-by-orderid-responseschema">Response Schema</h3>

Status Code **200**

| Name          | Type           | Required | Restrictions | Description                                                                                                                  |
| ------------- | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code        | string         | true     | none         | none                                                                                                                         |
| » data        | object         | true     | none         | none                                                                                                                         |
| »» orderId    | string         | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |
| »» symbol     | string         | true     | none         | symbol                                                                                                                       |
| »» clientOid  | string         | true     | none         | Client Order Id                                                                                                              |
| »» orderTime  | integer(int64) | true     | none         | Order placement time, milliseconds                                                                                           |
| »» status     | string         | true     | none         | Order status: NEW: New, DONE: Completed, TRIGGERED: Triggered, CANCELLED: Cancelled                                          |
| »» orders     | [object]       | true     | none         | none                                                                                                                         |
| »»» id        | string         | true     | none         | none                                                                                                                         |
| »»» symbol    | string         | true     | none         | none                                                                                                                         |
| »»» side      | string         | true     | none         | none                                                                                                                         |
| »»» price     | string         | true     | none         | none                                                                                                                         |
| »»» stopPrice | string         | true     | none         | none                                                                                                                         |
| »»» size      | string         | true     | none         | none                                                                                                                         |
| »»» status    | string         | true     | none         | none                                                                                                                         |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order - Old

<a id="opId038"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
      "description": "Cancel after n seconds，the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.**",
      "enum": [
        "TRADE",
        "MARGIN_TRADE"
      ],
      "default": "TRADE",
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot"
        },
        {
          "value": "MARGIN_TRADE",
          "name": "MARGIN_TRADE",
          "description": "Margin"
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

Place order to the Spot trading system, you can place two major types of orders:
limit and market. Orders can only be placed if your account has sufficient
funds. Once an order is placed, your funds will be put on hold for the duration
of the order. The amount of funds on hold depends on the order type and
parameters specified.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
      "description": "Cancel after n seconds，the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.**",
      "enum": ["TRADE", "MARGIN_TRADE"],
      "default": "TRADE",
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot"
        },
        {
          "value": "MARGIN_TRADE",
          "name": "MARGIN_TRADE",
          "description": "Margin"
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order---old-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------- | ---- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » clientOid   | body | string         | true     | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.                                                |
| » side        | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » type        | body | string         | false    | specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » remark      | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                                                                           |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| » size        | body | string         | false    | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds，the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| » tradeType   | body | string         | false    | The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.** |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: specify if the order is an 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price, you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value        |
| ------------- | ------------ |
| » side        | buy          |
| » side        | sell         |
| » type        | limit        |
| » type        | market       |
| » stp         | DC           |
| » stp         | CO           |
| » stp         | CN           |
| » stp         | CB           |
| » timeInForce | GTC          |
| » timeInForce | GTT          |
| » timeInForce | IOC          |
| » timeInForce | FOK          |
| » tradeType   | TRADE        |
| » tradeType   | MARGIN_TRADE |

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
        }
      },
      "required": ["orderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order---old-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description                                                                                                                  |
| ---------- | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code     | string | true     | none         | none                                                                                                                         |
| » data     | object | true     | none         | none                                                                                                                         |
| »» orderId | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Cancel Order - Old

<a id="opId043"></a>

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

Request via this endpoint to cancel all open orders. The response is a list of
ids of the canceled orders.

<h3 id="batch-cancel-order---old-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                                                                                                               |
| --------- | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol    | query | string | false    | symbol                                                                                                                                                                                    |
| tradeType | query | string | false    | The type of trading :TRADE(Spot Trading), MARGIN_TRADE(Cross Margin Trading), MARGIN_ISOLATED_TRADE(Isolated Margin Trading), and the default is TRADE to cancel the spot trading orders. |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | TRADE                 |
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
        "cancelledOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-cancel-order---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-cancel-order---old-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description |
| -------------------- | -------- | -------- | ------------ | ----------- |
| » code               | string   | true     | none         | none        |
| » data               | object   | true     | none         | none        |
| »» cancelledOrderIds | [string] | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Orders List - Old

<a id="opId044"></a>

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

Request your current order list via this endpoint. The return value is the data
after Pagination, sorted in descending order according to time.

<h3 id="get-orders-list---old-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                         |
| ----------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | query | string         | false    | Symbol                                                                                                                                              |
| status      | query | string         | false    | Active or done (done as default); only list orders with a specific status.                                                                          |
| side        | query | string         | false    | Buy or Sell                                                                                                                                         |
| type        | query | string         | false    | Order type                                                                                                                                          |
| tradeType   | query | string         | false    | The type of trading: TRADE - Spot Trading (TRADE as default), MARGIN_TRADE - Cross Margin Trading, MARGIN_ISOLATED_TRADE - Isolated Margin Trading. |
| startAt     | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                           |
| endAt       | query | integer(int64) | false    | End time (milliseconds)                                                                                                                             |
| currentPage | query | integer        | false    | Current request page.                                                                                                                               |
| pageSize    | query | integer        | false    | Number of results per request. Minimum is 10, maximum is 500.                                                                                       |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| status    | active                |
| status    | done                  |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |
| type      | limit_stop            |
| type      | market_stop           |
| type      | oco_limit             |
| type      | oco_stop              |
| tradeType | TRADE                 |
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
              "id": {
                "type": "string"
              },
              "symbol": {
                "type": "string"
              },
              "opType": {
                "type": "string"
              },
              "type": {
                "type": "string"
              },
              "side": {
                "type": "string"
              },
              "price": {
                "type": "string"
              },
              "size": {
                "type": "string"
              },
              "funds": {
                "type": "string"
              },
              "dealFunds": {
                "type": "string"
              },
              "dealSize": {
                "type": "string"
              },
              "fee": {
                "type": "string"
              },
              "feeCurrency": {
                "type": "string"
              },
              "stp": {
                "type": "string"
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
                "type": "string"
              },
              "postOnly": {
                "type": "boolean"
              },
              "hidden": {
                "type": "boolean"
              },
              "iceberg": {
                "type": "boolean"
              },
              "visibleSize": {
                "type": "string"
              },
              "cancelAfter": {
                "type": "integer"
              },
              "channel": {
                "type": "string"
              },
              "clientOid": {
                "type": "string"
              },
              "remark": {
                "type": "string"
              },
              "tags": {
                "type": "string"
              },
              "isActive": {
                "type": "boolean"
              },
              "cancelExist": {
                "type": "boolean"
              },
              "createdAt": {
                "type": "integer",
                "format": "int64"
              },
              "tradeType": {
                "type": "string"
              },
              "tax": {
                "type": "string"
              },
              "taxRate": {
                "type": "string"
              },
              "taxCurrency": {
                "type": "string"
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
              "dealFunds",
              "dealSize",
              "fee",
              "feeCurrency",
              "stp",
              "stop",
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
              "tags",
              "isActive",
              "cancelExist",
              "createdAt",
              "tradeType"
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

<h3 id="get-orders-list---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-orders-list---old-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description |
| ----------------- | -------------- | -------- | ------------ | ----------- |
| » code            | string         | true     | none         | none        |
| » data            | object         | true     | none         | none        |
| »» currentPage    | integer        | true     | none         | none        |
| »» pageSize       | integer        | true     | none         | none        |
| »» totalNum       | integer        | true     | none         | none        |
| »» totalPage      | integer        | true     | none         | none        |
| »» items          | [object]       | true     | none         | none        |
| »»» id            | string         | true     | none         | none        |
| »»» symbol        | string         | true     | none         | none        |
| »»» opType        | string         | true     | none         | none        |
| »»» type          | string         | true     | none         | none        |
| »»» side          | string         | true     | none         | none        |
| »»» price         | string         | true     | none         | none        |
| »»» size          | string         | true     | none         | none        |
| »»» funds         | string         | true     | none         | none        |
| »»» dealFunds     | string         | true     | none         | none        |
| »»» dealSize      | string         | true     | none         | none        |
| »»» fee           | string         | true     | none         | none        |
| »»» feeCurrency   | string         | true     | none         | none        |
| »»» stp           | string         | true     | none         | none        |
| »»» stop          | string         | true     | none         | none        |
| »»» stopTriggered | boolean        | true     | none         | none        |
| »»» stopPrice     | string         | true     | none         | none        |
| »»» timeInForce   | string         | true     | none         | none        |
| »»» postOnly      | boolean        | true     | none         | none        |
| »»» hidden        | boolean        | true     | none         | none        |
| »»» iceberg       | boolean        | true     | none         | none        |
| »»» visibleSize   | string         | true     | none         | none        |
| »»» cancelAfter   | integer        | true     | none         | none        |
| »»» channel       | string         | true     | none         | none        |
| »»» clientOid     | string         | true     | none         | none        |
| »»» remark        | string         | true     | none         | none        |
| »»» tags          | string         | true     | none         | none        |
| »»» isActive      | boolean        | true     | none         | none        |
| »»» cancelExist   | boolean        | true     | none         | none        |
| »»» createdAt     | integer(int64) | true     | none         | none        |
| »»» tradeType     | string         | true     | none         | none        |
| »»» tax           | string         | false    | none         | none        |
| »»» taxRate       | string         | false    | none         | none        |
| »»» taxCurrency   | string         | false    | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order Test - Old

<a id="opId039"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": [
        "BTC-USDT",
        "ETH-USDT",
        "KCS-USDT"
      ]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
      "description": "Cancel after n seconds，the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.**",
      "enum": [
        "TRADE",
        "MARGIN_TRADE"
      ],
      "default": "TRADE",
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot"
        },
        {
          "value": "MARGIN_TRADE",
          "name": "MARGIN_TRADE",
          "description": "Margin"
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

Order test endpoint, the request parameters and return parameters of this
endpoint are exactly the same as the order endpoint, and can be used to verify
whether the signature is correct and other operations. After placing an order,
the order will not enter the matching system, and the order cannot be queried.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "clientOid": {
      "type": "string",
      "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
      "description": "symbol",
      "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
    },
    "type": {
      "type": "string",
      "description": "specify if the order is an 'limit' order or 'market' order. \n\nThe type of order you specify when you place your order determines whether or not you need to request other parameters and also affects the execution of the matching engine.\n\nWhen placing a limit order, you must specify a price and size. The system will try to match the order according to market price or a price better than market price. If the order cannot be immediately matched, it will stay in the order book until it is matched or the user cancels.\n\nUnlike limit orders, the price for market orders fluctuates with market prices. When placing a market order, you do not need to specify a price, you only need to specify a quantity. Market orders are filled immediately and will not enter the order book. All market orders are takers and a taker fee will be charged.",
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
      ]
    },
    "remark": {
      "type": "string",
      "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
      "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
      "description": "Cancel after n seconds，the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "tradeType": {
      "type": "string",
      "description": "The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.**",
      "enum": ["TRADE", "MARGIN_TRADE"],
      "default": "TRADE",
      "x-api-enum": [
        {
          "value": "TRADE",
          "name": "TRADE",
          "description": "Spot"
        },
        {
          "value": "MARGIN_TRADE",
          "name": "MARGIN_TRADE",
          "description": "Margin"
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-test---old-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------- | ---- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » clientOid   | body | string         | true     | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.                                                |
| » side        | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| » type        | body | string         | false    | specify if the order is an 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » remark      | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                                                                           |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| » size        | body | string         | false    | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                                                                              |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds，the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| » tradeType   | body | string         | false    | The type of trading : **TRADE**（Spot Trade）, **MARGIN_TRADE** (Margin Trade). Default is **TRADE**. **Note: To improve the system performance and to accelerate order placing and processing, KuCoin has added a new interface for order placing of margin. For traders still using the current interface, please move to the new one as soon as possible. The current one will no longer accept margin orders by May 1st, 2021 (UTC). At the time, KuCoin will notify users via the announcement, please pay attention to it.** |

#### Detailed descriptions

**» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**» type**: specify if the order is an 'limit' order or 'market' order.

The type of order you specify when you place your order determines whether or
not you need to request other parameters and also affects the execution of the
matching engine.

When placing a limit order, you must specify a price and size. The system will
try to match the order according to market price or a price better than market
price. If the order cannot be immediately matched, it will stay in the order
book until it is matched or the user cancels.

Unlike limit orders, the price for market orders fluctuates with market prices.
When placing a market order, you do not need to specify a price, you only need
to specify a quantity. Market orders are filled immediately and will not enter
the order book. All market orders are takers and a taker fee will be charged.

**» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter     | Value        |
| ------------- | ------------ |
| » side        | buy          |
| » side        | sell         |
| » type        | limit        |
| » type        | market       |
| » stp         | DC           |
| » stp         | CO           |
| » stp         | CN           |
| » stp         | CB           |
| » timeInForce | GTC          |
| » timeInForce | GTT          |
| » timeInForce | IOC          |
| » timeInForce | FOK          |
| » tradeType   | TRADE        |
| » tradeType   | MARGIN_TRADE |

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
        }
      },
      "required": ["orderId"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-test---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-test---old-responseschema">Response Schema</h3>

Status Code **200**

| Name       | Type   | Required | Restrictions | Description                                                                                                                  |
| ---------- | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| » code     | string | true     | none         | none                                                                                                                         |
| » data     | object | true     | none         | none                                                                                                                         |
| »» orderId | string | true     | none         | The unique order id generated by the trading system,which can be used later for further actions such as canceling the order. |

<aside class="success">
This operation does not require authentication
</aside>

## Batch Add Orders - Old

<a id="opId040"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
            "description": "symbol",
            "example": [
              "BTC-USDT",
              "ETH-USDT",
              "KCS-USDT"
            ]
          },
          "type": {
            "type": "string",
            "description": "only limit (default is limit)",
            "enum": [
              "limit"
            ],
            "default": "limit",
            "x-api-enum": [
              {
                "value": "limit",
                "name": "limit",
                "description": "limit order"
              }
            ]
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
            "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
            "description": "Cancel after n seconds，the order timing strategy is GTT"
          },
          "tradeType": {
            "type": "string",
            "description": "The type of trading : **TRADE**（Spot Trade）",
            "enum": [
              "TRADE"
            ],
            "default": "TRADE",
            "x-api-enum": [
              {
                "value": "TRADE",
                "name": "TRADE",
                "description": "Spot"
              }
            ]
          },
          "stop": {
            "type": "string",
            "description": "Either loss or entry. Requires stopPrice to be defined",
            "enum": [
              "loss",
              "entry"
            ],
            "x-api-enum": [
              {
                "value": "loss",
                "name": "loss",
                "description": "<="
              },
              {
                "value": "entry",
                "name": "entry",
                "description": ">="
              }
            ]
          },
          "stopPrice": {
            "type": "string",
            "description": "Stop price, Need to be defined if stop is specified."
          }
        },
        "required": [
          "symbol",
          "side",
          "clientOid",
          "price",
          "size"
        ]
      }
    },
    "symbol": {
      "type": "string"
    }
  },
  "required": [
    "symbol"
  ]
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

Request via this endpoint to place 5 orders at the same time. The order type
must be a limit order of the same symbol.

> Body parameter

```json
{
  "type": "object",
  "properties": {
    "orderList": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "clientOid": {
            "type": "string",
            "description": "Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters.\n\nPlease remember the orderId created by the service provider, it used to check for updates in order status.",
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
            "description": "symbol",
            "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
          },
          "type": {
            "type": "string",
            "description": "only limit (default is limit)",
            "enum": ["limit"],
            "default": "limit",
            "x-api-enum": [
              {
                "value": "limit",
                "name": "limit",
                "description": "limit order"
              }
            ]
          },
          "remark": {
            "type": "string",
            "description": "Order placement remarks, length cannot exceed 20 characters (ASCII)"
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
            "description": "Specify quantity for order\n\nWhen **type** is limit, size refers to the amount of trading targets (the asset name written in front) for the trading pair. Teh Size must be based on the baseIncrement of the trading pair. The baseIncrement represents the precision for the trading pair. The size of an order must be a positive-integer multiple of baseIncrement and must be between baseMinSize and baseMaxSize.\n\nWhen **type** is market, select one out of two: size or funds"
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
            "description": "Cancel after n seconds，the order timing strategy is GTT"
          },
          "tradeType": {
            "type": "string",
            "description": "The type of trading : **TRADE**（Spot Trade）",
            "enum": ["TRADE"],
            "default": "TRADE",
            "x-api-enum": [
              {
                "value": "TRADE",
                "name": "TRADE",
                "description": "Spot"
              }
            ]
          },
          "stop": {
            "type": "string",
            "description": "Either loss or entry. Requires stopPrice to be defined",
            "enum": ["loss", "entry"],
            "x-api-enum": [
              {
                "value": "loss",
                "name": "loss",
                "description": "<="
              },
              {
                "value": "entry",
                "name": "entry",
                "description": ">="
              }
            ]
          },
          "stopPrice": {
            "type": "string",
            "description": "Stop price, Need to be defined if stop is specified."
          }
        },
        "required": ["symbol", "side", "clientOid", "price", "size"]
      }
    },
    "symbol": {
      "type": "string"
    }
  },
  "required": ["symbol"]
}
```

<h3 id="batch-add-orders---old-parameters">Parameters</h3>

| Name           | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body           | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » orderList    | body | [object]       | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| »» clientOid   | body | string         | true     | Client Order Id，The ClientOid field is a unique ID created by the user（we recommend using a UUID）, and can only contain numbers, letters, underscores （\_）, and hyphens （-）. This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| »» side        | body | string         | true     | specify if the order is to 'buy' or 'sell'                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| »» type        | body | string         | false    | only limit (default is limit)                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| »» remark      | body | string         | false    | Order placement remarks, length cannot exceed 20 characters (ASCII)                                                                                                                                                                                                                                                                                                                                                                                                                 |
| »» stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                            |
| »» price       | body | string         | true     | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| »» size        | body | string         | true     | Specify quantity for order                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                               |
| »» postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                                 |
| »» hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| »» iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| »» cancelAfter | body | integer(int64) | false    | Cancel after n seconds，the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                            |
| »» tradeType   | body | string         | false    | The type of trading : **TRADE**（Spot Trade）                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| »» stop        | body | string         | false    | Either loss or entry. Requires stopPrice to be defined                                                                                                                                                                                                                                                                                                                                                                                                                              |
| »» stopPrice   | body | string         | false    | Stop price, Need to be defined if stop is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » symbol       | body | string         | true     | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

#### Detailed descriptions

**»» clientOid**: Client Order Id，The ClientOid field is a unique ID created by
the user（we recommend using a UUID）, and can only contain numbers, letters,
underscores （\_）, and hyphens （-）. This field is returned when order
information is obtained. You can use clientOid to tag your orders. ClientOid is
different from the order ID created by the service provider. Please do not
initiate requests using the same clientOid. The maximum length for the ClientOid
is 40 characters.

Please remember the orderId created by the service provider, it used to check
for updates in order status.

**»» price**: Specify price for order

When placing a limit order, the price must be based on priceIncrement for the
trading pair. The price increment (priceIncrement) is the price precision for
the trading pair. For example, for the BTC-USDT trading pair, the priceIncrement
is 0.00001000. So the price for your orders cannot be less than 0.00001000 and
must be a multiple of priceIncrement. Otherwise, the order will return an
invalid priceIncrement error.

**»» size**: Specify quantity for order

When **type** is limit, size refers to the amount of trading targets (the asset
name written in front) for the trading pair. Teh Size must be based on the
baseIncrement of the trading pair. The baseIncrement represents the precision
for the trading pair. The size of an order must be a positive-integer multiple
of baseIncrement and must be between baseMinSize and baseMaxSize.

When **type** is market, select one out of two: size or funds

#### Enumerated Values

| Parameter      | Value |
| -------------- | ----- |
| »» side        | buy   |
| »» side        | sell  |
| »» type        | limit |
| »» stp         | DC    |
| »» stp         | CO    |
| »» stp         | CN    |
| »» stp         | CB    |
| »» timeInForce | GTC   |
| »» timeInForce | GTT   |
| »» timeInForce | IOC   |
| »» timeInForce | FOK   |
| »» tradeType   | TRADE |
| »» stop        | loss  |
| »» stop        | entry |

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
        "data": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "symbol": {
                "type": "string"
              },
              "type": {
                "type": "string"
              },
              "side": {
                "type": "string"
              },
              "price": {
                "type": "string"
              },
              "size": {
                "type": "string"
              },
              "funds": {
                "type": "string"
              },
              "stp": {
                "type": "string"
              },
              "stop": {
                "type": "string"
              },
              "stopPrice": {
                "type": "string"
              },
              "timeInForce": {
                "type": "string"
              },
              "cancelAfter": {
                "type": "integer"
              },
              "postOnly": {
                "type": "boolean"
              },
              "hidden": {
                "type": "boolean"
              },
              "iceberge": {
                "type": "boolean"
              },
              "iceberg": {
                "type": "boolean"
              },
              "visibleSize": {
                "type": "string"
              },
              "channel": {
                "type": "string"
              },
              "id": {
                "type": "string"
              },
              "status": {
                "type": "string"
              },
              "failMsg": {
                "type": "string"
              },
              "clientOid": {
                "type": "string"
              }
            },
            "required": [
              "symbol",
              "type",
              "side",
              "price",
              "size",
              "funds",
              "stp",
              "stop",
              "stopPrice",
              "timeInForce",
              "cancelAfter",
              "postOnly",
              "hidden",
              "iceberge",
              "iceberg",
              "visibleSize",
              "channel",
              "id",
              "status",
              "failMsg",
              "clientOid"
            ]
          }
        }
      },
      "required": ["data"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="batch-add-orders---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="batch-add-orders---old-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type     | Required | Restrictions | Description |
| --------------- | -------- | -------- | ------------ | ----------- |
| » code          | string   | true     | none         | none        |
| » data          | object   | true     | none         | none        |
| »» data         | [object] | true     | none         | none        |
| »»» symbol      | string   | true     | none         | none        |
| »»» type        | string   | true     | none         | none        |
| »»» side        | string   | true     | none         | none        |
| »»» price       | string   | true     | none         | none        |
| »»» size        | string   | true     | none         | none        |
| »»» funds       | string   | true     | none         | none        |
| »»» stp         | string   | true     | none         | none        |
| »»» stop        | string   | true     | none         | none        |
| »»» stopPrice   | string   | true     | none         | none        |
| »»» timeInForce | string   | true     | none         | none        |
| »»» cancelAfter | integer  | true     | none         | none        |
| »»» postOnly    | boolean  | true     | none         | none        |
| »»» hidden      | boolean  | true     | none         | none        |
| »»» iceberge    | boolean  | true     | none         | none        |
| »»» iceberg     | boolean  | true     | none         | none        |
| »»» visibleSize | string   | true     | none         | none        |
| »»» channel     | string   | true     | none         | none        |
| »»» id          | string   | true     | none         | none        |
| »»» status      | string   | true     | none         | none        |
| »»» failMsg     | string   | true     | none         | none        |
| »»» clientOid   | string   | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By OrderId - Old

<a id="opId041"></a>

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

This endpoint can be used to cancel a spot order by orderId. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-orderid---old-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
          }
        }
      },
      "required": ["cancelledOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-orderid---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-orderid---old-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type     | Required | Restrictions | Description |
| -------------------- | -------- | -------- | ------------ | ----------- |
| » code               | string   | true     | none         | none        |
| » data               | object   | true     | none         | none        |
| »» cancelledOrderIds | [string] | true     | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order By OrderId - Old

<a id="opId046"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/orders/{orderId}", {
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

r = requests.get('/api/v1/orders/{orderId}', headers = headers)

print(r.json())

```

`GET /api/v1/orders/{orderId}`

Request a single order info by order ID via this endpoint.

<h3 id="get-order-by-orderid---old-parameters">Parameters</h3>

| Name    | In   | Type   | Required | Description                                         |
| ------- | ---- | ------ | -------- | --------------------------------------------------- |
| orderId | path | string | true     | The unique order id generated by the trading system |

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
          "type": "string"
        },
        "symbol": {
          "type": "string"
        },
        "opType": {
          "type": "string"
        },
        "type": {
          "type": "string"
        },
        "side": {
          "type": "string"
        },
        "price": {
          "type": "string"
        },
        "size": {
          "type": "string"
        },
        "funds": {
          "type": "string"
        },
        "dealFunds": {
          "type": "string"
        },
        "dealSize": {
          "type": "string"
        },
        "fee": {
          "type": "string"
        },
        "feeCurrency": {
          "type": "string"
        },
        "stp": {
          "type": "string"
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
          "type": "string"
        },
        "postOnly": {
          "type": "boolean"
        },
        "hidden": {
          "type": "boolean"
        },
        "iceberg": {
          "type": "boolean"
        },
        "visibleSize": {
          "type": "string"
        },
        "cancelAfter": {
          "type": "integer"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string"
        },
        "remark": {
          "type": "string"
        },
        "tags": {
          "type": "string"
        },
        "isActive": {
          "type": "boolean"
        },
        "cancelExist": {
          "type": "boolean"
        },
        "createdAt": {
          "type": "integer",
          "format": "int64"
        },
        "tradeType": {
          "type": "string"
        },
        "tax": {
          "type": "string"
        },
        "taxRate": {
          "type": "string"
        },
        "taxCurrency": {
          "type": "string"
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
        "dealFunds",
        "dealSize",
        "fee",
        "stp",
        "stop",
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
        "tags",
        "isActive",
        "cancelExist",
        "createdAt",
        "tradeType",
        "feeCurrency"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-order-by-orderid---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-order-by-orderid---old-responseschema">Response Schema</h3>

Status Code **200**

| Name             | Type           | Required | Restrictions | Description |
| ---------------- | -------------- | -------- | ------------ | ----------- |
| » code           | string         | true     | none         | none        |
| » data           | object         | true     | none         | none        |
| »» id            | string         | true     | none         | none        |
| »» symbol        | string         | true     | none         | none        |
| »» opType        | string         | true     | none         | none        |
| »» type          | string         | true     | none         | none        |
| »» side          | string         | true     | none         | none        |
| »» price         | string         | true     | none         | none        |
| »» size          | string         | true     | none         | none        |
| »» funds         | string         | true     | none         | none        |
| »» dealFunds     | string         | true     | none         | none        |
| »» dealSize      | string         | true     | none         | none        |
| »» fee           | string         | true     | none         | none        |
| »» feeCurrency   | string         | true     | none         | none        |
| »» stp           | string         | true     | none         | none        |
| »» stop          | string         | true     | none         | none        |
| »» stopTriggered | boolean        | true     | none         | none        |
| »» stopPrice     | string         | true     | none         | none        |
| »» timeInForce   | string         | true     | none         | none        |
| »» postOnly      | boolean        | true     | none         | none        |
| »» hidden        | boolean        | true     | none         | none        |
| »» iceberg       | boolean        | true     | none         | none        |
| »» visibleSize   | string         | true     | none         | none        |
| »» cancelAfter   | integer        | true     | none         | none        |
| »» channel       | string         | true     | none         | none        |
| »» clientOid     | string         | true     | none         | none        |
| »» remark        | string         | true     | none         | none        |
| »» tags          | string         | true     | none         | none        |
| »» isActive      | boolean        | true     | none         | none        |
| »» cancelExist   | boolean        | true     | none         | none        |
| »» createdAt     | integer(int64) | true     | none         | none        |
| »» tradeType     | string         | true     | none         | none        |
| »» tax           | string         | false    | none         | none        |
| »» taxRate       | string         | false    | none         | none        |
| »» taxCurrency   | string         | false    | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By ClientOid - Old

<a id="opId042"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/order/client-order/{clientOid}", {
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

r = requests.delete('/api/v1/order/client-order/{clientOid}', headers = headers)

print(r.json())

```

`DELETE /api/v1/order/client-order/{clientOid}`

This endpoint can be used to cancel a spot order by clientOid. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to websocket.

<h3 id="cancel-order-by-clientoid---old-parameters">Parameters</h3>

| Name      | In   | Type   | Required | Description                                            |
| --------- | ---- | ------ | -------- | ------------------------------------------------------ |
| clientOid | path | string | true     | Client Order Id，unique identifier created by the user |

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
          "description": "Client Order Id，unique identifier created by the user"
        },
        "cancelledOrderId": {
          "type": "string",
          "description": "The unique order id generated by the trading system"
        },
        "cancelledOcoOrderIds": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      },
      "required": ["cancelledOrderId", "clientOid", "cancelledOcoOrderIds"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="cancel-order-by-clientoid---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="cancel-order-by-clientoid---old-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type     | Required | Restrictions | Description                                            |
| ----------------------- | -------- | -------- | ------------ | ------------------------------------------------------ |
| » code                  | string   | true     | none         | none                                                   |
| » data                  | object   | true     | none         | none                                                   |
| »» clientOid            | string   | true     | none         | Client Order Id，unique identifier created by the user |
| »» cancelledOrderId     | string   | true     | none         | The unique order id generated by the trading system    |
| »» cancelledOcoOrderIds | [string] | true     | none         | none                                                   |

<aside class="success">
This operation does not require authentication
</aside>

## Get Order By ClientOid - Old

<a id="opId047"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/order/client-order/{clientOid}", {
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

r = requests.get('/api/v1/order/client-order/{clientOid}', headers = headers)

print(r.json())

```

`GET /api/v1/order/client-order/{clientOid}`

Request via this interface to check the information of a single active order via
clientOid. The system will send a prompt that the order does not exist if the
order does not exist or has been settled.

<h3 id="get-order-by-clientoid---old-parameters">Parameters</h3>

| Name      | In   | Type   | Required | Description                                               |
| --------- | ---- | ------ | -------- | --------------------------------------------------------- |
| clientOid | path | string | true     | Unique order ID created by users to identify their orders |

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
          "type": "string"
        },
        "symbol": {
          "type": "string"
        },
        "opType": {
          "type": "string"
        },
        "type": {
          "type": "string"
        },
        "side": {
          "type": "string"
        },
        "price": {
          "type": "string"
        },
        "size": {
          "type": "string"
        },
        "funds": {
          "type": "string"
        },
        "dealFunds": {
          "type": "string"
        },
        "dealSize": {
          "type": "string"
        },
        "fee": {
          "type": "string"
        },
        "feeCurrency": {
          "type": "string"
        },
        "stp": {
          "type": "string"
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
          "type": "string"
        },
        "postOnly": {
          "type": "boolean"
        },
        "hidden": {
          "type": "boolean"
        },
        "iceberg": {
          "type": "boolean"
        },
        "visibleSize": {
          "type": "string"
        },
        "cancelAfter": {
          "type": "integer"
        },
        "channel": {
          "type": "string"
        },
        "clientOid": {
          "type": "string"
        },
        "remark": {
          "type": "string"
        },
        "tags": {
          "type": "string"
        },
        "isActive": {
          "type": "boolean"
        },
        "cancelExist": {
          "type": "boolean"
        },
        "createdAt": {
          "type": "integer",
          "format": "int64"
        },
        "tradeType": {
          "type": "string"
        },
        "tax": {
          "type": "string"
        },
        "taxRate": {
          "type": "string"
        },
        "taxCurrency": {
          "type": "string"
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
        "dealFunds",
        "dealSize",
        "fee",
        "feeCurrency",
        "stp",
        "stop",
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
        "tags",
        "isActive",
        "cancelExist",
        "createdAt",
        "tradeType"
      ]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-order-by-clientoid---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-order-by-clientoid---old-responseschema">Response Schema</h3>

Status Code **200**

| Name             | Type           | Required | Restrictions | Description |
| ---------------- | -------------- | -------- | ------------ | ----------- |
| » code           | string         | true     | none         | none        |
| » data           | object         | true     | none         | none        |
| »» id            | string         | true     | none         | none        |
| »» symbol        | string         | true     | none         | none        |
| »» opType        | string         | true     | none         | none        |
| »» type          | string         | true     | none         | none        |
| »» side          | string         | true     | none         | none        |
| »» price         | string         | true     | none         | none        |
| »» size          | string         | true     | none         | none        |
| »» funds         | string         | true     | none         | none        |
| »» dealFunds     | string         | true     | none         | none        |
| »» dealSize      | string         | true     | none         | none        |
| »» fee           | string         | true     | none         | none        |
| »» feeCurrency   | string         | true     | none         | none        |
| »» stp           | string         | true     | none         | none        |
| »» stop          | string         | true     | none         | none        |
| »» stopTriggered | boolean        | true     | none         | none        |
| »» stopPrice     | string         | true     | none         | none        |
| »» timeInForce   | string         | true     | none         | none        |
| »» postOnly      | boolean        | true     | none         | none        |
| »» hidden        | boolean        | true     | none         | none        |
| »» iceberg       | boolean        | true     | none         | none        |
| »» visibleSize   | string         | true     | none         | none        |
| »» cancelAfter   | integer        | true     | none         | none        |
| »» channel       | string         | true     | none         | none        |
| »» clientOid     | string         | true     | none         | none        |
| »» remark        | string         | true     | none         | none        |
| »» tags          | string         | true     | none         | none        |
| »» isActive      | boolean        | true     | none         | none        |
| »» cancelExist   | boolean        | true     | none         | none        |
| »» createdAt     | integer(int64) | true     | none         | none        |
| »» tradeType     | string         | true     | none         | none        |
| »» tax           | string         | false    | none         | none        |
| »» taxRate       | string         | false    | none         | none        |
| »» taxCurrency   | string         | false    | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Recent Orders List - Old

<a id="opId045"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/limit/orders", {
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

r = requests.get('/api/v1/limit/orders', headers = headers)

print(r.json())

```

`GET /api/v1/limit/orders`

Request your current order list via this endpoint. The return value is the data
after Pagination, sorted in descending order according to time.

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
            "type": "string"
          },
          "symbol": {
            "type": "string"
          },
          "opType": {
            "type": "string"
          },
          "type": {
            "type": "string"
          },
          "side": {
            "type": "string"
          },
          "price": {
            "type": "string"
          },
          "size": {
            "type": "string"
          },
          "funds": {
            "type": "string"
          },
          "dealFunds": {
            "type": "string"
          },
          "dealSize": {
            "type": "string"
          },
          "fee": {
            "type": "string"
          },
          "feeCurrency": {
            "type": "string"
          },
          "stp": {
            "type": "string"
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
            "type": "string"
          },
          "postOnly": {
            "type": "boolean"
          },
          "hidden": {
            "type": "boolean"
          },
          "iceberg": {
            "type": "boolean"
          },
          "visibleSize": {
            "type": "string"
          },
          "cancelAfter": {
            "type": "integer"
          },
          "channel": {
            "type": "string"
          },
          "clientOid": {
            "type": "string"
          },
          "remark": {
            "type": "string"
          },
          "tags": {
            "type": "string"
          },
          "isActive": {
            "type": "boolean"
          },
          "cancelExist": {
            "type": "boolean"
          },
          "createdAt": {
            "type": "integer",
            "format": "int64"
          },
          "tradeType": {
            "type": "string"
          },
          "tax": {
            "type": "string"
          },
          "taxRate": {
            "type": "string"
          },
          "taxCurrency": {
            "type": "string"
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
          "dealFunds",
          "dealSize",
          "fee",
          "feeCurrency",
          "stp",
          "stop",
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
          "tags",
          "isActive",
          "cancelExist",
          "createdAt",
          "tradeType"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-recent-orders-list---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-recent-orders-list---old-responseschema">Response Schema</h3>

Status Code **200**

| Name             | Type           | Required | Restrictions | Description |
| ---------------- | -------------- | -------- | ------------ | ----------- |
| » code           | string         | true     | none         | none        |
| » data           | [object]       | true     | none         | none        |
| »» id            | string         | true     | none         | none        |
| »» symbol        | string         | true     | none         | none        |
| »» opType        | string         | true     | none         | none        |
| »» type          | string         | true     | none         | none        |
| »» side          | string         | true     | none         | none        |
| »» price         | string         | true     | none         | none        |
| »» size          | string         | true     | none         | none        |
| »» funds         | string         | true     | none         | none        |
| »» dealFunds     | string         | true     | none         | none        |
| »» dealSize      | string         | true     | none         | none        |
| »» fee           | string         | true     | none         | none        |
| »» feeCurrency   | string         | true     | none         | none        |
| »» stp           | string         | true     | none         | none        |
| »» stop          | string         | true     | none         | none        |
| »» stopTriggered | boolean        | true     | none         | none        |
| »» stopPrice     | string         | true     | none         | none        |
| »» timeInForce   | string         | true     | none         | none        |
| »» postOnly      | boolean        | true     | none         | none        |
| »» hidden        | boolean        | true     | none         | none        |
| »» iceberg       | boolean        | true     | none         | none        |
| »» visibleSize   | string         | true     | none         | none        |
| »» cancelAfter   | integer        | true     | none         | none        |
| »» channel       | string         | true     | none         | none        |
| »» clientOid     | string         | true     | none         | none        |
| »» remark        | string         | true     | none         | none        |
| »» tags          | string         | true     | none         | none        |
| »» isActive      | boolean        | true     | none         | none        |
| »» cancelExist   | boolean        | true     | none         | none        |
| »» createdAt     | integer(int64) | true     | none         | none        |
| »» tradeType     | string         | true     | none         | none        |
| »» tax           | string         | false    | none         | none        |
| »» taxRate       | string         | false    | none         | none        |
| »» taxCurrency   | string         | false    | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

## Get Trade History - Old

<a id="opId048"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/fills?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v1/fills', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v1/fills`

Request recent fills via this endpoint. The return value is the data after
Pagination, sorted in descending order according to time.

<h3 id="get-trade-history---old-parameters">Parameters</h3>

| Name        | In    | Type           | Required | Description                                                                                                                                         |
| ----------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol      | query | string         | true     | symbol                                                                                                                                              |
| orderId     | query | string         | false    | The unique order ID generated by the trading system. (If orderId is specified, please ignore the other query parameters.)                           |
| side        | query | string         | false    | Specify if the order is to 'buy' or 'sell'.                                                                                                         |
| type        | query | string         | false    | limit, market, limit_stop or market_stop                                                                                                            |
| tradeType   | query | string         | false    | The type of trading: TRADE - Spot Trading (TRADE as default), MARGIN_TRADE - Cross Margin Trading, MARGIN_ISOLATED_TRADE - Isolated Margin Trading. |
| startAt     | query | integer(int64) | false    | Start time (milliseconds)                                                                                                                           |
| endAt       | query | integer(int64) | false    | End time (milliseconds)                                                                                                                             |
| currentPage | query | integer        | false    | Current request page.                                                                                                                               |
| pageSize    | query | integer        | false    | Number of results per request. Minimum is 10, maximum is 500.                                                                                       |

#### Detailed descriptions

**type**: limit, market, limit_stop or market_stop

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |
| type      | limit_stop            |
| type      | market_stop           |
| tradeType | TRADE                 |
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
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "tradeId": {
                "type": "string"
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
                "description": "Buy or sell"
              },
              "liquidity": {
                "type": "string",
                "description": "Liquidity type: taker or maker"
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
              "type": {
                "type": "string",
                "description": "Specify if the order is a 'limit' order or 'market' order. "
              },
              "createdAt": {
                "type": "integer",
                "format": "int64"
              }
            }
          }
        }
      },
      "required": ["currentPage", "pageSize", "totalNum", "totalPage", "items"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-trade-history---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-trade-history---old-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type           | Required | Restrictions | Description                                                                                                       |
| ------------------ | -------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                              |
| » data             | object         | true     | none         | none                                                                                                              |
| »» currentPage     | integer        | true     | none         | none                                                                                                              |
| »» pageSize        | integer        | true     | none         | none                                                                                                              |
| »» totalNum        | integer        | true     | none         | none                                                                                                              |
| »» totalPage       | integer        | true     | none         | none                                                                                                              |
| »» items           | [object]       | true     | none         | none                                                                                                              |
| »»» symbol         | string         | false    | none         | symbol                                                                                                            |
| »»» tradeId        | string         | false    | none         | none                                                                                                              |
| »»» orderId        | string         | false    | none         | The unique order id generated by the trading system                                                               |
| »»» counterOrderId | string         | false    | none         | Counterparty order ID                                                                                             |
| »»» side           | string         | false    | none         | Buy or sell                                                                                                       |
| »»» liquidity      | string         | false    | none         | Liquidity type: taker or maker                                                                                    |
| »»» forceTaker     | boolean        | false    | none         | none                                                                                                              |
| »»» price          | string         | false    | none         | Order Price                                                                                                       |
| »»» size           | string         | false    | none         | Order Size                                                                                                        |
| »»» funds          | string         | false    | none         | Order Funds                                                                                                       |
| »»» fee            | string         | false    | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                      |
| »»» feeRate        | string         | false    | none         | Fee rate                                                                                                          |
| »»» feeCurrency    | string         | false    | none         | Currency used to calculate trading fee                                                                            |
| »»» stop           | string         | false    | none         | Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty |
| »»» tradeType      | string         | false    | none         | Trade type, redundancy param                                                                                      |
| »»» type           | string         | false    | none         | Specify if the order is a 'limit' order or 'market' order.                                                        |
| »»» createdAt      | integer(int64) | false    | none         | none                                                                                                              |

<aside class="success">
This operation does not require authentication
</aside>

## Get Recent Trade History - Old

<a id="opId049"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v1/limit/fills", {
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

r = requests.get('/api/v1/limit/fills', headers = headers)

print(r.json())

```

`GET /api/v1/limit/fills`

Request a list of 1000 fills in the last 24 hours via this endpoint. The return
value is the data after Pagination, sorted in descending order according to
time.

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
            "type": "string"
          },
          "tradeId": {
            "type": "string"
          },
          "orderId": {
            "type": "string"
          },
          "counterOrderId": {
            "type": "string"
          },
          "side": {
            "type": "string"
          },
          "liquidity": {
            "type": "string"
          },
          "forceTaker": {
            "type": "boolean"
          },
          "price": {
            "type": "string"
          },
          "size": {
            "type": "string"
          },
          "funds": {
            "type": "string"
          },
          "fee": {
            "type": "string"
          },
          "feeRate": {
            "type": "string"
          },
          "feeCurrency": {
            "type": "string"
          },
          "stop": {
            "type": "string"
          },
          "tradeType": {
            "type": "string"
          },
          "type": {
            "type": "string"
          },
          "createdAt": {
            "type": "integer",
            "format": "int64"
          },
          "tax": {
            "type": "string"
          },
          "taxCurrency": {
            "type": "string"
          },
          "taxRate": {
            "type": "string"
          }
        },
        "required": [
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
          "type",
          "createdAt"
        ]
      }
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="get-recent-trade-history---old-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="get-recent-trade-history---old-responseschema">Response Schema</h3>

Status Code **200**

| Name              | Type           | Required | Restrictions | Description |
| ----------------- | -------------- | -------- | ------------ | ----------- |
| » code            | string         | true     | none         | none        |
| » data            | [object]       | true     | none         | none        |
| »» symbol         | string         | true     | none         | none        |
| »» tradeId        | string         | true     | none         | none        |
| »» orderId        | string         | true     | none         | none        |
| »» counterOrderId | string         | true     | none         | none        |
| »» side           | string         | true     | none         | none        |
| »» liquidity      | string         | true     | none         | none        |
| »» forceTaker     | boolean        | true     | none         | none        |
| »» price          | string         | true     | none         | none        |
| »» size           | string         | true     | none         | none        |
| »» funds          | string         | true     | none         | none        |
| »» fee            | string         | true     | none         | none        |
| »» feeRate        | string         | true     | none         | none        |
| »» feeCurrency    | string         | true     | none         | none        |
| »» stop           | string         | true     | none         | none        |
| »» tradeType      | string         | true     | none         | none        |
| »» type           | string         | true     | none         | none        |
| »» createdAt      | integer(int64) | true     | none         | none        |
| »» tax            | string         | false    | none         | none        |
| »» taxCurrency    | string         | false    | none         | none        |
| »» taxRate        | string         | false    | none         | none        |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
