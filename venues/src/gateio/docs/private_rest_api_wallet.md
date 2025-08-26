# [#](#wallet) Wallet

Wallet API

## [#](#query-chains-supported-for-specified-currency) Query chains supported for specified currency

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-chains-supported-for-specified-currency](https://www.gate.io/docs/developers/apiv4/en/#query-chains-supported-for-specified-currency)

> Code samples

`GET /wallet/currency_chains`

_Query chains supported for specified currency_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-parameters](https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-parameters)

| Name     | In    | Type   | Required | Description   |
| -------- | ----- | ------ | -------- | ------------- |
| currency | query | string | true     | Currency name |

> Example responses

> 200 Response

```
[
  {
    "chain": "ETH",
    "name_cn": "以太坊ERC20",
    "name_en": "ETH/ERC20",
    "contract_address": "",
    "is_disabled": 0,
    "is_deposit_disabled": 0,
    "is_withdraw_disabled": 0
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-responses](https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listcurrencychains-responseschema)

Status Code **200**

| Name                   | Type           | Description                                                                                     |
| ---------------------- | -------------- | ----------------------------------------------------------------------------------------------- |
| » chain                | string         | Chain name                                                                                      |
| » name_cn              | string         | Chain name in Chinese                                                                           |
| » name_en              | string         | Chain name in English                                                                           |
| » contract_address     | string         | Smart contract address for the currency; if no address is available, it will be an empty string |
| » is_disabled          | integer(int32) | If it is disabled. 0 means NOT being disabled                                                   |
| » is_deposit_disabled  | integer(int32) | Is deposit disabled. 0 means not disabled                                                       |
| » is_withdraw_disabled | integer(int32) | Is withdrawal disabled. 0 means not disabled                                                    |
| » decimal              | string         | Withdrawal precision                                                                            |

This operation does not require authentication

## [#](#generate-currency-deposit-address) Generate currency deposit address

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#generate-currency-deposit-address](https://www.gate.io/docs/developers/apiv4/en/#generate-currency-deposit-address)

> Code samples

`GET /wallet/deposit_address`

_Generate currency deposit address_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-parameters](https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-parameters)

| Name     | In    | Type   | Required | Description   |
| -------- | ----- | ------ | -------- | ------------- |
| currency | query | string | true     | Currency name |

> Example responses

> 200 Response

```
{
  "currency": "USDT",
  "address": "LPXtk1kWHioP62SzfqwKbYE3Z7Wt2ujYEc",
  "multichain_addresses": [
    {
      "chain": "TRX",
      "address": "LPXtk1kWHioP62SzfqwKbYE3Z7Wt2ujYEc",
      "payment_id": "",
      "payment_name": "",
      "obtain_failed": 0
    }
  ]
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-responses](https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-responses)

| Status | Meaning                                                                    | Description                    | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------------ | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Address successfully generated | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getdepositaddress-responseschema)

Status Code **200**

| Name                     | Type    | Description                                                                             |
| ------------------------ | ------- | --------------------------------------------------------------------------------------- |
| » currency               | string  | Currency detail                                                                         |
| » address                | string  | Deposit address                                                                         |
| » multichain_addresses   | array   | none                                                                                    |
| »» MultiChainAddressItem | object  | none                                                                                    |
| »»» chain                | string  | Name of the chain                                                                       |
| »»» address              | string  | Deposit address                                                                         |
| »»» payment_id           | string  | Notes that some currencies required(e.g., Tag, Memo) when depositing                    |
| »»» payment_name         | string  | Note type, `Tag` or `Memo`                                                              |
| »»» obtain_failed        | integer | The obtain failed status- 0: address successfully obtained- 1: failed to obtain address |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-withdrawal-records) Get withdrawal records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-withdrawal-records](https://www.gate.io/docs/developers/apiv4/en/#get-withdrawal-records)

> Code samples

`GET /wallet/withdrawals`

_Get withdrawal records_

Record query time range cannot exceed 30 days

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-parameters](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-parameters)

| Name              | In    | Type           | Required | Description                                                                                                                                                                               |
| ----------------- | ----- | -------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| currency          | query | string         | false    | Specify the currency. If not specified, returns all currencies                                                                                                                            |
| withdraw_id       | query | string         | false    | Withdrawal record ID starts with 'w', such as: w1879219868. When withdraw_id is not empty, only this specific withdrawal record will be queried, and time-based querying will be disabled |
| asset_class       | query | string         | false    | Currency type of withdrawal record, empty by default. Supports querying withdrawal records in main zone and innovation zone on demand.                                                    |
| withdraw_order_id | query | string         | false    | User-defined order number for withdrawal. Default is empty. When not empty, the specified user-defined order number record will be queried                                                |
| from              | query | integer(int64) | false    | Start time for querying records, defaults to 7 days before current time if not specified                                                                                                  |
| to                | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                                                                                                                    |
| limit             | query | integer        | false    | Maximum number of records returned in a single list                                                                                                                                       |
| offset            | query | integer        | false    | List offset, starting from 0                                                                                                                                                              |

#### [#](#detailed-descriptions-2) Detailed descriptions

**asset_class**: Currency type of withdrawal record, empty by default. Supports
querying withdrawal records in main zone and innovation zone on demand. Value
range: SPOT, PILOT

SPOT: Main Zone PILOT: Innovation Zone

> Example responses

> 200 Response

