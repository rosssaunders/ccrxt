# [#](#collateral-loan) Collateral-loan

Collateral loan, only for single currency collateral

## [#](#place-collateral-loan-order) Place collateral loan order

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#place-collateral-loan-order](https://www.gate.io/docs/developers/apiv4/en/#place-collateral-loan-order)

> Code samples

`POST /loan/collateral/orders`

_Place collateral loan order_

> Body parameter

```
{
  "collateral_amount": "1",
  "collateral_currency": "BTC",
  "borrow_amount": "49",
  "borrow_currency": "USDT"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-parameters](https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-parameters)

| Name                  | In   | Type   | Required | Description         |
| --------------------- | ---- | ------ | -------- | ------------------- |
| body                  | body | object | true     | none                |
| » collateral_amount   | body | string | true     | Collateral amount   |
| » collateral_currency | body | string | true     | Collateral currency |
| » borrow_amount       | body | string | true     | Borrowed amount     |
| » borrow_currency     | body | string | true     | Borrowed currency   |

> Example responses

> 200 Response

```
{
  "order_id": 10005578
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-responses](https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-responses)

| Status | Meaning                                                                    | Description               | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order placed successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createcollateralloan-responseschema)

Status Code **200**

| Name       | Type           | Description |
| ---------- | -------------- | ----------- |
| » order_id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-collateral-loan-order-list) Query collateral loan order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-order-list)

> Code samples

`GET /loan/collateral/orders`

_Query collateral loan order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-parameters)

| Name                | In    | Type           | Required | Description                                         |
| ------------------- | ----- | -------------- | -------- | --------------------------------------------------- |
| page                | query | integer(int32) | false    | Page number                                         |
| limit               | query | integer        | false    | Maximum number of records returned in a single list |
| collateral_currency | query | string         | false    | Collateral currency                                 |
| borrow_currency     | query | string         | false    | Borrowed currency                                   |

> Example responses

> 200 Response

```
[
  {
    "order_id": 10000421,
    "collateral_currency": "BTC",
    "borrow_currency": "USDT",
    "collateral_amount": "1",
    "borrow_amount": "1000",
    "repaid_amount": "10",
    "repaid_principal": "10",
    "repaid_interest": "0",
    "init_ltv": "0.0003934533764831",
    "current_ltv": "0.0004521768651985",
    "liquidate_ltv": "0.9",
    "status": "initial_status",
    "borrow_time": 1688462668,
    "left_repay_total": "990.0219384",
    "left_repay_interest": "0.0219384"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcollateralloanorders-responseschema)

Status Code **200**

| Name                   | Type           | Description                        |
| ---------------------- | -------------- | ---------------------------------- |
| _None_                 | array          | \[Collateral order\]               |
| » _None_               | object         | Collateral order                   |
| »» order_id            | integer(int64) | Order ID                           |
| »» collateral_currency | string         | Collateral currency                |
| »» collateral_amount   | string         | Collateral amount                  |
| »» borrow_currency     | string         | Borrowed currency                  |
| »» borrow_amount       | string         | Borrowed amount                    |
| »» repaid_amount       | string         | Repaid amount                      |
| »» repaid_principal    | string         | Repaid principal                   |
| »» repaid_interest     | string         | Repaid interest                    |
| »» init_ltv            | string         | Initial collateralization rate     |
| »» current_ltv         | string         | Current collateralization rate     |
| »» liquidate_ltv       | string         | Liquidation collateralization rate |
| »» status              | string         | Order status:                      |

\- initial: Initial state after placing the order  
\- collateral_deducted: Collateral deduction successful  
\- collateral_returning: Loan failed - Collateral return pending  
\- lent: Loan successful  
\- repaying: Repayment in progress  
\- liquidating: Liquidation in progress  
\- finished: Order completed  
\- closed_liquidated: Liquidation and repayment completed | | »» borrow_time |
integer(int64) | Borrowing time, timestamp in seconds | | »» left_repay_total |
string | Outstanding principal and interest (outstanding principal + outstanding
interest) | | »» left_repay_principal | string | Outstanding principal | | »»
left_repay_interest | string | Outstanding interest |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-single-order-details-5) Query single order details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-5](https://www.gate.io/docs/developers/apiv4/en/#query-single-order-details-5)

> Code samples

`GET /loan/collateral/orders/{order_id}`

_Query single order details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-parameters](https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-parameters)

| Name     | In   | Type           | Required | Description                                          |
| -------- | ---- | -------------- | -------- | ---------------------------------------------------- |
| order_id | path | integer(int64) | true     | Order ID returned when order is successfully created |

> Example responses

> 200 Response

```
{
  "order_id": 10000421,
  "collateral_currency": "BTC",
  "borrow_currency": "USDT",
  "collateral_amount": "1",
  "borrow_amount": "1000",
  "repaid_amount": "10",
  "repaid_principal": "10",
  "repaid_interest": "0",
  "init_ltv": "0.0003934533764831",
  "current_ltv": "0.0004521768651985",
  "liquidate_ltv": "0.9",
  "status": "initial_status",
  "borrow_time": 1688462668,
  "left_repay_total": "990.0219384",
  "left_repay_interest": "0.0219384"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-responses](https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-responses)

