# [#](#isolated-margin) Isolated-Margin

Isolated Margin

## [#](#margin-account-list) Margin account list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#margin-account-list](https://www.gate.io/docs/developers/apiv4/en/#margin-account-list)

> Code samples

`GET /margin/accounts`

_Margin account list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-parameters)

| Name          | In    | Type   | Required | Description  |
| ------------- | ----- | ------ | -------- | ------------ |
| currency_pair | query | string | false    | Trading pair |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "BTC_USDT",
    "account_type": "mmr",
    "leverage": "20",
    "locked": false,
    "risk": "1.3318",
    "mmr": "16.5949188975473644",
    "base": {
      "currency": "BTC",
      "available": "0.047060413211",
      "locked": "0",
      "borrowed": "0.047233",
      "interest": "0"
    },
    "quote": {
      "currency": "USDT",
      "available": "1234",
      "locked": "0",
      "borrowed": "0",
      "interest": "0"
    }
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccounts-responseschema)

Status Code **200**

| Name             | Type    | Description                                                                                                                                                           |
| ---------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _None_           | array   | \[Margin account information for a trading pair. `base` corresponds to base currency account information, `quote` corresponds to quote currency account information\] |
| » _None_         | object  | Margin account information for a trading pair. `base` corresponds to base currency account information, `quote` corresponds to quote currency account information     |
| »» currency_pair | string  | Trading pair                                                                                                                                                          |
| »» account_type  | string  | Account type: risk - risk rate account, mmr - maintenance margin rate account, inactive - market not activated                                                        |
| »» leverage      | string  | User's current market leverage multiplier                                                                                                                             |
| »» locked        | boolean | Whether the account is locked                                                                                                                                         |
| »» risk          | string  | Current risk rate of the margin account (returned when the account is a risk rate account)                                                                            |
| »» mmr           | string  | Leveraged Account Current Maintenance Margin Rate (returned when the Account is Account)                                                                              |
| »» base          | object  | Currency account information                                                                                                                                          |
| »»» currency     | string  | Currency name                                                                                                                                                         |
| »»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                    |
| »»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                       |
| »»» borrowed     | string  | Borrowed funds                                                                                                                                                        |
| »»» interest     | string  | Unpaid interest                                                                                                                                                       |
| »» quote         | object  | Currency account information                                                                                                                                          |
| »»» currency     | string  | Currency name                                                                                                                                                         |
| »»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                    |
| »»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                       |
| »»» borrowed     | string  | Borrowed funds                                                                                                                                                        |
| »»» interest     | string  | Unpaid interest                                                                                                                                                       |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-margin-account-balance-change-history) Query margin account balance change history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-margin-account-balance-change-history](https://www.gate.io/docs/developers/apiv4/en/#query-margin-account-balance-change-history)

> Code samples

`GET /margin/account_book`

_Query margin account balance change history_

Currently only provides transfer history to and from margin accounts. Query time
range cannot exceed 30 days

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-parameters)

| Name          | In    | Type           | Required | Description                                                                                                       |
| ------------- | ----- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------- |
| currency      | query | string         | false    | Query history for specified currency. If `currency` is specified, `currency_pair` must also be specified.         |
| currency_pair | query | string         | false    | Specify margin account currency pair. Used in combination with `currency`. Ignored if `currency` is not specified |
| type          | query | string         | false    | Query by specified account change type. If not specified, all change types will be included.                      |
| from          | query | integer(int64) | false    | Start timestamp                                                                                                   |
| to            | query | integer(int64) | false    | Termination Timestamp                                                                                             |
| page          | query | integer(int32) | false    | Page number                                                                                                       |
| limit         | query | integer        | false    | Maximum number of records returned in a single list                                                               |

#### [#](#detailed-descriptions-8) Detailed descriptions

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
    "id": "123456",
    "time": "1547633726",
    "time_ms": 1547633726123,
    "currency": "BTC",
    "currency_pair": "BTC_USDT",
    "change": "1.03",
    "balance": "4.59316525194"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginaccountbook-responseschema)