```
[
  [
    {
      "id": "w1879219868",
      "currency": "USDT",
      "address": "THISISTESTADDRESSFORGATEPAY",
      "amount": "4.023",
      "fee": "0",
      "txid": "Internal transaction 260594131",
      "chain": "BSC",
      "timestamp": "1745220149",
      "status": "DONE",
      "withdraw_order_id": "202504211521368538928",
      "block_number": "1000",
      "fail_reason": "",
      "type": "appbankgp",
      "timestamp2": "1745220149",
      "memo": ""
    }
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-responses](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawals-responseschema)

Status Code **200**

| Name                | Type   | Description                                                                                              |
| ------------------- | ------ | -------------------------------------------------------------------------------------------------------- |
| » id                | string | Record ID                                                                                                |
| » txid              | string | Hash record of the withdrawal                                                                            |
| » block_number      | string | Block Number                                                                                             |
| » withdraw_order_id | string | Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.) |
| » timestamp         | string | Operation time                                                                                           |
| » amount            | string | Token amount                                                                                             |
| » fee               | string | Fee                                                                                                      |
| » currency          | string | Currency name                                                                                            |
| » address           | string | Withdrawal address                                                                                       |
| » fail_reason       | string | Reason for withdrawal failure. Has a value when status = CANCEL, empty for all other statuses            |
| » timestamp2        | string | Withdrawal final time, i.e.: withdrawal cancellation time or withdrawal success time                     |

When status = CANCEL, corresponds to cancellation time  
When status = DONE and block_number > 0, it is the withdrawal success time | | »
memo | string | Additional remarks with regards to the withdrawal | | » status |
string | Transaction status

\- DONE: Completed (block_number > 0 is considered to be truly completed)  
\- CANCEL: Canceled  
\- REQUEST: Requesting  
\- MANUAL: Pending manual review  
\- BCODE: Recharge code operation  
\- EXTPEND: Sent awaiting confirmation  
\- FAIL: Failure on the chain awaiting confirmation  
\- INVALID: Invalid order  
\- VERIFY: Verifying  
\- PROCES: Processing  
\- PEND: Processing  
\- DMOVE: pending manual review  
\- REVIEW: Under review | | » chain | string | Name of the chain used in
withdrawals |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-deposit-records) Get deposit records

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-deposit-records](https://www.gate.io/docs/developers/apiv4/en/#get-deposit-records)

> Code samples

`GET /wallet/deposits`

_Get deposit records_

Record query time range cannot exceed 30 days

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeposits-parameters](https://www.gate.io/docs/developers/apiv4/en/#listdeposits-parameters)

| Name     | In    | Type           | Required | Description                                                                              |
| -------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------- |
| currency | query | string         | false    | Specify the currency. If not specified, returns all currencies                           |
| from     | query | integer(int64) | false    | Start time for querying records, defaults to 7 days before current time if not specified |
| to       | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                   |
| limit    | query | integer        | false    | Maximum number of entries returned in the list, limited to 500 transactions              |
| offset   | query | integer        | false    | List offset, starting from 0                                                             |

> Example responses

> 200 Response

```
[
  {
    "id": "210496",
    "timestamp": "1542000000",
    "withdraw_order_id": "order_123456",
    "currency": "USDT",
    "address": "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs",
    "txid": "128988928203223323290",
    "amount": "222.61",
    "memo": "",
    "status": "DONE",
    "chain": "TRX"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeposits-responses](https://www.gate.io/docs/developers/apiv4/en/#listdeposits-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listdeposits-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listdeposits-responseschema)

Status Code **200**

| Name                | Type   | Description                                                                                              |
| ------------------- | ------ | -------------------------------------------------------------------------------------------------------- |
| » id                | string | Record ID                                                                                                |
| » txid              | string | Hash record of the withdrawal                                                                            |
| » withdraw_order_id | string | Client order id, up to 32 length and can only include 0-9, A-Z, a-z, underscore(\_), hyphen(-) or dot(.) |
| » timestamp         | string | Operation time                                                                                           |
| » amount            | string | Token amount                                                                                             |
| » currency          | string | Currency name                                                                                            |
| » address           | string | Withdrawal address. Required for withdrawals                                                             |
| » memo              | string | Additional remarks with regards to the withdrawal                                                        |
| » status            | string | Trading Status                                                                                           |

\- REVIEW: Recharge review (compliance review)  
\- PEND: Processing  
\- DONE: Waiting for funds to be unlocked  
\- INVALID: Invalid data  
\- TRACK: Track the number of confirmations, waiting to add funds to the user
(spot)  
\- BLOCKED: Rejected Recharge  
\- DEP_CREDITED: Recharge to account, withdrawal is not unlocked | | » chain |
string | Name of the chain used in withdrawals |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#transfer-between-trading-accounts) Transfer between trading accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-between-trading-accounts](https://www.gate.io/docs/developers/apiv4/en/#transfer-between-trading-accounts)

> Code samples

`POST /wallet/transfers`

_Transfer between trading accounts_

Balance transfers between personal trading accounts. Currently supports the
following transfer operations:

1.  Spot account - Margin account
2.  Spot account - Perpetual futures account
3.  Spot account - Delivery futures account
4.  Spot account - Options account

> Body parameter

```
{
  "currency": "BTC",
  "from": "spot",
  "to": "margin",
  "amount": "1",
  "currency_pair": "BTC_USDT",
  "settle": ""
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-parameters](https://www.gate.io/docs/developers/apiv4/en/#transfer-parameters)

| Name            | In   | Type   | Required | Description                                                                                                                                       |
| --------------- | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| body            | body | object | true     | none                                                                                                                                              |
| » currency      | body | string | true     | Transfer currency name. For contract accounts, `currency` can be set to `POINT` (points) or supported settlement currencies (e.g., `BTC`, `USDT`) |
| » from          | body | string | true     | Account to transfer from                                                                                                                          |
| » to            | body | string | true     | Account to transfer to                                                                                                                            |
| » amount        | body | string | true     | Transfer amount                                                                                                                                   |
| » currency_pair | body | string | false    | Margin trading pair. Required when transferring to or from margin account                                                                         |
| » settle        | body | string | false    | Contract settlement currency. Required when transferring to or from contract account                                                              |

#### [#](#enumerated-values) Enumerated Values

| Parameter | Value    |
| --------- | -------- |
| » from    | spot     |
| » from    | margin   |
| » from    | futures  |
| » from    | delivery |
| » from    | options  |
| » to      | spot     |
| » to      | margin   |
| » to      | futures  |
| » to      | delivery |
| » to      | options  |

> Example responses

> 200 Response

```
{
  "tx_id": 59636381286
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-responses](https://www.gate.io/docs/developers/apiv4/en/#transfer-responses)

| Status | Meaning                                                                    | Description                   | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Transfer operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-responseschema](https://www.gate.io/docs/developers/apiv4/en/#transfer-responseschema)

Status Code **200**

_TransactionID_

| Name    | Type           | Description |
| ------- | -------------- | ----------- |
| » tx_id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#transfer-between-main-and-sub-accounts) Transfer between main and sub accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-between-main-and-sub-accounts](https://www.gate.io/docs/developers/apiv4/en/#transfer-between-main-and-sub-accounts)

> Code samples

`POST /wallet/sub_account_transfers`

_Transfer between main and sub accounts_

Supports transfers to/from sub-account's spot or futures accounts. Note that
regardless of which sub-account is operated, only the main account's spot
account is used

> Body parameter

```
{
  "sub_account": "10002",
  "sub_account_type": "spot",
  "currency": "BTC",
  "amount": "1",
  "direction": "to",
  "client_order_id": "da3ce7a088c8b0372b741419c7829033"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-parameters)

| Name               | In   | Type   | Required | Description                                                                                                                                                                                                               |
| ------------------ | ---- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body               | body | object | true     | none                                                                                                                                                                                                                      |
| » sub_account      | body | string | true     | Sub account user ID                                                                                                                                                                                                       |
| » sub_account_type | body | string | false    | Target sub-account trading account: spot - spot account, futures - perpetual contract account, delivery - delivery contract account, options - options account                                                            |
| » currency         | body | string | true     | Transfer currency name                                                                                                                                                                                                    |
| » amount           | body | string | true     | Transfer amount                                                                                                                                                                                                           |
| » direction        | body | string | true     | Transfer direction: to - transfer into sub-account, from - transfer out from sub-account                                                                                                                                  |
| » client_order_id  | body | string | false    | Customer-defined ID to prevent duplicate transfers. Can be a combination of letters (case-sensitive), numbers, hyphens '-', and underscores '\_'. Can be pure letters or pure numbers with length between 1-64 characters |

> Example responses

> 200 Response

```
{
  "tx_id": 59636381286
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-responses)

| Status | Meaning                                                                    | Description                   | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Transfer operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#transferwithsubaccount-responseschema)

Status Code **200**

_TransactionID_

| Name    | Type           | Description |
| ------- | -------------- | ----------- |
| » tx_id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-transfer-records-between-main-and-sub-accounts) Get transfer records between main and sub accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-transfer-records-between-main-and-sub-accounts](https://www.gate.io/docs/developers/apiv4/en/#get-transfer-records-between-main-and-sub-accounts)

> Code samples

`GET /wallet/sub_account_transfers`

_Get transfer records between main and sub accounts_

Record query time range cannot exceed 30 days

> Note: Only records after 2020-04-10 can be retrieved

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-parameters)

| Name    | In    | Type           | Required | Description                                                                                                                        |
| ------- | ----- | -------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| sub_uid | query | string         | false    | Sub-account user ID, you can query multiple records separated by `,`. If not specified, it will return records of all sub-accounts |
| from    | query | integer(int64) | false    | Start time for querying records, defaults to 7 days before current time if not specified                                           |
| to      | query | integer(int64) | false    | End timestamp for the query, defaults to current time if not specified                                                             |
| limit   | query | integer        | false    | Maximum number of records returned in a single list                                                                                |
| offset  | query | integer        | false    | List offset, starting from 0                                                                                                       |

> Example responses

> 200 Response

```
[
  {
    "timest": "1592809000",
    "uid": "10001",
    "sub_account": "10002",
    "sub_account_type": "spot",
    "currency": "BTC",
    "amount": "1",
    "direction": "to",
    "source": "web",
    "client_order_id": "da3ce7a088c8b0372b741419c7829033",
    "status": "success"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounttransfers-responseschema)

Status Code **200**

| Name               | Type   | Description                                                                                                                                                                                                               |
| ------------------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » timest           | string | Transfer timestamp                                                                                                                                                                                                        |
| » uid              | string | Main account user ID                                                                                                                                                                                                      |
| » sub_account      | string | Sub account user ID                                                                                                                                                                                                       |
| » sub_account_type | string | Target sub-account trading account: spot - spot account, futures - perpetual contract account, delivery - delivery contract account, options - options account                                                            |
| » currency         | string | Transfer currency name                                                                                                                                                                                                    |
| » amount           | string | Transfer amount                                                                                                                                                                                                           |
| » direction        | string | Transfer direction: to - transfer into sub-account, from - transfer out from sub-account                                                                                                                                  |
| » source           | string | Source of the transfer operation                                                                                                                                                                                          |
| » client_order_id  | string | Customer-defined ID to prevent duplicate transfers. Can be a combination of letters (case-sensitive), numbers, hyphens '-', and underscores '\_'. Can be pure letters or pure numbers with length between 1-64 characters |
| » status           | string | Sub-account transfer record status, currently only 'success'                                                                                                                                                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#transfer-between-sub-accounts) Transfer between sub-accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-between-sub-accounts](https://www.gate.io/docs/developers/apiv4/en/#transfer-between-sub-accounts)

> Code samples

`POST /wallet/sub_account_to_sub_account`

_Transfer between sub-accounts_

Supports balance transfers between two sub-accounts under the same main account.
You can use either the main account's API Key or the source sub-account's API
Key to perform the operation

> Body parameter

```
{
  "currency": "usdt",
  "sub_account_from": "10001",
  "sub_account_from_type": "spot",
  "sub_account_to": "10002",
  "sub_account_to_type": "spot",
  "amount": "1"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-parameters)

| Name                    | In   | Type   | Required | Description                                                                                                                         |
| ----------------------- | ---- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| body                    | body | object | true     | none                                                                                                                                |
| » currency              | body | string | true     | Transfer currency name                                                                                                              |
| » sub_account_type      | body | string | false    | Transfer from account (deprecated, use `sub_account_from_type` and `sub_account_to_type` instead)                                   |
| » sub_account_from      | body | string | true     | Transfer from the user id of the sub-account                                                                                        |
| » sub_account_from_type | body | string | true     | Source sub-account trading account: spot - spot account, futures - perpetual contract account, delivery - delivery contract account |
| » sub_account_to        | body | string | true     | Transfer to the user id of the sub-account                                                                                          |
| » sub_account_to_type   | body | string | true     | Target sub-account trading account: spot - spot account, futures - perpetual contract account, delivery - delivery contract account |
| » amount                | body | string | true     | Transfer amount                                                                                                                     |

> Example responses

> 200 Response

```
{
  "tx_id": 59636381286
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-responses)

| Status | Meaning                                                                    | Description                   | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Transfer operation successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#subaccounttosubaccount-responseschema)

Status Code **200**

_TransactionID_

| Name    | Type           | Description |
| ------- | -------------- | ----------- |
| » tx_id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#transfer-status-query) Transfer status query

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#transfer-status-query](https://www.gate.io/docs/developers/apiv4/en/#transfer-status-query)

> Code samples

`GET /wallet/order_status`

_Transfer status query_

Supports querying transfer status based on user-defined client_order_id or tx_id
returned by the transfer interface

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-parameters](https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-parameters)

| Name            | In    | Type   | Required | Description                                                                                                                                                                                                               |
| --------------- | ----- | ------ | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| client_order_id | query | string | false    | Customer-defined ID to prevent duplicate transfers. Can be a combination of letters (case-sensitive), numbers, hyphens '-', and underscores '\_'. Can be pure letters or pure numbers with length between 1-64 characters |
| tx_id           | query | string | false    | Transfer operation number, cannot be empty at the same time as client_order_id                                                                                                                                            |

> Example responses

> 200 Response

```
{
  "tx_id": "59636381286",
  "status": "SUCCESS"
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-responses](https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-responses)

| Status | Meaning                                                                    | Description                            | Schema |
| ------ | -------------------------------------------------------------------------- | -------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Transfer status retrieved successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-responseschema](https://www.gate.io/docs/developers/apiv4/en/#gettransferorderstatus-responseschema)

Status Code **200**

_TransferOrderStatus_

| Name     | Type   | Description                                                                                                                                                                                       |
| -------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » tx_id  | string | Order ID                                                                                                                                                                                          |
| » status | string | Transfer status: PENDING - Processing, SUCCESS - Transfer successful, FAIL - Transfer failed, PARTIAL_SUCCESS - Partially successful (this status appears when transferring between sub-accounts) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-withdrawal-status) Query withdrawal status

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-withdrawal-status](https://www.gate.io/docs/developers/apiv4/en/#query-withdrawal-status)

> Code samples

`GET /wallet/withdraw_status`

_Query withdrawal status_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-parameters](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-parameters)

