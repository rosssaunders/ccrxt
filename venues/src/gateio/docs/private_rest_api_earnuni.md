# [#](#earnuni) EarnUni

Lend & Earn

## [#](#query-lending-currency-list) Query lending currency list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-lending-currency-list](https://www.gate.io/docs/developers/apiv4/en/#query-lending-currency-list)

> Code samples

`GET /earn/uni/currencies`

_Query lending currency list_

> Example responses

> 200 Response

```
[
  {
    "currency": "AE",
    "min_lend_amount": "100",
    "max_lend_amount": "200000000",
    "max_rate": "0.00057",
    "min_rate": "0.000001"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunicurrencies-responses](https://www.gate.io/docs/developers/apiv4/en/#listunicurrencies-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunicurrencies-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunicurrencies-responseschema)

Status Code **200**

| Name               | Type   | Description                                             |
| ------------------ | ------ | ------------------------------------------------------- |
| _None_             | array  | \[Currency detail\]                                     |
| » _None_           | object | Currency detail                                         |
| »» currency        | string | Currency name                                           |
| »» min_lend_amount | string | The minimum lending amount, in the unit of the currency |
| »» max_lend_amount | string | The total maximum lending amount, in USDT               |
| »» max_rate        | string | Maximum rate (Hourly)                                   |
| »» min_rate        | string | Minimum rate (Hourly)                                   |

This operation does not require authentication

## [#](#query-single-lending-currency-details) Query single lending currency details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-single-lending-currency-details](https://www.gate.io/docs/developers/apiv4/en/#query-single-lending-currency-details)

> Code samples

`GET /earn/uni/currencies/{currency}`

_Query single lending currency details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-parameters](https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-parameters)

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| currency | path | string | true     | Currency    |

> Example responses

> 200 Response

```
{
  "currency": "AE",
  "min_lend_amount": "100",
  "max_lend_amount": "200000000",
  "max_rate": "0.00057",
  "min_rate": "0.000001"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-responses](https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getunicurrency-responseschema)

Status Code **200**

_Currency detail_

| Name              | Type   | Description                                             |
| ----------------- | ------ | ------------------------------------------------------- |
| » currency        | string | Currency name                                           |
| » min_lend_amount | string | The minimum lending amount, in the unit of the currency |
| » max_lend_amount | string | The total maximum lending amount, in USDT               |
| » max_rate        | string | Maximum rate (Hourly)                                   |
| » min_rate        | string | Minimum rate (Hourly)                                   |

This operation does not require authentication