Status Code **200**

| Name            | Type           | Description                                                                               |
| --------------- | -------------- | ----------------------------------------------------------------------------------------- |
| » id            | string         | Balance change record ID                                                                  |
| » time          | string         | Account change timestamp                                                                  |
| » time_ms       | integer(int64) | The timestamp of the change (in milliseconds)                                             |
| » currency      | string         | Currency changed                                                                          |
| » currency_pair | string         | Account trading pair                                                                      |
| » change        | string         | Amount changed. Positive value means transferring in, while negative out                  |
| » balance       | string         | Balance after change                                                                      |
| » type          | string         | Account book type. Please refer to [account book type](#accountbook-type) for more detail |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#funding-account-list) Funding account list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#funding-account-list](https://www.gate.io/docs/developers/apiv4/en/#funding-account-list)

> Code samples

`GET /margin/funding_accounts`

_Funding account list_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | false    | Query by specified currency name |

> Example responses

> 200 Response

```
[
  {
    "currency": "BTC",
    "available": "1.238",
    "locked": "0",
    "lent": "3.32",
    "total_lent": "3.32"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolatedfundingaccounts-responseschema)

Status Code **200**

| Name         | Type   | Description                                                              |
| ------------ | ------ | ------------------------------------------------------------------------ |
| » currency   | string | Currency name                                                            |
| » available  | string | Available assets to lend, which is identical to spot account `available` |
| » locked     | string | Locked amount. i.e. amount in `open` loans                               |
| » lent       | string | Outstanding loan amount yet to be repaid                                 |
| » total_lent | string | Amount used for lending. total_lent = lent + locked                      |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-user-auto-repayment-settings) Update user auto repayment settings

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-user-auto-repayment-settings](https://www.gate.io/docs/developers/apiv4/en/#update-user-auto-repayment-settings)

> Code samples

`POST /margin/auto_repay`

_Update user auto repayment settings_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-parameters](https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-parameters)

| Name   | In    | Type   | Required | Description                                                        |
| ------ | ----- | ------ | -------- | ------------------------------------------------------------------ |
| status | query | string | true     | Whether to enable auto repayment: `on` - enabled, `off` - disabled |

> Example responses

> 200 Response

```
{
  "status": "on"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-responses](https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-responses)

| Status | Meaning                                                                    | Description                            | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | User's current auto repayment settings | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-responseschema](https://www.gate.io/docs/developers/apiv4/en/#setisolatedautorepay-responseschema)

Status Code **200**

_AutoRepaySetting_

| Name     | Type   | Description                                             |
| -------- | ------ | ------------------------------------------------------- |
| » status | string | Auto repayment status: `on` - enabled, `off` - disabled |

#### [#](#enumerated-values-6) Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | on    |
| status   | off   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-auto-repayment-settings) Query user auto repayment settings

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-auto-repayment-settings](https://www.gate.io/docs/developers/apiv4/en/#query-user-auto-repayment-settings)

> Code samples

`GET /margin/auto_repay`

_Query user auto repayment settings_

> Example responses

> 200 Response

```
{
  "status": "on"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedautorepaystatus-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedautorepaystatus-responses)

| Status | Meaning                                                                    | Description                            | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | User's current auto repayment settings | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedautorepaystatus-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedautorepaystatus-responseschema)

Status Code **200**

_AutoRepaySetting_

| Name     | Type   | Description                                             |
| -------- | ------ | ------------------------------------------------------- |
| » status | string | Auto repayment status: `on` - enabled, `off` - disabled |

#### [#](#enumerated-values-7) Enumerated Values

| Property | Value |
| -------- | ----- |
| status   | on    |
| status   | off   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-maximum-transferable-amount-for-isolated-margin) Get maximum transferable amount for isolated margin

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-maximum-transferable-amount-for-isolated-margin](https://www.gate.io/docs/developers/apiv4/en/#get-maximum-transferable-amount-for-isolated-margin)

> Code samples

`GET /margin/transferable`

_Get maximum transferable amount for isolated margin_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-parameters)

| Name          | In    | Type   | Required | Description                      |
| ------------- | ----- | ------ | -------- | -------------------------------- |
| currency      | query | string | true     | Query by specified currency name |
| currency_pair | query | string | false    | Trading pair                     |

> Example responses

> 200 Response

```
{
  "currency": "ETH",
  "currency_pair": "ETH_USDT",
  "amount": "10000"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmargintransferable-responseschema)

Status Code **200**

_MarginTransferable_

| Name            | Type   | Description             |
| --------------- | ------ | ----------------------- |
| » currency      | string | Currency detail         |
| » currency_pair | string | Trading pair            |
| » amount        | string | Max transferable amount |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-lending-markets) List lending markets

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-lending-markets](https://www.gate.io/docs/developers/apiv4/en/#list-lending-markets)

> Code samples

`GET /margin/uni/currency_pairs`

_List lending markets_

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "AE_USDT",
    "base_min_borrow_amount": "100",
    "quote_min_borrow_amount": "100",
    "leverage": "3"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedunicurrencypairs-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolatedunicurrencypairs-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedunicurrencypairs-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolatedunicurrencypairs-responseschema)

