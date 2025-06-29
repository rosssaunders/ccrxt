---
title: Bullish Trading API - Public REST API
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

<h1 id="bullish-trading-api">Bullish Trading API v1.0.0</h1>

> Scroll down for code samples, example requests and responses. Select a
> language for code samples from the tabs above or the mobile navigation menu.

# Overview

Welcome to the _Bullish Trading API_ documentation!

This documentation provides resource descriptions and endpoints usage
instructions for the API.

The API:

- follows [REST](https://en.wikipedia.org/wiki/Representational_state_transfer)
  conventions
- has the base URL `api.exchange.bullish.com` (unless one of the sandbox
  environments is being used)
- has endpoints under the below categories:
  - Authenticated endpoints for private client data e.g.
    [Get Orders](#get-/v2/orders) endpoint
  - Non-authenticated endpoints for public data e.g.
    [Get Markets](#get-/v1/markets) endpoint
- uses bearer-based authentication
- enforces a blanket rate limit across all requests
- may add new fields to existing response payloads. Please do not use strict
  deserialisation as it may cause compatibility issues.

# Additional Links

- Code examples -
  [Bullish Github Repository](https://github.com/bullish-exchange)
- Deprecated features & APIs -
  [Deprecated Features Documentation](.././deprecated/index.html)
- Help center - [Bullish Help Center](https://support.bullish.com)
- Comprehensive guide for new Institutional users -
  [Institutional User Guide](https://support.bullish.com/hc/en-us/articles/28811587741721-Comprehensive-New-User-Guide-for-Institutions#h_01HPQBYXD72K2M2CRSRXQ5YEWG)
- Various order/custody status codes -
  [Error & Rejection Codes](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes)
- Operational status of our exchange -
  [official status page](https://bullish.statuspage.io/)

# Connectivity Options

Bullish offers 3 options for connectivity:

1. Cloudflare
2. Cloudflare Bypass
3. Direct Connect via AWS or GCP

Cloudflare Bypass and Direct Connect allow for a more optimised connection with
improved latencies. In GCP, generally our most optimal connection is to operate
within asia-southeast1-a Availability Zone. Please note, this may change at any
given moment and without warning to another Availability Zone within
asia-southeast1 for operational reasons. For AWS or GCP connectivity details,
please contact your sales representative.

# FIX API

The FIX API is available to institutional clients and is only accessible through
a AWS or GCP private connection:

- AWS PrivateLink
  - It provides private connectivity between VPCs, AWS service without exposing
    your traffic to the public internet. Bullish will provide the published
    service IDs for this service. You will be able to connect to our FIX service
    endpoint once the connection has been successfully established.
- GCP Private Service Connect
  - It allows customers to access managed services privately from inside their
    VPC network. Bullish will provide the published service IDs for this
    service. You will be able to connect to our FIX service endpoint once the
    connection has been successfully established.

For more details please refer to the full specification:

- [Bullish FIX Protocol Specification](../../fix-api/Bullish%20FIX%20Protocol%20Specification%20v1.0.12.pdf)
- [Bullish FIX Protocol Dictionary](../../fix-api/fix-dictionary.xml)
- [Bullish FIX CRT Certificate](../../fix-api/bullish.crt)

# Exchange Time

All timestamps are specified in
[EPOCH time](https://en.wikipedia.org/wiki/Unix_time).

# Pagination Support

If specified in the API documentation, an endpoint may return cursor paginated
responses. Default page size is 25.

There are 4 special query parameters used to control the pagination behaviour.

- `_pageSize` can be one of `5, 25, 50, 100`, default value is `25`
- `_metaData` can be false/true, if false the links are NOT part of the
  response. explicitly set `_metaData=true` to guarantee links are returned.
- `_nextPage` cursor to the next page. It is provided in the paginated response
  when `_metaData=true`.
- `_previousPage` cursor to the previous page. It is provided in the paginated
  response when `_metaData=true`.

The paginated data is returned in the following wrapped format with generated
links to the previous and next pages:

```json
{
  "data": [
    {
      "averageFillPrice": null,
      "baseFee": "0.00000000",
      "createdAtDatetime": "2018-11-18T00:00:00.000Z",
      "createdAtTimestamp": "1639464207402",
      "handle": null,
      "margin": false,
      "orderId": "390755652232282113",
      "price": "8520.7000",
      "quantity": "1.00000000",
      "quantityFilled": "0.00000000",
      "quoteAmount": "0.0000",
      "quoteFee": "0.0003",
      "side": "BUY",
      "status": "OPEN",
      "statusReason": "Open",
      "statusReasonCode": "6001",
      "stopPrice": null,
      "symbol": "BTCUSD",
      "timeInForce": "GTC",
      "type": "LMT"
    },
    ...
  ],
  "links": {
    "next": "/trading-api/v1/orders?_pageSize=5&symbol=BTCUSD&_nextPage=Mjk3NzM1MzQ5NDI0NjIwMDMy",
    "previous": "/trading-api/v1/orders?_pageSize=5&symbol=BTCUSD&_previousPage=Mjk3NzM1Mzg3NzQ3OTc1Njgw"
  }
}
```

# Filtering Support

- If specified in the API documentation, an endpoint may support filters on
  specific fields and values e.g. `GET /orders?status=OPEN`
- Only fields returned in an API response may be used as filter parameters. What
  you see is what you query on and by.
- For the fields in the API response usable as filter parameters, please consult
  the API endpoint of interest.
- Datetime and timestamp filters require additional keywords from this list -
  `[ gte, lte, gt, lt ]`. A few examples are provided below.
  - `/trading-api/v1/trades?createdAtTimestamp[gte]=1686447835000` queries for
    trades with `createdAtTimestamp` greater than or equal to `1686447835000`.
  - `/trading-api/v1/trades?createdAtDatetime[gte]=2023-04-06T00:00:00.000Z&createdAtDatetime[lte]=2023-06-07T00:00:00.000Z`
    queries for trades with `createdAtDatetime` greater than or equal to
    `2023-04-06T00:00:00.000Z` but less than or equal to
    `2023-06-07T00:00:00.000Z`
- By design, pagination query parameters start with an underscore. This
  differentiates them from filter query parameters e.g.
  `GET /orders?status=OPEN&_pageSize=25`

# Rate Limits

## Rate Limits per IP address

Each IP address is subject to a blanket rate limit of 500 requests per 10
seconds (approximately 50 requests per second). If an IP address is rate
limited, the http response status code will be `429 Too Many Requests` and the
IP address is blocked from making any requests for 60 seconds.

## Global Rate Limit

The global rate limit is an additional rate limit specific to the exchange. It
is used to help limit the flow of orders into the exchange. It affects all
clients fairly. When the global rate limit is breached the
`x-ratelimit-global-breach` header value will be set to `true` else `false`.

## Rate Limits per API category

In addition, API endpoints denoted by `Ratelimited: True` in the description are
also subject to rate limits. e.g. [Create Order](#post-/v2/orders). The API
endpoints fall under the below categories. The rate limit for each category is
_independently_ applied.

- Unauthenticated endpoints, rate limited at 50 requests per second.
- Authenticated `/orders` endpoints, rate limited at 50 requests per second.
- Other Authenticated endpoints, rate limited at 50 requests per second.

## Rate Limits Info

When rate limits are not exceeded, the http response header of the API endpoint
called will contain the below:

- `x-ratelimit-limit`: Maximum number of requests allowed for the specific API
  category within time period.
- `x-ratelimit-remaining`: Remaining number of requests allowed for the specific
  API category within time period.
- `x-ratelimit-reset`: The next time period in which `x-ratelimit-remaining` is
  reset back to the maximum allowed for the specific API category.
- `x-ratelimit-global-breach`: true/false, indicating whether the global limit
  has been breached.

## Exceeding Rate Limits

When rate limits are exceeded, the API endpoint will return the http response
status code `429 Too Many Requests` and the http response body will be:

```json
{
  "errorCode": 96000,
  "errorCodeName": "RATE_LIMIT_EXCEEDED",
  "message": "Rate limit exceeded"
}
```

## Exceeding The Global Rate Limit

When the global rate limit is exceeded, the API endpoint will return the http
response status code `429 Too Many Requests` and the http response body will be:

```json
{
  "errorCode": 96001,
  "errorCodeName": "GLOBAL_RATE_LIMIT_EXCEEDED",
  "message": "Global rate limit exceeded"
}
```

## Increasing Rate Limits

Bullish has a tiered rate limit offering based on volume.

|                   | Default      | 30-Day Volume between $1B USD and $2B USD | 30-Day Volume over $2B USD |
| ----------------- | ------------ | ----------------------------------------- | -------------------------- |
| Cloudflare        | 50 msgs/sec  | same as default                           | same as default            |
| Cloudflare Bypass | 100 msgs/sec | up to 200 msgs/sec                        | up to 500 msgs/sec         |

For more information on increasing your rate limits, please reach out to your
sales representative.

## Obtaining Your Rate Limit Token

Each trading account has a unique rate limit token that can be obtained by
querying [Get Trading Accounts](#get-/v1/accounts/trading-accounts). The rate
limit token must be provided in the HTTP request header `BX-RATELIMIT-TOKEN` to
enjoy higher tiers of rate limits else the default of 50 msgs/sec is applied.

# Price And Quantity Precision

A market consists of two assets, namely base asset and quote asset. For
instance, the `BTCUSD` market has `BTC` as base asset and `USD` as quote asset.
The price of a market is denominated in the quote asset, while the quantity is
denominated in the base asset. For instance, the price in `BTCUSD` market is in
`USD` while the quantity is in `BTC`. Each asset has its own precision. For
this, the below asset APIs can be queried.

- The [Get Assets](#get-/v1/assets) API endpoint for all supported asset symbols
- The [Get Asset By Symbol](#get-/v1/assets/-symbol-) API endpoint for a
  specific asset symbol

The precision of an asset is the number of decimal places used to express its
value in. For instance, the quantity in the market `BTCUSD` is in BTC. As BTC
has a precision of 8, the quantity in `BTCUSD` has 8 decimal places. It follows
that in order to express a quantity of 1 BTC in `BTCUSD` market, it has to be
`1.00000000` BTC.

Alternatively, the below market APIs can also be queried to obtain the price and
quantity precisions for different markets.

- The [Get Markets](#get-/v1/markets) API endpoint for all supported market
  symbols
- The [Get Market By Symbol](#get-/v1/markets/-symbol-) API endpoint for a
  specific market symbol

Following the logic outlined above,

- The precision for quantity is defined by the `basePrecision`
- The precision for price is defined by `quotePrecision`

# Numeric Identifier Contraints

Numeric (i64) identifiers such as `nonce` and `handle` should not have leading
zero's. For example `009990822212000000` is invalid but `9990822212000000` is
valid.

# Order timeInForce

The order `timeInForce` can be set to the following values:

- `GTC` - good until cancelled - the order is open forever unless fully filled
  or cancelled
- `FOK` - fill or kill - if the order cannot immediately be filled in full it is
  cancelled
- `IOC` - immediate or cancel - the order is immediately filled in full or in
  part and any remainder is cancelled

# Error & Rejection Codes

Details of `statusReasonCode` and `statusReason` can be referenced from
[Error & Rejection Codes](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes)

# Authentication

The API uses bearer based authentication. A JWT token is valid for 24 hours
only. To generate a JWT token see -
[Generate A JWT Token](#overview--generate-a-jwt-token)

# How To Send Authenticated Requests

To send an authenticated request, you must follow these steps:

1. [Generate An API Key](#overview--generate-an-api-key)
2. [Add Authenticated Request Header](#overview--add-authenticated-request-header)
3. [Generate A JWT Token](#overview--generate-a-jwt-token)
4. [Construct The BX-NONCE Header](#overview--construct-the-bx-nonce-header)
5. [Construct The Command You Want To Send](#overview--construct-the-command-you-want-to-send)
6. [Construct The BX-SIGNATURE Header](#overview--construct-the-bx-signature-header)
7. [Fetch Trading Account Ids](#overview--fetch-trading-account-ids)
8. [Send The HTTP Authenticated Request](#overview--send-the-http-authenticated-request)
9. [How To Ensure The Order Of _Create Order_ or _Cancel Order_ Requests](#overview--how-to-ensure-the-order-of-create-order-or-cancel-order-requests)
10. [How do EMS/Brokers Flag Their Executions Sent To Bullish](#overview--how-do-emsbrokers-flag-their-executions-sent-to-bullish)

## Generate An API Key

A prerequisite to generate API keys is to have a Bullish account. To generate an
API key follow these steps:

1. Log in into your Bullish account
2. Click on your account initials at the upper right corner and then click
   **Settings**
3. Click **API Keys** and then click **Add API Key**
4. Select the API Key type, either ECDSA or HMAC
5. Enter a key name in the _Key Name_ field
6. Adding an IP whitelist is optional. Should an IP whitelist be added, login
   requests must be from within the IP whitelist range.
7. Click _Generate API Key_

### HMAC API Key Notes

- A HMAC API Key is a shared secret key that is used for HMAC based signing of
  trading API requests
- Always store your HMAC secret in a secure medium as they are used to sign your
  requests. Do not share your HMAC secret in any publicly accessible areas such
  as code repositories, client side code, or other vulnerable areas and make
  sure the keys are not shipped with your mobile or web apps.
- You do not need the `metadata` string to extract your `userId` like you would
  a Bullish API Key
- HMAC API Keys can only be used for Trading on Bullish; the JWT generated using
  an HMAC API Key is only valid for trading endpoints.

### ECDSA API Key Notes

- An ECDSA API key is a public/private key pair used for ECDSA based signing of
  trading and custody API requests
- The private key is what you will use to sign your requests. Always store your
  private keys in a secure medium as they are used to sign your requests. Do not
  share your private keys in any publicly accessible areas such as code
  repositories, client side code, or other vulnerable areas and make sure the
  keys are not shipped with your mobile or web apps.
- From here on the:
  - public key will be referred to as `PUBLIC_KEY`
  - private key will be referred to as `PRIVATE_KEY`
- Key and signature format details:
  - Curve: ECDSA R1 (prime256v1/secp256r1/P-256)
  - Signature encoding: DER
  - Hashing Algorithm: SHA256
  - Key format: X.509 SubjectPublicKeyInfo format, PEM encoded

An ECDSA API key additionally has a `metadata` string associated with it which
is displayed along side the key. You must base64 decode the `metadata` to
extract your `userId` (example follows). You will need the `userId` in the next
step.

```shell
echo eyJwdWJsaWNLZXkiOiJQVUJfUjFfNWNpVW52TW5rVThMOVBCWnZaa1BGcjhqdkRnUHpzcHhWNGlqOThIN1JqM1FSNzJyMkEiLCJhY2NvdW50SWQiOjIyMjAwMDAwMDAwMDAwNCwiY3JlZGVudGlhbElkIjoiMTAifQ== | base64 --decode
{"publicKey":"PUB_R1_5ciUnvMnkU8L9PBZvZkPFr8jvDgPzspxV4ij98H7Rj3QR72r2A","userId":"12345","accountId":"12345","credentialId":"10"}
```

## Add Authenticated Request Header

Each authenticated request must include a `Authorization` header:

- `Authorization: Bearer <JWT_TOKEN>`

The JWT is valid for 24 hours.

## Generate a JWT Token

### Bullish API Key

#### **`Deprecated`** [[more info](.././deprecated/index.html#overview--bullish-key)]: _Existing Bullish API key will no longer be usable as of `28 June, 2024`_

To generate/get a JWT token for a Bullish API Key you will need to perform the
following request:

#### POST /trading-api/v2/users/login

- Body
  - `publicKey` - bullish account public key
  - `userId` - bullish user id corresponding to the metadata
  - `signature` - signed JSON string encoding of `loginPayload`, see the code
    sample for how to get it
  - `expirationTime` - epoch timestamp in seconds that is 5 minutes in the
    future
  - `nonce` - epoch timestamp in seconds; note this login API nonce has no
    connection to the orders API nonce
  - `biometricsUsed` - set to `false`
  - `sessionKey` - set to `null`

```json
{
  "publicKey": "<PUBLIC_KEY>",
  "signature": "<SIGNATURE>",
  "loginPayload": {
    "userId": "100008771"
    "nonce": 1638776636,
    "expirationTime": 1638776936,
    "biometricsUsed": false,
    "sessionKey": null
  }
}
```

- Response

```json
{
  "authorizer": "<AUTHORIZER>",
  "token": "<JWT_TOKEN>"
}
```

See
[generate JWT](https://github.com/bullish-exchange/api-examples/blob/master/session)
for sample Python scripts.

### HMAC API Key

To generate/get a JWT token for a HMAC API Key you will need to perform the
following request:

#### GET /trading-api/v1/users/hmac/login

- Response

```json
{
  "authorizer": "<AUTHORIZER>",
  "token": "<JWT_TOKEN>"
}
```

You will need to provide a series of headers along with the request in order to
successfully generate a JWT token.

- `BX-TIMESTAMP` - number of milliseconds since EPOCH
- `BX-NONCE` - client side incremented 64-bit unsigned integer
- `BX-PUBLIC-KEY` - Public key for the HMAC Key being used to generate the JWT
- `BX-SIGNATURE` - The signed message related to the login request. Outlined
  below

To sign the login request for an HMAC API Key login we will need to construct a
string that concatenates the following fields:

- timestamp - value provided for the `BX-TIMESTAMP` header
- nonce - value provided for the `BX-NONCE`
- request method - `GET`
- request path - `/trading-api/v1/users/hmac/login`

See
[generate JWT](https://github.com/bullish-exchange/api-examples/blob/master/session/generate_jwt_hmac.py)
for a sample Python script.

### ECDSA API Key

To generate/get a JWT token for a ECDSA API Key you will need to perform the
following request:

#### POST /trading-api/v2/users/login

- Body
  - `publicKey` - ECDSA public key; new line characters should be UNIX encoded
    (\n, not \r\n or \r)
  - `userId` - bullish user id corresponding to the metadata
  - `signature` - signed JSON string encoding of `loginPayload`, see the code
    sample for how to get it
  - `expirationTime` - epoch timestamp in seconds that is 5 minutes in the
    future
  - `nonce` - epoch timestamp in seconds; note this login API nonce has no
    connection to the orders API nonce
  - `biometricsUsed` - set to `false`
  - `sessionKey` - set to `null`

```json
{
  "publicKey": "<PUBLIC_KEY>",
  "signature": "<SIGNATURE>",
  "loginPayload": {
    "userId": "100008771"
    "nonce": 1638776636,
    "expirationTime": 1638776936,
    "biometricsUsed": false,
    "sessionKey": null
  }
}
```

- Response

```json
{
  "authorizer": "<AUTHORIZER>",
  "token": "<JWT_TOKEN>"
}
```

See
[generate JWT](https://github.com/bullish-exchange/api-examples/blob/master/session/generate_jwt_ecdsa.py)
for a sample Python script.

## Logout using a JWT Token

Users can better manage their sessions by logging out of unused sessions. This
can be done by calling the `GET /trading-api/v1/users/logout` endpoint with the
JWT Token in the header - see
[Add Authenticated Request Header](#overview--add-authenticated-request-header).

## Construct The BX-NONCE Header

The header `BX-NONCE` value is a unique client-side 64-bit unsigned integer. It
has the following characteristics:

- Each request the client sends how have incrementing `BX-NONCE` value
- To prevent a client to send the max value of a 64-bit unsigned integer and
  thus immediately exhaust all unique nonces the exchange will only accept a
  nonce within a specified range
- The lower and upper bounds of the current nonce range are specified by nonce
  endpoint e.g. `GET /nonce`
- The nonce range is updated daily
- The nonce `lowerBound` is the start of day EPOCH timestamp in micro seconds
- The nonce `upperBound` is the end of day EPOCH timestamp in micro seconds

## Construct The Command You Want To Send

Each authenticated request contains a `<COMMAND>` to be executed by the API. A
`<COMMAND>` has the following properties:

- A `<COMMAND>` is JSON encoded
- Every field in the JSON payload must have a value. Use `null` to represent the
  absence of a value
- The fields must be specified and encoded in the order presented in this
  documentation

Find below two `<COMMAND>` examples:

1. [Create Spot Order](#overview--create-spot-order-example)
2. [Create Margin Order](#overview--create-margin-order-example)
3. [Cancel Order](#overview--cancel-order-example)

### Create Spot Order Example

To create a Spot buy limit order:

- for the BTCUSD market
- at a price of `55071.5000`
- for a quantity of `1.87000000`
- with a time-in-force of `GTC` (good till cancelled)

The COMMAND would be constructed like below:

```json
{
  "timestamp": "<TIMESTAMP>",
  "nonce": "<NONCE>",
  "authorizer": "<AUTHORIZER>",
  "command": {
    "commandType": "V2CreateOrder",
    "handle": null,
    "symbol": "BTCUSD",
    "type": "LMT",
    "side": "BUY",
    "price": "55071.5000",
    "stopPrice": null,
    "quantity": "1.87000000",
    "timeInForce": "GTC",
    "allowMargin": false,
    "tradingAccountId": "111234567890"
  }
}
```

### Create Margin Order Example

With reference to the same payload in
[Create Spot Order](#overview--create-spot-order-example), setting the field
`allowMargin` to `true` will make it a margin order.

Documentation is available
[here](https://github.com/bullish-exchange/api-docs/blob/master/src/trading-api/next/websocket-private-data-update-messages-for-borrowers.pdf)
on what happens in the event of liquidation.

### Cancel Order Example

To cancel a buy limit order:

- for the BTCUSD market
- where the `orderId` is `390755251743358977`

The COMMAND would be constructed like below:

```json
{
  "timestamp": "<TIMESTAMP>",
  "nonce": "<NONCE>",
  "authorizer": "<AUTHORIZER>"
  "command": {
    "commandType": "V2CancelOrder",
    "orderId": "390755251743358977",
    "handle": null,
    "symbol": "BTCUSD",
    "tradingAccountId": "111234567890"
  }
}
```

### Add Amm instruction

```json
{
  "timestamp": "1621490985000",
  "nonce": "123456789",
  "authorizer": "03E02367E8C900000500000000000000",
  "command": {
    "commandType": "V2AddLiquidity",
    "symbol": "BTCUSD",
    "baseQuantity": "0.00000000",
    "quoteQuantity": "0.00000000",
    "upperBound": "14000.0000",
    "lowerBound": "12000.0000",
    "feeTierId": "1",
    "tradingAccountId": "111234567890"
  }
}
```

### Remove Amm instruction

```json
{
  "timestamp": "<TIMESTAMP>",
  "nonce": "<NONCE>",
  "authorizer": "<AUTHORIZER>"
  "command": {
    "commandType": "V2RemoveLiquidity",
    "liquidityId": "557839859386417160",
    "symbol": "BTCUSD",
    "tradingAccountId": "111234567890"
  }
}
```

### Command for Transfer Asset

`POST /trading-api/v1/command?commandType=V1TransferAsset`

```json
{
  "timestamp": "<TIMESTAMP>",
  "nonce": "<NONCE>",
  "authorizer": "<AUTHORIZER>"
  "command": {
    "commandType": "V1TransferAsset",
    "assetSymbol": "USD",
    "quantity": "7.0000",
    "fromTradingAccountId": "11123456789",
    "toTradingAccountId": "11198765432"
  }
}
```

## Construct The BX-SIGNATURE Header

The following signing formats demonstrates how to obtain the `BX-SIGNATURE`
header.

## Signing Format

This signing format works with `/trading-api/v2/orders`,
`/trading-api/v2/amm-instructions`, `/trading-api/v2/command` and
[Custody](#tag--custody) APIs. This signing format has the following benefits:

- Null fields need not be included in the HTTP request body
- Fields need not be strictly ordered in the HTTP request body

Construct a string that concatenates the following fields:

- timestamp - value provided for the `BX-TIMESTAMP` header
- nonce - value provided for the `BX-NONCE`
- request method - e.g. `POST`
- request path - e.g. `/trading-api/v2/orders`
- request JSON string, removing any spaces and newline characters.

Note that the same request JSON string used in signing must be sent as the HTTP
request body.

#### How To Sign - ECDSA API Key

1. Hash the above string using a SHA-256 hash function and sign the resulting
   hexdigest with your ECDSA `<PRIVATE_KEY>`.
2. DER encode the signature, and base64 encode the DER encoded signature.

See
[sign a request with ECDSA](https://github.com/bullish-exchange/api-examples/blob/master/orders/create_order_ecdsa.py)
for a sample Python script.

#### How To Sign - HMAC API Key

1. Hash the above string using a SHA-256 hash function and take the hexdigest.
2. Sign the hexdigest from step 2 with your HMAC Secret Key.

See
[sign a request with HMAC](https://github.com/bullish-exchange/api-examples/blob/master/orders/create_order_hmac.py)
for a sample Python script.

## Fetch Trading Account Ids

Trading Account Ids may be required by some endpoints. They can be fetched from
[Get All Trading Accounts](#get-/v1/accounts/trading-accounts).

See an API example
[Here](https://github.com/bullish-exchange/api-examples/blob/master/session/get_trading_accounts.py).

## Send The HTTP Authenticated Request

See
[create an order](https://github.com/bullish-exchange/api-examples/blob/master/orders/create_order_hmac.py)
for a sample Python script.

## How To Ensure The Order Of _Create Order_ or _Cancel Order_ Requests

To ensure the order of the _create order_ or _cancel order_ requests, you must
wait for an acknowledgement response which will contain the `orderId` generated
on the server side. Also remember that the `nonce` parameter, for these two
requests, must be a unique increasing integer value.

For example, let us assume the following:

- You sent 10 _create order_ requests in a row without waiting for an `orderId`
- The `nonce` increases with each request sent and thus
- The `nonce` is unique for each request

Because the requests received by the Bullish exchange are processed in parallel
the following two possible scenarios can happen:

- _happy scenario_ which has a small chance to occur: all 10 requests are
  processed in the exact order sent by the client, no error, all great and you
  are happy
- _unhappy scenario_ which has a higher chance to occur: the requests are not
  processed in the exact order sent by you, because the requests might arrive at
  the bullish processing server at different times and thus the validations of
  the `nonce` for each request take place at random times. Because of that all
  the requests that are validated and have the `nonce` higher than the latest
  valid `nonce` will be accepted as valid and the ones which have the `nonce`
  smaller than the last valid `nonce` will be considered invalid and dropped. In
  the worse case scenario the request with the highest `nonce`, the 10th request
  you sent, is validated first, and the rest of the 9 requests will fail the
  validation because they have the `nonce` smaller than the 10th. Also if some
  requests are failing because of some other errors, e.g. incorrect inputs or
  internal error, you will not know because you did not wait for the
  acknowledgement from the server side for each request you sent.

If you wait the acknowledgement from the server side you ensure the order of the
requests you sent and you can also verify the status of the order(s) you created
or cancelled.

## How To Enable Out-Of-Order Processing of Order Requests

The header `BX-NONCE-WINDOW-ENABLED` is a string representation of a boolean
value which enables out-of-order processing of _Create Order_ or _Cancel Order_
requests up to a window size of 100 from the highest previously used `nonce`
value (inclusive).

The `nonce` parameter is required to be both unique and incremental, but setting
`BX-NONCE-WINDOW-ENABLED` to `true` loosens this requirement such that the
`nonce` is only required to be `unique`. For example, the client is able to send
`nonce` values from `1...100` in any order and all the values will be valid.

# WebSockets

Connection request to the web-socket uses
[JWT_COOKIE based authentication](https://github.com/bullish-exchange/api-examples/blob/master/websocket/private_data_web_socket.py).
To generate a JWT token see -
[Generate A JWT Token](#overview--generate-a-jwt-token). Each web-socket exposes
a set of topics that can be subscribed to.

## Servers

- `wss://api.exchange.bullish.com` - PRODUCTION
- `wss://registered.api.exchange.bullish.com` - PRODUCTION (Cloudflare Bypass)
- `wss://prod.access.bullish.com` - PRODUCTION (Direct Connect)
- `wss://api.simnext.bullish-test.com` - SANDBOX
- `wss://registered.api.simnext.bullish-test.com` - SANDBOX (Cloudflare Bypass)
- `wss://simnext.access.bullish.com` - SANDBOX (Direct Connect)

## Max Open WebSocket Connections

Each WebSocket category has a maximum number of open connections. Once it is
reached, new WebSocket requests will be rejected. The WebSocket connections fall
under the below categories.

- Unauthenticated WebSockets, maximum of 100 open connections per IP address.
- Authenticated WebSockets, maximum of 10 open connections per API key.

## Send A Message Over The WebSocket

Messages sent by the client to the server over a web-socket follows the
[JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification). The server
then returns a response following the same JSON-RPC 2.0 format. The `id` field
sent by the client will be included in the response, allowing the client to map
the server's responses to the messages sent by the client. The client ensures
the uniqueness of the `id` field.

Find below the message types accepted by the web-socket:

1. [Subscription Message](#overview--subscribe-to-a-topic)
2. [Keepalive Ping Message](#overview--keep-websocket-open)

## Subscribe To A Topic

Subscribe to receive a snapshot of your existing data and subsequently receive
updates.

Two types of subscription:

1. Subscribe by `<TOPIC>`
2. Subscribe by `<TOPIC>` and `<SYMBOL>` plus optional fields (if any)

Message fields:

- `<TOPIC>`: subscription topic
- `<SYMBOL>`: market symbol
- `<COMMAND_ID>`: unique unsigned long value

The subscription message would be constructed like below:

Subscribe by `<TOPIC>`

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "<TOPIC>"
  },
  "id": "<COMMAND_ID>"
}
```

Subscribe by `<TOPIC>` and `<SYMBOL>`

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "<TOPIC>",
    "symbol": "<SYMBOL>"
  },
  "id": "<COMMAND_ID>"
}
```

Sample subscription messages:

- Orders

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "orders"
  },
  "id": "1611082473000"
}
```

- L1 Order Book

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
      "topic": "l1Orderbook"
      "symbol": "BTCUSD"
  },
  "id": "1611082473000"
}
```

Find below the available `<TOPIC>`:

1. [Multi-OrderBook Data WebSocket](#overview--multi-orderbook-websocket-unauthenticated)
2. [Anonymous Trades WebSocket](#overview--anonymous-trades-websocket-unauthenticated)
3. [Anonymous Market Data WebSocket](#overview--anonymous-market-data-price-tick-unauthenticated)
4. [Index Data WebSocket](#overview--index-data-websocket-unauthenticated)
5. [Private Data WebSocket](#overview--private-data-websocket-authenticated)

## Keep WebSocket Open

Keep the web-socket connection open by sending keepalive ping messages
periodically. The web-socket closes automatically after 5 minutes.

The keepalive ping message would be constructed like below:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "keepalivePing",
  "params": {},
  "id": "<COMMAND_ID>"
}
```

## Receive A Message From The WebSocket

JSON-RPC responses are of the following format:

Success responses

```json
{
  "jsonrpc": "2.0",
  "id": "1650865877698",
  "result": {
    "responseCode": "200",
    "responseCodeName": "OK",
    "message": "Successfully subscribed"
  }
}
```

Error responses

```json
{
  "jsonrpc": "2.0",
  "id": "1650865877698",
  "error": {
    "code": "-32602",
    "errorCode": "29013",
    "errorCodeName": "INVALID_TOPIC_ERROR",
    "message": "'a-random-topic' is not a valid topic"
}
```

- `code`: [JSON-RPC 2.0 error code](https://www.jsonrpc.org/specification)
- `responseCode`/`errorCode`: unique ID for response/error code
- `responseCodeName`/`errorCodeName`: unique name for response/error code
- `message`: textual description of the `responseCode`/`errorCode`

Snapshot responses are of the following format:

```json
{
  "type": "snapshot",
  "dataType": "<DATA_TYPE>",
  "data": [ { <TOPIC_RESPONSE> } ]
}
```

Update responses are of the following format:

```json
{
  "type": "update",
  "dataType": "<DATA_TYPE>",
  "data": { <TOPIC_RESPONSE> }
}
```

Error responses are of the following format:

```json
{
  "type": "error",
  "dataType": "V1TAErrorResponse",
  "data": {
    "errorCode": <ERROR_CODE>,
    "errorCodeName": "<ERROR_CODE_NAME>"
  }
}
```

## Heartbeat

- this is a `beta/experimental` feature that is currently being tested
- the heart beat message is periodically sent approximately every 30 seconds on
  the `heartbeat` topic for the
  [Private Data WebSocket](#overview--private-data-websocket-authenticated) API
  and
  [Multi-OrderBook WebSocket)](#overview--multi-orderbook-websocket-unauthenticated)
  API
- the heat beat serves to validate end to end communication between the exchange
  and the client
- if 3 heart beats are missed, then it is advisable to check the
  [official status page](https://bullish.statuspage.io/) for any announcements
  on the degradation of the exchanges features
- if no announcements have been made, it is advisable to disconnect and
  reconnect the websocket API given the issue may be isolated to a specific
  gateway

## Multi-OrderBook WebSocket (unauthenticated)

**Route**

- `/trading-api/v1/market-data/orderbook`

This allows simultaneous subscriptions to multiple L1 and L2 orderbooks of
different markets:

It also provides a heartbeat topic which sends heartbeat every 30s as an
indicator of platform healthiness. Please refer to the
[heartbeat session](#overview--heartbeat) for the details.

### Multi-Orderbook Subscription

The orderbooks of different markets to be subscribed are controlled by the
parameters in the subscription message listed below: | Parameters | Type |
Description |
|:----------------------|:-------|:--------------------------------------------------------------------------------|
| topic | String | l1 orderbook: l1Orderbook<br />l2 orderbook:
l2Orderbook<br />heartbeat: heartbeat | | symbol | String | market symbol |

L1 Subscription Message Sample:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "l1Orderbook",
    "symbol": "BTCUSD"
  },
  "id": "1611082473000"
}
```

L2 Subscription Message Sample:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "l2Orderbook",
    "symbol": "BTCUSD"
  },
  "id": "1611082473000"
}
```

Heartbeat Subscription Message Sample:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "heartbeat"
  },
  "id": "1611082473000"
}
```

### Multi-Orderbook Response

- L1 Update Response | Name | Type | Description |
  |:---------------|:-------|:-----------------------------------------------------------------
  | | sequenceNumber | String | incrementing, unique, unsigned integer that
  identifies a state of the L1-orderbook | | symbol | String | market symbol | |
  timestamp | String | denotes the time the update was created | | bid | Array |
  nested array containing price and quantity of highest bid | | ask | Array |
  nested array containing price and quantity of lowest ask |

```json
{
  "type": "update",
  "dataType": "V1TALevel1",
  "data": {
    "symbol": "BTCUSD",
    "bid": ["5199.5000", "61.95995262"],
    "ask": ["5199.6000", "96.59626782"],
    "sequenceNumber": "8",
    "datetime": "2020-06-29T06:28:55.500Z",
    "timestamp": "1593412135500"
  }
}
```

- l2Snapshot response

| Name                 | Type   | Description                                                                                                                                                                                                           |
| -------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol               | String | market symbol                                                                                                                                                                                                         |
| bids                 | Array  | array of size 200 where even indices denote price, odd indices denote absolute quantities                                                                                                                             |
| asks                 | Array  | array of size 200 where even indices denote price, odd indices denote absolute quantities                                                                                                                             |
| sequenceNumberRange  | Array  | array of size 2 where first element denotes lower bound, second element denotes upper bound of sequence numbers <br /> lower and upper bound are equal for initial snapshot; this may differ for subsequent snapshots |
| datetime             | String | denotes the time the update was created by the engine, ISO 8601 with millisecond as string                                                                                                                            |
| timestamp            | String | denotes the time the update was created by the engine                                                                                                                                                                 |
| publishedAtTimestamp | String | denotes the time the update was broadcasted to connected websockets                                                                                                                                                   |

```json
{
  "type": "snapshot",
  "dataType": "V1TALevel2",
  "data": {
    "symbol": "BTCUSDC",
    "bids": [
      "5199.5000",
      "110.92467647",
      "5199.4000",
      "20.92470365",
      "5199.3000",
      "0.92473034",
      "5199.2000",
      "0.92475701",
      "5199.1000",
      "0.92478369",
      "5199.0000",
      "0.92481038",
      "5198.9000",
      "0.92483705",
      "5198.8000",
      "0.92486375",
      "5198.7000",
      "0.92489042",
      "5198.6000",
      "0.92491712"
    ],
    "asks": [
      "5199.6000",
      "96.37848193",
      "5199.7000",
      "0.92465082",
      "5199.8000",
      "11.04464563",
      "5199.9000",
      "0.92459696",
      "5200.0000",
      "0.92457029",
      "5200.1000",
      "0.92454362",
      "5200.2000",
      "0.92451695",
      "5200.3000",
      "0.92449028",
      "5200.4000",
      "0.92446361",
      "5200.5000",
      "0.92443695"
    ],
    "sequenceNumberRange": [1370055970, 1370055970],
    "datetime": "2025-02-14T07:15:33.797Z",
    "timestamp": "1739517333797",
    "publishedAtTimestamp": "1739517333798"
  }
}
```

See
[connect to multi-orderbook web-socket](https://github.com/bullish-exchange/api-examples/blob/master/websocket/multi_orderbook_web_socket.py)
for a sample Python script.

## Unified Anonymous Trades WebSocket (unauthenticated)

**Route**

- `/trading-api/v1/market-data/trades`

This allows simultaneous trade subscriptions to multiple markets. Additionally,
instead of sending trades one by one, trades are sent in batches.

Upon subscribing to a market, the client will first receive a snapshot of the
latest 100 trades, followed by batches of trade updates.

### Unified Anonymous Trade Subscription

Anonymous trades from different markets to be subscribed to are controlled by
the parameters in the subscription message listed below: | Parameters | Type |
Description |
|:----------------------|:-------|:--------------------------------------------------------------------------------|
| topic | String | anonymousTrades | | symbol | String | market symbol |

Trade Subscription Message Sample:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "anonymousTrades",
    "symbol": "BTCUSDC"
  },
  "id": "1611082473000"
}
```

### Trade Response Fields

Each trade in a snapshot or update contains the following fields: | Name | Type
| Description |
|:---------------------|:----------|:------------------------------------------------------------------------------------------|
| tradeId | String | unique trade ID | | symbol | String | market symbol | |
price | String | price, see
[asset value](#overview--price-and-quantity-precision) format | | quantity |
String | quantity, see [asset value](#overview--price-and-quantity-precision)
format | | side | String | order side | | isTaker | Boolean | denotes whether
this is a taker's trade | | createdAtTimestamp | String | denotes the time the
order was ACK'd by the exchange | | createdAtDatetime | String | denotes the
time the order was ACK'd by the exchange, ISO 8601 with millisecond as string |
| publishedAtTimestamp | String | denotes the time the update was broadcasted to
connected websockets |

### Unified Anonymous Trade Snapshot Response

The client will receive a trade snapshot with the latest 100 trades upon
subscription.

Sample:

```json
{
  "type": "snapshot",
  "dataType": "V1TAAnonymousTradeUpdate",
  "data": {
    "trades": [
      {
        "tradeId": "100069000000063765",
        "isTaker": true,
        "price": "23404.8636",
        "createdAtTimestamp": "1721879160353",
        "quantity": "0.00029411",
        "publishedAtTimestamp": "1721879162124",
        "side": "SELL",
        "createdAtDatetime": "2024-07-25T03:46:00.353Z",
        "symbol": "BTCUSDC"
      },
      {
        "tradeId": "100069000000063764",
        "isTaker": true,
        "price": "23405.3380",
        "createdAtTimestamp": "1721879155351",
        "quantity": "0.00029411",
        "publishedAtTimestamp": "1721879162124",
        "side": "SELL",
        "createdAtDatetime": "2024-07-25T03:45:55.351Z",
        "symbol": "BTCUSDC"
      },
      ...
      {
          "tradeId": "100069000000063666",
          "isTaker": true,
          "price": "23001.8708",
          "createdAtTimestamp": "1721879028067",
          "quantity": "0.00029411",
          "publishedAtTimestamp": "1721879162124",
          "side": "SELL",
          "createdAtDatetime": "2024-07-25T03:43:48.067Z",
          "symbol": "BTCUSDC"
      }
    ],
    "createdAtTimestamp": "1721879160353",
    "publishedAtTimestamp": "1721879162125",
    "symbol": "BTCUSDC"
  }
}
```

### Unified Anonymous Trade Update Response

After receiving the snapshot, the client will receive subsequent trade updates
in batches.

Sample:

```json
{
  "type": "update",
  "dataType": "V1TAAnonymousTradeUpdate",
  "data": {
    "trades": [
      {
        "tradeId": "100028000018887830",
        "isTaker": true,
        "price": "111.8940",
        "createdAtTimestamp": "1722408780738",
        "quantity": "0.00100000",
        "publishedAtTimestamp": "1722408780790",
        "side": "BUY",
        "createdAtDatetime": "2024-07-31T06:53:00.738Z",
        "symbol": "BTCUSDC"
      },
      {
        "tradeId": "100028000018887837",
        "isTaker": false,
        "price": "111.8716",
        "createdAtTimestamp": "1722408780738",
        "quantity": "0.00009595",
        "publishedAtTimestamp": "1722408780790",
        "side": "SELL",
        "createdAtDatetime": "2024-07-31T06:53:00.738Z",
        "symbol": "BTCUSDC"
      },
      ...
      {
        "tradeId": "100028000018887992",
        "isTaker": true,
        "price": "112.2896",
        "createdAtTimestamp": "1722408780786",
        "quantity": "0.00100000",
        "publishedAtTimestamp": "1722408780790",
        "side": "BUY",
        "createdAtDatetime": "2024-07-31T06:53:00.786Z",
        "symbol": "BTCUSDC"
      }
    ],
    "createdAtTimestamp": "1722408780786",
    "publishedAtTimestamp": "1722408780790",
    "symbol": "BTCUSDC"
  }
}
```

## Anonymous Market Data Price Tick (unauthenticated)

**Route**

- `/trading-api/v1/market-data/tick/{symbol}`

**Note**: This endpoint does not require subscriptions.

On connection, the client receives current Tick by Market Symbol. See the data
model: [Get Market Tick](#get-/v1/markets/-symbol-/tick)

### Tick response example

```json
{
  "type": "snapshot",
  "dataType": "V1TATickerResponse",
  "data": {
    "askVolume": "3.56000000",
    "average": "5200.0400",
    "baseVolume": "1.00000000",
    "bestAsk": "6543.0000",
    "bestBid": "2345.0000",
    "bidVolume": "2.00000000",
    "change": "0.0000",
    "close": "5200.0400",
    "createdAtTimestamp": "1591058897000",
    "publishedAtTimestamp": "1591058898000",
    "high": "5200.0400",
    "last": "5200.0400",
    "lastTradeDatetime": "2020-06-02T00:40:39.500Z",
    "lastTradeSize": "1.00000000",
    "low": "5200.0400",
    "open": "5200.0400",
    "percentage": "0.00",
    "quoteVolume": "5200.0400",
    "symbol": "BTC-USDC-PERP",
    "type": "ticker",
    "vwap": "5200.0400",
    "currentPrice": "0.0007",
    "ammData": [
      {
        "feeTierId": "1",
        "currentPrice": "0.0007",
        "baseReservesQuantity": "96153.00000000",
        "quoteReservesQuantity": "500005200.0400",
        "bidSpreadFee": "0.00000005",
        "askSpreadFee": "0.00000006"
      },
      {
        "feeTierId": "2",
        "currentPrice": "0.0017",
        "baseReservesQuantity": "96153.00000000",
        "quoteReservesQuantity": "500005200.0400",
        "bidSpreadFee": "0.00000015",
        "askSpreadFee": "0.00000016"
      }
    ],
    "createdAtDatetime": "2020-06-02T00:48:17.000Z",
    "markPrice": "26000.0000",
    "fundingRate": "0.114100",
    "openInterest": "9.00000000"
  }
}
```

## Index Data websocket (unauthenticated)

**Route**

- `/trading-api/v1/index-data`

### Index Price Subscription

The index price of different assets to be subscribed are controlled by the
parameters in the subscription message listed below: | Parameters | Type |
Description |
|:----------------------|:-------|:--------------------------------------------------------------------------------|
| topic | String | Index Price: `indexPrice` <br/> | | assetSymbol | String |
Asset symbol, such as `BTC` or `USDC` |

Index Price Subscription Sample:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "indexPrice",
    "assetSymbol": "USDC"
  },
  "id": "1611082473000"
}
```

### IndexPrice Response

On successful subscription for an `assetSymbol`, the client receives a snapshot
with the current index price, and updates after.

| Name               | Type   | Description                                                                      |
| :----------------- | :----- | :------------------------------------------------------------------------------- |
| assetSymbol        | String | asset symbol                                                                     |
| price              | String | price in USD, see [asset value](#overview--price-and-quantity-precision) format  |
| updatedAtDatetime  | String | denotes the time the index price was updated by the exchange, in ISO 8601 format |
| updatedAtTimestamp | String | denotes the epoch millisecond time the index price was updated by the exchange   |

```json
{
  "type": "update",
  "dataType": "V1TAIndexPrice",
  "data": {
    "assetSymbol": "USDC",
    "price": "1.0000",
    "updatedAtDatetime": "2024-06-29T06:29:50.500Z",
    "updatedAtTimestamp": "1719642590000"
  }
}
```

## Private Data WebSocket (authenticated)

- All private data updates are realtime.

Establishing a websocket connection

1. Getting private data from a single trading account.

- Connect to
  `/trading-api/v1/private-data?tradingAccountId=<Id of the Trading Account>`
- For example, to subscribe to the `orders` topic, the following subscription
  message is sent:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "orders"
  },
  "id": "1611082473000"
}
```

2. Getting private data from multiple trading accounts.

- Connect to `/trading-api/v1/private-data`
- For example, to subscribe to the `orders` topic for each of your trading
  accounts, the following subscription message is sent for each trading account
  you wish to subscribe to:

```json
{
  "jsonrpc": "2.0",
  "type": "command",
  "method": "subscribe",
  "params": {
    "topic": "orders",
    "tradingAccountId": "<Id of the Trading Account>"
  },
  "id": "1611082473000"
}
```

| Topic                  | Description                                                                                                                           | Data Type                 | Subscription Type |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------ | :------------------------ | :---------------- |
| orders                 | Provides snapshot and updates on your orders. The snapshot will contain all open orders and the 20 most recent closed orders.         | `V1TAOrder`               | By `<TOPIC>`      |
| trades                 | Provides snapshot and updates on your trades. The snapshot will contain the 20 most recent trades.                                    | `V1TATrade`               | By `<TOPIC>`      |
| ~spotAccounts~         | `Deprecated`[[more info](.././deprecated/index.html#overview--spot-account)] Provides snapshot and updates on assets in your account. | `V1TASpotAccount`         | By `<TOPIC>`      |
| assetAccounts          | Provides snapshot and updates on assets in your account.                                                                              | `V1TAAssetAccount`        | By `<TOPIC>`      |
| tradingAccounts        | Provides snapshot and updates on your trading account summary.                                                                        | `V1TATradingAccount`      | By `<TOPIC>`      |
| heartbeat              | Provides heartbeat updates for healthcheck.                                                                                           | `V1TAHeartbeat`           | By `<TOPIC>`      |
| ~derivativesPositions~ | `Deprecated`[*Replaced by:* `derivativesPositionsV2`] Provides derivative position information on your trading account.               | `V1TAPerpetualPosition`   | By `<TOPIC>`      |
| derivativesPositionsV2 | Provides derivative position information on your trading account.                                                                     | `V1TADerivativesPosition` | By `<TOPIC>`      |
| ammInstructions        | Provides amm instructions update on your trading account.                                                                             | `V1TAAmmInstruction`      | By `<TOPIC>`      |

### orders response

| Name                  | Type    | Description                                                                                                                                                                                                                                                                       |
| :-------------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ~handle~              | String  | unique numeric (i64) identifier generated on the client side expressed as a string value <br /><br />`Deprecated`_to be remove towards the end of Q3 2024._ <br /> _Replaced by:_ `clientOrderId`                                                                                 |
| clientOrderId         | String  | unique numeric (i64) identifier generated on the client side expressed as a string value                                                                                                                                                                                          |
| orderId               | String  | unique order ID                                                                                                                                                                                                                                                                   |
| symbol                | String  | market symbol                                                                                                                                                                                                                                                                     |
| price                 | String  | price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                          |
| averageFillPrice      | String  | average fill price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                             |
| stopPrice             | String  | stop price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                     |
| ~margin~              | Boolean | indicates if the order was allowed to borrow (does not indicate that borrowing occurred) <br /><br />`Deprecated`_to be remove towards the end of Q3 2024._ <br /> _Replaced by:_ `allowBorrow`                                                                                   |
| allowBorrow           | Boolean | indicates if the order was allowed to borrow (does not indicate that borrowing occurred)                                                                                                                                                                                          |
| quantity              | String  | quantity, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                       |
| quoteAmount           | String  | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                     |
| quantityFilled        | String  | quantity filled, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                |
| baseFee               | String  | base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                        |
| quoteFee              | String  | quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                       |
| ~borrowedQuantity~    | String  | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format - BUY order borrows quote, SELL order borrows base <br /><br />`Deprecated`_to be remove towards the end of Q3 2024._ <br /> _Replaced by:_ `borrowedBaseQuantity` & `borrowedQuoteQuantity` |
| borrowedBaseQuantity  | String  | base quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                         |
| borrowedQuoteQuantity | String  | quote quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                        |
| isLiquidation         | Boolean | indicates if the order was executed as a liquidation order                                                                                                                                                                                                                        |
| side                  | String  | order side                                                                                                                                                                                                                                                                        |
| type                  | String  | order type                                                                                                                                                                                                                                                                        |
| timeInForce           | String  | time in force                                                                                                                                                                                                                                                                     |
| status                | String  | order status                                                                                                                                                                                                                                                                      |
| statusReason          | String  | status reason, describes why the order is in a specific state                                                                                                                                                                                                                     |
| statusReasonCode      | Integer | status reason code, see [details](https://github.com/bullish-exchange/api-docs/wiki/Error-&-Rejection-Codes#order-statusreasoncode-map)                                                                                                                                           |
| createdAtDatetime     | String  | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                                                                                                                                                                         |
| createdAtTimestamp    | String  | denotes the time the order was ACK'd by the exchange                                                                                                                                                                                                                              |
| publishedAtTimestamp  | String  | denotes the time the update was broadcasted to connected websockets                                                                                                                                                                                                               |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TAOrder",
  "data": [
    {
      "handle": null,
      "orderId": "392883006043848705",
      "symbol": "BTCUSD",
      "price": "66858.2000",
      "averageFillPrice": "66858.2000",
      "stopPrice": null,
      "margin": false,
      "quantity": "2.00000000",
      "quantityFilled": "2.00000000",
      "quoteAmount": "23000.0000",
      "baseFee": "0.00000000",
      "quoteFee": "0.0005",
      "side": "BUY",
      "borrowedQuantity": "0.0010",
      "isLiquidation": false,
      "type": "LMT",
      "timeInForce": "GTC",
      "status": "CLOSED",
      "statusReason": "Executed",
      "statusReasonCode": 6002,
      "createdAtDatetime": "2021-12-30T07:36:35.918Z",
      "createdAtTimestamp": "1640849795918",
      "publishedAtTimestamp": "1640849795920"
    }
  ]
}
```

### trades response

| Name                   | Type    | Description                                                                                                  |
| :--------------------- | :------ | :----------------------------------------------------------------------------------------------------------- |
| tradeId                | String  | unique trade ID                                                                                              |
| orderId                | String  | unique order ID                                                                                              |
| handle                 | String  | unique numeric identifier (i64) generated on the client side expressed as a string value                     |
| symbol                 | String  | market symbol                                                                                                |
| price                  | String  | price, see [asset value](#overview--price-and-quantity-precision) format                                     |
| quantity               | String  | quantity, see [asset value](#overview--price-and-quantity-precision) format                                  |
| quoteAmount            | String  | quote quantity deducted from asset account, see [asset value](#overview-price-and-quantity-precision) format |
| baseFee                | String  | base fee, see [asset value](#overview--price-and-quantity-precision) format                                  |
| quoteFee               | String  | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                 |
| side                   | String  | order side                                                                                                   |
| tradeRebateAmount      | String  | amount of rebate that is credited to the user as part of the trade                                           |
| tradeRebateAssetSymbol | String  | symbol of the asset in which the rebate is paid                                                              |
| isTaker                | Boolean | denotes whether this is a taker's trade                                                                      |
| createdAtDatetime      | String  | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string                 |
| createdAtTimestamp     | String  | denotes the time the trade was executed by the exchange                                                      |
| publishedAtTimestamp   | String  | denotes the time the update was broadcasted to connected websockets                                          |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TATrade",
  "data": [
    {
      "tradeId": "100014000000000118",
      "orderId": "392883006043848705",
      "handle": "123456",
      "symbol": "BTCUSD",
      "price": "66858.2000",
      "quantity": "2.00000000",
      "quoteAmount": "23000.0000",
      "baseFee": "0.00000000",
      "quoteFee": "66.8582",
      "side": "BUY",
      "isTaker": false,
      "tradeRebateAmount": "3.0000",
      "tradeRebateAssetSymbol": "USDC",
      "createdAtDatetime": "2021-12-30T07:36:35.918Z",
      "createdAtTimestamp": "1640849795918",
      "publishedAtTimestamp": "1640849795920"
    }
  ]
}
```

### assetAccounts response

- `V1TAAssetAccount` provides a more granular view of the assets in your trading
  account compared to `V1TASpotAccount`.

| Name                 | Type   | Description                                                                                                                                        |
| -------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId     | String | id of the trading account                                                                                                                          |
| assetId              | String | asset ID                                                                                                                                           |
| assetSymbol          | String | asset symbol                                                                                                                                       |
| availableQuantity    | String | the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format                             |
| borrowedQuantity     | String | the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format                                     |
| lockedQuantity       | String | the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format |
| loanedQuantity       | String | the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format                                 |
| updatedAtDatetime    | String | denotes the time the asset account was updated by the exchange, ISO 8601 with millisecond as string                                                |
| updatedAtTimestamp   | String | denotes the time the asset account was updated by the exchange                                                                                     |
| publishedAtTimestamp | String | denotes the time the update was broadcasted to connected websockets                                                                                |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TAAssetAccount",
  "data": [
    {
      "tradingAccountId": "1111",
      "assetId": "1",
      "assetSymbol": "BTC",
      "availableQuantity": "4.00000000",
      "borrowedQuantity": "20.00000000",
      "lockedQuantity": "0.00000000",
      "loanedQuantity": "10.00000000",
      "updatedAtDatetime": "2021-12-30T07:36:35.918Z",
      "updatedAtTimestamp": "1640849795918",
      "publishedAtTimestamp": "1640849795920"
    },
    {
      "tradingAccountId": "1111",
      "assetId": "2",
      "assetSymbol": "USD",
      "availableQuantity": "229016.0734",
      "borrowedQuantity": "20000.0000",
      "lockedQuantity": "0.0000",
      "loanedQuantity": "10000.0000",
      "updatedAtDatetime": "2021-12-30T07:36:35.918Z",
      "updatedAtTimestamp": "1640849795918",
      "publishedAtTimestamp": "1640849795920"
    }
  ]
}
```

### tradingAccounts response

- Provides a summary of the total borrowed and total collateral values on the
  specific trading account ID. `totalBorrowedQuantity` and
  `totalCollateralQuantity` do not represent the absolute quantity of the
  borrowed assets and are notional values represented in the reference asset.
- `snapshot` contains a list with a single entry corresponding to the trading
  account ID specified in the `tradingAccountId` query parameter when opening
  the websocket connection.

| Name                                   | Type   | Description                                                                                                                                   |
| -------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId                       | String | id of the trading account                                                                                                                     |
| totalBorrowedQuantity                  | String | total borrowed across all assets in this trading account displayed in the reference asset                                                     |
| totalCollateralQuantity                | String | total collateral across all assets in this trading account displayed in the reference asset                                                   |
| totalBorrowedUSD                       | String | total borrowed across all assets in this trading account displayed in USD                                                                     |
| totalCollateralUSD                     | String | total collateral across all assets in this trading account displayed in USD                                                                   |
| initialMarginUSD                       | String | The minimum margin one must maintain in order to be able to purposefully increase risk                                                        |
| warningMarginUSD                       | String | The minimum margin when the customer will receive warning via email/notifications over UI                                                     |
| liquidationMarginUSD                   | String | The minimum value of margin one must maintain in order to avoid liquidation                                                                   |
| fullLiquidationMarginUSD               | String | The value of margin when full liquidation occurs                                                                                              |
| endCustomerId                          | String | The end customer id used for self trade prevention (default is institution id, max 32 characters)                                             |
| defaultedMarginUSD                     | String | The value of margin when this trading account will be moved into a Defaulted state                                                            |
| riskLimitUSD                           | String | The maximum allowed borrowing for this trading account displayed in USD                                                                       |
| totalLiabilitiesUSD                    | String | The total liabilities for this trading account displayed in USD                                                                               |
| maxInitialLeverage                     | String | The maximum initial leverage                                                                                                                  |
| isPrimaryAccount                       | String | Whether this trading account is the primary account                                                                                           |
| isBorrowing                            | String | Whether this trading account is borrowing any asset                                                                                           |
| isLending                              | String | Whether this trading account has any open loan offers                                                                                         |
| isDefaulted                            | String | Whether this trading account is in a defaulted state                                                                                          |
| ~takerFee~                             | String | Deprecated and no longer accurate. See `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-) instead |
| ~makerFee~                             | String | Deprecated and no longer accurate. See `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-) instead |
| referenceAssetSymbol                   | String | asset symbol                                                                                                                                  |
| liquidityAddonUSD                      | String | Expected market impact of unwinding the portfolio in the case of a liquidation event                                                          |
| marketRiskUSD                          | String | The worst possible loss on the portfolio based on scenario analysis                                                                           |
| marginProfile                          | Object | Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values                     |
| initialMarketRiskMultiplierPct         | String | Market risk multiplier used to calculate initial margin requirement of the account                                                            |
| warningMarketRiskMultiplierPct         | String | Market risk multiplier used to calculate warning margin requirement of the account                                                            |
| liquidationMarketRiskMultiplierPct     | String | Market risk multiplier used to calculate liquidation margin requirement of the account                                                        |
| fullLiquidationMarketRiskMultiplierPct | String | Market risk multiplier used to calculate full liquidation margin requirement of the account                                                   |
| defaultedMarketRiskMultiplierPct       | String | Market risk multiplier used to calculate defaulted margin requirement of the account                                                          |
| updatedAtDatetime                      | String | denotes the time the trading account was updated by the exchange, ISO 8601 with millisecond as string                                         |
| updatedAtTimestamp                     | String | denotes the time the trading account was updated by the exchange                                                                              |
| publishedAtTimestamp                   | String | denotes the time the update was broadcasted to connected websockets                                                                           |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TATradingAccount",
  "data": [
    {
      "tradingAccountId": "1111",
      "totalBorrowedQuantity": "12000.0000",
      "totalCollateralQuantity": "13000.0000",
      "totalBorrowedUSD": "12000.0000",
      "totalCollateralUSD": "13000.0000",
      "referenceAssetSymbol": "USD",
      "initialMarginUSD": "900000.0000",
      "warningMarginUSD": "700000.0000",
      "liquidationMarginUSD": "600000.0000",
      "fullLiquidationMarginUSD": "500000.0000",
      "endCustomerId": "PrimeBroker",
      "defaultedMarginUSD": "300000.0000",
      "riskLimitUSD": "1000000.0000",
      "totalLiabilitiesUSD": "13000.0000",
      "maxInitialLeverage": "3",
      "isPrimaryAccount": true,
      "isBorrowing": true,
      "isLending": false,
      "isDefaulted": false,
      "takerFee": null,
      "makerFee": null,
      "liquidityAddonUSD": "100.0000",
      "marketRiskUSD": "200.0000",
      "marginProfile": {
        "initialMarketRiskMultiplierPct": "200.00",
        "warningMarketRiskMultiplierPct": "150.00",
        "liquidationMarketRiskMultiplierPct": "100.00",
        "fullLiquidationMarketRiskMultiplierPct": "75.00",
        "defaultedMarketRiskMultiplierPct": "50.00"
      },
      "updatedAtDatetime": "2021-12-30T07:36:35.918Z",
      "updatedAtTimestamp": "1640849795918",
      "publishedAtTimestamp": "1640849795920"
    }
  ]
}
```

### heartbeat response

- Provides heartbeat update every 30s as an indicator of platform healthiness.
  Please refer to the [heartbeat session](#overview--heartbeat) for the details.

| Name               | Type   | Description                              |
| ------------------ | ------ | ---------------------------------------- |
| sequenceNumber     | String | sequence number of the heartbeat         |
| createdAtTimestamp | String | time at which the heartbeat is generated |

```json
{
  "type": "update",
  "dataType": "V1TAHeartbeat",
  "data": [
    {
      "sequenceNumber": "3",
      "createdAtTimestamp": "1611082473000"
    }
  ]
}
```

### Derivative position response

`Deprecated`: _Topic derivativesPosition will be replaced by
`derivativesPositionV2`_

- Provide a detail view of the derivative position of each market.

| Name                 | Type   | Description                                                                                                                                                                                               |
| -------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId     | String | Id of the trading account                                                                                                                                                                                 |
| symbol               | String | Market symbol, eg. BTC-USDC-PERP                                                                                                                                                                          |
| side                 | String | Side of the position                                                                                                                                                                                      |
| quantity             | String | Current size of the position [asset value](#overview--price-and-quantity-precision) format                                                                                                                |
| notional             | String | Notional value of the current position, calculated using the mark price                                                                                                                                   |
| entryNotional        | String | Notional value of the position, using the average entry price                                                                                                                                             |
| mtmPnl               | String | Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement                                                                         |
| reportedMtmPnl       | String | The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase |
| reportedFundingPnl   | String | Sum of all funding payments received since the position was opened. This is updated every time funding is paid.                                                                                           |
| realizedPnl          | String | Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.                                        |
| createdAtDatetime    | String | denotes the time the position was created by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| createdAtTimestamp   | String | denotes the time the position was created by the exchange, number of milliseconds since EPOCH                                                                                                             |
| updatedAtDatetime    | String | denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| updatedAtTimestamp   | String | denotes the time the position was updated by the exchange number of milliseconds since EPOCH                                                                                                              |
| publishedAtTimestamp | String | denotes the time the update was broadcasted to connected websockets                                                                                                                                       |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TAPerpetualPosition",
  "data": [
    {
      "tradingAccountId": "111234567890",
      "symbol": "BTC-USDC-PERP",
      "side": "BUY",
      "quantity": "1.00000000",
      "notional": "30000.0000",
      "entryNotional": "30000.0000",
      "mtmPnl": "110.0000",
      "reportedMtmPnl": "120.0000",
      "reportedFundingPnl": "130.0000",
      "realizedPnl": "140.0000",
      "createdAtDatetime": "2020-01-01T00:00:00.000Z",
      "createdAtTimestamp": "1577836800000",
      "updatedAtDatetime": "2020-01-02T00:00:00.000Z",
      "updatedAtTimestamp": "1577923200000",
      "publishedAtTimestamp": "1577923300000"
    }
  ]
}
```

### Derivative position V2 response

- Provide a detail view of the derivative position of each market.

| Name                  | Type   | Description                                                                                                                                                                                               |
| --------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId      | String | Id of the trading account                                                                                                                                                                                 |
| symbol                | String | Market symbol, eg. BTC-USDC-PERP                                                                                                                                                                          |
| side                  | String | Side of the position                                                                                                                                                                                      |
| quantity              | String | Current size of the position [asset value](#overview--price-and-quantity-precision) format                                                                                                                |
| notional              | String | Notional value of the current position, calculated using the mark price                                                                                                                                   |
| entryNotional         | String | Notional value of the position, using the average entry price                                                                                                                                             |
| mtmPnl                | String | Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement                                                                         |
| reportedMtmPnl        | String | The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase |
| reportedFundingPnl    | String | Sum of all funding payments received since the position was opened. This is updated every time funding is paid.                                                                                           |
| realizedPnl           | String | Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.                                        |
| settlementAssetSymbol | String | Settlement asset symbol                                                                                                                                                                                   |
| eventType             | String | Derivatives position update event types                                                                                                                                                                   |
| createdAtDatetime     | String | denotes the time the position was created by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| createdAtTimestamp    | String | denotes the time the position was created by the exchange, number of milliseconds since EPOCH                                                                                                             |
| updatedAtDatetime     | String | denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| updatedAtTimestamp    | String | denotes the time the position was updated by the exchange number of milliseconds since EPOCH                                                                                                              |
| publishedAtTimestamp  | String | denotes the time the update was broadcasted to connected websockets                                                                                                                                       |

```json
{
  "tradingAccountId": "1111",
  "type": "snapshot",
  "dataType": "V1TADerivativesPosition",
  "data": [
    {
      "tradingAccountId": "111234567890",
      "symbol": "BTC-USDC-PERP",
      "side": "BUY",
      "quantity": "1.00000000",
      "notional": "30000.0000",
      "entryNotional": "30000.0000",
      "mtmPnl": "110.0000",
      "reportedMtmPnl": "120.0000",
      "reportedFundingPnl": "130.0000",
      "realizedPnl": "140.0000",
      "settlementAssetSymbol": "USDC",
      "eventType": "settlementUpdate",
      "createdAtDatetime": "2020-01-01T00:00:00.000Z",
      "createdAtTimestamp": "1577836800000",
      "updatedAtDatetime": "2020-01-02T00:00:00.000Z",
      "updatedAtTimestamp": "1577923200000",
      "publishedAtTimestamp": "1577923300000"
    }
  ]
}
```

### ammInstruction response

- Provides `updates` of the active AMM instructions on the specific trading
  account ID.
- This topic does not provide `snapshot`

| Name                  | Type   | Description                                                                                                                                   |
| --------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId      | String | id of the trading account                                                                                                                     |
| instructionId         | String | unique AMM instruction ID                                                                                                                     |
| symbol                | String | market symbol                                                                                                                                 |
| baseFee               | String | base fee, see [asset value](#overview--price-and-quantity-precision) format                                                                   |
| quoteFee              | String | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                                                  |
| status                | String | order status                                                                                                                                  |
| statusReason          | String | status reason, describes why the order is in a specific state                                                                                 |
| statusReasonCode      | String | status reason code, see [details](#overview--error--rejection-codes)                                                                          |
| createdAtDatetime     | String | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                                     |
| createdAtTimestamp    | String | denotes the time the order was ACK'd by the exchange                                                                                          |
| baseCurrentQuantity   | String | amount of base asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status                                        |
| baseInvestQuantity    | String | initial base investment                                                                                                                       |
| basePrice             | String | current price of base asset                                                                                                                   |
| baseWithdrawQuantity  | String | amount of base asset returned when AMM instruction is terminated                                                                              |
| currentValue          | String | value of assets (base and quote) in USD amount that this AMM instruction currently holds                                                      |
| feeTierId             | String | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)                                                                     |
| finalValue            | String | value of assets (base and quote) in USD amount when AMM instruction was terminated, only for AMM instruction with `CLOSED` status             |
| impermanentLoss       | String | impermanent loss                                                                                                                              |
| liquidity             | String | liquidity amount                                                                                                                              |
| lastDistributedPrice  | String | (Perpetual market only) The price used at the time of settlement for AMM Instructions that can be used to determine mtmPnl and the actual Pnl |
| lowerBound            | String | lower bound of price range, in quote currency                                                                                                 |
| price                 | String | current price of AMM, see [Get Tick By Symbol](#get-/v1/markets/-symbol-/tick)                                                                |
| quoteCurrentQuantity  | String | amount of quote asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status                                       |
| quoteInvestQuantity   | String | initial quote investment                                                                                                                      |
| quotePrice            | String | current price of quote asset                                                                                                                  |
| quoteWithdrawQuantity | String | amount of quote asset returned when AMM instruction is terminated                                                                             |
| requestID             | String | unique request ID                                                                                                                             |
| updatedAtDatetime     | String | denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string                                         |
| updatedAtTimestamp    | String | denotes the time the AMM instruction was updated by the exchange                                                                              |
| upperBound            | String | upper bound of price range, in quote currency                                                                                                 |

```json
{
  "tradingAccountId": "1111",
  "type": "update",
  "dataType": "V1TAAmmInstruction",
  "data": [
    {
      "instructionId": "100",
      "symbol": "BTCUSDC",
      "baseFee": "1.00000000",
      "quoteFee": "1.0000",
      "status": "OPEN",
      "statusReason": "Ok",
      "statusReasonCode": "1001",
      "createdAtDatetime": "2021-05-20T01:01:01.000Z",
      "createdAtTimestamp": "1621490985000",
      "baseCurrentQuantity": "0.00000000",
      "baseInvestQuantity": "0.00000008",
      "basePrice": "345.67000000",
      "baseWithdrawQuantity": "0.00000010",
      "currentValue": "0.0000",
      "feeTierId": "1",
      "finalValue": "0.0001",
      "impermanentLoss": "0.0000",
      "liquidity": "0.0001",
      "lowerBound": "0.0013",
      "price": "456.7800",
      "quoteCurrentQuantity": "0.0000",
      "quoteInvestQuantity": "0.0009",
      "quotePrice": "1.0000",
      "quoteWithdrawQuantity": "0.0011",
      "lastDistributedPrice": null
      "requestId": "197735387747975680",
      "updatedAtDatetime": "2021-05-20T01:01:01.000Z",
      "updatedAtTimestamp": "1621490985000",
      "upperBound": "14000.0000",
    }
}
```

See
[connect to private data web-socket](https://github.com/bullish-exchange/api-examples/blob/master/websocket/private_data_web_socket.py)
for a sample Python script.

# Quickly Try The API

To quickly try the API you can use the `TRY` green button which you can find on
the bottom right side of each endpoint documentation section, next to
`FILL EXAMPLE` and `CLEAR` buttons.

## Try The Non-Authenticated Endpoints

To try the endpoints for which the authentication is not required follow below
steps:

1. Fill in the input parameters, including the request headers
2. Click the `TRY` green button
3. Inspect the results

## Try The Authenticated Endpoints

To try the endpoints for which authentication is required follow below steps:

1. Obtain a [bearer token](#overview--generate-a-jwt-token) using your API key
2. Set it in the dedicated `api-token` field in the [Authentication](#auth)
   section
3. Fill in the input parameters
4. Click the `TRY` green button
5. Inspect the results

# Test Instruments

Bullish has test instruments which are used for internal testing. Clients can
ignore these test instruments.

## Test Markets

Bullish currently has 1 test market.

- `DEMOONEDEMOTWO`

## Test Assets

Bullish currently has 2 test assets.

- `DEMOONE`
- `DEMOTWO`

# API Change Log

## 2025 Changes

- June
  - new REST API - [Get Historical Trades](#get-/v1/history/trades)
  - new REST API - [Get Historical Orders](#get-/v2/history/orders)
- May
  - Deprecated Features to be removed June 2025:
    - Hybrid OrderBook WebSocket (unauthenticated)
    - Market Data WebSocket (authenticated)
    - Anonymous Trades WebSocket (unauthenticated) true
  - Support for fee rebates - [Get Trades](#get-/v1/trades) new fields
    `tradeRebateAmount` and `tradeRebateAssetSymbol`
- March
  - updated REST API - [Get Trading Account](#tag--trading-accounts) new field
    `tradeFees`
  - updated REST API - [Get Trading Account](#tag--trading-accounts) Deprecated
    fields `makerFee` and `takerFee`
  - updated REST API - [Get Markets](#tag--market-data) new field `feeGroupId`
  - updated REST API - [Get Markets](#tag--market-data) Deprecated fields
    `makerFee` and `takerFee`
  - Removal of request parameter `depth` from
    [Get Market Orderbook](#get-/v1/markets/-symbol-/orderbook/hybrid)
  - Removal of subscription parameter `depth` from
    [unauthenticated multi-orderbook websocket](#overview--multi-orderbook-websocket-unauthenticated)
  - updated REST API - [Order Amendment Commands](#post-/v2/command-amend) new
    command `V1AmendOrder`

## 2024 Changes

- December
  - updated REST API - [Get Trading Account](#tag--trading-accounts) new field
    `totalLiabilitiesUSD`
  - updated WebSocket API - `/private-data` topic - `tradingAccounts` new field
    `totalLiabilitiesUSD`
- November
  - updated REST API - [Get Trading Account](#tag--trading-accounts) new fields
    `liquidityAddonUSD`, `marketRiskUSD` and `marginProfile`
  - updated WebSocket API - `/private-data` topic - `tradingAccounts` new fields
    `liquidityAddonUSD`, `marketRiskUSD` and `marginProfile`
  - updated REST API - [Get Assets](#tag--asset-data) new nested fields
    `underlyingAsset`
  - updated REST API - [Get Markets](#tag--market-data) new field
    `expiryDatetime`
  - Deprecated REST API - [Perpetual Settlement History]
  - New REST API -
    [Derivatives Settlement History](#get-/v1/history/derivatives-settlement)
  - Deprecating REST API - `GET /trading-api/v1/history/perpetual-settlement`
  - WebSocket API `/private-data` new topic - `derivativesPositionV2`
- October
  - New REST API -
    [Portfolio Margin Simulator](#tag--portfolio-margin-simulator)
- September
  - New REST API -
    [Funding Rate History](#get-/v1/history/markets/-symbol-/funding-rate)
  - updated REST API - [Get Markets](#tag--market-data) deprecated fields
    `maxInitialLeverage`, `warningLeverage`, `liquidationLeverage`,
    `fullLiquidationLeverage` and `defaultedLeverage`
- August
  - New
    [Unified Anonymous Trade WS API](#overview--unified-anonymous-trades-websocket-unauthenticated)
- July
  - updated REST API - [Get Trading Account](#tag--trading-accounts) new field
    `isConcentrationRiskEnabled`
  - updated REST API - [Get Markets](#tag--market-data) new fields
    `openInterestUSD`, `concentrationRiskThresholdUSD` and
    `concentrationRiskPercentage`
  - updated REST API - [Get Markets](#tag--market-data) new fields
    `roundingCorrectionFactor`, `makerMinLiquidityAddition`,
    `liquidityInvestEnabled` and `liquidityWithdrawEnabled`
  - New APIs
    - WebSocket `/trading-api/v1/index-data` for
      [index price updates](#overview--index-data-websocket-unauthenticated)
    - REST - `GET /trading-api/v1/index-prices`
    - REST - `GET /trading-api/v1/index-prices/{assetSymbol}`
- June
  - updated REST API - [Get Assets](#tag--asset-data) new fields `name` and
    `collateralBands`
  - updated REST API - [Get Assets](#tag--asset-data) deprecating field
    `collateralRating`
- May
  - WebSocket API `/private-data` new topic - `ammInstructions`
- April
  - Moved deprecated items to
    [Deprecated Features & APIs](.././deprecated/index.html)
    - Bullish Key
    - Old Signing Format
    - Hybrid OrderBook WebSocket (unauthenticated)
    - Market Data WebSocket (authenticated)
    - V1 Orders APIs
    - V1 AMM Instructions APIs
    - REST - `GET /accounts/spot`
    - REST - `GET /accounts/spot/{symbol}`
  - Removed decommissioned items
    - REST - `POST /trading-api/v1/users/login`
- March
  - Added POST_ONLY order type for
    [POST /trading-api/v2/orders](#post-/v2/orders)
  - Added POST_ONLY order type for [GET /markets](#get-/v1/markets)
  - WebSocket API `/private-data` response models contain new field -
    `tradingAccountId`
- February
  - Response model changes for [GET /trading-api/assets](#get-/v1/assets) and
    [GET `/trading-api/assets/{symbol}`](#get-/v1/assets/-symbol-)
    - new fields `totalOfferedLoanQuantity` and `loanBorrowedQuantity`
  - new REST API -
    [GET /trading-api/v1/history/transfer](#get-/v1/history/transfer)
  - Updated REST API -
    [Get Trading-Accounts](#get-/v1/accounts/trading-accounts) new fields
    `totalBorrowedUSD` `totalCollateralUSD`, `initialMarginUSD`,
    `warningMarginUSD` `liquidationMarginUSD`, `fullLiquidationMarginUSD`,
    `defaultedMarginUSD`, `endCustomerId`
  - Updated WS API - `/private-data` websocket, `tradingAccounts` response model
    updated with new fields
- January
  - Added new field `quoteAmount` for response models
    - REST - `GET /orders`
    - REST - `GET /trades`
    - Websocket - `orders` and `trades` topic

## 2023 Changes

- December
  - History APIs require date/time range to be specified.
  - Direct Connect connectivity option added.
  - Deprecation of authenticated L1 websocket in favour of
    [unauthenticated multi-orderbook websocket](#overview--multi-orderbook-websocket-unauthenticated).
  - Deprecation of unauthenticated per-market L2 websocket in favour of
    [unauthenticated multi-orderbook websocket](#overview--multi-orderbook-websocket-unauthenticated).
  - Added test instruments.
- November
  - New APIs for placing commands into the exchange. Uses
    [signing format](#overview--signing-format) and allows non-strict precision
    on price/quantities.
    - [POST /trading-api/v2/orders](#post-/v2/orders) for creating orders,
      [GET /trading-api/v2/orders](#get-/v2/orders) for fetching orders.
    - [POST /trading-api/v2/amm-instructions](#post-/v2/amm-instructions) for
      creating AMM instructions,
      [GET /trading-api/v2/amm-instructions](#get-/v2/amm-instructions) for
      fetching AMM instructions.
    - [POST /trading-api/v2/command](#post-/v2/command) for all other commands.
  - Deprecation of following APIs, will be removed towards the end of Q3 2024.
    - `/trading-api/v1/orders`
    - `/trading-api/v1/amm-instructions`
    - `/trading-api/v1/command`
  - Response model changes for [GET /trading-api/v2/orders](#get-/v2/orders),
    [GET /trading-api/v1/orders](#get-/v1/orders) and
    [Private Data WebSocket (authenticated)](#overview--private-data-websocket-authenticated)
    orders topic.
    - `allowBorrow`, `borrowedBaseQuantity`, `borrowedQuoteQuantity`,
      `clientOrderId` added.
    - `margin`, `borrowedQuantity`, `handle` deprecated, will be removed towards
      the end of Q3 2024.
  - Response model changes for
    [GET /trading-api/v2/amm-instructions](#get-/v2/amm-instructions) and
    [GET /trading-api/v1/amm-instructions](#get-/v1/amm-instructions)
    - `instructionId` added.
    - `liquidityId` deprecated, will be removed towards the end of Q3 2024.
  - New
    [Multi-OrderBook WebSocket (unauthenticated)](#overview--multi-orderbook-websocket-unauthenticated)
    API
  - new REST API - [derivatives positions](#get-/v1/derivatives-positions)
  - new REST API - perpetual settlement
  - updated REST API - [Get Markets](#get-/v1/markets) new fields `marketType`
    and new fields for perpetual market only:
    `contractMultiplier`,`settlementAssetSymbol`, `underlyingBaseSymbol`,
    `underlyingQuoteSymbol`
  - updated REST API - [Get Market Tick](#get-/v1/markets/-symbol-/tick) new
    fields `markPrice` `fundingRate`, `openInterest`
  - updated REST API -
    [Get Trading-Accounts](#get-/v1/accounts/trading-accounts) new field
    `riskLimitUSD`
  - updated REST API - [Get AMM Instruction](#get-/v2/amm-instructions) new
    field `lastDistributedPrice`
  - New
    [Get Market Tick WebSocket API](#overview--anonymous-market-data-price-tick-unauthenticated)
  - Websocket - Updated API
    [Private Data WebSocket](#overview--private-data-websocket-authenticated)
    new Topic `derivativesPositions`
  - Websocket - Updated API
    [Private Data WebSocket](#overview--private-data-websocket-authenticated)
    Topic `tradingAccounts`, response model updated.
- October
  - New heartbeat topic for
    [Private Data WebSocket (authenticated)](#overview--private-data-websocket-authenticated)
  - New ECDSA API Keys and ECDSA based signing
    - Login API - `POST /trading-api/v2/users/login` supports ECDSA signatures
    - BX-SIGNATURE header supports signatures generated via ECDSA
- September
  - New HMAC API Keys and HMAC based signing
    - Login API - `GET /v1/users/hmac/login`
    - BX-SIGNATURE header supports signatures generated via HMAC and EDDSA
  - New FIX API - The FIX API is available to institutional clients
- August - New REST API - `GET /history/borrow-interest`
  - New REST API - `Cancel All Open Limit Orders after Delay` -
    `POST /command?commandType=V1DelayedCancelAllOrders`
  - New REST API - logout session - `GET /v1/users/logout`
- July - Margin related REST and WS changes
  - `borrowedQuantity` and `isLiquidation` fields added to `GET /orders` and
    `WS /private-data` orders topic
  - Calculation for `free` and `used` fields changed in `GET /accounts/spot`,
    `GET /accounts/spot/{symbol}` and `WS /private-data` `spotAccounts` topic
  - New `GET /accounts/asset` and `GET /accounts/asset/{symbol}`
  - New `assetAccounts` and `tradingAccounts` topics in `WS /private-data`
  - `isLending`, `isBorrowing`, `isDefaulted`, `maxInitialLeverage`, `makerFee`
    and `takerFee` fields added to `GET /accounts/trading-accounts`
  - `collateralRating` and `maxBorrow` fields added to `/assets`
- May - add V1CancelAllOrdersByMarket to cancel all open orders by trading
  account id and market
- April - add V1CancelAllOrders to cancel all open limit orders by trading
  account id
- April - add `/accounts/trading-accounts` endpoint to fetch all trading
  accounts
- April - new hybrid orderbook WebSocket API with greater depth and aggregation
  factor
  - Private Data WebSocket `/v1/private-data` ->
    [New API](#overview--private-data-websocket-authenticated)
    `/v1/private-data?tradingAccountId=111234567890`
  - Get account details [Current API](../v1/#get-/accounts/spot) ->
    [New API](#tag--accounts)
  - Get trade details [Current API](../v1/#get-/trades) ->
    [New API](#get-/v1/trades)
  - Get AMM Instruction by ID
    [Current API](../v1/#get-/amm-instructions/-liquidityId-) ->
    [New API](#get-/v2/amm-instructions/-instructionid-)
  - Get AMM instruction details [Current API](../v1/#get-/amm-instructions) ->
    [New API](#get-/v2/amm-instructions)
  - Remove AMM instruction [Current API](../v1/#delete-/amm-instructions) ->
    [New API](../v2/#delete-/amm-instructions)
  - Add AMM instruction [Current API](../v1/#post-/amm-instructions) ->
    [New API](#post-/v2/amm-instructions)
  - Get order details [Current API](../v1/#get-/orders) ->
    [New API](#get-/v2/orders)
  - Cancel order [Current API](../v1/#delete-/orders) ->
    [New API](../v2/#delete-/orders)
  - Create order [Current API](../v1/#post-/orders) ->
    [New API](#post-/v2/orders)
  - Updated REST API:
  - New REST API:
    [To transfer asset between two trading accounts ](#tag--command-entry)
  - New REST API:
    [To retrieve all the trading account details for current user](../v2/#get-/v1/accounts/trading-accounts)
    Gets details for all trading accounts accessible by the API key used in the
    request. Requires
    [bearer token](#overview--add-authenticated-request-header) in authorization
    header. The trading account's id will be used in all other REST API
- March - introduce "trading account Id" to authenticated REST API and websocket
- March - add custody SIGNET support, remove SEN support
- March - add unsolicited amend status reason code
- March - add nonce window to `/orders` to allow out-of-order order requests to
  be processed
- March - `/v1/users/login` to be deprecated towards the end of Q2 2023
- January - add AMM instructions API

## 2022 Changes

- November - deprecate subscription topics in `/private-data` - `events`,
  `positions` and `marginAccounts`
- November - deprecate `/accounts/margin`, `/accounts/margin/{symbol}`,
  `/positions` and `/positions/{symbol}`
- October - add Custody API
- August - add anonymous trades WebSocket API
- July - deprecate WebSocket API `/v1/private` and `/v1/market-data`
- July - add hybrid orderbook WebSocket API
- June - add handle field to `GET /trades` and `GET /trades/{tradeId}`
- June - add handle field to `V1TATrade` for private data WebSocket
- June - orderbook default depth of `10` -
  `GET /markets/{symbol}/orderbook/hybrid`
- May - add `events` topic to private data WebSocket
- April - add market data WebSocket API
- April - add private data WebSocket API
- March - add optional `depth` parameter to
  `GET /markets/{symbol}/orderbook/hybrid?depth=10`
- March - add IOC time-in-force order type
- March - add FOK time-in-force order type
- March - add order `statusReasonCode` map to API documentation
- March - add historical anonymous trades API -
  `GET /history/markets/{symbol}/trades`
- February - add filter by status=CANCELLED to `GET /orders?status=CANCELLED`
- January - add pagination support to `GET /markets/{symbol}/candle`

Base URLs:

- <a href="https://api.exchange.bullish.com/trading-api">https://api.exchange.bullish.com/trading-api</a>

- <a href="https://registered.api.exchange.bullish.com/trading-api">https://registered.api.exchange.bullish.com/trading-api</a>

- <a href="https://prod.access.bullish.com/trading-api">https://prod.access.bullish.com/trading-api</a>

- <a href="https://api.bugbounty.bullish.com/trading-api">https://api.bugbounty.bullish.com/trading-api</a>

- <a href="https://api.simnext.bullish-test.com/trading-api">https://api.simnext.bullish-test.com/trading-api</a>

- <a href="https://registered.api.simnext.bullish-test.com/trading-api">https://registered.api.simnext.bullish-test.com/trading-api</a>

- <a href="https://simnext.access.bullish.com/trading-api">https://simnext.access.bullish.com/trading-api</a>

Email: <a href="mailto:support@bullish.com">Bullish Help Center</a> Web:
<a href="https://support.bullish.com">Bullish Help Center</a>

# Authentication

- HTTP Authentication, scheme: bearer

<h1 id="bullish-trading-api-nonce">nonce</h1>

Non-authenticated API for getting nonce range information

## user-get-current-nonce-range

<a id="opIduser-get-current-nonce-range"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/nonce", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/nonce', headers = headers)

print(r.json())

```

`GET /v1/nonce`

_Get The Current Nonce Range_

Get the current nonce range. The lower bound of nonce range is EPOCH start of
day in microseconds, and upper bound of nonce range is EPOCH end of day in
microseconds.

**Ratelimited:** `False`

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": ["upperBound", "lowerBound"],
  "properties": {
    "lowerBound": {
      "description": "lower bound of nonce range",
      "type": "integer",
      "example": 8455
    },
    "upperBound": {
      "description": "upper bound of nonce range",
      "type": "integer",
      "example": 9455
    }
  }
}
```

<h3 id="user-get-current-nonce-range-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                |
| ------ | -------------------------------------------------------------------------- | --------------------- | --------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Nonce](#schemanonce) |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                  |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                  |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                  |

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="bullish-trading-api-asset-data">asset-data</h1>

Non-authenticated APIs for accessing general asset data information

## asset-data-get-assets

<a id="opIdasset-data-get-assets"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/assets", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/assets', headers = headers)

print(r.json())

```

`GET /v1/assets`

_Get Assets_

Get supported assets. Clients can ignore [test assets](#overview--test-assets).

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 10,
  "items": {
    "type": "object",
    "required": [
      "assetId",
      "symbol",
      "name",
      "precision",
      "minBalanceInterest",
      "minFee",
      "apr",
      "collateralRating",
      "maxBorrow",
      "totalOfferedLoanQuantity",
      "loanBorrowedQuantity",
      "collateralBands",
      "underlyingAsset"
    ],
    "properties": {
      "assetId": {
        "description": "unique asset ID",
        "allOf": [
          {
            "type": "string",
            "description": "unique asset ID",
            "example": "1"
          }
        ]
      },
      "symbol": {
        "description": "asset symbol",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "name": {
        "description": "asset name",
        "allOf": [
          {
            "type": "string",
            "description": "asset name",
            "example": "Bitcoin"
          }
        ]
      },
      "precision": {
        "description": "number of decimal digits 'after the dot' for asset amount",
        "type": "string",
        "example": "8"
      },
      "minBalanceInterest": {
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "minFee": {
        "description": "minimum fee",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "apr": {
        "description": "annualized percentage rate",
        "type": "String",
        "example": "12.50"
      },
      "collateralRating": {
        "deprecated": true,
        "description": "collateral rating applied to this asset, a value of 100.00 indicates 100%. `Deprecated in favour of collateral bands`",
        "type": "String",
        "example": "95.00"
      },
      "maxBorrow": {
        "description": "maximum quantity that can be borrowed for this asset",
        "type": "String",
        "example": "10.00000000"
      },
      "totalOfferedLoanQuantity": {
        "description": "quantity of an asset that is across all loan offers on the exchange",
        "type": "String",
        "example": "5.00000000"
      },
      "loanBorrowedQuantity": {
        "description": "amount of loans that is currently being borrowed for the asset",
        "type": "String",
        "example": "3.00000000"
      },
      "collateralBands": {
        "description": "list of collateral bands for the asset. A collateral band holds the upper limit of the USD notional and the corresponding collateral percentage which applies to it. An asset's collateral value will be capped by the highest limit of the collateral bands, any remaining amount greater than this limit will have a collateral percentage of 0. If an asset has an empty list of CollateralBands, this signifies that the asset has a collateralValue of 0.",
        "type": "array",
        "items": {
          "allOf": [
            {
              "type": "object",
              "properties": {
                "collateralPercentage": {
                  "description": "collateral percentage applied to the asset for this band - a value of 90.00 indicates 90% of the asset is eligible to be used as collateral",
                  "type": "String",
                  "example": "95.00"
                },
                "bandLimitUSD": {
                  "description": "upper limit in USD for this band",
                  "type": "String",
                  "example": "1000000.0000"
                }
              }
            }
          ]
        }
      },
      "underlyingAsset": {
        "description": "underlying asset for the asset.",
        "allOf": [
          {
            "type": "object",
            "properties": {
              "symbol": {
                "description": "underlying asset symbol",
                "type": "String",
                "example": "BTC"
              },
              "assetId": {
                "description": "underlying asset ID",
                "type": "String",
                "example": "1"
              },
              "bpmMinReturnStart": {
                "description": "start of the 1/1000 biggest downward price movement of an underlying asset over 6 hours",
                "type": "String",
                "example": "40.0000"
              },
              "bpmMinReturnEnd": {
                "description": "end of the 1/1000 biggest downward price movement of an underlying asset over 6 hours",
                "type": "String",
                "example": "20.0000"
              },
              "bpmMaxReturnStart": {
                "description": "start of the 1/1000 biggest upward price movement of an underlying asset over 6 hours",
                "type": "String",
                "example": "30.0000"
              },
              "bpmMaxReturnEnd": {
                "description": "end of the 1/1000 biggest upward price movement of an underlying asset over 6 hours",
                "type": "String",
                "example": "50.0000"
              },
              "marketRiskFloorPctStart": {
                "description": "the percentage range of risk reduction allowed for a portfolio",
                "type": "String",
                "example": "1.00"
              },
              "marketRiskFloorPctEnd": {
                "description": "the percentage range of risk reduction allowed for a portfolio",
                "type": "String",
                "example": "5.00"
              },
              "bpmTransitionDateTimeStart": {
                "description": "the start datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset",
                "type": "String",
                "example": "2024-08-02T12:00:00.000Z"
              },
              "bpmTransitionDateTimeEnd": {
                "description": "the end datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset",
                "type": "String",
                "example": "2024-08-02T18:00:00.000Z"
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="asset-data-get-assets-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="asset-data-get-assets-responseschema">Response Schema</h3>

Status Code **200**

| Name                          | Type                                      | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ----------------------------- | ----------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_                   | [[Asset](#schemaasset)]                   | false    | none         | none                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| » assetId                     | [AssetID](#schemaassetid)                 | true     | none         | unique asset ID                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| » symbol                      | [AssetSymbol](#schemaassetsymbol)         | true     | none         | asset symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| » name                        | [AssetName](#schemaassetname)             | true     | none         | asset name                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » precision                   | string                                    | true     | none         | number of decimal digits 'after the dot' for asset amount                                                                                                                                                                                                                                                                                                                                                                                                      |
| » minBalanceInterest          | [AssetValue](#schemaassetvalue)           | true     | none         | see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                                                                                                                                                                                                              |
| » minFee                      | [AssetValue](#schemaassetvalue)           | true     | none         | minimum fee                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| » apr                         | String                                    | true     | none         | annualized percentage rate                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| » collateralRating            | String                                    | true     | none         | collateral rating applied to this asset, a value of 100.00 indicates 100%. `Deprecated in favour of collateral bands`                                                                                                                                                                                                                                                                                                                                          |
| » maxBorrow                   | String                                    | true     | none         | maximum quantity that can be borrowed for this asset                                                                                                                                                                                                                                                                                                                                                                                                           |
| » totalOfferedLoanQuantity    | String                                    | true     | none         | quantity of an asset that is across all loan offers on the exchange                                                                                                                                                                                                                                                                                                                                                                                            |
| » loanBorrowedQuantity        | String                                    | true     | none         | amount of loans that is currently being borrowed for the asset                                                                                                                                                                                                                                                                                                                                                                                                 |
| » collateralBands             | [allOf]                                   | true     | none         | list of collateral bands for the asset. A collateral band holds the upper limit of the USD notional and the corresponding collateral percentage which applies to it. An asset's collateral value will be capped by the highest limit of the collateral bands, any remaining amount greater than this limit will have a collateral percentage of 0. If an asset has an empty list of CollateralBands, this signifies that the asset has a collateralValue of 0. |
| »» collateralPercentage       | String                                    | false    | none         | collateral percentage applied to the asset for this band - a value of 90.00 indicates 90% of the asset is eligible to be used as collateral                                                                                                                                                                                                                                                                                                                    |
| »» bandLimitUSD               | String                                    | false    | none         | upper limit in USD for this band                                                                                                                                                                                                                                                                                                                                                                                                                               |
| » underlyingAsset             | [UnderlyingAsset](#schemaunderlyingasset) | true     | none         | underlying asset for the asset.                                                                                                                                                                                                                                                                                                                                                                                                                                |
| »» symbol                     | String                                    | false    | none         | underlying asset symbol                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| »» assetId                    | String                                    | false    | none         | underlying asset ID                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| »» bpmMinReturnStart          | String                                    | false    | none         | start of the 1/1000 biggest downward price movement of an underlying asset over 6 hours                                                                                                                                                                                                                                                                                                                                                                        |
| »» bpmMinReturnEnd            | String                                    | false    | none         | end of the 1/1000 biggest downward price movement of an underlying asset over 6 hours                                                                                                                                                                                                                                                                                                                                                                          |
| »» bpmMaxReturnStart          | String                                    | false    | none         | start of the 1/1000 biggest upward price movement of an underlying asset over 6 hours                                                                                                                                                                                                                                                                                                                                                                          |
| »» bpmMaxReturnEnd            | String                                    | false    | none         | end of the 1/1000 biggest upward price movement of an underlying asset over 6 hours                                                                                                                                                                                                                                                                                                                                                                            |
| »» marketRiskFloorPctStart    | String                                    | false    | none         | the percentage range of risk reduction allowed for a portfolio                                                                                                                                                                                                                                                                                                                                                                                                 |
| »» marketRiskFloorPctEnd      | String                                    | false    | none         | the percentage range of risk reduction allowed for a portfolio                                                                                                                                                                                                                                                                                                                                                                                                 |
| »» bpmTransitionDateTimeStart | String                                    | false    | none         | the start datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset                                                                                                                                                                                                                                                                                                                                  |
| »» bpmTransitionDateTimeEnd   | String                                    | false    | none         | the end datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset                                                                                                                                                                                                                                                                                                                                    |

<aside class="success">
This operation does not require authentication
</aside>

## asset-data-get-asset

<a id="opIdasset-data-get-asset"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/assets/{symbol}", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/assets/{symbol}', headers = headers)

print(r.json())

```

`GET /v1/assets/{symbol}`

_Get Asset by Symbol_

Get Asset by Symbol

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "assetId",
    "symbol",
    "name",
    "precision",
    "minBalanceInterest",
    "minFee",
    "apr",
    "collateralRating",
    "maxBorrow",
    "totalOfferedLoanQuantity",
    "loanBorrowedQuantity",
    "collateralBands",
    "underlyingAsset"
  ],
  "properties": {
    "assetId": {
      "description": "unique asset ID",
      "allOf": [
        {
          "type": "string",
          "description": "unique asset ID",
          "example": "1"
        }
      ]
    },
    "symbol": {
      "description": "asset symbol",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "name": {
      "description": "asset name",
      "allOf": [
        {
          "type": "string",
          "description": "asset name",
          "example": "Bitcoin"
        }
      ]
    },
    "precision": {
      "description": "number of decimal digits 'after the dot' for asset amount",
      "type": "string",
      "example": "8"
    },
    "minBalanceInterest": {
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "minFee": {
      "description": "minimum fee",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "apr": {
      "description": "annualized percentage rate",
      "type": "String",
      "example": "12.50"
    },
    "collateralRating": {
      "deprecated": true,
      "description": "collateral rating applied to this asset, a value of 100.00 indicates 100%. `Deprecated in favour of collateral bands`",
      "type": "String",
      "example": "95.00"
    },
    "maxBorrow": {
      "description": "maximum quantity that can be borrowed for this asset",
      "type": "String",
      "example": "10.00000000"
    },
    "totalOfferedLoanQuantity": {
      "description": "quantity of an asset that is across all loan offers on the exchange",
      "type": "String",
      "example": "5.00000000"
    },
    "loanBorrowedQuantity": {
      "description": "amount of loans that is currently being borrowed for the asset",
      "type": "String",
      "example": "3.00000000"
    },
    "collateralBands": {
      "description": "list of collateral bands for the asset. A collateral band holds the upper limit of the USD notional and the corresponding collateral percentage which applies to it. An asset's collateral value will be capped by the highest limit of the collateral bands, any remaining amount greater than this limit will have a collateral percentage of 0. If an asset has an empty list of CollateralBands, this signifies that the asset has a collateralValue of 0.",
      "type": "array",
      "items": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "collateralPercentage": {
                "description": "collateral percentage applied to the asset for this band - a value of 90.00 indicates 90% of the asset is eligible to be used as collateral",
                "type": "String",
                "example": "95.00"
              },
              "bandLimitUSD": {
                "description": "upper limit in USD for this band",
                "type": "String",
                "example": "1000000.0000"
              }
            }
          }
        ]
      }
    },
    "underlyingAsset": {
      "description": "underlying asset for the asset.",
      "allOf": [
        {
          "type": "object",
          "properties": {
            "symbol": {
              "description": "underlying asset symbol",
              "type": "String",
              "example": "BTC"
            },
            "assetId": {
              "description": "underlying asset ID",
              "type": "String",
              "example": "1"
            },
            "bpmMinReturnStart": {
              "description": "start of the 1/1000 biggest downward price movement of an underlying asset over 6 hours",
              "type": "String",
              "example": "40.0000"
            },
            "bpmMinReturnEnd": {
              "description": "end of the 1/1000 biggest downward price movement of an underlying asset over 6 hours",
              "type": "String",
              "example": "20.0000"
            },
            "bpmMaxReturnStart": {
              "description": "start of the 1/1000 biggest upward price movement of an underlying asset over 6 hours",
              "type": "String",
              "example": "30.0000"
            },
            "bpmMaxReturnEnd": {
              "description": "end of the 1/1000 biggest upward price movement of an underlying asset over 6 hours",
              "type": "String",
              "example": "50.0000"
            },
            "marketRiskFloorPctStart": {
              "description": "the percentage range of risk reduction allowed for a portfolio",
              "type": "String",
              "example": "1.00"
            },
            "marketRiskFloorPctEnd": {
              "description": "the percentage range of risk reduction allowed for a portfolio",
              "type": "String",
              "example": "5.00"
            },
            "bpmTransitionDateTimeStart": {
              "description": "the start datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset",
              "type": "String",
              "example": "2024-08-02T12:00:00.000Z"
            },
            "bpmTransitionDateTimeEnd": {
              "description": "the end datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset",
              "type": "String",
              "example": "2024-08-02T18:00:00.000Z"
            }
          }
        }
      ]
    }
  }
}
```

<h3 id="asset-data-get-asset-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                |
| ------ | -------------------------------------------------------------------------- | --------------------- | --------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Asset](#schemaasset) |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                  |

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="bullish-trading-api-market-data">market-data</h1>

Non-authenticated APIs for accessing general market data information

## market-data-get-markets

<a id="opIdmarket-data-get-markets"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/markets", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets', headers = headers)

print(r.json())

```

`GET /v1/markets`

_Get Markets_

Get Markets. Clients can ignore [test markets](#overview--test-markets). Note ->
"Leverage = Collateral ÷ (Collateral - Debt)"

<h3 id="market-data-get-markets-parameters">Parameters</h3>

| Name       | In    | Type                                            | Required | Description                            |
| ---------- | ----- | ----------------------------------------------- | -------- | -------------------------------------- |
| marketType | query | [MarketTypeAsString](#schemamarkettypeasstring) | false    | Market Types to filter markets against |

#### Enumerated Values

| Parameter  | Value        |
| ---------- | ------------ |
| marketType | SPOT         |
| marketType | PERPETUAL    |
| marketType | DATED_FUTURE |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 10,
  "items": {
    "type": "object",
    "required": [
      "marketId",
      "symbol",
      "quoteAssetId",
      "baseAssetId",
      "quoteSymbol",
      "baseSymbol",
      "quotePrecision",
      "basePrecision",
      "pricePrecision",
      "quantityPrecision",
      "costPrecision",
      "priceBuffer",
      "minQuantityLimit",
      "maxQuantityLimit",
      "maxPriceLimit",
      "minPriceLimit",
      "maxCostLimit",
      "minCostLimit",
      "timeZone",
      "tickSize",
      "liquidityTickSize",
      "liquidityPrecision",
      "feeGroupId",
      "roundingCorrectionFactor",
      "makerMinLiquidityAddition",
      "spotTradingEnabled",
      "marginTradingEnabled",
      "marketEnabled",
      "createOrderEnabled",
      "cancelOrderEnabled",
      "liquidityInvestEnabled",
      "liquidityWithdrawEnabled",
      "feeTiers",
      "marketType",
      "openInterestUSD",
      "concentrationRiskThresholdUSD",
      "concentrationRiskPercentage",
      "expiryDatetime"
    ],
    "properties": {
      "marketId": {
        "description": "unique market ID",
        "allOf": [
          {
            "type": "string",
            "example": "10000"
          }
        ]
      },
      "symbol": {
        "description": "market symbol",
        "allOf": [
          {
            "type": "string",
            "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
            "example": "BTCUSDC"
          }
        ]
      },
      "baseSymbol": {
        "description": "base asset symbol (only applies to spot market)",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "underlyingBaseSymbol": {
        "description": "underlying base asset symbol (only applies to derivative market)",
        "example": null,
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "quoteSymbol": {
        "description": "quote asset symbol (only applies to spot market)",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "underlyingQuoteSymbol": {
        "description": "underlying quote asset symbol (only applies to derivative market)",
        "example": null,
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "quoteAssetId": {
        "description": "quote asset id",
        "allOf": [
          {
            "type": "string",
            "description": "unique asset ID",
            "example": "1"
          }
        ]
      },
      "baseAssetId": {
        "description": "base asset id",
        "allOf": [
          {
            "type": "string",
            "description": "unique asset ID",
            "example": "1"
          }
        ]
      },
      "quotePrecision": {
        "description": "quote precision",
        "type": "integer",
        "example": 4
      },
      "basePrecision": {
        "description": "base precision",
        "type": "integer",
        "example": 8
      },
      "pricePrecision": {
        "description": "number of decimal digits 'after the dot' for price",
        "type": "integer",
        "example": 8
      },
      "quantityPrecision": {
        "description": "number of decimal digits 'after the dot' for quantity",
        "type": "integer",
        "example": 8
      },
      "costPrecision": {
        "description": "number of decimal digits 'after the dot' for cost, `price * quantity`",
        "type": "integer",
        "example": 8
      },
      "priceBuffer": {
        "description": "buffer range of limit price from the last traded price.",
        "type": "string",
        "example": 0.3
      },
      "minQuantityLimit": {
        "description": "order quantity should be > min, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "maxQuantityLimit": {
        "description": "order quantity should be < max, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "maxPriceLimit": {
        "description": "order price should be < max, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "minPriceLimit": {
        "description": "order price should be > min, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "maxCostLimit": {
        "description": "order cost, `price * quantity` should be < max, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "minCostLimit": {
        "description": "order cost, `price * quantity` should be > min, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "timeZone": {
        "description": "time zone",
        "type": "string",
        "example": "Etc/UTC"
      },
      "tickSize": {
        "description": "tick size, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "liquidityTickSize": {
        "description": "liquidity tick size.",
        "type": "string",
        "example": "100.0000"
      },
      "liquidityPrecision": {
        "description": "liquidity precision.",
        "type": "integer",
        "example": 4
      },
      "makerFee": {
        "description": "Deprecated and no longer accurate. See `feeGroupId`",
        "type": "integer",
        "example": 0,
        "deprecated": true
      },
      "takerFee": {
        "description": "Deprecated and no longer accurate. See `feeGroupId`",
        "type": "integer",
        "example": 2,
        "deprecated": true
      },
      "roundingCorrectionFactor": {
        "description": "rounding correction factor for market",
        "type": "string",
        "example": "0.00000001"
      },
      "makerMinLiquidityAddition": {
        "description": "minimum amount required to invest liquidity to market.",
        "type": "string",
        "example": "5000"
      },
      "orderTypes": {
        "type": "array",
        "items": {
          "allOf": [
            {
              "type": "string",
              "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
              "example": "LMT"
            }
          ]
        }
      },
      "spotTradingEnabled": {
        "description": "spot trading enabled (only applies for Spot markets)",
        "type": "boolean",
        "example": true
      },
      "marginTradingEnabled": {
        "description": "margin trading enabled (only applies for Spot markets)",
        "type": "boolean",
        "example": true
      },
      "marketEnabled": {
        "description": "market enabled",
        "type": "boolean",
        "example": true
      },
      "createOrderEnabled": {
        "description": "able to create order",
        "type": "boolean",
        "example": true
      },
      "cancelOrderEnabled": {
        "description": "able to cancel order",
        "type": "boolean",
        "example": true
      },
      "liquidityInvestEnabled": {
        "description": "able to invest liquidity to market.",
        "type": "boolean",
        "example": true
      },
      "liquidityWithdrawEnabled": {
        "description": "able to withdraw liquidity from market.",
        "type": "boolean",
        "example": true
      },
      "feeGroupId": {
        "description": "Identifier to the trade fee assigned to this market. Used with `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-)",
        "type": "integer",
        "example": 1
      },
      "feeTiers": {
        "description": "all available fee tiers.",
        "type": "array",
        "minItems": 0,
        "items": {
          "allOf": [
            {
              "type": "object",
              "description": "unique fee tier",
              "required": [
                "feeTierId",
                "staticSpreadFee",
                "isDislocationEnabled"
              ],
              "properties": {
                "feeTierId": {
                  "allOf": [
                    {
                      "type": "string",
                      "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
                      "example": "1"
                    }
                  ]
                },
                "staticSpreadFee": {
                  "description": "static spread fee",
                  "type": "string",
                  "example": "0.00040000"
                },
                "isDislocationEnabled": {
                  "description": "dislocation enabled/disabled",
                  "type": "boolean",
                  "example": true
                }
              }
            }
          ]
        }
      },
      "marketType": {
        "description": "market type, e.g. \"SPOT\" for market like \"BTCUSD\", \"PERPETUAL\" for market like \"BTC-USDC-PERP\", \"DATED_FUTURE\" for market like \"BTC-USDC-20250901\", \"OPTION\" for market like \"BTC-USDC-20250901-90000-C\"",
        "allOf": [
          {
            "type": "string",
            "description": "market type can have the following string values `\"SPOT\"`, `\"PERPETUAL\"`, `\"DATED_FUTURE\"`",
            "enum": ["SPOT", "PERPETUAL", "DATED_FUTURE"],
            "example": "SPOT"
          }
        ]
      },
      "contractMultiplier": {
        "description": "contract multiplier. (only applies to perpetual market)",
        "type": "integer",
        "example": null
      },
      "settlementAssetSymbol": {
        "description": "settlement asset symbol. (only applies to perpetual market)",
        "type": "string",
        "example": null
      },
      "openInterestUSD": {
        "description": "cumulative notional value of all open interest for a specific derivative contract on the exchange.",
        "type": "string",
        "example": null
      },
      "concentrationRiskThresholdUSD": {
        "description": "open interest notional of an account for a specific derivative contract.",
        "type": "string",
        "example": null
      },
      "concentrationRiskPercentage": {
        "description": "percentage of the total open interest for a specific derivative contract.",
        "type": "string",
        "example": null
      },
      "expiryDatetime": {
        "description": "denotes the time when the market expires in ISO 8601 with millisecond format as string",
        "type": "string",
        "example": "2024-10-04T08:00:00.000Z"
      }
    }
  }
}
```

<h3 id="market-data-get-markets-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Not Found             | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="market-data-get-markets-responseschema">Response Schema</h3>

Status Code **200**

| Name                            | Type                                            | Required | Restrictions | Description                                                                                                                                                                                              |
| ------------------------------- | ----------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_                     | [[Market](#schemamarket)]                       | false    | none         | none                                                                                                                                                                                                     |
| » marketId                      | [MarketID](#schemamarketid)                     | true     | none         | unique market ID                                                                                                                                                                                         |
| » symbol                        | [MarketSymbol](#schemamarketsymbol)             | true     | none         | market symbol                                                                                                                                                                                            |
| » baseSymbol                    | [AssetSymbol](#schemaassetsymbol)               | true     | none         | base asset symbol (only applies to spot market)                                                                                                                                                          |
| » underlyingBaseSymbol          | [AssetSymbol](#schemaassetsymbol)               | false    | none         | underlying base asset symbol (only applies to derivative market)                                                                                                                                         |
| » quoteSymbol                   | [AssetSymbol](#schemaassetsymbol)               | true     | none         | quote asset symbol (only applies to spot market)                                                                                                                                                         |
| » underlyingQuoteSymbol         | [AssetSymbol](#schemaassetsymbol)               | false    | none         | underlying quote asset symbol (only applies to derivative market)                                                                                                                                        |
| » quoteAssetId                  | [AssetID](#schemaassetid)                       | true     | none         | quote asset id                                                                                                                                                                                           |
| » baseAssetId                   | [AssetID](#schemaassetid)                       | true     | none         | base asset id                                                                                                                                                                                            |
| » quotePrecision                | integer                                         | true     | none         | quote precision                                                                                                                                                                                          |
| » basePrecision                 | integer                                         | true     | none         | base precision                                                                                                                                                                                           |
| » pricePrecision                | integer                                         | true     | none         | number of decimal digits 'after the dot' for price                                                                                                                                                       |
| » quantityPrecision             | integer                                         | true     | none         | number of decimal digits 'after the dot' for quantity                                                                                                                                                    |
| » costPrecision                 | integer                                         | true     | none         | number of decimal digits 'after the dot' for cost, `price * quantity`                                                                                                                                    |
| » priceBuffer                   | string                                          | true     | none         | buffer range of limit price from the last traded price.                                                                                                                                                  |
| » minQuantityLimit              | [AssetValue](#schemaassetvalue)                 | true     | none         | order quantity should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                                        |
| » maxQuantityLimit              | [AssetValue](#schemaassetvalue)                 | true     | none         | order quantity should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                                        |
| » maxPriceLimit                 | [AssetValue](#schemaassetvalue)                 | true     | none         | order price should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                                           |
| » minPriceLimit                 | [AssetValue](#schemaassetvalue)                 | true     | none         | order price should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                                           |
| » maxCostLimit                  | [AssetValue](#schemaassetvalue)                 | true     | none         | order cost, `price * quantity` should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| » minCostLimit                  | [AssetValue](#schemaassetvalue)                 | true     | none         | order cost, `price * quantity` should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| » timeZone                      | string                                          | true     | none         | time zone                                                                                                                                                                                                |
| » tickSize                      | [AssetValue](#schemaassetvalue)                 | true     | none         | tick size, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                             |
| » liquidityTickSize             | string                                          | true     | none         | liquidity tick size.                                                                                                                                                                                     |
| » liquidityPrecision            | integer                                         | true     | none         | liquidity precision.                                                                                                                                                                                     |
| » makerFee                      | integer                                         | false    | none         | Deprecated and no longer accurate. See `feeGroupId`                                                                                                                                                      |
| » takerFee                      | integer                                         | false    | none         | Deprecated and no longer accurate. See `feeGroupId`                                                                                                                                                      |
| » roundingCorrectionFactor      | string                                          | true     | none         | rounding correction factor for market                                                                                                                                                                    |
| » makerMinLiquidityAddition     | string                                          | true     | none         | minimum amount required to invest liquidity to market.                                                                                                                                                   |
| » orderTypes                    | [allOf]                                         | false    | none         | none                                                                                                                                                                                                     |
| » spotTradingEnabled            | boolean                                         | true     | none         | spot trading enabled (only applies for Spot markets)                                                                                                                                                     |
| » marginTradingEnabled          | boolean                                         | true     | none         | margin trading enabled (only applies for Spot markets)                                                                                                                                                   |
| » marketEnabled                 | boolean                                         | true     | none         | market enabled                                                                                                                                                                                           |
| » createOrderEnabled            | boolean                                         | true     | none         | able to create order                                                                                                                                                                                     |
| » cancelOrderEnabled            | boolean                                         | true     | none         | able to cancel order                                                                                                                                                                                     |
| » liquidityInvestEnabled        | boolean                                         | true     | none         | able to invest liquidity to market.                                                                                                                                                                      |
| » liquidityWithdrawEnabled      | boolean                                         | true     | none         | able to withdraw liquidity from market.                                                                                                                                                                  |
| » feeGroupId                    | integer                                         | true     | none         | Identifier to the trade fee assigned to this market. Used with `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-)                                            |
| » feeTiers                      | [allOf]                                         | true     | none         | all available fee tiers.                                                                                                                                                                                 |
| »» feeTierId                    | [FeeTierId](#schemafeetierid)                   | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)                                                                                                                                |
| »» staticSpreadFee              | string                                          | true     | none         | static spread fee                                                                                                                                                                                        |
| »» isDislocationEnabled         | boolean                                         | true     | none         | dislocation enabled/disabled                                                                                                                                                                             |
| » marketType                    | [MarketTypeAsString](#schemamarkettypeasstring) | true     | none         | market type, e.g. "SPOT" for market like "BTCUSD", "PERPETUAL" for market like "BTC-USDC-PERP", "DATED_FUTURE" for market like "BTC-USDC-20250901", "OPTION" for market like "BTC-USDC-20250901-90000-C" |
| » contractMultiplier            | integer                                         | false    | none         | contract multiplier. (only applies to perpetual market)                                                                                                                                                  |
| » settlementAssetSymbol         | string                                          | false    | none         | settlement asset symbol. (only applies to perpetual market)                                                                                                                                              |
| » openInterestUSD               | string                                          | true     | none         | cumulative notional value of all open interest for a specific derivative contract on the exchange.                                                                                                       |
| » concentrationRiskThresholdUSD | string                                          | true     | none         | open interest notional of an account for a specific derivative contract.                                                                                                                                 |
| » concentrationRiskPercentage   | string                                          | true     | none         | percentage of the total open interest for a specific derivative contract.                                                                                                                                |
| » expiryDatetime                | string                                          | true     | none         | denotes the time when the market expires in ISO 8601 with millisecond format as string                                                                                                                   |

#### Enumerated Values

| Property   | Value        |
| ---------- | ------------ |
| marketType | SPOT         |
| marketType | PERPETUAL    |
| marketType | DATED_FUTURE |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-get-market

<a id="opIdmarket-data-get-market"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}', headers = headers)

print(r.json())

```

`GET /v1/markets/{symbol}`

_Get Market by Symbol_

Get Market by Symbol. Note -> "Leverage = Collateral ÷ (Collateral - Debt)"

<h3 id="market-data-get-market-parameters">Parameters</h3>

| Name   | In   | Type                                | Required | Description |
| ------ | ---- | ----------------------------------- | -------- | ----------- |
| symbol | path | [MarketSymbol](#schemamarketsymbol) | true     | none        |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "marketId",
    "symbol",
    "quoteAssetId",
    "baseAssetId",
    "quoteSymbol",
    "baseSymbol",
    "quotePrecision",
    "basePrecision",
    "pricePrecision",
    "quantityPrecision",
    "costPrecision",
    "priceBuffer",
    "minQuantityLimit",
    "maxQuantityLimit",
    "maxPriceLimit",
    "minPriceLimit",
    "maxCostLimit",
    "minCostLimit",
    "timeZone",
    "tickSize",
    "liquidityTickSize",
    "liquidityPrecision",
    "feeGroupId",
    "roundingCorrectionFactor",
    "makerMinLiquidityAddition",
    "spotTradingEnabled",
    "marginTradingEnabled",
    "marketEnabled",
    "createOrderEnabled",
    "cancelOrderEnabled",
    "liquidityInvestEnabled",
    "liquidityWithdrawEnabled",
    "feeTiers",
    "marketType",
    "openInterestUSD",
    "concentrationRiskThresholdUSD",
    "concentrationRiskPercentage",
    "expiryDatetime"
  ],
  "properties": {
    "marketId": {
      "description": "unique market ID",
      "allOf": [
        {
          "type": "string",
          "example": "10000"
        }
      ]
    },
    "symbol": {
      "description": "market symbol",
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "baseSymbol": {
      "description": "base asset symbol (only applies to spot market)",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "underlyingBaseSymbol": {
      "description": "underlying base asset symbol (only applies to derivative market)",
      "example": null,
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "quoteSymbol": {
      "description": "quote asset symbol (only applies to spot market)",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "underlyingQuoteSymbol": {
      "description": "underlying quote asset symbol (only applies to derivative market)",
      "example": null,
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "quoteAssetId": {
      "description": "quote asset id",
      "allOf": [
        {
          "type": "string",
          "description": "unique asset ID",
          "example": "1"
        }
      ]
    },
    "baseAssetId": {
      "description": "base asset id",
      "allOf": [
        {
          "type": "string",
          "description": "unique asset ID",
          "example": "1"
        }
      ]
    },
    "quotePrecision": {
      "description": "quote precision",
      "type": "integer",
      "example": 4
    },
    "basePrecision": {
      "description": "base precision",
      "type": "integer",
      "example": 8
    },
    "pricePrecision": {
      "description": "number of decimal digits 'after the dot' for price",
      "type": "integer",
      "example": 8
    },
    "quantityPrecision": {
      "description": "number of decimal digits 'after the dot' for quantity",
      "type": "integer",
      "example": 8
    },
    "costPrecision": {
      "description": "number of decimal digits 'after the dot' for cost, `price * quantity`",
      "type": "integer",
      "example": 8
    },
    "priceBuffer": {
      "description": "buffer range of limit price from the last traded price.",
      "type": "string",
      "example": 0.3
    },
    "minQuantityLimit": {
      "description": "order quantity should be > min, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "maxQuantityLimit": {
      "description": "order quantity should be < max, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "maxPriceLimit": {
      "description": "order price should be < max, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "minPriceLimit": {
      "description": "order price should be > min, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "maxCostLimit": {
      "description": "order cost, `price * quantity` should be < max, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "minCostLimit": {
      "description": "order cost, `price * quantity` should be > min, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "timeZone": {
      "description": "time zone",
      "type": "string",
      "example": "Etc/UTC"
    },
    "tickSize": {
      "description": "tick size, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "liquidityTickSize": {
      "description": "liquidity tick size.",
      "type": "string",
      "example": "100.0000"
    },
    "liquidityPrecision": {
      "description": "liquidity precision.",
      "type": "integer",
      "example": 4
    },
    "makerFee": {
      "description": "Deprecated and no longer accurate. See `feeGroupId`",
      "type": "integer",
      "example": 0,
      "deprecated": true
    },
    "takerFee": {
      "description": "Deprecated and no longer accurate. See `feeGroupId`",
      "type": "integer",
      "example": 2,
      "deprecated": true
    },
    "roundingCorrectionFactor": {
      "description": "rounding correction factor for market",
      "type": "string",
      "example": "0.00000001"
    },
    "makerMinLiquidityAddition": {
      "description": "minimum amount required to invest liquidity to market.",
      "type": "string",
      "example": "5000"
    },
    "orderTypes": {
      "type": "array",
      "items": {
        "allOf": [
          {
            "type": "string",
            "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
            "example": "LMT"
          }
        ]
      }
    },
    "spotTradingEnabled": {
      "description": "spot trading enabled (only applies for Spot markets)",
      "type": "boolean",
      "example": true
    },
    "marginTradingEnabled": {
      "description": "margin trading enabled (only applies for Spot markets)",
      "type": "boolean",
      "example": true
    },
    "marketEnabled": {
      "description": "market enabled",
      "type": "boolean",
      "example": true
    },
    "createOrderEnabled": {
      "description": "able to create order",
      "type": "boolean",
      "example": true
    },
    "cancelOrderEnabled": {
      "description": "able to cancel order",
      "type": "boolean",
      "example": true
    },
    "liquidityInvestEnabled": {
      "description": "able to invest liquidity to market.",
      "type": "boolean",
      "example": true
    },
    "liquidityWithdrawEnabled": {
      "description": "able to withdraw liquidity from market.",
      "type": "boolean",
      "example": true
    },
    "feeGroupId": {
      "description": "Identifier to the trade fee assigned to this market. Used with `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-)",
      "type": "integer",
      "example": 1
    },
    "feeTiers": {
      "description": "all available fee tiers.",
      "type": "array",
      "minItems": 0,
      "items": {
        "allOf": [
          {
            "type": "object",
            "description": "unique fee tier",
            "required": [
              "feeTierId",
              "staticSpreadFee",
              "isDislocationEnabled"
            ],
            "properties": {
              "feeTierId": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
                    "example": "1"
                  }
                ]
              },
              "staticSpreadFee": {
                "description": "static spread fee",
                "type": "string",
                "example": "0.00040000"
              },
              "isDislocationEnabled": {
                "description": "dislocation enabled/disabled",
                "type": "boolean",
                "example": true
              }
            }
          }
        ]
      }
    },
    "marketType": {
      "description": "market type, e.g. \"SPOT\" for market like \"BTCUSD\", \"PERPETUAL\" for market like \"BTC-USDC-PERP\", \"DATED_FUTURE\" for market like \"BTC-USDC-20250901\", \"OPTION\" for market like \"BTC-USDC-20250901-90000-C\"",
      "allOf": [
        {
          "type": "string",
          "description": "market type can have the following string values `\"SPOT\"`, `\"PERPETUAL\"`, `\"DATED_FUTURE\"`",
          "enum": ["SPOT", "PERPETUAL", "DATED_FUTURE"],
          "example": "SPOT"
        }
      ]
    },
    "contractMultiplier": {
      "description": "contract multiplier. (only applies to perpetual market)",
      "type": "integer",
      "example": null
    },
    "settlementAssetSymbol": {
      "description": "settlement asset symbol. (only applies to perpetual market)",
      "type": "string",
      "example": null
    },
    "openInterestUSD": {
      "description": "cumulative notional value of all open interest for a specific derivative contract on the exchange.",
      "type": "string",
      "example": null
    },
    "concentrationRiskThresholdUSD": {
      "description": "open interest notional of an account for a specific derivative contract.",
      "type": "string",
      "example": null
    },
    "concentrationRiskPercentage": {
      "description": "percentage of the total open interest for a specific derivative contract.",
      "type": "string",
      "example": null
    },
    "expiryDatetime": {
      "description": "denotes the time when the market expires in ISO 8601 with millisecond format as string",
      "type": "string",
      "example": "2024-10-04T08:00:00.000Z"
    }
  }
}
```

<h3 id="market-data-get-market-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                  |
| ------ | -------------------------------------------------------------------------- | --------------------- | ----------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Market](#schemamarket) |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Not Found             | None                    |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                    |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                    |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-get-market-orderbook

<a id="opIdmarket-data-get-market-orderbook"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/orderbook/hybrid",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/orderbook/hybrid', headers = headers)

print(r.json())

```

`GET /v1/markets/{symbol}/orderbook/hybrid`

_Get Market Order Book_

Get Order Book by Market Symbol

**Ratelimited:** `False`

<h3 id="market-data-get-market-orderbook-parameters">Parameters</h3>

| Name   | In   | Type                                | Required | Description   |
| ------ | ---- | ----------------------------------- | -------- | ------------- |
| symbol | path | [MarketSymbol](#schemamarketsymbol) | true     | symbol to get |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": ["bids", "asks", "datetime", "timestamp", "sequenceNumber"],
  "properties": {
    "bids": {
      "description": "bids",
      "type": "array",
      "minItems": 0,
      "maxItems": 10,
      "items": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "price": {
                "allOf": [
                  {
                    "description": "see [asset value](#overview--price-and-quantity-precision) format",
                    "type": "string",
                    "example": "1.00000000"
                  }
                ]
              },
              "priceLevelQuantity": {
                "allOf": [
                  {
                    "description": "see [asset value](#overview--price-and-quantity-precision) format",
                    "type": "string",
                    "example": "1.00000000"
                  }
                ]
              }
            }
          }
        ]
      }
    },
    "asks": {
      "description": "asks",
      "type": "array",
      "minItems": 0,
      "maxItems": 10,
      "items": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "price": {
                "allOf": [
                  {
                    "description": "see [asset value](#overview--price-and-quantity-precision) format",
                    "type": "string",
                    "example": "1.00000000"
                  }
                ]
              },
              "priceLevelQuantity": {
                "allOf": [
                  {
                    "description": "see [asset value](#overview--price-and-quantity-precision) format",
                    "type": "string",
                    "example": "1.00000000"
                  }
                ]
              }
            }
          }
        ]
      }
    },
    "datetime": {
      "description": "date and time of order book snapshot, ISO 8601 with millisecond as string",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    },
    "timestamp": {
      "description": "timestamp of order book snapshot",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "sequenceNumber": {
      "description": "an incremented unique identifier of the order book snapshot",
      "type": "integer",
      "example": 999
    }
  }
}
```

<h3 id="market-data-get-market-orderbook-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                        |
| ------ | -------------------------------------------------------------------------- | --------------------- | ----------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [OrderBook](#schemaorderbook) |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None                          |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                          |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                          |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-get-anonymous-trades-by-market-symbol

<a id="opIdmarket-data-get-anonymous-trades-by-market-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/trades",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/trades', headers = headers)

print(r.json())

```

`GET /v1/markets/{symbol}/trades`

_Get Latest Market Trades_

Get Market Trades by Market Symbol.

- return 100 most recent trades
- lookup from local cache

**Ratelimited:** `False`

<h3 id="market-data-get-anonymous-trades-by-market-symbol-parameters">Parameters</h3>

| Name   | In   | Type                                | Required | Description   |
| ------ | ---- | ----------------------------------- | -------- | ------------- |
| symbol | path | [MarketSymbol](#schemamarketsymbol) | true     | symbol to get |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 100,
  "items": {
    "type": "object",
    "required": [
      "tradeId",
      "symbol",
      "price",
      "quantity",
      "side",
      "isTaker",
      "createdAtTimestamp",
      "createdAtDatetime"
    ],
    "properties": {
      "tradeId": {
        "description": "unique trade ID",
        "allOf": [
          {
            "type": "string",
            "example": "100020000000000060"
          }
        ]
      },
      "symbol": {
        "description": "market symbol",
        "allOf": [
          {
            "type": "string",
            "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
            "example": "BTCUSDC"
          }
        ]
      },
      "price": {
        "description": "price",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quantity": {
        "description": "quantity",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "side": {
        "description": "order side",
        "allOf": [
          {
            "type": "string",
            "description": "order side can have the following string values `\"BUY\"`, `\"SELL\"`",
            "example": "BUY"
          }
        ],
        "example": "BUY"
      },
      "isTaker": {
        "description": "denotes whether this is a taker's trade",
        "allOf": [
          {
            "type": "boolean",
            "format": "true or false",
            "example": true
          }
        ]
      },
      "createdAtDatetime": {
        "description": "denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string",
        "allOf": [
          {
            "type": "string",
            "format": "date-time",
            "example": "2025-05-20T01:01:01.000Z",
            "description": "ISO 8601 with millisecond as string"
          }
        ]
      },
      "createdAtTimestamp": {
        "description": "denotes the time the trade was executed by the exchange",
        "allOf": [
          {
            "type": "string",
            "format": "string",
            "example": "1621490985000",
            "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
          }
        ]
      }
    }
  }
}
```

<h3 id="market-data-get-anonymous-trades-by-market-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="market-data-get-anonymous-trades-by-market-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type                                                  | Required | Restrictions | Description                                                                                  |
| -------------------- | ----------------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------- |
| _anonymous_          | [[ObfuscatedTrade](#schemaobfuscatedtrade)]           | false    | none         | none                                                                                         |
| » tradeId            | [TradeID](#schematradeid)                             | true     | none         | unique trade ID                                                                              |
| » symbol             | [MarketSymbol](#schemamarketsymbol)                   | true     | none         | market symbol                                                                                |
| » price              | [AssetValue](#schemaassetvalue)                       | true     | none         | price                                                                                        |
| » quantity           | [AssetValue](#schemaassetvalue)                       | true     | none         | quantity                                                                                     |
| » side               | [OrderSideAsString](#schemaordersideasstring)         | true     | none         | order side                                                                                   |
| » isTaker            | [Boolean](#schemaboolean)(true or false)              | true     | none         | denotes whether this is a taker's trade                                                      |
| » createdAtDatetime  | [DateTime](#schemadatetime)(date-time)                | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string |
| » createdAtTimestamp | [TimeStampAsString](#schematimestampasstring)(string) | true     | none         | denotes the time the trade was executed by the exchange                                      |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-current-tick-by-market-symbol

<a id="opIdmarket-data-current-tick-by-market-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/tick", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/tick', headers = headers)

print(r.json())

```

`GET /v1/markets/{symbol}/tick`

_Get Market Tick_

Get Current Tick by Market Symbol.

- return top 100

**Ratelimited:** `False`

<h3 id="market-data-current-tick-by-market-symbol-parameters">Parameters</h3>

| Name   | In   | Type                                | Required | Description                                          |
| ------ | ---- | ----------------------------------- | -------- | ---------------------------------------------------- |
| symbol | path | [MarketSymbol](#schemamarketsymbol) | true     | symbol to get. Only perpetual markets are supported. |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "createdAtDatetime",
    "createdAtTimestamp",
    "high",
    "low",
    "bestBid",
    "bidVolume",
    "bestAsk",
    "askVolume",
    "vwap",
    "open",
    "close",
    "last",
    "change",
    "percentage",
    "average",
    "baseVolume",
    "quoteVolume",
    "bancorPrice",
    "lastTradeDatetime",
    "lastTradeTimestamp",
    "lastTradeQuantity",
    "ammData"
  ],
  "properties": {
    "createdAtDatetime": {
      "description": "denotes the time of the current tick on the exchange, ISO 8601 with millisecond as string",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    },
    "createdAtTimestamp": {
      "description": "denotes the time of the current tick on the exchange",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "high": {
      "description": "highest price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "low": {
      "description": "lowest price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "bestBid": {
      "description": "current best bid (buy) price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "bidVolume": {
      "description": "current best bid (buy) quantity (may be missing or undefined), see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "bestAsk": {
      "description": "current best ask (sell) price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "askVolume": {
      "description": "current best ask (sell) quantity (may be missing or undefined), see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "vwap": {
      "description": "volume weighed average price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "open": {
      "description": "opening price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "close": {
      "description": "price of last trade (closing price for current period), see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "last": {
      "description": "price of last trade (closing price for current period), see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "change": {
      "description": "absolute change, `last - open`, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "percentage": {
      "description": "relative change, `(change/open) * 100`, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "average": {
      "description": "average price, `(last + open) / 2`, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "baseVolume": {
      "description": "volume of base asset traded for last 24 hours, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteVolume": {
      "description": "volume of quote asset traded for last 24 hours, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "bancorPrice": {
      "description": "current price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "markPrice": {
      "description": "mark price represents the fair value of a contract at the current time.",
      "type": "string",
      "example": "19999.00"
    },
    "fundingRate": {
      "description": "funding rate is used to calculate funding, which measures the relative difference between the index price and mark price.",
      "type": "string",
      "example": "0.01"
    },
    "openInterest": {
      "description": "open interest is the total quantity of open long positions and short positions, see [asset value](#overview--price-and-quantity-precision) format (only applies to derivatives market)",
      "type": "string",
      "example": "100000.32452"
    },
    "lastTradeDatetime": {
      "description": "time of the last trade on this symbol, ISO 8601 with millisecond as string",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    },
    "lastTradeTimestamp": {
      "description": "time of the last trade on this symbol",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "lastTradeQuantity": {
      "description": "quantity of the last trade on this symbol, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "ammData": {
      "description": "AMM data of all available fee tiers.",
      "type": "array",
      "minItems": 0,
      "items": {
        "allOf": [
          {
            "type": "object",
            "description": "AMM data",
            "required": [
              "feeTierId",
              "bidSpreadFee",
              "askSpreadFee",
              "baseReservesQuantity",
              "quoteReservesQuantity",
              "currentPrice"
            ],
            "properties": {
              "feeTierId": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
                    "example": "1"
                  }
                ]
              },
              "bidSpreadFee": {
                "description": "bid spread fee",
                "type": "string",
                "example": "0.00040000"
              },
              "askSpreadFee": {
                "description": "ask spread fee",
                "type": "string",
                "example": "0.00040000"
              },
              "baseReservesQuantity": {
                "description": "base reserves quantity",
                "type": "string",
                "example": "245.56257825"
              },
              "quoteReservesQuantity": {
                "description": "quote reserves quantity",
                "type": "string",
                "example": "3424383.3629"
              },
              "currentPrice": {
                "description": "current AMM price",
                "type": "string",
                "example": "16856.0000"
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="market-data-current-tick-by-market-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema              |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Tick](#schematick) |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None                |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-current-candle-by-market-symbol

<a id="opIdmarket-data-current-candle-by-market-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/candle?createdAtDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z&createdAtDatetime%5Blte%5D=2025-05-20T01%3A01%3A01.000Z&timeBucket=1m",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/markets/{symbol}/candle', params={
  'createdAtDatetime[gte]': '2025-05-20T01:01:01.000Z',  'createdAtDatetime[lte]': '2025-05-20T01:01:01.000Z',  'timeBucket': '1m'
}, headers = headers)

print(r.json())

```

`GET /v1/markets/{symbol}/candle`

_Get Market Candle_

Get Current OHLCV Candle by Market Symbol

- [supports pagination](#overview--pagination-support)
- filtering on `createdAtDatetime`, `createdAtTimestamp` requires additional
  keywords, [see filtering support](#overview--filtering-support)

**Ratelimited:** `False`

<h3 id="market-data-current-candle-by-market-symbol-parameters">Parameters</h3>

| Name                   | In    | Type                                | Required | Description                                                    |
| ---------------------- | ----- | ----------------------------------- | -------- | -------------------------------------------------------------- |
| symbol                 | path  | [MarketSymbol](#schemamarketsymbol) | true     | none                                                           |
| createdAtDatetime[gte] | query | [DateTime](#schemadatetime)         | true     | start timestamp of window, ISO 8601 with millisecond as string |
| createdAtDatetime[lte] | query | [DateTime](#schemadatetime)         | true     | end timestamp of window, ISO 8601 with millisecond as string   |
| timeBucket             | query | [TimeBucket](#schematimebucket)     | true     | time bucket size                                               |

#### Enumerated Values

| Parameter  | Value |
| ---------- | ----- |
| timeBucket | 1m    |
| timeBucket | 5m    |
| timeBucket | 30m   |
| timeBucket | 1h    |
| timeBucket | 6h    |
| timeBucket | 12h   |
| timeBucket | 1d    |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 1,
  "maxItems": 25,
  "items": {
    "type": "object",
    "properties": {
      "open": {
        "description": "see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "high": {
        "description": "see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "low": {
        "description": "see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "close": {
        "description": "see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "volume": {
        "description": "see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "createdAtTimestamp": {
        "allOf": [
          {
            "type": "string",
            "format": "string",
            "example": "1621490985000",
            "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
          }
        ]
      },
      "createdAtDatetime": {
        "description": "date and time of the candle, ISO 8601 with millisecond as string",
        "allOf": [
          {
            "type": "string",
            "format": "date-time",
            "example": "2025-05-20T01:01:01.000Z",
            "description": "ISO 8601 with millisecond as string"
          }
        ]
      },
      "publishedAtTimestamp": {
        "description": "date and time of the candle getting published, ISO 8601 with millisecond as string",
        "allOf": [
          {
            "type": "string",
            "format": "string",
            "example": "1621490985000",
            "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
          }
        ]
      }
    }
  }
}
```

<h3 id="market-data-current-candle-by-market-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="market-data-current-candle-by-market-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name                   | Type                                                  | Required | Restrictions | Description                                                                                       |
| ---------------------- | ----------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| _anonymous_            | [[OHLCVCandle](#schemaohlcvcandle)]                   | false    | none         | none                                                                                              |
| » open                 | [AssetValue](#schemaassetvalue)                       | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| » high                 | [AssetValue](#schemaassetvalue)                       | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| » low                  | [AssetValue](#schemaassetvalue)                       | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| » close                | [AssetValue](#schemaassetvalue)                       | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| » volume               | [AssetValue](#schemaassetvalue)                       | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| » createdAtTimestamp   | [TimeStampAsString](#schematimestampasstring)(string) | false    | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| » createdAtDatetime    | [DateTime](#schemadatetime)(date-time)                | false    | none         | date and time of the candle, ISO 8601 with millisecond as string                                  |
| » publishedAtTimestamp | [TimeStampAsString](#schematimestampasstring)(string) | false    | none         | date and time of the candle getting published, ISO 8601 with millisecond as string                |

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="bullish-trading-api-index-data">index-data</h1>

## get-index-prices

<a id="opIdget-index-prices"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/index-prices", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/index-prices', headers = headers)

print(r.json())

```

`GET /v1/index-prices`

_Get Index Prices_

Retrieves the index price of all supported assets

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": [
      "assetSymbol",
      "price",
      "updatedAtDatetime",
      "updatedAtTimestamp"
    ],
    "properties": {
      "assetSymbol": {
        "description": "Asset symbol",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "price": {
        "description": "Asset price in USD",
        "example": "66100.0000",
        "type": "string"
      },
      "updatedAtDatetime": {
        "description": "Date and time when the index price is updated",
        "allOf": [
          {
            "type": "string",
            "format": "date-time",
            "example": "2025-05-20T01:01:01.000Z",
            "description": "ISO 8601 with millisecond as string"
          }
        ]
      },
      "updatedAtTimestamp": {
        "description": "Timestamp when the index price is updated",
        "allOf": [
          {
            "type": "string",
            "format": "int64",
            "example": "1621490985000",
            "description": "number of milliseconds since EPOCH as string"
          }
        ]
      }
    }
  }
}
```

<h3 id="get-index-prices-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="get-index-prices-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type                                   | Required | Restrictions | Description                                   |
| -------------------- | -------------------------------------- | -------- | ------------ | --------------------------------------------- |
| _anonymous_          | [[IndexPrice](#schemaindexprice)]      | false    | none         | none                                          |
| » assetSymbol        | [AssetSymbol](#schemaassetsymbol)      | true     | none         | Asset symbol                                  |
| » price              | string                                 | true     | none         | Asset price in USD                            |
| » updatedAtDatetime  | [DateTime](#schemadatetime)(date-time) | true     | none         | Date and time when the index price is updated |
| » updatedAtTimestamp | [TimeStamp](#schematimestamp)(int64)   | true     | none         | Timestamp when the index price is updated     |

<aside class="success">
This operation does not require authentication
</aside>

## get-index-price-by-symbol

<a id="opIdget-index-price-by-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/index-prices/{assetSymbol}",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/index-prices/{assetSymbol}', headers = headers)

print(r.json())

```

`GET /v1/index-prices/{assetSymbol}`

_Get Index Price by Asset Symbol_

Retrieves the index price of a specified asset

<h3 id="get-index-price-by-symbol-parameters">Parameters</h3>

| Name        | In   | Type                              | Required | Description |
| ----------- | ---- | --------------------------------- | -------- | ----------- |
| assetSymbol | path | [AssetSymbol](#schemaassetsymbol) | true     | none        |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "assetSymbol",
    "price",
    "updatedAtDatetime",
    "updatedAtTimestamp"
  ],
  "properties": {
    "assetSymbol": {
      "description": "Asset symbol",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "price": {
      "description": "Asset price in USD",
      "example": "66100.0000",
      "type": "string"
    },
    "updatedAtDatetime": {
      "description": "Date and time when the index price is updated",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    },
    "updatedAtTimestamp": {
      "description": "Timestamp when the index price is updated",
      "allOf": [
        {
          "type": "string",
          "format": "int64",
          "example": "1621490985000",
          "description": "number of milliseconds since EPOCH as string"
        }
      ]
    }
  }
}
```

<h3 id="get-index-price-by-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                          |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [IndexPrice](#schemaindexprice) |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Not found             | None                            |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                            |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                            |

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="bullish-trading-api-time">time</h1>

Non-authenticated API for reading time data

## market-data-get-current-exchange-time

<a id="opIdmarket-data-get-current-exchange-time"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch("https://api.exchange.bullish.com/trading-api/v1/time", {
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/time', headers = headers)

print(r.json())

```

`GET /v1/time`

_Get Exchange Time_

Get Current Exchange Time

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": ["timestamp", "datetime"],
  "properties": {
    "timestamp": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "datetime": {
      "description": "ISO 8601 with millisecond as string",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    }
  }
}
```

<h3 id="market-data-get-current-exchange-time-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                                                            |
| ------ | -------------------------------------------------------------------------- | --------------------- | ----------------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [CurrentExchangeTimeResponse](#schemacurrentexchangetimeresponse) |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                                              |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                                              |

<aside class="success">
This operation does not require authentication
</aside>
