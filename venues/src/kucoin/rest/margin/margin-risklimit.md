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

risklimit

<h1 id="margin-default">Default</h1>

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
