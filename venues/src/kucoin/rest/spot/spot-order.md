---
title: spot v1.0.0
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

<h1 id="spot">spot v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

order

<h1 id="spot-default">Default</h1>

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