| Status | Meaning                                                                    | Description                        | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Order details queried successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getcollateralloanorderdetail-responseschema)

Status Code **200**

_Collateral order_

| Name                  | Type           | Description                        |
| --------------------- | -------------- | ---------------------------------- |
| » order_id            | integer(int64) | Order ID                           |
| » collateral_currency | string         | Collateral currency                |
| » collateral_amount   | string         | Collateral amount                  |
| » borrow_currency     | string         | Borrowed currency                  |
| » borrow_amount       | string         | Borrowed amount                    |
| » repaid_amount       | string         | Repaid amount                      |
| » repaid_principal    | string         | Repaid principal                   |
| » repaid_interest     | string         | Repaid interest                    |
| » init_ltv            | string         | Initial collateralization rate     |
| » current_ltv         | string         | Current collateralization rate     |
| » liquidate_ltv       | string         | Liquidation collateralization rate |
| » status              | string         | Order status:                      |

\- initial: Initial state after placing the order  
\- collateral_deducted: Collateral deduction successful  
\- collateral_returning: Loan failed - Collateral return pending  
\- lent: Loan successful  
\- repaying: Repayment in progress  
\- liquidating: Liquidation in progress  
\- finished: Order completed  
\- closed_liquidated: Liquidation and repayment completed | | » borrow_time |
integer(int64) | Borrowing time, timestamp in seconds | | » left_repay_total |
string | Outstanding principal and interest (outstanding principal + outstanding
interest) | | » left_repay_principal | string | Outstanding principal | | »
left_repay_interest | string | Outstanding interest |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#collateral-loan-repayment) Collateral loan repayment

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#collateral-loan-repayment](https://www.gate.io/docs/developers/apiv4/en/#collateral-loan-repayment)

> Code samples

`POST /loan/collateral/repay`

_Collateral loan repayment_

> Body parameter

```
{
  "order_id": 37438962,
  "repay_amount": "1000",
  "repaid_all": false
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-parameters](https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-parameters)

| Name           | In   | Type           | Required | Description                                                                            |
| -------------- | ---- | -------------- | -------- | -------------------------------------------------------------------------------------- |
| body           | body | object         | true     | none                                                                                   |
| » order_id     | body | integer(int64) | true     | Order ID                                                                               |
| » repay_amount | body | string         | true     | Repayment amount, it is mandatory when making partial repayments                       |
| » repaid_all   | body | boolean        | true     | Repayment method, set to `true` for full repayment, and `false` for partial repayment; |

#### [#](#detailed-descriptions-55) Detailed descriptions

**» repaid_all**: Repayment method, set to `true` for full repayment, and
`false` for partial repayment; When partial repayment, the repay_amount
parameter cannot be greater than the remaining amount to be repaid by the user.

> Example responses

> 200 Response

```
{
  "repaid_principal": "11",
  "repaid_interest": "111"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-responses](https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-responseschema](https://www.gate.io/docs/developers/apiv4/en/#repaycollateralloan-responseschema)

Status Code **200**

_Repay_

| Name               | Type   | Description |
| ------------------ | ------ | ----------- |
| » repaid_principal | string | Principal   |
| » repaid_interest  | string | Interest    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-collateral-loan-repayment-records) Query collateral loan repayment records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-repayment-records](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-loan-repayment-records)

> Code samples

`GET /loan/collateral/repay_records`

_Query collateral loan repayment records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-parameters)

| Name                | In    | Type           | Required | Description                                                            |
| ------------------- | ----- | -------------- | -------- | ---------------------------------------------------------------------- |
| source              | query | string         | true     | Operation type: repay - Regular repayment, liquidate - Liquidation     |
| borrow_currency     | query | string         | false    | Borrowed currency                                                      |
| collateral_currency | query | string         | false    | Collateral currency                                                    |
| page                | query | integer(int32) | false    | Page number                                                            |
| limit               | query | integer        | false    | Maximum number of records returned in a single list                    |
| from                | query | integer(int64) | false    | Start timestamp for the query                                          |
| to                  | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified |

> Example responses

> 200 Response

```
[
  {
    "order_id": 10000425,
    "record_id": 181,
    "repaid_amount": "10.00000000000000000000",
    "borrow_currency": "USDT",
    "collateral_currency": "BTC",
    "collateral_amount": "1.00000000000000000000",
    "init_ltv": "0.00039345337648310000",
    "borrow_time": 1688471851,
    "repay_time": 1688526310,
    "total_interest": "0.25446901544300000000",
    "before_left_principal": "11.00000000",
    "pre_left_principal": "990.00000000000000000000",
    "after_left_principal": "990.00000000000000000000",
    "before_left_collateral": "1.00000000000000000000",
    "after_left_collateral": "1.00000000000000000000"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listrepayrecords-responseschema)

Status Code **200**

| Name                      | Type           | Description                             |
| ------------------------- | -------------- | --------------------------------------- |
| » _None_                  | object         | Repayment record                        |
| »» order_id               | integer(int64) | Order ID                                |
| »» record_id              | integer(int64) | Repayment record ID                     |
| »» repaid_amount          | string         | Repayment amount                        |
| »» borrow_currency        | string         | Borrowed currency                       |
| »» collateral_currency    | string         | Collateral currency                     |
| »» init_ltv               | string         | Initial collateralization rate          |
| »» borrow_time            | integer(int64) | Borrowing time, timestamp               |
| »» repay_time             | integer(int64) | Repayment time, timestamp               |
| »» total_interest         | string         | Total interest                          |
| »» before_left_principal  | string         | Principal to be repaid before repayment |
| »» after_left_principal   | string         | Principal to be repaid after repayment  |
| »» before_left_collateral | string         | Collateral amount before repayment      |
| »» after_left_collateral  | string         | Collateral amount after repayment       |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#increase-or-redeem-collateral) Increase or redeem collateral

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#increase-or-redeem-collateral](https://www.gate.io/docs/developers/apiv4/en/#increase-or-redeem-collateral)

> Code samples

`POST /loan/collateral/collaterals`

_Increase or redeem collateral_

> Body parameter

```
{
  "collateral_amount": "1212",
  "collateral_currency": "BTC",
  "order_id": 1130,
  "type": "append"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#operatecollateral-parameters](https://www.gate.io/docs/developers/apiv4/en/#operatecollateral-parameters)

| Name                  | In   | Type           | Required | Description                                                           |
| --------------------- | ---- | -------------- | -------- | --------------------------------------------------------------------- |
| body                  | body | object         | true     | none                                                                  |
| » order_id            | body | integer(int64) | true     | Order ID                                                              |
| » collateral_currency | body | string         | true     | Collateral currency                                                   |
| » collateral_amount   | body | string         | true     | Collateral amount                                                     |
| » type                | body | string         | true     | Operation type: append - add collateral, redeem - withdraw collateral |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#operatecollateral-responses](https://www.gate.io/docs/developers/apiv4/en/#operatecollateral-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Operation successful | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-collateral-adjustment-records) Query collateral adjustment records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records](https://www.gate.io/docs/developers/apiv4/en/#query-collateral-adjustment-records)

> Code samples

`GET /loan/collateral/collaterals`

_Query collateral adjustment records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-parameters)

| Name                | In    | Type           | Required | Description                                                            |
| ------------------- | ----- | -------------- | -------- | ---------------------------------------------------------------------- |
| page                | query | integer(int32) | false    | Page number                                                            |
| limit               | query | integer        | false    | Maximum number of records returned in a single list                    |
| from                | query | integer(int64) | false    | Start timestamp for the query                                          |
| to                  | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified |
| borrow_currency     | query | string         | false    | Borrowed currency                                                      |
| collateral_currency | query | string         | false    | Collateral currency                                                    |

> Example responses

> 200 Response