Status Code **200**

| Name                       | Type   | Description                             |
| -------------------------- | ------ | --------------------------------------- |
| _None_                     | array  | \[Currency pair of the loan\]           |
| » _None_                   | object | Currency pair of the loan               |
| »» currency_pair           | string | Trading pair                            |
| »» base_min_borrow_amount  | string | Minimum borrow amount of base currency  |
| »» quote_min_borrow_amount | string | Minimum borrow amount of quote currency |
| »» leverage                | string | Position leverage                       |

This operation does not require authentication

## [#](#get-lending-market-details) Get lending market details

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-lending-market-details](https://www.gate.io/docs/developers/apiv4/en/#get-lending-market-details)

> Code samples

`GET /margin/uni/currency_pairs/{currency_pair}`

_Get lending market details_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-parameters)

| Name          | In   | Type   | Required | Description  |
| ------------- | ---- | ------ | -------- | ------------ |
| currency_pair | path | string | true     | Trading pair |

> Example responses

> 200 Response

```
{
  "currency_pair": "AE_USDT",
  "base_min_borrow_amount": "100",
  "quote_min_borrow_amount": "100",
  "leverage": "3"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedunicurrencypair-responseschema)

Status Code **200**

_Currency pair of the loan_

| Name                      | Type   | Description                             |
| ------------------------- | ------ | --------------------------------------- |
| » currency_pair           | string | Trading pair                            |
| » base_min_borrow_amount  | string | Minimum borrow amount of base currency  |
| » quote_min_borrow_amount | string | Minimum borrow amount of quote currency |
| » leverage                | string | Position leverage                       |

This operation does not require authentication

## [#](#estimate-interest-rate-for-isolated-margin-currencies) Estimate interest rate for isolated margin currencies

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#estimate-interest-rate-for-isolated-margin-currencies](https://www.gate.io/docs/developers/apiv4/en/#estimate-interest-rate-for-isolated-margin-currencies)

> Code samples

`GET /margin/uni/estimate_rate`

_Estimate interest rate for isolated margin currencies_

Interest rates change hourly based on lending depth, so completely accurate
rates cannot be provided.

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-parameters)

| Name       | In    | Type            | Required | Description                                  |
| ---------- | ----- | --------------- | -------- | -------------------------------------------- |
| currencies | query | array\[string\] | true     | Array of currency names to query, maximum 10 |

> Example responses

> 200 Response

```
{
  "BTC": "0.000002",
  "GT": "0.000001"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarginuniestimaterate-responseschema)

Status Code **200**

_Estimate current hourly lending rates, returned by currency_

