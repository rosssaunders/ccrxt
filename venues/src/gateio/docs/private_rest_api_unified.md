# [#](#unified) Unified

Unified account

## [#](#get-unified-account-information) Get unified account information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-unified-account-information](https://www.gate.io/docs/developers/apiv4/en/#get-unified-account-information)

> Code samples

`GET /unified/accounts`

_Get unified account information_

The assets of each currency in the account will be adjusted according to their
liquidity, defined by corresponding adjustment coefficients, and then uniformly
converted to USD to calculate the total asset value and position value of the
account.

For specific formulas, please refer to [Margin Formula](#margin-formula)

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | false    | Query by specified currency name |
| sub_uid  | query | string | false    | Sub account user ID              |

> Example responses

> 200 Response

```
{
  "user_id": 10001,
  "locked": false,
  "balances": {
    "ETH": {
      "available": "0",
      "freeze": "0",
      "borrowed": "0.075393666654",
      "negative_liab": "0",
      "futures_pos_liab": "0",
      "equity": "1016.1",
      "total_freeze": "0",
      "total_liab": "0",
      "spot_in_use": "1.111"
    },
    "POINT": {
      "available": "9999999999.017023138734",
      "freeze": "0",
      "borrowed": "0",
      "negative_liab": "0",
      "futures_pos_liab": "0",
      "equity": "12016.1",
      "total_freeze": "0",
      "total_liab": "0",
      "spot_in_use": "12"
    },
    "USDT": {
      "available": "0.00000062023",
      "freeze": "0",
      "borrowed": "0",
      "negative_liab": "0",
      "futures_pos_liab": "0",
      "equity": "16.1",
      "total_freeze": "0",
      "total_liab": "0",
      "spot_in_use": "12"
    }
  },
  "total": "230.94621713",
  "borrowed": "161.66395521",
  "total_initial_margin": "1025.0524665088",
  "total_margin_balance": "3382495.944473949183",
  "total_maintenance_margin": "205.01049330176",
  "total_initial_margin_rate": "3299.827135672679",
  "total_maintenance_margin_rate": "16499.135678363399",
  "total_available_margin": "3381470.892007440383",
  "unified_account_total": "3381470.892007440383",
  "unified_account_total_liab": "0",
  "unified_account_total_equity": "100016.1",
  "leverage": "2",
  "spot_order_loss": "12",
  "spot_hedge": false
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedaccounts-responseschema)

Status Code **200**

| Name                             | Type           | Description                                                                                                                                                                                                                     |
| -------------------------------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » user_id                        | integer(int64) | User ID                                                                                                                                                                                                                         |
| » refresh_time                   | integer(int64) | Last refresh time                                                                                                                                                                                                               |
| » locked                         | boolean        | Whether the account is locked, valid in cross-currency margin/combined margin mode, false in other modes such as single-currency margin mode                                                                                    |
| » balances                       | object         | none                                                                                                                                                                                                                            |
| »» UnifiedBalance                | object         | none                                                                                                                                                                                                                            |
| »»» available                    | string         | Available balance, valid in single currency margin/cross-currency margin/combined margin mode, calculation varies by mode                                                                                                       |
| »»» freeze                       | string         | Locked balance, valid in single currency margin/cross-currency margin/combined margin mode                                                                                                                                      |
| »»» borrowed                     | string         | Borrowed amount, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                                      |
| »»» negative_liab                | string         | Negative balance borrowing, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                           |
| »»» futures_pos_liab             | string         | Contract opening position borrowing currency (abandoned, to be offline field)                                                                                                                                                   |
| »»» equity                       | string         | Equity, valid in single currency margin/cross currency margin/combined margin mode                                                                                                                                              |
| »»» total_freeze                 | string         | Total frozen (deprecated, to be removed)                                                                                                                                                                                        |
| »»» total_liab                   | string         | Total borrowed amount, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                                |
| »»» spot_in_use                  | string         | The amount of spot hedging is valid in the combined margin mode, and is 0 in other margin modes such as single currency and cross-currency margin modes                                                                         |
| »»» funding                      | string         | Uniloan financial management amount, effective when turned on as a unified account margin switch                                                                                                                                |
| »»» funding_version              | string         | Funding version                                                                                                                                                                                                                 |
| »»» cross_balance                | string         | Full margin balance is valid in single currency margin mode, and is 0 in other modes such as cross currency margin/combined margin mode                                                                                         |
| »»» iso_balance                  | string         | Isolated margin balance is valid in single-currency margin mode and is 0 in other modes such as cross-currency margin/combined margin mode                                                                                      |
| »»» im                           | string         | Full-position initial margin is valid in single-currency margin mode and is 0 in other modes such as cross-currency margin/combined margin mode                                                                                 |
| »»» mm                           | string         | Cross margin maintenance margin, valid in single-currency margin mode, 0 in other modes such as cross-currency margin/combined margin mode                                                                                      |
| »»» imr                          | string         | Full-position initial margin rate is valid in single-currency margin mode and is 0 in other modes such as cross-currency margin/combined margin mode                                                                            |
| »»» mmr                          | string         | Full-position maintenance margin rate is valid in single-currency margin mode and is 0 in other modes such as cross-currency margin/combined margin mode                                                                        |
| »»» margin_balance               | string         | Full margin balance is valid in single currency margin mode and is 0 in other modes such as cross currency margin/combined margin mode                                                                                          |
| »»» available_margin             | string         | Cross margin available balance, valid in single currency margin mode, 0 in other modes such as cross-currency margin/combined margin mode                                                                                       |
| »»» enabled_collateral           | boolean        | Currency enabled as margin: true - Enabled, false - Disabled                                                                                                                                                                    |
| »» total                         | string         | Total account assets converted to USD, i.e. the sum of `(available + freeze) * price` in all currencies (deprecated, to be removed, replaced by unified_account_total)                                                          |
| »» borrowed                      | string         | Total borrowed amount converted to USD, i.e. the sum of `borrowed * price` of all currencies (excluding point cards), valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode |
| »» total_initial_margin          | string         | Total initial margin, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                                 |
| »» total_margin_balance          | string         | Total margin balance, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                                 |
| »» total_maintenance_margin      | string         | Total maintenance margin is valid in cross-currency margin/combined margin mode, and is 0 in other modes such as single-currency margin mode                                                                                    |
| »» total_initial_margin_rate     | string         | Total initial margin rate, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                            |
| »» total_maintenance_margin_rate | string         | Total maintenance margin rate, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                        |
| »» total_available_margin        | string         | Available margin amount, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                              |
| »» unified_account_total         | string         | Total unified account assets, valid in single currency margin/cross-currency margin/combined margin mode                                                                                                                        |
| »» unified_account_total_liab    | string         | Total unified account borrowed amount, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                |
| »» unified_account_total_equity  | string         | Total unified account equity, valid in single currency margin/cross-currency margin/combined margin mode                                                                                                                        |
| »» leverage                      | string         | Actual leverage ratio, valid in cross-currency margin/combined margin mode                                                                                                                                                      |
| »» spot_order_loss               | string         | Total pending order loss, in USDT, valid in cross-currency margin/combined margin mode, 0 in other modes such as single-currency margin mode                                                                                    |
| »» spot_hedge                    | boolean        | Spot hedging status: true - enabled, false - disabled                                                                                                                                                                           |
| »» use_funding                   | boolean        | Whether to use Earn funds as margin                                                                                                                                                                                             |
| »» is_all_collateral             | boolean        | Whether all currencies are used as margin: true - all currencies as margin, false - no                                                                                                                                          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-maximum-borrowable-amount-for-unified-account) Query maximum borrowable amount for unified account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-maximum-borrowable-amount-for-unified-account](https://www.gate.io/docs/developers/apiv4/en/#query-maximum-borrowable-amount-for-unified-account)

> Code samples

`GET /unified/borrowable`

_Query maximum borrowable amount for unified account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | true     | Query by specified currency name |

> Example responses

> 200 Response

```
{
  "currency": "ETH",
  "amount": "10000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowable-responseschema)

Status Code **200**

_UnifiedBorrowable_

| Name       | Type   | Description           |
| ---------- | ------ | --------------------- |
| » currency | string | Currency detail       |
| » amount   | string | Max borrowable amount |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-maximum-transferable-amount-for-unified-account) Query maximum transferable amount for unified account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-maximum-transferable-amount-for-unified-account](https://www.gate.io/docs/developers/apiv4/en/#query-maximum-transferable-amount-for-unified-account)

> Code samples

`GET /unified/transferable`

_Query maximum transferable amount for unified account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | true     | Query by specified currency name |

> Example responses

> 200 Response

```
{
  "currency": "ETH",
  "amount": "10000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferable-responseschema)

Status Code **200**

_UnifiedTransferable_

| Name       | Type   | Description                 |
| ---------- | ------ | --------------------------- |
| » currency | string | Currency detail             |
| » amount   | string | Maximum transferable amount |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-query-maximum-transferable-amount-for-unified-accounts-each-currency-shows-the-maximum-value-after-user-withdrawal-the-transferable-amount-for-all-currencies-will-change) Batch query maximum transferable amount for unified accounts. Each currency shows the maximum value. After user withdrawal, the transferable amount for all currencies will change

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-query-maximum-transferable-amount-for-unified-accounts-each-currency-shows-the-maximum-value-after-user-withdrawal-the-transferable-amount-for-all-currencies-will-change](https://www.gate.io/docs/developers/apiv4/en/#batch-query-maximum-transferable-amount-for-unified-accounts-each-currency-shows-the-maximum-value-after-user-withdrawal-the-transferable-amount-for-all-currencies-will-change)

> Code samples

`GET /unified/transferables`

_Batch query maximum transferable amount for unified accounts. Each currency
shows the maximum value. After user withdrawal, the transferable amount for all
currencies will change_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-parameters)

| Name       | In    | Type   | Required | Description                                                                                    |
| ---------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------- |
| currencies | query | string | true     | Specify the currency name to query in batches, and support up to 100 pass parameters at a time |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "amount": "123456"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedtransferables-responseschema)

Status Code **200**

| Name                  | Type   | Description                                              |
| --------------------- | ------ | -------------------------------------------------------- |
| » TransferablesResult | object | Batch query unified account maximum transferable results |
| »» currency           | string | Currency detail                                          |
| »» amount             | string | Maximum transferable amount                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#batch-query-unified-account-maximum-borrowable-amount) Batch query unified account maximum borrowable amount

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#batch-query-unified-account-maximum-borrowable-amount](https://www.gate.io/docs/developers/apiv4/en/#batch-query-unified-account-maximum-borrowable-amount)

> Code samples

`GET /unified/batch_borrowable`

_Batch query unified account maximum borrowable amount_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-parameters)

| Name       | In    | Type            | Required | Description                                                                                 |
| ---------- | ----- | --------------- | -------- | ------------------------------------------------------------------------------------------- |
| currencies | query | array\[string\] | true     | Specify currency names for querying in an array, separated by commas, maximum 10 currencies |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "amount": "123456"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedborrowablelist-responseschema)

Status Code **200**

| Name                | Type   | Description                                            |
| ------------------- | ------ | ------------------------------------------------------ |
| » UnifiedBorrowable | object | Batch query unified account maximum borrowable results |
| »» currency         | string | Currency detail                                        |
| »» amount           | string | Maximum borrowable amount                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#borrow-or-repay) Borrow or repay

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#borrow-or-repay](https://www.gate.io/docs/developers/apiv4/en/#borrow-or-repay)

> Code samples

`POST /unified/loans`

_Borrow or repay_

When borrowing, ensure the borrowed amount is not below the minimum borrowing
threshold for the specific cryptocurrency and does not exceed the maximum
borrowing limit set by the platform and user.

Loan interest will be automatically deducted from the account at regular
intervals. Users are responsible for managing repayment of borrowed amounts.

For repayment, use `repaid_all=true` to repay all available amounts

> Body parameter

```
{
  "currency": "BTC",
  "amount": "0.1",
  "type": "borrow",
  "repaid_all": false,
  "text": "t-test"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-parameters](https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-parameters)

| Name         | In   | Type    | Required | Description                                                                                                                    |
| ------------ | ---- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------ |
| body         | body | object  | true     | none                                                                                                                           |
| » currency   | body | string  | true     | Currency                                                                                                                       |
| » type       | body | string  | true     | Type: `borrow` - borrow, `repay` - repay                                                                                       |
| » amount     | body | string  | true     | Borrow or repayment amount                                                                                                     |
| » repaid_all | body | boolean | false    | Full repayment, only used for repayment operations. When set to `true`, overrides `amount` and directly repays the full amount |
| » text       | body | string  | false    | User defined custom ID                                                                                                         |

#### [#](#enumerated-values-4) Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| » type    | borrow |
| » type    | repay  |

> Example responses

> 200 Response

```
{
  "tran_id": 9527
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-responses](https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createunifiedloan-responseschema)

Status Code **200**

_Unified account borrowing and repayment response result_

| Name      | Type           | Description    |
| --------- | -------------- | -------------- |
| » tran_id | integer(int64) | Transaction ID |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-loans) Query loans

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-loans](https://www.gate.io/docs/developers/apiv4/en/#query-loans)

> Code samples

`GET /unified/loans`

_Query loans_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| currency | query | string         | false    | Query by specified currency name                                         |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |
| type     | query | string         | false    | Loan type: platform borrowing - platform, margin borrowing - margin      |

> Example responses

> 200 Response

```
[
  {
    "currency": "USDT",
    "currency_pari": "GT_USDT",
    "amount": "1",
    "type": "margin",
    "change_time": 1673247054000,
    "create_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloans-responseschema)

Status Code **200**

| Name             | Type           | Description                                                         |
| ---------------- | -------------- | ------------------------------------------------------------------- |
| _None_           | array          | \[Borrowing\]                                                       |
| » _None_         | object         | Borrowing                                                           |
| »» currency      | string         | Currency                                                            |
| »» currency_pair | string         | Trading pair                                                        |
| »» amount        | string         | Amount to Repay                                                     |
| »» type          | string         | Loan type: platform borrowing - platform, margin borrowing - margin |
| »» create_time   | integer(int64) | Created time                                                        |
| »» update_time   | integer(int64) | Last Update Time                                                    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-loan-records) Query loan records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-loan-records](https://www.gate.io/docs/developers/apiv4/en/#query-loan-records)

> Code samples

`GET /unified/loan_records`

_Query loan records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| type     | query | string         | false    | Loan record type: borrow - borrowing, repay - repayment                  |
| currency | query | string         | false    | Query by specified currency name                                         |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |

> Example responses

> 200 Response

```
[
  {
    "id": 16442,
    "type": "borrow",
    "margin_mode": "cross",
    "currency_pair": "AE_USDT",
    "currency": "USDT",
    "amount": "1000",
    "create_time": 1673247054000,
    "repayment_type": "auto_repay"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloanrecords-responseschema)

Status Code **200**

| Name              | Type           | Description                                                                                                                                                                                                                              |
| ----------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » _None_          | object         | Borrowing Records                                                                                                                                                                                                                        |
| »» id             | integer(int64) | id                                                                                                                                                                                                                                       |
| »» type           | string         | Type: `borrow` - borrow, `repay` - repay                                                                                                                                                                                                 |
| »» repayment_type | string         | Repayment type: none - No repayment type, manual_repay - Manual repayment, auto_repay - Automatic repayment, cancel_auto_repay - Automatic repayment after order cancellation, different_currencies_repayment - Cross-currency repayment |
| »» borrow_type    | string         | Borrowing type, returned when querying loan records: manual_borrow - Manual borrowing, auto_borrow - Automatic borrowing                                                                                                                 |
| »» currency_pair  | string         | Trading pair                                                                                                                                                                                                                             |
| »» currency       | string         | Currency                                                                                                                                                                                                                                 |
| »» amount         | string         | Borrow or repayment amount                                                                                                                                                                                                               |
| »» create_time    | integer(int64) | Created time                                                                                                                                                                                                                             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-interest-deduction-records) Query interest deduction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-interest-deduction-records](https://www.gate.io/docs/developers/apiv4/en/#query-interest-deduction-records)

> Code samples

`GET /unified/interest_records`

_Query interest deduction records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-parameters)

| Name     | In    | Type           | Required | Description                                                                                              |
| -------- | ----- | -------------- | -------- | -------------------------------------------------------------------------------------------------------- |
| currency | query | string         | false    | Query by specified currency name                                                                         |
| page     | query | integer(int32) | false    | Page number                                                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100                                 |
| from     | query | integer(int64) | false    | Start timestamp for the query                                                                            |
| to       | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                                   |
| type     | query | string         | false    | Loan type: platform borrowing - platform, margin borrowing - margin. Defaults to margin if not specified |

> Example responses

> 200 Response

```
[
  {
    "status": 1,
    "currency_pair": "BTC_USDT",
    "currency": "USDT",
    "actual_rate": "0.00000236",
    "interest": "0.00006136",
    "type": "platform",
    "create_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedloaninterestrecords-responseschema)

Status Code **200**

| Name             | Type           | Description                                                    |
| ---------------- | -------------- | -------------------------------------------------------------- |
| _None_           | array          | \[Interest Deduction Record\]                                  |
| » _None_         | object         | Interest Deduction Record                                      |
| »» currency      | string         | Currency name                                                  |
| »» currency_pair | string         | Trading pair                                                   |
| »» actual_rate   | string         | Actual Rate                                                    |
| »» interest      | string         | Interest                                                       |
| »» status        | integer        | Status: 0 - fail, 1 - success                                  |
| »» type          | string         | Type: platform - Platform borrowing, margin - Margin borrowing |
| »» create_time   | integer(int64) | Created time                                                   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-user-risk-unit-details) Get user risk unit details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-user-risk-unit-details](https://www.gate.io/docs/developers/apiv4/en/#get-user-risk-unit-details)

> Code samples

`GET /unified/risk_units`

_Get user risk unit details_

Get user risk unit details, only valid in portfolio margin mode

> Example responses

> 200 Response

```
{
  "user_id": 0,
  "spot_hedge": true,
  "risk_units": [
    {
      "symbol": "BTC",
      "spot_in_use": "-13500.000001223",
      "maintain_margin": "2334.002",
      "initial_margin": "2334.002",
      "delta": "0.22",
      "gamma": "0.42",
      "theta": "0.29",
      "vega": "0.22"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedriskunits-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedriskunits-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedriskunits-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedriskunits-responseschema)

Status Code **200**

| Name                | Type           | Description                                           |
| ------------------- | -------------- | ----------------------------------------------------- |
| » user_id           | integer(int64) | User ID                                               |
| » spot_hedge        | boolean        | Spot hedging status: true - enabled, false - disabled |
| » risk_units        | array          | Risk unit                                             |
| »» RiskUnits        | object         | none                                                  |
| »»» symbol          | string         | Risk unit flag                                        |
| »»» spot_in_use     | string         | Spot hedging occupied amount                          |
| »»» maintain_margin | string         | Maintenance margin for risk unit                      |
| »»» initial_margin  | string         | Initial margin for risk unit                          |
| »»» delta           | string         | Total Delta of risk unit                              |
| »»» gamma           | string         | Total Gamma of risk unit                              |
| »»» theta           | string         | Total Theta of risk unit                              |
| »»» vega            | string         | Total Vega of risk unit                               |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#set-unified-account-mode) Set unified account mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#set-unified-account-mode](https://www.gate.io/docs/developers/apiv4/en/#set-unified-account-mode)

> Code samples

`PUT /unified/unified_mode`

_Set unified account mode_

Each account mode switch only requires passing the corresponding account mode
parameter, and also supports turning on or off the configuration switches under
the corresponding account mode during the switch.

- When enabling the classic account mode, mode=classic

```
 PUT /unified/unified_mode
 {
 "mode": "classic"
 }
```

- When enabling the cross-currency margin "multi_currency", "settings": {
  "usdt_futures": true } }

```
- When enabling the portfolio margin mode, mode=portfolio
```

PUT /unified/unified_mode { "mode": "portfolio", "settings": { "spot_hedge":
true } }

```
- When enabling the single-currency margin mode, mode=single_currency
```

PUT /unified/unified_mode { "mode": "single_currency" }

````

<Example>

> Body parameter

```json
{
  "mode": "portfolio",
  "settings": {
    "spot_hedge": true,
    "usdt_futures": true,
    "options": true
  }
}
````

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setunifiedmode-parameters](https://www.gate.io/docs/developers/apiv4/en/#setunifiedmode-parameters)

| Name            | In   | Type    | Required | Description                                                                                    |
| --------------- | ---- | ------- | -------- | ---------------------------------------------------------------------------------------------- |
| body            | body | object  | true     | none                                                                                           |
| » mode          | body | string  | true     | Unified account mode:                                                                          |
| » settings      | body | object  | false    | none                                                                                           |
| »» usdt_futures | body | boolean | false    | USDT futures switch. In cross-currency margin mode, can only be enabled and cannot be disabled |
| »» spot_hedge   | body | boolean | false    | Spot hedging switch                                                                            |
| »» use_funding  | body | boolean | false    | Earn switch, when mode is cross-currency margin mode, whether to use Earn funds as margin      |
| »» options      | body | boolean | false    | Options switch. In cross-currency margin mode, can only be enabled and cannot be disabled      |

#### [#](#detailed-descriptions-7) Detailed descriptions

**» mode**: Unified account mode:

- `classic`: Classic account mode
- `multi_currency`: Cross-currency margin mode
- `portfolio`: Portfolio margin mode
- `single_currency`: Single-currency margin mode

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setunifiedmode-responses](https://www.gate.io/docs/developers/apiv4/en/#setunifiedmode-responses)

| Status | Meaning                                                                            | Description      | Schema |
| ------ | ---------------------------------------------------------------------------------- | ---------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Set successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-mode-of-the-unified-account) Query mode of the unified account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-mode-of-the-unified-account](https://www.gate.io/docs/developers/apiv4/en/#query-mode-of-the-unified-account)

> Code samples

`GET /unified/unified_mode`

_Query mode of the unified account_

Unified account mode:

- `classic`: Classic account mode
- `multi_currency`: Cross-currency margin mode
- `portfolio`: Portfolio margin mode
- `single_currency`: Single-currency margin mode

> Example responses

> 200 Response

```
{
  "mode": "portfolio",
  "settings": {
    "spot_hedge": true,
    "usdt_futures": true,
    "options": true
  }
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedmode-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedmode-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedmode-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedmode-responseschema)

Status Code **200**

| Name   | Type   | Description           |
| ------ | ------ | --------------------- |
| » mode | string | Unified account mode: |

\- `classic`: Classic account mode  
\- `multi_currency`: Cross-currency margin mode  
\- `portfolio`: Portfolio margin mode  
\- `single_currency`: Single-currency margin mode | | » settings | object | none
| | »» usdt_futures | boolean | USDT futures switch. In cross-currency margin
mode, can only be enabled and cannot be disabled | | »» spot_hedge | boolean |
Spot hedging switch | | »» use_funding | boolean | Earn switch, when mode is
cross-currency margin mode, whether to use Earn funds as margin | | »» options |
boolean | Options switch. In cross-currency margin mode, can only be enabled and
cannot be disabled |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-unified-account-estimated-interest-rate) Query unified account estimated interest rate

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-estimated-interest-rate](https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-estimated-interest-rate)

> Code samples

`GET /unified/estimate_rate`

_Query unified account estimated interest rate_

Interest rates fluctuate hourly based on lending depth, so exact rates cannot be
provided. When a currency is not supported, the interest rate returned will be
an empty string

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-parameters)

| Name       | In    | Type            | Required | Description                                                                                 |
| ---------- | ----- | --------------- | -------- | ------------------------------------------------------------------------------------------- |
| currencies | query | array\[string\] | true     | Specify currency names for querying in an array, separated by commas, maximum 10 currencies |

> Example responses

> 200 Response

```
{
  "BTC": "0.000002",
  "GT": "0.000001",
  "ETH": ""
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-responses](https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunifiedestimaterate-responseschema)

Status Code **200**

_Estimate current hourly lending rates, returned by currency_

| Name                       | Type   | Description |
| -------------------------- | ------ | ----------- |
| » **additionalProperties** | string | none        |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-unified-account-tiered) Query unified account tiered

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-tiered](https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-tiered)