| Name     | In    | Type   | Required | Description                      |
| -------- | ----- | ------ | -------- | -------------------------------- |
| currency | query | string | false    | Query by specified currency name |

> Example responses

> 200 Response

```
[
  {
    "currency": "GT",
    "name": "GateToken",
    "name_cn": "GateToken",
    "deposit": "0",
    "withdraw_percent": "0%",
    "withdraw_fix": "0.01",
    "withdraw_day_limit": "20000",
    "withdraw_day_limit_remain": "20000",
    "withdraw_amount_mini": "0.11",
    "withdraw_eachtime_limit": "20000",
    "withdraw_fix_on_chains": {
      "BTC": "20",
      "ETH": "15",
      "TRX": "0",
      "EOS": "2.5"
    },
    "withdraw_percent_on_chains": {
      "ETH": "0%",
      "GTEVM": "0%"
    }
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-responses](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listwithdrawstatus-responseschema)

Status Code **200**

| Name                         | Type   | Description                                  |
| ---------------------------- | ------ | -------------------------------------------- |
| » currency                   | string | Currency                                     |
| » name                       | string | Currency name                                |
| » name_cn                    | string | Currency Chinese name                        |
| » deposit                    | string | Deposit fee                                  |
| » withdraw_percent           | string | Withdrawal fee rate percentage               |
| » withdraw_fix               | string | Fixed withdrawal fee                         |
| » withdraw_day_limit         | string | Daily allowed withdrawal amount              |
| » withdraw_amount_mini       | string | Minimum withdrawal amount                    |
| » withdraw_day_limit_remain  | string | Daily withdrawal amount left                 |
| » withdraw_eachtime_limit    | string | Maximum amount for each withdrawal           |
| » withdraw_fix_on_chains     | object | Fixed withdrawal fee on multiple chains      |
| »» **additionalProperties**  | string | none                                         |
| » withdraw_percent_on_chains | object | Percentage withdrawal fee on multiple chains |
| »» **additionalProperties**  | string | none                                         |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-sub-account-balance-information) Query sub-account balance information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-balance-information](https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-balance-information)

> Code samples

`GET /wallet/sub_account_balances`

_Query sub-account balance information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-parameters)

| Name    | In    | Type   | Required | Description                                                                                                                        |
| ------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| sub_uid | query | string | false    | Sub-account user ID, you can query multiple records separated by `,`. If not specified, it will return records of all sub-accounts |

> Example responses

> 200 Response

```
[
  {
    "uid": "10003",
    "available": {
      "BTC": "0.1",
      "GT": "2000",
      "USDT": "10"
    }
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountbalances-responseschema)

Status Code **200**

| Name                        | Type   | Description                      |
| --------------------------- | ------ | -------------------------------- |
| » uid                       | string | User ID                          |
| » available                 | object | Available balances of currencies |
| »» **additionalProperties** | string | none                             |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-sub-account-isolated-margin-account-balance-information) Query sub-account isolated margin account balance information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-isolated-margin-account-balance-information](https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-isolated-margin-account-balance-information)