```
[
  {
    "order_id": 10000417,
    "record_id": 10000452,
    "borrow_currency": "USDT",
    "borrow_amount": "1000.00000000000000000000",
    "collateral_currency": "BTC",
    "pre_collateral": "1.00000000000000000000",
    "after_collateral": "2.00000000000000000000",
    "pre_ltv": "0.00039345555621480000",
    "after_ltv": "0.00019672777810740000",
    "operate_time": 1688461924
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcollateralrecords-responseschema)

Status Code **200**

| Name                   | Type           | Description                          |
| ---------------------- | -------------- | ------------------------------------ |
| » _None_               | object         | Collateral record                    |
| »» order_id            | integer(int64) | Order ID                             |
| »» record_id           | integer(int64) | Collateral record ID                 |
| »» borrow_currency     | string         | Borrowed currency                    |
| »» borrow_amount       | string         | Borrowed amount                      |
| »» collateral_currency | string         | Collateral currency                  |
| »» before_collateral   | string         | Collateral amount before adjustment  |
| »» after_collateral    | string         | Collateral amount after adjustment   |
| »» before_ltv          | string         | Collateral ratio before adjustment   |
| »» after_ltv           | string         | Collateral ratio after adjustment    |
| »» operate_time        | integer(int64) | Operation time, timestamp in seconds |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-total-borrowing-and-collateral-amount) Query user's total borrowing and collateral amount

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-borrowing-and-collateral-amount](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-borrowing-and-collateral-amount)

> Code samples

`GET /loan/collateral/total_amount`

_Query user's total borrowing and collateral amount_

> Example responses

> 200 Response

```
{
  "borrow_amount": "11",
  "collateral_amount": "111"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getusertotalamount-responses](https://www.gate.io/docs/developers/apiv4/en/#getusertotalamount-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getusertotalamount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getusertotalamount-responseschema)

Status Code **200**

_User's total borrowing and collateral amount_

| Name                | Type   | Description                     |
| ------------------- | ------ | ------------------------------- |
| » borrow_amount     | string | Total borrowing amount in USDT  |
| » collateral_amount | string | Total collateral amount in USDT |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-collateralization-ratio-and-remaining-borrowable-currencies) Query user's collateralization ratio and remaining borrowable currencies

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-collateralization-ratio-and-remaining-borrowable-currencies](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-collateralization-ratio-and-remaining-borrowable-currencies)

> Code samples

`GET /loan/collateral/ltv`

_Query user's collateralization ratio and remaining borrowable currencies_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-parameters](https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-parameters)

| Name                | In    | Type   | Required | Description         |
| ------------------- | ----- | ------ | -------- | ------------------- |
| collateral_currency | query | string | true     | Collateral currency |
| borrow_currency     | query | string | true     | Borrowed currency   |

> Example responses

> 200 Response

```
{
  "collateral_currency": "BTC",
  "borrow_currency": "USDT",
  "init_ltv": "0.7",
  "alert_ltv": "0.8",
  "liquidate_ltv": "0.9",
  "min_borrow_amount": "3",
  "left_borrowable_amount": "4233030.635065916703"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-responses](https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getuserltvinfo-responseschema)

Status Code **200**

_User's currency statistics data_

| Name                     | Type   | Description                                       |
| ------------------------ | ------ | ------------------------------------------------- |
| » collateral_currency    | string | Collateral currency                               |
| » borrow_currency        | string | Borrowed currency                                 |
| » init_ltv               | string | Initial collateralization rate                    |
| » alert_ltv              | string | Warning collateralization rate                    |
| » liquidate_ltv          | string | Liquidation collateralization rate                |
| » min_borrow_amount      | string | Minimum borrowable amount for the loan currency   |
| » left_borrowable_amount | string | Remaining borrowable amount for the loan currency |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-supported-borrowing-and-collateral-currencies) Query supported borrowing and collateral currencies

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies](https://www.gate.io/docs/developers/apiv4/en/#query-supported-borrowing-and-collateral-currencies)

> Code samples

`GET /loan/collateral/currencies`

_Query supported borrowing and collateral currencies_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-parameters)

| Name          | In    | Type   | Required | Description                                                                                                                                                                    |
| ------------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| loan_currency | query | string | false    | Parameter loan_currency. If omitted, returns all supported borrowing currencies; if provided, returns the array of collateral currencies supported for that borrowing currency |

> Example responses

> 200 Response

```
[
  {
    "loan_currency": "BTC",
    "collateral_currency": [
      "BTC",
      "ETH",
      "GT"
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-responses](https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcollateralcurrencies-responseschema)

Status Code **200**

| Name                   | Type   | Description                                   |
| ---------------------- | ------ | --------------------------------------------- |
| » _None_               | object | Supported borrowing and collateral currencies |
| »» loan_currency       | string | Borrowed currency                             |
| »» collateral_currency | array  | List of supported collateral currencies       |

This operation does not require authentication
