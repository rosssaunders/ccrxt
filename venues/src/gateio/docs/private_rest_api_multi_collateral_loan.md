# [#](#multi-collateral-loan) Multi-collateral-loan

Multi-currency collateral

## [#](#place-multi-currency-collateral-order) Place multi-currency collateral order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-multi-currency-collateral-order](https://www.gate.io/docs/developers/apiv4/en/#place-multi-currency-collateral-order)

> Code samples

`POST /loan/multi_collateral/orders`

_Place multi-currency collateral order_

> Body parameter

```
{
  "order_id": 1721387470,
  "order_type": "fixed",
  "fixed_type": "7d",
  "fixed_rate": 0.00001,
  "auto_renew": true,
  "auto_repay": true,
  "borrow_currency": "BTC",
  "borrow_amount": "1",
  "collateral_currencies": [
    {
      "currency": "USDT",
      "amount": "1000"
    }
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-parameters](https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-parameters)

| Name                    | In   | Type    | Required | Description                                                                             |
| ----------------------- | ---- | ------- | -------- | --------------------------------------------------------------------------------------- |
| body                    | body | object  | true     | none                                                                                    |
| » order_id              | body | string  | false    | Order ID                                                                                |
| » order_type            | body | string  | false    | current - current rate, fixed - fixed rate, defaults to current if not specified        |
| » fixed_type            | body | string  | false    | Fixed interest rate lending period: 7d - 7 days, 30d - 30 days. Required for fixed rate |
| » fixed_rate            | body | string  | false    | Fixed interest rate, required for fixed rate                                            |
| » auto_renew            | body | boolean | false    | Fixed interest rate, auto-renewal                                                       |
| » auto_repay            | body | boolean | false    | Fixed interest rate, auto-repayment                                                     |
| » borrow_currency       | body | string  | true     | Borrowed currency                                                                       |
| » borrow_amount         | body | string  | true     | Borrowed amount                                                                         |
| » collateral_currencies | body | array   | false    | Collateral currency and amount                                                          |
| »» CollateralCurrency   | body | object  | false    | none                                                                                    |
| »»» currency            | body | string  | false    | Currency                                                                                |
| »»» amount              | body | string  | false    | Amount                                                                                  |

> Example responses

> 200 Response

```
{
  "order_id": 10005578
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-responses](https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-responses)

| Status | Meaning                                                                    | Description               | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order placed successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createmulticollateral-responseschema)

Status Code **200**

| Name       | Type           | Description |
| ---------- | -------------- | ----------- |
| » order_id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-multi-currency-collateral-order-list) Query multi-currency collateral order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-multi-currency-collateral-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-multi-currency-collateral-order-list)

> Code samples

`GET /loan/multi_collateral/orders`

_Query multi-currency collateral order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-parameters)

| Name       | In    | Type    | Required | Description                                                                                                                                   |
| ---------- | ----- | ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| page       | query | integer | false    | Page number                                                                                                                                   |
| limit      | query | integer | false    | Maximum number of records returned in a single list                                                                                           |
| sort       | query | string  | false    | Sort type: `time_desc` - Created time descending (default), `ltv_asc` - Collateral ratio ascending, `ltv_desc` - Collateral ratio descending. |
| order_type | query | string  | false    | Order type: current - Query current orders, fixed - Query fixed orders, defaults to current orders if not specified                           |

> Example responses

> 200 Response