> Code samples

`GET /wallet/sub_account_margin_balances`

_Query sub-account isolated margin account balance information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-parameters)

| Name    | In    | Type   | Required | Description                                                                                                                        |
| ------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| sub_uid | query | string | false    | Sub-account user ID, you can query multiple records separated by `,`. If not specified, it will return records of all sub-accounts |

> Example responses

> 200 Response

```
[
  {
    "uid": "10000",
    "available": [
      {
        "locked": false,
        "currency_pair": "BTC_USDT",
        "risk": "9999.99",
        "base": {
          "available": "0.1",
          "borrowed": "0",
          "interest": "0",
          "currency": "BTC",
          "locked": "0"
        },
        "quote": {
          "available": "0",
          "borrowed": "0",
          "interest": "0",
          "currency": "USDT",
          "locked": "0"
        }
      }
    ]
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountmarginbalances-responseschema)

Status Code **200**

| Name              | Type    | Description                                                                                                                                                       |
| ----------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » uid             | string  | User ID                                                                                                                                                           |
| » available       | array   | Margin account balances                                                                                                                                           |
| »» _None_         | object  | Margin account information for a trading pair. `base` corresponds to base currency account information, `quote` corresponds to quote currency account information |
| »»» currency_pair | string  | Trading pair                                                                                                                                                      |
| »»» account_type  | string  | Account type: risk - risk rate account, mmr - maintenance margin rate account, inactive - market not activated                                                    |
| »»» leverage      | string  | User's current market leverage multiplier                                                                                                                         |
| »»» locked        | boolean | Whether the account is locked                                                                                                                                     |
| »»» risk          | string  | Current risk rate of the margin account (returned when the account is a risk rate account)                                                                        |
| »»» mmr           | string  | Leveraged Account Current Maintenance Margin Rate (returned when the Account is Account)                                                                          |
| »»» base          | object  | Currency account information                                                                                                                                      |
| »»»» currency     | string  | Currency name                                                                                                                                                     |
| »»»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                |
| »»»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                   |
| »»»» borrowed     | string  | Borrowed funds                                                                                                                                                    |
| »»»» interest     | string  | Unpaid interest                                                                                                                                                   |
| »»» quote         | object  | Currency account information                                                                                                                                      |
| »»»» currency     | string  | Currency name                                                                                                                                                     |
| »»»» available    | string  | Amount available for margin trading, available = margin + borrowed                                                                                                |
| »»»» locked       | string  | Frozen funds, such as amounts already placed in margin market for order trading                                                                                   |
| »»»» borrowed     | string  | Borrowed funds                                                                                                                                                    |
| »»»» interest     | string  | Unpaid interest                                                                                                                                                   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-sub-account-perpetual-futures-account-balance-information) Query sub-account perpetual futures account balance information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-perpetual-futures-account-balance-information](https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-perpetual-futures-account-balance-information)

> Code samples

`GET /wallet/sub_account_futures_balances`

_Query sub-account perpetual futures account balance information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-parameters)

| Name    | In    | Type   | Required | Description                                                                                                                        |
| ------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| sub_uid | query | string | false    | Sub-account user ID, you can query multiple records separated by `,`. If not specified, it will return records of all sub-accounts |
| settle  | query | string | false    | Query balance of specified settlement currency                                                                                     |

> Example responses

> 200 Response

```
[
  [
    {
      "available": {
        "btc": {
          "available": "0.0009",
          "bonus": "0",
          "cross_available": "0.0009",
          "cross_initial_margin": "0",
          "cross_maintenance_margin": "0",
          "cross_order_margin": "0",
          "cross_unrealised_pnl": "0",
          "currency": "BTC",
          "enable_credit": false,
          "enable_evolved_classic": true,
          "enable_new_dual_mode": false,
          "history": {
            "bonus_dnw": "0",
            "bonus_offset": "0",
            "cross_settle": "0",
            "dnw": "0.0009",
            "fee": "0",
            "fund": "0",
            "pnl": "0",
            "point_dnw": "0",
            "point_fee": "0",
            "point_refr": "0",
            "refr": "0"
          },
          "in_dual_mode": false,
          "isolated_position_margin": "0",
          "maintenance_margin": "0",
          "margin_mode": 0,
          "margin_mode_name": "classic",
          "order_margin": "0",
          "point": "0",
          "position_initial_margin": "0",
          "position_margin": "0",
          "total": "0.0009",
          "unrealised_pnl": "0",
          "update_id": 11,
          "update_time": 1741766400,
          "user": 10003
        },
        "usd": {},
        "usdt": {
          "available": "500.7",
          "bonus": "0",
          "cross_available": "500.7",
          "cross_initial_margin": "0",
          "cross_maintenance_margin": "0",
          "cross_order_margin": "0",
          "cross_unrealised_pnl": "0",
          "currency": "USDT",
          "enable_credit": true,
          "enable_evolved_classic": true,
          "enable_new_dual_mode": true,
          "history": {
            "bonus_dnw": "0",
            "bonus_offset": "0",
            "cross_settle": "-1.854650083",
            "dnw": "1.89047097",
            "fee": "-0.141010882",
            "fund": "0",
            "pnl": "0.10519",
            "point_dnw": "0",
            "point_fee": "0",
            "point_refr": "0",
            "refr": "0"
          },
          "in_dual_mode": true,
          "isolated_position_margin": "0",
          "maintenance_margin": "0",
          "margin_mode": 1,
          "margin_mode_name": "multi_currency",
          "order_margin": "0",
          "point": "0",
          "position_initial_margin": "0",
          "position_margin": "0",
          "total": "0.000000005",
          "unrealised_pnl": "0",
          "update_id": 37,
          "update_time": 1741766400,
          "user": 10003
        }
      },
      "uid": "10003"
    }
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountfuturesbalances-responseschema)

Status Code **200**

| Name                                                                           | Type    | Description                                                                                                                                                                                     |
| ------------------------------------------------------------------------------ | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » uid                                                                          | string  | User ID                                                                                                                                                                                         |
| » available                                                                    | object  | Futures account balances                                                                                                                                                                        |
| »» **additionalProperties**                                                    | object  | none                                                                                                                                                                                            |
| »»» total                                                                      | string  | total is the balance after the user's accumulated deposit, withdraw, profit and loss (including realized profit and loss, fund, fee and referral rebate), excluding unrealized profit and loss. |
| total = SUM(history_dnw, history_pnl, history_fee, history_refr, history_fund) |
| »»» unrealised_pnl                                                             | string  | Unrealized PNL                                                                                                                                                                                  |
| »»» position_margin                                                            | string  | Position margin                                                                                                                                                                                 |
| »»» order_margin                                                               | string  | Order margin of unfinished orders                                                                                                                                                               |
| »»» available                                                                  | string  | Available balance for transferring or trading (including bonus. Bonus cannot be withdrawn, so transfer amount needs to deduct bonus)                                                            |
| »»» point                                                                      | string  | Point card amount                                                                                                                                                                               |
| »»» currency                                                                   | string  | Settlement currency                                                                                                                                                                             |
| »»» in_dual_mode                                                               | boolean | Whether dual mode is enabled                                                                                                                                                                    |
| »»» position_mode                                                              | string  | 持仓模式，single-单向持仓，dual-双向持仓，split-分仓(in_dual_mode失效了)                                                                                                                        |
| »»» enable_credit                                                              | boolean | Whether portfolio margin account mode is enabled                                                                                                                                                |
| »»» position_initial_margin                                                    | string  | Initial margin occupied by positions, applicable to unified account mode                                                                                                                        |
| »»» maintenance_margin                                                         | string  | Maintenance margin occupied by positions, applicable to new classic account margin mode and unified account mode                                                                                |
| »»» bonus                                                                      | string  | Bonus                                                                                                                                                                                           |
| »»» enable_evolved_classic                                                     | boolean | Classic account margin mode, true-new mode, false-old mode                                                                                                                                      |
| »»» cross_order_margin                                                         | string  | Cross margin order margin, applicable to new classic account margin mode                                                                                                                        |
| »»» cross_initial_margin                                                       | string  | Cross margin initial margin, applicable to new classic account margin mode                                                                                                                      |
| »»» cross_maintenance_margin                                                   | string  | Cross margin maintenance margin, applicable to new classic account margin mode                                                                                                                  |
| »»» cross_unrealised_pnl                                                       | string  | Cross margin unrealized P&L, applicable to new classic account margin mode                                                                                                                      |
| »»» cross_available                                                            | string  | Cross margin available balance, applicable to new classic account margin mode                                                                                                                   |
| »»» cross_margin_balance                                                       | string  | Cross margin balance, applicable to new classic account margin mode                                                                                                                             |
| »»» cross_mmr                                                                  | string  | Cross margin maintenance margin rate, applicable to new classic account margin mode                                                                                                             |
| »»» cross_imr                                                                  | string  | Cross margin initial margin rate, applicable to new classic account margin mode                                                                                                                 |
| »»» isolated_position_margin                                                   | string  | Isolated position margin, applicable to new classic account margin mode                                                                                                                         |
| »»» enable_new_dual_mode                                                       | boolean | Whether to open a new two-way position mode                                                                                                                                                     |
| »»» margin_mode                                                                | integer | Margin mode, 0-classic margin mode, 1-cross-currency margin mode, 2-combined margin mode                                                                                                        |
| »»» enable_tiered_mm                                                           | boolean | Whether to enable tiered maintenance margin calculation                                                                                                                                         |
| »»» position_voucher_total                                                     | string  | Total Position Experience Coupon Amount in Account                                                                                                                                              |
| »»» history                                                                    | object  | Statistical data                                                                                                                                                                                |
| »»»» dnw                                                                       | string  | total amount of deposit and withdraw                                                                                                                                                            |
| »»»» pnl                                                                       | string  | total amount of trading profit and loss                                                                                                                                                         |
| »»»» fee                                                                       | string  | total amount of fee                                                                                                                                                                             |
| »»»» refr                                                                      | string  | total amount of referrer rebates                                                                                                                                                                |
| »»»» fund                                                                      | string  | total amount of funding costs                                                                                                                                                                   |
| »»»» point_dnw                                                                 | string  | total amount of point deposit and withdraw                                                                                                                                                      |
| »»»» point_fee                                                                 | string  | total amount of point fee                                                                                                                                                                       |
| »»»» point_refr                                                                | string  | total amount of referrer rebates of point fee                                                                                                                                                   |
| »»»» bonus_dnw                                                                 | string  | total amount of perpetual contract bonus transfer                                                                                                                                               |
| »»»» bonus_offset                                                              | string  | total amount of perpetual contract bonus deduction                                                                                                                                              |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-sub-account-cross-margin-account-balance-information) Query sub-account cross margin account balance information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-cross-margin-account-balance-information](https://www.gate.io/docs/developers/apiv4/en/#query-sub-account-cross-margin-account-balance-information)

> Code samples

`GET /wallet/sub_account_cross_margin_balances`

_Query sub-account cross margin account balance information_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-parameters)

