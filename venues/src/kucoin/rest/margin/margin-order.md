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

order

<h1 id="margin-default">Default</h1>

## Add Order

<a id="opId001"></a>

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
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

fetch('/api/v3/hf/margin/order',
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

r = requests.post('/api/v3/hf/margin/order', headers = headers)

print(r.json())

```

`POST /api/v3/hf/margin/order`

Place order in the Cross-margin or Isolated-margin trading system. You can place
two major types of order: Limit and market. Orders can only be placed if your
account has sufficient funds. Once an order is placed, your funds will be put on
hold for the duration of the order. The amount of funds on hold depends on the
order type and parameters specified.

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » isIsolated  | body | boolean        | false    | True - isolated margin; false - cross margin. Default is false                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |

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
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        }
      },
      "required": ["orderId", "borrowSize", "clientOid", "loanApplyId"]
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

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | The user self-defined order ID.                                                                                               |

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
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

fetch('/api/v3/hf/margin/order/test',
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

r = requests.post('/api/v3/hf/margin/order/test', headers = headers)

print(r.json())

```

`POST /api/v3/hf/margin/order/test`

Order test endpoint: This endpoint’s request and return parameters are identical
to the order endpoint, and can be used to verify whether the signature is
correct, among other operations. After placing an order, the order will not
enter the matching system, and the order cannot be queried.

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
    "cancelAfter": {
      "type": "integer",
      "format": "int64",
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "isIsolated": {
      "type": "boolean",
      "description": "True - isolated margin; false - cross margin. Default is false",
      "default": false
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-test-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | [Hidden order](https://www.kucoin.com/docs-new/doc-338146) or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                    |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in [Iceberg orders](https://www.kucoin.com/docs-new/doc-338146)                                                                                                                                                                                                                                                                                                                                                       |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » isIsolated  | body | boolean        | false    | True - isolated margin; false - cross margin. Default is false                                                                                                                                                                                                                                                                                                                                                                                                                 |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |

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
        "loanApplyId": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "number",
          "description": "ID of the borrowing response. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "The user self-defined order ID."
        }
      },
      "required": ["orderId", "borrowSize", "clientOid", "loanApplyId"]
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

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | number | true     | none         | ID of the borrowing response. The field is returned only after placing the order under the mode of Auto-Borrow.               |
| »» clientOid   | string | true     | none         | The user self-defined order ID.                                                                                               |

<aside class="success">
This operation does not require authentication
</aside>

## Cancel Order By OrderId

<a id="opId003"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/hf/margin/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.delete('/api/v3/hf/margin/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders/{orderId}`

This endpoint can be used to cancel a margin order by orderId. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-orderid-parameters">Parameters</h3>

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

<a id="opId010"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("/api/v3/hf/margin/orders/{orderId}?symbol=BTC-USDT,ETH-USDT,KCS-USDT", {
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

r = requests.get('/api/v3/hf/margin/orders/{orderId}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/{orderId}`

This endpoint can be used to obtain information for a single Margin order using
the order ID. After the user successfully places an order, the order is in the
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
          "description": "Specify if the order is a 'limit' order or 'market' order. ",
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
          "description": "Currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
          "description": "Whether it’s a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether it’s a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether it’s a iceberg order."
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
          "description": "Client Order Id, unique identifier created by the user"
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
          "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
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
          "description": "Users in some regions have this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order is active; false-The status of the order is done"
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
        "cancelExist",
        "createdAt",
        "lastUpdatedAt",
        "tradeType",
        "inOrderBook",
        "cancelledSize",
        "cancelledFunds",
        "remainSize",
        "remainFunds",
        "tax",
        "active"
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

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | object         | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                              |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | true     | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

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

## Cancel Order By ClientOid

<a id="opId004"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
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

r = requests.delete('/api/v3/hf/margin/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders/client-order/{clientOid}`

This endpoint can be used to cancel a margin order by clientOid. This endpoint
only sends cancellation requests. The results of the requests must be obtained
by checking the order status or subscribing to Websocket.

<h3 id="cancel-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id, unique identifier created by the user |

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
          "description": "Client Order Id, unique identifier created by the user"
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
| »» clientOid | string | true     | none         | Client Order Id, unique identifier created by the user |

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

fetch(
  "/api/v3/hf/margin/orders/client-order/{clientOid}?symbol=BTC-USDT,ETH-USDT,KCS-USDT",
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

r = requests.get('/api/v3/hf/margin/orders/client-order/{clientOid}', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
]
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/client-order/{clientOid}`

This endpoint can be used to obtain information for a single Margin order using
the client order ID. After the user successfully places an order, the order is
in the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-order-by-clientoid-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                            |
| --------- | ----- | ------ | -------- | ------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                 |
| clientOid | path  | string | true     | Client Order Id, unique identifier created by the user |

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
          "description": "Specify if the order is a 'limit' order or 'market' order. ",
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
          "description": "Currency used to calculate trading fee"
        },
        "stp": {
          "type": "string",
          "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
          "description": "Whether it’s a postOnly order."
        },
        "hidden": {
          "type": "boolean",
          "description": "Whether it’s a hidden order."
        },
        "iceberg": {
          "type": "boolean",
          "description": "Whether it’s a iceberg order."
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
          "description": "Client Order Id, unique identifier created by the user"
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
          "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
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
          "description": "Users in some regions have this field"
        },
        "active": {
          "type": "boolean",
          "description": "Order status: true-The status of the order is active; false-The status of the order is done"
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
        "cancelExist",
        "createdAt",
        "lastUpdatedAt",
        "tradeType",
        "inOrderBook",
        "cancelledSize",
        "cancelledFunds",
        "remainSize",
        "remainFunds",
        "tax",
        "active"
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

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | object         | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                              |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | false    | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

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

## Cancel All Orders By Symbol

<a id="opId005"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.delete('/api/v3/hf/margin/orders', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`DELETE /api/v3/hf/margin/orders`

This interface can cancel all open Margin orders by symbol. This endpoint only
sends cancellation requests. The results of the requests must be obtained by
checking the order status or subscribing to Websocket.

<h3 id="cancel-all-orders-by-symbol-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                        |
| --------- | ----- | ------ | -------- | -------------------------------------------------------------------------------------------------- |
| symbol    | query | string | true     | symbol                                                                                             |
| tradeType | query | string | true     | Transaction type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
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

## Get Symbols With Open Order

<a id="opId006"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/order/active/symbols?tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v3/hf/margin/order/active/symbols', params={
  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/order/active/symbols`

This interface can query all Margin symbols that have active orders.

<h3 id="get-symbols-with-open-order-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                        |
| --------- | ----- | ------ | -------- | ------------------------------------------------------------------ |
| tradeType | query | string | true     | Cross Margin: MARGIN_TRADE, Isolated Margin: MARGIN_ISOLATED_TRADE |

#### Detailed descriptions

**tradeType**: Cross Margin: MARGIN_TRADE, Isolated Margin:
MARGIN_ISOLATED_TRADE

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
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
        "symbolSize": {
          "type": "integer",
          "description": "Symbol Size"
        },
        "symbols": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "description": "The symbol that has active orders"
        }
      },
      "required": ["symbolSize", "symbols"]
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

| Name          | Type     | Required | Restrictions | Description                       |
| ------------- | -------- | -------- | ------------ | --------------------------------- |
| » code        | string   | true     | none         | none                              |
| » data        | object   | true     | none         | none                              |
| »» symbolSize | integer  | true     | none         | Symbol Size                       |
| »» symbols    | [string] | true     | none         | The symbol that has active orders |

<aside class="success">
This operation does not require authentication
</aside>

## Get Open Orders

<a id="opId007"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/active?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v3/hf/margin/orders/active', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/active`

This interface is to obtain all Margin active order lists, and the return value
of the active order interface is the paged data of all uncompleted order lists.
The returned data is sorted in descending order according to the latest update
time of the order. After the user successfully places an order, the order is in
the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-open-orders-parameters">Parameters</h3>

| Name      | In    | Type   | Required | Description                                                                                                  |
| --------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------ |
| symbol    | query | string | true     | symbol                                                                                                       |
| tradeType | query | string | true     | Order type: MARGIN_TRADE - cross margin trading order, MARGIN_ISOLATED_TRADE - isolated margin trading order |

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
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
            "description": "Specify if the order is a 'limit' order or 'market' order. ",
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
            "description": "Trading fee"
          },
          "feeCurrency": {
            "type": "string",
            "description": "Currency used to calculate trading fee"
          },
          "stp": {
            "type": "string",
            "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
            "description": "Whether it’s a postOnly order."
          },
          "hidden": {
            "type": "boolean",
            "description": "Whether it’s a hidden order."
          },
          "iceberg": {
            "type": "boolean",
            "description": "Whether it’s a iceberg order."
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
            "description": "Client Order Id, unique identifier created by the user"
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
            "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
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
            "description": "Users in some regions have this field"
          },
          "active": {
            "type": "boolean",
            "description": "Order status: true-The status of the order is active; false-The status of the order is done"
          }
        },
        "required": [
          "stopPrice",
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
          "stopTriggered",
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

| Name              | Type           | Required | Restrictions | Description                                                                                                               |
| ----------------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| » code            | string         | true     | none         | none                                                                                                                      |
| » data            | [object]       | true     | none         | none                                                                                                                      |
| »» id             | string         | true     | none         | The unique order id generated by the trading system                                                                       |
| »» symbol         | string         | true     | none         | symbol                                                                                                                    |
| »» opType         | string         | true     | none         | none                                                                                                                      |
| »» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                |
| »» side           | string         | true     | none         | Buy or sell                                                                                                               |
| »» price          | string         | true     | none         | Order Price                                                                                                               |
| »» size           | string         | true     | none         | Order Size                                                                                                                |
| »» funds          | string         | true     | none         | Order Funds                                                                                                               |
| »» dealSize       | string         | true     | none         | Number of filled transactions                                                                                             |
| »» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                              |
| »» fee            | string         | true     | none         | Trading fee                                                                                                               |
| »» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                    |
| »» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC |
| »» stop           | string         | false    | none         | none                                                                                                                      |
| »» stopTriggered  | boolean        | true     | none         | none                                                                                                                      |
| »» stopPrice      | string         | true     | none         | none                                                                                                                      |
| »» timeInForce    | string         | true     | none         | Time in force                                                                                                             |
| »» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                            |
| »» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                              |
| »» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                             |
| »» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                              |
| »» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                               |
| »» channel        | string         | true     | none         | none                                                                                                                      |
| »» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                    |
| »» remark         | string         | false    | none         | Order placement remarks                                                                                                   |
| »» tags           | string         | false    | none         | Order tag                                                                                                                 |
| »» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                     |
| »» createdAt      | integer(int64) | true     | none         | none                                                                                                                      |
| »» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                      |
| »» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                              |
| »» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                              |
| »» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                           |
| »» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                            |
| »» remainSize     | string         | true     | none         | Number of remain transactions                                                                                             |
| »» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                              |
| »» tax            | string         | true     | none         | Users in some regions have this field                                                                                     |
| »» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                               |

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

<a id="opId008"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/orders/done?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v3/hf/margin/orders/done', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/orders/done`

This interface is to obtain all Margin closed order lists, and the return value
of the active order interface is the paged data of all uncompleted order lists.
The returned data is sorted in descending order according to the latest update
time of the order. After the user successfully places an order, the order is in
the Active state, and the user can use inOrderBook to determine whether the
order has entered the order. Canceled or fully filled orders are marked as
completed Done status.

<h3 id="get-closed-orders-parameters">Parameters</h3>

| Name      | In    | Type           | Required | Description                                                                                               |
| --------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| symbol    | query | string         | true     | symbol                                                                                                    |
| tradeType | query | string         | true     | Transaction type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade        |
| side      | query | string         | false    | Specify if the order is to 'buy' or 'sell'.                                                               |
| type      | query | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                |
| lastId    | query | integer(int64) | false    | The ID of the last set of data from the previous data batch. By default, the latest information is given. |
| limit     | query | integer        | false    | Default20, Max100                                                                                         |
| startAt   | query | integer(int64) | false    | Start time (milliseconds)                                                                                 |
| endAt     | query | integer(int64) | false    | End time (milliseconds)                                                                                   |

#### Detailed descriptions

**lastId**: The ID of the last set of data from the previous data batch. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId, which can be used as a query
parameter to look up new data from the next page.

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |

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
          "description": "The ID of the last set of data from the previous data batch. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page.",
          "format": "int64"
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
                "description": "Specify if the order is a 'limit' order or 'market' order. ",
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
                "description": "Buy or sell"
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
                "description": "Currency used to calculate trading fee"
              },
              "stp": {
                "type": "string",
                "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC",
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
                "description": "Whether it’s a postOnly order."
              },
              "hidden": {
                "type": "boolean",
                "description": "Whether it’s a hidden order."
              },
              "iceberg": {
                "type": "boolean",
                "description": "Whether it’s a iceberg order."
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
                "description": "Client Order Id, unique identifier created by the user"
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
                "description": "Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook"
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
                "description": "Users in some regions have this field"
              },
              "active": {
                "type": "boolean",
                "description": "Order status: true-The status of the order is active; false-The status of the order is done"
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
              "cancelExist",
              "createdAt",
              "lastUpdatedAt",
              "tradeType",
              "inOrderBook",
              "cancelledSize",
              "cancelledFunds",
              "remainSize",
              "remainFunds",
              "tax",
              "active"
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

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» lastId          | integer(int64) | true     | none         | The ID of the last set of data from the previous data batch. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page. |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» id             | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                       |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                    |
| »»» opType         | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                               |
| »»» price          | string         | true     | none         | Order Price                                                                                                                                                                                                                                                                                                                                               |
| »»» size           | string         | true     | none         | Order Size                                                                                                                                                                                                                                                                                                                                                |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                               |
| »»» dealSize       | string         | true     | none         | Number of filled transactions                                                                                                                                                                                                                                                                                                                             |
| »»» dealFunds      | string         | true     | none         | Funds of filled transactions                                                                                                                                                                                                                                                                                                                              |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                              |
| »»» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                    |
| »»» stp            | string         | false    | none         | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into these strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                 |
| »»» stop           | string         | false    | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» stopTriggered  | boolean        | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» stopPrice      | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» timeInForce    | string         | true     | none         | Time in force                                                                                                                                                                                                                                                                                                                                             |
| »»» postOnly       | boolean        | true     | none         | Whether it’s a postOnly order.                                                                                                                                                                                                                                                                                                                            |
| »»» hidden         | boolean        | true     | none         | Whether it’s a hidden order.                                                                                                                                                                                                                                                                                                                              |
| »»» iceberg        | boolean        | true     | none         | Whether it’s a iceberg order.                                                                                                                                                                                                                                                                                                                             |
| »»» visibleSize    | string         | true     | none         | Visible size of iceberg order in order book.                                                                                                                                                                                                                                                                                                              |
| »»» cancelAfter    | integer        | true     | none         | A GTT timeInForce that expires in n seconds                                                                                                                                                                                                                                                                                                               |
| »»» channel        | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» clientOid      | string         | true     | none         | Client Order Id, unique identifier created by the user                                                                                                                                                                                                                                                                                                    |
| »»» remark         | string         | false    | none         | Order placement remarks                                                                                                                                                                                                                                                                                                                                   |
| »»» tags           | string         | false    | none         | Order tag                                                                                                                                                                                                                                                                                                                                                 |
| »»» cancelExist    | boolean        | true     | none         | Whether there is a cancellation record for the order.                                                                                                                                                                                                                                                                                                     |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» lastUpdatedAt  | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                              |
| »»» inOrderBook    | boolean        | true     | none         | Whether to enter the orderbook: True: enter the orderbook; False: do not enter the orderbook                                                                                                                                                                                                                                                              |
| »»» cancelledSize  | string         | true     | none         | Number of canceled transactions                                                                                                                                                                                                                                                                                                                           |
| »»» cancelledFunds | string         | true     | none         | Funds of canceled transactions                                                                                                                                                                                                                                                                                                                            |
| »»» remainSize     | string         | true     | none         | Number of remain transactions                                                                                                                                                                                                                                                                                                                             |
| »»» remainFunds    | string         | true     | none         | Funds of remain transactions                                                                                                                                                                                                                                                                                                                              |
| »»» tax            | string         | true     | none         | Users in some regions have this field                                                                                                                                                                                                                                                                                                                     |
| »»» active         | boolean        | true     | none         | Order status: true-The status of the order is active; false-The status of the order is done                                                                                                                                                                                                                                                               |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| type        | limit  |
| type        | market |
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

<a id="opId009"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "/api/v3/hf/margin/fills?symbol=BTC-USDT,ETH-USDT,KCS-USDT&tradeType=type,string,enum,MARGIN_TRADE%2CMARGIN_ISOLATED_TRADE,x-api-enum,%5Bobject%20Object%5D%2C%5Bobject%20Object%5D",
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

r = requests.get('/api/v3/hf/margin/fills', params={
  'symbol': [
  "BTC-USDT",
  "ETH-USDT",
  "KCS-USDT"
],  'tradeType': {
  "type": "string",
  "enum": [
    "MARGIN_TRADE",
    "MARGIN_ISOLATED_TRADE"
  ],
  "x-api-enum": [
    {
      "value": "MARGIN_TRADE",
      "name": "",
      "description": ""
    },
    {
      "value": "MARGIN_ISOLATED_TRADE",
      "name": "",
      "description": ""
    }
  ]
}
}, headers = headers)

print(r.json())

```

`GET /api/v3/hf/margin/fills`

This endpoint can be used to obtain a list of the latest Margin transaction
details. The returned data is sorted in descending order according to the latest
update time of the order.

<h3 id="get-trade-history-parameters">Parameters</h3>

| Name      | In    | Type           | Required | Description                                                                                               |
| --------- | ----- | -------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| symbol    | query | string         | true     | symbol                                                                                                    |
| tradeType | query | string         | true     | Trade type: MARGIN_TRADE - cross margin trade, MARGIN_ISOLATED_TRADE - isolated margin trade              |
| orderId   | query | string         | false    | The unique order id generated by the trading system                                                       |
| side      | query | string         | false    | Specify if the order is to 'buy' or 'sell'.                                                               |
| type      | query | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                |
| lastId    | query | integer(int64) | false    | The ID of the last set of data from the previous data batch. By default, the latest information is given. |
| limit     | query | integer        | false    | Default20, Max100                                                                                         |
| startAt   | query | integer(int64) | false    | Start time (milliseconds)                                                                                 |
| endAt     | query | integer(int64) | false    | End time (milliseconds)                                                                                   |

#### Detailed descriptions

**orderId**: The unique order id generated by the trading system (If orderId is
specified, please ignore the other query parameters)

**lastId**: The ID of the last set of data from the previous data batch. By
default, the latest information is given. lastId is used to filter data and
paginate. If lastId is not entered, the default is a maximum of 100 returned
data items. The return results include lastId, which can be used as a query
parameter to look up new data from the next page.

#### Enumerated Values

| Parameter | Value                 |
| --------- | --------------------- |
| tradeType | MARGIN_TRADE          |
| tradeType | MARGIN_ISOLATED_TRADE |
| side      | buy                   |
| side      | sell                  |
| type      | limit                 |
| type      | market                |

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
                "description": "ID of transaction detail",
                "format": "int64"
              },
              "symbol": {
                "type": "string",
                "description": "symbol",
                "example": ["BTC-USDT", "ETH-USDT", "KCS-USDT"]
              },
              "tradeId": {
                "type": "integer",
                "description": "Trade ID, symbol latitude increment",
                "format": "int64"
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
              "tax": {
                "type": "string",
                "description": "Users in some regions have this field"
              },
              "taxRate": {
                "type": "string",
                "description": "Tax Rate: Users in some regions must query this field"
              },
              "type": {
                "type": "string",
                "description": "Specify if the order is a 'limit' order or 'market' order. ",
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
              "tax",
              "taxRate",
              "type",
              "createdAt"
            ]
          }
        },
        "lastId": {
          "type": "integer",
          "description": "The ID of the last set of data from the previous data batch. By default, the latest information is given.\nlastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page.",
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

| Name               | Type           | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                               |
| ------------------ | -------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » code             | string         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| » data             | object         | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» items           | [object]       | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» id             | integer(int64) | true     | none         | ID of transaction detail                                                                                                                                                                                                                                                                                                                                  |
| »»» symbol         | string         | true     | none         | symbol                                                                                                                                                                                                                                                                                                                                                    |
| »»» tradeId        | integer(int64) | true     | none         | Trade ID, symbol latitude increment                                                                                                                                                                                                                                                                                                                       |
| »»» orderId        | string         | true     | none         | The unique order id generated by the trading system                                                                                                                                                                                                                                                                                                       |
| »»» counterOrderId | string         | true     | none         | Counterparty order ID                                                                                                                                                                                                                                                                                                                                     |
| »»» side           | string         | true     | none         | Buy or sell                                                                                                                                                                                                                                                                                                                                               |
| »»» liquidity      | string         | true     | none         | Liquidity type: taker or maker                                                                                                                                                                                                                                                                                                                            |
| »»» forceTaker     | boolean        | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »»» price          | string         | true     | none         | Order Price                                                                                                                                                                                                                                                                                                                                               |
| »»» size           | string         | true     | none         | Order Size                                                                                                                                                                                                                                                                                                                                                |
| »»» funds          | string         | true     | none         | Order Funds                                                                                                                                                                                                                                                                                                                                               |
| »»» fee            | string         | true     | none         | [Handling fees](https://www.kucoin.com/docs-new/api-5327739)                                                                                                                                                                                                                                                                                              |
| »»» feeRate        | string         | true     | none         | Fee rate                                                                                                                                                                                                                                                                                                                                                  |
| »»» feeCurrency    | string         | true     | none         | Currency used to calculate trading fee                                                                                                                                                                                                                                                                                                                    |
| »»» stop           | string         | true     | none         | Take Profit and Stop Loss type, currently HFT does not support the Take Profit and Stop Loss type, so it is empty                                                                                                                                                                                                                                         |
| »»» tradeType      | string         | true     | none         | Trade type, redundancy param                                                                                                                                                                                                                                                                                                                              |
| »»» tax            | string         | true     | none         | Users in some regions have this field                                                                                                                                                                                                                                                                                                                     |
| »»» taxRate        | string         | true     | none         | Tax Rate: Users in some regions must query this field                                                                                                                                                                                                                                                                                                     |
| »»» type           | string         | true     | none         | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                |
| »»» createdAt      | integer(int64) | true     | none         | none                                                                                                                                                                                                                                                                                                                                                      |
| »» lastId          | integer(int64) | true     | none         | The ID of the last set of data from the previous data batch. By default, the latest information is given.<br>lastId is used to filter data and paginate. If lastId is not entered, the default is a maximum of 100 returned data items. The return results include lastId, which can be used as a query parameter to look up new data from the next page. |

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

## Add Order - V1

<a id="opId012"></a>

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
      ]
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
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": [
        "cross",
        "isolated"
      ],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
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

fetch('/api/v1/margin/order',
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

r = requests.post('/api/v1/margin/order', headers = headers)

print(r.json())

```

`POST /api/v1/margin/order`

Place order in the Cross-margin or Isolated-margin trading system. You can place
two major types of order: Limit and market. Orders can only be placed if your
account has sufficient funds. Once an order is placed, your funds will be put on
hold for the duration of the order. The amount of funds on hold depends on the
order type and parameters specified.

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
      ]
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
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": ["cross", "isolated"],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order---v1-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/doc-338146) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                       |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/doc-338146) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                          |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                     |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |
| » marginModel | body | string         | false    | The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.                                                                                                                                                                                                                                                                                                                                                                 |

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

| Parameter     | Value    |
| ------------- | -------- |
| » side        | buy      |
| » side        | sell     |
| » type        | limit    |
| » type        | market   |
| » stp         | DC       |
| » stp         | CO       |
| » stp         | CN       |
| » stp         | CB       |
| » timeInForce | GTC      |
| » timeInForce | GTT      |
| » timeInForce | IOC      |
| » timeInForce | FOK      |
| » marginModel | cross    |
| » marginModel | isolated |

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
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "This return value is invalid"
        }
      },
      "required": ["orderId", "loanApplyId", "borrowSize", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order---v1-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order---v1-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | This return value is invalid                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

## Add Order Test - V1

<a id="opId013"></a>

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": [
        "cross",
        "isolated"
      ],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
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

fetch('/api/v1/margin/order/test',
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

r = requests.post('/api/v1/margin/order/test', headers = headers)

print(r.json())

```

`POST /api/v1/margin/order/test`

Order test endpoint: This endpoint’s request and return parameters are identical
to the order endpoint, and can be used to verify whether the signature is
correct, among other operations. After placing an order, the order will not
enter the matching system, and the order cannot be queried.

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
      ]
    },
    "stp": {
      "type": "string",
      "description": "[Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC",
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
      "description": "[Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading",
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
      "description": "Cancel after n seconds, the order timing strategy is GTT"
    },
    "funds": {
      "type": "string",
      "description": "When **type** is market, select one out of two: size or funds"
    },
    "autoBorrow": {
      "type": "boolean",
      "description": "When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.",
      "default": false
    },
    "autoRepay": {
      "type": "boolean",
      "description": "AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.",
      "default": false
    },
    "marginModel": {
      "type": "string",
      "description": "The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.",
      "enum": ["cross", "isolated"],
      "default": "cross",
      "x-api-enum": [
        {
          "value": "cross",
          "name": "",
          "description": ""
        },
        {
          "value": "isolated",
          "name": "",
          "description": ""
        }
      ]
    }
  },
  "required": ["symbol", "side", "clientOid"]
}
```

<h3 id="add-order-test---v1-parameters">Parameters</h3>

| Name          | In   | Type           | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------- | ---- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| body          | body | object         | false    | none                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » clientOid   | body | string         | true     | Client Order ID: The ClientOid field is a unique ID created by the user (we recommend using a UUID), and can only contain numbers, letters, underscores (\_), and hyphens (-). This field is returned when order information is obtained. You can use clientOid to tag your orders. ClientOid is different from the order ID created by the service provider. Please do not initiate requests using the same clientOid. The maximum length for the ClientOid is 40 characters. |
| » side        | body | string         | true     | Specify if the order is to 'buy' or 'sell'.                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » symbol      | body | string         | true     | symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| » type        | body | string         | false    | Specify if the order is a 'limit' order or 'market' order.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » stp         | body | string         | false    | [Self Trade Prevention](https://www.kucoin.com/docs-new/api-5176570) is divided into four strategies: CN, CO, CB , and DC                                                                                                                                                                                                                                                                                                                                                      |
| » price       | body | string         | false    | Specify price for order                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » size        | body | string         | false    | Specify quantity for order.                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » timeInForce | body | string         | false    | [Time in force](https://www.kucoin.com/docs-new/api-5176570) is a special strategy used during trading                                                                                                                                                                                                                                                                                                                                                                         |
| » postOnly    | body | boolean        | false    | passive order labels, this is disabled when the order timing strategy is IOC or FOK                                                                                                                                                                                                                                                                                                                                                                                            |
| » hidden      | body | boolean        | false    | Hidden or not (not shown in order book)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| » iceberg     | body | boolean        | false    | Whether or not only visible portions of orders are shown in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                     |
| » visibleSize | body | string         | false    | Maximum visible quantity in iceberg orders                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » cancelAfter | body | integer(int64) | false    | Cancel after n seconds, the order timing strategy is GTT                                                                                                                                                                                                                                                                                                                                                                                                                       |
| » funds       | body | string         | false    | When **type** is market, select one out of two: size or funds                                                                                                                                                                                                                                                                                                                                                                                                                  |
| » autoBorrow  | body | boolean        | false    | When Margin Account has inefficient balance, our system autoborrows inefficient assets and opens positions according to the lowest market interest rate.                                                                                                                                                                                                                                                                                                                       |
| » autoRepay   | body | boolean        | false    | AutoPay allows the return of borrowed assets when you close a position. Our system automatically triggers the repayment and the maximum repayment amount equals to the filled-order amount.                                                                                                                                                                                                                                                                                    |
| » marginModel | body | string         | false    | The type of trading, including cross (cross mode) and isolated (isolated mode). It is set at cross by default.                                                                                                                                                                                                                                                                                                                                                                 |

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

| Parameter     | Value    |
| ------------- | -------- |
| » side        | buy      |
| » side        | sell     |
| » type        | limit    |
| » type        | market   |
| » stp         | DC       |
| » stp         | CO       |
| » stp         | CN       |
| » stp         | CB       |
| » timeInForce | GTC      |
| » timeInForce | GTT      |
| » timeInForce | IOC      |
| » timeInForce | FOK      |
| » marginModel | cross    |
| » marginModel | isolated |

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
        "loanApplyId": {
          "type": "string",
          "description": "Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "borrowSize": {
          "type": "string",
          "description": "Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow."
        },
        "clientOid": {
          "type": "string",
          "description": "This return value is invalid"
        }
      },
      "required": ["orderId", "loanApplyId", "borrowSize", "clientOid"]
    }
  },
  "required": ["code", "data"]
}
```

<h3 id="add-order-test---v1-responses">Responses</h3>

| Status | Meaning                                                 | Description | Schema |
| ------ | ------------------------------------------------------- | ----------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | Inline |

<h3 id="add-order-test---v1-responseschema">Response Schema</h3>

Status Code **200**

| Name           | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| » code         | string | true     | none         | none                                                                                                                          |
| » data         | object | true     | none         | none                                                                                                                          |
| »» orderId     | string | true     | none         | The unique order ID generated by the trading system, which can be used later for further actions such as canceling the order. |
| »» loanApplyId | string | true     | none         | Borrow order ID. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» borrowSize  | string | true     | none         | Borrowed amount. The field is returned only after placing the order under the mode of Auto-Borrow.                            |
| »» clientOid   | string | true     | none         | This return value is invalid                                                                                                  |

<aside class="success">
This operation does not require authentication
</aside>

# Schemas
