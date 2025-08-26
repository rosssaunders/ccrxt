# [#](#withdrawal) Withdrawal

Withdrawal API

## [#](#withdraw) Withdraw

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdraw](https://www.gate.io/docs/developers/apiv4/en/#withdraw)

> Code samples

`POST /withdrawals`

_Withdraw_

If the recipient's on-chain address is also Gate, no transaction fee will be
charged

> Body parameter

```
{
  "withdraw_order_id": "order_123456",
  "currency": "USDT",
  "address": "1HkxtBAMrA3tP5ENnYY2CZortjZvFDH5Cs",
  "amount": "222.61",
  "memo": "",
  "chain": "TRX"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdraw-parameters](https://www.gate.io/docs/developers/apiv4/en/#withdraw-parameters)

| Name                | In   | Type   | Required | Description                                                                                                                                                                               |
| ------------------- | ---- | ------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| body                | body | object | true     | none                                                                                                                                                                                      |
| » withdraw_order_id | body | string | false    | User-defined order number for withdrawal. Default is empty. When not empty, the specified user-defined order number record will be queried                                                |
| » amount            | body | string | true     | Token amount                                                                                                                                                                              |
| » currency          | body | string | true     | Currency name                                                                                                                                                                             |
| » address           | body | string | false    | Withdrawal address. Required for withdrawals                                                                                                                                              |
| » memo              | body | string | false    | Additional remarks with regards to the withdrawal                                                                                                                                         |
| » withdraw_id       | body | string | false    | Withdrawal record ID starts with 'w', such as: w1879219868. When withdraw_id is not empty, only this specific withdrawal record will be queried, and time-based querying will be disabled |
| » asset_class       | body | string | false    | Withdrawal record currency type, empty by default. Supports users to query withdrawal records in main area and innovation area on demand.                                                 |
| » chain             | body | string | true     | Name of the chain used in withdrawals                                                                                                                                                     |

#### [#](#detailed-descriptions) Detailed descriptions

**» asset_class**: Withdrawal record currency type, empty by default. Supports
users to query withdrawal records in main area and innovation area on demand.
Valid values: SPOT, PILOT

SPOT: Main area PILOT: Innovation area

> Example responses

> 200 Response

```
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
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdraw-responses](https://www.gate.io/docs/developers/apiv4/en/#withdraw-responses)

| Status | Meaning                                                                    | Description                                                                       | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Withdrawal request accepted. Check withdrawal record status for processing result | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdraw-responseschema](https://www.gate.io/docs/developers/apiv4/en/#withdraw-responseschema)

Status Code **200**

| Name                | Type   | Description                                                                                                                                                                               |
| ------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » id                | string | Record ID                                                                                                                                                                                 |
| » txid              | string | Hash record of the withdrawal                                                                                                                                                             |
| » withdraw_order_id | string | User-defined order number for withdrawal. Default is empty. When not empty, the specified user-defined order number record will be queried                                                |
| » timestamp         | string | Operation time                                                                                                                                                                            |
| » amount            | string | Token amount                                                                                                                                                                              |
| » currency          | string | Currency name                                                                                                                                                                             |
| » address           | string | Withdrawal address. Required for withdrawals                                                                                                                                              |
| » memo              | string | Additional remarks with regards to the withdrawal                                                                                                                                         |
| » withdraw_id       | string | Withdrawal record ID starts with 'w', such as: w1879219868. When withdraw_id is not empty, only this specific withdrawal record will be queried, and time-based querying will be disabled |
| » asset_class       | string | Withdrawal record currency type, empty by default. Supports users to query withdrawal records in main area and innovation area on demand.                                                 |

Valid values: SPOT, PILOT

SPOT: Main area  
PILOT: Innovation area | | » status | string | Transaction status

\- DONE: Completed  
\- CANCEL: Cancelled  
\- REQUEST: Requesting  
\- MANUAL: Pending manual review  
\- BCODE: GateCode operation  
\- EXTPEND: Sent, waiting for confirmation  
\- FAIL: Failed on chain, waiting for confirmation  
\- INVALID: Invalid order  
\- VERIFY: Verifying  
\- PROCES: Processing  
\- PEND: Processing  
\- DMOVE: Pending manual review  
\- REVIEW: Under review | | » chain | string | Name of the chain used in
withdrawals |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#uid-transfer) UID transfer

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#uid-transfer](https://www.gate.io/docs/developers/apiv4/en/#uid-transfer)

> Code samples

`POST /withdrawals/push`

_UID transfer_

Transfers between main spot accounts. Both parties cannot be sub-accounts

> Body parameter

```
{
  "receive_uid": 12233,
  "currency": "USDT",
  "amount": "1.1"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-parameters](https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-parameters)

| Name          | In   | Type           | Required | Description     |
| ------------- | ---- | -------------- | -------- | --------------- |
| body          | body | object         | true     | none            |
| » receive_uid | body | integer(int64) | true     | Recipient UID   |
| » currency    | body | string         | true     | Currency name   |
| » amount      | body | string         | true     | Transfer amount |

> Example responses

> 200 Response

```
{
  "id": 111
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-responses](https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-responses)

| Status | Meaning                                                                    | Description                                                            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------------------------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Request accepted. Check withdrawal record status for processing result | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-responseschema](https://www.gate.io/docs/developers/apiv4/en/#withdrawpushorder-responseschema)

Status Code **200**

| Name | Type           | Description |
| ---- | -------------- | ----------- |
| » id | integer(int64) | Order ID    |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#cancel-withdrawal-with-specified-id) Cancel withdrawal with specified ID

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancel-withdrawal-with-specified-id](https://www.gate.io/docs/developers/apiv4/en/#cancel-withdrawal-with-specified-id)

> Code samples

`DELETE /withdrawals/{withdrawal_id}`

_Cancel withdrawal with specified ID_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-parameters](https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-parameters)