| Name                       | Type   | Description |
| -------------------------- | ------ | ----------- |
| » **additionalProperties** | string | none        |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#borrow-or-repay-2) Borrow or repay

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#borrow-or-repay-2](https://www.gate.io/docs/developers/apiv4/en/#borrow-or-repay-2)

> Code samples

`POST /margin/uni/loans`

_Borrow or repay_

> Body parameter

```
{
  "currency": "BTC",
  "amount": "0.1",
  "type": "borrow",
  "currency_pair": "BTC_USDT",
  "repaid_all": false
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createisolateduniloan-parameters](https://www.gate.io/docs/developers/apiv4/en/#createisolateduniloan-parameters)

| Name            | In   | Type    | Required | Description                                                                                               |
| --------------- | ---- | ------- | -------- | --------------------------------------------------------------------------------------------------------- |
| body            | body | object  | true     | none                                                                                                      |
| » currency      | body | string  | true     | Currency                                                                                                  |
| » type          | body | string  | true     | Type: `borrow` - borrow, `repay` - repay                                                                  |
| » amount        | body | string  | true     | Borrow or repayment amount                                                                                |
| » repaid_all    | body | boolean | false    | Full repayment. For repayment operations only. When `true`, overrides `amount` and repays the full amount |
| » currency_pair | body | string  | true     | Trading pair                                                                                              |

#### [#](#enumerated-values-8) Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| » type    | borrow |
| » type    | repay  |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createisolateduniloan-responses](https://www.gate.io/docs/developers/apiv4/en/#createisolateduniloan-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Operation successful | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-loans-2) Query loans

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-loans-2](https://www.gate.io/docs/developers/apiv4/en/#query-loans-2)

> Code samples

`GET /margin/uni/loans`

_Query loans_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-parameters)

| Name          | In    | Type           | Required | Description                                         |
| ------------- | ----- | -------------- | -------- | --------------------------------------------------- |
| currency_pair | query | string         | false    | Trading pair                                        |
| currency      | query | string         | false    | Query by specified currency name                    |
| page          | query | integer(int32) | false    | Page number                                         |
| limit         | query | integer        | false    | Maximum number of records returned in a single list |

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
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloans-responseschema)

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

## [#](#query-loan-records-2) Query loan records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-loan-records-2](https://www.gate.io/docs/developers/apiv4/en/#query-loan-records-2)

> Code samples

`GET /margin/uni/loan_records`

_Query loan records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-parameters)

| Name          | In    | Type           | Required | Description                                         |
| ------------- | ----- | -------------- | -------- | --------------------------------------------------- |
| type          | query | string         | false    | Type: `borrow` - borrow, `repay` - repay            |
| currency      | query | string         | false    | Query by specified currency name                    |
| currency_pair | query | string         | false    | Trading pair                                        |
| page          | query | integer(int32) | false    | Page number                                         |
| limit         | query | integer        | false    | Maximum number of records returned in a single list |

#### [#](#enumerated-values-9) Enumerated Values

| Parameter | Value  |
| --------- | ------ |
| type      | borrow |
| type      | repay  |

> Example responses

> 200 Response

```
[
  {
    "type": "borrow",
    "currency_pair": "AE_USDT",
    "currency": "USDT",
    "amount": "1000",
    "create_time": 1673247054000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloanrecords-responseschema)

Status Code **200**

| Name             | Type           | Description                              |
| ---------------- | -------------- | ---------------------------------------- |
| » _None_         | object         | Borrowing Records                        |
| »» type          | string         | Type: `borrow` - borrow, `repay` - repay |
| »» currency_pair | string         | Trading pair                             |
| »» currency      | string         | Currency                                 |
| »» amount        | string         | Borrow or repayment amount               |
| »» create_time   | integer(int64) | Created time                             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-interest-deduction-records-2) Query interest deduction records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-interest-deduction-records-2](https://www.gate.io/docs/developers/apiv4/en/#query-interest-deduction-records-2)

> Code samples

`GET /margin/uni/interest_records`

_Query interest deduction records_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-parameters)