| Name    | In    | Type   | Required | Description                                                                                                                        |
| ------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| sub_uid | query | string | false    | Sub-account user ID, you can query multiple records separated by `,`. If not specified, it will return records of all sub-accounts |

> Example responses

> 200 Response

```
[
  {
    "uid": "100000",
    "available": {
      "user_id": 100003,
      "locked": false,
      "total": "20.000000",
      "borrowed": "0.000000",
      "interest": "0",
      "borrowed_net": "0",
      "net": "20",
      "leverage": "3",
      "risk": "9999.99",
      "total_initial_margin": "0.00",
      "total_margin_balance": "20.00",
      "total_maintenance_margin": "0.00",
      "total_initial_margin_rate": "9999.9900",
      "total_maintenance_margin_rate": "9999.9900",
      "total_available_margin": "20.00",
      "balances": {
        "USDT": {
          "available": "20.000000",
          "freeze": "0.000000",
          "borrowed": "0.000000",
          "interest": "0.000000"
        }
      }
    }
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountcrossmarginbalances-responseschema)

Status Code **200**

| Name                              | Type           | Description                                                                                                            |
| --------------------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------------- |
| » uid                             | string         | User ID                                                                                                                |
| » available                       | object         | none                                                                                                                   |
| »» user_id                        | integer(int64) | Cross margin account user ID. 0 means this sub-account has not yet opened a cross margin account                       |
| »» locked                         | boolean        | Whether the account is locked                                                                                          |
| »» balances                       | object         | none                                                                                                                   |
| »»» CrossMarginBalance            | object         | none                                                                                                                   |
| »»»» available                    | string         | Available balance                                                                                                      |
| »»»» freeze                       | string         | Locked balance                                                                                                         |
| »»»» borrowed                     | string         | Borrowed balance                                                                                                       |
| »»»» interest                     | string         | Unpaid interest                                                                                                        |
| »»» total                         | string         | Total account value in USDT, i.e., the sum of all currencies' `(available+freeze)*price*discount`                      |
| »»» borrowed                      | string         | Total borrowed value in USDT, i.e., the sum of all currencies' `borrowed*price*discount`                               |
| »»» borrowed_net                  | string         | Total borrowed value in USDT \* leverage factor                                                                        |
| »»» net                           | string         | Total net assets in USDT                                                                                               |
| »»» leverage                      | string         | Position leverage                                                                                                      |
| »»» interest                      | string         | Total unpaid interest in USDT, i.e., the sum of all currencies' `interest*price*discount`                              |
| »»» risk                          | string         | Risk rate. When it falls below 110%, liquidation will be triggered. Calculation formula: `total / (borrowed+interest)` |
| »»» total_initial_margin          | string         | Total initial margin                                                                                                   |
| »»» total_margin_balance          | string         | Total margin balance                                                                                                   |
| »»» total_maintenance_margin      | string         | Total maintenance margin                                                                                               |
| »»» total_initial_margin_rate     | string         | Total initial margin rate                                                                                              |
| »»» total_maintenance_margin_rate | string         | Total maintenance margin rate                                                                                          |
| »»» total_available_margin        | string         | Total available margin                                                                                                 |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-withdrawal-address-whitelist) Query withdrawal address whitelist

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-withdrawal-address-whitelist](https://www.gate.io/docs/developers/apiv4/en/#query-withdrawal-address-whitelist)

> Code samples

`GET /wallet/saved_address`

_Query withdrawal address whitelist_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-parameters)

| Name     | In    | Type    | Required | Description                        |
| -------- | ----- | ------- | -------- | ---------------------------------- |
| currency | query | string  | true     | Currency                           |
| chain    | query | string  | false    | Chain name                         |
| limit    | query | string  | false    | Maximum number returned, up to 100 |
| page     | query | integer | false    | Page number                        |

> Example responses

> 200 Response

```
[
  {
    "currency": "usdt",
    "chain": "TRX",
    "address": "TWYirLzw2RARB2jfeFcfRPmeuU3rC7rakT",
    "name": "gate",
    "tag": "",
    "verified": "1"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-responses](https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsavedaddress-responseschema)

Status Code **200**

| Name       | Type   | Description                                               |
| ---------- | ------ | --------------------------------------------------------- |
| » currency | string | Currency                                                  |
| » chain    | string | Chain name                                                |
| » address  | string | Address                                                   |
| » name     | string | Name                                                      |
| » tag      | string | Tag                                                       |
| » verified | string | Whether to pass the verification 0-unverified, 1-verified |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-trading-fees) Query personal trading fees

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-fees](https://www.gate.io/docs/developers/apiv4/en/#query-personal-trading-fees)

> Code samples

`GET /wallet/fee`

_Query personal trading fees_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettradefee-parameters](https://www.gate.io/docs/developers/apiv4/en/#gettradefee-parameters)

| Name          | In    | Type   | Required | Description                                                                        |
| ------------- | ----- | ------ | -------- | ---------------------------------------------------------------------------------- |
| currency_pair | query | string | false    | Specify currency pair to get more accurate fee settings.                           |
| settle        | query | string | false    | Specify the settlement currency of the contract to get more accurate fee settings. |

#### [#](#detailed-descriptions-3) Detailed descriptions

**currency_pair**: Specify currency pair to get more accurate fee settings.

This field is optional. Usually fee settings are the same for all currency
pairs.

**settle**: Specify the settlement currency of the contract to get more accurate
fee settings.

This field is optional. Generally, the fee settings for all settlement
currencies are the same.

#### [#](#enumerated-values-2) Enumerated Values

| Parameter | Value |
| --------- | ----- |
| settle    | BTC   |
| settle    | USDT  |
| settle    | USD   |

> Example responses

> 200 Response

```
{
  "user_id": 10001,
  "taker_fee": "0.002",
  "maker_fee": "0.002",
  "futures_taker_fee": "-0.00025",
  "futures_maker_fee": "0.00075",
  "gt_discount": false,
  "gt_taker_fee": "0",
  "gt_maker_fee": "0",
  "loan_fee": "0.18",
  "point_type": "1",
  "delivery_taker_fee": "0.00016",
  "delivery_maker_fee": "-0.00015",
  "debit_fee": 3
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettradefee-responses](https://www.gate.io/docs/developers/apiv4/en/#gettradefee-responses)

| Status | Meaning                                                                    | Description      | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettradefee-responseschema](https://www.gate.io/docs/developers/apiv4/en/#gettradefee-responseschema)

Status Code **200**

| Name                 | Type           | Description                                                                          |
| -------------------- | -------------- | ------------------------------------------------------------------------------------ |
| » user_id            | integer(int64) | User ID                                                                              |
| » taker_fee          | string         | taker fee rate                                                                       |
| » maker_fee          | string         | maker fee rate                                                                       |
| » gt_discount        | boolean        | Whether GT deduction discount is enabled                                             |
| » gt_taker_fee       | string         | Taker fee rate if using GT deduction. It will be 0 if GT deduction is disabled       |
| » gt_maker_fee       | string         | Maker fee rate with GT deduction. Returns 0 if GT deduction is disabled              |
| » loan_fee           | string         | Loan fee rate of margin lending                                                      |
| » point_type         | string         | Point card type: 0 - Original version, 1 - New version since 202009                  |
| » futures_taker_fee  | string         | Perpetual contract taker fee rate                                                    |
| » futures_maker_fee  | string         | Perpetual contract maker fee rate                                                    |
| » delivery_taker_fee | string         | Delivery contract taker fee rate                                                     |
| » delivery_maker_fee | string         | Delivery contract maker fee rate                                                     |
| » debit_fee          | integer        | Deduction types for rates, 1 - GT deduction, 2 - Point card deduction, 3 - VIP rates |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-personal-account-totals) Query personal account totals

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-personal-account-totals](https://www.gate.io/docs/developers/apiv4/en/#query-personal-account-totals)

> Code samples

`GET /wallet/total_balance`

_Query personal account totals_

This query endpoint returns the total _estimated value_ of all currencies in
each account converted to the input currency. Exchange rates and related account
balance information may be cached for up to 1 minute. It is not recommended to
use this interface data for real-time calculations.

For real-time calculations, query the corresponding balance interface based on
account type, such as:

- `GET /spot/accounts` to query spot account
- `GET /margin/accounts` to query margin account
- `GET /futures/{settle}/accounts` to query futures account

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-parameters](https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-parameters)

| Name     | In    | Type   | Required | Description                                                                                                 |
| -------- | ----- | ------ | -------- | ----------------------------------------------------------------------------------------------------------- |
| currency | query | string | false    | Target currency type for statistical conversion. Accepts BTC, CNY, USD, and USDT. USDT is the default value |

> Example responses

> 200 Response

```
{
  "details": {
    "cross_margin": {
      "amount": "0",
      "currency": "USDT"
    },
    "spot": {
      "currency": "USDT",
      "amount": "42264489969935775.5160259954878034182418"
    },
    "finance": {
      "amount": "662714381.70310327810191647181",
      "currency": "USDT"
    },
    "margin": {
      "amount": "1259175.664137668554329559",
      "currency": "USDT",
      "borrowed": "0.00"
    },
    "quant": {
      "amount": "591702859674467879.6488202650892478553852",
      "currency": "USDT"
    },
    "futures": {
      "amount": "2384175.5606114082065",
      "currency": "USDT",
      "unrealised_pnl": "0.00"
    },
    "delivery": {
      "currency": "USDT",
      "amount": "1519804.9756702",
      "unrealised_pnl": "0.00"
    },
    "warrant": {
      "amount": "0",
      "currency": "USDT"
    },
    "cbbc": {
      "currency": "USDT",
      "amount": "0"
    }
  },
  "total": {
    "currency": "USDT",
    "amount": "633967350312281193.068368815439797304437",
    "unrealised_pnl": "0.00",
    "borrowed": "0.00"
  }
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-responses](https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-responses)

| Status | Meaning                                                                    | Description                                | Schema |
| ------ | -------------------------------------------------------------------------- | ------------------------------------------ | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Request is valid and successfully returned | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-responseschema](https://www.gate.io/docs/developers/apiv4/en/#gettotalbalance-responseschema)

Status Code **200**

_User's total balance information_

| Name              | Type   | Description                                                                                   |
| ----------------- | ------ | --------------------------------------------------------------------------------------------- |
| » total           | object | Total balances calculated with specified currency unit                                        |
| »» amount         | string | Account total balance amount                                                                  |
| »» currency       | string | Currency                                                                                      |
| »» unrealised_pnl | string | Unrealised_pnl, this field will only appear in futures, options, delivery, and total accounts |
| »» borrowed       | string | Total borrowed amount, this field will only appear in margin and cross_margin accounts        |
| » details         | object | Total balances in different accounts                                                          |

\- cross_margin: cross margin account  
\- spot: spot account  
\- finance: finance account  
\- margin: margin account  
\- quant: quant account  
\- futures: perpetual contract account  
\- delivery: delivery contract account  
\- warrant: warrant account  
\- cbbc: CBBC account | | »» **additionalProperties** | object | Total balances
calculated with specified currency unit | | »»» amount | string | Account total
balance amount | | »»» currency | string | Currency | | »»» unrealised_pnl |
string | Unrealised_pnl, this field will only appear in futures, options,
delivery, and total accounts | | »»» borrowed | string | Total borrowed amount,
this field will only appear in margin and cross_margin accounts |

#### [#](#enumerated-values-3) Enumerated Values

| Property | Value |
| -------- | ----- |
| currency | BTC   |
| currency | CNY   |
| currency | USD   |
| currency | USDT  |
| currency | BTC   |
| currency | CNY   |
| currency | USD   |
| currency | USDT  |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-list-of-convertible-small-balance-currencies) Get list of convertible small balance currencies

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-list-of-convertible-small-balance-currencies](https://www.gate.io/docs/developers/apiv4/en/#get-list-of-convertible-small-balance-currencies)

> Code samples

`GET /wallet/small_balance`

_Get list of convertible small balance currencies_

> Example responses

> 200 Response

```
[
  [
    {
      "currency": "FLOKI",
      "available_balance": "182.29400000",
      "estimated_as_btc": "0.00000012",
      "convertible_to_gt": "0.001080"
    },
    {
      "currency": "MBLK",
      "available_balance": "0.91723337",
      "estimated_as_btc": "0.00000102",
      "convertible_to_gt": "0.009188"
    }
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsmallbalance-responses](https://www.gate.io/docs/developers/apiv4/en/#listsmallbalance-responses)

| Status | Meaning                                                                    | Description | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsmallbalance-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsmallbalance-responseschema)

Status Code **200**

| Name                 | Type   | Description                |
| -------------------- | ------ | -------------------------- |
| » _None_             | object | Small Balance Conversion   |
| »» currency          | string | Currency                   |
| »» available_balance | string | Available balance          |
| »» estimated_as_btc  | string | Estimated as BTC           |
| »» convertible_to_gt | string | Estimated conversion to GT |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#convert-small-balance-currency) Convert small balance currency

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#convert-small-balance-currency](https://www.gate.io/docs/developers/apiv4/en/#convert-small-balance-currency)

> Code samples

`POST /wallet/small_balance`

_Convert small balance currency_

> Body parameter

```
{
  "currency": [
    "FLOKI",
    "MBLK"
  ],
  "is_all": true
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#convertsmallbalance-parameters](https://www.gate.io/docs/developers/apiv4/en/#convertsmallbalance-parameters)

| Name       | In   | Type    | Required | Description              |
| ---------- | ---- | ------- | -------- | ------------------------ |
| body       | body | object  | true     | none                     |
| » currency | body | array   | false    | Currency to be converted |
| » is_all   | body | boolean | false    | Whether to convert all   |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#convertsmallbalance-responses](https://www.gate.io/docs/developers/apiv4/en/#convertsmallbalance-responses)

| Status | Meaning                                                                    | Description | Schema |
| ------ | -------------------------------------------------------------------------- | ----------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-convertible-small-balance-currency-history) Get convertible small balance currency history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-convertible-small-balance-currency-history](https://www.gate.io/docs/developers/apiv4/en/#get-convertible-small-balance-currency-history)

> Code samples

`GET /wallet/small_balance_history`

_Get convertible small balance currency history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-parameters)

