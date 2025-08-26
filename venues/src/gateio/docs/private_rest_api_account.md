# [#](#account) Account

Retrieve user account information

## [#](#retrieve-user-account-information) Retrieve user account information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#retrieve-user-account-information](https://www.gate.io/docs/developers/apiv4/en/#retrieve-user-account-information)

> Code samples

`GET /account/detail`

_Retrieve user account information_

> Example responses

> 200 Response

```
{
  "user_id": 1667201533,
  "ip_whitelist": [
    "127.0.0.1"
  ],
  "currency_pairs": [
    "USDT_BTC"
  ],
  "key": {
    "mode": 1
  },
  "tier": 2,
  "copy_trading_role": 1
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getaccountdetail-responses](https://www.gate.io/docs/developers/apiv4/en/#getaccountdetail-responses)

| Status | Meaning                                                                    | Description            | Schema |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getaccountdetail-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getaccountdetail-responseschema)

Status Code **200**

_AccountDetail_

| Name                | Type           | Description                                                                                     |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------- |
| » ip_whitelist      | array          | IP Whitelist                                                                                    |
| » currency_pairs    | array          | Trading pair whitelist                                                                          |
| » user_id           | integer(int64) | User ID                                                                                         |
| » tier              | integer(int64) | User VIP level                                                                                  |
| » key               | object         | API Key details                                                                                 |
| »» mode             | integer(int32) | Mode: 1 - Classic mode, 2 - Legacy unified mode                                                 |
| » copy_trading_role | integer(int32) | User role: 0 - Normal user, 1 - Copy trading leader, 2 - Follower, 3 - Both leader and follower |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#get-user-transaction-rate-limit-information) Get user transaction rate limit information

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#get-user-transaction-rate-limit-information](https://www.gate.io/docs/developers/apiv4/en/#get-user-transaction-rate-limit-information)

> Code samples

`GET /account/rate_limit`

_Get user transaction rate limit information_

> Example responses

> 200 Response

```
[
  {
    "type": "spot",
    "tier": "1",
    "ratio": "0",
    "main_ratio": "0",
    "updated_at": "1728230400"
  },
  {
    "type": "futures",
    "tier": "1",
    "ratio": "0",
    "main_ratio": "0",
    "updated_at": "1728230400"
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getaccountratelimit-responses](https://www.gate.io/docs/developers/apiv4/en/#getaccountratelimit-responses)

| Status | Meaning                                                                    | Description            | Schema     |
| ------ | -------------------------------------------------------------------------- | ---------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Successfully retrieved | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getaccountratelimit-responseschema](https://www.gate.io/docs/developers/apiv4/en/#getaccountratelimit-responseschema)

Status Code **200**

_AccountRateLimit_

| Name             | Type   | Description                                                                                                                          |
| ---------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| AccountRateLimit | array  | Account rate limit                                                                                                                   |
| » tier           | string | Frequency limit level (For detailed frequency limit rules, see [Transaction ratio frequency limit](#rate-limit-based-on-fill-ratio)) |
| » ratio          | string | Fill rate                                                                                                                            |
| » main_ratio     | string | Total fill ratio of main account                                                                                                     |
| » updated_at     | string | Update time                                                                                                                          |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#create-stp-user-group) Create STP user group

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#create-stp-user-group](https://www.gate.io/docs/developers/apiv4/en/#create-stp-user-group)

> Code samples

`POST /account/stp_groups`

_Create STP user group_

Only the main account is allowed to create a new STP user group

> Body parameter

```
{
  "name": "stp_name"
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-parameters](https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-parameters)

| Name          | In   | Type           | Required | Description    |
| ------------- | ---- | -------------- | -------- | -------------- |
| body          | body | object         | true     | none           |
| » id          | body | integer(int64) | false    | STP Group ID   |
| » name        | body | string         | true     | STP Group name |
| » creator_id  | body | integer(int64) | false    | Creator ID     |
| » create_time | body | integer(int64) | false    | Creation time  |

> Example responses

> 200 Response

```
{
  "id": 123435,
  "name": "group",
  "create_time": 1548000000,
  "creator_id": 10000
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-responses](https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-responses)

| Status | Meaning                                                                    | Description                                                       | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | User added successfully, returning current users in the STP group | Inline |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-responseschema](https://www.gate.io/docs/developers/apiv4/en/#createstpgroup-responseschema)

Status Code **200**

| Name          | Type           | Description    |
| ------------- | -------------- | -------------- |
| » id          | integer(int64) | STP Group ID   |
| » name        | string         | STP Group name |
| » creator_id  | integer(int64) | Creator ID     |
| » create_time | integer(int64) | Creation time  |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-stp-user-groups-created-by-the-user) Query STP user groups created by the user

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-stp-user-groups-created-by-the-user](https://www.gate.io/docs/developers/apiv4/en/#query-stp-user-groups-created-by-the-user)

> Code samples

`GET /account/stp_groups`

_Query STP user groups created by the user_

Only query STP user groups created by the current main account

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-parameters](https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-parameters)

| Name | In    | Type   | Required | Description          |
| ---- | ----- | ------ | -------- | -------------------- |
| name | query | string | false    | Fuzzy search by name |

> Example responses

> 200 Response

```
[
  {
    "id": 123435,
    "name": "group",
    "create_time": 1548000000,
    "creator_id": 10000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-responses](https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-responseschema](https://www.gate.io/docs/developers/apiv4/en/#liststpgroups-responseschema)

Status Code **200**

| Name          | Type           | Description    |
| ------------- | -------------- | -------------- |
| _None_        | array          | none           |
| » id          | integer(int64) | STP Group ID   |
| » name        | string         | STP Group name |
| » creator_id  | integer(int64) | Creator ID     |
| » create_time | integer(int64) | Creation time  |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-users-in-the-stp-user-group) Query users in the STP user group

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-users-in-the-stp-user-group](https://www.gate.io/docs/developers/apiv4/en/#query-users-in-the-stp-user-group)

> Code samples

`GET /account/stp_groups/{stp_id}/users`

_Query users in the STP user group_

Only the main account that created this STP group can query the account ID list
in the current STP group

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-parameters](https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-parameters)

| Name   | In   | Type           | Required | Description  |
| ------ | ---- | -------------- | -------- | ------------ |
| stp_id | path | integer(int64) | true     | STP Group ID |

> Example responses

> 200 Response

```
[
  {
    "user_id": 10000,
    "stp_id": 1,
    "create_time": 1548000000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-responses](https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-responses)

| Status | Meaning                                                                    | Description                 | Schema     |
| ------ | -------------------------------------------------------------------------- | --------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | List retrieved successfully | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#liststpgroupsusers-responseschema)

Status Code **200**

| Name          | Type           | Description   |
| ------------- | -------------- | ------------- |
| _None_        | array          | none          |
| » user_id     | integer(int64) | User ID       |
| » stp_id      | integer(int64) | STP Group ID  |
| » create_time | integer(int64) | Creation time |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#add-users-to-the-stp-user-group) Add users to the STP user group

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#add-users-to-the-stp-user-group](https://www.gate.io/docs/developers/apiv4/en/#add-users-to-the-stp-user-group)

> Code samples

`POST /account/stp_groups/{stp_id}/users`

_Add users to the STP user group_

- Only the main account that created this STP group can add users to the STP
  user group
- Only accounts under the current main account are allowed, cross-main account
  is not permitted

> Body parameter

```
[
  1,
  2,
  3
]
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-parameters](https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-parameters)

| Name   | In   | Type             | Required | Description  |
| ------ | ---- | ---------------- | -------- | ------------ |
| stp_id | path | integer(int64)   | true     | STP Group ID |
| body   | body | array\[integer\] | true     | User ID      |

> Example responses

> 200 Response

```
[
  {
    "user_id": 10000,
    "stp_id": 1,
    "create_time": 1548000000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-responses](https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-responses)

| Status | Meaning                                                                    | Description                                                       | Schema     |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------- | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | User added successfully, returning current users in the STP group | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#addstpgroupusers-responseschema)

Status Code **200**

| Name          | Type           | Description   |
| ------------- | -------------- | ------------- |
| _None_        | array          | none          |
| » user_id     | integer(int64) | User ID       |
| » stp_id      | integer(int64) | STP Group ID  |
| » create_time | integer(int64) | Creation time |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#delete-users-from-the-stp-user-group) Delete users from the STP user group

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#delete-users-from-the-stp-user-group](https://www.gate.io/docs/developers/apiv4/en/#delete-users-from-the-stp-user-group)

> Code samples

`DELETE /account/stp_groups/{stp_id}/users`

_Delete users from the STP user group_

- Only the main account that created this STP group is allowed to delete users
  from the STP user group
- Deletion is limited to accounts under the current main account; cross-account
  deletion is not permitted

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-parameters](https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-parameters)

| Name    | In    | Type           | Required | Description                                           |
| ------- | ----- | -------------- | -------- | ----------------------------------------------------- |
| stp_id  | path  | integer(int64) | true     | STP Group ID                                          |
| user_id | query | integer(int64) | true     | STP user IDs, multiple IDs can be separated by commas |

> Example responses

> 200 Response

```
[
  {
    "user_id": 10000,
    "stp_id": 1,
    "create_time": 1548000000
  }
]
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-responses](https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-responses)

| Status | Meaning                                                                    | Description                                                        | Schema     |
| ------ | -------------------------------------------------------------------------- | ------------------------------------------------------------------ | ---------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Users deleted successfully, returns current users in the STP group | \[Inline\] |

### Response Schema

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-responseschema](https://www.gate.io/docs/developers/apiv4/en/#deletestpgroupusers-responseschema)

Status Code **200**

| Name          | Type           | Description   |
| ------------- | -------------- | ------------- |
| _None_        | array          | none          |
| » user_id     | integer(int64) | User ID       |
| » stp_id      | integer(int64) | STP Group ID  |
| » create_time | integer(int64) | Creation time |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#configure-gt-fee-deduction) Configure GT fee deduction

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#configure-gt-fee-deduction](https://www.gate.io/docs/developers/apiv4/en/#configure-gt-fee-deduction)

> Code samples

`POST /account/debit_fee`

_Configure GT fee deduction_

Enable or disable GT fee deduction for the current account

> Body parameter

```
{
  "enabled": true
}
```

### Parameters

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setdebitfee-parameters](https://www.gate.io/docs/developers/apiv4/en/#setdebitfee-parameters)

| Name      | In   | Type                        | Required | Description                         |
| --------- | ---- | --------------------------- | -------- | ----------------------------------- |
| body      | body | [DebitFee](#schemadebitfee) | true     | none                                |
| » enabled | body | boolean                     | true     | Whether GT fee deduction is enabled |

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#setdebitfee-responses](https://www.gate.io/docs/developers/apiv4/en/#setdebitfee-responses)

| Status | Meaning                                                                    | Description | Schema |
| ------ | -------------------------------------------------------------------------- | ----------- | ------ |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | None   |

WARNING

To perform this operation, you must be authenticated by API key and secret

## [#](#query-gt-fee-deduction-configuration) Query GT fee deduction configuration

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#query-gt-fee-deduction-configuration](https://www.gate.io/docs/developers/apiv4/en/#query-gt-fee-deduction-configuration)

> Code samples

`GET /account/debit_fee`

_Query GT fee deduction configuration_

Query the GT fee deduction configuration for the current account

> Example responses

> 200 Response

```
{
  "enabled": true
}
```

### Responses

**Source:**
[https://www.gate.io/docs/developers/apiv4/en/#getdebitfee-responses](https://www.gate.io/docs/developers/apiv4/en/#getdebitfee-responses)

| Status | Meaning                                                                    | Description | Schema                      |
| ------ | -------------------------------------------------------------------------- | ----------- | --------------------------- |
| 200    | [OK (opens new window)](https://tools.ietf.org/html/rfc7231#section-6.3.1) | Success     | [DebitFee](#schemadebitfee) |

WARNING

To perform this operation, you must be authenticated by API key and secret