> Code samples

`GET /unified/currency_discount_tiers`

_Query unified account tiered_

> Example responses

> 200 Response

```
[
  [
    {
      "currency": "USDT",
      "discount_tiers": [
        {
          "tier": "1",
          "discount": "1",
          "lower_limit": "0",
          "leverage": "10",
          "upper_limit": "+"
        }
      ]
    },
    {
      "currency": "USDC",
      "discount_tiers": [
        {
          "tier": "1",
          "discount": "1",
          "lower_limit": "0",
          "leverage": "10",
          "upper_limit": "10000000"
        },
        {
          "tier": "2",
          "discount": "0.98",
          "lower_limit": "10000000",
          "leverage": "10",
          "upper_limit": "15000000"
        },
        {
          "tier": "3",
          "discount": "0.95",
          "lower_limit": "15000000",
          "leverage": "10",
          "upper_limit": "20000000"
        },
        {
          "tier": "4",
          "discount": "0.925",
          "lower_limit": "20000000",
          "leverage": "10",
          "upper_limit": "50000000"
        },
        {
          "tier": "5",
          "discount": "0.9",
          "lower_limit": "50000000",
          "leverage": "10",
          "upper_limit": "100000000"
        },
        {
          "tier": "6",
          "discount": "0",
          "lower_limit": "100000000",
          "leverage": "10",
          "upper_limit": "+"
        }
      ]
    },
    {
      "currency": "BTC",
      "discount_tiers": [
        {
          "tier": "1",
          "discount": "0.98",
          "lower_limit": "0",
          "leverage": "10",
          "upper_limit": "1000"
        },
        {
          "tier": "2",
          "discount": "0.95",
          "lower_limit": "1000",
          "leverage": "10",
          "upper_limit": "10000"
        },
        {
          "tier": "3",
          "discount": "0.9",
          "lower_limit": "10000",
          "leverage": "10",
          "upper_limit": "50000"
        },
        {
          "tier": "4",
          "discount": "0.85",
          "lower_limit": "50000",
          "leverage": "10",
          "upper_limit": "+"
        }
      ]
    },
    {
      "currency": "ETH",
      "discount_tiers": [
        {
          "tier": "1",
          "discount": "0.98",
          "lower_limit": "0",
          "leverage": "10",
          "upper_limit": "1000"
        },
        {
          "tier": "2",
          "discount": "0.95",
          "lower_limit": "1000",
          "leverage": "10",
          "upper_limit": "10000"
        },
        {
          "tier": "3",
          "discount": "0.9",
          "lower_limit": "10000",
          "leverage": "10",
          "upper_limit": "50000"
        },
        {
          "tier": "4",
          "discount": "0.85",
          "lower_limit": "50000",
          "leverage": "10",
          "upper_limit": "+"
        }
      ]
    }
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencydiscounttiers-responses](https://www.gate.io/docs/developers/apiv4/en/#listcurrencydiscounttiers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencydiscounttiers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcurrencydiscounttiers-responseschema)

Status Code **200**

| Name              | Type   | Description                                |
| ----------------- | ------ | ------------------------------------------ |
| » _None_          | object | Unified account tiered discount            |
| »» currency       | string | Currency name                              |
| »» discount_tiers | array  | Tiered discount                            |
| »»» tier          | string | Tier                                       |
| »»» discount      | string | Discount                                   |
| »»» lower_limit   | string | Lower limit                                |
| »»» upper_limit   | string | Upper limit, + indicates positive infinity |
| »»» leverage      | string | Position leverage                          |

This operation does not require authentication

## [#](#query-unified-account-tiered-loan-margin) Query unified account tiered loan margin

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-tiered-loan-margin](https://www.gate.io/docs/developers/apiv4/en/#query-unified-account-tiered-loan-margin)

> Code samples

`GET /unified/loan_margin_tiers`

_Query unified account tiered loan margin_

> Example responses

> 200 Response

```
[
  {
    "currency": "USDT",
    "margin_tiers": [
      {
        "tier": "1",
        "margin_rate": "0.02",
        "lower_limit": "200000",
        "upper_limit": "400000",
        "leverage": "3"
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listloanmargintiers-responses](https://www.gate.io/docs/developers/apiv4/en/#listloanmargintiers-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listloanmargintiers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listloanmargintiers-responseschema)

Status Code **200**

| Name             | Type   | Description                                            |
| ---------------- | ------ | ------------------------------------------------------ |
| » _None_         | object | Unified account borrowing margin tiers                 |
| »» currency      | string | Currency name                                          |
| »» margin_tiers  | array  | Tiered margin                                          |
| »»» MarginTiers  | object | none                                                   |
| »»»» tier        | string | Tier                                                   |
| »»»» margin_rate | string | Discount                                               |
| »»»» lower_limit | string | Lower limit                                            |
| »»»» upper_limit | string | Upper limit, "" indicates greater than (the last tier) |
| »»»» leverage    | string | Position leverage                                      |

This operation does not require authentication

## [#](#portfolio-margin-calculator) Portfolio margin calculator

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#portfolio-margin-calculator](https://www.gate.io/docs/developers/apiv4/en/#portfolio-margin-calculator)

> Code samples

`POST /unified/portfolio_calculator`

_Portfolio margin calculator_

Portfolio Margin Calculator

When inputting simulated position portfolios, each position includes the
position name and quantity held, supporting markets within the range of BTC and
ETH perpetual contracts, options, and spot markets. When inputting simulated
orders, each order includes the market identifier, order price, and order
quantity, supporting markets within the range of BTC and ETH perpetual
contracts, options, and spot markets. Market orders are not included.

> Body parameter

```
{
  "spot_balances": [
    {
      "currency": "BTC",
      "equity": "-1",
      "freeze": "10"
    }
  ],
  "spot_orders": [
    {
      "currency_pairs": "BTC_USDT",
      "order_price": "344",
      "size": "100",
      "left": "100",
      "type": "sell"
    }
  ],
  "futures_positions": [
    {
      "contract": "BTC_USDT",
      "size": "100"
    }
  ],
  "futures_orders": [
    {
      "contract": "BTC_USDT",
      "size": "10",
      "left": "8"
    }
  ],
  "options_positions": [
    {
      "options_name": "BTC_USDT-20240329-32000-C",
      "size": "10"
    }
  ],
  "options_orders": [
    {
      "options_name": "BTC_USDT-20240329-32000-C",
      "size": "100",
      "left": "80"
    }
  ],
  "spot_hedge": false
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-parameters](https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-parameters)

| Name                    | In   | Type    | Required | Description                                                                                                                                                               |
| ----------------------- | ---- | ------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body                    | body | object  | true     | none                                                                                                                                                                      |
| » spot_balances         | body | array   | false    | Spot                                                                                                                                                                      |
| »» _None_               | body | object  | false    | Spot                                                                                                                                                                      |
| »»» currency            | body | string  | true     | Currency name                                                                                                                                                             |
| »»» equity              | body | string  | true     | Currency equity, where equity = balance - borrowed, represents the net delta exposure of your spot positions, which can be negative. Currently only supports BTC and ETH  |
| »» spot_orders          | body | array   | false    | Spot orders                                                                                                                                                               |
| »»» _None_              | body | object  | false    | Spot orders                                                                                                                                                               |
| »»»» currency_pairs     | body | string  | true     | Market                                                                                                                                                                    |
| »»»» order_price        | body | string  | true     | Price                                                                                                                                                                     |
| »»»» count              | body | string  | false    | Initial order quantity for spot trading pairs, not involved in actual calculation. Currently only supports BTC and ETH Currently only supports three currencies: BTC, ETH |
| »»»» left               | body | string  | true     | Unfilled quantity, involved in actual calculation                                                                                                                         |
| »»»» type               | body | string  | true     | Order type, sell - sell order, buy - buy order                                                                                                                            |
| »»» futures_positions   | body | array   | false    | Futures positions                                                                                                                                                         |
| »»»» _None_             | body | object  | false    | Futures positions                                                                                                                                                         |
| »»»»» contract          | body | string  | true     | Futures name, currently only supports USDT perpetual contracts for BTC and ETH                                                                                            |
| »»»»» size              | body | string  | true     | Position size, measured in contract quantity                                                                                                                              |
| »»»» futures_orders     | body | array   | false    | Futures order                                                                                                                                                             |
| »»»»» _None_            | body | object  | false    | Futures order                                                                                                                                                             |
| »»»»»» contract         | body | string  | true     | Futures name, currently only supports USDT perpetual contracts for BTC and ETH                                                                                            |
| »»»»»» size             | body | string  | true     | Contract quantity, representing the initial order quantity, not involved in actual settlement                                                                             |
| »»»»»» left             | body | string  | true     | Unfilled contract quantity, involved in actual calculation                                                                                                                |
| »»»»» options_positions | body | array   | false    | Options positions                                                                                                                                                         |
| »»»»»» _None_           | body | object  | false    | Options positions                                                                                                                                                         |
| »»»»»»» options_name    | body | string  | true     | Option name, currently only supports USDT options for BTC and ETH                                                                                                         |
| »»»»»»» size            | body | string  | true     | Position size, measured in contract quantity                                                                                                                              |
| »»»»»» options_orders   | body | array   | false    | Option orders                                                                                                                                                             |
| »»»»»»» _None_          | body | object  | false    | Option orders                                                                                                                                                             |
| »»»»»»»» options_name   | body | string  | true     | Option name, currently only supports USDT options for BTC and ETH                                                                                                         |
| »»»»»»»» size           | body | string  | true     | Initial order quantity, not involved in actual calculation                                                                                                                |
| »»»»»»»» left           | body | string  | true     | Unfilled contract quantity, involved in actual calculation                                                                                                                |
| »»»»»»» spot_hedge      | body | boolean | false    | Whether to enable spot hedging                                                                                                                                            |

> Example responses

> 200 Response

```
{
  "maintain_margin_total": "0.000000000000",
  "initial_margin_total": "0.000000000000",
  "calculate_time": "1709014486",
  "risk_unit": [
    {
      "symbol": "BTC",
      "margin_result": [
        {
          "type": "original_position",
          "profit_loss_ranges": [
            {
              "price_percentage": "-0.200000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "-0.160000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "-0.120000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "-0.080000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "-0.040000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.000000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.040000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.080000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.120000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.160000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            },
            {
              "price_percentage": "0.200000000000",
              "implied_volatility_percentage": "-0.300000000000",
              "profit_loss": "0.000000000000"
            }
          ],
          "max_loss": {
            "price_percentage": "-0.200000000000",
            "implied_volatility_percentage": "-0.300000000000",
            "profit_loss": "0.000000000000"
          },
          "mr1": "0.000000000000",
          "mr2": "0.000000000000",
          "mr3": "0.000000000000",
          "mr4": "0.000000000000"
        }
      ],
      "maintain_margin": "0.000000000000",
      "initial_margin": "0.000000000000"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-responses](https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-responseschema](https://www.gate.io/docs/developers/apiv4/en/#calculateportfoliomargin-responseschema)

Status Code **200**

_Portfolio margin calculator output_

| Name                    | Type           | Description                                                                                                                                                                                                             |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » maintain_margin_total | string         | Total maintenance margin, including only portfolio margin calculation results for positions in risk units, excluding borrowing margin. If borrowing exists, conventional borrowing margin requirements will still apply |
| » initial_margin_total  | string         | Total initial margin, calculated as the maximum of the following three combinations: position, position + positive delta orders, position + negative delta orders                                                       |
| » calculate_time        | integer(int64) | Calculation time                                                                                                                                                                                                        |
| » risk_unit             | array          | Risk unit                                                                                                                                                                                                               |
| »» _None_               | object         | Risk unit                                                                                                                                                                                                               |
| »»» symbol              | string         | Risk unit name                                                                                                                                                                                                          |
| »»» spot_in_use         | string         | Spot hedge usage                                                                                                                                                                                                        |
| »»» maintain_margin     | string         | Maintenance margin                                                                                                                                                                                                      |
| »»» initial_margin      | string         | Initial margin                                                                                                                                                                                                          |
| »»» margin_result       | array          | Margin result                                                                                                                                                                                                           |
| »»»» _None_             | object         | Margin result                                                                                                                                                                                                           |
| »»»»» type              | string         | Position combination type                                                                                                                                                                                               |

`original_position` - Original position  
`long_delta_original_position` - Positive delta + Original position  
`short_delta_original_position` - Negative delta + Original position | | »»»»»
profit_loss_ranges | array | Results of 33 stress scenarios for MR1 | | »»»»»»
_None_ | object | Profit and loss range | | »»»»»»» price_percentage | string |
Percentage change in price | | »»»»»»» implied_volatility_percentage | string |
Percentage change in implied volatility | | »»»»»»» profit_loss | string | PnL |
| »»»»»» max_loss | object | Profit and loss range | | »»»»»»» price_percentage
| string | Percentage change in price | | »»»»»»» implied_volatility_percentage
| string | Percentage change in implied volatility | | »»»»»»» profit_loss |
string | PnL | | »»»»»» mr1 | string | Stress testing | | »»»»»» mr2 | string |
Basis spread risk | | »»»»»» mr3 | string | Volatility spread risk | | »»»»»»
mr4 | string | Option short risk | | »»»»» delta | string | Total Delta of risk
unit | | »»»»» gamma | string | Total Gamma of risk unit | | »»»»» theta |
string | Total Theta of risk unit | | »»»»» vega | string | Total Vega of risk
unit |

This operation does not require authentication

## [#](#maximum-and-minimum-currency-leverage-that-can-be-set) Maximum and minimum currency leverage that can be set

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#maximum-and-minimum-currency-leverage-that-can-be-set](https://www.gate.io/docs/developers/apiv4/en/#maximum-and-minimum-currency-leverage-that-can-be-set)

> Code samples

`GET /unified/leverage/user_currency_config`

_Maximum and minimum currency leverage that can be set_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-parameters](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-parameters)

| Name     | In    | Type   | Required | Description |
| -------- | ----- | ------ | -------- | ----------- |
| currency | query | string | true     | Currency    |

> Example responses

> 200 Response

```
{
  "current_leverage": "2",
  "min_leverage": "0",
  "max_leverage": "0",
  "debit": "0",
  "available_margin": "0",
  "borrowable": "0",
  "except_leverage_borrowable": "0"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-responses](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencyconfig-responseschema)

Status Code **200**

| Name                         | Type   | Description                                                                           |
| ---------------------------- | ------ | ------------------------------------------------------------------------------------- |
| » current_leverage           | string | Current leverage ratio                                                                |
| » min_leverage               | string | Minimum adjustable leverage ratio                                                     |
| » max_leverage               | string | Maximum adjustable leverage ratio                                                     |
| » debit                      | string | Current liabilities                                                                   |
| » available_margin           | string | Available Margin                                                                      |
| » borrowable                 | string | Maximum borrowable amount at current leverage                                         |
| » except_leverage_borrowable | string | Maximum borrowable from margin and maximum borrowable from Earn, whichever is smaller |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-user-currency-leverage) Get user currency leverage

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-user-currency-leverage](https://www.gate.io/docs/developers/apiv4/en/#get-user-currency-leverage)

> Code samples

`GET /unified/leverage/user_currency_setting`

_Get user currency leverage_

Get user currency leverage. If currency is not specified, query all currencies

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-parameters](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-parameters)

| Name     | In    | Type   | Required | Description |
| -------- | ----- | ------ | -------- | ----------- |
| currency | query | string | false    | Currency    |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "leverage": "3"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-responses](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getuserleveragecurrencysetting-responseschema)

Status Code **200**

| Name        | Type   | Description                                    |
| ----------- | ------ | ---------------------------------------------- |
| _None_      | array  | \[Leverage multiplier for borrowing currency\] |
| » _None_    | object | Leverage multiplier for borrowing currency     |
| »» currency | string | Currency name                                  |
| »» leverage | string | Multiplier                                     |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#set-loan-currency-leverage) Set loan currency leverage

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#set-loan-currency-leverage](https://www.gate.io/docs/developers/apiv4/en/#set-loan-currency-leverage)

> Code samples

`POST /unified/leverage/user_currency_setting`

_Set loan currency leverage_

> Body parameter

```
{
  "currency": "BTC",
  "leverage": "3"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setuserleveragecurrencysetting-parameters](https://www.gate.io/docs/developers/apiv4/en/#setuserleveragecurrencysetting-parameters)

| Name       | In   | Type   | Required | Description   |
| ---------- | ---- | ------ | -------- | ------------- |
| body       | body | object | true     | none          |
| » currency | body | string | true     | Currency name |
| » leverage | body | string | true     | Multiplier    |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setuserleveragecurrencysetting-responses](https://www.gate.io/docs/developers/apiv4/en/#setuserleveragecurrencysetting-responses)

| Status | Meaning                                                                            | Description      | Schema |
| ------ | ---------------------------------------------------------------------------------- | ---------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Set successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-of-loan-currencies-supported-by-unified-account) List of loan currencies supported by unified account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-of-loan-currencies-supported-by-unified-account](https://www.gate.io/docs/developers/apiv4/en/#list-of-loan-currencies-supported-by-unified-account)

> Code samples

`GET /unified/currencies`

_List of loan currencies supported by unified account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-parameters)

| Name     | In    | Type   | Required | Description |
| -------- | ----- | ------ | -------- | ----------- |
| currency | query | string | false    | Currency    |

> Example responses

> 200 Response

```
[
  {
    "name": "BTC",
    "prec": "0.000001",
    "min_borrow_amount": "0.01",
    "user_max_borrow_amount": "1000000",
    "total_max_borrow_amount": "1000000",
    "loan_status": "enable"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedcurrencies-responseschema)

Status Code **200**

| Name                      | Type   | Description                                  |
| ------------------------- | ------ | -------------------------------------------- |
| » name                    | string | Currency name                                |
| » prec                    | string | Currency precision                           |
| » min_borrow_amount       | string | Minimum borrowable limit, in currency units  |
| » user_max_borrow_amount  | string | User's maximum borrowable limit, in USDT     |
| » total_max_borrow_amount | string | Platform's maximum borrowable limit, in USDT |
| » loan_status             | string | Lending status                               |

\- `disable` : Lending prohibited  
\- `enable` : Lending supported |

This operation does not require authentication

## [#](#get-historical-lending-rates) Get historical lending rates

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-historical-lending-rates](https://www.gate.io/docs/developers/apiv4/en/#get-historical-lending-rates)

> Code samples

`GET /unified/history_loan_rate`

_Get historical lending rates_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-parameters](https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| tier     | query | string         | false    | VIP level for the floating rate to be queried                            |
| currency | query | string         | true     | Currency                                                                 |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |

> Example responses

> 200 Response

```
{
  "currency": "USDT",
  "tier": "1",
  "tier_up_rate": "1.18",
  "rates": [
    {
      "time": 1729047616000,
      "rate": "0.00010287"
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-responses](https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#gethistoryloanrate-responseschema)

Status Code **200**

| Name           | Type           | Description                                                                                                                                                                   |
| -------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » currency     | string         | Currency name                                                                                                                                                                 |
| » tier         | string         | VIP level for the floating rate to be retrieved                                                                                                                               |
| » tier_up_rate | string         | Floating rate corresponding to VIP level                                                                                                                                      |
| » rates        | array          | Historical interest rate information, one data point per hour, array size determined by page and limit parameters from the API request, sorted by time from recent to distant |
| »» time        | integer(int64) | Hourly timestamp corresponding to this interest rate, in milliseconds                                                                                                         |
| »» rate        | string         | Historical interest rate for this hour                                                                                                                                        |

This operation does not require authentication

## [#](#set-collateral-currency) Set collateral currency

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#set-collateral-currency](https://www.gate.io/docs/developers/apiv4/en/#set-collateral-currency)

> Code samples

`POST /unified/collateral_currencies`

_Set collateral currency_

> Body parameter

```
{
  "collateral_type": 1,
  "enable_list": [
    "BTC",
    "ETH"
  ],
  "disable_list": [
    "SOL",
    "GT"
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-parameters](https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-parameters)

| Name              | In   | Type    | Required | Description                                                                                                                                                                                   |
| ----------------- | ---- | ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body              | body | object  | true     | none                                                                                                                                                                                          |
| » collateral_type | body | integer | false    | User-set collateral mode: 0(all)-All currencies as collateral, 1(custom)-Custom currencies as collateral. When collateral_type is 0(all), enable_list and disable_list parameters are invalid |
| » enable_list     | body | array   | false    | Currency list, where collateral_type=1(custom) indicates the addition logic                                                                                                                   |
| » disable_list    | body | array   | false    | Disable list, indicating the disable logic                                                                                                                                                    |

#### [#](#enumerated-values-5) Enumerated Values

| Parameter         | Value |
| ----------------- | ----- |
| » collateral_type | 0     |
| » collateral_type | 1     |

> Example responses

> 200 Response

```
{
  "is_success": true
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-responses](https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-responses)

| Status | Meaning                                                                    | Description          | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Updated successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-responseschema](https://www.gate.io/docs/developers/apiv4/en/#setunifiedcollateral-responseschema)

Status Code **200**

_Unified account collateral mode settings response_

| Name         | Type    | Description                        |
| ------------ | ------- | ---------------------------------- |
| » is_success | boolean | Whether the setting was successful |

WARNING

To perform this operation, you must be authenticated by API key and secret