| Name     | In    | Type           | Required | Description                                                              |
| -------- | ----- | -------------- | -------- | ------------------------------------------------------------------------ |
| currency | query | string         | false    | Currency to convert                                                      |
| page     | query | integer(int32) | false    | Page number                                                              |
| limit    | query | integer(int32) | false    | Maximum number of items returned. Default: 100, minimum: 1, maximum: 100 |

> Example responses

> 200 Response

```
[
  [
    {
      "id": "28583810",
      "create_time": 1706670777,
      "currency": "FLOKI",
      "amount": "182.29400000",
      "gt_amount": "0.001079"
    }
  ]
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-responses](https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-responses)

| Status | Meaning                                                                    | Description | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsmallbalancehistory-responseschema)

Status Code **200**

| Name           | Type           | Description                |
| -------------- | -------------- | -------------------------- |
| » _None_       | object         | Small Balance Conversion   |
| »» id          | string         | Order ID                   |
| »» currency    | string         | Currency                   |
| »» amount      | string         | Swap Amount                |
| »» gt_amount   | string         | GT amount                  |
| »» create_time | integer(int64) | Exchange time (in seconds) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-uid-transfer-history) Get UID transfer history

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-uid-transfer-history](https://www.gate.io/docs/developers/apiv4/en/#get-uid-transfer-history)

> Code samples

`GET /wallet/push`

_Get UID transfer history_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpushorders-parameters](https://www.gate.io/docs/developers/apiv4/en/#listpushorders-parameters)

| Name             | In    | Type           | Required | Description                                                                                                              |
| ---------------- | ----- | -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------ |
| id               | query | integer(int32) | false    | Order ID                                                                                                                 |
| from             | query | integer(int32) | false    | Start time for querying records. If not specified, defaults to 7 days before the current time. Unix timestamp in seconds |
| to               | query | integer(int32) | false    | End time for querying records. If not specified, defaults to the current time. Unix timestamp in seconds                 |
| limit            | query | integer(int32) | false    | Maximum number of items returned in the list, default value is 100                                                       |
| offset           | query | integer(int32) | false    | List offset, starting from 0                                                                                             |
| transaction_type | query | string         | false    | Order type returned in the list: `withdraw`, `deposit`. Default is `withdraw`.                                           |

> Example responses

> 200 Response

```
[
  {
    "id": 111,
    "push_uid": 1132,
    "receive_uid": 12324,
    "currency": "BTC",
    "amount": "1.2",
    "status": "PENDING",
    "create_time": 1706670777,
    "message": "The other party has not completed KYC,There is a security risk in your account, please contact customer service",
    "transaction_type": "withdraw"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpushorders-responses](https://www.gate.io/docs/developers/apiv4/en/#listpushorders-responses)

| Status | Meaning                                                                    | Description | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listpushorders-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listpushorders-responseschema)

Status Code **200**

| Name          | Type           | Description        |
| ------------- | -------------- | ------------------ |
| » id          | integer(int64) | Order ID           |
| » push_uid    | integer(int64) | Initiator User ID  |
| » receive_uid | integer(int64) | Recipient User ID  |
| » currency    | string         | Currency name      |
| » amount      | string         | Transfer amount    |
| » create_time | integer(int64) | Creation time      |
| » status      | string         | Withdrawal status: |

\- CREATING: Creating  
\- PENDING: Waiting for recipient (Please contact the recipient to accept the
transfer on Gate official website)  
\- CANCELLING: Cancelling  
\- CANCELLED: Cancelled  
\- REFUSING: Refusing  
\- REFUSED: Refused  
\- RECEIVING: Receiving  
\- RECEIVED: Success | | » message | string | PENDING reason tips | | »
transaction_type | string | Order Type |

WARNING

To perform this operation, you must be authenticated by API key and secret
