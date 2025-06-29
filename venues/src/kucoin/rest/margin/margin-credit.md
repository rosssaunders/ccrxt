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

credit

<h1 id="margin-default">Default</h1>

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

# Schemas