```
[
  {
    "order_id": "10005578",
    "order_type": "fixed",
    "fixed_type": "7d",
    "fixed_rate": 0.00001,
    "expire_time": 1703820105,
    "auto_renew": true,
    "auto_repay": true,
    "current_ltv": "0.0001004349664281",
    "status": "lent",
    "borrow_time": 1702615021,
    "total_left_repay_usdt": "106.491212982",
    "total_left_collateral_usdt": "1060300.18",
    "borrow_currencies": [
      {
        "currency": "GT",
        "index_price": "10.6491",
        "left_repay_principal": "10",
        "left_repay_interest": "0.00002",
        "left_repay_usdt": "106.491212982"
      }
    ],
    "collateral_currencies": [
      {
        "currency": "BTC",
        "index_price": "112794.7",
        "left_collateral": "9.4",
        "left_collateral_usdt": "1060270.18"
      },
      {
        "currency": "USDT",
        "index_price": "1",
        "left_collateral": "30",
        "left_collateral_usdt": "30"
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralorders-responseschema)

Status Code **200**

| Name           | Type           | Description                                                  |
| -------------- | -------------- | ------------------------------------------------------------ |
| _None_         | array          | \[Multi-Collateral Order\]                                   |
| » _None_       | object         | Multi-Collateral Order                                       |
| »» order_id    | string         | Order ID                                                     |
| »» order_type  | string         | current - current, fixed - fixed                             |
| »» fixed_type  | string         | Fixed interest rate loan periods: 7d - 7 days, 30d - 30 days |
| »» fixed_rate  | string         | Fixed interest rate                                          |
| »» expire_time | integer(int64) | Expiration time, timestamp, unit in seconds                  |
| »» auto_renew  | boolean        | Fixed interest rate, auto-renewal                            |
| »» auto_repay  | boolean        | Fixed interest rate, auto-repayment                          |
| »» current_ltv | string         | Current collateralization rate                               |
| »» status      | string         | Order status:                                                |

\- initial: Initial state after placing the order  
\- collateral_deducted: Collateral deduction successful  
\- collateral_returning: Loan failed - Collateral return pending  
\- lent: Loan successful  
\- repaying: Repayment in progress  
\- liquidating: Liquidation in progress  
\- finished: Order completed  
\- closed_liquidated: Liquidation and repayment completed | | »» borrow_time |
integer(int64) | Borrowing time, timestamp in seconds | | »»
total_left_repay_usdt | string | Total outstanding value converted to USDT | |
»» total_left_collateral_usdt | string | Total collateral value converted to
USDT | | »» borrow_currencies | array | Borrowing Currency List | | »»»
BorrowCurrencyInfo | object | none | | »»»» currency | string | Currency | |
»»»» index_price | string | Currency Index Price | | »»»» left_repay_principal |
string | Outstanding principal | | »»»» left_repay_interest | string |
Outstanding interest | | »»»» left_repay_usdt | string | Remaining total
outstanding value converted to USDT | | »»» collateral_currencies | array |
Collateral Currency List | | »»»» CollateralCurrencyInfo | object | none | |
»»»»» currency | string | Currency | | »»»»» index_price | string | Currency
Index Price | | »»»»» left_collateral | string | Remaining collateral amount | |
»»»»» left_collateral_usdt | string | Remaining collateral value converted to
USDT |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-order-details) Query order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-order-details](https://www.gate.io/docs/developers/apiv4/en/#query-order-details)

> Code samples

`GET /loan/multi_collateral/orders/{order_id}`

_Query order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-parameters](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-parameters)

| Name     | In   | Type   | Required | Description                                          |
| -------- | ---- | ------ | -------- | ---------------------------------------------------- |
| order_id | path | string | true     | Order ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "order_id": "10005578",
  "order_type": "fixed",
  "fixed_type": "7d",
  "fixed_rate": 0.00001,
  "expire_time": 1703820105,
  "auto_renew": true,
  "auto_repay": true,
  "current_ltv": "0.0001004349664281",
  "status": "lent",
  "borrow_time": 1702615021,
  "total_left_repay_usdt": "106.491212982",
  "total_left_collateral_usdt": "1060300.18",
  "borrow_currencies": [
    {
      "currency": "GT",
      "index_price": "10.6491",
      "left_repay_principal": "10",
      "left_repay_interest": "0.00002",
      "left_repay_usdt": "106.491212982"
    }
  ],
  "collateral_currencies": [
    {
      "currency": "BTC",
      "index_price": "112794.7",
      "left_collateral": "9.4",
      "left_collateral_usdt": "1060270.18"
    },
    {
      "currency": "USDT",
      "index_price": "1",
      "left_collateral": "30",
      "left_collateral_usdt": "30"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-responses](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-responses)

| Status | Meaning                                                                    | Description                        | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details queried successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralorderdetail-responseschema)

Status Code **200**

_Multi-Collateral Order_

| Name          | Type           | Description                                                  |
| ------------- | -------------- | ------------------------------------------------------------ |
| » order_id    | string         | Order ID                                                     |
| » order_type  | string         | current - current, fixed - fixed                             |
| » fixed_type  | string         | Fixed interest rate loan periods: 7d - 7 days, 30d - 30 days |
| » fixed_rate  | string         | Fixed interest rate                                          |
| » expire_time | integer(int64) | Expiration time, timestamp, unit in seconds                  |
| » auto_renew  | boolean        | Fixed interest rate, auto-renewal                            |
| » auto_repay  | boolean        | Fixed interest rate, auto-repayment                          |
| » current_ltv | string         | Current collateralization rate                               |
| » status      | string         | Order status:                                                |

\- initial: Initial state after placing the order  
\- collateral_deducted: Collateral deduction successful  
\- collateral_returning: Loan failed - Collateral return pending  
\- lent: Loan successful  
\- repaying: Repayment in progress  
\- liquidating: Liquidation in progress  
\- finished: Order completed  
\- closed_liquidated: Liquidation and repayment completed | | » borrow_time |
integer(int64) | Borrowing time, timestamp in seconds | | »
total_left_repay_usdt | string | Total outstanding value converted to USDT | | »
total_left_collateral_usdt | string | Total collateral value converted to USDT |
| » borrow_currencies | array | Borrowing Currency List | | »»
BorrowCurrencyInfo | object | none | | »»» currency | string | Currency | | »»»
index_price | string | Currency Index Price | | »»» left_repay_principal |
string | Outstanding principal | | »»» left_repay_interest | string |
Outstanding interest | | »»» left_repay_usdt | string | Remaining total
outstanding value converted to USDT | | »» collateral_currencies | array |
Collateral Currency List | | »»» CollateralCurrencyInfo | object | none | | »»»»
currency | string | Currency | | »»»» index_price | string | Currency Index
Price | | »»»» left_collateral | string | Remaining collateral amount | | »»»»
left_collateral_usdt | string | Remaining collateral value converted to USDT |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#multi-currency-collateral-repayment) Multi-currency collateral repayment

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#multi-currency-collateral-repayment](https://www.gate.io/docs/developers/apiv4/en/#multi-currency-collateral-repayment)

> Code samples

`POST /loan/multi_collateral/repay`

_Multi-currency collateral repayment_

> Body parameter

```
{
  "order_id": 10005578,
  "repay_items": [
    {
      "currency": "btc",
      "amount": "1",
      "repaid_all": false
    }
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-parameters](https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-parameters)

| Name                  | In   | Type           | Required | Description                                                                   |
| --------------------- | ---- | -------------- | -------- | ----------------------------------------------------------------------------- |
| body                  | body | object         | true     | none                                                                          |
| » order_id            | body | integer(int64) | true     | Order ID                                                                      |
| » repay_items         | body | array          | true     | Repay Currency Item                                                           |
| »» MultiLoanRepayItem | body | object         | false    | none                                                                          |
| »»» currency          | body | string         | false    | Repayment currency                                                            |
| »»» amount            | body | string         | false    | Amount                                                                        |
| »»» repaid_all        | body | boolean        | false    | Repayment method, set to true for full repayment, false for partial repayment |

> Example responses

> 200 Response

```
{
  "order_id": 10005679,
  "repaid_currencies": [
    {
      "succeeded": false,
      "label": "INVALID_PARAM_VALUE",
      "message": "Invalid parameter value",
      "currency": "BTC",
      "repaid_principal": "1",
      "repaid_interest": "0.0001"
    },
    {
      "succeeded": true,
      "currency": "BTC",
      "repaid_principal": "1",
      "repaid_interest": "0.0001"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-responses](https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-responseschema](https://www.gate.io/docs/developers/apiv4/en/#repaymulticollateralloan-responseschema)

Status Code **200**

_Multi-currency collateral repayment_

| Name                 | Type           | Description                                                    |
| -------------------- | -------------- | -------------------------------------------------------------- |
| » order_id           | integer(int64) | Order ID                                                       |
| » repaid_currencies  | array          | Repay Currency List                                            |
| »» RepayCurrencyRes  | object         | none                                                           |
| »»» succeeded        | boolean        | Whether the repayment was successful                           |
| »»» label            | string         | Error identifier for failed operations; empty when successful  |
| »»» message          | string         | Error description for failed operations; empty when successful |
| »»» currency         | string         | Repayment currency                                             |
| »»» repaid_principal | string         | Principal                                                      |
| »»» repaid_interest  | string         | Principal                                                      |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-multi-currency-collateral-repayment-records) Query multi-currency collateral repayment records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-multi-currency-collateral-repayment-records](https://www.gate.io/docs/developers/apiv4/en/#query-multi-currency-collateral-repayment-records)

> Code samples

`GET /loan/multi_collateral/repay`

_Query multi-currency collateral repayment records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-parameters)

| Name            | In    | Type           | Required | Description                                                            |
| --------------- | ----- | -------------- | -------- | ---------------------------------------------------------------------- |
| type            | query | string         | true     | Operation type: repay - Regular repayment, liquidate - Liquidation     |
| borrow_currency | query | string         | false    | Borrowed currency                                                      |
| page            | query | integer        | false    | Page number                                                            |
| limit           | query | integer        | false    | Maximum number of records returned in a single list                    |
| from            | query | integer(int64) | false    | Start timestamp for the query                                          |
| to              | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified |

> Example responses

> 200 Response

```
[
  {
    "order_id": 10005679,
    "record_id": 1348,
    "init_ltv": "0.2141",
    "before_ltv": "0.215",
    "after_ltv": "0.312",
    "borrow_time": 1702995889,
    "repay_time": 1703053927,
    "borrow_currencies": [
      {
        "currency": "BAT",
        "index_price": "103.02",
        "before_amount": "1",
        "before_amount_usdt": "103.02",
        "after_amount": "0.999017",
        "after_amount_usdt": "102.91873134"
      }
    ],
    "collateral_currencies": [
      {
        "currency": "ETC",
        "index_price": "0.6014228107",
        "before_amount": "1000",
        "before_amount_usdt": "601.4228107",
        "after_amount": "1000",
        "after_amount_usdt": "601.4228107"
      }
    ],
    "repaid_currencies": [
      {
        "currency": "BAT",
        "index_price": "103.02",
        "repaid_amount": "0.001",
        "repaid_principal": "0.000983",
        "repaid_interest": "0.000017",
        "repaid_amount_usdt": "0.10302"
      }
    ],
    "total_interest_list": [
      {
        "currency": "BAT",
        "index_price": "103.02",
        "amount": "0.000017",
        "amount_usdt": "0.00175134"
      }
    ],
    "left_repay_interest_list": [
      {
        "currency": "BAT",
        "index_price": "103.02",
        "before_amount": "0.000017",
        "before_amount_usdt": "0.00175134",
        "after_amount": "0",
        "after_amount_usdt": "0"
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmultirepayrecords-responseschema)

Status Code **200**

| Name                          | Type           | Description                                          |
| ----------------------------- | -------------- | ---------------------------------------------------- |
| » _None_                      | object         | Multi-Collateral Repayment Record                    |
| »» order_id                   | integer(int64) | Order ID                                             |
| »» record_id                  | integer(int64) | Repayment record ID                                  |
| »» init_ltv                   | string         | Initial collateralization rate                       |
| »» before_ltv                 | string         | Ltv before the operation                             |
| »» after_ltv                  | string         | Ltv after the operation                              |
| »» borrow_time                | integer(int64) | Borrowing time, timestamp in seconds                 |
| »» repay_time                 | integer(int64) | Repayment time, timestamp in seconds                 |
| »» borrow_currencies          | array          | List of borrowing information                        |
| »»» currency                  | string         | Currency                                             |
| »»» index_price               | string         | Currency Index Price                                 |
| »»» before_amount             | string         | Amount before the operation                          |
| »»» before_amount_usdt        | string         | USDT Amount before the operation                     |
| »»» after_amount              | string         | Amount after the operation                           |
| »»» after_amount_usdt         | string         | USDT Amount after the operation                      |
| »» collateral_currencies      | array          | List of collateral information                       |
| »»» currency                  | string         | Currency                                             |
| »»» index_price               | string         | Currency Index Price                                 |
| »»» before_amount             | string         | Amount before the operation                          |
| »»» before_amount_usdt        | string         | USDT Amount before the operation                     |
| »»» after_amount              | string         | Amount after the operation                           |
| »»» after_amount_usdt         | string         | USDT Amount after the operation                      |
| »» repaid_currencies          | array          | Repay Currency List                                  |
| »»» RepayRecordRepaidCurrency | object         | none                                                 |
| »»»» currency                 | string         | Repayment currency                                   |
| »»»» index_price              | string         | Currency Index Price                                 |
| »»»» repaid_amount            | string         | Repayment amount                                     |
| »»»» repaid_principal         | string         | Principal                                            |
| »»»» repaid_interest          | string         | Interest                                             |
| »»»» repaid_amount_usdt       | string         | Repayment amount converted to USDT                   |
| »»» total_interest_list       | array          | Total Interest List                                  |
| »»»» RepayRecordTotalInterest | object         | none                                                 |
| »»»»» currency                | string         | Currency                                             |
| »»»»» index_price             | string         | Currency Index Price                                 |
| »»»»» amount                  | string         | Interest Amount                                      |
| »»»»» amount_usdt             | string         | Interest amount converted to USDT                    |
| »»»» left_repay_interest_list | array          | List of remaining interest to be repaid              |
| »»»»» RepayRecordLeftInterest | object         | none                                                 |
| »»»»»» currency               | string         | Currency                                             |
| »»»»»» index_price            | string         | Currency Index Price                                 |
| »»»»»» before_amount          | string         | Interest amount before repayment                     |
| »»»»»» before_amount_usdt     | string         | Converted value of interest before repayment in USDT |
| »»»»»» after_amount           | string         | Interest amount after repayment                      |
| »»»»»» after_amount_usdt      | string         | Converted value of interest after repayment in USDT  |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#add-or-withdraw-collateral) Add or withdraw collateral

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#add-or-withdraw-collateral](https://www.gate.io/docs/developers/apiv4/en/#add-or-withdraw-collateral)

> Code samples

`POST /loan/multi_collateral/mortgage`

_Add or withdraw collateral_

> Body parameter

```
{
  "order_id": 10005578,
  "type": "append",
  "collaterals": [
    {
      "currency": "btc",
      "amount": "0.5"
    }
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-parameters](https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-parameters)

| Name          | In   | Type           | Required | Description                                                           |
| ------------- | ---- | -------------- | -------- | --------------------------------------------------------------------- |
| body          | body | object         | true     | none                                                                  |
| » order_id    | body | integer(int64) | true     | Order ID                                                              |
| » type        | body | string         | true     | Operation type: append - add collateral, redeem - withdraw collateral |
| » collaterals | body | array          | false    | Collateral currency list                                              |
| »» currency   | body | string         | false    | Currency                                                              |
| »» amount     | body | string         | false    | Amount                                                                |

> Example responses

> 200 Response

```
{
  "order_id": 10005679,
  "collateral_currencies": [
    {
      "succeeded": true,
      "currency": "btc",
      "amount": "0.5"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-responses](https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-responseschema](https://www.gate.io/docs/developers/apiv4/en/#operatemulticollateral-responseschema)

Status Code **200**

_Multi-collateral adjustment result_

| Name                     | Type           | Description                                                     |
| ------------------------ | -------------- | --------------------------------------------------------------- |
| » order_id               | integer(int64) | Order ID                                                        |
| » collateral_currencies  | array          | Collateral currency information                                 |
| »» CollateralCurrencyRes | object         | none                                                            |
| »»» succeeded            | boolean        | Update success status                                           |
| »»» label                | string         | Error identifier for failed operations; empty when successful   |
| »»» message              | string         | Error description for failed operations; empty when successful  |
| »»» currency             | string         | Currency                                                        |
| »»» amount               | string         | Successfully operated collateral quantity; 0 if operation fails |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-collateral-adjustment-records-2) Query collateral adjustment records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records-2](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records-2)

> Code samples

`GET /loan/multi_collateral/mortgage`

_Query collateral adjustment records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-parameters)

| Name                | In    | Type           | Required | Description                                                            |
| ------------------- | ----- | -------------- | -------- | ---------------------------------------------------------------------- |
| page                | query | integer        | false    | Page number                                                            |
| limit               | query | integer        | false    | Maximum number of records returned in a single list                    |
| from                | query | integer(int64) | false    | Start timestamp for the query                                          |
| to                  | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified |
| collateral_currency | query | string         | false    | Collateral currency                                                    |

> Example responses

> 200 Response

```
[
  {
    "order_id": 10000417,
    "record_id": 10000452,
    "before_ltv": "0.00039345555621480000",
    "after_ltv": "0.00019672777810740000",
    "operate_time": 1688461924,
    "borrow_currencies": [
      {
        "currency": "BTC",
        "index_price": "30000",
        "before_amount": "0.1",
        "before_amount_usdt": "1000",
        "after_amount": "0.6",
        "after_amount_usdt": "1006"
      }
    ],
    "collateral_currencies": [
      {
        "currency": "BTC",
        "index_price": "30000",
        "before_amount": "0.1",
        "before_amount_usdt": "1000",
        "after_amount": "0.6",
        "after_amount_usdt": "1006"
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralrecords-responseschema)

Status Code **200**

| Name                     | Type           | Description                          |
| ------------------------ | -------------- | ------------------------------------ |
| » _None_                 | object         | Multi-Collateral adjustment record   |
| »» order_id              | integer(int64) | Order ID                             |
| »» record_id             | integer(int64) | Collateral record ID                 |
| »» before_ltv            | string         | Collateral ratio before adjustment   |
| »» after_ltv             | string         | Collateral ratio before adjustment   |
| »» operate_time          | integer(int64) | Operation time, timestamp in seconds |
| »» borrow_currencies     | array          | Borrowing Currency List              |
| »»» currency             | string         | Currency                             |
| »»» index_price          | string         | Currency Index Price                 |
| »»» before_amount        | string         | Amount before the operation          |
| »»» before_amount_usdt   | string         | USDT Amount before the operation     |
| »»» after_amount         | string         | Amount after the operation           |
| »»» after_amount_usdt    | string         | USDT Amount after the operation      |
| »» collateral_currencies | array          | Collateral Currency List             |
| »»» currency             | string         | Currency                             |
| »»» index_price          | string         | Currency Index Price                 |
| »»» before_amount        | string         | Amount before the operation          |
| »»» before_amount_usdt   | string         | USDT Amount before the operation     |
| »»» after_amount         | string         | Amount after the operation           |
| »»» after_amount_usdt    | string         | USDT Amount after the operation      |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-collateral-and-borrowing-currency-quota-information) Query user's collateral and borrowing currency quota information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-collateral-and-borrowing-currency-quota-information](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-collateral-and-borrowing-currency-quota-information)

> Code samples

`GET /loan/multi_collateral/currency_quota`

_Query user's collateral and borrowing currency quota information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-parameters](https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-parameters)

| Name     | In    | Type   | Required | Description                                                                                                                                                    |
| -------- | ----- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| type     | query | string | true     | Currency type: collateral - Collateral currency, borrow - Borrowing currency                                                                                   |
| currency | query | string | true     | When it is a collateral currency, multiple currencies can be provided separated by commas; when it is a borrowing currency, only one currency can be provided. |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "index_price": "35306.1",
    "min_quota": "0",
    "left_quota": "2768152.4958445218723677",
    "left_quote_usdt": "97732668833.536273678"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-responses](https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listusercurrencyquota-responseschema)

Status Code **200**

| Name               | Type   | Description                                           |
| ------------------ | ------ | ----------------------------------------------------- |
| » _None_           | object | Currency Quota                                        |
| »» currency        | string | Currency                                              |
| »» index_price     | string | Currency Index Price                                  |
| »» min_quota       | string | Minimum borrowing/collateral limit for the currency   |
| »» left_quota      | string | Remaining borrowing/collateral quota for the currency |
| »» left_quote_usdt | string | Remaining currency limit converted to USDT            |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-supported-borrowing-and-collateral-currencies-for-multi-currency-collateral) Query supported borrowing and collateral currencies for multi-currency collateral

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies-for-multi-currency-collateral](https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies-for-multi-currency-collateral)

> Code samples

`GET /loan/multi_collateral/currencies`

_Query supported borrowing and collateral currencies for multi-currency
collateral_

> Example responses

> 200 Response

```
{
  "loan_currencies": [
    {
      "currency": "BTC",
      "price": "1212"
    },
    {
      "currency": "GT",
      "price": "12"
    }
  ],
  "collateral_currencies": [
    {
      "currency": "BTC",
      "index_price": "1212",
      "discount": "0.7"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralcurrencies-responses](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralcurrencies-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralcurrencies-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listmulticollateralcurrencies-responseschema)

Status Code **200**

_Borrowing and collateral currencies supported for Multi-Collateral_

| Name                     | Type   | Description                             |
| ------------------------ | ------ | --------------------------------------- |
| » loan_currencies        | array  | List of supported borrowing currencies  |
| »» MultiLoanItem         | object | none                                    |
| »»» currency             | string | Currency                                |
| »»» price                | string | Latest price of the currency            |
| »» collateral_currencies | array  | List of supported collateral currencies |
| »»» MultiCollateralItem  | object | none                                    |
| »»»» currency            | string | Currency                                |
| »»»» index_price         | string | Currency Index Price                    |
| »»»» discount            | string | Discount                                |

This operation does not require authentication

## [#](#query-collateralization-ratio-information) Query collateralization ratio information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-collateralization-ratio-information](https://www.gate.io/docs/developers/apiv4/en/#query-collateralization-ratio-information)

> Code samples

`GET /loan/multi_collateral/ltv`

_Query collateralization ratio information_

Multi-currency collateral ratio is fixed, independent of currency

> Example responses

> 200 Response

```
{
  "init_ltv": "0.7",
  "alert_ltv": "0.8",
  "liquidate_ltv": "0.9"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralltv-responses](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralltv-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralltv-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralltv-responseschema)

Status Code **200**

_Multi-collateral ratio_

| Name            | Type   | Description                        |
| --------------- | ------ | ---------------------------------- |
| » init_ltv      | string | Initial collateralization rate     |
| » alert_ltv     | string | Warning collateralization rate     |
| » liquidate_ltv | string | Liquidation collateralization rate |

This operation does not require authentication

## [#](#query-currency-s-7-day-and-30-day-fixed-interest-rates) Query currency's 7-day and 30-day fixed interest rates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-currency-s-7-day-and-30-day-fixed-interest-rates](https://www.gate.io/docs/developers/apiv4/en/#query-currency-s-7-day-and-30-day-fixed-interest-rates)

> Code samples

`GET /loan/multi_collateral/fixed_rate`

_Query currency's 7-day and 30-day fixed interest rates_

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "rate_7d": "0.000023",
    "rate_30d": "0.1",
    "update_time": 1703820105
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralfixrate-responses](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralfixrate-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralfixrate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralfixrate-responseschema)

Status Code **200**

| Name           | Type           | Description                                   |
| -------------- | -------------- | --------------------------------------------- |
| » _None_       | object         | Multi-collateral fixed interest rate          |
| »» currency    | string         | Currency                                      |
| »» rate_7d     | string         | Fixed interest rate for 7-day lending period  |
| »» rate_30d    | string         | Fixed interest rate for 30-day lending period |
| »» update_time | integer(int64) | Update time, timestamp in seconds             |

This operation does not require authentication

## [#](#query-currency-s-current-interest-rate) Query currency's current interest rate

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-currency-s-current-interest-rate](https://www.gate.io/docs/developers/apiv4/en/#query-currency-s-current-interest-rate)

> Code samples

`GET /loan/multi_collateral/current_rate`

_Query currency's current interest rate_

Query currency's current interest rate for the previous hour, current interest
rate updates hourly

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-parameters](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-parameters)

| Name       | In    | Type            | Required | Description                                                               |
| ---------- | ----- | --------------- | -------- | ------------------------------------------------------------------------- |
| currencies | query | array\[string\] | true     | Specify currency name query array, separated by commas, maximum 100 items |
| vip_level  | query | string          | false    | VIP level, defaults to 0 if not specified                                 |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "current_rate": "0.000023"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-responses](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getmulticollateralcurrentrate-responseschema)

Status Code **200**

| Name            | Type   | Description                            |
| --------------- | ------ | -------------------------------------- |
| » _None_        | object | Multi-collateral current interest rate |
| »» currency     | string | Currency                               |
| »» current_rate | string | Currency current interest rate         |

This operation does not require authentication
