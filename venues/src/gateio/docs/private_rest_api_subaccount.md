# [#](#subaccount) SubAccount

Sub-account management

## [#](#create-a-new-sub-account) Create a new sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-a-new-sub-account](https://www.gate.io/docs/developers/apiv4/en/#create-a-new-sub-account)

> Code samples

`POST /sub_accounts`

_Create a new sub-account_

> Body parameter

```
{
  "remark": "remark",
  "login_name": "sub_account_for_trades"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-parameters)

| Name         | In   | Type   | Required | Description                                                                                                          |
| ------------ | ---- | ------ | -------- | -------------------------------------------------------------------------------------------------------------------- |
| body         | body | object | true     | none                                                                                                                 |
| » remark     | body | string | false    | Note                                                                                                                 |
| » login_name | body | string | true     | Sub-account login name: Only letters, numbers and underscores are supported, cannot contain other invalid characters |
| » password   | body | string | false    | The sub-account's password. (Default: the same as main account's password)                                           |
| » email      | body | string | false    | The sub-account's email address. (Default: the same as main account's email address)                                 |

> Example responses

> 201 Response

```
{
  "remark": "remark",
  "login_name": "sub_account_for_trades",
  "user_id": 10001,
  "state": 1,
  "create_time": 168888888
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-responses)

| Status | Meaning                                                                         | Description          | Schema |
| ------ | ------------------------------------------------------------------------------- | -------------------- | ------ |
| 201    | [Created (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.2) | Created successfully | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createsubaccounts-responseschema)

Status Code **201**

| Name          | Type           | Description                                                                                                          |
| ------------- | -------------- | -------------------------------------------------------------------------------------------------------------------- |
| » remark      | string         | Note                                                                                                                 |
| » login_name  | string         | Sub-account login name: Only letters, numbers and underscores are supported, cannot contain other invalid characters |
| » password    | string         | The sub-account's password. (Default: the same as main account's password)                                           |
| » email       | string         | The sub-account's email address. (Default: the same as main account's email address)                                 |
| » state       | integer(int32) | Sub-account status: 1-normal, 2-locked                                                                               |
| » type        | integer(int32) | Sub-account type: 1-Regular sub-account, 3-Cross margin sub-account                                                  |
| » user_id     | integer(int64) | Sub-account user ID                                                                                                  |
| » create_time | integer(int64) | Created time                                                                                                         |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-sub-accounts) List sub-accounts

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-sub-accounts](https://www.gate.io/docs/developers/apiv4/en/#list-sub-accounts)

> Code samples

`GET /sub_accounts`

_List sub-accounts_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-parameters)

| Name | In    | Type   | Required | Description                                                                                                            |
| ---- | ----- | ------ | -------- | ---------------------------------------------------------------------------------------------------------------------- |
| type | query | string | false    | Enter `0` to list all types of sub-accounts (currently supporting cross-margin sub-accounts and regular sub-accounts). |

#### [#](#detailed-descriptions-4) Detailed descriptions

**type**: Enter `0` to list all types of sub-accounts (currently supporting
cross-margin sub-accounts and regular sub-accounts). Enter `1` to query regular
sub-accounts only. If no parameter is passed, only regular sub-accounts will be
queried by default.

> Example responses

> 200 Response

```
[
  {
    "remark": "remark",
    "login_name": "sub_account_for_trades",
    "user_id": 10001,
    "state": 1,
    "create_time": 168888888
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listsubaccounts-responseschema)

Status Code **200**

| Name          | Type           | Description                                                                                                          |
| ------------- | -------------- | -------------------------------------------------------------------------------------------------------------------- |
| _None_        | array          | none                                                                                                                 |
| » remark      | string         | Note                                                                                                                 |
| » login_name  | string         | Sub-account login name: Only letters, numbers and underscores are supported, cannot contain other invalid characters |
| » password    | string         | The sub-account's password. (Default: the same as main account's password)                                           |
| » email       | string         | The sub-account's email address. (Default: the same as main account's email address)                                 |
| » state       | integer(int32) | Sub-account status: 1-normal, 2-locked                                                                               |
| » type        | integer(int32) | Sub-account type: 1-Regular sub-account, 3-Cross margin sub-account                                                  |
| » user_id     | integer(int64) | Sub-account user ID                                                                                                  |
| » create_time | integer(int64) | Created time                                                                                                         |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-sub-account) Get sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-sub-account](https://www.gate.io/docs/developers/apiv4/en/#get-sub-account)

> Code samples

`GET /sub_accounts/{user_id}`

_Get sub-account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-parameters)

| Name    | In   | Type           | Required | Description         |
| ------- | ---- | -------------- | -------- | ------------------- |
| user_id | path | integer(int64) | true     | Sub-account user ID |

> Example responses

> 200 Response

```
{
  "remark": "remark",
  "login_name": "sub_account_for_trades",
  "user_id": 10001,
  "state": 1,
  "create_time": 168888888
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-responses)