| Name          | In    | Type           | Required | Description                                         |
| ------------- | ----- | -------------- | -------- | --------------------------------------------------- |
| currency_pair | query | string         | false    | Trading pair                                        |
| currency      | query | string         | false    | Query by specified currency name                    |
| page          | query | integer(int32) | false    | Page number                                         |
| limit         | query | integer        | false    | Maximum number of records returned in a single list |
| from          | query | integer(int64) | false    | Start timestamp                                     |
| to            | query | integer(int64) | false    | Termination Timestamp                               |

#### [#](#detailed-descriptions-9) Detailed descriptions

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
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolateduniloaninterestrecords-responseschema)

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

## [#](#query-maximum-borrowable-amount-by-currency) Query maximum borrowable amount by currency

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-maximum-borrowable-amount-by-currency](https://www.gate.io/docs/developers/apiv4/en/#query-maximum-borrowable-amount-by-currency)

> Code samples

`GET /margin/uni/borrowable`

_Query maximum borrowable amount by currency_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-parameters)

| Name          | In    | Type   | Required | Description                      |
| ------------- | ----- | ------ | -------- | -------------------------------- |
| currency      | query | string | true     | Query by specified currency name |
| currency_pair | query | string | true     | Trading pair                     |

> Example responses

> 200 Response

```
{
  "currency": "AE",
  "borrowable": "1123.344",
  "currency_pair": "AE_USDT"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolateduniborrowable-responseschema)

Status Code **200**

_MaxUniBorrowable_

| Name            | Type   | Description        |
| --------------- | ------ | ------------------ |
| » currency      | string | Currency           |
| » currency_pair | string | Trading pair       |
| » borrowable    | string | Maximum borrowable |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-own-leverage-lending-tiers-in-current-market) Query user's own leverage lending tiers in current market

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-own-leverage-lending-tiers-in-current-market](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-own-leverage-lending-tiers-in-current-market)

> Code samples

`GET /margin/user/loan_margin_tiers`

_Query user's own leverage lending tiers in current market_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-parameters)

| Name          | In    | Type   | Required | Description  |
| ------------- | ----- | ------ | -------- | ------------ |
| currency_pair | query | string | true     | Trading pair |

> Example responses

> 200 Response

```
[
  {
    "tier_amount": "100",
    "mmr": "0.9",
    "leverage": "1"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedusermargintier-responseschema)

Status Code **200**

| Name           | Type   | Description                     |
| -------------- | ------ | ------------------------------- |
| _None_         | array  | \[Market gradient information\] |
| » _None_       | object | Market gradient information     |
| »» upper_limit | string | Maximum loan limit              |
| »» mmr         | string | Maintenance margin rate         |
| »» leverage    | string | Maximum leverage multiple       |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-current-market-leverage-lending-tiers) Query current market leverage lending tiers

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-current-market-leverage-lending-tiers](https://www.gate.io/docs/developers/apiv4/en/#query-current-market-leverage-lending-tiers)

> Code samples

`GET /margin/loan_margin_tiers`

_Query current market leverage lending tiers_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-parameters](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-parameters)

| Name          | In    | Type   | Required | Description  |
| ------------- | ----- | ------ | -------- | ------------ |
| currency_pair | query | string | true     | Trading pair |

> Example responses

> 200 Response