## [#](#create-lending-or-redemption) Create lending or redemption

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-lending-or-redemption](https://www.gate.io/docs/developers/apiv4/en/#create-lending-or-redemption)

> Code samples

`POST /earn/uni/lends`

_Create lending or redemption_

Lending: When lending, a minimum lending rate must be set. After successful
lending is determined on an hourly basis, earnings will be calculated based on
the determined rate. Earnings for each hour will be settled at the top of the
hour. If lending fails due to an excessively high interest rate, no interest
will be earned for that hour. If funds are redeemed before the hourly for that
hour. Priority: Under the same interest rate, wealth management products created
or modified earlier will be prioritized for lending. Redemption: For funds that
failed to be lent, redemption will be credited immediately. For funds
successfully lent, they are entitled to the earnings for that hour, and
redemption will be credited in the next hourly interval. Note: The two minutes
before and after the hourly mark are the settlement period, during which lending
and redemption are prohibited.

> Body parameter

```
{
  "currency": "AE",
  "amount": "100",
  "min_rate": "0.00001",
  "type": "lend"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createunilend-parameters](https://www.gate.io/docs/developers/apiv4/en/#createunilend-parameters)

| Name       | In   | Type   | Required | Description                                                                                                               |
| ---------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------- |
| body       | body | object | true     | none                                                                                                                      |
| » currency | body | string | true     | Currency name                                                                                                             |
| » amount   | body | string | true     | Amount to deposit into lending pool                                                                                       |
| » type     | body | string | true     | Operation type: lend - Lend, redeem - Redeem                                                                              |
| » min_rate | body | string | false    | Minimum interest rate. If set too high, lending may fail and no interest will be earned. Required for lending operations. |

#### [#](#enumerated-values-133) Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| » type    | lend   |
| » type    | redeem |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createunilend-responses](https://www.gate.io/docs/developers/apiv4/en/#createunilend-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Operation successful | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-lending-order-list) Query user's lending order list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-lending-order-list](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-lending-order-list)

> Code samples

`GET /earn/uni/lends`

_Query user's lending order list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-parameters](https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| currency | query | string         | false    | Query by specified currency name                                         |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "current_amount": "20.999992",
    "amount": "20.999992",
    "lent_amount": "0",
    "frozen_amount": "0",
    "min_rate": "0.1",
    "interest_status": "interest_dividend",
    "reinvest_left_amount": 0,
    "create_time": 1673247054000,
    "update_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-responses](https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listuserunilends-responseschema)

Status Code **200**

| Name                    | Type           | Description                                                                                     |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------------------- |
| » _None_                | object         | Loan record                                                                                     |
| »» currency             | string         | Currency                                                                                        |
| »» current_amount       | string         | Current amount                                                                                  |
| »» amount               | string         | Total Lending Amount                                                                            |
| »» lent_amount          | string         | Lent Amount                                                                                     |
| »» frozen_amount        | string         | Pending Redemption Amount                                                                       |
| »» min_rate             | string         | Minimum interest rate                                                                           |
| »» interest_status      | string         | Interest status: interest_dividend - Normal dividend, interest_reinvest - Interest reinvestment |
| »» reinvest_left_amount | string         | Non-reinvested Amount                                                                           |
| »» create_time          | integer(int64) | Lending Order Creation Time                                                                     |
| »» update_time          | integer(int64) | Lending Order Last Update Time                                                                  |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#amend-user-lending-information) Amend user lending information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#amend-user-lending-information](https://www.gate.io/docs/developers/apiv4/en/#amend-user-lending-information)

> Code samples

`PATCH /earn/uni/lends`

_Amend user lending information_

Currently only supports amending minimum interest rate (hourly)

> Body parameter

```
{
  "currency": "AE",
  "min_rate": "0.0001"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#changeunilend-parameters](https://www.gate.io/docs/developers/apiv4/en/#changeunilend-parameters)

| Name       | In   | Type   | Required | Description           |
| ---------- | ---- | ------ | -------- | --------------------- |
| body       | body | object | true     | none                  |
| » currency | body | string | false    | Currency name         |
| » min_rate | body | string | false    | Minimum interest rate |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#changeunilend-responses](https://www.gate.io/docs/developers/apiv4/en/#changeunilend-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Updated successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-lending-transaction-records) Query lending transaction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-lending-transaction-records](https://www.gate.io/docs/developers/apiv4/en/#query-lending-transaction-records)

> Code samples

`GET /earn/uni/lend_records`

_Query lending transaction records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| currency | query | string         | false    | Query by specified currency name                                         |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |
| from     | query | integer(int64) | false    | Start timestamp                                                          |
| to       | query | integer(int64) | false    | Termination Timestamp                                                    |
| type     | query | string         | false    | Operation type: lend - Lend, redeem - Redeem                             |

#### [#](#detailed-descriptions-53) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

#### [#](#enumerated-values-134) Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| type      | lend   |
| type      | redeem |

> Example responses

> 200 Response

```
[
  {
    "type": "lend",
    "currency": "BTC",
    "amount": "1",
    "last_wallet_amount": "0.2",
    "last_lent_amount": "0",
    "last_frozen_amount": "0",
    "create_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunilendrecords-responseschema)

Status Code **200**

| Name                  | Type           | Description                               |
| --------------------- | -------------- | ----------------------------------------- |
| » _None_              | object         | Lending Record                            |
| »» currency           | string         | Currency name                             |
| »» amount             | string         | Current Amount                            |
| »» last_wallet_amount | string         | Previous Available Amount                 |
| »» last_lent_amount   | string         | Previous Lent Amount                      |
| »» last_frozen_amount | string         | Previous Frozen Amount                    |
| »» type               | string         | Record Type: lend - Lend, redeem - Redeem |
| »» create_time        | integer(int64) | Created time                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-total-interest-income-for-specified-currency) Query user's total interest income for specified currency

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-interest-income-for-specified-currency](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-total-interest-income-for-specified-currency)

> Code samples

`GET /earn/uni/interests/{currency}`

_Query user's total interest income for specified currency_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-parameters](https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-parameters)

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| currency | path | string | true     | Currency    |

> Example responses

> 200 Response

```
{
  "currency": "AE",
  "interest": "123.345"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-responses](https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getuniinterest-responseschema)

Status Code **200**

_UniLendInterest_

| Name       | Type   | Description     |
| ---------- | ------ | --------------- |
| » currency | string | Currency        |
| » interest | string | Interest income |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-dividend-records) Query user dividend records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-dividend-records](https://www.gate.io/docs/developers/apiv4/en/#query-user-dividend-records)

> Code samples

`GET /earn/uni/interest_records`

_Query user dividend records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| currency | query | string         | false    | Query by specified currency name                                         |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |
| from     | query | integer(int64) | false    | Start timestamp                                                          |
| to       | query | integer(int64) | false    | Termination Timestamp                                                    |

#### [#](#detailed-descriptions-54) Detailed descriptions

**from**: Start timestamp

Specify start time, time format is Unix timestamp. If not specified, it defaults
to (the data start time of the time range actually returned by to and limit)

**to**: Termination Timestamp

Specify the end time. If not specified, it defaults to the current time, and the
time format is a Unix timestamp

> Example responses

> 200 Response

```
[
  {
    "status": 1,
    "currency": "AE",
    "actual_rate": "0.0005",
    "interest": "0.05",
    "interest_status": "interest_dividend",
    "create_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listuniinterestrecords-responseschema)

Status Code **200**

| Name               | Type           | Description                                                                                     |
| ------------------ | -------------- | ----------------------------------------------------------------------------------------------- |
| » _None_           | object         | Interest Record                                                                                 |
| »» status          | integer        | Status: 0 - fail, 1 - success                                                                   |
| »» currency        | string         | Currency                                                                                        |
| »» actual_rate     | string         | Actual Rate                                                                                     |
| »» interest        | string         | Interest                                                                                        |
| »» interest_status | string         | Interest status: interest_dividend - Normal dividend, interest_reinvest - Interest reinvestment |
| »» create_time     | integer(int64) | Created time                                                                                    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-currency-interest-compounding-status) Query currency interest compounding status

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-currency-interest-compounding-status](https://www.gate.io/docs/developers/apiv4/en/#query-currency-interest-compounding-status)

> Code samples

`GET /earn/uni/interest_status/{currency}`

_Query currency interest compounding status_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-parameters](https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-parameters)

| Name     | In   | Type   | Required | Description |
| -------- | ---- | ------ | -------- | ----------- |
| currency | path | string | true     | Currency    |

> Example responses

> 200 Response

```
{
  "currency": "BTC",
  "interest_status": "interest_dividend"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-responses](https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getuniintereststatus-responseschema)

Status Code **200**

_UniCurrencyInterest_

| Name              | Type   | Description                                                                                     |
| ----------------- | ------ | ----------------------------------------------------------------------------------------------- |
| » currency        | string | Currency                                                                                        |
| » interest_status | string | Interest status: interest_dividend - Normal dividend, interest_reinvest - Interest reinvestment |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#uniloan-currency-annualized-trend-chart) UniLoan currency annualized trend chart

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#uniloan-currency-annualized-trend-chart](https://www.gate.io/docs/developers/apiv4/en/#uniloan-currency-annualized-trend-chart)

> Code samples

`GET /earn/uni/chart`

_UniLoan currency annualized trend chart_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunichart-parameters](https://www.gate.io/docs/developers/apiv4/en/#listunichart-parameters)

| Name  | In    | Type           | Required | Description                                      |
| ----- | ----- | -------------- | -------- | ------------------------------------------------ |
| from  | query | integer(int64) | true     | Start timestamp in seconds, maximum span 30 days |
| to    | query | integer(int64) | true     | End timestamp in seconds, maximum span 30 days   |
| asset | query | string         | true     | Currency name                                    |

> Example responses

> 200 Response

```
[
  {
    "time": 1719705600,
    "value": "0.01"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunichart-responses](https://www.gate.io/docs/developers/apiv4/en/#listunichart-responses)

| Status | Meaning                                                                    | Description | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunichart-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunichart-responseschema)

Status Code **200**

| Name    | Type           | Description |
| ------- | -------------- | ----------- |
| » time  | integer(int64) | none        |
| » value | string         | none        |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#currency-estimated-annualized-interest-rate) Currency estimated annualized interest rate

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#currency-estimated-annualized-interest-rate](https://www.gate.io/docs/developers/apiv4/en/#currency-estimated-annualized-interest-rate)

> Code samples

`GET /earn/uni/rate`

_Currency estimated annualized interest rate_

> Example responses

> 200 Response

```
[
  {
    "currency": "USDT",
    "est_rate": "0.0226"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunirate-responses](https://www.gate.io/docs/developers/apiv4/en/#listunirate-responses)

| Status | Meaning                                                                    | Description | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | none        | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunirate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunirate-responseschema)

Status Code **200**

| Name       | Type   | Description            |
| ---------- | ------ | ---------------------- |
| » currency | string | none                   |
| » est_rate | string | Unconverted percentage |

WARNING

To perform this operation, you must be authenticated by API key and secret