| Status | Meaning                                                                    | Description            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getsubaccount-responseschema)

Status Code **200**

| Name          | Type           | Description                                                                                                          |
| ------------- | -------------- | -------------------------------------------------------------------------------------------------------------------- |
| » remark      | string         | Note                                                                                                                 |
| » login_name  | string         | Sub-account login name: Only letters, numbers and underscores are supported, cannot contain other invalid characters |
| » password    | string         | The sub-account's password. (Default: the same as main account's password)                                           |
| » email       | string         | The sub-account's email address. (Default: the same as main account's email address)                                 |
| » state       | integer(int32) | Sub-account status: 1-normal, 2-locked                                                                               |
| » type        | integer(int32) | Sub-account type: 1-Regular sub-account, 3-Cross margin sub-account                                                  |
| » user_id     | integer(int64) | Sub-account user ID                                                                                                  |
| » create_time | integer(int64) | Created time                                                                                                         |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#create-new-sub-account-api-key-pair) Create new sub-account API key pair

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-new-sub-account-api-key-pair](https://www.gate.io/docs/developers/apiv4/en/#create-new-sub-account-api-key-pair)

> Code samples

`POST /sub_accounts/{user_id}/keys`

_Create new sub-account API key pair_

> Body parameter

```
{
  "mode": 1,
  "name": "spot",
  "perms": [
    {
      "read_only": false,
      "name": "options"
    },
    {
      "read_only": false,
      "name": "spot"
    },
    {
      "read_only": false,
      "name": "delivery"
    },
    {
      "read_only": false,
      "name": "wallet"
    },
    {
      "read_only": false,
      "name": "futures"
    }
  ],
  "ip_whitelist": [
    "127.0.0.1",
    "127.0.0.2"
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createsubaccountkeys-parameters](https://www.gate.io/docs/developers/apiv4/en/#createsubaccountkeys-parameters)

| Name           | In   | Type                                  | Required | Description                                               |
| -------------- | ---- | ------------------------------------- | -------- | --------------------------------------------------------- |
| user_id        | path | integer(int64)                        | true     | Sub-account user ID                                       |
| body           | body | [SubAccountKey](#schemasubaccountkey) | true     | none                                                      |
| » mode         | body | integer(int32)                        | false    | Mode: 1 - classic 2 - portfolio account                   |
| » name         | body | string                                | false    | API Key Name                                              |
| » perms        | body | array                                 | false    | none                                                      |
| »» name        | body | string                                | false    | Permission function name (no value will be cleared)       |
| »» read_only   | body | boolean                               | false    | Read Only                                                 |
| » ip_whitelist | body | array                                 | false    | IP whitelist (list will be cleared if no value is passed) |

#### [#](#detailed-descriptions-5) Detailed descriptions

**»» name**: Permission function name (no value will be cleared)

- wallet: wallet
- spot: spot/margin
- futures: perpetual contract
- delivery: delivery contract
- earn: earn
- custody: custody
- options: options
- account: account information
- loan: lending
- margin: margin
- unified: unified account
- copy: copy trading

> Example responses

> 200 Response

```
{
  "state": 1,
  "name": "spot",
  "user_id": 100000,
  "perms": [
    {
      "name": "options",
      "read_only": false
    },
    {
      "name": "spot",
      "read_only": false
    },
    {
      "name": "delivery",
      "read_only": false
    },
    {
      "name": "wallet",
      "read_only": false
    },
    {
      "name": "futures",
      "read_only": false
    }
  ],
  "ip_whitelist": [
    "127.0.0.1",
    "127.0.0.2"
  ],
  "mode": 1,
  "secret": "cddcc6e5e78060e013860bdbe5e737830b96821c027664586fb38b411808f4fd",
  "key": "eb8815bf99d7bb5f8ad6497bdc4774a8",
  "created_at": 1663683330,
  "updated_at": 1663683330
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createsubaccountkeys-responses](https://www.gate.io/docs/developers/apiv4/en/#createsubaccountkeys-responses)

| Status | Meaning                                                                    | Description          | Schema                                |
| ------ | -------------------------------------------------------------------------- | -------------------- | ------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Created successfully | [SubAccountKey](#schemasubaccountkey) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#list-all-api-key-pairs-of-the-sub-account) List all API key pairs of the sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#list-all-api-key-pairs-of-the-sub-account](https://www.gate.io/docs/developers/apiv4/en/#list-all-api-key-pairs-of-the-sub-account)

> Code samples

`GET /sub_accounts/{user_id}/keys`

_List all API key pairs of the sub-account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountkeys-parameters](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountkeys-parameters)

| Name    | In   | Type    | Required | Description         |
| ------- | ---- | ------- | -------- | ------------------- |
| user_id | path | integer | true     | Sub-account user ID |

> Example responses

> 200 Response

```
[
  {
    "state": 1,
    "name": "spot",
    "user_id": 1000000,
    "perms": [
      {
        "name": "futures",
        "read_only": false
      },
      {
        "name": "wallet",
        "read_only": false
      },
      {
        "name": "delivery",
        "read_only": false
      },
      {
        "name": "options",
        "read_only": false
      },
      {
        "name": "spot",
        "read_only": false
      }
    ],
    "mode": 1,
    "ip_whitelist": [
      "127.0.0.1",
      "127.0.0.2"
    ],
    "key": "75c3264105b74693d8cb5c7f1a8e2420",
    "created_at": 1663642892,
    "last_access": 1663642892,
    "update_at": 1663642892
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listsubaccountkeys-responses](https://www.gate.io/docs/developers/apiv4/en/#listsubaccountkeys-responses)

| Status | Meaning                                                                    | Description                 | Schema                                    |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ----------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[[SubAccountKey](#schemasubaccountkey)\] |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#update-sub-account-api-key-pair) Update sub-account API key pair

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#update-sub-account-api-key-pair](https://www.gate.io/docs/developers/apiv4/en/#update-sub-account-api-key-pair)

> Code samples

`PUT /sub_accounts/{user_id}/keys/{key}`

_Update sub-account API key pair_

> Body parameter

```
{
  "mode": 1,
  "name": "spot",
  "perms": [
    {
      "read_only": false,
      "name": "options"
    },
    {
      "read_only": false,
      "name": "spot"
    },
    {
      "read_only": false,
      "name": "delivery"
    },
    {
      "read_only": false,
      "name": "wallet"
    },
    {
      "read_only": false,
      "name": "futures"
    }
  ],
  "ip_whitelist": [
    "127.0.0.1",
    "127.0.0.2"
  ]
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatesubaccountkeys-parameters](https://www.gate.io/docs/developers/apiv4/en/#updatesubaccountkeys-parameters)

| Name           | In   | Type                                  | Required | Description                                               |
| -------------- | ---- | ------------------------------------- | -------- | --------------------------------------------------------- |
| user_id        | path | integer                               | true     | Sub-account user ID                                       |
| key            | path | string                                | true     | Sub-account API key                                       |
| body           | body | [SubAccountKey](#schemasubaccountkey) | true     | none                                                      |
| » mode         | body | integer(int32)                        | false    | Mode: 1 - classic 2 - portfolio account                   |
| » name         | body | string                                | false    | API Key Name                                              |
| » perms        | body | array                                 | false    | none                                                      |
| »» name        | body | string                                | false    | Permission function name (no value will be cleared)       |
| »» read_only   | body | boolean                               | false    | Read Only                                                 |
| » ip_whitelist | body | array                                 | false    | IP whitelist (list will be cleared if no value is passed) |

#### [#](#detailed-descriptions-6) Detailed descriptions

**»» name**: Permission function name (no value will be cleared)

- wallet: wallet
- spot: spot/margin
- futures: perpetual contract
- delivery: delivery contract
- earn: earn
- custody: custody
- options: options
- account: account information
- loan: lending
- margin: margin
- unified: unified account
- copy: copy trading

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#updatesubaccountkeys-responses](https://www.gate.io/docs/developers/apiv4/en/#updatesubaccountkeys-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Updated successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#delete-sub-account-api-key-pair) Delete sub-account API key pair

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#delete-sub-account-api-key-pair](https://www.gate.io/docs/developers/apiv4/en/#delete-sub-account-api-key-pair)

> Code samples

`DELETE /sub_accounts/{user_id}/keys/{key}`

_Delete sub-account API key pair_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#deletesubaccountkeys-parameters](https://www.gate.io/docs/developers/apiv4/en/#deletesubaccountkeys-parameters)

| Name    | In   | Type    | Required | Description         |
| ------- | ---- | ------- | -------- | ------------------- |
| user_id | path | integer | true     | Sub-account user ID |
| key     | path | string  | true     | Sub-account API key |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#deletesubaccountkeys-responses](https://www.gate.io/docs/developers/apiv4/en/#deletesubaccountkeys-responses)

| Status | Meaning                                                                            | Description          | Schema |
| ------ | ---------------------------------------------------------------------------------- | -------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Deleted successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-specific-api-key-pair-of-the-sub-account) Get specific API key pair of the sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-specific-api-key-pair-of-the-sub-account](https://www.gate.io/docs/developers/apiv4/en/#get-specific-api-key-pair-of-the-sub-account)

> Code samples

`GET /sub_accounts/{user_id}/keys/{key}`

_Get specific API key pair of the sub-account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsubaccountkey-parameters](https://www.gate.io/docs/developers/apiv4/en/#getsubaccountkey-parameters)

| Name    | In   | Type    | Required | Description         |
| ------- | ---- | ------- | -------- | ------------------- |
| user_id | path | integer | true     | Sub-account user ID |
| key     | path | string  | true     | Sub-account API key |

> Example responses

> 200 Response

```
{
  "state": 1,
  "name": "spot",
  "user_id": 1000000,
  "perms": [
    {
      "name": "futures",
      "read_only": false
    },
    {
      "name": "wallet",
      "read_only": false
    },
    {
      "name": "delivery",
      "read_only": false
    },
    {
      "name": "options",
      "read_only": false
    },
    {
      "name": "spot",
      "read_only": false
    }
  ],
  "mode": 1,
  "ip_whitelist": [
    "127.0.0.1",
    "127.0.0.2"
  ],
  "key": "75c3264105b74693d8cb5c7f1a8e2420",
  "created_at": 1663642892,
  "last_access": 1663642892,
  "update_at": 1663642892
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getsubaccountkey-responses](https://www.gate.io/docs/developers/apiv4/en/#getsubaccountkey-responses)

| Status | Meaning                                                                    | Description            | Schema                                |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------------------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | [SubAccountKey](#schemasubaccountkey) |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#lock-sub-account) Lock sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#lock-sub-account](https://www.gate.io/docs/developers/apiv4/en/#lock-sub-account)

> Code samples

`POST /sub_accounts/{user_id}/lock`

_Lock sub-account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#locksubaccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#locksubaccount-parameters)

| Name    | In   | Type           | Required | Description         |
| ------- | ---- | -------------- | -------- | ------------------- |
| user_id | path | integer(int64) | true     | Sub-account user ID |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#locksubaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#locksubaccount-responses)

| Status | Meaning                                                                            | Description         | Schema |
| ------ | ---------------------------------------------------------------------------------- | ------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Locked successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#unlock-sub-account) Unlock sub-account

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#unlock-sub-account](https://www.gate.io/docs/developers/apiv4/en/#unlock-sub-account)

> Code samples

`POST /sub_accounts/{user_id}/unlock`

_Unlock sub-account_

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#unlocksubaccount-parameters](https://www.gate.io/docs/developers/apiv4/en/#unlocksubaccount-parameters)

| Name    | In   | Type           | Required | Description         |
| ------- | ---- | -------------- | -------- | ------------------- |
| user_id | path | integer(int64) | true     | Sub-account user ID |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#unlocksubaccount-responses](https://www.gate.io/docs/developers/apiv4/en/#unlocksubaccount-responses)

| Status | Meaning                                                                            | Description           | Schema |
| ------ | ---------------------------------------------------------------------------------- | --------------------- | ------ |
| 204    | [No Content (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.5) | Unlocked successfully | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-sub-account-mode) Get sub-account mode

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-sub-account-mode](https://www.gate.io/docs/developers/apiv4/en/#get-sub-account-mode)

> Code samples

`GET /sub_accounts/unified_mode`

_Get sub-account mode_

Unified account mode:

- `classic`: Classic account mode
- `multi_currency`: Multi-currency margin mode
- `portfolio`: Portfolio margin mode

> Example responses

> 200 Response

```
[
  {
    "user_id": 110285555,
    "is_unified": true,
    "mode": "multi_currency"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedmode-responses](https://www.gate.io/docs/developers/apiv4/en/#listunifiedmode-responses)

| Status | Meaning                                                                    | Description      | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Query successful | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#listunifiedmode-responseschema](https://www.gate.io/docs/developers/apiv4/en/#listunifiedmode-responseschema)

Status Code **200**

| Name         | Type           | Description                     |
| ------------ | -------------- | ------------------------------- |
| » user_id    | integer(int64) | User ID                         |
| » is_unified | boolean        | Whether it is a unified account |
| » mode       | string         | Unified account mode:           |

\- `classic`: Classic account mode  
\- `multi_currency`: Multi-currency margin mode  
\- `portfolio`: Portfolio margin mode |

WARNING

To perform this operation, you must be authenticated by API key and secret