| Name          | In   | Type   | Required | Description |
| ------------- | ---- | ------ | -------- | ----------- |
| withdrawal_id | path | string | true     | none        |

> Example responses

> 202 Response

```
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
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-responses](https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-responses)

| Status | Meaning                                                                          | Description                                                                | Schema |
| ------ | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------ |
| 202    | [Accepted (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.3) | Cancellation request accepted. Check record status for cancellation result | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-responseschema](https://www.gate.io/docs/developers/apiv4/en/#cancelwithdrawal-responseschema)

Status Code **202**

| Name                | Type   | Description                                                                                                                                                                               |
| ------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| » id                | string | Record ID                                                                                                                                                                                 |
| » txid              | string | Hash record of the withdrawal                                                                                                                                                             |
| » withdraw_order_id | string | User-defined order number for withdrawal. Default is empty. When not empty, the specified user-defined order number record will be queried                                                |
| » timestamp         | string | Operation time                                                                                                                                                                            |
| » amount            | string | Token amount                                                                                                                                                                              |
| » currency          | string | Currency name                                                                                                                                                                             |
| » address           | string | Withdrawal address. Required for withdrawals                                                                                                                                              |
| » memo              | string | Additional remarks with regards to the withdrawal                                                                                                                                         |
| » withdraw_id       | string | Withdrawal record ID starts with 'w', such as: w1879219868. When withdraw_id is not empty, only this specific withdrawal record will be queried, and time-based querying will be disabled |
| » asset_class       | string | Withdrawal record currency type, empty by default. Supports users to query withdrawal records in main area and innovation area on demand.                                                 |

Valid values: SPOT, PILOT

SPOT: Main area  
PILOT: Innovation area | | » status | string | Transaction status

\- DONE: Completed  
\- CANCEL: Cancelled  
\- REQUEST: Requesting  
\- MANUAL: Pending manual review  
\- BCODE: GateCode operation  
\- EXTPEND: Sent, waiting for confirmation  
\- FAIL: Failed on chain, waiting for confirmation  
\- INVALID: Invalid order  
\- VERIFY: Verifying  
\- PROCES: Processing  
\- PEND: Processing  
\- DMOVE: Pending manual review  
\- REVIEW: Under review | | » chain | string | Name of the chain used in
withdrawals |

WARNING

To perform this operation, you must be authenticated by API key and secret
