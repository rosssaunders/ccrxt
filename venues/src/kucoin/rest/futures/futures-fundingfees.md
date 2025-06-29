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

fundingfees

<h1 id="futures-default">Default</h1>

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
