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

order

<h1 id="futures-default">Default</h1>

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

# Schemas