```
[
  {
    "tier_amount": "100",
    "mmr": "0.9",
    "leverage": "1"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-responses](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getisolatedmarketmargintier-responseschema)

Status Code **200**

| Name           | Type   | Description                     |
| -------------- | ------ | ------------------------------- |
| _None_         | array  | \[Market gradient information\] |
| » _None_       | object | Market gradient information     |
| »» upper_limit | string | Maximum loan limit              |
| »» mmr         | string | Maintenance margin rate         |
| »» leverage    | string | Maximum leverage multiple       |

This operation does not require authentication

## [#](#set-user-market-leverage-multiplier) Set user market leverage multiplier

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#set-user-market-leverage-multiplier](https://www.gate.io/docs/developers/apiv4/en/#set-user-market-leverage-multiplier)

> Code samples

`POST /margin/leverage/user_market_setting`

_Set user market leverage multiplier_

> Body parameter

```
{
  "currency_pair": "BTC_USDT",
  "leverage": "10"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setisolatedusermarketleverage-parameters](https://www.gate.io/docs/developers/apiv4/en/#setisolatedusermarketleverage-parameters)

| Name            | In   | Type   | Required | Description       |
| --------------- | ---- | ------ | -------- | ----------------- |
| body            | body | object | true     | none              |
| » currency_pair | body | string | false    | Market            |
| » leverage      | body | string | true     | Position leverage |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setisolatedusermarketleverage-responses](https://www.gate.io/docs/developers/apiv4/en/#setisolatedusermarketleverage-responses)

| Status | Meaning                                                                            | Description      | Schema |
| ------ | ---------------------------------------------------------------------------------- | ---------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Set successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-user-s-isolated-margin-account-list) Query user's isolated margin account list

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-user-s-isolated-margin-account-list](https://www.gate.io/docs/developers/apiv4/en/#query-user-s-isolated-margin-account-list)

> Code samples

`GET /margin/user/account`

_Query user's isolated margin account list_

Supports querying risk ratio isolated accounts and margin ratio isolated
accounts

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-parameters)

| Name          | In    | Type   | Required | Description  |
| ------------- | ----- | ------ | -------- | ------------ |
| currency_pair | query | string | false    | Trading pair |

> Example responses

> 200 Response

```
[
  {
    "currency_pair": "BTC_USDT",
    "account_type": "mmr",
    "leverage": "20",
    "locked": false,
    "risk": "1.3318",
    "mmr": "16.5949188975473644",
    "base": {
      "currency": "BTC",
      "available": "0.047060413211",
      "locked": "0",
      "borrowed": "0.047233",
      "interest": "0"
    },
    "quote": {
      "currency": "USDT",
      "available": "1234",
      "locked": "0",
      "borrowed": "0",
      "interest": "0"
    }
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-responses](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listisolatedmarginuseraccount-responseschema)

Status Code **200**

| Name             | Type    | Description                                                                                                                                                           |
| ---------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _None_           | array   | \[Margin account information for a trading pair. `base` corresponds to base currency account information, `quote` corresponds to quote currency account information\] |
| » _None_         | object  | Margin account information for a trading pair. `base` corresponds to base currency account information, `quote` corresponds to quote currency account information     |
| »» currency_pair | string  | Trading pair                                                                                                                                                          |
| »» account_type  | string  | Account type: risk - risk rate account, mmr - maintenance margin rate account, inactive - market not activated                                                        |
| »» leverage      | string  | User's current market leverage multiplier                                                                                                                             |
| »» locked        | boolean | Whether the account is locked                                                                                                                                         |
| »» risk          | string  | Current risk rate of the margin account (returned when the account is a risk rate account)                                                                            |
| »» mmr           | string  | Leveraged Account Current Maintenance Margin Rate (returned when the Account is Account)                                                                              |
| »» base          | object  | Currency account information                                                                                                                                          |
| »»» currency     | string  | Currency name                                                                                                                                                         |
| »»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                    |
| »»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                       |
| »»» borrowed     | string  | Borrowed funds                                                                                                                                                        |
| »»» interest     | string  | Unpaid interest                                                                                                                                                       |
| »» quote         | object  | Currency account information                                                                                                                                          |
| »»» currency     | string  | Currency name                                                                                                                                                         |
| »»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                    |
| »»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                       |
| »»» borrowed     | string  | Borrowed funds                                                                                                                                                        |
| »»» interest     | string  | Unpaid interest                                                                                                                                                       |

WARNING

To perform this operation, you must be authenticated by API key and secret
