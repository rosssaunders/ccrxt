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

debit

<h1 id="margin-default">Default</h1>

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

# Schemas
