---
title: Bullish Trading API - Private REST API
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

<h1 id="bullish-trading-api-orders">orders</h1>

Authenticated APIs for interacting with orders

## trade-get-orders-v2

<a id="opIdtrade-get-orders-v2"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v2/orders?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v2/orders', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v2/orders`

_Get Orders_

Retrieve a list of orders placed by a trading account with specified filters.

- Only the last 24 hours of data is available for querying

This endpoint requires [authentication](#overview--generate-a-jwt-token) and
supports [pagination](#overview--pagination-support). To filter by
`createdAtDatetime` and `createdAtTimestamp`, additional parameters are
required. For detailed instructions, see the
[Filtering Support](#overview--filtering-support) section. Additionally, this
endpoint is subjected to rate limiting.

<h3 id="trade-get-orders-v2-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| symbol           | query  | [MarketSymbol](#schemamarketsymbol)                                             | false    | none                                                                                         |
| clientOrderId    | query  | [OrderHandle](#schemaorderhandle)                                               | false    | Unique numeric (i64) identifier generated on the client side expressed as a string value     |
| side             | query  | [OrderSide](#schemaorderside)                                                   | false    | order side                                                                                   |
| status           | query  | [OrderStatus](#schemaorderstatus)                                               | false    | order status                                                                                 |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

#### Enumerated Values

| Parameter | Value     |
| --------- | --------- |
| side      | BUY       |
| side      | SELL      |
| status    | OPEN      |
| status    | CLOSED    |
| status    | CANCELLED |
| status    | REJECTED  |

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
      "orderId",
      "clientOrderId",
      "symbol",
      "price",
      "stopPrice",
      "averageFillPrice",
      "allowBorrow",
      "quantity",
      "quantityFilled",
      "quoteAmount",
      "baseFee",
      "quoteFee",
      "isLiquidation",
      "side",
      "type",
      "timeInForce",
      "status",
      "statusReason",
      "statusReasonCode",
      "createdAtTimestamp",
      "createdAtDatetime"
    ],
    "properties": {
      "clientOrderId": {
        "allOf": [
          {
            "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
            "type": "string",
            "example": "187"
          }
        ]
      },
      "orderId": {
        "description": "unique order ID",
        "allOf": [
          {
            "type": "string",
            "example": "297735387747975680"
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
        "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "averageFillPrice": {
        "description": "average fill price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "stopPrice": {
        "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "allowBorrow": {
        "description": "indicates if the order was allowed to borrow (does not indicate that borrowing occurred)",
        "type": "boolean",
        "example": false
      },
      "quantity": {
        "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quantityFilled": {
        "description": "quantity filled, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteAmount": {
        "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "baseFee": {
        "description": "base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
        "example": "0.00100000",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteFee": {
        "description": "quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
        "example": "0.0010",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "borrowedBaseQuantity": {
        "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "borrowedQuoteQuantity": {
        "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "isLiquidation": {
        "description": "indicates if the order was executed as a liquidation order",
        "type": "boolean",
        "example": false
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
      "type": {
        "description": "order type",
        "allOf": [
          {
            "type": "string",
            "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
            "example": "LMT"
          }
        ],
        "example": "LMT"
      },
      "timeInForce": {
        "description": "time in force",
        "allOf": [
          {
            "type": "string",
            "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
          }
        ],
        "example": "GTC"
      },
      "status": {
        "description": "order status",
        "allOf": [
          {
            "type": "string",
            "description": "order status can have the following string values `\"OPEN\"`, `\"CLOSED\"`, `\"CANCELLED\"`, `\"REJECTED\"`",
            "example": "OPEN"
          }
        ],
        "example": "OPEN"
      },
      "statusReason": {
        "description": "status reason, describes why the order is in a specific state",
        "type": "string",
        "example": "User cancelled"
      },
      "statusReasonCode": {
        "description": "status reason code, see [details](#overview--error--rejection-codes)",
        "type": "string",
        "example": "1002"
      },
      "createdAtDatetime": {
        "description": "denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string",
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
        "description": "denotes the time the order was ACK'd by the exchange",
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

<h3 id="trade-get-orders-v2-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="trade-get-orders-v2-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type                                                        | Required | Restrictions | Description                                                                                                                 |
| ----------------------- | ----------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_             | [[Order](#schemaorder)]                                     | false    | none         | none                                                                                                                        |
| » clientOrderId         | [OrderHandle](#schemaorderhandle)                           | true     | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value                                    |
| » orderId               | [OrderID](#schemaorderid)                                   | true     | none         | unique order ID                                                                                                             |
| » symbol                | [MarketSymbol](#schemamarketsymbol)                         | true     | none         | market symbol                                                                                                               |
| » price                 | [AssetValue](#schemaassetvalue)                             | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                                    |
| » averageFillPrice      | [AssetValue](#schemaassetvalue)                             | true     | none         | average fill price, see [asset value](#overview--price-and-quantity-precision) format                                       |
| » stopPrice             | [AssetValue](#schemaassetvalue)                             | true     | none         | stop price, see [asset value](#overview--price-and-quantity-precision) format                                               |
| » allowBorrow           | boolean                                                     | true     | none         | indicates if the order was allowed to borrow (does not indicate that borrowing occurred)                                    |
| » quantity              | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                                 |
| » quantityFilled        | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity filled, see [asset value](#overview--price-and-quantity-precision) format                                          |
| » quoteAmount           | [AssetValue](#schemaassetvalue)                             | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format               |
| » baseFee               | [AssetValue](#schemaassetvalue)                             | true     | none         | base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format  |
| » quoteFee              | [AssetValue](#schemaassetvalue)                             | true     | none         | quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format |
| » borrowedBaseQuantity  | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| » borrowedQuoteQuantity | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| » isLiquidation         | boolean                                                     | true     | none         | indicates if the order was executed as a liquidation order                                                                  |
| » side                  | [OrderSideAsString](#schemaordersideasstring)               | true     | none         | order side                                                                                                                  |
| » type                  | [OrderTypeAsString](#schemaordertypeasstring)               | true     | none         | order type                                                                                                                  |
| » timeInForce           | [OrderTimeInForceAsString](#schemaordertimeinforceasstring) | true     | none         | time in force                                                                                                               |
| » status                | [OrderStatusAsString](#schemaorderstatusasstring)           | true     | none         | order status                                                                                                                |
| » statusReason          | string                                                      | true     | none         | status reason, describes why the order is in a specific state                                                               |
| » statusReasonCode      | string                                                      | true     | none         | status reason code, see [details](#overview--error--rejection-codes)                                                        |
| » createdAtDatetime     | [DateTime](#schemadatetime)(date-time)                      | true     | none         | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                   |
| » createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring)(string)       | true     | none         | denotes the time the order was ACK'd by the exchange                                                                        |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## trade-create-order-v2

<a id="opIdtrade-create-order-v2"></a>

> Code samples

```javascript
const inputBody = '{
  "commandType": "V3CreateOrder",
  "clientOrderId": "1234",
  "symbol": "BTCUSDC",
  "type": "LIMIT",
  "side": "BUY",
  "price": "31000.1",
  "quantity": "1.1",
  "timeInForce": "GTC",
  "allowBorrow": true,
  "tradingAccountId": "111000000000001"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':{
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE':{
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP':{
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE':undefined,
  'BX-NONCE-WINDOW-ENABLED':{
  "description": "string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests)\n",
  "type": "string",
  "enum": [
    "false",
    "true"
  ],
  "default": "false"
}
};

fetch('https://api.exchange.bullish.com/trading-api/v2/orders',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE': {
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP': {
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE': undefined,
  'BX-NONCE-WINDOW-ENABLED': {
  "description": "string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests)\n",
  "type": "string",
  "enum": [
    "false",
    "true"
  ],
  "default": "false"
}
}

r = requests.post('https://api.exchange.bullish.com/trading-api/v2/orders', headers = headers)

print(r.json())

```

`POST /v2/orders`

_Create Order_

Creates an order, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header.

This endpoint uses the [signing format](#overview--signing-format) which does
not require strict field ordering and addition of null fields in the request
body. Quantities and prices does not require strict precision. Eg. for asset
precision of 4 - `100`, `100.0`, `100.00`, `100.000` and `100.0000` are all
accepted.

**Ratelimited:** `True`. Higher tiers of rate limits available by providing the
`BX-RATELIMIT-TOKEN` request header.

> Body parameter

```json
{
  "commandType": "V3CreateOrder",
  "clientOrderId": "1234",
  "symbol": "BTCUSDC",
  "type": "LIMIT",
  "side": "BUY",
  "price": "31000.1",
  "quantity": "1.1",
  "timeInForce": "GTC",
  "allowBorrow": true,
  "tradingAccountId": "111000000000001"
}
```

<h3 id="trade-create-order-v2-parameters">Parameters</h3>

| Name                    | In     | Type                                                                                                | Required | Description                                                                                                                                                         |
| ----------------------- | ------ | --------------------------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Authorization           | header | [#/components/headers/Authorization](#schema#/components/headers/authorization)                     | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                                                        |
| BX-SIGNATURE            | header | [#/components/headers/BX-SIGNATURE](#schema#/components/headers/bx-signature)                       | true     | signature obtained using the [signing format](#overview--signing-format)                                                                                            |
| BX-TIMESTAMP            | header | [#/components/headers/BX-TIMESTAMP](#schema#/components/headers/bx-timestamp)                       | true     | timestamp is the number of milliseconds since EPOCH                                                                                                                 |
| BX-NONCE                | header | undefined                                                                                           | true     | nonce is a client side incremented unsigned 64 bit integer                                                                                                          |
| BX-NONCE-WINDOW-ENABLED | header | [#/components/headers/BX-NONCE-WINDOW-ENABLED](#schema#/components/headers/bx-nonce-window-enabled) | false    | string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests) |
| body                    | body   | [CreateOrderCommandV3](#schemacreateordercommandv3)                                                 | true     | new order request body                                                                                                                                              |

#### Detailed descriptions

**BX-NONCE-WINDOW-ENABLED**: string representation of a boolean value,
[enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests)

#### Enumerated Values

| Parameter               | Value |
| ----------------------- | ----- |
| BX-NONCE-WINDOW-ENABLED | false |
| BX-NONCE-WINDOW-ENABLED | true  |

> Example responses

> 200 Response

```json
{
  "message": "Command acknowledged - CreateOrder",
  "requestId": "633910976353665024",
  "orderId": "633910775316480001",
  "clientOrderId": "1234567",
  "x-widdershins-oldRef": "#/components/schemas/CreateOrderCommandResponseV3/example"
}
```

<h3 id="trade-create-order-v2-responses">Responses</h3>

| Status | Meaning                                                          | Description                                                                                                                                                                                                                                                                                                                                                                                                                            | Schema                                                              |
| ------ | ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)          | Status OK. The create order command was successfully acknowledged. To check the current status of the order, query [Get Order by ID](#get-/trading-api/v2/orders/-orderId-) using the `orderId` received in the response payload. Please consult the section [How To Ensure The Order Of _Create Order_ or _Cancel Order_ Requests](#overview--how-to-ensure-the-order-of-create-order-or-cancel-order-requests) for more information. | [CreateOrderCommandResponseV3](#schemacreateordercommandresponsev3) |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1) | Bad Request                                                                                                                                                                                                                                                                                                                                                                                                                            |

For example, sending a request with the `BX-SIGNATURE` header missing will
result in the following
response:|[BadOrderEntryResponse](#schemabadorderentryresponse)|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|Not
Authenticated|None|
|403|[Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)|Access
Forbidden|None|
|429|[Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)|Too Many
Requests|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|Internal
Server Error|None|

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## trade-get-order-by-id-v2

<a id="opIdtrade-get-order-by-id-v2"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v2/orders/{orderId}?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v2/orders/{orderId}', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v2/orders/{orderId}`

_Get Order by ID_

Retrieve a specific order using its unique identifier.

This endpoint requires [authentication](#overview--generate-a-jwt-token) and is
subjected to rate limiting.

<h3 id="trade-get-order-by-id-v2-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |
| orderId          | path   | number                                                                          | true     | order ID                                                                                     |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "orderId",
    "clientOrderId",
    "symbol",
    "price",
    "stopPrice",
    "averageFillPrice",
    "allowBorrow",
    "quantity",
    "quantityFilled",
    "quoteAmount",
    "baseFee",
    "quoteFee",
    "isLiquidation",
    "side",
    "type",
    "timeInForce",
    "status",
    "statusReason",
    "statusReasonCode",
    "createdAtTimestamp",
    "createdAtDatetime"
  ],
  "properties": {
    "clientOrderId": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
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
      "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "averageFillPrice": {
      "description": "average fill price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "stopPrice": {
      "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "allowBorrow": {
      "description": "indicates if the order was allowed to borrow (does not indicate that borrowing occurred)",
      "type": "boolean",
      "example": false
    },
    "quantity": {
      "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantityFilled": {
      "description": "quantity filled, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteAmount": {
      "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "baseFee": {
      "description": "base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
      "example": "0.00100000",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteFee": {
      "description": "quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
      "example": "0.0010",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedBaseQuantity": {
      "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedQuoteQuantity": {
      "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "isLiquidation": {
      "description": "indicates if the order was executed as a liquidation order",
      "type": "boolean",
      "example": false
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
    "type": {
      "description": "order type",
      "allOf": [
        {
          "type": "string",
          "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
          "example": "LMT"
        }
      ],
      "example": "LMT"
    },
    "timeInForce": {
      "description": "time in force",
      "allOf": [
        {
          "type": "string",
          "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
        }
      ],
      "example": "GTC"
    },
    "status": {
      "description": "order status",
      "allOf": [
        {
          "type": "string",
          "description": "order status can have the following string values `\"OPEN\"`, `\"CLOSED\"`, `\"CANCELLED\"`, `\"REJECTED\"`",
          "example": "OPEN"
        }
      ],
      "example": "OPEN"
    },
    "statusReason": {
      "description": "status reason, describes why the order is in a specific state",
      "type": "string",
      "example": "User cancelled"
    },
    "statusReasonCode": {
      "description": "status reason code, see [details](#overview--error--rejection-codes)",
      "type": "string",
      "example": "1002"
    },
    "createdAtDatetime": {
      "description": "denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the order was ACK'd by the exchange",
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
```

<h3 id="trade-get-order-by-id-v2-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                |
| ------ | -------------------------------------------------------------------------- | --------------------- | --------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Order](#schemaorder) |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                  |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                  |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                  |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## command-entry

<a id="opIdcommand-entry"></a>

> Code samples

```javascript
const inputBody = '{
  "commandType": "V1AmendOrder",
  "orderId": "297735387747975680",
  "symbol": "BTCUSDC",
  "type": "LIMIT",
  "price": "1.00000000",
  "clientOrderId": "633914459442118656",
  "quantity": "1.00000000",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/AmendOrderCommandV1/example"
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':{
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE':{
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP':{
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE':undefined,
  'BX-NONCE-WINDOW-ENABLED':{
  "description": "string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests)\n",
  "type": "string",
  "enum": [
    "false",
    "true"
  ],
  "default": "false"
}
};

fetch('https://api.exchange.bullish.com/trading-api/v2/command#amend',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE': {
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP': {
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE': undefined,
  'BX-NONCE-WINDOW-ENABLED': {
  "description": "string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests)\n",
  "type": "string",
  "enum": [
    "false",
    "true"
  ],
  "default": "false"
}
}

r = requests.post('https://api.exchange.bullish.com/trading-api/v2/command#amend', headers = headers)

print(r.json())

```

`POST /v2/command#amend`

_Order Amendment Command_

Ability to amend the `price` ,`quantity` and `type` (i.e. change from Taker Only
to Maker only and vice-versa) on Limit orders. It can be applied only to open
orders (`quantityFilled=0` and `status=OPEN`).

This submits a command to the trading engine to amend an order. A successful
response indicates that the command entry was acknowledged but does not indicate
that the command was executed.

This endpoint uses the [signing format](#overview--signing-format) which does
not require strict field ordering and addition of null fields in the request
body. Quantities and prices does not require strict precision. Eg. for asset
precision of 4 - `100`, `100.0`, `100.00`, `100.000` and `100.0000` are all
accepted.

Requires

- [bearer token](#overview--add-authenticated-request-header) in authorization
  header

**Ratelimited:** `True`. Higher tiers of rate limits available by providing the
`BX-RATELIMIT-TOKEN` request header.

> Body parameter

```json
{
  "commandType": "V1AmendOrder",
  "orderId": "297735387747975680",
  "symbol": "BTCUSDC",
  "type": "LIMIT",
  "price": "1.00000000",
  "clientOrderId": "633914459442118656",
  "quantity": "1.00000000",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/AmendOrderCommandV1/example"
}
```

<h3 id="command-entry-parameters">Parameters</h3>

| Name                    | In     | Type                                                                                                | Required | Description                                                                                                                                                          |
| ----------------------- | ------ | --------------------------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Authorization           | header | [#/components/headers/Authorization](#schema#/components/headers/authorization)                     | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                                                         |
| BX-SIGNATURE            | header | [#/components/headers/BX-SIGNATURE](#schema#/components/headers/bx-signature)                       | true     | signature obtained using the [signing format](#overview--signing-format)                                                                                             |
| BX-TIMESTAMP            | header | [#/components/headers/BX-TIMESTAMP](#schema#/components/headers/bx-timestamp)                       | true     | timestamp is the number of milliseconds since EPOCH                                                                                                                  |
| BX-NONCE                | header | undefined                                                                                           | true     | nonce is a client side incremented unsigned 64 bit integer                                                                                                           |
| BX-NONCE-WINDOW-ENABLED | header | [#/components/headers/BX-NONCE-WINDOW-ENABLED](#schema#/components/headers/bx-nonce-window-enabled) | false    | string representation of a boolean value, [enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests), |
| body                    | body   | any                                                                                                 | true     | none                                                                                                                                                                 |

#### Detailed descriptions

**BX-NONCE-WINDOW-ENABLED**: string representation of a boolean value,
[enables out-of-order order requests to be processed](#overview--how-to-enable-out-of-order-processing-of-order-requests),
only valid for `V3CancelOrder`

#### Enumerated Values

| Parameter               | Value |
| ----------------------- | ----- |
| BX-NONCE-WINDOW-ENABLED | false |
| BX-NONCE-WINDOW-ENABLED | true  |

> Example responses

> Status OK. This means a command was successfully acknowledged.

```json
{
  "message": "Command acknowledged - AmendOrder",
  "requestId": "633910976353665024",
  "orderId": "633910775316480001",
  "clientOrderId": "1234567-1"
}
```

> 400 Response

```json
{
  "type": "object",
  "required": ["message", "errorCode", "errorCodeName"],
  "properties": {
    "message": {
      "description": "message",
      "type": "string",
      "example": "Missing signature header"
    },
    "errorCode": {
      "description": "unique error code",
      "type": "integer",
      "example": 6029
    },
    "errorCodeName": {
      "description": "unique error code name",
      "type": "string",
      "example": "MISSING_SIGNATURE_HEADER"
    }
  }
}
```

<h3 id="command-entry-responses">Responses</h3>

| Status | Meaning                                                          | Description                                                    | Schema |
| ------ | ---------------------------------------------------------------- | -------------------------------------------------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)          | Status OK. This means a command was successfully acknowledged. | Inline |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1) | Bad Request                                                    |

For example, sending a request with the `BX-SIGNATURE` header missing will
result in the following
response:|[BadOrderEntryResponse](#schemabadorderentryresponse)|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|Not
Authenticated|None|
|403|[Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)|Access
Forbidden|None|
|429|[Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)|Too Many
Requests|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|Internal
Server Error|None|

<h3 id="command-entry-responseschema">Response Schema</h3>

Status Code **200**

| Name            | Type                          | Required | Restrictions | Description                                             |
| --------------- | ----------------------------- | -------- | ------------ | ------------------------------------------------------- |
| » message       | string                        | true     | none         | message                                                 |
| » requestId     | [RequestID](#schemarequestid) | true     | none         | unique request ID                                       |
| » orderId       | [OrderID](#schemaorderid)     | true     | none         | unique order ID                                         |
| » clientOrderId | string                        | false    | none         | Will be updated as part of a successful order amendment |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-custody">custody</h1>

Authenticated APIs for custody,
[Custody Basic Examples](https://github.com/bullish-exchange/api-examples/blob/master/bullish/rest/custody_basics.py)

Custody APIs have a limit of 40 requests per IP, per minute. This is combined
across all endpoints of type **/wallets/\***

## custody-get-history

<a id="opIdcustody-get-history"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch("https://api.exchange.bullish.com/trading-api/v1/wallets/transactions", {
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/wallets/transactions', headers = headers)

print(r.json())

```

`GET /v1/wallets/transactions`

_Get Custody Transaction History_

Get custody transaction history, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

- [supports pagination](#overview--pagination-support)

**Ratelimited:** `True` - see [custody limits](#tag--custody)

<h3 id="custody-get-history-parameters">Parameters</h3>

| Name                   | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization          | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| createdAtDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | false    | start timestamp of period, ISO 8601 with millisecond as string                               |
| createdAtDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | false    | end timestamp of period, ISO 8601 with millisecond as string                                 |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "custodyTransactionId": {
        "allOf": [
          {
            "type": "string",
            "example": "DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979",
            "description": "unique identifier for tracking a deposit or withdrawal"
          }
        ]
      },
      "direction": {
        "allOf": [
          {
            "type": "string",
            "example": "DEPOSIT",
            "description": "direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL'"
          }
        ]
      },
      "quantity": {
        "allOf": [
          {
            "type": "string",
            "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
            "example": "100000.00"
          }
        ]
      },
      "symbol": {
        "allOf": [
          {
            "type": "string",
            "example": "USDC",
            "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
          }
        ]
      },
      "network": {
        "allOf": [
          {
            "type": "string",
            "example": "EOS",
            "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
          }
        ]
      },
      "fee": {
        "allOf": [
          {
            "type": "string",
            "example": "3.00",
            "description": "withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
          }
        ]
      },
      "memo": {
        "allOf": [
          {
            "type": "string",
            "example": "925891241",
            "description": "memo or destination tag used during deposit to help identify account to credit funds to"
          }
        ]
      },
      "createdAtDateTime": {
        "allOf": [
          {
            "type": "string",
            "example": "2022-09-16T07:56:15.000Z",
            "description": "time of initial transaction"
          }
        ]
      },
      "status": {
        "allOf": [
          {
            "type": "string",
            "example": "COMPLETE",
            "description": "one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED'"
          }
        ]
      },
      "transactionDetails": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "address": {
                "type": "string",
                "description": "crypto network address",
                "example": "0xb0a64d976972d87b0783eeb1ff88306cd1891f02"
              },
              "blockchainTxId": {
                "type": "string",
                "description": "transaction id on chain",
                "example": "0xec557f2c7278d2dae2d98a27b9bd43f386789a4209090cbbd11595f1bed4a4c2"
              },
              "swiftUetr": {
                "type": "string",
                "description": "unique end-to-end-transaction reference for swift transactions",
                "example": "b55aa5cd-baa2-4122-8c17-ae9b856ae36a"
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="custody-get-history-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="custody-get-history-responseschema">Response Schema</h3>

Status Code **200**

| Name                   | Type                                                              | Required | Restrictions | Description                                                                                                                                                                           |
| ---------------------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_            | [[CustodyHistory](#schemacustodyhistory)]                         | false    | none         | none                                                                                                                                                                                  |
| » custodyTransactionId | [CustodyTransactionHistoryID](#schemacustodytransactionhistoryid) | false    | none         | unique identifier for tracking a deposit or withdrawal                                                                                                                                |
| » direction            | [CustodyDirection](#schemacustodydirection)                       | false    | none         | direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL'                                                                                                       |
| » quantity             | [CustodyQuantity](#schemacustodyquantity)                         | false    | none         | total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted. |
| » symbol               | [CustodySymbol](#schemacustodysymbol)                             | false    | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                                                                                          |
| » network              | [NetworkID](#schemanetworkid)                                     | false    | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                                                                                                                           |
| » fee                  | [CustodyWithdrawalFee](#schemacustodywithdrawalfee)               | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)                                                                           |
| » memo                 | [CustodyDepositMemo](#schemacustodydepositmemo)                   | false    | none         | memo or destination tag used during deposit to help identify account to credit funds to                                                                                               |
| » createdAtDateTime    | [CustodyCreatedAtDateTime](#schemacustodycreatedatdatetime)       | false    | none         | time of initial transaction                                                                                                                                                           |
| » status               | [CustodyTransactionStatus](#schemacustodytransactionstatus)       | false    | none         | one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED'                                                                                                                                   |
| » transactionDetails   | [CustodyTransactionDetails](#schemacustodytransactiondetails)     | false    | none         | none                                                                                                                                                                                  |
| »» address             | string                                                            | false    | none         | crypto network address                                                                                                                                                                |
| »» blockchainTxId      | string                                                            | false    | none         | transaction id on chain                                                                                                                                                               |
| »» swiftUetr           | string                                                            | false    | none         | unique end-to-end-transaction reference for swift transactions                                                                                                                        |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## custody-get-limits

<a id="opIdcustody-get-limits"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/wallets/limits/{symbol}",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/wallets/limits/{symbol}', headers = headers)

print(r.json())

```

`GET /v1/wallets/limits/{symbol}`

_Get Withdrawal Limits for Symbol_

Get withdrawal limits for symbol, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

Please note that Custody endpoints utilize a non-multiplied asset format for
long decimal assets like SHIB and PEPE, ensuring consistency with real-world
asset representation. This differs from Trading endpoints, which use a
multiplied asset format, such as SHIB1M and PEPE1M. For more information, please
see
[help centre](https://bullishexchange.atlassian.net/wiki/spaces/BHC/pages/20807684/Understanding+Multiplied+Assets+PEPE1M+and+SHIB1M)

**Ratelimited:** `True` - see [custody limits](#tag--custody)

<h3 id="custody-get-limits-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| symbol        | path   | [CustodySymbol](#schemacustodysymbol)                                           | true     | none                                                                                         |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "available": {
      "allOf": [
        {
          "type": "string",
          "example": "20000.0",
          "description": "remaining limit on amount of coin or token that could be withdrawn now, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    },
    "twentyFourHour": {
      "allOf": [
        {
          "type": "string",
          "example": "1000000.00",
          "description": "limit on amount of coin or token that can be withdrawn over a 24 hour period, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    }
  }
}
```

<h3 id="custody-get-limits-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                                |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [CustodyLimits](#schemacustodylimits) |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                  |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## custody-get-deposit-instructions

<a id="opIdcustody-get-deposit-instructions"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/wallets/deposit-instructions/fiat/{symbol}",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/wallets/deposit-instructions/fiat/{symbol}', headers = headers)

print(r.json())

```

`GET /v1/wallets/deposit-instructions/fiat/{symbol}`

_Get Deposit Instructions for Fiat_

Get deposit instructions, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

**Ratelimited:** `True` - see [custody limits](#tag--custody)

<h3 id="custody-get-deposit-instructions-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| symbol        | path   | [InstrumentId](#schemainstrumentid)                                             | true     | none                                                                                         |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "network": {
        "type": "string",
        "example": "SWIFT",
        "description": "the network that the account belongs to and the transaction will be performed on SWIFT, ABA or SEPA"
      },
      "symbol": {
        "type": "string",
        "example": "USD",
        "description": "the currency associated with the account, e.g. USD, EUR"
      },
      "accountNumber": {
        "allOf": [
          {
            "type": "string",
            "description": "bank account number",
            "example": "9873481227"
          }
        ],
        "example": "5090022533",
        "description": "the Bullish account number, varies for SWIFT/ABA and SEPA"
      },
      "name": {
        "type": "string",
        "example": "Bullish (GI) Limited",
        "description": "official Bullish account holder name"
      },
      "physicalAddress": {
        "type": "string",
        "example": "26/F, The Centrium, 60 Wyndham Street, Central, Hong Kong",
        "description": "bullish entity's physical address for the bank account"
      },
      "memo": {
        "type": "string",
        "example": "8VZPKSGPA",
        "description": "client specific reference to identify which account desposits should be allocated to on the exhange"
      },
      "bank": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "name": {
                "allOf": [
                  {
                    "type": "string",
                    "example": "Silvergate Bank",
                    "description": "name of bank"
                  }
                ]
              },
              "physicalAddress": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "physical location of bank",
                    "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                  }
                ]
              },
              "routingCode": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "routing code of bank",
                    "example": "322286803"
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="custody-get-deposit-instructions-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="custody-get-deposit-instructions-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type                                                                      | Required | Restrictions | Description                                                                                         |
| ------------------ | ------------------------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| _anonymous_        | [[CustodyFiatDepositInstructions](#schemacustodyfiatdepositinstructions)] | false    | none         | none                                                                                                |
| » network          | string                                                                    | false    | none         | the network that the account belongs to and the transaction will be performed on SWIFT, ABA or SEPA |
| » symbol           | string                                                                    | false    | none         | the currency associated with the account, e.g. USD, EUR                                             |
| » accountNumber    | [CustodyBankAccountNumber](#schemacustodybankaccountnumber)               | false    | none         | the Bullish account number, varies for SWIFT/ABA and SEPA                                           |
| » name             | string                                                                    | false    | none         | official Bullish account holder name                                                                |
| » physicalAddress  | string                                                                    | false    | none         | bullish entity's physical address for the bank account                                              |
| » memo             | string                                                                    | false    | none         | client specific reference to identify which account desposits should be allocated to on the exhange |
| » bank             | [CustodyBankDetails](#schemacustodybankdetails)                           | false    | none         | none                                                                                                |
| »» name            | [CustodyBankName](#schemacustodybankname)                                 | false    | none         | name of bank                                                                                        |
| »» physicalAddress | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress)           | false    | none         | physical location of bank                                                                           |
| »» routingCode     | [CustodyBankRoutingCode](#schemacustodybankroutingcode)                   | false    | none         | routing code of bank                                                                                |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## custody-get-withdrawal-instructions

<a id="opIdcustody-get-withdrawal-instructions"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/wallets/withdrawal-instructions/fiat/{symbol}",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/wallets/withdrawal-instructions/fiat/{symbol}', headers = headers)

print(r.json())

```

`GET /v1/wallets/withdrawal-instructions/fiat/{symbol}`

_Get Withdrawal Instructions for Fiat_

Get withdrawal instructions added by the user, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header. Please note that before withdrawal destinations can be used for
withdrawing to, they must be whitelisted on the Bullish website.

**Ratelimited:** `True` - see [custody limits](#tag--custody)

<h3 id="custody-get-withdrawal-instructions-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| symbol        | path   | [CustodyFiatSymbol](#schemacustodyfiatsymbol)                                   | true     | none                                                                                         |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "destinationId": {
        "allOf": [
          {
            "type": "string",
            "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
            "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
          }
        ]
      },
      "accountNumber": {
        "allOf": [
          {
            "type": "string",
            "description": "bank account number",
            "example": "9873481227"
          }
        ]
      },
      "network": {
        "allOf": [
          {
            "type": "string",
            "description": "the fiat network, e.g. SWIFT, ABA or SEPA",
            "example": "SWIFT"
          }
        ]
      },
      "symbol": {
        "allOf": [
          {
            "type": "string",
            "example": "USD",
            "description": "symbol representing fiat currency, e.g. USD, EUR"
          }
        ]
      },
      "name": {
        "allOf": [
          {
            "type": "string",
            "example": "Silvergate Bank",
            "description": "name of bank"
          }
        ]
      },
      "physicalAddress": {
        "allOf": [
          {
            "type": "string",
            "description": "physical location of bank",
            "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
          }
        ]
      },
      "fee": {
        "allOf": [
          {
            "type": "string",
            "example": "3.00",
            "description": "withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
          }
        ]
      },
      "memo": {
        "allOf": [
          {
            "type": "string",
            "example": "MZAXEMRXA",
            "description": "memo or destination tag that will be used as a reference on transaction"
          }
        ]
      },
      "bank": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "name": {
                "allOf": [
                  {
                    "type": "string",
                    "example": "Silvergate Bank",
                    "description": "name of bank"
                  }
                ]
              },
              "physicalAddress": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "physical location of bank",
                    "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                  }
                ]
              },
              "routingCode": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "routing code of bank",
                    "example": "322286803"
                  }
                ]
              }
            }
          }
        ]
      },
      "intermediaryBank": {
        "allOf": [
          {
            "type": "object",
            "properties": {
              "name": {
                "example": "Middle Bank",
                "allOf": [
                  {
                    "type": "string",
                    "example": "Silvergate Bank",
                    "description": "name of bank"
                  }
                ]
              },
              "physicalAddress": {
                "example": "523 Exchange Square, Canary Wharf, E14 2WA",
                "allOf": [
                  {
                    "type": "string",
                    "description": "physical location of bank",
                    "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                  }
                ]
              },
              "routingCode": {
                "example": "321176234",
                "allOf": [
                  {
                    "type": "string",
                    "description": "routing code of bank",
                    "example": "322286803"
                  }
                ]
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="custody-get-withdrawal-instructions-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="custody-get-withdrawal-instructions-responseschema">Response Schema</h3>

Status Code **200**

| Name               | Type                                                                            | Required | Restrictions | Description                                                                                                 |
| ------------------ | ------------------------------------------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| _anonymous_        | [[CustodyFiatWithdrawalInstructions](#schemacustodyfiatwithdrawalinstructions)] | false    | none         | none                                                                                                        |
| » destinationId    | [CustodyDestinationID](#schemacustodydestinationid)                             | false    | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account                |
| » accountNumber    | [CustodyBankAccountNumber](#schemacustodybankaccountnumber)                     | false    | none         | bank account number                                                                                         |
| » network          | [CustodyBankNetworkID](#schemacustodybanknetworkid)                             | false    | none         | the fiat network, e.g. SWIFT, ABA or SEPA                                                                   |
| » symbol           | [CustodyFiatSymbol](#schemacustodyfiatsymbol)                                   | false    | none         | symbol representing fiat currency, e.g. USD, EUR                                                            |
| » name             | [CustodyBankName](#schemacustodybankname)                                       | false    | none         | name of bank                                                                                                |
| » physicalAddress  | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress)                 | false    | none         | physical location of bank                                                                                   |
| » fee              | [CustodyWithdrawalFee](#schemacustodywithdrawalfee)                             | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |
| » memo             | [CustodyWithdrawalMemo](#schemacustodywithdrawalmemo)                           | false    | none         | memo or destination tag that will be used as a reference on transaction                                     |
| » bank             | [CustodyBankDetails](#schemacustodybankdetails)                                 | false    | none         | none                                                                                                        |
| »» name            | [CustodyBankName](#schemacustodybankname)                                       | false    | none         | name of bank                                                                                                |
| »» physicalAddress | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress)                 | false    | none         | physical location of bank                                                                                   |
| »» routingCode     | [CustodyBankRoutingCode](#schemacustodybankroutingcode)                         | false    | none         | routing code of bank                                                                                        |
| » intermediaryBank | [CustodyBankIntermediateDetails](#schemacustodybankintermediatedetails)         | false    | none         | none                                                                                                        |
| »» name            | [CustodyBankName](#schemacustodybankname)                                       | false    | none         | name of bank                                                                                                |
| »» physicalAddress | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress)                 | false    | none         | physical location of bank                                                                                   |
| »» routingCode     | [CustodyBankRoutingCode](#schemacustodybankroutingcode)                         | false    | none         | routing code of bank                                                                                        |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## custody-withdrawal

<a id="opIdcustody-withdrawal"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "required": [
    "timestamp",
    "nonce",
    "authorizer",
    "command"
  ],
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
    "nonce": {
      "type": "string",
      "description": "a UUID withdrawal nonce to protect against replay attacks",
      "example": "1628376611"
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "withdrawal command",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "destinationId",
            "symbol",
            "network",
            "quantity"
          ],
          "properties": {
            "commandType": {
              "description": "the command type, it must be 'V1Withdrawal'",
              "type": "string",
              "example": "V1Withdrawal"
            },
            "destinationId": {
              "allOf": [
                {
                  "type": "string",
                  "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
                  "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
                }
              ]
            },
            "symbol": {
              "example": "USDC",
              "allOf": [
                {
                  "type": "string",
                  "example": "USDC",
                  "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
                }
              ]
            },
            "network": {
              "example": "ETH",
              "allOf": [
                {
                  "type": "string",
                  "example": "EOS",
                  "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
                }
              ]
            },
            "quantity": {
              "example": "100000.000001",
              "allOf": [
                {
                  "type": "string",
                  "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
                  "example": "100000.00"
                }
              ]
            }
          }
        }
      ]
    }
  }
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':{
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE':{
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
};

fetch('https://api.exchange.bullish.com/trading-api/v1/wallets/withdrawal',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE': {
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.post('https://api.exchange.bullish.com/trading-api/v1/wallets/withdrawal', headers = headers)

print(r.json())

```

`POST /v1/wallets/withdrawal`

_Create Withdrawal_

Trigger a withdrawal, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

The `BX-SIGNATURE` header should be created by signing the request with an ECDSA
API Key as follows:

1. Construct a string that concatenates the following fields:
   - `timestamp` - current epoch milliseconds e.g. 1697008474031
   - `nonce` - a UUID identifier to protect against replay attacks e.g.
     255241a1-2cde-4954-87b1-13beef547960
   - `request method` - e.g. POST
   - `request path` - e.g. /trading-api/v1/wallets/withdrawal
   - `request body JSON string`, removing any spaces and newline characters
2. Hash the string using a SHA-256 hash function and sign the resulting
   hexdigest with your `<PRIVATE_KEY>`.
3. DER encode the signature, and BASE64 encode the DER encoded signature.

> **Bullish requires you to whitelist a withdrawal destination address before
> submitting a withdrawal request. You may view, approve, and manage your list
> of destination addresses in Account Settings on the Bullish website. If you
> attempt a withdrawal without first whitelisting an address in Account
> Settings, then the withdrawal attempt will fail.**

For a full example of using the withdrawal endpoint please see the
[Custody Withdrawal Example](https://github.com/bullish-exchange/api-examples/blob/master/ecdsa/rest/custody_withdrawal.py)

Please note that Custody endpoints utilize a non-multiplied asset format for
long decimal assets like SHIB and PEPE, ensuring consistency with real-world
asset representation. This differs from Trading endpoints, which use a
multiplied asset format, such as SHIB1M and PEPE1M. For more information, please
see
[help centre](https://bullishexchange.atlassian.net/wiki/spaces/BHC/pages/20807684/Understanding+Multiplied+Assets+PEPE1M+and+SHIB1M)

The currently supported precisions for withdrawal quantities are as follows.
Please note that fees are always specified in units of the symbol itself, not in
smaller denominations (e.g. BTC not Satoshi, ETH not Wei) :

| Symbol | Precision | Remarks                                                                                                                                                            |
| ------ | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| USD    | 2dp       |                                                                                                                                                                    |
| BTC    | 8dp       |                                                                                                                                                                    |
| DOGE   | 8dp       |                                                                                                                                                                    |
| EOS    | 4dp       |                                                                                                                                                                    |
| ETH    | 8dp       |                                                                                                                                                                    |
| LTC    | 8dp       |                                                                                                                                                                    |
| XRP    | 6dp       |                                                                                                                                                                    |
| AAVE   | 8dp       |                                                                                                                                                                    |
| CRV    | 8dp       |                                                                                                                                                                    |
| LINK   | 8dp       |                                                                                                                                                                    |
| MANA   | 8dp       |                                                                                                                                                                    |
| MATIC  | 8dp       |                                                                                                                                                                    |
| SUSHI  | 8dp       |                                                                                                                                                                    |
| UNI    | 8dp       |                                                                                                                                                                    |
| USDC   | 6dp       |                                                                                                                                                                    |
| USDT   | 6dp       |                                                                                                                                                                    |
| SHIB   | 2dp       | Please ensure to use the non-multiplied asset format (e.g., SHIB, PEPE) when creating withdrawals, as Custody endpoints align with real-world asset representation |
| PEPE   | 2dp       | Please ensure to use the non-multiplied asset format (e.g., SHIB, PEPE) when creating withdrawals, as Custody endpoints align with real-world asset representation |

**Ratelimited:** `True` - see [custody limits](#tag--custody)

> Body parameter

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "type": "string",
      "description": "a UUID withdrawal nonce to protect against replay attacks",
      "example": "1628376611"
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "withdrawal command",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "destinationId",
            "symbol",
            "network",
            "quantity"
          ],
          "properties": {
            "commandType": {
              "description": "the command type, it must be 'V1Withdrawal'",
              "type": "string",
              "example": "V1Withdrawal"
            },
            "destinationId": {
              "allOf": [
                {
                  "type": "string",
                  "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
                  "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
                }
              ]
            },
            "symbol": {
              "example": "USDC",
              "allOf": [
                {
                  "type": "string",
                  "example": "USDC",
                  "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
                }
              ]
            },
            "network": {
              "example": "ETH",
              "allOf": [
                {
                  "type": "string",
                  "example": "EOS",
                  "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
                }
              ]
            },
            "quantity": {
              "example": "100000.000001",
              "allOf": [
                {
                  "type": "string",
                  "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
                  "example": "100000.00"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

<h3 id="custody-withdrawal-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| BX-SIGNATURE  | header | [#/components/headers/BX-SIGNATURE](#schema#/components/headers/bx-signature)   | true     | ECDSA signature created by signing the request with an ECDSA Key                             |
| body          | body   | [CustodyApiEcdsaWithdrawalRequest](#schemacustodyapiecdsawithdrawalrequest)     | true     | withdrawal request                                                                           |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "properties": {
    "statusReason": {
      "description": "status reason, describes why withdrawal challenge is in a specific state",
      "type": "string",
      "example": "Withdrawal accepted"
    },
    "statusReasonCode": {
      "description": "status reason code, see [details](#overview--error--rejection-codes)",
      "type": "integer",
      "example": 1001
    },
    "custodyTransactionId": {
      "allOf": [
        {
          "type": "string",
          "example": "DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979",
          "description": "unique identifier for tracking a withdrawal during signing and in history"
        }
      ]
    }
  }
}
```

<h3 id="custody-withdrawal-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                                                              |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------------------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [CustodyApiWithdrawalResponse](#schemacustodyapiwithdrawalresponse) |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                                                |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                                                |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-trades">trades</h1>

Authenticated APIs for reading trade data

## trade-get-trades

<a id="opIdtrade-get-trades"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/trades?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/trades', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/trades`

_Get Trades_

Get a list of trades based on specified filters.

- requires [bearer token](#overview--add-authenticated-request-header) in
  authorization header
- Only the last 24 hours of data is available for querying
- [supports pagination](#overview--pagination-support)
- filtering on `createdAtDatetime`, `createdAtTimestamp` requires additional
  keywords, [see filtering support](#overview--filtering-support)

**Ratelimited:** `True`

<h3 id="trade-get-trades-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| symbol           | query  | [MarketSymbol](#schemamarketsymbol)                                             | false    | none                                                                                         |
| orderId          | query  | [OrderID](#schemaorderid)                                                       | false    | unique order ID                                                                              |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

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
      "tradeId",
      "orderId",
      "symbol",
      "price",
      "quantity",
      "quoteAmount",
      "baseFee",
      "quoteFee",
      "side",
      "isTaker",
      "tradeRebateAmount",
      "tradeRebateAssetSymbol",
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
      "orderId": {
        "description": "unique order ID",
        "allOf": [
          {
            "type": "string",
            "example": "297735387747975680"
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
        "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quantity": {
        "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteAmount": {
        "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "baseFee": {
        "description": "base fee, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteFee": {
        "description": "quote fee, see [asset value](#overview--price-and-quantity-precision) format",
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
      "tradeRebateAmount": {
        "description": "amount of rebate that is credited to the user as part of the trade.",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "tradeRebateAssetSymbol": {
        "description": "the symbol of the asset in which the rebate is paid",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "USDC"
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

<h3 id="trade-get-trades-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="trade-get-trades-responseschema">Response Schema</h3>

Status Code **200**

| Name                     | Type                                                  | Required | Restrictions | Description                                                                                                   |
| ------------------------ | ----------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------- |
| _anonymous_              | [[Trade](#schematrade)]                               | false    | none         | none                                                                                                          |
| » tradeId                | [TradeID](#schematradeid)                             | true     | none         | unique trade ID                                                                                               |
| » orderId                | [OrderID](#schemaorderid)                             | true     | none         | unique order ID                                                                                               |
| » symbol                 | [MarketSymbol](#schemamarketsymbol)                   | true     | none         | market symbol                                                                                                 |
| » price                  | [AssetValue](#schemaassetvalue)                       | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                      |
| » quantity               | [AssetValue](#schemaassetvalue)                       | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                   |
| » quoteAmount            | [AssetValue](#schemaassetvalue)                       | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format |
| » baseFee                | [AssetValue](#schemaassetvalue)                       | true     | none         | base fee, see [asset value](#overview--price-and-quantity-precision) format                                   |
| » quoteFee               | [AssetValue](#schemaassetvalue)                       | true     | none         | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                  |
| » side                   | [OrderSideAsString](#schemaordersideasstring)         | true     | none         | order side                                                                                                    |
| » isTaker                | [Boolean](#schemaboolean)(true or false)              | true     | none         | denotes whether this is a taker's trade                                                                       |
| » tradeRebateAmount      | [AssetValue](#schemaassetvalue)                       | true     | none         | amount of rebate that is credited to the user as part of the trade.                                           |
| » tradeRebateAssetSymbol | [QuoteAssetSymbol](#schemaquoteassetsymbol)           | true     | none         | the symbol of the asset in which the rebate is paid                                                           |
| » createdAtDatetime      | [DateTime](#schemadatetime)(date-time)                | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string                  |
| » createdAtTimestamp     | [TimeStampAsString](#schematimestampasstring)(string) | true     | none         | denotes the time the trade was executed by the exchange                                                       |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## trade-get-trade-by-id

<a id="opIdtrade-get-trade-by-id"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/trades/{tradeId}?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/trades/{tradeId}', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/trades/{tradeId}`

_Get Trade by ID_

Gets a trade by ID, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

**Ratelimited:** `True`

<h3 id="trade-get-trade-by-id-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| tradeId          | path   | number                                                                          | true     | trade ID                                                                                     |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "tradeId",
    "orderId",
    "symbol",
    "price",
    "quantity",
    "quoteAmount",
    "baseFee",
    "quoteFee",
    "side",
    "isTaker",
    "tradeRebateAmount",
    "tradeRebateAssetSymbol",
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
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
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
      "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantity": {
      "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteAmount": {
      "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "baseFee": {
      "description": "base fee, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteFee": {
      "description": "quote fee, see [asset value](#overview--price-and-quantity-precision) format",
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
    "tradeRebateAmount": {
      "description": "amount of rebate that is credited to the user as part of the trade.",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "tradeRebateAssetSymbol": {
      "description": "the symbol of the asset in which the rebate is paid",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "USDC"
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
```

<h3 id="trade-get-trade-by-id-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                |
| ------ | -------------------------------------------------------------------------- | --------------------- | --------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [Trade](#schematrade) |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                  |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                  |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None                  |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                  |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-accounts">accounts</h1>

Authenticated APIs for reading account data

## user-get-asset-accounts

<a id="opIduser-get-asset-accounts"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/accounts/asset?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/accounts/asset', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/accounts/asset`

_Get Asset Accounts_

Gets the asset accounts, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

**Ratelimited:** `True`

<h3 id="user-get-asset-accounts-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

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
      "tradingAccountId",
      "assetId",
      "assetSymbol",
      "availableQuantity",
      "borrowedQuantity",
      "lockedQuantity",
      "loanedQuantity",
      "updatedAtDatetime",
      "updatedAtTimestamp"
    ],
    "properties": {
      "tradingAccountId": {
        "allOf": [
          {
            "description": "unique trading account ID",
            "type": "string",
            "example": "111000000000001"
          }
        ]
      },
      "assetId": {
        "description": "asset ID",
        "allOf": [
          {
            "type": "string",
            "description": "unique asset ID",
            "example": "1"
          }
        ]
      },
      "assetSymbol": {
        "description": "asset symbol",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "availableQuantity": {
        "description": "the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "borrowedQuantity": {
        "description": "the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "lockedQuantity": {
        "description": "the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "loanedQuantity": {
        "description": "the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "updatedAtDatetime": {
        "description": "denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string",
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
        "description": "denotes the time the AMM instruction was updated by the exchange",
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

<h3 id="user-get-asset-accounts-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="user-get-asset-accounts-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type                                                  | Required | Restrictions | Description                                                                                                                                        |
| -------------------- | ----------------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_          | [[AssetAccount](#schemaassetaccount)]                 | false    | none         | none                                                                                                                                               |
| » tradingAccountId   | [TradingAccountId](#schematradingaccountid)           | true     | none         | unique trading account ID                                                                                                                          |
| » assetId            | [AssetID](#schemaassetid)                             | true     | none         | asset ID                                                                                                                                           |
| » assetSymbol        | [AssetSymbol](#schemaassetsymbol)                     | true     | none         | asset symbol                                                                                                                                       |
| » availableQuantity  | [AssetValue](#schemaassetvalue)                       | true     | none         | the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format                             |
| » borrowedQuantity   | [AssetValue](#schemaassetvalue)                       | true     | none         | the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format                                     |
| » lockedQuantity     | [AssetValue](#schemaassetvalue)                       | true     | none         | the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format |
| » loanedQuantity     | [AssetValue](#schemaassetvalue)                       | true     | none         | the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format                                 |
| » updatedAtDatetime  | [DateTime](#schemadatetime)(date-time)                | true     | none         | denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string                                              |
| » updatedAtTimestamp | [TimeStampAsString](#schematimestampasstring)(string) | true     | none         | denotes the time the AMM instruction was updated by the exchange                                                                                   |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## user-get-asset-account-by-symbol

<a id="opIduser-get-asset-account-by-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/accounts/asset/{symbol}?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/accounts/asset/{symbol}', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/accounts/asset/{symbol}`

_Get Asset Account by Symbol_

Gets the asset account by symbol, requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header

**Ratelimited:** `True`

<h3 id="user-get-asset-account-by-symbol-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| symbol           | path   | [AssetSymbol](#schemaassetsymbol)                                               | true     | none                                                                                         |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "tradingAccountId",
    "assetId",
    "assetSymbol",
    "availableQuantity",
    "borrowedQuantity",
    "lockedQuantity",
    "loanedQuantity",
    "updatedAtDatetime",
    "updatedAtTimestamp"
  ],
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "assetId": {
      "description": "asset ID",
      "allOf": [
        {
          "type": "string",
          "description": "unique asset ID",
          "example": "1"
        }
      ]
    },
    "assetSymbol": {
      "description": "asset symbol",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "availableQuantity": {
      "description": "the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedQuantity": {
      "description": "the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "lockedQuantity": {
      "description": "the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "loanedQuantity": {
      "description": "the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "updatedAtDatetime": {
      "description": "denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the AMM instruction was updated by the exchange",
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
```

<h3 id="user-get-asset-account-by-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                              |
| ------ | -------------------------------------------------------------------------- | --------------------- | ----------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [AssetAccount](#schemaassetaccount) |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                                |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                                |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None                                |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-sessions">sessions</h1>

## logout

<a id="opIdlogout"></a>

> Code samples

```javascript
const headers = {
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch("https://api.exchange.bullish.com/trading-api/v1/users/logout", {
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
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/users/logout', headers = headers)

print(r.json())

```

`GET /v1/users/logout`

_Logout_

Logout of the session associated with the JWT. It requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header.

**Ratelimited:** `True`

<h3 id="logout-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |

<h3 id="logout-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | None   |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## login

<a id="opIdlogin"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  "BX-SIGNATURE": {
    description:
      "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
    schema: {
      type: "string"
    },
    type: "string"
  },
  "BX-TIMESTAMP": {
    description: "command timestamp is the number of milliseconds since EPOCH",
    schema: {
      type: "string"
    },
    type: "string"
  },
  "BX-NONCE": undefined,
  "BX-PUBLIC-KEY": {
    description: "Public key being used to generate the JWT",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch("https://api.exchange.bullish.com/trading-api/v1/users/hmac/login", {
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
  'Accept': 'application/json',
  'BX-SIGNATURE': {
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP': {
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE': undefined,
  'BX-PUBLIC-KEY': {
  "description": "Public key being used to generate the JWT",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/users/hmac/login', headers = headers)

print(r.json())

```

`GET /v1/users/hmac/login`

_HMAC Login_

Login and generate a new session associated with a JWT using HMAC. Once you log
in from an IP, the same IP must be used for the duration of the session for any
subsequent requests.

**Ratelimited:** `True`

<h3 id="login-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                                                          |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| BX-SIGNATURE  | header | [#/components/headers/BX-SIGNATURE](#schema#/components/headers/bx-signature)   | true     | HMAC signature is a signature resulted from the [signed message of the `login` request](#overview--add-authenticated-request-header) |
| BX-TIMESTAMP  | header | [#/components/headers/BX-TIMESTAMP](#schema#/components/headers/bx-timestamp)   | true     | timestamp is the number of milliseconds since EPOCH                                                                                  |
| BX-NONCE      | header | undefined                                                                       | true     | nonce is a client side incremented 64-bit unsigned integer                                                                           |
| BX-PUBLIC-KEY | header | [#/components/headers/BX-PUBLIC-KEY](#schema#/components/headers/bx-public-key) | true     | public key for the HMAC Key being used to generate the JWT                                                                           |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": ["authorizer", "token"],
  "properties": {
    "authorizer": {
      "description": "Authorizer",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "token": {
      "description": "JWT token",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoic2FuZGVlcCByYWtocmEifQ.wyVq6PlKaldWXtu-jz2hJCvkGl1lM2S7HUKCH8LnXp0",
          "description": "JWT token"
        }
      ]
    }
  }
}
```

<h3 id="login-responses">Responses</h3>

| Status | Meaning                                                                    | Description                                           | Schema                                |
| ------ | -------------------------------------------------------------------------- | ----------------------------------------------------- | ------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | returns JWT and the `authorizer` for signing requests | [LoginResponse](#schemaloginresponse) |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)           | Bad Request                                           | None                                  |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated                                     | None                                  |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden                                      | None                                  |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests                                     | None                                  |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error                                 | None                                  |

<aside class="success">
This operation does not require authentication
</aside>

<h1 id="bullish-trading-api-trading-accounts">trading-accounts</h1>

## user-get-trading-accounts

<a id="opIduser-get-trading-accounts"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/accounts/trading-accounts",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/accounts/trading-accounts', headers = headers)

print(r.json())

```

`GET /v1/accounts/trading-accounts`

_Get all trading Accounts details_

Gets details for all trading accounts accessible by the API key used in the
request. It requires [bearer token](#overview--add-authenticated-request-header)
in authorization header. The trading account's id will be used in all other REST
API

**Ratelimited:** `True`

<h3 id="user-get-trading-accounts-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                  |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 25,
  "items": {
    "type": "object",
    "required": [
      "isBorrowing",
      "isLending",
      "isPrimaryAccount",
      "maxInitialLeverage",
      "rateLimitToken",
      "tradingAccountDescription",
      "tradingAccountId",
      "tradingAccountName",
      "isDefaulted",
      "riskLimitUSD",
      "totalBorrowedUSD",
      "totalCollateralUSD",
      "initialMarginUSD",
      "warningMarginUSD",
      "liquidationMarginUSD",
      "fullLiquidationMarginUSD",
      "defaultedMarginUSD",
      "endCustomerId",
      "isConcentrationRiskEnabled",
      "liquidityAddonUSD",
      "marketRiskUSD",
      "marginProfile",
      "totalLiabilitiesUSD",
      "tradeFeeRate"
    ],
    "properties": {
      "isBorrowing": {
        "description": "whether the trading account is borrowing",
        "type": "string",
        "example": "false"
      },
      "isLending": {
        "description": "whether the trading account is lending",
        "type": "string",
        "example": "false"
      },
      "makerFee": {
        "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
        "type": "string",
        "example": "0.00000000",
        "deprecated": true
      },
      "takerFee": {
        "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
        "type": "string",
        "example": "0.00020000",
        "deprecated": true
      },
      "maxInitialLeverage": {
        "description": "max initial leverage",
        "type": "string",
        "example": "1"
      },
      "tradingAccountId": {
        "description": "id of the trading account",
        "allOf": [
          {
            "description": "unique trading account ID",
            "type": "string",
            "example": "111000000000001"
          }
        ]
      },
      "tradingAccountName": {
        "description": "name of the trading account",
        "type": "string",
        "example": "algo trading account"
      },
      "tradingAccountDescription": {
        "description": "description of the trading account",
        "type": "string",
        "example": "algo trading account with experimental strategy"
      },
      "isPrimaryAccount": {
        "description": "whether this is the primary account",
        "type": "string",
        "example": "false"
      },
      "rateLimitToken": {
        "description": "unique rate limit token of the trading account",
        "type": "string",
        "example": "97d98951b12fb11f330dd9cb1b807d888c702679ee602edcf1ebc6bac17ad63d"
      },
      "isDefaulted": {
        "description": "whether the trading account is defaulted",
        "type": "String",
        "example": "false"
      },
      "tradeFeeRate": {
        "description": "Trade fees per `feeGroupId` for this trading account",
        "type": "array",
        "minItems": 0,
        "items": {
          "allOf": [
            {
              "type": "object",
              "required": ["feeGroupId", "makerFee", "takerFee"],
              "properties": {
                "feeGroupId": {
                  "type": "integer",
                  "description": "Identifier for this particular fee tier",
                  "example": 1
                },
                "makerFee": {
                  "type": "string",
                  "description": "Maker Fee in basis points (bps)",
                  "example": "10"
                },
                "takerFee": {
                  "type": "string",
                  "description": "Taker Fee in basis points (bps)",
                  "example": "10"
                }
              }
            }
          ]
        }
      },
      "riskLimitUSD": {
        "description": "The maximum allowed borrowing for this trading account (in USD currency)",
        "type": "string",
        "example": "10000.0000"
      },
      "totalLiabilitiesUSD": {
        "description": "The The total liabilities for this trading account (in USD currency)",
        "type": "string",
        "example": "14000.0000"
      },
      "totalBorrowedUSD": {
        "description": "total borrowed across all assets in this trading account displayed in the reference asset in USD",
        "type": "string",
        "example": "12000.0000"
      },
      "totalCollateralUSD": {
        "description": "total collateral across all assets in this trading account displayed in the reference asset in USD",
        "type": "string",
        "example": "13000.0000"
      },
      "initialMarginUSD": {
        "description": "The minimum margin one must maintain in order to be able to purposefully increase risk",
        "type": "string",
        "example": "0000.0000"
      },
      "warningMarginUSD": {
        "description": "The minimum margin when the customer will receive warning via email/notifications over UI",
        "type": "string",
        "example": "0000.0000"
      },
      "liquidationMarginUSD": {
        "description": "The minimum value of margin one must maintain in order to avoid liquidation",
        "type": "string",
        "example": "0000.0000"
      },
      "fullLiquidationMarginUSD": {
        "description": "The value of margin when full liquidation occurs",
        "type": "string",
        "example": "0000.0000"
      },
      "defaultedMarginUSD": {
        "description": "The value of margin when this trading account will be moved into a Defaulted state",
        "type": "string",
        "example": "0000.0000"
      },
      "endCustomerId": {
        "description": "The end customer id used for self trade prevention (default is institution id, max 32 characters)",
        "type": "string",
        "example": "PrimeBroker"
      },
      "isConcentrationRiskEnabled": {
        "description": "whether concentration risk checks are enforced for an account when sending new orders. By default, concentration risk checks will apply to all accounts",
        "type": "string",
        "example": true
      },
      "liquidityAddonUSD": {
        "description": "expected market impact of unwinding the portfolio in the case of a liquidation event",
        "type": "string",
        "example": "1000.0000"
      },
      "marketRiskUSD": {
        "description": "the worst possible loss on the portfolio based on scenario analysis",
        "type": "string",
        "example": "2000.0000"
      },
      "marginProfile": {
        "description": "Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values",
        "allOf": [
          {
            "properties": {
              "initialMarketRiskMultiplierPct": {
                "description": "market risk multiplier used to calculate initial margin requirement of the account",
                "type": "string",
                "example": "200.00"
              },
              "warningMarketRiskMultiplierPct": {
                "description": "market risk multiplier used to calculate warning margin requirement of the account",
                "type": "string",
                "example": "150.00"
              },
              "liquidationMarketRiskMultiplierPct": {
                "description": "market risk multiplier used to calculate liquidation margin requirement of the account",
                "type": "string",
                "example": "100.00"
              },
              "fullLiquidationMarketRiskMultiplierPct": {
                "description": "market risk multiplier used to calculate full liquidation margin requirement of the account",
                "type": "string",
                "example": "75.00"
              },
              "defaultedMarketRiskMultiplierPct": {
                "description": "market risk multiplier used to calculate defaulted margin requirement of the account",
                "type": "string",
                "example": "50.00"
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="user-get-trading-accounts-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="user-get-trading-accounts-responseschema">Response Schema</h3>

Status Code **200**

| Name                                      | Type                                                      | Required | Restrictions | Description                                                                                                                                             |
| ----------------------------------------- | --------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_                               | [[TradingAccountResponse](#schematradingaccountresponse)] | false    | none         | none                                                                                                                                                    |
| » isBorrowing                             | string                                                    | true     | none         | whether the trading account is borrowing                                                                                                                |
| » isLending                               | string                                                    | true     | none         | whether the trading account is lending                                                                                                                  |
| » makerFee                                | string                                                    | false    | none         | Deprecated and no longer accurate. See `tradeFeeRate` instead                                                                                           |
| » takerFee                                | string                                                    | false    | none         | Deprecated and no longer accurate. See `tradeFeeRate` instead                                                                                           |
| » maxInitialLeverage                      | string                                                    | true     | none         | max initial leverage                                                                                                                                    |
| » tradingAccountId                        | [TradingAccountId](#schematradingaccountid)               | true     | none         | id of the trading account                                                                                                                               |
| » tradingAccountName                      | string                                                    | true     | none         | name of the trading account                                                                                                                             |
| » tradingAccountDescription               | string                                                    | true     | none         | description of the trading account                                                                                                                      |
| » isPrimaryAccount                        | string                                                    | true     | none         | whether this is the primary account                                                                                                                     |
| » rateLimitToken                          | string                                                    | true     | none         | unique rate limit token of the trading account                                                                                                          |
| » isDefaulted                             | String                                                    | true     | none         | whether the trading account is defaulted                                                                                                                |
| » tradeFeeRate                            | [allOf]                                                   | true     | none         | Trade fees per `feeGroupId` for this trading account                                                                                                    |
| »» feeGroupId                             | integer                                                   | true     | none         | Identifier for this particular fee tier                                                                                                                 |
| »» makerFee                               | string                                                    | true     | none         | Maker Fee in basis points (bps)                                                                                                                         |
| »» takerFee                               | string                                                    | true     | none         | Taker Fee in basis points (bps)                                                                                                                         |
| » riskLimitUSD                            | string                                                    | true     | none         | The maximum allowed borrowing for this trading account (in USD currency)                                                                                |
| » totalLiabilitiesUSD                     | string                                                    | true     | none         | The The total liabilities for this trading account (in USD currency)                                                                                    |
| » totalBorrowedUSD                        | string                                                    | true     | none         | total borrowed across all assets in this trading account displayed in the reference asset in USD                                                        |
| » totalCollateralUSD                      | string                                                    | true     | none         | total collateral across all assets in this trading account displayed in the reference asset in USD                                                      |
| » initialMarginUSD                        | string                                                    | true     | none         | The minimum margin one must maintain in order to be able to purposefully increase risk                                                                  |
| » warningMarginUSD                        | string                                                    | true     | none         | The minimum margin when the customer will receive warning via email/notifications over UI                                                               |
| » liquidationMarginUSD                    | string                                                    | true     | none         | The minimum value of margin one must maintain in order to avoid liquidation                                                                             |
| » fullLiquidationMarginUSD                | string                                                    | true     | none         | The value of margin when full liquidation occurs                                                                                                        |
| » defaultedMarginUSD                      | string                                                    | true     | none         | The value of margin when this trading account will be moved into a Defaulted state                                                                      |
| » endCustomerId                           | string                                                    | true     | none         | The end customer id used for self trade prevention (default is institution id, max 32 characters)                                                       |
| » isConcentrationRiskEnabled              | string                                                    | true     | none         | whether concentration risk checks are enforced for an account when sending new orders. By default, concentration risk checks will apply to all accounts |
| » liquidityAddonUSD                       | string                                                    | true     | none         | expected market impact of unwinding the portfolio in the case of a liquidation event                                                                    |
| » marketRiskUSD                           | string                                                    | true     | none         | the worst possible loss on the portfolio based on scenario analysis                                                                                     |
| » marginProfile                           | object                                                    | true     | none         | Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values                               |
| »» initialMarketRiskMultiplierPct         | string                                                    | false    | none         | market risk multiplier used to calculate initial margin requirement of the account                                                                      |
| »» warningMarketRiskMultiplierPct         | string                                                    | false    | none         | market risk multiplier used to calculate warning margin requirement of the account                                                                      |
| »» liquidationMarketRiskMultiplierPct     | string                                                    | false    | none         | market risk multiplier used to calculate liquidation margin requirement of the account                                                                  |
| »» fullLiquidationMarketRiskMultiplierPct | string                                                    | false    | none         | market risk multiplier used to calculate full liquidation margin requirement of the account                                                             |
| »» defaultedMarketRiskMultiplierPct       | string                                                    | false    | none         | market risk multiplier used to calculate defaulted margin requirement of the account                                                                    |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## user-get-trading-account-by-id

<a id="opIduser-get-trading-account-by-id"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/accounts/trading-accounts/{tradingAccountId}?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/accounts/trading-accounts/{tradingAccountId}', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/accounts/trading-accounts/{tradingAccountId}`

_Get trading account details by trading account id_

Gets details for specific trading account by `tradingAccountId` and API key used
in the request. It requires
[bearer token](#overview--add-authenticated-request-header) in authorization
header.

**Ratelimited:** `True`

<h3 id="user-get-trading-account-by-id-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |

> Example responses

> 200 Response

```json
{
  "type": "object",
  "required": [
    "isBorrowing",
    "isLending",
    "isPrimaryAccount",
    "maxInitialLeverage",
    "rateLimitToken",
    "tradingAccountDescription",
    "tradingAccountId",
    "tradingAccountName",
    "isDefaulted",
    "riskLimitUSD",
    "totalBorrowedUSD",
    "totalCollateralUSD",
    "initialMarginUSD",
    "warningMarginUSD",
    "liquidationMarginUSD",
    "fullLiquidationMarginUSD",
    "defaultedMarginUSD",
    "endCustomerId",
    "isConcentrationRiskEnabled",
    "liquidityAddonUSD",
    "marketRiskUSD",
    "marginProfile",
    "totalLiabilitiesUSD",
    "tradeFeeRate"
  ],
  "properties": {
    "isBorrowing": {
      "description": "whether the trading account is borrowing",
      "type": "string",
      "example": "false"
    },
    "isLending": {
      "description": "whether the trading account is lending",
      "type": "string",
      "example": "false"
    },
    "makerFee": {
      "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
      "type": "string",
      "example": "0.00000000",
      "deprecated": true
    },
    "takerFee": {
      "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
      "type": "string",
      "example": "0.00020000",
      "deprecated": true
    },
    "maxInitialLeverage": {
      "description": "max initial leverage",
      "type": "string",
      "example": "1"
    },
    "tradingAccountId": {
      "description": "id of the trading account",
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "tradingAccountName": {
      "description": "name of the trading account",
      "type": "string",
      "example": "algo trading account"
    },
    "tradingAccountDescription": {
      "description": "description of the trading account",
      "type": "string",
      "example": "algo trading account with experimental strategy"
    },
    "isPrimaryAccount": {
      "description": "whether this is the primary account",
      "type": "string",
      "example": "false"
    },
    "rateLimitToken": {
      "description": "unique rate limit token of the trading account",
      "type": "string",
      "example": "97d98951b12fb11f330dd9cb1b807d888c702679ee602edcf1ebc6bac17ad63d"
    },
    "isDefaulted": {
      "description": "whether the trading account is defaulted",
      "type": "String",
      "example": "false"
    },
    "tradeFeeRate": {
      "description": "Trade fees per `feeGroupId` for this trading account",
      "type": "array",
      "minItems": 0,
      "items": {
        "allOf": [
          {
            "type": "object",
            "required": ["feeGroupId", "makerFee", "takerFee"],
            "properties": {
              "feeGroupId": {
                "type": "integer",
                "description": "Identifier for this particular fee tier",
                "example": 1
              },
              "makerFee": {
                "type": "string",
                "description": "Maker Fee in basis points (bps)",
                "example": "10"
              },
              "takerFee": {
                "type": "string",
                "description": "Taker Fee in basis points (bps)",
                "example": "10"
              }
            }
          }
        ]
      }
    },
    "riskLimitUSD": {
      "description": "The maximum allowed borrowing for this trading account (in USD currency)",
      "type": "string",
      "example": "10000.0000"
    },
    "totalLiabilitiesUSD": {
      "description": "The The total liabilities for this trading account (in USD currency)",
      "type": "string",
      "example": "14000.0000"
    },
    "totalBorrowedUSD": {
      "description": "total borrowed across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "12000.0000"
    },
    "totalCollateralUSD": {
      "description": "total collateral across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "13000.0000"
    },
    "initialMarginUSD": {
      "description": "The minimum margin one must maintain in order to be able to purposefully increase risk",
      "type": "string",
      "example": "0000.0000"
    },
    "warningMarginUSD": {
      "description": "The minimum margin when the customer will receive warning via email/notifications over UI",
      "type": "string",
      "example": "0000.0000"
    },
    "liquidationMarginUSD": {
      "description": "The minimum value of margin one must maintain in order to avoid liquidation",
      "type": "string",
      "example": "0000.0000"
    },
    "fullLiquidationMarginUSD": {
      "description": "The value of margin when full liquidation occurs",
      "type": "string",
      "example": "0000.0000"
    },
    "defaultedMarginUSD": {
      "description": "The value of margin when this trading account will be moved into a Defaulted state",
      "type": "string",
      "example": "0000.0000"
    },
    "endCustomerId": {
      "description": "The end customer id used for self trade prevention (default is institution id, max 32 characters)",
      "type": "string",
      "example": "PrimeBroker"
    },
    "isConcentrationRiskEnabled": {
      "description": "whether concentration risk checks are enforced for an account when sending new orders. By default, concentration risk checks will apply to all accounts",
      "type": "string",
      "example": true
    },
    "liquidityAddonUSD": {
      "description": "expected market impact of unwinding the portfolio in the case of a liquidation event",
      "type": "string",
      "example": "1000.0000"
    },
    "marketRiskUSD": {
      "description": "the worst possible loss on the portfolio based on scenario analysis",
      "type": "string",
      "example": "2000.0000"
    },
    "marginProfile": {
      "description": "Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values",
      "allOf": [
        {
          "properties": {
            "initialMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate initial margin requirement of the account",
              "type": "string",
              "example": "200.00"
            },
            "warningMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate warning margin requirement of the account",
              "type": "string",
              "example": "150.00"
            },
            "liquidationMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate liquidation margin requirement of the account",
              "type": "string",
              "example": "100.00"
            },
            "fullLiquidationMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate full liquidation margin requirement of the account",
              "type": "string",
              "example": "75.00"
            },
            "defaultedMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate defaulted margin requirement of the account",
              "type": "string",
              "example": "50.00"
            }
          }
        }
      ]
    }
  }
}
```

<h3 id="user-get-trading-account-by-id-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                                                  |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [TradingAccountResponse](#schematradingaccountresponse) |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)           | Bad Request Parameter | None                                                    |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                                                    |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                                                    |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None                                                    |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                                    |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                                    |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## command

<a id="opIdcommand"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "required": [
    "timestamp",
    "nonce",
    "authorizer",
    "command"
  ],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "assetSymbol",
            "quantity",
            "fromTradingAccountId",
            "toTradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, e.g. 'V1TransferAsset'",
              "type": "string",
              "example": "V1TransferAsset"
            },
            "assetSymbol": {
              "description": "Symbol of the asset. i.e. currency",
              "allOf": [
                {
                  "type": "string",
                  "description": "asset symbol as denoted in the world",
                  "example": "BTC"
                }
              ]
            },
            "quantity": {
              "description": "Quantity of the asset.",
              "type": "string",
              "example": "100"
            },
            "fromTradingAccountId": {
              "description": "Source of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            },
            "toTradingAccountId": {
              "description": "Destination of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':{
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE':{
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP':{
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE':undefined
};

fetch('https://api.exchange.bullish.com/trading-api/v1/command?commandType=V1TransferAsset',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-SIGNATURE': {
  "description": "command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header)",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-TIMESTAMP': {
  "description": "command timestamp is the number of milliseconds since EPOCH",
  "schema": {
    "type": "string"
  },
  "type": "string"
},
  'BX-NONCE': undefined
}

r = requests.post('https://api.exchange.bullish.com/trading-api/v1/command?commandType=V1TransferAsset', headers = headers)

print(r.json())

```

`POST /v1/command?commandType=V1TransferAsset`

_Transfer Asset_

Send command to transfer asset between two trading accounts.

> Body parameter

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "assetSymbol",
            "quantity",
            "fromTradingAccountId",
            "toTradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, e.g. 'V1TransferAsset'",
              "type": "string",
              "example": "V1TransferAsset"
            },
            "assetSymbol": {
              "description": "Symbol of the asset. i.e. currency",
              "allOf": [
                {
                  "type": "string",
                  "description": "asset symbol as denoted in the world",
                  "example": "BTC"
                }
              ]
            },
            "quantity": {
              "description": "Quantity of the asset.",
              "type": "string",
              "example": "100"
            },
            "fromTradingAccountId": {
              "description": "Source of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            },
            "toTradingAccountId": {
              "description": "Destination of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

<h3 id="command-parameters">Parameters</h3>

| Name          | In     | Type                                                                            | Required | Description                                                                                                                          |
| ------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Authorization | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                         |
| BX-SIGNATURE  | header | [#/components/headers/BX-SIGNATURE](#schema#/components/headers/bx-signature)   | true     | command signature is a signature resulted from the [signed message of the `<COMMAND>`](#overview--construct-the-bx-signature-header) |
| BX-TIMESTAMP  | header | [#/components/headers/BX-TIMESTAMP](#schema#/components/headers/bx-timestamp)   | true     | command timestamp is the number of milliseconds since EPOCH                                                                          |
| BX-NONCE      | header | undefined                                                                       | true     | command nonce is a client side incremented unsigned 64 bit integer                                                                   |
| body          | body   | [TradingAccountTransferRequest](#schematradingaccounttransferrequest)           | true     | Command for action                                                                                                                   |

> Example responses

> 200 Response

```json
{
  "message": "Command acknowledged - TransferAsset",
  "requestId": "633909659774222336"
}
```

<h3 id="command-responses">Responses</h3>

| Status | Meaning                                                          | Description                                                                                                                          | Schema                                                                  |
| ------ | ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)          | Status OK. This means the request was successfully acknowledged. It does not necessarily mean the command was successfully executed. | [TradingAccountTransferResponse](#schematradingaccounttransferresponse) |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1) | Bad Request                                                                                                                          |

For example, sending a request with the `BX-SIGNATURE` header missing will
result in the following
response:|[BadOrderEntryResponse](#schemabadorderentryresponse)|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|Not
Authenticated|None|
|403|[Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)|Access
Forbidden|None|
|429|[Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)|Too Many
Requests|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|Internal
Server Error|None|

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-derivatives">derivatives</h1>

## get-derivatives-positions

<a id="opIdget-derivatives-positions"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch("https://api.exchange.bullish.com/trading-api/v1/derivatives-positions", {
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/derivatives-positions', headers = headers)

print(r.json())

```

`GET /v1/derivatives-positions`

_Get derivatives positions_

Get derivatives positions

<h3 id="get-derivatives-positions-parameters">Parameters</h3>

| Name             | In     | Type                                                                            | Required | Description                                                                                                                                                                                                  |
| ---------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Authorization    | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                                                                                                 |
| tradingAccountId | query  | [TradingAccountId](#schematradingaccountid)                                     | false    | Id of the trading account. `tradingAccountId` is mandatory in the query for users with multiple trading accounts. For users with a single trading account, it can be automatically retrieved from the login. |
| symbol           | query  | [MarketSymbol](#schemamarketsymbol)                                             | false    | none                                                                                                                                                                                                         |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "description": "Derivatives Position of one market for the trading account",
    "type": "string",
    "properties": {
      "tradingAccountId": {
        "allOf": [
          {
            "description": "unique trading account ID",
            "type": "string",
            "example": "111000000000001"
          }
        ]
      },
      "symbol": {
        "example": "BTC-USDC-PERP",
        "allOf": [
          {
            "type": "string",
            "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
            "example": "BTCUSDC"
          }
        ]
      },
      "side": {
        "allOf": [
          {
            "type": "string",
            "example": "BUY",
            "enum": ["BUY", "SELL"]
          }
        ]
      },
      "quantity": {
        "description": "Current size of the  position [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "notional": {
        "description": "Notional value of the current position, calculated using the mark price",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "entryNotional": {
        "description": "Notional value of the position, using the average entry price",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "mtmPnl": {
        "description": "Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "reportedMtmPnl": {
        "description": "The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "reportedFundingPnl": {
        "description": "Sum of all funding payments received  since the position was opened. This is updated every time funding is paid.",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "realizedPnl": {
        "description": "Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "settlementAssetSymbol": {
        "description": "Settlement Asset Symbol",
        "type": "string",
        "example": "USDC"
      },
      "createdAtDatetime": {
        "description": "Denotes the time the position was created by the exchange, ISO 8601 with millisecond as string",
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
        "description": "Denotes the time the position was created by the exchange, number of milliseconds since EPOCH",
        "allOf": [
          {
            "type": "string",
            "format": "string",
            "example": "1621490985000",
            "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
          }
        ]
      },
      "updatedAtDatetime": {
        "description": "Denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string",
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
        "description": "Denotes the time the position was updated by the exchange, number of milliseconds since EPOCH",
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

<h3 id="get-derivatives-positions-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="get-derivatives-positions-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type                                                                | Required | Restrictions | Description                                                                                                                                                                                               |
| ----------------------- | ------------------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_             | [[DerivativesPositionResponse](#schemaderivativespositionresponse)] | false    | none         | [Derivatives Position of one market for the trading account]                                                                                                                                              |
| » tradingAccountId      | [TradingAccountId](#schematradingaccountid)                         | false    | none         | unique trading account ID                                                                                                                                                                                 |
| » symbol                | [MarketSymbol](#schemamarketsymbol)                                 | false    | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market                                                                                                                             |
| » side                  | [OrderSide](#schemaorderside)                                       | false    | none         | none                                                                                                                                                                                                      |
| » quantity              | [AssetValue](#schemaassetvalue)                                     | false    | none         | Current size of the position [asset value](#overview--price-and-quantity-precision) format                                                                                                                |
| » notional              | [UsdcValue](#schemausdcvalue)                                       | false    | none         | Notional value of the current position, calculated using the mark price                                                                                                                                   |
| » entryNotional         | [UsdcValue](#schemausdcvalue)                                       | false    | none         | Notional value of the position, using the average entry price                                                                                                                                             |
| » mtmPnl                | [UsdcValue](#schemausdcvalue)                                       | false    | none         | Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement                                                                         |
| » reportedMtmPnl        | [UsdcValue](#schemausdcvalue)                                       | false    | none         | The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase |
| » reportedFundingPnl    | [UsdcValue](#schemausdcvalue)                                       | false    | none         | Sum of all funding payments received since the position was opened. This is updated every time funding is paid.                                                                                           |
| » realizedPnl           | [UsdcValue](#schemausdcvalue)                                       | false    | none         | Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.                                        |
| » settlementAssetSymbol | string                                                              | false    | none         | Settlement Asset Symbol                                                                                                                                                                                   |
| » createdAtDatetime     | [DateTime](#schemadatetime)(date-time)                              | false    | none         | Denotes the time the position was created by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| » createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring)(string)               | false    | none         | Denotes the time the position was created by the exchange, number of milliseconds since EPOCH                                                                                                             |
| » updatedAtDatetime     | [DateTime](#schemadatetime)(date-time)                              | false    | none         | Denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| » updatedAtTimestamp    | [TimeStampAsString](#schematimestampasstring)(string)               | false    | none         | Denotes the time the position was updated by the exchange, number of milliseconds since EPOCH                                                                                                             |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | BUY   |
| side     | SELL  |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-history">history</h1>

## trade-get-orders-v2

<a id="opIdtrade-get-orders-v2"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v2/history/orders?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v2/history/orders', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v2/history/orders`

_Get Historical Orders_

Retrieve a list of orders placed by a trading account with specified filters.

- Only the last 90 days of data is available for querying

This endpoint requires [authentication](#overview--generate-a-jwt-token) and
supports [pagination](#overview--pagination-support). To filter by
`createdAtDatetime` and `createdAtTimestamp`, additional parameters are
required. For detailed instructions, see the
[Filtering Support](#overview--filtering-support) section. Additionally, this
endpoint is subjected to rate limiting.

<h3 id="trade-get-orders-v2-parameters">Parameters</h3>

| Name                   | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization          | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| symbol                 | query  | [MarketSymbol](#schemamarketsymbol)                                             | false    | none                                                                                         |
| orderId                | query  | [OrderID](#schemaorderid)                                                       | false    | none                                                                                         |
| clientOrderId          | query  | [OrderHandle](#schemaorderhandle)                                               | false    | Unique numeric (i64) identifier generated on the client side expressed as a string value     |
| side                   | query  | [OrderSide](#schemaorderside)                                                   | false    | order side                                                                                   |
| status                 | query  | [OrderStatus](#schemaorderstatus)                                               | false    | order status                                                                                 |
| tradingAccountId       | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |
| createdAtDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | false    | start timestamp of period, ISO 8601 with millisecond as string                               |
| createdAtDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | false    | end timestamp of period, ISO 8601 with millisecond as string                                 |

#### Enumerated Values

| Parameter | Value     |
| --------- | --------- |
| side      | BUY       |
| side      | SELL      |
| status    | OPEN      |
| status    | CLOSED    |
| status    | CANCELLED |
| status    | REJECTED  |

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
      "orderId",
      "clientOrderId",
      "symbol",
      "price",
      "stopPrice",
      "averageFillPrice",
      "allowBorrow",
      "quantity",
      "quantityFilled",
      "quoteAmount",
      "baseFee",
      "quoteFee",
      "isLiquidation",
      "side",
      "type",
      "timeInForce",
      "status",
      "statusReason",
      "statusReasonCode",
      "createdAtTimestamp",
      "createdAtDatetime"
    ],
    "properties": {
      "clientOrderId": {
        "allOf": [
          {
            "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
            "type": "string",
            "example": "187"
          }
        ]
      },
      "orderId": {
        "description": "unique order ID",
        "allOf": [
          {
            "type": "string",
            "example": "297735387747975680"
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
        "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "averageFillPrice": {
        "description": "average fill price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "stopPrice": {
        "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "allowBorrow": {
        "description": "indicates if the order was allowed to borrow (does not indicate that borrowing occurred)",
        "type": "boolean",
        "example": false
      },
      "quantity": {
        "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quantityFilled": {
        "description": "quantity filled, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteAmount": {
        "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "baseFee": {
        "description": "base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
        "example": "0.00100000",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteFee": {
        "description": "quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
        "example": "0.0010",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "borrowedBaseQuantity": {
        "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "borrowedQuoteQuantity": {
        "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "isLiquidation": {
        "description": "indicates if the order was executed as a liquidation order",
        "type": "boolean",
        "example": false
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
      "type": {
        "description": "order type",
        "allOf": [
          {
            "type": "string",
            "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
            "example": "LMT"
          }
        ],
        "example": "LMT"
      },
      "timeInForce": {
        "description": "time in force",
        "allOf": [
          {
            "type": "string",
            "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
          }
        ],
        "example": "GTC"
      },
      "status": {
        "description": "order status",
        "allOf": [
          {
            "type": "string",
            "description": "order status can have the following string values `\"OPEN\"`, `\"CLOSED\"`, `\"CANCELLED\"`, `\"REJECTED\"`",
            "example": "OPEN"
          }
        ],
        "example": "OPEN"
      },
      "statusReason": {
        "description": "status reason, describes why the order is in a specific state",
        "type": "string",
        "example": "User cancelled"
      },
      "statusReasonCode": {
        "description": "status reason code, see [details](#overview--error--rejection-codes)",
        "type": "string",
        "example": "1002"
      },
      "createdAtDatetime": {
        "description": "denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string",
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
        "description": "denotes the time the order was ACK'd by the exchange",
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

<h3 id="trade-get-orders-v2-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="trade-get-orders-v2-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type                                                        | Required | Restrictions | Description                                                                                                                 |
| ----------------------- | ----------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_             | [[Order](#schemaorder)]                                     | false    | none         | none                                                                                                                        |
| » clientOrderId         | [OrderHandle](#schemaorderhandle)                           | true     | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value                                    |
| » orderId               | [OrderID](#schemaorderid)                                   | true     | none         | unique order ID                                                                                                             |
| » symbol                | [MarketSymbol](#schemamarketsymbol)                         | true     | none         | market symbol                                                                                                               |
| » price                 | [AssetValue](#schemaassetvalue)                             | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                                    |
| » averageFillPrice      | [AssetValue](#schemaassetvalue)                             | true     | none         | average fill price, see [asset value](#overview--price-and-quantity-precision) format                                       |
| » stopPrice             | [AssetValue](#schemaassetvalue)                             | true     | none         | stop price, see [asset value](#overview--price-and-quantity-precision) format                                               |
| » allowBorrow           | boolean                                                     | true     | none         | indicates if the order was allowed to borrow (does not indicate that borrowing occurred)                                    |
| » quantity              | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                                 |
| » quantityFilled        | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity filled, see [asset value](#overview--price-and-quantity-precision) format                                          |
| » quoteAmount           | [AssetValue](#schemaassetvalue)                             | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format               |
| » baseFee               | [AssetValue](#schemaassetvalue)                             | true     | none         | base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format  |
| » quoteFee              | [AssetValue](#schemaassetvalue)                             | true     | none         | quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format |
| » borrowedBaseQuantity  | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| » borrowedQuoteQuantity | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| » isLiquidation         | boolean                                                     | true     | none         | indicates if the order was executed as a liquidation order                                                                  |
| » side                  | [OrderSideAsString](#schemaordersideasstring)               | true     | none         | order side                                                                                                                  |
| » type                  | [OrderTypeAsString](#schemaordertypeasstring)               | true     | none         | order type                                                                                                                  |
| » timeInForce           | [OrderTimeInForceAsString](#schemaordertimeinforceasstring) | true     | none         | time in force                                                                                                               |
| » status                | [OrderStatusAsString](#schemaorderstatusasstring)           | true     | none         | order status                                                                                                                |
| » statusReason          | string                                                      | true     | none         | status reason, describes why the order is in a specific state                                                               |
| » statusReasonCode      | string                                                      | true     | none         | status reason code, see [details](#overview--error--rejection-codes)                                                        |
| » createdAtDatetime     | [DateTime](#schemadatetime)(date-time)                      | true     | none         | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                   |
| » createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring)(string)       | true     | none         | denotes the time the order was ACK'd by the exchange                                                                        |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## trade-get-trades

<a id="opIdtrade-get-trades"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/trades?tradingAccountId=111000000000001",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/trades', params={
  'tradingAccountId': '111000000000001'
}, headers = headers)

print(r.json())

```

`GET /v1/history/trades`

_Get Historical Trades_

Get a list of trades based on specified filters.

- requires [bearer token](#overview--add-authenticated-request-header) in
  authorization header
- Only the last 90 days of data is available for querying
- [supports pagination](#overview--pagination-support)
- filtering on `createdAtDatetime`, `createdAtTimestamp` requires additional
  keywords, [see filtering support](#overview--filtering-support)

**Ratelimited:** `True`

<h3 id="trade-get-trades-parameters">Parameters</h3>

| Name                   | In     | Type                                                                            | Required | Description                                                                                  |
| ---------------------- | ------ | ------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------------- |
| Authorization          | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token) |
| symbol                 | query  | [MarketSymbol](#schemamarketsymbol)                                             | false    | none                                                                                         |
| orderId                | query  | [OrderID](#schemaorderid)                                                       | false    | unique order ID                                                                              |
| tradeId                | query  | [TradeID](#schematradeid)                                                       | false    | unique trade ID                                                                              |
| tradingAccountId       | query  | [TradingAccountId](#schematradingaccountid)                                     | true     | Id of the trading account                                                                    |
| createdAtDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | false    | start timestamp of period, ISO 8601 with millisecond as string                               |
| createdAtDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | false    | end timestamp of period, ISO 8601 with millisecond as string                                 |

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
      "tradeId",
      "orderId",
      "symbol",
      "price",
      "quantity",
      "quoteAmount",
      "baseFee",
      "quoteFee",
      "side",
      "isTaker",
      "tradeRebateAmount",
      "tradeRebateAssetSymbol",
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
      "orderId": {
        "description": "unique order ID",
        "allOf": [
          {
            "type": "string",
            "example": "297735387747975680"
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
        "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quantity": {
        "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteAmount": {
        "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "baseFee": {
        "description": "base fee, see [asset value](#overview--price-and-quantity-precision) format",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "quoteFee": {
        "description": "quote fee, see [asset value](#overview--price-and-quantity-precision) format",
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
      "tradeRebateAmount": {
        "description": "amount of rebate that is credited to the user as part of the trade.",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "tradeRebateAssetSymbol": {
        "description": "the symbol of the asset in which the rebate is paid",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "USDC"
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

<h3 id="trade-get-trades-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None   |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="trade-get-trades-responseschema">Response Schema</h3>

Status Code **200**

| Name                     | Type                                                  | Required | Restrictions | Description                                                                                                   |
| ------------------------ | ----------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------- |
| _anonymous_              | [[Trade](#schematrade)]                               | false    | none         | none                                                                                                          |
| » tradeId                | [TradeID](#schematradeid)                             | true     | none         | unique trade ID                                                                                               |
| » orderId                | [OrderID](#schemaorderid)                             | true     | none         | unique order ID                                                                                               |
| » symbol                 | [MarketSymbol](#schemamarketsymbol)                   | true     | none         | market symbol                                                                                                 |
| » price                  | [AssetValue](#schemaassetvalue)                       | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                      |
| » quantity               | [AssetValue](#schemaassetvalue)                       | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                   |
| » quoteAmount            | [AssetValue](#schemaassetvalue)                       | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format |
| » baseFee                | [AssetValue](#schemaassetvalue)                       | true     | none         | base fee, see [asset value](#overview--price-and-quantity-precision) format                                   |
| » quoteFee               | [AssetValue](#schemaassetvalue)                       | true     | none         | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                  |
| » side                   | [OrderSideAsString](#schemaordersideasstring)         | true     | none         | order side                                                                                                    |
| » isTaker                | [Boolean](#schemaboolean)(true or false)              | true     | none         | denotes whether this is a taker's trade                                                                       |
| » tradeRebateAmount      | [AssetValue](#schemaassetvalue)                       | true     | none         | amount of rebate that is credited to the user as part of the trade.                                           |
| » tradeRebateAssetSymbol | [QuoteAssetSymbol](#schemaquoteassetsymbol)           | true     | none         | the symbol of the asset in which the rebate is paid                                                           |
| » createdAtDatetime      | [DateTime](#schemadatetime)(date-time)                | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string                  |
| » createdAtTimestamp     | [TimeStampAsString](#schematimestampasstring)(string) | true     | none         | denotes the time the trade was executed by the exchange                                                       |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## get-derivatives-settlement-history

<a id="opIdget-derivatives-settlement-history"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/derivatives-settlement?settlementDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z&settlementDatetime%5Blte%5D=2025-05-20T01%3A01%3A01.000Z",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/derivatives-settlement', params={
  'settlementDatetime[gte]': '2025-05-20T01:01:01.000Z',  'settlementDatetime[lte]': '2025-05-20T01:01:01.000Z'
}, headers = headers)

print(r.json())

```

`GET /v1/history/derivatives-settlement`

_Get Historical Hourly Derivatives Settlement_

Get historical derivatives settlement.

- [supports pagination](#overview--pagination-support)
- filtering on `settlementDatetime` requires additional keywords,
  [see filtering support](#overview--filtering-support)

<h3 id="get-derivatives-settlement-history-parameters">Parameters</h3>

| Name                    | In     | Type                                                                            | Required | Description                                                                                                                                                                                                  |
| ----------------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Authorization           | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [ token ](#overview--generate-a-jwt-token)                                                                                                               |
| tradingAccountId        | query  | [TradingAccountId](#schematradingaccountid)                                     | false    | Id of the trading account. `tradingAccountId` is mandatory in the query for users with multiple trading accounts. For users with a single trading account, it can be automatically retrieved from the login. |
| symbol                  | query  | [DatedFutureMarketSymbol](#schemadatedfuturemarketsymbol)                       | false    | none                                                                                                                                                                                                         |
| settlementDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | true     | start timestamp of window, ISO 8601 with millisecond as string                                                                                                                                               |
| settlementDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | true     | end timestamp of window, ISO 8601 with millisecond as string                                                                                                                                                 |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 25,
  "items": {
    "description": "Derivatives Settlement of one market for the trading account",
    "type": "string",
    "properties": {
      "tradingAccountId": {
        "allOf": [
          {
            "description": "unique trading account ID",
            "type": "string",
            "example": "111000000000001"
          }
        ]
      },
      "symbol": {
        "example": "BTC-USDC-PERP",
        "allOf": [
          {
            "type": "string",
            "description": "market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for DATED FUTURE markets.",
            "example": "BTC-USDC-20241201"
          }
        ]
      },
      "side": {
        "allOf": [
          {
            "type": "string",
            "example": "BUY",
            "enum": ["BUY", "SELL"]
          }
        ]
      },
      "settlementQuantity": {
        "description": "position size at the time of the settlement",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "deltaTradingQuantity": {
        "description": "the change in the position size from the account’s trading activities",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "mtmPnl": {
        "description": "mark to market profit (losses) accumulated since the last settlement",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "fundingPnl": {
        "description": "funding profits (losses) accumulated since the last settlement. Applicable for perpetuals only.",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "eventType": {
        "description": "derivatives position update event types",
        "type": "string",
        "example": "settlementUpdate"
      },
      "settlementMarkPrice": {
        "description": "market price at which the position was settled for this past cycle",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "settlementIndexPrice": {
        "description": "index price at which the position was settled for this past cycle",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.0000"
          }
        ]
      },
      "settlementFundingRate": {
        "description": "funding rate at which the position was settled for this past cycle. Applicable for perpetuals only.",
        "type": "string",
        "example": "10.0"
      },
      "settlementDatetime": {
        "description": "Denotes the time the position was settled by the exchange, ISO 8601 with millisecond as string",
        "allOf": [
          {
            "type": "string",
            "format": "date-time",
            "example": "2025-05-20T01:01:01.000Z",
            "description": "ISO 8601 with millisecond as string"
          }
        ]
      },
      "settlementTimestamp": {
        "description": "Denotes the time the position was settled by the exchange, number of milliseconds since EPOCH",
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

<h3 id="get-derivatives-settlement-history-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="get-derivatives-settlement-history-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type                                                                    | Required | Restrictions | Description                                                                                         |
| ----------------------- | ----------------------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| _anonymous_             | [[DerivativesSettlementResponse](#schemaderivativessettlementresponse)] | false    | none         | [Derivatives Settlement of one market for the trading account]                                      |
| » tradingAccountId      | [TradingAccountId](#schematradingaccountid)                             | false    | none         | unique trading account ID                                                                           |
| » symbol                | [DatedFutureMarketSymbol](#schemadatedfuturemarketsymbol)               | false    | none         | market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for DATED FUTURE markets.   |
| » side                  | [OrderSide](#schemaorderside)                                           | false    | none         | none                                                                                                |
| » settlementQuantity    | [AssetValue](#schemaassetvalue)                                         | false    | none         | position size at the time of the settlement                                                         |
| » deltaTradingQuantity  | [AssetValue](#schemaassetvalue)                                         | false    | none         | the change in the position size from the account’s trading activities                               |
| » mtmPnl                | [UsdcValue](#schemausdcvalue)                                           | false    | none         | mark to market profit (losses) accumulated since the last settlement                                |
| » fundingPnl            | [UsdcValue](#schemausdcvalue)                                           | false    | none         | funding profits (losses) accumulated since the last settlement. Applicable for perpetuals only.     |
| » eventType             | string                                                                  | false    | none         | derivatives position update event types                                                             |
| » settlementMarkPrice   | [UsdcValue](#schemausdcvalue)                                           | false    | none         | market price at which the position was settled for this past cycle                                  |
| » settlementIndexPrice  | [UsdcValue](#schemausdcvalue)                                           | false    | none         | index price at which the position was settled for this past cycle                                   |
| » settlementFundingRate | string                                                                  | false    | none         | funding rate at which the position was settled for this past cycle. Applicable for perpetuals only. |
| » settlementDatetime    | [DateTime](#schemadatetime)(date-time)                                  | false    | none         | Denotes the time the position was settled by the exchange, ISO 8601 with millisecond as string      |
| » settlementTimestamp   | [TimeStampAsString](#schematimestampasstring)(string)                   | false    | none         | Denotes the time the position was settled by the exchange, number of milliseconds since EPOCH       |

#### Enumerated Values

| Property | Value |
| -------- | ----- |
| side     | BUY   |
| side     | SELL  |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## get-transfer-history

<a id="opIdget-transfer-history"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/transfer?createdAtDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z&createdAtDatetime%5Blte%5D=2025-05-20T01%3A01%3A01.000Z",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/transfer', params={
  'createdAtDatetime[gte]': '2025-05-20T01:01:01.000Z',  'createdAtDatetime[lte]': '2025-05-20T01:01:01.000Z'
}, headers = headers)

print(r.json())

```

`GET /v1/history/transfer`

_Get Historical Account Transfer_

Get historical transfers.

- [supports pagination](#overview--pagination-support)
- filtering on `createdAtDatetime` and `createdAtTimestamp` requires additional
  keywords, [see filtering support](#overview--filtering-support)

<h3 id="get-transfer-history-parameters">Parameters</h3>

| Name                   | In     | Type                                                                            | Required | Description                                                                                                                                                                                                  |
| ---------------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Authorization          | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                                                                                                 |
| tradingAccountId       | query  | [TradingAccountId](#schematradingaccountid)                                     | false    | Id of the trading account. `tradingAccountId` is mandatory in the query for users with multiple trading accounts. For users with a single trading account, it can be automatically retrieved from the login. |
| status                 | query  | string                                                                          | false    | Status of the transfer request. Defaults to `CLOSED`                                                                                                                                                         |
| requestId              | query  | string                                                                          | false    | Unique identifier of the transfer request                                                                                                                                                                    |
| assetSymbol            | query  | [AssetSymbol](#schemaassetsymbol)                                               | false    | Asset symbol of the transfer request                                                                                                                                                                         |
| createdAtDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | true     | start datetime of window, ISO 8601 with millisecond as string                                                                                                                                                |
| createdAtDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | true     | end datetime of window, ISO 8601 with millisecond as string                                                                                                                                                  |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 25,
  "items": {
    "description": "Get account transfer history",
    "type": "string",
    "properties": {
      "requestId": {
        "description": "unique identifier of the transfer request",
        "type": "string",
        "example": "1"
      },
      "toTradingAccountId": {
        "description": "recipient's trading account",
        "allOf": [
          {
            "description": "unique trading account ID",
            "type": "string",
            "example": "111000000000001"
          }
        ]
      },
      "fromTradingAccountId": {
        "description": "sender's trading account",
        "type": "string",
        "example": "121000000000001"
      },
      "assetSymbol": {
        "description": "asset currency of the transfer",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "quantity": {
        "description": "transfer quantity",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "status": {
        "description": "transfer status [CLOSED/OPEN/REJECTED]",
        "type": "string",
        "example": "CLOSED"
      },
      "statusReasonCode": {
        "description": "status reason code",
        "type": "string",
        "example": "6002"
      },
      "statusReason": {
        "description": "readable status reason",
        "type": "string",
        "example": "Executed"
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
}
```

<h3 id="get-transfer-history-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="get-transfer-history-responseschema">Response Schema</h3>

Status Code **200**

| Name                   | Type                                                              | Required | Restrictions | Description                                                                                       |
| ---------------------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| _anonymous_            | [[SubAccountTransferResponse](#schemasubaccounttransferresponse)] | false    | none         | [Get account transfer history]                                                                    |
| » requestId            | string                                                            | false    | none         | unique identifier of the transfer request                                                         |
| » toTradingAccountId   | [TradingAccountId](#schematradingaccountid)                       | false    | none         | recipient's trading account                                                                       |
| » fromTradingAccountId | string                                                            | false    | none         | sender's trading account                                                                          |
| » assetSymbol          | [AssetSymbol](#schemaassetsymbol)                                 | false    | none         | asset currency of the transfer                                                                    |
| » quantity             | [AssetValue](#schemaassetvalue)                                   | false    | none         | transfer quantity                                                                                 |
| » status               | string                                                            | false    | none         | transfer status [CLOSED/OPEN/REJECTED]                                                            |
| » statusReasonCode     | string                                                            | false    | none         | status reason code                                                                                |
| » statusReason         | string                                                            | false    | none         | readable status reason                                                                            |
| » createdAtTimestamp   | [TimeStampAsString](#schematimestampasstring)(string)             | false    | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| » createdAtDatetime    | [DateTime](#schemadatetime)(date-time)                            | false    | none         | ISO 8601 with millisecond as string                                                               |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

## market-data-get-historical-anonymous-trades-by-market-symbol

<a id="opIdmarket-data-get-historical-anonymous-trades-by-market-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/markets/{symbol}/trades",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/markets/{symbol}/trades', headers = headers)

print(r.json())

```

`GET /v1/history/markets/{symbol}/trades`

_Get Historical Market Trades_

Get Historical Market Trades by Market Symbol. Supports querying of up to 7 days
of data at a time.

- [supports pagination](#overview--pagination-support)

**Ratelimited:** `False`

<h3 id="market-data-get-historical-anonymous-trades-by-market-symbol-parameters">Parameters</h3>

| Name                   | In    | Type                                | Required | Description                                                    |
| ---------------------- | ----- | ----------------------------------- | -------- | -------------------------------------------------------------- |
| symbol                 | path  | [MarketSymbol](#schemamarketsymbol) | true     | symbol to get                                                  |
| createdAtDatetime[gte] | query | [DateTime](#schemadatetime)         | false    | start timestamp of period, ISO 8601 with millisecond as string |
| createdAtDatetime[lte] | query | [DateTime](#schemadatetime)         | false    | end timestamp of period, ISO 8601 with millisecond as string   |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 25,
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

<h3 id="market-data-get-historical-anonymous-trades-by-market-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="market-data-get-historical-anonymous-trades-by-market-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name                 | Type                                                    | Required | Restrictions | Description                                                                                  |
| -------------------- | ------------------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------- |
| _anonymous_          | [[ObfuscatedTradeWithId](#schemaobfuscatedtradewithid)] | false    | none         | none                                                                                         |
| » tradeId            | [TradeID](#schematradeid)                               | true     | none         | unique trade ID                                                                              |
| » symbol             | [MarketSymbol](#schemamarketsymbol)                     | true     | none         | market symbol                                                                                |
| » price              | [AssetValue](#schemaassetvalue)                         | true     | none         | price                                                                                        |
| » quantity           | [AssetValue](#schemaassetvalue)                         | true     | none         | quantity                                                                                     |
| » side               | [OrderSideAsString](#schemaordersideasstring)           | true     | none         | order side                                                                                   |
| » isTaker            | [Boolean](#schemaboolean)(true or false)                | true     | none         | denotes whether this is a taker's trade                                                      |
| » createdAtDatetime  | [DateTime](#schemadatetime)(date-time)                  | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string |
| » createdAtTimestamp | [TimeStampAsString](#schematimestampasstring)(string)   | true     | none         | denotes the time the trade was executed by the exchange                                      |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-get-funding-rate-history-by-market-symbol

<a id="opIdmarket-data-get-funding-rate-history-by-market-symbol"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json"
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/markets/{symbol}/funding-rate",
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

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/markets/{symbol}/funding-rate', headers = headers)

print(r.json())

```

`GET /v1/history/markets/{symbol}/funding-rate`

_Get Historical Funding Rate_

Get historical hourly funding rate for the requested perpetual market

- [supports pagination](#overview--pagination-support)

<h3 id="market-data-get-funding-rate-history-by-market-symbol-parameters">Parameters</h3>

| Name                   | In    | Type                                        | Required | Description                                                    |
| ---------------------- | ----- | ------------------------------------------- | -------- | -------------------------------------------------------------- |
| symbol                 | path  | [PerpMarketSymbol](#schemaperpmarketsymbol) | true     | symbol to get                                                  |
| updatedAtDatetime[gte] | query | [DateTime](#schemadatetime)                 | false    | start timestamp of period, ISO 8601 with millisecond as string |
| updatedAtDatetime[lte] | query | [DateTime](#schemadatetime)                 | false    | end timestamp of period, ISO 8601 with millisecond as string   |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 100,
  "items": {
    "description": "Hourly Funding Rate History of one market",
    "type": "array",
    "properties": {
      "fundingRate": {
        "description": "funding rate for this hour",
        "type": "string",
        "example": "0.1"
      },
      "updatedAtDatetime": {
        "description": "date time of the last funding rate update for the hour",
        "type": "string",
        "example": "2024-09-16T12:59:59.000Z"
      }
    }
  }
}
```

<h3 id="market-data-get-funding-rate-history-by-market-symbol-responses">Responses</h3>

| Status | Meaning                                                                    | Description             | Schema |
| ------ | -------------------------------------------------------------------------- | ----------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                      | Inline |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)           | Bad Request             | None   |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Market Symbol Not Found | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests       | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error   | None   |

<h3 id="market-data-get-funding-rate-history-by-market-symbol-responseschema">Response Schema</h3>

Status Code **200**

| Name                | Type                                                              | Required | Restrictions | Description                                            |
| ------------------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------ |
| _anonymous_         | [[FundingRateHistoryResponse](#schemafundingratehistoryresponse)] | false    | none         | [Hourly Funding Rate History of one market]            |
| » fundingRate       | string                                                            | false    | none         | funding rate for this hour                             |
| » updatedAtDatetime | string                                                            | false    | none         | date time of the last funding rate update for the hour |

<aside class="success">
This operation does not require authentication
</aside>

## market-data-get-historical-borrow-interest

<a id="opIdmarket-data-get-historical-borrow-interest"></a>

> Code samples

```javascript
const headers = {
  Accept: "application/json",
  Authorization: {
    description:
      "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
    schema: {
      type: "string"
    },
    type: "string"
  }
}

fetch(
  "https://api.exchange.bullish.com/trading-api/v1/history/borrow-interest?assetSymbol=BTC&createdAtDatetime%5Bgte%5D=2025-05-20T01%3A01%3A01.000Z&createdAtDatetime%5Blte%5D=2025-05-20T01%3A01%3A01.000Z",
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
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.get('https://api.exchange.bullish.com/trading-api/v1/history/borrow-interest', params={
  'assetSymbol': 'BTC',  'createdAtDatetime[gte]': '2025-05-20T01:01:01.000Z',  'createdAtDatetime[lte]': '2025-05-20T01:01:01.000Z'
}, headers = headers)

print(r.json())

```

`GET /v1/history/borrow-interest`

_Get Historical Hourly Borrow Interest_

Get Historical Hourly Borrow Interest. Each entry denotes the hourly quantities
for the specific asset. Total borrowed quantity is inclusive of interest.
`interest = totalBorrowedQuantity - borrowedQuantity` which denotes the interest
charged in the particular hour for the asset.

- [supports pagination](#overview--pagination-support)
- filtering `createdAtDatetime`, `createdAtTimestamp` requires additional
  keywords, [see filtering support](#overview--filtering-support)

**Ratelimited:** `True`

<h3 id="market-data-get-historical-borrow-interest-parameters">Parameters</h3>

| Name                   | In     | Type                                                                            | Required | Description                                                                                                                                                                                                  |
| ---------------------- | ------ | ------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Authorization          | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | true     | authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)                                                                                                                 |
| tradingAccountId       | query  | [TradingAccountId](#schematradingaccountid)                                     | false    | Id of the trading account. `tradingAccountId` is mandatory in the query for users with multiple trading accounts. For users with a single trading account, it can be automatically retrieved from the login. |
| assetSymbol            | query  | [AssetSymbol](#schemaassetsymbol)                                               | true     | none                                                                                                                                                                                                         |
| createdAtDatetime[gte] | query  | [DateTime](#schemadatetime)                                                     | true     | start timestamp of period, ISO 8601 with millisecond as string                                                                                                                                               |
| createdAtDatetime[lte] | query  | [DateTime](#schemadatetime)                                                     | true     | end timestamp of period, ISO 8601 with millisecond as string                                                                                                                                                 |

> Example responses

> 200 Response

```json
{
  "type": "array",
  "minItems": 0,
  "maxItems": 25,
  "items": {
    "type": "object",
    "required": [
      "assetId",
      "assetSymbol",
      "borrowedQuantity",
      "totalBorrowedQuantity",
      "createdAtDatetime",
      "createdAtTimestamp"
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
      "assetSymbol": {
        "description": "asset symbol",
        "allOf": [
          {
            "type": "string",
            "description": "asset symbol as denoted in the world",
            "example": "BTC"
          }
        ]
      },
      "borrowedQuantity": {
        "description": "the principal borrowed quantity",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "totalBorrowedQuantity": {
        "description": "the sum of the principal borrowed quantity and the interest charged",
        "allOf": [
          {
            "description": "see [asset value](#overview--price-and-quantity-precision) format",
            "type": "string",
            "example": "1.00000000"
          }
        ]
      },
      "createdAtDatetime": {
        "description": "denotes the hour in which the principal quantity was borrowed or when the interest was charged, ISO 8601 with millisecond as string",
        "type": "string",
        "format": "date-time",
        "example": "2020-08-21T08:00:00.000Z"
      },
      "createdAtTimestamp": {
        "description": "denotes the hour in which the principal quantity was borrowed or when the interest was charged",
        "type": "string",
        "format": "string",
        "example": "1621490985000"
      }
    }
  }
}
```

<h3 id="market-data-get-historical-borrow-interest-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema |
| ------ | -------------------------------------------------------------------------- | --------------------- | ------ |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | Inline |
| 404    | [Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)             | Resource Not Found    | None   |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None   |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None   |

<h3 id="market-data-get-historical-borrow-interest-responseschema">Response Schema</h3>

Status Code **200**

| Name                    | Type                                      | Required | Restrictions | Description                                                                                                                         |
| ----------------------- | ----------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_             | [[BorrowInterest](#schemaborrowinterest)] | false    | none         | none                                                                                                                                |
| » assetId               | [AssetID](#schemaassetid)                 | true     | none         | unique asset ID                                                                                                                     |
| » assetSymbol           | [AssetSymbol](#schemaassetsymbol)         | true     | none         | asset symbol                                                                                                                        |
| » borrowedQuantity      | [AssetValue](#schemaassetvalue)           | true     | none         | the principal borrowed quantity                                                                                                     |
| » totalBorrowedQuantity | [AssetValue](#schemaassetvalue)           | true     | none         | the sum of the principal borrowed quantity and the interest charged                                                                 |
| » createdAtDatetime     | string(date-time)                         | true     | none         | denotes the hour in which the principal quantity was borrowed or when the interest was charged, ISO 8601 with millisecond as string |
| » createdAtTimestamp    | string(string)                            | true     | none         | denotes the hour in which the principal quantity was borrowed or when the interest was charged                                      |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

<h1 id="bullish-trading-api-portfolio-margin-simulator">portfolio-margin-simulator</h1>

## simulate-portfolio-margin

<a id="opIdsimulate-portfolio-margin"></a>

> Code samples

```javascript
const inputBody = '{
  "type": "object",
  "required": [
    "tradingAccountId"
  ],
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "positions": {
      "type": "array",
      "description": "portfolio position to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": [
              "symbol",
              "quantity"
            ],
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC-USDC-PERP"
              },
              "quantity": {
                "description": "size of the position",
                "type": "string",
                "example": "1.0"
              }
            }
          }
        ]
      }
    },
    "orders": {
      "type": "array",
      "description": "pending orders to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": [
              "symbol",
              "quantity"
            ],
            "properties": {
              "symbol": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                    "example": "BTCUSDC"
                  }
                ]
              },
              "quantity": {
                "description": "quantity placed for order",
                "type": "string",
                "example": "1.0"
              },
              "limitPrice": {
                "description": "limit price for order",
                "type": "string",
                "example": "10000.0"
              }
            }
          }
        ]
      }
    },
    "referencePrices": {
      "type": "array",
      "description": "reference price to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC"
              },
              "price": {
                "description": "reference price for asset or market",
                "type": "string",
                "example": "12000.0"
              }
            }
          }
        ]
      }
    }
  }
}';
const headers = {
  'Content-Type':'application/json',
  'Accept':'application/json',
  'Authorization':{
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
};

fetch('https://api.exchange.bullish.com/trading-api/v1/simulate-portfolio-margin',
{
  method: 'POST',
  body: inputBody,
  headers: headers
})
.then(function(res) {
    return res.json();
}).then(function(body) {
    console.log(body);
});

```

```python
import requests
headers = {
  'Content-Type': 'application/json',
  'Accept': 'application/json',
  'Authorization': {
  "description": "authorization header, its value must be 'Bearer ' + [token](#overview--generate-a-jwt-token)",
  "schema": {
    "type": "string"
  },
  "type": "string"
}
}

r = requests.post('https://api.exchange.bullish.com/trading-api/v1/simulate-portfolio-margin', headers = headers)

print(r.json())

```

`POST /v1/simulate-portfolio-margin`

_Portfolio Margin Simulator_

Use Portfolio margin simulator to determine your margin requirements and risk
levels based on your current portfolio balances. You can also append position
details on top of your portfolio specifics to see simulated results.

> Body parameter

```json
{
  "type": "object",
  "required": ["tradingAccountId"],
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "positions": {
      "type": "array",
      "description": "portfolio position to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": ["symbol", "quantity"],
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC-USDC-PERP"
              },
              "quantity": {
                "description": "size of the position",
                "type": "string",
                "example": "1.0"
              }
            }
          }
        ]
      }
    },
    "orders": {
      "type": "array",
      "description": "pending orders to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": ["symbol", "quantity"],
            "properties": {
              "symbol": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                    "example": "BTCUSDC"
                  }
                ]
              },
              "quantity": {
                "description": "quantity placed for order",
                "type": "string",
                "example": "1.0"
              },
              "limitPrice": {
                "description": "limit price for order",
                "type": "string",
                "example": "10000.0"
              }
            }
          }
        ]
      }
    },
    "referencePrices": {
      "type": "array",
      "description": "reference price to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC"
              },
              "price": {
                "description": "reference price for asset or market",
                "type": "string",
                "example": "12000.0"
              }
            }
          }
        ]
      }
    }
  }
}
```

<h3 id="simulate-portfolio-margin-parameters">Parameters</h3>

| Name            | In     | Type                                                                            | Required | Description                                                                                    |
| --------------- | ------ | ------------------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------- |
| Authorization   | header | [#/components/headers/Authorization](#schema#/components/headers/authorization) | false    | authorization header, its value must be 'Bearer ' + [ token ](#overview--generate-a-jwt-token) |
| includeExisting | query  | [Boolean](#schemaboolean)                                                       | false    | none                                                                                           |
| body            | body   | [PortfolioSimulationRequest](#schemaportfoliosimulationrequest)                 | false    | none                                                                                           |

> Example responses

> 200 Response

```json
{
  "description": "Simulation result",
  "type": "string",
  "properties": {
    "collateralUSD": {
      "description": "total collateral across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "13000.0000"
    },
    "borrowedUSD": {
      "description": "total borrowed across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "12000.0000"
    },
    "initialMarginUSD": {
      "description": "The minimum margin one must maintain in order to be able to purposefully increase risk",
      "type": "string",
      "example": "14000.0000"
    },
    "warningMarginUSD": {
      "description": "The minimum margin when the customer will receive warning via email/notifications over UI",
      "type": "string",
      "example": "15000.0000"
    },
    "liquidationMarginUSD": {
      "description": "The minimum value of margin one must maintain in order to avoid liquidation",
      "type": "string",
      "example": "16000.0000"
    },
    "fullLiquidationMarginUSD": {
      "description": "The value of margin when full liquidation occurs",
      "type": "string",
      "example": "17000.0000"
    },
    "defaultedMarginUSD": {
      "description": "The value of margin when this trading account will be moved into a Defaulted state",
      "type": "string",
      "example": "18000.0000"
    },
    "liquidityAddonUSD": {
      "description": "expected market impact of unwinding the portfolio in the case of a liquidation event",
      "type": "string",
      "example": "19000.0000"
    },
    "marketRiskUSD": {
      "description": "the worst possible loss on the portfolio based on scenario analysis",
      "type": "string",
      "example": "20000.0000"
    }
  }
}
```

<h3 id="simulate-portfolio-margin-responses">Responses</h3>

| Status | Meaning                                                                    | Description           | Schema                                                            |
| ------ | -------------------------------------------------------------------------- | --------------------- | ----------------------------------------------------------------- |
| 200    | [OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)                    | OK                    | [PortfolioSimulationResponse](#schemaportfoliosimulationresponse) |
| 400    | [Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)           | Bad Request           | None                                                              |
| 401    | [Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)            | Not Authenticated     | None                                                              |
| 403    | [Forbidden](https://tools.ietf.org/html/rfc7231#section-6.5.3)             | Access Forbidden      | None                                                              |
| 429    | [Too Many Requests](https://tools.ietf.org/html/rfc6585#section-4)         | Too Many Requests     | None                                                              |
| 500    | [Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1) | Internal Server Error | None                                                              |

<aside class="warning">
To perform this operation, you must be authenticated by means of one of the following methods:
jwtTokenAuth
</aside>

# Schemas

<h2 id="tocS_ApiResponse">ApiResponse</h2>
<!-- backwards compatibility -->
<a id="schemaapiresponse"></a>
<a id="schema_ApiResponse"></a>
<a id="tocSapiresponse"></a>
<a id="tocsapiresponse"></a>

```json
{
  "type": "object",
  "properties": {
    "code": {
      "type": "integer",
      "format": "int32"
    },
    "type": {
      "type": "string"
    },
    "message": {
      "type": "string"
    }
  }
}
```

### Properties

| Name    | Type           | Required | Restrictions | Description |
| ------- | -------------- | -------- | ------------ | ----------- |
| code    | integer(int32) | false    | none         | none        |
| type    | string         | false    | none         | none        |
| message | string         | false    | none         | none        |

<h2 id="tocS_Boolean">Boolean</h2>
<!-- backwards compatibility -->
<a id="schemaboolean"></a>
<a id="schema_Boolean"></a>
<a id="tocSboolean"></a>
<a id="tocsboolean"></a>

```json
true
```

### Properties

| Name        | Type                   | Required | Restrictions | Description |
| ----------- | ---------------------- | -------- | ------------ | ----------- |
| _anonymous_ | boolean(true or false) | false    | none         | none        |

<h2 id="tocS_MarketTypeAsString">MarketTypeAsString</h2>
<!-- backwards compatibility -->
<a id="schemamarkettypeasstring"></a>
<a id="schema_MarketTypeAsString"></a>
<a id="tocSmarkettypeasstring"></a>
<a id="tocsmarkettypeasstring"></a>

```json
"SPOT"
```

market type can have the following string values `"SPOT"`, `"PERPETUAL"`,
`"DATED_FUTURE"`

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------ |
| _anonymous_ | string | false    | none         | market type can have the following string values `"SPOT"`, `"PERPETUAL"`, `"DATED_FUTURE"` |

#### Enumerated Values

| Property    | Value        |
| ----------- | ------------ |
| _anonymous_ | SPOT         |
| _anonymous_ | PERPETUAL    |
| _anonymous_ | DATED_FUTURE |

<h2 id="tocS_OrderTypeAsString">OrderTypeAsString</h2>
<!-- backwards compatibility -->
<a id="schemaordertypeasstring"></a>
<a id="schema_OrderTypeAsString"></a>
<a id="tocSordertypeasstring"></a>
<a id="tocsordertypeasstring"></a>

```json
"LMT"
```

order type can have the following string values `"LMT"`, `"MKT"`,
`"STOP_LIMIT"`, `"POST_ONLY"`.

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                      |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------ |
| _anonymous_ | string | false    | none         | order type can have the following string values `"LMT"`, `"MKT"`, `"STOP_LIMIT"`, `"POST_ONLY"`. |

<h2 id="tocS_OrderTypeAsStringV2">OrderTypeAsStringV2</h2>
<!-- backwards compatibility -->
<a id="schemaordertypeasstringv2"></a>
<a id="schema_OrderTypeAsStringV2"></a>
<a id="tocSordertypeasstringv2"></a>
<a id="tocsordertypeasstringv2"></a>

```json
"LIMIT"
```

order type can have the following string values `"LIMIT"`, `"MARKET"`,
`"STOP_LIMIT"`, `"POST_ONLY"`.

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                           |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | order type can have the following string values `"LIMIT"`, `"MARKET"`, `"STOP_LIMIT"`, `"POST_ONLY"`. |

<h2 id="tocS_OrderTypeAsStringAmend">OrderTypeAsStringAmend</h2>
<!-- backwards compatibility -->
<a id="schemaordertypeasstringamend"></a>
<a id="schema_OrderTypeAsStringAmend"></a>
<a id="tocSordertypeasstringamend"></a>
<a id="tocsordertypeasstringamend"></a>

```json
"LIMIT"
```

order type can have the following string values `"LIMIT"`, `"POST_ONLY"`

### Properties

| Name        | Type   | Required | Restrictions | Description                                                              |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------ |
| _anonymous_ | string | false    | none         | order type can have the following string values `"LIMIT"`, `"POST_ONLY"` |

<h2 id="tocS_OrderSideAsString">OrderSideAsString</h2>
<!-- backwards compatibility -->
<a id="schemaordersideasstring"></a>
<a id="schema_OrderSideAsString"></a>
<a id="tocSordersideasstring"></a>
<a id="tocsordersideasstring"></a>

```json
"BUY"
```

order side can have the following string values `"BUY"`, `"SELL"`

### Properties

| Name        | Type   | Required | Restrictions | Description                                                       |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | order side can have the following string values `"BUY"`, `"SELL"` |

<h2 id="tocS_OrderTimeInForceAsString">OrderTimeInForceAsString</h2>
<!-- backwards compatibility -->
<a id="schemaordertimeinforceasstring"></a>
<a id="schema_OrderTimeInForceAsString"></a>
<a id="tocSordertimeinforceasstring"></a>
<a id="tocsordertimeinforceasstring"></a>

```json
{
  "type": "string",
  "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
}
```

time in force can have the following string values `"GTC"`, `"FOK"`, `"IOC"`,
see [details](#overview--order-timeinforce)

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                                               |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | time in force can have the following string values `"GTC"`, `"FOK"`, `"IOC"`, see [details](#overview--order-timeinforce) |

<h2 id="tocS_OrderStatusAsString">OrderStatusAsString</h2>
<!-- backwards compatibility -->
<a id="schemaorderstatusasstring"></a>
<a id="schema_OrderStatusAsString"></a>
<a id="tocSorderstatusasstring"></a>
<a id="tocsorderstatusasstring"></a>

```json
"OPEN"
```

order status can have the following string values `"OPEN"`, `"CLOSED"`,
`"CANCELLED"`, `"REJECTED"`

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                         |
| ----------- | ------ | -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | order status can have the following string values `"OPEN"`, `"CLOSED"`, `"CANCELLED"`, `"REJECTED"` |

<h2 id="tocS_TimeStampAsString">TimeStampAsString</h2>
<!-- backwards compatibility -->
<a id="schematimestampasstring"></a>
<a id="schema_TimeStampAsString"></a>
<a id="tocStimestampasstring"></a>
<a id="tocstimestampasstring"></a>

```json
"1621490985000"
```

unsigned 64 bit integer value which is the number of milliseconds since EPOCH
expressed as string

### Properties

| Name        | Type           | Required | Restrictions | Description                                                                                       |
| ----------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| _anonymous_ | string(string) | false    | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |

<h2 id="tocS_NonceAsString">NonceAsString</h2>
<!-- backwards compatibility -->
<a id="schemanonceasstring"></a>
<a id="schema_NonceAsString"></a>
<a id="tocSnonceasstring"></a>
<a id="tocsnonceasstring"></a>

```json
"123456789"
```

the nonce is a client side incremented unsigned 64 bit integer expressed as
string

### Properties

| Name        | Type           | Required | Restrictions | Description                                                                        |
| ----------- | -------------- | -------- | ------------ | ---------------------------------------------------------------------------------- |
| _anonymous_ | string(string) | false    | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string |

<h2 id="tocS_SpotAccountID">SpotAccountID</h2>
<!-- backwards compatibility -->
<a id="schemaspotaccountid"></a>
<a id="schema_SpotAccountID"></a>
<a id="tocSspotaccountid"></a>
<a id="tocsspotaccountid"></a>

```json
"1"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_OrderID">OrderID</h2>
<!-- backwards compatibility -->
<a id="schemaorderid"></a>
<a id="schema_OrderID"></a>
<a id="tocSorderid"></a>
<a id="tocsorderid"></a>

```json
"297735387747975680"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_AMMInstructionID">AMMInstructionID</h2>
<!-- backwards compatibility -->
<a id="schemaamminstructionid"></a>
<a id="schema_AMMInstructionID"></a>
<a id="tocSamminstructionid"></a>
<a id="tocsamminstructionid"></a>

```json
"297735387747975680"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_RequestID">RequestID</h2>
<!-- backwards compatibility -->
<a id="schemarequestid"></a>
<a id="schema_RequestID"></a>
<a id="tocSrequestid"></a>
<a id="tocsrequestid"></a>

```json
"197735387747975680"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_OrderHandle">OrderHandle</h2>
<!-- backwards compatibility -->
<a id="schemaorderhandle"></a>
<a id="schema_OrderHandle"></a>
<a id="tocSorderhandle"></a>
<a id="tocsorderhandle"></a>

```json
"187"
```

Unique numeric (i64) identifier generated on the client side expressed as a
string value

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                              |
| ----------- | ------ | -------- | ------------ | ---------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value |

<h2 id="tocS_TradingAccountId">TradingAccountId</h2>
<!-- backwards compatibility -->
<a id="schematradingaccountid"></a>
<a id="schema_TradingAccountId"></a>
<a id="tocStradingaccountid"></a>
<a id="tocstradingaccountid"></a>

```json
"111000000000001"
```

unique trading account ID

### Properties

| Name        | Type   | Required | Restrictions | Description               |
| ----------- | ------ | -------- | ------------ | ------------------------- |
| _anonymous_ | string | false    | none         | unique trading account ID |

<h2 id="tocS_TradingAccountIds">TradingAccountIds</h2>
<!-- backwards compatibility -->
<a id="schematradingaccountids"></a>
<a id="schema_TradingAccountIds"></a>
<a id="tocStradingaccountids"></a>
<a id="tocstradingaccountids"></a>

```json
{
  "description": "list of trading account ids.",
  "type": "array",
  "items": {
    "type": "string",
    "example": "111000000000001"
  }
}
```

list of trading account ids.

### Properties

_None_

<h2 id="tocS_MarketID">MarketID</h2>
<!-- backwards compatibility -->
<a id="schemamarketid"></a>
<a id="schema_MarketID"></a>
<a id="tocSmarketid"></a>
<a id="tocsmarketid"></a>

```json
"10000"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_TradeID">TradeID</h2>
<!-- backwards compatibility -->
<a id="schematradeid"></a>
<a id="schema_TradeID"></a>
<a id="tocStradeid"></a>
<a id="tocstradeid"></a>

```json
"100020000000000060"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_AssetValue">AssetValue</h2>
<!-- backwards compatibility -->
<a id="schemaassetvalue"></a>
<a id="schema_AssetValue"></a>
<a id="tocSassetvalue"></a>
<a id="tocsassetvalue"></a>

```json
"1.00000000"
```

see [asset value](#overview--price-and-quantity-precision) format

### Properties

| Name        | Type   | Required | Restrictions | Description                                                       |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | see [asset value](#overview--price-and-quantity-precision) format |

<h2 id="tocS_UsdcValue">UsdcValue</h2>
<!-- backwards compatibility -->
<a id="schemausdcvalue"></a>
<a id="schema_UsdcValue"></a>
<a id="tocSusdcvalue"></a>
<a id="tocsusdcvalue"></a>

```json
"1.0000"
```

see [asset value](#overview--price-and-quantity-precision) format

### Properties

| Name        | Type   | Required | Restrictions | Description                                                       |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | see [asset value](#overview--price-and-quantity-precision) format |

<h2 id="tocS_UserId">UserId</h2>
<!-- backwards compatibility -->
<a id="schemauserid"></a>
<a id="schema_UserId"></a>
<a id="tocSuserid"></a>
<a id="tocsuserid"></a>

```json
"12345"
```

Bullish user ID

### Properties

| Name        | Type   | Required | Restrictions | Description     |
| ----------- | ------ | -------- | ------------ | --------------- |
| _anonymous_ | string | false    | none         | Bullish user ID |

<h2 id="tocS_TimeStampInSeconds">TimeStampInSeconds</h2>
<!-- backwards compatibility -->
<a id="schematimestampinseconds"></a>
<a id="schema_TimeStampInSeconds"></a>
<a id="tocStimestampinseconds"></a>
<a id="tocstimestampinseconds"></a>

```json
1621490985
```

number of seconds since EPOCH as integer

### Properties

| Name        | Type           | Required | Restrictions | Description                              |
| ----------- | -------------- | -------- | ------------ | ---------------------------------------- |
| _anonymous_ | integer(int64) | false    | none         | number of seconds since EPOCH as integer |

<h2 id="tocS_DateTime">DateTime</h2>
<!-- backwards compatibility -->
<a id="schemadatetime"></a>
<a id="schema_DateTime"></a>
<a id="tocSdatetime"></a>
<a id="tocsdatetime"></a>

```json
"2025-05-20T01:01:01.000Z"
```

ISO 8601 with millisecond as string

### Properties

| Name        | Type              | Required | Restrictions | Description                         |
| ----------- | ----------------- | -------- | ------------ | ----------------------------------- |
| _anonymous_ | string(date-time) | false    | none         | ISO 8601 with millisecond as string |

<h2 id="tocS_TimeStamp">TimeStamp</h2>
<!-- backwards compatibility -->
<a id="schematimestamp"></a>
<a id="schema_TimeStamp"></a>
<a id="tocStimestamp"></a>
<a id="tocstimestamp"></a>

```json
"1621490985000"
```

number of milliseconds since EPOCH as string

### Properties

| Name        | Type          | Required | Restrictions | Description                                  |
| ----------- | ------------- | -------- | ------------ | -------------------------------------------- |
| _anonymous_ | string(int64) | false    | none         | number of milliseconds since EPOCH as string |

<h2 id="tocS_CustodyDestinationID">CustodyDestinationID</h2>
<!-- backwards compatibility -->
<a id="schemacustodydestinationid"></a>
<a id="schema_CustodyDestinationID"></a>
<a id="tocScustodydestinationid"></a>
<a id="tocscustodydestinationid"></a>

```json
"1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038"
```

destination id provided by bullish that uniquely identifies a whitelisted
address or account

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                  |
| ----------- | ------ | -------- | ------------ | -------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account |

<h2 id="tocS_NetworkID">NetworkID</h2>
<!-- backwards compatibility -->
<a id="schemanetworkid"></a>
<a id="schema_NetworkID"></a>
<a id="tocSnetworkid"></a>
<a id="tocsnetworkid"></a>

```json
"EOS"
```

the network of the native coin or token, e.g. BTC, ETH, EOS

### Properties

| Name        | Type   | Required | Restrictions | Description                                                 |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------- |
| _anonymous_ | string | false    | none         | the network of the native coin or token, e.g. BTC, ETH, EOS |

<h2 id="tocS_CustodyNetworkAddress">CustodyNetworkAddress</h2>
<!-- backwards compatibility -->
<a id="schemacustodynetworkaddress"></a>
<a id="schema_CustodyNetworkAddress"></a>
<a id="tocScustodynetworkaddress"></a>
<a id="tocscustodynetworkaddress"></a>

```json
"0xb0a64d976972d87b0783eeb1ff88306cd1891f02"
```

an address on the given network

### Properties

| Name        | Type   | Required | Restrictions | Description                     |
| ----------- | ------ | -------- | ------------ | ------------------------------- |
| _anonymous_ | string | false    | none         | an address on the given network |

<h2 id="tocS_CustodySymbol">CustodySymbol</h2>
<!-- backwards compatibility -->
<a id="schemacustodysymbol"></a>
<a id="schema_CustodySymbol"></a>
<a id="tocScustodysymbol"></a>
<a id="tocscustodysymbol"></a>

```json
"USDC"
```

symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB

### Properties

| Name        | Type   | Required | Restrictions | Description                                                  |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------ |
| _anonymous_ | string | false    | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB |

<h2 id="tocS_CustodyFiatSymbol">CustodyFiatSymbol</h2>
<!-- backwards compatibility -->
<a id="schemacustodyfiatsymbol"></a>
<a id="schema_CustodyFiatSymbol"></a>
<a id="tocScustodyfiatsymbol"></a>
<a id="tocscustodyfiatsymbol"></a>

```json
"USD"
```

symbol representing fiat currency, e.g. USD, EUR

### Properties

| Name        | Type   | Required | Restrictions | Description                                      |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------ |
| _anonymous_ | string | false    | none         | symbol representing fiat currency, e.g. USD, EUR |

<h2 id="tocS_CustodyTransactionID">CustodyTransactionID</h2>
<!-- backwards compatibility -->
<a id="schemacustodytransactionid"></a>
<a id="schema_CustodyTransactionID"></a>
<a id="tocScustodytransactionid"></a>
<a id="tocscustodytransactionid"></a>

```json
"DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979"
```

unique identifier for tracking a withdrawal during signing and in history

### Properties

| Name        | Type   | Required | Restrictions | Description                                                               |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | unique identifier for tracking a withdrawal during signing and in history |

<h2 id="tocS_CustodyTransactionHistoryID">CustodyTransactionHistoryID</h2>
<!-- backwards compatibility -->
<a id="schemacustodytransactionhistoryid"></a>
<a id="schema_CustodyTransactionHistoryID"></a>
<a id="tocScustodytransactionhistoryid"></a>
<a id="tocscustodytransactionhistoryid"></a>

```json
"DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979"
```

unique identifier for tracking a deposit or withdrawal

### Properties

| Name        | Type   | Required | Restrictions | Description                                            |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------ |
| _anonymous_ | string | false    | none         | unique identifier for tracking a deposit or withdrawal |

<h2 id="tocS_CustodyDirection">CustodyDirection</h2>
<!-- backwards compatibility -->
<a id="schemacustodydirection"></a>
<a id="schema_CustodyDirection"></a>
<a id="tocScustodydirection"></a>
<a id="tocscustodydirection"></a>

```json
"DEPOSIT"
```

direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL'

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                     |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL' |

<h2 id="tocS_CustodyWithdrawalChallenge">CustodyWithdrawalChallenge</h2>
<!-- backwards compatibility -->
<a id="schemacustodywithdrawalchallenge"></a>
<a id="schema_CustodyWithdrawalChallenge"></a>
<a id="tocScustodywithdrawalchallenge"></a>
<a id="tocscustodywithdrawalchallenge"></a>

```json
"041f3105d6e20fc84399dece611f4e6dbf8ad59d51b0db7fd6acf518d38401d4"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_CustodyBankName">CustodyBankName</h2>
<!-- backwards compatibility -->
<a id="schemacustodybankname"></a>
<a id="schema_CustodyBankName"></a>
<a id="tocScustodybankname"></a>
<a id="tocscustodybankname"></a>

```json
"Silvergate Bank"
```

name of bank

### Properties

| Name        | Type   | Required | Restrictions | Description  |
| ----------- | ------ | -------- | ------------ | ------------ |
| _anonymous_ | string | false    | none         | name of bank |

<h2 id="tocS_CustodyPhysicalBankAddress">CustodyPhysicalBankAddress</h2>
<!-- backwards compatibility -->
<a id="schemacustodyphysicalbankaddress"></a>
<a id="schema_CustodyPhysicalBankAddress"></a>
<a id="tocScustodyphysicalbankaddress"></a>
<a id="tocscustodyphysicalbankaddress"></a>

```json
"4250 Executive Square Suite 300 La Jolla, CA 92037"
```

physical location of bank

### Properties

| Name        | Type   | Required | Restrictions | Description               |
| ----------- | ------ | -------- | ------------ | ------------------------- |
| _anonymous_ | string | false    | none         | physical location of bank |

<h2 id="tocS_CustodyBankAccountNumber">CustodyBankAccountNumber</h2>
<!-- backwards compatibility -->
<a id="schemacustodybankaccountnumber"></a>
<a id="schema_CustodyBankAccountNumber"></a>
<a id="tocScustodybankaccountnumber"></a>
<a id="tocscustodybankaccountnumber"></a>

```json
"9873481227"
```

bank account number

### Properties

| Name        | Type   | Required | Restrictions | Description         |
| ----------- | ------ | -------- | ------------ | ------------------- |
| _anonymous_ | string | false    | none         | bank account number |

<h2 id="tocS_CustodyBankNetworkID">CustodyBankNetworkID</h2>
<!-- backwards compatibility -->
<a id="schemacustodybanknetworkid"></a>
<a id="schema_CustodyBankNetworkID"></a>
<a id="tocScustodybanknetworkid"></a>
<a id="tocscustodybanknetworkid"></a>

```json
"SWIFT"
```

the fiat network, e.g. SWIFT, ABA or SEPA

### Properties

| Name        | Type   | Required | Restrictions | Description                               |
| ----------- | ------ | -------- | ------------ | ----------------------------------------- |
| _anonymous_ | string | false    | none         | the fiat network, e.g. SWIFT, ABA or SEPA |

<h2 id="tocS_CustodyBankRoutingCode">CustodyBankRoutingCode</h2>
<!-- backwards compatibility -->
<a id="schemacustodybankroutingcode"></a>
<a id="schema_CustodyBankRoutingCode"></a>
<a id="tocScustodybankroutingcode"></a>
<a id="tocscustodybankroutingcode"></a>

```json
"322286803"
```

routing code of bank

### Properties

| Name        | Type   | Required | Restrictions | Description          |
| ----------- | ------ | -------- | ------------ | -------------------- |
| _anonymous_ | string | false    | none         | routing code of bank |

<h2 id="tocS_CustodyQuantity">CustodyQuantity</h2>
<!-- backwards compatibility -->
<a id="schemacustodyquantity"></a>
<a id="schema_CustodyQuantity"></a>
<a id="tocScustodyquantity"></a>
<a id="tocscustodyquantity"></a>

```json
"100000.00"
```

total quantity of symbol to withdraw including fee in units of symbol, not in
smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received
will have fee subtracted.

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                                                                                                           |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted. |

<h2 id="tocS_CustodyWithdrawalFee">CustodyWithdrawalFee</h2>
<!-- backwards compatibility -->
<a id="schemacustodywithdrawalfee"></a>
<a id="schema_CustodyWithdrawalFee"></a>
<a id="tocScustodywithdrawalfee"></a>
<a id="tocscustodywithdrawalfee"></a>

```json
"3.00"
```

withdrawal fee charged in units of symbol, not in smaller denominations (e.g.
BTC not Satoshi, ETH not Wei)

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                                 |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |

<h2 id="tocS_CustodyDepositMemo">CustodyDepositMemo</h2>
<!-- backwards compatibility -->
<a id="schemacustodydepositmemo"></a>
<a id="schema_CustodyDepositMemo"></a>
<a id="tocScustodydepositmemo"></a>
<a id="tocscustodydepositmemo"></a>

```json
"925891241"
```

memo or destination tag used during deposit to help identify account to credit
funds to

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                             |
| ----------- | ------ | -------- | ------------ | --------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | memo or destination tag used during deposit to help identify account to credit funds to |

<h2 id="tocS_CustodyWithdrawalMemo">CustodyWithdrawalMemo</h2>
<!-- backwards compatibility -->
<a id="schemacustodywithdrawalmemo"></a>
<a id="schema_CustodyWithdrawalMemo"></a>
<a id="tocScustodywithdrawalmemo"></a>
<a id="tocscustodywithdrawalmemo"></a>

```json
"MZAXEMRXA"
```

memo or destination tag that will be used as a reference on transaction

### Properties

| Name        | Type   | Required | Restrictions | Description                                                             |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | memo or destination tag that will be used as a reference on transaction |

<h2 id="tocS_CustodyWithdrawalLabel">CustodyWithdrawalLabel</h2>
<!-- backwards compatibility -->
<a id="schemacustodywithdrawallabel"></a>
<a id="schema_CustodyWithdrawalLabel"></a>
<a id="tocScustodywithdrawallabel"></a>
<a id="tocscustodywithdrawallabel"></a>

```json
"Our cold wallet"
```

descriptive label of destination provided by user

### Properties

| Name        | Type   | Required | Restrictions | Description                                       |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------- |
| _anonymous_ | string | false    | none         | descriptive label of destination provided by user |

<h2 id="tocS_CustodyTransactionStatus">CustodyTransactionStatus</h2>
<!-- backwards compatibility -->
<a id="schemacustodytransactionstatus"></a>
<a id="schema_CustodyTransactionStatus"></a>
<a id="tocScustodytransactionstatus"></a>
<a id="tocscustodytransactionstatus"></a>

```json
"COMPLETE"
```

one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED'

### Properties

| Name        | Type   | Required | Restrictions | Description                                         |
| ----------- | ------ | -------- | ------------ | --------------------------------------------------- |
| _anonymous_ | string | false    | none         | one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED' |

<h2 id="tocS_CustodyTransactionDetails">CustodyTransactionDetails</h2>
<!-- backwards compatibility -->
<a id="schemacustodytransactiondetails"></a>
<a id="schema_CustodyTransactionDetails"></a>
<a id="tocScustodytransactiondetails"></a>
<a id="tocscustodytransactiondetails"></a>

```json
{
  "type": "object",
  "properties": {
    "address": {
      "type": "string",
      "description": "crypto network address",
      "example": "0xb0a64d976972d87b0783eeb1ff88306cd1891f02"
    },
    "blockchainTxId": {
      "type": "string",
      "description": "transaction id on chain",
      "example": "0xec557f2c7278d2dae2d98a27b9bd43f386789a4209090cbbd11595f1bed4a4c2"
    },
    "swiftUetr": {
      "type": "string",
      "description": "unique end-to-end-transaction reference for swift transactions",
      "example": "b55aa5cd-baa2-4122-8c17-ae9b856ae36a"
    }
  }
}
```

### Properties

| Name           | Type   | Required | Restrictions | Description                                                    |
| -------------- | ------ | -------- | ------------ | -------------------------------------------------------------- |
| address        | string | false    | none         | crypto network address                                         |
| blockchainTxId | string | false    | none         | transaction id on chain                                        |
| swiftUetr      | string | false    | none         | unique end-to-end-transaction reference for swift transactions |

<h2 id="tocS_CustodyAvailableWithdrawalLimit">CustodyAvailableWithdrawalLimit</h2>
<!-- backwards compatibility -->
<a id="schemacustodyavailablewithdrawallimit"></a>
<a id="schema_CustodyAvailableWithdrawalLimit"></a>
<a id="tocScustodyavailablewithdrawallimit"></a>
<a id="tocscustodyavailablewithdrawallimit"></a>

```json
"20000.0"
```

remaining limit on amount of coin or token that could be withdrawn now, in units
of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH
not Wei)

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                                                                                             |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | remaining limit on amount of coin or token that could be withdrawn now, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |

<h2 id="tocS_Custody24HWithdrawalLimit">Custody24HWithdrawalLimit</h2>
<!-- backwards compatibility -->
<a id="schemacustody24hwithdrawallimit"></a>
<a id="schema_Custody24HWithdrawalLimit"></a>
<a id="tocScustody24hwithdrawallimit"></a>
<a id="tocscustody24hwithdrawallimit"></a>

```json
"1000000.00"
```

limit on amount of coin or token that can be withdrawn over a 24 hour period, in
units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi,
ETH not Wei)

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                                                                                                   |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | limit on amount of coin or token that can be withdrawn over a 24 hour period, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |

<h2 id="tocS_CustodyCreatedAtDateTime">CustodyCreatedAtDateTime</h2>
<!-- backwards compatibility -->
<a id="schemacustodycreatedatdatetime"></a>
<a id="schema_CustodyCreatedAtDateTime"></a>
<a id="tocScustodycreatedatdatetime"></a>
<a id="tocscustodycreatedatdatetime"></a>

```json
"2022-09-16T07:56:15.000Z"
```

time of initial transaction

### Properties

| Name        | Type   | Required | Restrictions | Description                 |
| ----------- | ------ | -------- | ------------ | --------------------------- |
| _anonymous_ | string | false    | none         | time of initial transaction |

<h2 id="tocS_CustodyUpdatedAtDateTime">CustodyUpdatedAtDateTime</h2>
<!-- backwards compatibility -->
<a id="schemacustodyupdatedatdatetime"></a>
<a id="schema_CustodyUpdatedAtDateTime"></a>
<a id="tocScustodyupdatedatdatetime"></a>
<a id="tocscustodyupdatedatdatetime"></a>

```json
"2022-09-16T07:59:23.000Z"
```

last updated time of transaction

### Properties

| Name        | Type   | Required | Restrictions | Description                      |
| ----------- | ------ | -------- | ------------ | -------------------------------- |
| _anonymous_ | string | false    | none         | last updated time of transaction |

<h2 id="tocS_PublicKey">PublicKey</h2>
<!-- backwards compatibility -->
<a id="schemapublickey"></a>
<a id="schema_PublicKey"></a>
<a id="tocSpublickey"></a>
<a id="tocspublickey"></a>

```json
"PUB_R1_6PTdfWbXvXWQduhcCiRooGHTVpriu15xMqfr7EDq6sWLDj7NjS"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_Signature">Signature</h2>
<!-- backwards compatibility -->
<a id="schemasignature"></a>
<a id="schema_Signature"></a>
<a id="tocSsignature"></a>
<a id="tocssignature"></a>

```json
"SIG_R1_K35d5hSY5FbNoJwrCfmH6QvPG7m9XmhL2mgWYcSB7q2hKJ2mv39Luck9WBJroSB635ZAXhdg36TYG7QJX1fTidbsMvyE8N"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

<h2 id="tocS_SpotAccount">SpotAccount</h2>
<!-- backwards compatibility -->
<a id="schemaspotaccount"></a>
<a id="schema_SpotAccount"></a>
<a id="tocSspotaccount"></a>
<a id="tocsspotaccount"></a>

```json
{
  "type": "object",
  "required": [
    "accountId",
    "type",
    "symbol",
    "total",
    "free",
    "used",
    "updatedAtDatetime",
    "updatedAtTimestamp"
  ],
  "properties": {
    "accountId": {
      "description": "unique spot account ID",
      "allOf": [
        {
          "type": "string",
          "example": "1"
        }
      ]
    },
    "type": {
      "description": "Spot Account",
      "allOf": [
        {
          "type": "string",
          "description": "Type of Account",
          "example": "spot"
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
    "total": {
      "description": "total is `free` + `used` assets within the account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "free": {
      "description": "refers to the assets that are available to use on the account excluding borrowed assets, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "used": {
      "description": "refers to the assets that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "updatedAtDatetime": {
      "description": "denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the AMM instruction was updated by the exchange",
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
```

### Properties

| Name               | Type                                          | Required | Restrictions | Description                                                                                                                                                |
| ------------------ | --------------------------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| accountId          | [SpotAccountID](#schemaspotaccountid)         | true     | none         | unique spot account ID                                                                                                                                     |
| type               | string                                        | true     | none         | Spot Account                                                                                                                                               |
| symbol             | [AssetSymbol](#schemaassetsymbol)             | true     | none         | asset symbol                                                                                                                                               |
| total              | [AssetValue](#schemaassetvalue)               | true     | none         | total is `free` + `used` assets within the account, see [asset value](#overview--price-and-quantity-precision) format                                      |
| free               | [AssetValue](#schemaassetvalue)               | true     | none         | refers to the assets that are available to use on the account excluding borrowed assets, see [asset value](#overview--price-and-quantity-precision) format |
| used               | [AssetValue](#schemaassetvalue)               | true     | none         | refers to the assets that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format              |
| updatedAtDatetime  | [DateTime](#schemadatetime)                   | true     | none         | denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string                                                      |
| updatedAtTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time the AMM instruction was updated by the exchange                                                                                           |

<h2 id="tocS_AssetAccount">AssetAccount</h2>
<!-- backwards compatibility -->
<a id="schemaassetaccount"></a>
<a id="schema_AssetAccount"></a>
<a id="tocSassetaccount"></a>
<a id="tocsassetaccount"></a>

```json
{
  "type": "object",
  "required": [
    "tradingAccountId",
    "assetId",
    "assetSymbol",
    "availableQuantity",
    "borrowedQuantity",
    "lockedQuantity",
    "loanedQuantity",
    "updatedAtDatetime",
    "updatedAtTimestamp"
  ],
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "assetId": {
      "description": "asset ID",
      "allOf": [
        {
          "type": "string",
          "description": "unique asset ID",
          "example": "1"
        }
      ]
    },
    "assetSymbol": {
      "description": "asset symbol",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "availableQuantity": {
      "description": "the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedQuantity": {
      "description": "the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "lockedQuantity": {
      "description": "the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "loanedQuantity": {
      "description": "the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "updatedAtDatetime": {
      "description": "denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the AMM instruction was updated by the exchange",
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
```

### Properties

| Name               | Type                                          | Required | Restrictions | Description                                                                                                                                        |
| ------------------ | --------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| tradingAccountId   | [TradingAccountId](#schematradingaccountid)   | true     | none         | unique trading account ID                                                                                                                          |
| assetId            | [AssetID](#schemaassetid)                     | true     | none         | asset ID                                                                                                                                           |
| assetSymbol        | [AssetSymbol](#schemaassetsymbol)             | true     | none         | asset symbol                                                                                                                                       |
| availableQuantity  | [AssetValue](#schemaassetvalue)               | true     | none         | the assets that are available to use on the account, see [asset value](#overview--price-and-quantity-precision) format                             |
| borrowedQuantity   | [AssetValue](#schemaassetvalue)               | true     | none         | the assets on the account that are borrowed, see [asset value](#overview--price-and-quantity-precision) format                                     |
| lockedQuantity     | [AssetValue](#schemaassetvalue)               | true     | none         | the assets on the account that are locked in orders, loans and AMM instructions, see [asset value](#overview--price-and-quantity-precision) format |
| loanedQuantity     | [AssetValue](#schemaassetvalue)               | true     | none         | the assets on the account that are being loaned, see [asset value](#overview--price-and-quantity-precision) format                                 |
| updatedAtDatetime  | [DateTime](#schemadatetime)                   | true     | none         | denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string                                              |
| updatedAtTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time the AMM instruction was updated by the exchange                                                                                   |

<h2 id="tocS_AmendOrderRequest">AmendOrderRequest</h2>
<!-- backwards compatibility -->
<a id="schemaamendorderrequest"></a>
<a id="schema_AmendOrderRequest"></a>
<a id="tocSamendorderrequest"></a>
<a id="tocsamendorderrequest"></a>

```json
{
  "type": "object",
  "required": ["quantity"],
  "properties": {
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
        }
      ]
    },
    "handle": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "symbol": {
      "description": "symbol",
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "price": {
      "description": "updated price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "stopPrice": {
      "description": "updated stop price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantity": {
      "description": "updated quantity, see [asset value](#overview--price-and-quantity-precision) format",
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
```

### Properties

| Name      | Type                                | Required | Restrictions | Description                                                                              |
| --------- | ----------------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------------- |
| orderId   | [OrderID](#schemaorderid)           | false    | none         | unique order ID                                                                          |
| handle    | [OrderHandle](#schemaorderhandle)   | false    | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value |
| symbol    | [MarketSymbol](#schemamarketsymbol) | false    | none         | symbol                                                                                   |
| price     | [AssetValue](#schemaassetvalue)     | false    | none         | updated price, see [asset value](#overview--price-and-quantity-precision) format         |
| stopPrice | [AssetValue](#schemaassetvalue)     | false    | none         | updated stop price, see [asset value](#overview--price-and-quantity-precision) format    |
| quantity  | [AssetValue](#schemaassetvalue)     | true     | none         | updated quantity, see [asset value](#overview--price-and-quantity-precision) format      |

<h2 id="tocS_CreateOrderResponse">CreateOrderResponse</h2>
<!-- backwards compatibility -->
<a id="schemacreateorderresponse"></a>
<a id="schema_CreateOrderResponse"></a>
<a id="tocScreateorderresponse"></a>
<a id="tocscreateorderresponse"></a>

```json
{
  "type": "object",
  "required": ["message", "requestId", "orderId", "test"],
  "properties": {
    "message": {
      "description": "message",
      "type": "string",
      "example": "Command acknowledged - CreateOrder"
    },
    "requestId": {
      "description": "unique request ID",
      "allOf": [
        {
          "type": "string",
          "example": "197735387747975680"
        }
      ]
    },
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
        }
      ]
    }
  }
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |
| orderId   | [OrderID](#schemaorderid)     | true     | none         | unique order ID   |

<h2 id="tocS_CreateOrderCommandResponseV3">CreateOrderCommandResponseV3</h2>
<!-- backwards compatibility -->
<a id="schemacreateordercommandresponsev3"></a>
<a id="schema_CreateOrderCommandResponseV3"></a>
<a id="tocScreateordercommandresponsev3"></a>
<a id="tocscreateordercommandresponsev3"></a>

```json
{
  "message": "Command acknowledged - CreateOrder",
  "requestId": "633910976353665024",
  "orderId": "633910775316480001",
  "clientOrderId": "1234567",
  "x-widdershins-oldRef": "#/components/schemas/CreateOrderCommandResponseV3/example"
}
```

### Properties

| Name          | Type                          | Required | Restrictions | Description                                                                        |
| ------------- | ----------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------- |
| message       | string                        | true     | none         | message                                                                            |
| requestId     | [RequestID](#schemarequestid) | true     | none         | unique request ID                                                                  |
| orderId       | [OrderID](#schemaorderid)     | true     | none         | unique order ID                                                                    |
| clientOrderId | string                        | false    | none         | unique numeric identifier generated on the client side expressed as a string value |

<h2 id="tocS_AmendOrderCommandResponseV1">AmendOrderCommandResponseV1</h2>
<!-- backwards compatibility -->
<a id="schemaamendordercommandresponsev1"></a>
<a id="schema_AmendOrderCommandResponseV1"></a>
<a id="tocSamendordercommandresponsev1"></a>
<a id="tocsamendordercommandresponsev1"></a>

```json
{
  "message": "Command acknowledged - AmendOrder",
  "requestId": "633910976353665024",
  "orderId": "633910775316480001",
  "clientOrderId": "1234567-1"
}
```

### Properties

| Name          | Type                          | Required | Restrictions | Description                                             |
| ------------- | ----------------------------- | -------- | ------------ | ------------------------------------------------------- |
| message       | string                        | true     | none         | message                                                 |
| requestId     | [RequestID](#schemarequestid) | true     | none         | unique request ID                                       |
| orderId       | [OrderID](#schemaorderid)     | true     | none         | unique order ID                                         |
| clientOrderId | string                        | false    | none         | Will be updated as part of a successful order amendment |

<h2 id="tocS_CancelOrderCommandResponseV3">CancelOrderCommandResponseV3</h2>
<!-- backwards compatibility -->
<a id="schemacancelordercommandresponsev3"></a>
<a id="schema_CancelOrderCommandResponseV3"></a>
<a id="tocScancelordercommandresponsev3"></a>
<a id="tocscancelordercommandresponsev3"></a>

```json
{
  "message": "Command acknowledged - CancelOrder",
  "requestId": "633910976353665024",
  "orderId": "633910775316480001"
}
```

### Properties

| Name          | Type                          | Required | Restrictions | Description                                                                        |
| ------------- | ----------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------- |
| message       | string                        | true     | none         | message                                                                            |
| requestId     | [RequestID](#schemarequestid) | true     | none         | unique request ID                                                                  |
| orderId       | [OrderID](#schemaorderid)     | false    | none         | unique order ID                                                                    |
| clientOrderId | string                        | false    | none         | unique numeric identifier generated on the client side expressed as a string value |

<h2 id="tocS_CreateOrderCommand">CreateOrderCommand</h2>
<!-- backwards compatibility -->
<a id="schemacreateordercommand"></a>
<a id="schema_CreateOrderCommand"></a>
<a id="tocScreateordercommand"></a>
<a id="tocscreateordercommand"></a>

```json
{
  "type": "object",
  "required": [
    "commandType",
    "handle",
    "symbol",
    "type",
    "side",
    "price",
    "stopPrice",
    "quantity",
    "allowMargin",
    "timeInForce",
    "tradingAccountId"
  ],
  "properties": {
    "commandType": {
      "description": "The command type, it must be 'V2CreateOrder'",
      "type": "string",
      "example": "V2CreateOrder"
    },
    "handle": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "type": {
      "allOf": [
        {
          "type": "string",
          "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
          "example": "LMT"
        }
      ],
      "example": "LMT"
    },
    "side": {
      "allOf": [
        {
          "type": "string",
          "description": "order side can have the following string values `\"BUY\"`, `\"SELL\"`",
          "example": "BUY"
        }
      ],
      "example": "BUY"
    },
    "price": {
      "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "stopPrice": {
      "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantity": {
      "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "timeInForce": {
      "allOf": [
        {
          "type": "string",
          "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
        }
      ],
      "example": "GTC"
    },
    "allowMargin": {
      "description": "allows to borrow on the order",
      "type": "boolean",
      "example": false
    },
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                                        | Required | Restrictions | Description                                                                                                               |
| ---------------- | ----------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| commandType      | string                                                      | true     | none         | The command type, it must be 'V2CreateOrder'                                                                              |
| handle           | [OrderHandle](#schemaorderhandle)                           | true     | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value                                  |
| symbol           | [MarketSymbol](#schemamarketsymbol)                         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market                                             |
| type             | [OrderTypeAsString](#schemaordertypeasstring)               | true     | none         | order type can have the following string values `"LMT"`, `"MKT"`, `"STOP_LIMIT"`, `"POST_ONLY"`.                          |
| side             | [OrderSideAsString](#schemaordersideasstring)               | true     | none         | order side can have the following string values `"BUY"`, `"SELL"`                                                         |
| price            | [AssetValue](#schemaassetvalue)                             | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                                  |
| stopPrice        | [AssetValue](#schemaassetvalue)                             | true     | none         | stop price, see [asset value](#overview--price-and-quantity-precision) format                                             |
| quantity         | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                               |
| timeInForce      | [OrderTimeInForceAsString](#schemaordertimeinforceasstring) | true     | none         | time in force can have the following string values `"GTC"`, `"FOK"`, `"IOC"`, see [details](#overview--order-timeinforce) |
| allowMargin      | boolean                                                     | true     | none         | allows to borrow on the order                                                                                             |
| tradingAccountId | [TradingAccountId](#schematradingaccountid)                 | true     | none         | unique trading account ID                                                                                                 |

<h2 id="tocS_CreateOrderCommandV3">CreateOrderCommandV3</h2>
<!-- backwards compatibility -->
<a id="schemacreateordercommandv3"></a>
<a id="schema_CreateOrderCommandV3"></a>
<a id="tocScreateordercommandv3"></a>
<a id="tocscreateordercommandv3"></a>

```json
{
  "type": "object",
  "required": [
    "commandType",
    "symbol",
    "type",
    "side",
    "quantity",
    "timeInForce",
    "tradingAccountId"
  ],
  "properties": {
    "commandType": {
      "description": "The command type, it must be 'V3CreateOrder'",
      "type": "string"
    },
    "clientOrderId": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "type": {
      "allOf": [
        {
          "type": "string",
          "description": "order type can have the following string values `\"LIMIT\"`, `\"MARKET\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
          "example": "LIMIT"
        }
      ]
    },
    "side": {
      "allOf": [
        {
          "type": "string",
          "description": "order side can have the following string values `\"BUY\"`, `\"SELL\"`",
          "example": "BUY"
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
    "stopPrice": {
      "description": "stop price",
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
    "timeInForce": {
      "allOf": [
        {
          "type": "string",
          "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
        }
      ]
    },
    "allowBorrow": {
      "description": "allows to borrow on the order",
      "type": "boolean",
      "default": false,
      "example": false
    },
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                                        | Required | Restrictions | Description                                                                                                               |
| ---------------- | ----------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| commandType      | string                                                      | true     | none         | The command type, it must be 'V3CreateOrder'                                                                              |
| clientOrderId    | [OrderHandle](#schemaorderhandle)                           | false    | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value                                  |
| symbol           | [MarketSymbol](#schemamarketsymbol)                         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market                                             |
| type             | [OrderTypeAsStringV2](#schemaordertypeasstringv2)           | true     | none         | order type can have the following string values `"LIMIT"`, `"MARKET"`, `"STOP_LIMIT"`, `"POST_ONLY"`.                     |
| side             | [OrderSideAsString](#schemaordersideasstring)               | true     | none         | order side can have the following string values `"BUY"`, `"SELL"`                                                         |
| price            | [AssetValue](#schemaassetvalue)                             | false    | none         | price                                                                                                                     |
| stopPrice        | [AssetValue](#schemaassetvalue)                             | false    | none         | stop price                                                                                                                |
| quantity         | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity                                                                                                                  |
| timeInForce      | [OrderTimeInForceAsString](#schemaordertimeinforceasstring) | true     | none         | time in force can have the following string values `"GTC"`, `"FOK"`, `"IOC"`, see [details](#overview--order-timeinforce) |
| allowBorrow      | boolean                                                     | false    | none         | allows to borrow on the order                                                                                             |
| tradingAccountId | [TradingAccountId](#schematradingaccountid)                 | true     | none         | unique trading account ID                                                                                                 |

<h2 id="tocS_AmendOrderV1">AmendOrderV1</h2>
<!-- backwards compatibility -->
<a id="schemaamendorderv1"></a>
<a id="schema_AmendOrderV1"></a>
<a id="tocSamendorderv1"></a>
<a id="tocsamendorderv1"></a>

```json
{
  "type": "object",
  "required": ["commandType", "symbol", "tradingAccountId"],
  "properties": {
    "commandType": {
      "description": "The command type, it must be 'V1AmendOrder'",
      "type": "string"
    },
    "orderId": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "type": {
      "allOf": [
        {
          "type": "string",
          "description": "order type can have the following string values `\"LIMIT\"`, `\"MARKET\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
          "example": "LIMIT"
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
    "clientOrderId": {
      "description": "unique numeric identifier generated on the client side expressed as a string value",
      "type": "string"
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
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                              | Required | Restrictions | Description                                                                                           |
| ---------------- | ------------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------- |
| commandType      | string                                            | true     | none         | The command type, it must be 'V1AmendOrder'                                                           |
| orderId          | [OrderHandle](#schemaorderhandle)                 | false    | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value              |
| symbol           | [MarketSymbol](#schemamarketsymbol)               | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market                         |
| type             | [OrderTypeAsStringV2](#schemaordertypeasstringv2) | false    | none         | order type can have the following string values `"LIMIT"`, `"MARKET"`, `"STOP_LIMIT"`, `"POST_ONLY"`. |
| price            | [AssetValue](#schemaassetvalue)                   | false    | none         | price                                                                                                 |
| clientOrderId    | string                                            | false    | none         | unique numeric identifier generated on the client side expressed as a string value                    |
| quantity         | [AssetValue](#schemaassetvalue)                   | false    | none         | quantity                                                                                              |
| tradingAccountId | [TradingAccountId](#schematradingaccountid)       | true     | none         | unique trading account ID                                                                             |

<h2 id="tocS_TradingAccountResponse">TradingAccountResponse</h2>
<!-- backwards compatibility -->
<a id="schematradingaccountresponse"></a>
<a id="schema_TradingAccountResponse"></a>
<a id="tocStradingaccountresponse"></a>
<a id="tocstradingaccountresponse"></a>

```json
{
  "type": "object",
  "required": [
    "isBorrowing",
    "isLending",
    "isPrimaryAccount",
    "maxInitialLeverage",
    "rateLimitToken",
    "tradingAccountDescription",
    "tradingAccountId",
    "tradingAccountName",
    "isDefaulted",
    "riskLimitUSD",
    "totalBorrowedUSD",
    "totalCollateralUSD",
    "initialMarginUSD",
    "warningMarginUSD",
    "liquidationMarginUSD",
    "fullLiquidationMarginUSD",
    "defaultedMarginUSD",
    "endCustomerId",
    "isConcentrationRiskEnabled",
    "liquidityAddonUSD",
    "marketRiskUSD",
    "marginProfile",
    "totalLiabilitiesUSD",
    "tradeFeeRate"
  ],
  "properties": {
    "isBorrowing": {
      "description": "whether the trading account is borrowing",
      "type": "string",
      "example": "false"
    },
    "isLending": {
      "description": "whether the trading account is lending",
      "type": "string",
      "example": "false"
    },
    "makerFee": {
      "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
      "type": "string",
      "example": "0.00000000",
      "deprecated": true
    },
    "takerFee": {
      "description": "Deprecated and no longer accurate. See `tradeFeeRate` instead",
      "type": "string",
      "example": "0.00020000",
      "deprecated": true
    },
    "maxInitialLeverage": {
      "description": "max initial leverage",
      "type": "string",
      "example": "1"
    },
    "tradingAccountId": {
      "description": "id of the trading account",
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "tradingAccountName": {
      "description": "name of the trading account",
      "type": "string",
      "example": "algo trading account"
    },
    "tradingAccountDescription": {
      "description": "description of the trading account",
      "type": "string",
      "example": "algo trading account with experimental strategy"
    },
    "isPrimaryAccount": {
      "description": "whether this is the primary account",
      "type": "string",
      "example": "false"
    },
    "rateLimitToken": {
      "description": "unique rate limit token of the trading account",
      "type": "string",
      "example": "97d98951b12fb11f330dd9cb1b807d888c702679ee602edcf1ebc6bac17ad63d"
    },
    "isDefaulted": {
      "description": "whether the trading account is defaulted",
      "type": "String",
      "example": "false"
    },
    "tradeFeeRate": {
      "description": "Trade fees per `feeGroupId` for this trading account",
      "type": "array",
      "minItems": 0,
      "items": {
        "allOf": [
          {
            "type": "object",
            "required": ["feeGroupId", "makerFee", "takerFee"],
            "properties": {
              "feeGroupId": {
                "type": "integer",
                "description": "Identifier for this particular fee tier",
                "example": 1
              },
              "makerFee": {
                "type": "string",
                "description": "Maker Fee in basis points (bps)",
                "example": "10"
              },
              "takerFee": {
                "type": "string",
                "description": "Taker Fee in basis points (bps)",
                "example": "10"
              }
            }
          }
        ]
      }
    },
    "riskLimitUSD": {
      "description": "The maximum allowed borrowing for this trading account (in USD currency)",
      "type": "string",
      "example": "10000.0000"
    },
    "totalLiabilitiesUSD": {
      "description": "The The total liabilities for this trading account (in USD currency)",
      "type": "string",
      "example": "14000.0000"
    },
    "totalBorrowedUSD": {
      "description": "total borrowed across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "12000.0000"
    },
    "totalCollateralUSD": {
      "description": "total collateral across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "13000.0000"
    },
    "initialMarginUSD": {
      "description": "The minimum margin one must maintain in order to be able to purposefully increase risk",
      "type": "string",
      "example": "0000.0000"
    },
    "warningMarginUSD": {
      "description": "The minimum margin when the customer will receive warning via email/notifications over UI",
      "type": "string",
      "example": "0000.0000"
    },
    "liquidationMarginUSD": {
      "description": "The minimum value of margin one must maintain in order to avoid liquidation",
      "type": "string",
      "example": "0000.0000"
    },
    "fullLiquidationMarginUSD": {
      "description": "The value of margin when full liquidation occurs",
      "type": "string",
      "example": "0000.0000"
    },
    "defaultedMarginUSD": {
      "description": "The value of margin when this trading account will be moved into a Defaulted state",
      "type": "string",
      "example": "0000.0000"
    },
    "endCustomerId": {
      "description": "The end customer id used for self trade prevention (default is institution id, max 32 characters)",
      "type": "string",
      "example": "PrimeBroker"
    },
    "isConcentrationRiskEnabled": {
      "description": "whether concentration risk checks are enforced for an account when sending new orders. By default, concentration risk checks will apply to all accounts",
      "type": "string",
      "example": true
    },
    "liquidityAddonUSD": {
      "description": "expected market impact of unwinding the portfolio in the case of a liquidation event",
      "type": "string",
      "example": "1000.0000"
    },
    "marketRiskUSD": {
      "description": "the worst possible loss on the portfolio based on scenario analysis",
      "type": "string",
      "example": "2000.0000"
    },
    "marginProfile": {
      "description": "Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values",
      "allOf": [
        {
          "properties": {
            "initialMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate initial margin requirement of the account",
              "type": "string",
              "example": "200.00"
            },
            "warningMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate warning margin requirement of the account",
              "type": "string",
              "example": "150.00"
            },
            "liquidationMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate liquidation margin requirement of the account",
              "type": "string",
              "example": "100.00"
            },
            "fullLiquidationMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate full liquidation margin requirement of the account",
              "type": "string",
              "example": "75.00"
            },
            "defaultedMarketRiskMultiplierPct": {
              "description": "market risk multiplier used to calculate defaulted margin requirement of the account",
              "type": "string",
              "example": "50.00"
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name                       | Type                                        | Required | Restrictions | Description                                                                                                                                             |
| -------------------------- | ------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| isBorrowing                | string                                      | true     | none         | whether the trading account is borrowing                                                                                                                |
| isLending                  | string                                      | true     | none         | whether the trading account is lending                                                                                                                  |
| makerFee                   | string                                      | false    | none         | Deprecated and no longer accurate. See `tradeFeeRate` instead                                                                                           |
| takerFee                   | string                                      | false    | none         | Deprecated and no longer accurate. See `tradeFeeRate` instead                                                                                           |
| maxInitialLeverage         | string                                      | true     | none         | max initial leverage                                                                                                                                    |
| tradingAccountId           | [TradingAccountId](#schematradingaccountid) | true     | none         | id of the trading account                                                                                                                               |
| tradingAccountName         | string                                      | true     | none         | name of the trading account                                                                                                                             |
| tradingAccountDescription  | string                                      | true     | none         | description of the trading account                                                                                                                      |
| isPrimaryAccount           | string                                      | true     | none         | whether this is the primary account                                                                                                                     |
| rateLimitToken             | string                                      | true     | none         | unique rate limit token of the trading account                                                                                                          |
| isDefaulted                | String                                      | true     | none         | whether the trading account is defaulted                                                                                                                |
| tradeFeeRate               | [allOf]                                     | true     | none         | Trade fees per `feeGroupId` for this trading account                                                                                                    |
| riskLimitUSD               | string                                      | true     | none         | The maximum allowed borrowing for this trading account (in USD currency)                                                                                |
| totalLiabilitiesUSD        | string                                      | true     | none         | The The total liabilities for this trading account (in USD currency)                                                                                    |
| totalBorrowedUSD           | string                                      | true     | none         | total borrowed across all assets in this trading account displayed in the reference asset in USD                                                        |
| totalCollateralUSD         | string                                      | true     | none         | total collateral across all assets in this trading account displayed in the reference asset in USD                                                      |
| initialMarginUSD           | string                                      | true     | none         | The minimum margin one must maintain in order to be able to purposefully increase risk                                                                  |
| warningMarginUSD           | string                                      | true     | none         | The minimum margin when the customer will receive warning via email/notifications over UI                                                               |
| liquidationMarginUSD       | string                                      | true     | none         | The minimum value of margin one must maintain in order to avoid liquidation                                                                             |
| fullLiquidationMarginUSD   | string                                      | true     | none         | The value of margin when full liquidation occurs                                                                                                        |
| defaultedMarginUSD         | string                                      | true     | none         | The value of margin when this trading account will be moved into a Defaulted state                                                                      |
| endCustomerId              | string                                      | true     | none         | The end customer id used for self trade prevention (default is institution id, max 32 characters)                                                       |
| isConcentrationRiskEnabled | string                                      | true     | none         | whether concentration risk checks are enforced for an account when sending new orders. By default, concentration risk checks will apply to all accounts |
| liquidityAddonUSD          | string                                      | true     | none         | expected market impact of unwinding the portfolio in the case of a liquidation event                                                                    |
| marketRiskUSD              | string                                      | true     | none         | the worst possible loss on the portfolio based on scenario analysis                                                                                     |
| marginProfile              | [MarginProfile](#schemamarginprofile)       | true     | none         | Contains the market risk multipliers applied to a trading account to derive the five individual Margin Requirement values                               |

<h2 id="tocS_MarginProfile">MarginProfile</h2>
<!-- backwards compatibility -->
<a id="schemamarginprofile"></a>
<a id="schema_MarginProfile"></a>
<a id="tocSmarginprofile"></a>
<a id="tocsmarginprofile"></a>

```json
{
  "properties": {
    "initialMarketRiskMultiplierPct": {
      "description": "market risk multiplier used to calculate initial margin requirement of the account",
      "type": "string",
      "example": "200.00"
    },
    "warningMarketRiskMultiplierPct": {
      "description": "market risk multiplier used to calculate warning margin requirement of the account",
      "type": "string",
      "example": "150.00"
    },
    "liquidationMarketRiskMultiplierPct": {
      "description": "market risk multiplier used to calculate liquidation margin requirement of the account",
      "type": "string",
      "example": "100.00"
    },
    "fullLiquidationMarketRiskMultiplierPct": {
      "description": "market risk multiplier used to calculate full liquidation margin requirement of the account",
      "type": "string",
      "example": "75.00"
    },
    "defaultedMarketRiskMultiplierPct": {
      "description": "market risk multiplier used to calculate defaulted margin requirement of the account",
      "type": "string",
      "example": "50.00"
    }
  }
}
```

### Properties

| Name                                   | Type   | Required | Restrictions | Description                                                                                 |
| -------------------------------------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------- |
| initialMarketRiskMultiplierPct         | string | false    | none         | market risk multiplier used to calculate initial margin requirement of the account          |
| warningMarketRiskMultiplierPct         | string | false    | none         | market risk multiplier used to calculate warning margin requirement of the account          |
| liquidationMarketRiskMultiplierPct     | string | false    | none         | market risk multiplier used to calculate liquidation margin requirement of the account      |
| fullLiquidationMarketRiskMultiplierPct | string | false    | none         | market risk multiplier used to calculate full liquidation margin requirement of the account |
| defaultedMarketRiskMultiplierPct       | string | false    | none         | market risk multiplier used to calculate defaulted margin requirement of the account        |

<h2 id="tocS_CreateAMMInstructionResponse">CreateAMMInstructionResponse</h2>
<!-- backwards compatibility -->
<a id="schemacreateamminstructionresponse"></a>
<a id="schema_CreateAMMInstructionResponse"></a>
<a id="tocScreateamminstructionresponse"></a>
<a id="tocscreateamminstructionresponse"></a>

```json
{
  "type": "object",
  "required": ["message", "requestId", "liquidityId", "test"],
  "properties": {
    "message": {
      "description": "message",
      "type": "string",
      "example": "Command acknowledged - AddLiquidity"
    },
    "requestId": {
      "description": "unique request ID",
      "allOf": [
        {
          "type": "string",
          "example": "197735387747975680"
        }
      ]
    },
    "liquidityId": {
      "description": "unique AMM instruction ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
        }
      ]
    }
  }
}
```

### Properties

| Name        | Type                                        | Required | Restrictions | Description               |
| ----------- | ------------------------------------------- | -------- | ------------ | ------------------------- |
| message     | string                                      | true     | none         | message                   |
| requestId   | [RequestID](#schemarequestid)               | true     | none         | unique request ID         |
| liquidityId | [AMMInstructionID](#schemaamminstructionid) | true     | none         | unique AMM instruction ID |

<h2 id="tocS_CreateAMMInstructionCommandResponseV3">CreateAMMInstructionCommandResponseV3</h2>
<!-- backwards compatibility -->
<a id="schemacreateamminstructioncommandresponsev3"></a>
<a id="schema_CreateAMMInstructionCommandResponseV3"></a>
<a id="tocScreateamminstructioncommandresponsev3"></a>
<a id="tocscreateamminstructioncommandresponsev3"></a>

```json
{
  "message": "Command acknowledged - CreateAMMInstruction",
  "requestId": "633906221577404416",
  "instructionId": "633906221577404424"
}
```

### Properties

| Name          | Type                                        | Required | Restrictions | Description               |
| ------------- | ------------------------------------------- | -------- | ------------ | ------------------------- |
| message       | string                                      | true     | none         | message                   |
| requestId     | [RequestID](#schemarequestid)               | true     | none         | unique request ID         |
| instructionId | [AMMInstructionID](#schemaamminstructionid) | true     | none         | unique AMM instruction ID |

<h2 id="tocS_TerminateAMMInstructionCommandResponseV3">TerminateAMMInstructionCommandResponseV3</h2>
<!-- backwards compatibility -->
<a id="schematerminateamminstructioncommandresponsev3"></a>
<a id="schema_TerminateAMMInstructionCommandResponseV3"></a>
<a id="tocSterminateamminstructioncommandresponsev3"></a>
<a id="tocsterminateamminstructioncommandresponsev3"></a>

```json
{
  "message": "Command acknowledged - TerminateAMMInstruction",
  "requestId": "633906221577404416",
  "instructionId": "633906221577404424"
}
```

### Properties

| Name          | Type                                        | Required | Restrictions | Description               |
| ------------- | ------------------------------------------- | -------- | ------------ | ------------------------- |
| message       | string                                      | true     | none         | message                   |
| requestId     | [RequestID](#schemarequestid)               | true     | none         | unique request ID         |
| instructionId | [AMMInstructionID](#schemaamminstructionid) | true     | none         | unique AMM instruction ID |

<h2 id="tocS_BadOrderEntryResponse">BadOrderEntryResponse</h2>
<!-- backwards compatibility -->
<a id="schemabadorderentryresponse"></a>
<a id="schema_BadOrderEntryResponse"></a>
<a id="tocSbadorderentryresponse"></a>
<a id="tocsbadorderentryresponse"></a>

```json
{
  "type": "object",
  "required": ["message", "errorCode", "errorCodeName"],
  "properties": {
    "message": {
      "description": "message",
      "type": "string",
      "example": "Missing signature header"
    },
    "errorCode": {
      "description": "unique error code",
      "type": "integer",
      "example": 6029
    },
    "errorCodeName": {
      "description": "unique error code name",
      "type": "string",
      "example": "MISSING_SIGNATURE_HEADER"
    }
  }
}
```

### Properties

| Name          | Type    | Required | Restrictions | Description            |
| ------------- | ------- | -------- | ------------ | ---------------------- |
| message       | string  | true     | none         | message                |
| errorCode     | integer | true     | none         | unique error code      |
| errorCodeName | string  | true     | none         | unique error code name |

<h2 id="tocS_CreateAMMInstructionCommand">CreateAMMInstructionCommand</h2>
<!-- backwards compatibility -->
<a id="schemacreateamminstructioncommand"></a>
<a id="schema_CreateAMMInstructionCommand"></a>
<a id="tocScreateamminstructioncommand"></a>
<a id="tocscreateamminstructioncommand"></a>

```json
{
  "type": "object",
  "required": [
    "commandType",
    "symbol",
    "baseQuantity",
    "quoteQuantity",
    "upperBound",
    "lowerBound",
    "feeTierId",
    "tradingAccountId"
  ],
  "properties": {
    "commandType": {
      "description": "The command type, it must be 'V2AddLiquidity'",
      "type": "string",
      "example": "V2AddLiquidity"
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "baseQuantity": {
      "description": "base quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteQuantity": {
      "description": "quote quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "upperBound": {
      "type": "string",
      "description": "upper bound of price range, in quote currency",
      "example": "14000.0000"
    },
    "lowerBound": {
      "type": "string",
      "description": "lower bound of price range, in quote currency",
      "example": "12000.0000"
    },
    "feeTierId": {
      "allOf": [
        {
          "type": "string",
          "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
          "example": "1"
        }
      ]
    },
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                       |
| ---------------- | ------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V2AddLiquidity'                                     |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market     |
| baseQuantity     | [AssetValue](#schemaassetvalue)             | true     | none         | base quantity, see [asset value](#overview--price-and-quantity-precision) format  |
| quoteQuantity    | [AssetValue](#schemaassetvalue)             | true     | none         | quote quantity, see [asset value](#overview--price-and-quantity-precision) format |
| upperBound       | string                                      | true     | none         | upper bound of price range, in quote currency                                     |
| lowerBound       | string                                      | true     | none         | lower bound of price range, in quote currency                                     |
| feeTierId        | [FeeTierId](#schemafeetierid)               | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)         |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                         |

<h2 id="tocS_CreateAMMInstructionCommandV3">CreateAMMInstructionCommandV3</h2>
<!-- backwards compatibility -->
<a id="schemacreateamminstructioncommandv3"></a>
<a id="schema_CreateAMMInstructionCommandV3"></a>
<a id="tocScreateamminstructioncommandv3"></a>
<a id="tocscreateamminstructioncommandv3"></a>

```json
{
  "commandType": "V3CreateAMMInstruction",
  "symbol": "BTCUSDC",
  "baseQuantity": "0",
  "quoteQuantity": "50000.1",
  "upperBound": "25000",
  "lowerBound": "20000",
  "feeTierId": "1",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/CreateAMMInstructionCommandV3/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                   |
| ---------------- | ------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V3CreateAMMInstruction'                         |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market |
| baseQuantity     | [AssetValue](#schemaassetvalue)             | true     | none         | base quantity                                                                 |
| quoteQuantity    | [AssetValue](#schemaassetvalue)             | true     | none         | quote quantity                                                                |
| upperBound       | string                                      | true     | none         | upper bound of price range, in quote currency                                 |
| lowerBound       | string                                      | true     | none         | lower bound of price range, in quote currency                                 |
| feeTierId        | [FeeTierId](#schemafeetierid)               | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)     |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                     |

<h2 id="tocS_CancelOrderCommand">CancelOrderCommand</h2>
<!-- backwards compatibility -->
<a id="schemacancelordercommand"></a>
<a id="schema_CancelOrderCommand"></a>
<a id="tocScancelordercommand"></a>
<a id="tocscancelordercommand"></a>

```json
{
  "type": "object",
  "required": [
    "commandType",
    "orderId",
    "handle",
    "symbol",
    "tradingAccountId"
  ],
  "properties": {
    "commandType": {
      "description": "The command type, it must be 'V2CancelOrder'",
      "type": "string",
      "example": "V2CancelOrder"
    },
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
        }
      ]
    },
    "handle": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                              |
| ---------------- | ------------------------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V2CancelOrder'                                             |
| orderId          | [OrderID](#schemaorderid)                   | true     | none         | unique order ID                                                                          |
| handle           | [OrderHandle](#schemaorderhandle)           | true     | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market            |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                                |

<h2 id="tocS_CancelOrderCommandV3">CancelOrderCommandV3</h2>
<!-- backwards compatibility -->
<a id="schemacancelordercommandv3"></a>
<a id="schema_CancelOrderCommandV3"></a>
<a id="tocScancelordercommandv3"></a>
<a id="tocscancelordercommandv3"></a>

```json
{
  "commandType": "V3CancelOrder",
  "orderId": "297735387747975680",
  "symbol": "BTCUSDC",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/CancelOrderCommandV3/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                              |
| ---------------- | ------------------------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V3CancelOrder'                                             |
| orderId          | [OrderID](#schemaorderid)                   | false    | none         | unique order ID                                                                          |
| clientOrderId    | [OrderHandle](#schemaorderhandle)           | false    | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market            |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                                |

<h2 id="tocS_AmendOrderCommandV1">AmendOrderCommandV1</h2>
<!-- backwards compatibility -->
<a id="schemaamendordercommandv1"></a>
<a id="schema_AmendOrderCommandV1"></a>
<a id="tocSamendordercommandv1"></a>
<a id="tocsamendordercommandv1"></a>

```json
{
  "commandType": "V1AmendOrder",
  "orderId": "297735387747975680",
  "symbol": "BTCUSDC",
  "type": "LIMIT",
  "price": "1.00000000",
  "clientOrderId": "633914459442118656",
  "quantity": "1.00000000",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/AmendOrderCommandV1/example"
}
```

### Properties

| Name             | Type                                                    | Required | Restrictions | Description                                                                        |
| ---------------- | ------------------------------------------------------- | -------- | ------------ | ---------------------------------------------------------------------------------- |
| commandType      | string                                                  | true     | none         | The command type, it must be 'V1AmendOrder'                                        |
| orderId          | [OrderID](#schemaorderid)                               | false    | none         | unique order ID                                                                    |
| symbol           | [MarketSymbol](#schemamarketsymbol)                     | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market      |
| type             | [OrderTypeAsStringAmend](#schemaordertypeasstringamend) | false    | none         | order type can have the following string values `"LIMIT"`, `"POST_ONLY"`           |
| price            | [AssetValue](#schemaassetvalue)                         | false    | none         | price                                                                              |
| clientOrderId    | string                                                  | false    | none         | unique numeric identifier generated on the client side expressed as a string value |
| quantity         | [AssetValue](#schemaassetvalue)                         | false    | none         | quantity                                                                           |
| tradingAccountId | [TradingAccountId](#schematradingaccountid)             | true     | none         | unique trading account ID                                                          |

<h2 id="tocS_TerminateAMMInstructionCommandV3">TerminateAMMInstructionCommandV3</h2>
<!-- backwards compatibility -->
<a id="schematerminateamminstructioncommandv3"></a>
<a id="schema_TerminateAMMInstructionCommandV3"></a>
<a id="tocSterminateamminstructioncommandv3"></a>
<a id="tocsterminateamminstructioncommandv3"></a>

```json
{
  "commandType": "V3TerminateAMMInstruction",
  "instructionId": "297735387747975680",
  "symbol": "BTCUSDC",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/TerminateAMMInstructionCommandV3/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                   |
| ---------------- | ------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V3TerminateAMMInstruction'                      |
| instructionId    | [AMMInstructionID](#schemaamminstructionid) | true     | none         | unique AMM instruction ID                                                     |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                     |

<h2 id="tocS_CancelAllOrdersRequest">CancelAllOrdersRequest</h2>
<!-- backwards compatibility -->
<a id="schemacancelallordersrequest"></a>
<a id="schema_CancelAllOrdersRequest"></a>
<a id="tocScancelallordersrequest"></a>
<a id="tocscancelallordersrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": ["commandType", "tradingAccountId"],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V1CancelAllOrders'",
              "type": "string"
            },
            "tradingAccountId": {
              "description": "unique trading account Id",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          },
          "example": {
            "commandType": "V1CancelAllOrders",
            "tradingAccountId": "111000000000001"
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                    | Required | Restrictions | Description                                                                                       |
| ---------- | ------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)           | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)                   | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                         | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CancelAllOrdersCommand](#schemacancelallorderscommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_CancelAllOrdersCommand">CancelAllOrdersCommand</h2>
<!-- backwards compatibility -->
<a id="schemacancelallorderscommand"></a>
<a id="schema_CancelAllOrdersCommand"></a>
<a id="tocScancelallorderscommand"></a>
<a id="tocscancelallorderscommand"></a>

```json
{
  "commandType": "V1CancelAllOrders",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/CancelAllOrdersCommand/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                      |
| ---------------- | ------------------------------------------- | -------- | ------------ | ------------------------------------------------ |
| commandType      | string                                      | true     | none         | The command type, it must be 'V1CancelAllOrders' |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account Id                        |

<h2 id="tocS_CancelAllOrdersResponse">CancelAllOrdersResponse</h2>
<!-- backwards compatibility -->
<a id="schemacancelallordersresponse"></a>
<a id="schema_CancelAllOrdersResponse"></a>
<a id="tocScancelallordersresponse"></a>
<a id="tocscancelallordersresponse"></a>

```json
{
  "message": "Command acknowledged - CancelAllOrders",
  "requestId": "633900538459062272"
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |

<h2 id="tocS_DelayedCancelAllOrdersRequest">DelayedCancelAllOrdersRequest</h2>
<!-- backwards compatibility -->
<a id="schemadelayedcancelallordersrequest"></a>
<a id="schema_DelayedCancelAllOrdersRequest"></a>
<a id="tocSdelayedcancelallordersrequest"></a>
<a id="tocsdelayedcancelallordersrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": ["commandType", "delayBySeconds", "tradingAccountId"],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V1DelayedCancelAllOrders'",
              "type": "string",
              "example": "V1DelayedCancelAllOrders"
            },
            "cancelId": {
              "allOf": [
                {
                  "description": "Unique id for this cancel request which is an unsigned 64 bit integer expressed as string",
                  "type": "string",
                  "example": "123456789"
                }
              ]
            },
            "delayBySeconds": {
              "description": "Delay of the cancel-all-order in seconds",
              "allOf": [
                {
                  "description": "Delay the cancel-all-orders request by (seconds) as a timeout mechanism",
                  "type": "string",
                  "enum": ["5", "10", "15", "20", "25", "30", "40", "50", "60"],
                  "example": "5"
                }
              ]
            },
            "tradingAccountId": {
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          },
          "example": {
            "commandType": "V1DelayedCancelAllOrders",
            "delayBySeconds": "5",
            "tradingAccountId": "111000000000001"
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                                  | Required | Restrictions | Description                                                                                       |
| ---------- | --------------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                         | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)                                 | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                                       | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [DelayedCancelAllOrdersCommand](#schemadelayedcancelallorderscommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_DelayedCancelAllOrdersCommand">DelayedCancelAllOrdersCommand</h2>
<!-- backwards compatibility -->
<a id="schemadelayedcancelallorderscommand"></a>
<a id="schema_DelayedCancelAllOrdersCommand"></a>
<a id="tocSdelayedcancelallorderscommand"></a>
<a id="tocsdelayedcancelallorderscommand"></a>

```json
{
  "commandType": "V1DelayedCancelAllOrders",
  "delayBySeconds": "5",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/DelayedCancelAllOrdersCommand/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                                               |
| ---------------- | ------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V1DelayedCancelAllOrders'                                   |
| cancelId         | [CancelId](#schemacancelid)                 | false    | none         | Unique id for this cancel request which is an unsigned 64 bit integer expressed as string |
| delayBySeconds   | [DelayBySeconds](#schemadelaybyseconds)     | true     | none         | Delay of the cancel-all-order in seconds                                                  |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                                                 |

<h2 id="tocS_DelayedCancelAllOrdersResponse">DelayedCancelAllOrdersResponse</h2>
<!-- backwards compatibility -->
<a id="schemadelayedcancelallordersresponse"></a>
<a id="schema_DelayedCancelAllOrdersResponse"></a>
<a id="tocSdelayedcancelallordersresponse"></a>
<a id="tocsdelayedcancelallordersresponse"></a>

```json
{
  "message": "Command acknowledged - DelayedCancelAllOrders",
  "requestId": "633914459442118656"
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |

<h2 id="tocS_UnsetDelayedCancelAllOrdersRequest">UnsetDelayedCancelAllOrdersRequest</h2>
<!-- backwards compatibility -->
<a id="schemaunsetdelayedcancelallordersrequest"></a>
<a id="schema_UnsetDelayedCancelAllOrdersRequest"></a>
<a id="tocSunsetdelayedcancelallordersrequest"></a>
<a id="tocsunsetdelayedcancelallordersrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": ["commandType", "tradingAccountId"],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V1UnsetDelayedCancelAllOrders'",
              "type": "string",
              "example": "V1UnsetDelayedCancelAllOrders"
            },
            "tradingAccountId": {
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          },
          "example": {
            "commandType": "V1UnsetDelayedCancelAllOrders",
            "tradingAccountId": "111000000000001"
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                                            | Required | Restrictions | Description                                                                                       |
| ---------- | ------------------------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                                   | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)                                           | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                                                 | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [UnsetDelayedCancelAllOrdersCommand](#schemaunsetdelayedcancelallorderscommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_UnsetDelayedCancelAllOrdersCommand">UnsetDelayedCancelAllOrdersCommand</h2>
<!-- backwards compatibility -->
<a id="schemaunsetdelayedcancelallorderscommand"></a>
<a id="schema_UnsetDelayedCancelAllOrdersCommand"></a>
<a id="tocSunsetdelayedcancelallorderscommand"></a>
<a id="tocsunsetdelayedcancelallorderscommand"></a>

```json
{
  "commandType": "V1UnsetDelayedCancelAllOrders",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/UnsetDelayedCancelAllOrdersCommand/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                                  |
| ---------------- | ------------------------------------------- | -------- | ------------ | ------------------------------------------------------------ |
| commandType      | string                                      | true     | none         | The command type, it must be 'V1UnsetDelayedCancelAllOrders' |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                                    |

<h2 id="tocS_UnsetDelayedCancelAllOrdersResponse">UnsetDelayedCancelAllOrdersResponse</h2>
<!-- backwards compatibility -->
<a id="schemaunsetdelayedcancelallordersresponse"></a>
<a id="schema_UnsetDelayedCancelAllOrdersResponse"></a>
<a id="tocSunsetdelayedcancelallordersresponse"></a>
<a id="tocsunsetdelayedcancelallordersresponse"></a>

```json
{
  "message": "Command acknowledged - UnsetDelayedCancelAllOrders",
  "requestId": "633914459442118656"
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |

<h2 id="tocS_JWT">JWT</h2>
<!-- backwards compatibility -->
<a id="schemajwt"></a>
<a id="schema_JWT"></a>
<a id="tocSjwt"></a>
<a id="tocsjwt"></a>

```json
"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoic2FuZGVlcCByYWtocmEifQ.wyVq6PlKaldWXtu-jz2hJCvkGl1lM2S7HUKCH8LnXp0"
```

JWT token

### Properties

| Name        | Type           | Required | Restrictions | Description |
| ----------- | -------------- | -------- | ------------ | ----------- |
| _anonymous_ | string(string) | false    | none         | JWT token   |

<h2 id="tocS_Authorizer">Authorizer</h2>
<!-- backwards compatibility -->
<a id="schemaauthorizer"></a>
<a id="schema_Authorizer"></a>
<a id="tocSauthorizer"></a>
<a id="tocsauthorizer"></a>

```json
"03E02367E8C900000500000000000000"
```

JWT authorizer you obtain along with the
[JWT token](#overview--generate-a-jwt-token)

### Properties

| Name        | Type           | Required | Restrictions | Description                                                                           |
| ----------- | -------------- | -------- | ------------ | ------------------------------------------------------------------------------------- |
| _anonymous_ | string(string) | false    | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token) |

<h2 id="tocS_TradingAccountTransferRequest">TradingAccountTransferRequest</h2>
<!-- backwards compatibility -->
<a id="schematradingaccounttransferrequest"></a>
<a id="schema_TradingAccountTransferRequest"></a>
<a id="tocStradingaccounttransferrequest"></a>
<a id="tocstradingaccounttransferrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "assetSymbol",
            "quantity",
            "fromTradingAccountId",
            "toTradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, e.g. 'V1TransferAsset'",
              "type": "string",
              "example": "V1TransferAsset"
            },
            "assetSymbol": {
              "description": "Symbol of the asset. i.e. currency",
              "allOf": [
                {
                  "type": "string",
                  "description": "asset symbol as denoted in the world",
                  "example": "BTC"
                }
              ]
            },
            "quantity": {
              "description": "Quantity of the asset.",
              "type": "string",
              "example": "100"
            },
            "fromTradingAccountId": {
              "description": "Source of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            },
            "toTradingAccountId": {
              "description": "Destination of the asset transfer",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                | Required | Restrictions | Description                                                                                       |
| ---------- | --------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)       | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)               | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                     | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [TransferAssetCommand](#schematransferassetcommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_TradingAccountTransferResponse">TradingAccountTransferResponse</h2>
<!-- backwards compatibility -->
<a id="schematradingaccounttransferresponse"></a>
<a id="schema_TradingAccountTransferResponse"></a>
<a id="tocStradingaccounttransferresponse"></a>
<a id="tocstradingaccounttransferresponse"></a>

```json
{
  "message": "Command acknowledged - TransferAsset",
  "requestId": "633909659774222336"
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |

<h2 id="tocS_TransferAssetCommand">TransferAssetCommand</h2>
<!-- backwards compatibility -->
<a id="schematransferassetcommand"></a>
<a id="schema_TransferAssetCommand"></a>
<a id="tocStransferassetcommand"></a>
<a id="tocstransferassetcommand"></a>

```json
{
  "type": "object",
  "required": [
    "commandType",
    "assetSymbol",
    "quantity",
    "fromTradingAccountId",
    "toTradingAccountId"
  ],
  "properties": {
    "commandType": {
      "description": "The command type, e.g. 'V1TransferAsset'",
      "type": "string",
      "example": "V1TransferAsset"
    },
    "assetSymbol": {
      "description": "Symbol of the asset. i.e. currency",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "quantity": {
      "description": "Quantity of the asset.",
      "type": "string",
      "example": "100"
    },
    "fromTradingAccountId": {
      "description": "Source of the asset transfer",
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "toTradingAccountId": {
      "description": "Destination of the asset transfer",
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    }
  }
}
```

### Properties

| Name                 | Type                                        | Required | Restrictions | Description                              |
| -------------------- | ------------------------------------------- | -------- | ------------ | ---------------------------------------- |
| commandType          | string                                      | true     | none         | The command type, e.g. 'V1TransferAsset' |
| assetSymbol          | [AssetSymbol](#schemaassetsymbol)           | true     | none         | Symbol of the asset. i.e. currency       |
| quantity             | string                                      | true     | none         | Quantity of the asset.                   |
| fromTradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | Source of the asset transfer             |
| toTradingAccountId   | [TradingAccountId](#schematradingaccountid) | true     | none         | Destination of the asset transfer        |

<h2 id="tocS_TransferAssetCommandV2">TransferAssetCommandV2</h2>
<!-- backwards compatibility -->
<a id="schematransferassetcommandv2"></a>
<a id="schema_TransferAssetCommandV2"></a>
<a id="tocStransferassetcommandv2"></a>
<a id="tocstransferassetcommandv2"></a>

```json
{
  "commandType": "V2TransferAsset",
  "assetSymbol": "BTC",
  "quantity": "100.1",
  "fromTradingAccountId": "111000000000001",
  "toTradingAccountId": "111000000000002",
  "x-widdershins-oldRef": "#/components/schemas/TransferAssetCommandV2/example"
}
```

### Properties

| Name                 | Type                                        | Required | Restrictions | Description                                                                                                                                                                            |
| -------------------- | ------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| commandType          | string                                      | true     | none         | The command type, e.g. 'V2TransferAsset'                                                                                                                                               |
| assetSymbol          | [AssetSymbol](#schemaassetsymbol)           | true     | none         | Symbol of the asset. i.e. currency                                                                                                                                                     |
| quantity             | string                                      | true     | none         | Quantity of the asset. Can be representated with any number of trailing zeroes up to asset precision <br>(eg `100.1`, `100.10`, `100.100`, `100.1000` are valid for asset precision 4) |
| fromTradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | Source of the asset transfer                                                                                                                                                           |
| toTradingAccountId   | [TradingAccountId](#schematradingaccountid) | true     | none         | Destination of the asset transfer                                                                                                                                                      |

<h2 id="tocS_CancelAllOrdersByMarketRequest">CancelAllOrdersByMarketRequest</h2>
<!-- backwards compatibility -->
<a id="schemacancelallordersbymarketrequest"></a>
<a id="schema_CancelAllOrdersByMarketRequest"></a>
<a id="tocScancelallordersbymarketrequest"></a>
<a id="tocscancelallordersbymarketrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": ["commandType", "symbol", "tradingAccountId"],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V1CancelAllOrdersByMarket'",
              "type": "string"
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
            "tradingAccountId": {
              "description": "unique trading account Id",
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          },
          "example": {
            "commandType": "V1CancelAllOrdersByMarket",
            "symbol": "BTCUSDC",
            "tradingAccountId": "111000000000001"
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                                    | Required | Restrictions | Description                                                                                       |
| ---------- | ----------------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                           | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)                                   | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                                         | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CancelAllOrdersByMarketCommand](#schemacancelallordersbymarketcommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_CancelAllOrdersByMarketCommand">CancelAllOrdersByMarketCommand</h2>
<!-- backwards compatibility -->
<a id="schemacancelallordersbymarketcommand"></a>
<a id="schema_CancelAllOrdersByMarketCommand"></a>
<a id="tocScancelallordersbymarketcommand"></a>
<a id="tocscancelallordersbymarketcommand"></a>

```json
{
  "commandType": "V1CancelAllOrdersByMarket",
  "symbol": "BTCUSDC",
  "tradingAccountId": "111000000000001",
  "x-widdershins-oldRef": "#/components/schemas/CancelAllOrdersByMarketCommand/example"
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                              |
| ---------------- | ------------------------------------------- | -------- | ------------ | -------------------------------------------------------- |
| commandType      | string                                      | true     | none         | The command type, it must be 'V1CancelAllOrdersByMarket' |
| symbol           | [MarketSymbol](#schemamarketsymbol)         | true     | none         | market symbol                                            |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account Id                                |

<h2 id="tocS_CancelAllOrdersByMarketResponse">CancelAllOrdersByMarketResponse</h2>
<!-- backwards compatibility -->
<a id="schemacancelallordersbymarketresponse"></a>
<a id="schema_CancelAllOrdersByMarketResponse"></a>
<a id="tocScancelallordersbymarketresponse"></a>
<a id="tocscancelallordersbymarketresponse"></a>

```json
{
  "message": "Command acknowledged - CancelAllOrdersByMarket",
  "requestId": "633914459442118656"
}
```

### Properties

| Name      | Type                          | Required | Restrictions | Description       |
| --------- | ----------------------------- | -------- | ------------ | ----------------- |
| message   | string                        | true     | none         | message           |
| requestId | [RequestID](#schemarequestid) | true     | none         | unique request ID |

<h2 id="tocS_LoginRequest">LoginRequest</h2>
<!-- backwards compatibility -->
<a id="schemaloginrequest"></a>
<a id="schema_LoginRequest"></a>
<a id="tocSloginrequest"></a>
<a id="tocsloginrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
  "properties": {
    "publicKey": {
      "allOf": [
        {
          "type": "string",
          "example": "PUB_R1_6PTdfWbXvXWQduhcCiRooGHTVpriu15xMqfr7EDq6sWLDj7NjS"
        }
      ]
    },
    "signature": {
      "allOf": [
        {
          "type": "string",
          "example": "SIG_R1_K35d5hSY5FbNoJwrCfmH6QvPG7m9XmhL2mgWYcSB7q2hKJ2mv39Luck9WBJroSB635ZAXhdg36TYG7QJX1fTidbsMvyE8N"
        }
      ]
    },
    "loginPayload": {
      "allOf": [
        {
          "type": "object",
          "required": [
            "userId",
            "nonce",
            "expirationTime",
            "biometricsUsed",
            "sessionKey"
          ],
          "properties": {
            "userId": {
              "description": "Bullish user ID corresponding to the metadata",
              "allOf": [
                {
                  "type": "string",
                  "example": "12345",
                  "description": "Bullish user ID"
                }
              ]
            },
            "nonce": {
              "description": "epoch timestamp in seconds; note this login API nonce has no connection to the orders API nonce",
              "allOf": [
                {
                  "type": "integer",
                  "format": "int64",
                  "example": 1621490985,
                  "description": "number of seconds since EPOCH as integer"
                }
              ]
            },
            "expirationTime": {
              "description": "epoch timestamp in seconds that is 5 minutes in the future",
              "allOf": [
                {
                  "type": "integer",
                  "format": "int64",
                  "example": 1621490985,
                  "description": "number of seconds since EPOCH as integer"
                }
              ]
            },
            "biometricsUsed": {
              "description": "biometrics flag. always `false`",
              "type": "boolean",
              "example": false
            },
            "sessionKey": {
              "description": "session key. always `null`",
              "type": "string",
              "example": null
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name         | Type                                | Required | Restrictions | Description |
| ------------ | ----------------------------------- | -------- | ------------ | ----------- |
| publicKey    | [PublicKey](#schemapublickey)       | false    | none         | none        |
| signature    | [Signature](#schemasignature)       | false    | none         | none        |
| loginPayload | [LoginPayload](#schemaloginpayload) | false    | none         | none        |

<h2 id="tocS_LoginPayload">LoginPayload</h2>
<!-- backwards compatibility -->
<a id="schemaloginpayload"></a>
<a id="schema_LoginPayload"></a>
<a id="tocSloginpayload"></a>
<a id="tocsloginpayload"></a>

```json
{
  "type": "object",
  "required": [
    "userId",
    "nonce",
    "expirationTime",
    "biometricsUsed",
    "sessionKey"
  ],
  "properties": {
    "userId": {
      "description": "Bullish user ID corresponding to the metadata",
      "allOf": [
        {
          "type": "string",
          "example": "12345",
          "description": "Bullish user ID"
        }
      ]
    },
    "nonce": {
      "description": "epoch timestamp in seconds; note this login API nonce has no connection to the orders API nonce",
      "allOf": [
        {
          "type": "integer",
          "format": "int64",
          "example": 1621490985,
          "description": "number of seconds since EPOCH as integer"
        }
      ]
    },
    "expirationTime": {
      "description": "epoch timestamp in seconds that is 5 minutes in the future",
      "allOf": [
        {
          "type": "integer",
          "format": "int64",
          "example": 1621490985,
          "description": "number of seconds since EPOCH as integer"
        }
      ]
    },
    "biometricsUsed": {
      "description": "biometrics flag. always `false`",
      "type": "boolean",
      "example": false
    },
    "sessionKey": {
      "description": "session key. always `null`",
      "type": "string",
      "example": null
    }
  }
}
```

### Properties

| Name           | Type                                            | Required | Restrictions | Description                                                                                     |
| -------------- | ----------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------- |
| userId         | [UserId](#schemauserid)                         | true     | none         | Bullish user ID corresponding to the metadata                                                   |
| nonce          | [TimeStampInSeconds](#schematimestampinseconds) | true     | none         | epoch timestamp in seconds; note this login API nonce has no connection to the orders API nonce |
| expirationTime | [TimeStampInSeconds](#schematimestampinseconds) | true     | none         | epoch timestamp in seconds that is 5 minutes in the future                                      |
| biometricsUsed | boolean                                         | true     | none         | biometrics flag. always `false`                                                                 |
| sessionKey     | string                                          | true     | none         | session key. always `null`                                                                      |

<h2 id="tocS_LoginResponse">LoginResponse</h2>
<!-- backwards compatibility -->
<a id="schemaloginresponse"></a>
<a id="schema_LoginResponse"></a>
<a id="tocSloginresponse"></a>
<a id="tocsloginresponse"></a>

```json
{
  "type": "object",
  "required": ["authorizer", "token"],
  "properties": {
    "authorizer": {
      "description": "Authorizer",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "token": {
      "description": "JWT token",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoic2FuZGVlcCByYWtocmEifQ.wyVq6PlKaldWXtu-jz2hJCvkGl1lM2S7HUKCH8LnXp0",
          "description": "JWT token"
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                            | Required | Restrictions | Description |
| ---------- | ------------------------------- | -------- | ------------ | ----------- |
| authorizer | [Authorizer](#schemaauthorizer) | true     | none         | Authorizer  |
| token      | [JWT](#schemajwt)               | true     | none         | JWT token   |

<h2 id="tocS_CreateOrderRequest">CreateOrderRequest</h2>
<!-- backwards compatibility -->
<a id="schemacreateorderrequest"></a>
<a id="schema_CreateOrderRequest"></a>
<a id="tocScreateorderrequest"></a>
<a id="tocscreateorderrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "handle",
            "symbol",
            "type",
            "side",
            "price",
            "stopPrice",
            "quantity",
            "allowMargin",
            "timeInForce",
            "tradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V2CreateOrder'",
              "type": "string",
              "example": "V2CreateOrder"
            },
            "handle": {
              "allOf": [
                {
                  "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
                  "type": "string",
                  "example": "187"
                }
              ]
            },
            "symbol": {
              "allOf": [
                {
                  "type": "string",
                  "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                  "example": "BTCUSDC"
                }
              ]
            },
            "type": {
              "allOf": [
                {
                  "type": "string",
                  "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
                  "example": "LMT"
                }
              ],
              "example": "LMT"
            },
            "side": {
              "allOf": [
                {
                  "type": "string",
                  "description": "order side can have the following string values `\"BUY\"`, `\"SELL\"`",
                  "example": "BUY"
                }
              ],
              "example": "BUY"
            },
            "price": {
              "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
              "allOf": [
                {
                  "description": "see [asset value](#overview--price-and-quantity-precision) format",
                  "type": "string",
                  "example": "1.00000000"
                }
              ]
            },
            "stopPrice": {
              "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
              "allOf": [
                {
                  "description": "see [asset value](#overview--price-and-quantity-precision) format",
                  "type": "string",
                  "example": "1.00000000"
                }
              ]
            },
            "quantity": {
              "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
              "allOf": [
                {
                  "description": "see [asset value](#overview--price-and-quantity-precision) format",
                  "type": "string",
                  "example": "1.00000000"
                }
              ]
            },
            "timeInForce": {
              "allOf": [
                {
                  "type": "string",
                  "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
                }
              ],
              "example": "GTC"
            },
            "allowMargin": {
              "description": "allows to borrow on the order",
              "type": "boolean",
              "example": false
            },
            "tradingAccountId": {
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                            | Required | Restrictions | Description                                                                                       |
| ---------- | ----------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)   | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)           | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                 | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CreateOrderCommand](#schemacreateordercommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_CreateAMMInstructionRequest">CreateAMMInstructionRequest</h2>
<!-- backwards compatibility -->
<a id="schemacreateamminstructionrequest"></a>
<a id="schema_CreateAMMInstructionRequest"></a>
<a id="tocScreateamminstructionrequest"></a>
<a id="tocscreateamminstructionrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "symbol",
            "baseQuantity",
            "quoteQuantity",
            "upperBound",
            "lowerBound",
            "feeTierId",
            "tradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V2AddLiquidity'",
              "type": "string",
              "example": "V2AddLiquidity"
            },
            "symbol": {
              "allOf": [
                {
                  "type": "string",
                  "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                  "example": "BTCUSDC"
                }
              ]
            },
            "baseQuantity": {
              "description": "base quantity, see [asset value](#overview--price-and-quantity-precision) format",
              "allOf": [
                {
                  "description": "see [asset value](#overview--price-and-quantity-precision) format",
                  "type": "string",
                  "example": "1.00000000"
                }
              ]
            },
            "quoteQuantity": {
              "description": "quote quantity, see [asset value](#overview--price-and-quantity-precision) format",
              "allOf": [
                {
                  "description": "see [asset value](#overview--price-and-quantity-precision) format",
                  "type": "string",
                  "example": "1.00000000"
                }
              ]
            },
            "upperBound": {
              "type": "string",
              "description": "upper bound of price range, in quote currency",
              "example": "14000.0000"
            },
            "lowerBound": {
              "type": "string",
              "description": "lower bound of price range, in quote currency",
              "example": "12000.0000"
            },
            "feeTierId": {
              "allOf": [
                {
                  "type": "string",
                  "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
                  "example": "1"
                }
              ]
            },
            "tradingAccountId": {
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                              | Required | Restrictions | Description                                                                                       |
| ---------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                     | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | [NonceAsString](#schemanonceasstring)                             | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer | [Authorizer](#schemaauthorizer)                                   | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CreateAMMInstructionCommand](#schemacreateamminstructioncommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_CancelOrderRequest">CancelOrderRequest</h2>
<!-- backwards compatibility -->
<a id="schemacancelorderrequest"></a>
<a id="schema_CancelOrderRequest"></a>
<a id="tocScancelorderrequest"></a>
<a id="tocscancelorderrequest"></a>

```json
{
  "type": "object",
  "required": [
    "timestamp",
    "nonce",
    "authorizer",
    "tradingAccountId",
    "command"
  ],
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
    "nonce": {
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "123456789",
          "description": "the nonce is a client side incremented unsigned 64 bit integer expressed as string"
        }
      ]
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "command": {
      "description": "the command to be executed which is sent in the request payload",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "orderId",
            "handle",
            "symbol",
            "tradingAccountId"
          ],
          "properties": {
            "commandType": {
              "description": "The command type, it must be 'V2CancelOrder'",
              "type": "string",
              "example": "V2CancelOrder"
            },
            "orderId": {
              "description": "unique order ID",
              "allOf": [
                {
                  "type": "string",
                  "example": "297735387747975680"
                }
              ]
            },
            "handle": {
              "allOf": [
                {
                  "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
                  "type": "string",
                  "example": "187"
                }
              ]
            },
            "symbol": {
              "allOf": [
                {
                  "type": "string",
                  "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                  "example": "BTCUSDC"
                }
              ]
            },
            "tradingAccountId": {
              "allOf": [
                {
                  "description": "unique trading account ID",
                  "type": "string",
                  "example": "111000000000001"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                            | Required | Restrictions | Description                                                                                       |
| ---------------- | ----------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp        | [TimeStampAsString](#schematimestampasstring)   | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce            | [NonceAsString](#schemanonceasstring)           | true     | none         | the nonce is a client side incremented unsigned 64 bit integer expressed as string                |
| authorizer       | [Authorizer](#schemaauthorizer)                 | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| tradingAccountId | [TradingAccountId](#schematradingaccountid)     | true     | none         | unique trading account ID                                                                         |
| command          | [CancelOrderCommand](#schemacancelordercommand) | true     | none         | the command to be executed which is sent in the request payload                                   |

<h2 id="tocS_Order">Order</h2>
<!-- backwards compatibility -->
<a id="schemaorder"></a>
<a id="schema_Order"></a>
<a id="tocSorder"></a>
<a id="tocsorder"></a>

```json
{
  "type": "object",
  "required": [
    "orderId",
    "clientOrderId",
    "symbol",
    "price",
    "stopPrice",
    "averageFillPrice",
    "allowBorrow",
    "quantity",
    "quantityFilled",
    "quoteAmount",
    "baseFee",
    "quoteFee",
    "isLiquidation",
    "side",
    "type",
    "timeInForce",
    "status",
    "statusReason",
    "statusReasonCode",
    "createdAtTimestamp",
    "createdAtDatetime"
  ],
  "properties": {
    "clientOrderId": {
      "allOf": [
        {
          "description": "Unique numeric (i64) identifier generated on the client side expressed as a string value",
          "type": "string",
          "example": "187"
        }
      ]
    },
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
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
      "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "averageFillPrice": {
      "description": "average fill price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "stopPrice": {
      "description": "stop price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "allowBorrow": {
      "description": "indicates if the order was allowed to borrow (does not indicate that borrowing occurred)",
      "type": "boolean",
      "example": false
    },
    "quantity": {
      "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantityFilled": {
      "description": "quantity filled, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteAmount": {
      "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "baseFee": {
      "description": "base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
      "example": "0.00100000",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteFee": {
      "description": "quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format",
      "example": "0.0010",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedBaseQuantity": {
      "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "borrowedQuoteQuantity": {
      "description": "quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "isLiquidation": {
      "description": "indicates if the order was executed as a liquidation order",
      "type": "boolean",
      "example": false
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
    "type": {
      "description": "order type",
      "allOf": [
        {
          "type": "string",
          "description": "order type can have the following string values `\"LMT\"`, `\"MKT\"`, `\"STOP_LIMIT\"`, `\"POST_ONLY\"`.",
          "example": "LMT"
        }
      ],
      "example": "LMT"
    },
    "timeInForce": {
      "description": "time in force",
      "allOf": [
        {
          "type": "string",
          "description": "time in force can have the following string values `\"GTC\"`, `\"FOK\"`, `\"IOC\"`, see [details](#overview--order-timeinforce)"
        }
      ],
      "example": "GTC"
    },
    "status": {
      "description": "order status",
      "allOf": [
        {
          "type": "string",
          "description": "order status can have the following string values `\"OPEN\"`, `\"CLOSED\"`, `\"CANCELLED\"`, `\"REJECTED\"`",
          "example": "OPEN"
        }
      ],
      "example": "OPEN"
    },
    "statusReason": {
      "description": "status reason, describes why the order is in a specific state",
      "type": "string",
      "example": "User cancelled"
    },
    "statusReasonCode": {
      "description": "status reason code, see [details](#overview--error--rejection-codes)",
      "type": "string",
      "example": "1002"
    },
    "createdAtDatetime": {
      "description": "denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the order was ACK'd by the exchange",
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
```

### Properties

| Name                  | Type                                                        | Required | Restrictions | Description                                                                                                                 |
| --------------------- | ----------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------- |
| clientOrderId         | [OrderHandle](#schemaorderhandle)                           | true     | none         | Unique numeric (i64) identifier generated on the client side expressed as a string value                                    |
| orderId               | [OrderID](#schemaorderid)                                   | true     | none         | unique order ID                                                                                                             |
| symbol                | [MarketSymbol](#schemamarketsymbol)                         | true     | none         | market symbol                                                                                                               |
| price                 | [AssetValue](#schemaassetvalue)                             | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                                    |
| averageFillPrice      | [AssetValue](#schemaassetvalue)                             | true     | none         | average fill price, see [asset value](#overview--price-and-quantity-precision) format                                       |
| stopPrice             | [AssetValue](#schemaassetvalue)                             | true     | none         | stop price, see [asset value](#overview--price-and-quantity-precision) format                                               |
| allowBorrow           | boolean                                                     | true     | none         | indicates if the order was allowed to borrow (does not indicate that borrowing occurred)                                    |
| quantity              | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                                 |
| quantityFilled        | [AssetValue](#schemaassetvalue)                             | true     | none         | quantity filled, see [asset value](#overview--price-and-quantity-precision) format                                          |
| quoteAmount           | [AssetValue](#schemaassetvalue)                             | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format               |
| baseFee               | [AssetValue](#schemaassetvalue)                             | true     | none         | base fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format  |
| quoteFee              | [AssetValue](#schemaassetvalue)                             | true     | none         | quote fee rate that will be charged upon trade execution, see [asset value](#overview--price-and-quantity-precision) format |
| borrowedBaseQuantity  | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| borrowedQuoteQuantity | [AssetValue](#schemaassetvalue)                             | false    | none         | quantity borrowed, see [asset value](#overview--price-and-quantity-precision) format                                        |
| isLiquidation         | boolean                                                     | true     | none         | indicates if the order was executed as a liquidation order                                                                  |
| side                  | [OrderSideAsString](#schemaordersideasstring)               | true     | none         | order side                                                                                                                  |
| type                  | [OrderTypeAsString](#schemaordertypeasstring)               | true     | none         | order type                                                                                                                  |
| timeInForce           | [OrderTimeInForceAsString](#schemaordertimeinforceasstring) | true     | none         | time in force                                                                                                               |
| status                | [OrderStatusAsString](#schemaorderstatusasstring)           | true     | none         | order status                                                                                                                |
| statusReason          | string                                                      | true     | none         | status reason, describes why the order is in a specific state                                                               |
| statusReasonCode      | string                                                      | true     | none         | status reason code, see [details](#overview--error--rejection-codes)                                                        |
| createdAtDatetime     | [DateTime](#schemadatetime)                                 | true     | none         | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                   |
| createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring)               | true     | none         | denotes the time the order was ACK'd by the exchange                                                                        |

<h2 id="tocS_AMMInstruction">AMMInstruction</h2>
<!-- backwards compatibility -->
<a id="schemaamminstruction"></a>
<a id="schema_AMMInstruction"></a>
<a id="tocSamminstruction"></a>
<a id="tocsamminstruction"></a>

```json
{
  "type": "object",
  "required": [
    "apy",
    "baseCurrentQuantity",
    "baseFee",
    "baseInvestQuantity",
    "basePrice",
    "baseWithdrawQuantity",
    "createdAtDateTime",
    "createdAtTimestamp",
    "currentValue",
    "dislocationEnabled",
    "feeTierId",
    "impermanentLoss",
    "initialBasePrice",
    "initialQuotePrice",
    "initialValue",
    "liquidityId",
    "instructionId",
    "lowerBound",
    "price",
    "quoteFee",
    "quoteInvestQuantity",
    "quotePrice",
    "quoteWithdrawQuantity",
    "requestId",
    "staticSpreadFee",
    "status",
    "statusReason",
    "statusReasonCode",
    "symbol",
    "updatedAtDateTime",
    "updatedAtTimestamp",
    "upperBound",
    "yieldEarn"
  ],
  "properties": {
    "liquidityId": {
      "description": "unique AMM instruction ID",
      "deprecated": true,
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
        }
      ]
    },
    "instructionId": {
      "description": "unique AMM instruction ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
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
    "baseFee": {
      "description": "base fee, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteFee": {
      "description": "quote fee, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "status": {
      "description": "order status",
      "allOf": [
        {
          "type": "string",
          "description": "order status can have the following string values `\"OPEN\"`, `\"CLOSED\"`, `\"CANCELLED\"`, `\"REJECTED\"`",
          "example": "OPEN"
        }
      ],
      "example": "OPEN"
    },
    "statusReason": {
      "description": "status reason, describes why the order is in a specific state",
      "type": "string",
      "example": "Ok"
    },
    "statusReasonCode": {
      "description": "status reason code, see [details](#overview--error--rejection-codes)",
      "type": "integer",
      "example": 1001
    },
    "createdAtDatetime": {
      "description": "denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the order was ACK'd by the exchange",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "24HrApy": {
      "type": "string",
      "description": "APY of the last 24 Hours, only for AMM instructions with `OPEN` status",
      "example": "2.3319"
    },
    "24HrYieldEarn": {
      "type": "string",
      "description": "amount of money earned in USD from the last 24 Hours, only for AMM instructions with `OPEN` status",
      "example": "0.00"
    },
    "apy": {
      "type": "string",
      "description": "yield generated from the time AMM instruction was created, in annualised percentage",
      "example": "0.0000"
    },
    "baseCurrentQuantity": {
      "type": "string",
      "description": "amount of base asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status",
      "example": "0.00000000"
    },
    "baseInvestQuantity": {
      "type": "string",
      "description": "initial base investment",
      "example": "0.00000008"
    },
    "basePrice": {
      "type": "string",
      "description": "current price of base asset",
      "example": "345.6700"
    },
    "baseWithdrawQuantity": {
      "type": "string",
      "description": "amount of base asset returned when AMM instruction is terminated",
      "example": "0.00000010"
    },
    "currentValue": {
      "type": "string",
      "description": "value of assets (base and quote) in USD amount that this AMM instruction currently holds",
      "example": "0.0000"
    },
    "dislocationEnabled": {
      "description": "dislocation enabled/disabled",
      "type": "boolean",
      "example": false
    },
    "feeTierId": {
      "allOf": [
        {
          "type": "string",
          "description": "unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
          "example": "1"
        }
      ]
    },
    "finalValue": {
      "type": "string",
      "description": "value of assets (base and quote) in USD amount when AMM instruction was terminated, only for AMM instruction with `CLOSED` status",
      "example": "0.0001"
    },
    "impermanentLoss": {
      "type": "string",
      "description": "impermanent loss",
      "example": "0.0000"
    },
    "initialBasePrice": {
      "type": "string",
      "description": "price of base asset when AMM instruction was created",
      "example": "100.0000"
    },
    "initialQuotePrice": {
      "type": "string",
      "description": "price of quote asset when AMM instruction was created",
      "example": "0.0100"
    },
    "initialValue": {
      "type": "string",
      "description": "value of assets (base and quote) in USD amount when AMM instruction was created",
      "example": "0.0000"
    },
    "lowerBound": {
      "type": "string",
      "description": "lower bound of price range, in quote currency",
      "example": "0.0013"
    },
    "price": {
      "type": "string",
      "description": "current price of AMM, see [Get Tick By Symbol](#get-/v1/markets/-symbol-/tick)",
      "example": "456.7800"
    },
    "quoteCurrentQuantity": {
      "type": "string",
      "description": "amount of quote asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status",
      "example": "0.0000"
    },
    "quoteInvestQuantity": {
      "type": "string",
      "description": "initial quote investment",
      "example": "0.0009"
    },
    "quotePrice": {
      "type": "string",
      "description": "current price of quote asset",
      "example": "1.0000"
    },
    "quoteWithdrawQuantity": {
      "type": "string",
      "description": "amount of quote asset returned when AMM instruction is terminated",
      "example": "0.0011"
    },
    "lastDistributedPrice": {
      "type": "string",
      "description": "(Perpetual market only) The price used at the time of settlement for AMM Instructions that can be used to determine mtmPnl and the actual Pnl",
      "example": null
    },
    "requestId": {
      "description": "unique request ID",
      "allOf": [
        {
          "type": "string",
          "example": "197735387747975680"
        }
      ]
    },
    "staticSpreadFee": {
      "type": "string",
      "description": "static spread fee, see [Get Market By Symbol](#get-/v1/markets/-symbol-)",
      "example": "0.00200000"
    },
    "updatedAtDatetime": {
      "description": "denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string",
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
      "description": "denotes the time the AMM instruction was updated by the exchange",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "upperBound": {
      "type": "string",
      "description": "upper bound of price range, in quote currency",
      "example": "14000.0000"
    },
    "yieldEarn": {
      "type": "string",
      "description": "amount of money earned in USD",
      "example": "0.00"
    }
  }
}
```

### Properties

| Name                  | Type                                              | Required | Restrictions | Description                                                                                                                                   |
| --------------------- | ------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| liquidityId           | [AMMInstructionID](#schemaamminstructionid)       | true     | none         | unique AMM instruction ID                                                                                                                     |
| instructionId         | [AMMInstructionID](#schemaamminstructionid)       | true     | none         | unique AMM instruction ID                                                                                                                     |
| symbol                | [MarketSymbol](#schemamarketsymbol)               | true     | none         | market symbol                                                                                                                                 |
| baseFee               | [AssetValue](#schemaassetvalue)                   | true     | none         | base fee, see [asset value](#overview--price-and-quantity-precision) format                                                                   |
| quoteFee              | [AssetValue](#schemaassetvalue)                   | true     | none         | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                                                  |
| status                | [OrderStatusAsString](#schemaorderstatusasstring) | true     | none         | order status                                                                                                                                  |
| statusReason          | string                                            | true     | none         | status reason, describes why the order is in a specific state                                                                                 |
| statusReasonCode      | integer                                           | true     | none         | status reason code, see [details](#overview--error--rejection-codes)                                                                          |
| createdAtDatetime     | [DateTime](#schemadatetime)                       | false    | none         | denotes the time the order was ACK'd by the exchange, ISO 8601 with millisecond as string                                                     |
| createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring)     | true     | none         | denotes the time the order was ACK'd by the exchange                                                                                          |
| 24HrApy               | string                                            | false    | none         | APY of the last 24 Hours, only for AMM instructions with `OPEN` status                                                                        |
| 24HrYieldEarn         | string                                            | false    | none         | amount of money earned in USD from the last 24 Hours, only for AMM instructions with `OPEN` status                                            |
| apy                   | string                                            | true     | none         | yield generated from the time AMM instruction was created, in annualised percentage                                                           |
| baseCurrentQuantity   | string                                            | true     | none         | amount of base asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status                                        |
| baseInvestQuantity    | string                                            | true     | none         | initial base investment                                                                                                                       |
| basePrice             | string                                            | true     | none         | current price of base asset                                                                                                                   |
| baseWithdrawQuantity  | string                                            | true     | none         | amount of base asset returned when AMM instruction is terminated                                                                              |
| currentValue          | string                                            | true     | none         | value of assets (base and quote) in USD amount that this AMM instruction currently holds                                                      |
| dislocationEnabled    | boolean                                           | true     | none         | dislocation enabled/disabled                                                                                                                  |
| feeTierId             | [FeeTierId](#schemafeetierid)                     | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)                                                                     |
| finalValue            | string                                            | false    | none         | value of assets (base and quote) in USD amount when AMM instruction was terminated, only for AMM instruction with `CLOSED` status             |
| impermanentLoss       | string                                            | true     | none         | impermanent loss                                                                                                                              |
| initialBasePrice      | string                                            | true     | none         | price of base asset when AMM instruction was created                                                                                          |
| initialQuotePrice     | string                                            | true     | none         | price of quote asset when AMM instruction was created                                                                                         |
| initialValue          | string                                            | true     | none         | value of assets (base and quote) in USD amount when AMM instruction was created                                                               |
| lowerBound            | string                                            | true     | none         | lower bound of price range, in quote currency                                                                                                 |
| price                 | string                                            | true     | none         | current price of AMM, see [Get Tick By Symbol](#get-/v1/markets/-symbol-/tick)                                                                |
| quoteCurrentQuantity  | string                                            | false    | none         | amount of quote asset this AMM instruction currently holds, only for AMM instruction with `OPEN` status                                       |
| quoteInvestQuantity   | string                                            | true     | none         | initial quote investment                                                                                                                      |
| quotePrice            | string                                            | true     | none         | current price of quote asset                                                                                                                  |
| quoteWithdrawQuantity | string                                            | true     | none         | amount of quote asset returned when AMM instruction is terminated                                                                             |
| lastDistributedPrice  | string                                            | false    | none         | (Perpetual market only) The price used at the time of settlement for AMM Instructions that can be used to determine mtmPnl and the actual Pnl |
| requestId             | [RequestID](#schemarequestid)                     | true     | none         | unique request ID                                                                                                                             |
| staticSpreadFee       | string                                            | true     | none         | static spread fee, see [Get Market By Symbol](#get-/v1/markets/-symbol-)                                                                      |
| updatedAtDatetime     | [DateTime](#schemadatetime)                       | false    | none         | denotes the time the AMM instruction was updated by the exchange, ISO 8601 with millisecond as string                                         |
| updatedAtTimestamp    | [TimeStampAsString](#schematimestampasstring)     | true     | none         | denotes the time the AMM instruction was updated by the exchange                                                                              |
| upperBound            | string                                            | true     | none         | upper bound of price range, in quote currency                                                                                                 |
| yieldEarn             | string                                            | true     | none         | amount of money earned in USD                                                                                                                 |

<h2 id="tocS_ObfuscatedTradeWithId">ObfuscatedTradeWithId</h2>
<!-- backwards compatibility -->
<a id="schemaobfuscatedtradewithid"></a>
<a id="schema_ObfuscatedTradeWithId"></a>
<a id="tocSobfuscatedtradewithid"></a>
<a id="tocsobfuscatedtradewithid"></a>

```json
{
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
```

### Properties

| Name               | Type                                          | Required | Restrictions | Description                                                                                  |
| ------------------ | --------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------- |
| tradeId            | [TradeID](#schematradeid)                     | true     | none         | unique trade ID                                                                              |
| symbol             | [MarketSymbol](#schemamarketsymbol)           | true     | none         | market symbol                                                                                |
| price              | [AssetValue](#schemaassetvalue)               | true     | none         | price                                                                                        |
| quantity           | [AssetValue](#schemaassetvalue)               | true     | none         | quantity                                                                                     |
| side               | [OrderSideAsString](#schemaordersideasstring) | true     | none         | order side                                                                                   |
| isTaker            | [Boolean](#schemaboolean)                     | true     | none         | denotes whether this is a taker's trade                                                      |
| createdAtDatetime  | [DateTime](#schemadatetime)                   | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string |
| createdAtTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time the trade was executed by the exchange                                      |

<h2 id="tocS_ObfuscatedTrade">ObfuscatedTrade</h2>
<!-- backwards compatibility -->
<a id="schemaobfuscatedtrade"></a>
<a id="schema_ObfuscatedTrade"></a>
<a id="tocSobfuscatedtrade"></a>
<a id="tocsobfuscatedtrade"></a>

```json
{
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
```

### Properties

| Name               | Type                                          | Required | Restrictions | Description                                                                                  |
| ------------------ | --------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------- |
| tradeId            | [TradeID](#schematradeid)                     | true     | none         | unique trade ID                                                                              |
| symbol             | [MarketSymbol](#schemamarketsymbol)           | true     | none         | market symbol                                                                                |
| price              | [AssetValue](#schemaassetvalue)               | true     | none         | price                                                                                        |
| quantity           | [AssetValue](#schemaassetvalue)               | true     | none         | quantity                                                                                     |
| side               | [OrderSideAsString](#schemaordersideasstring) | true     | none         | order side                                                                                   |
| isTaker            | [Boolean](#schemaboolean)                     | true     | none         | denotes whether this is a taker's trade                                                      |
| createdAtDatetime  | [DateTime](#schemadatetime)                   | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string |
| createdAtTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time the trade was executed by the exchange                                      |

<h2 id="tocS_CustodyApiWithdrawalCommand">CustodyApiWithdrawalCommand</h2>
<!-- backwards compatibility -->
<a id="schemacustodyapiwithdrawalcommand"></a>
<a id="schema_CustodyApiWithdrawalCommand"></a>
<a id="tocScustodyapiwithdrawalcommand"></a>
<a id="tocscustodyapiwithdrawalcommand"></a>

```json
{
  "type": "object",
  "required": ["commandType", "destinationId", "symbol", "network", "quantity"],
  "properties": {
    "commandType": {
      "description": "the command type, it must be 'V1WithdrawalChallenge'",
      "type": "string",
      "example": "V1WithdrawalChallenge"
    },
    "destinationId": {
      "allOf": [
        {
          "type": "string",
          "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
          "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
        }
      ]
    },
    "symbol": {
      "example": "USDC",
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "network": {
      "example": "ETH",
      "allOf": [
        {
          "type": "string",
          "example": "EOS",
          "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
        }
      ]
    },
    "quantity": {
      "example": "100000.000001",
      "allOf": [
        {
          "type": "string",
          "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
          "example": "100000.00"
        }
      ]
    }
  }
}
```

### Properties

| Name          | Type                                                | Required | Restrictions | Description                                                                                                                                                                           |
| ------------- | --------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| commandType   | string                                              | true     | none         | the command type, it must be 'V1WithdrawalChallenge'                                                                                                                                  |
| destinationId | [CustodyDestinationID](#schemacustodydestinationid) | true     | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account                                                                                          |
| symbol        | [CustodySymbol](#schemacustodysymbol)               | true     | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                                                                                          |
| network       | [NetworkID](#schemanetworkid)                       | true     | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                                                                                                                           |
| quantity      | [CustodyQuantity](#schemacustodyquantity)           | true     | none         | total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted. |

<h2 id="tocS_CustodyApiEcdsaWithdrawalCommand">CustodyApiEcdsaWithdrawalCommand</h2>
<!-- backwards compatibility -->
<a id="schemacustodyapiecdsawithdrawalcommand"></a>
<a id="schema_CustodyApiEcdsaWithdrawalCommand"></a>
<a id="tocScustodyapiecdsawithdrawalcommand"></a>
<a id="tocscustodyapiecdsawithdrawalcommand"></a>

```json
{
  "type": "object",
  "required": ["commandType", "destinationId", "symbol", "network", "quantity"],
  "properties": {
    "commandType": {
      "description": "the command type, it must be 'V1Withdrawal'",
      "type": "string",
      "example": "V1Withdrawal"
    },
    "destinationId": {
      "allOf": [
        {
          "type": "string",
          "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
          "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
        }
      ]
    },
    "symbol": {
      "example": "USDC",
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "network": {
      "example": "ETH",
      "allOf": [
        {
          "type": "string",
          "example": "EOS",
          "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
        }
      ]
    },
    "quantity": {
      "example": "100000.000001",
      "allOf": [
        {
          "type": "string",
          "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
          "example": "100000.00"
        }
      ]
    }
  }
}
```

### Properties

| Name          | Type                                                | Required | Restrictions | Description                                                                                                                                                                           |
| ------------- | --------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| commandType   | string                                              | true     | none         | the command type, it must be 'V1Withdrawal'                                                                                                                                           |
| destinationId | [CustodyDestinationID](#schemacustodydestinationid) | true     | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account                                                                                          |
| symbol        | [CustodySymbol](#schemacustodysymbol)               | true     | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                                                                                          |
| network       | [NetworkID](#schemanetworkid)                       | true     | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                                                                                                                           |
| quantity      | [CustodyQuantity](#schemacustodyquantity)           | true     | none         | total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted. |

<h2 id="tocS_CustodyApiWithdrawalRequest">CustodyApiWithdrawalRequest</h2>
<!-- backwards compatibility -->
<a id="schemacustodyapiwithdrawalrequest"></a>
<a id="schema_CustodyApiWithdrawalRequest"></a>
<a id="tocScustodyapiwithdrawalrequest"></a>
<a id="tocscustodyapiwithdrawalrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "type": "string",
      "description": "Withdrawal nonce, independent of header nonce, recommendation re-use header nonce",
      "example": "1628376611"
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "withdrawal command",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "destinationId",
            "symbol",
            "network",
            "quantity"
          ],
          "properties": {
            "commandType": {
              "description": "the command type, it must be 'V1WithdrawalChallenge'",
              "type": "string",
              "example": "V1WithdrawalChallenge"
            },
            "destinationId": {
              "allOf": [
                {
                  "type": "string",
                  "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
                  "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
                }
              ]
            },
            "symbol": {
              "example": "USDC",
              "allOf": [
                {
                  "type": "string",
                  "example": "USDC",
                  "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
                }
              ]
            },
            "network": {
              "example": "ETH",
              "allOf": [
                {
                  "type": "string",
                  "example": "EOS",
                  "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
                }
              ]
            },
            "quantity": {
              "example": "100000.000001",
              "allOf": [
                {
                  "type": "string",
                  "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
                  "example": "100000.00"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                              | Required | Restrictions | Description                                                                                       |
| ---------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                     | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | string                                                            | true     | none         | Withdrawal nonce, independent of header nonce, recommendation re-use header nonce                 |
| authorizer | [Authorizer](#schemaauthorizer)                                   | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CustodyApiWithdrawalCommand](#schemacustodyapiwithdrawalcommand) | true     | none         | withdrawal command                                                                                |

<h2 id="tocS_CustodyApiEcdsaWithdrawalRequest">CustodyApiEcdsaWithdrawalRequest</h2>
<!-- backwards compatibility -->
<a id="schemacustodyapiecdsawithdrawalrequest"></a>
<a id="schema_CustodyApiEcdsaWithdrawalRequest"></a>
<a id="tocScustodyapiecdsawithdrawalrequest"></a>
<a id="tocscustodyapiecdsawithdrawalrequest"></a>

```json
{
  "type": "object",
  "required": ["timestamp", "nonce", "authorizer", "command"],
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
    "nonce": {
      "type": "string",
      "description": "a UUID withdrawal nonce to protect against replay attacks",
      "example": "1628376611"
    },
    "authorizer": {
      "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "03E02367E8C900000500000000000000",
          "description": "JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)"
        }
      ]
    },
    "command": {
      "description": "withdrawal command",
      "allOf": [
        {
          "type": "object",
          "required": [
            "commandType",
            "destinationId",
            "symbol",
            "network",
            "quantity"
          ],
          "properties": {
            "commandType": {
              "description": "the command type, it must be 'V1Withdrawal'",
              "type": "string",
              "example": "V1Withdrawal"
            },
            "destinationId": {
              "allOf": [
                {
                  "type": "string",
                  "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
                  "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
                }
              ]
            },
            "symbol": {
              "example": "USDC",
              "allOf": [
                {
                  "type": "string",
                  "example": "USDC",
                  "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
                }
              ]
            },
            "network": {
              "example": "ETH",
              "allOf": [
                {
                  "type": "string",
                  "example": "EOS",
                  "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
                }
              ]
            },
            "quantity": {
              "example": "100000.000001",
              "allOf": [
                {
                  "type": "string",
                  "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
                  "example": "100000.00"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name       | Type                                                                        | Required | Restrictions | Description                                                                                       |
| ---------- | --------------------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp  | [TimeStampAsString](#schematimestampasstring)                               | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| nonce      | string                                                                      | true     | none         | a UUID withdrawal nonce to protect against replay attacks                                         |
| authorizer | [Authorizer](#schemaauthorizer)                                             | true     | none         | JWT authorizer you obtain along with the [JWT token](#overview--generate-a-jwt-token)             |
| command    | [CustodyApiEcdsaWithdrawalCommand](#schemacustodyapiecdsawithdrawalcommand) | true     | none         | withdrawal command                                                                                |

<h2 id="tocS_CustodyApiWithdrawalResponse">CustodyApiWithdrawalResponse</h2>
<!-- backwards compatibility -->
<a id="schemacustodyapiwithdrawalresponse"></a>
<a id="schema_CustodyApiWithdrawalResponse"></a>
<a id="tocScustodyapiwithdrawalresponse"></a>
<a id="tocscustodyapiwithdrawalresponse"></a>

```json
{
  "type": "object",
  "properties": {
    "statusReason": {
      "description": "status reason, describes why withdrawal challenge is in a specific state",
      "type": "string",
      "example": "Withdrawal accepted"
    },
    "statusReasonCode": {
      "description": "status reason code, see [details](#overview--error--rejection-codes)",
      "type": "integer",
      "example": 1001
    },
    "custodyTransactionId": {
      "allOf": [
        {
          "type": "string",
          "example": "DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979",
          "description": "unique identifier for tracking a withdrawal during signing and in history"
        }
      ]
    }
  }
}
```

### Properties

| Name                 | Type                                                | Required | Restrictions | Description                                                               |
| -------------------- | --------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------- |
| statusReason         | string                                              | false    | none         | status reason, describes why withdrawal challenge is in a specific state  |
| statusReasonCode     | integer                                             | false    | none         | status reason code, see [details](#overview--error--rejection-codes)      |
| custodyTransactionId | [CustodyTransactionID](#schemacustodytransactionid) | false    | none         | unique identifier for tracking a withdrawal during signing and in history |

<h2 id="tocS_CustodyHistory">CustodyHistory</h2>
<!-- backwards compatibility -->
<a id="schemacustodyhistory"></a>
<a id="schema_CustodyHistory"></a>
<a id="tocScustodyhistory"></a>
<a id="tocscustodyhistory"></a>

```json
{
  "type": "object",
  "properties": {
    "custodyTransactionId": {
      "allOf": [
        {
          "type": "string",
          "example": "DB:9e6304a08c9cc2a33e6bc6429a088eae2a6b940c8e312aede3a3780257b9b979",
          "description": "unique identifier for tracking a deposit or withdrawal"
        }
      ]
    },
    "direction": {
      "allOf": [
        {
          "type": "string",
          "example": "DEPOSIT",
          "description": "direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL'"
        }
      ]
    },
    "quantity": {
      "allOf": [
        {
          "type": "string",
          "description": "total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted.",
          "example": "100000.00"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "network": {
      "allOf": [
        {
          "type": "string",
          "example": "EOS",
          "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
        }
      ]
    },
    "fee": {
      "allOf": [
        {
          "type": "string",
          "example": "3.00",
          "description": "withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    },
    "memo": {
      "allOf": [
        {
          "type": "string",
          "example": "925891241",
          "description": "memo or destination tag used during deposit to help identify account to credit funds to"
        }
      ]
    },
    "createdAtDateTime": {
      "allOf": [
        {
          "type": "string",
          "example": "2022-09-16T07:56:15.000Z",
          "description": "time of initial transaction"
        }
      ]
    },
    "status": {
      "allOf": [
        {
          "type": "string",
          "example": "COMPLETE",
          "description": "one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED'"
        }
      ]
    },
    "transactionDetails": {
      "allOf": [
        {
          "type": "object",
          "properties": {
            "address": {
              "type": "string",
              "description": "crypto network address",
              "example": "0xb0a64d976972d87b0783eeb1ff88306cd1891f02"
            },
            "blockchainTxId": {
              "type": "string",
              "description": "transaction id on chain",
              "example": "0xec557f2c7278d2dae2d98a27b9bd43f386789a4209090cbbd11595f1bed4a4c2"
            },
            "swiftUetr": {
              "type": "string",
              "description": "unique end-to-end-transaction reference for swift transactions",
              "example": "b55aa5cd-baa2-4122-8c17-ae9b856ae36a"
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name                 | Type                                                              | Required | Restrictions | Description                                                                                                                                                                           |
| -------------------- | ----------------------------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| custodyTransactionId | [CustodyTransactionHistoryID](#schemacustodytransactionhistoryid) | false    | none         | unique identifier for tracking a deposit or withdrawal                                                                                                                                |
| direction            | [CustodyDirection](#schemacustodydirection)                       | false    | none         | direction of transaction from API user's perspective, 'DEPOSIT' or 'WITHDRAWAL'                                                                                                       |
| quantity             | [CustodyQuantity](#schemacustodyquantity)                         | false    | none         | total quantity of symbol to withdraw including fee in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) - quantity received will have fee subtracted. |
| symbol               | [CustodySymbol](#schemacustodysymbol)                             | false    | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                                                                                          |
| network              | [NetworkID](#schemanetworkid)                                     | false    | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                                                                                                                           |
| fee                  | [CustodyWithdrawalFee](#schemacustodywithdrawalfee)               | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)                                                                           |
| memo                 | [CustodyDepositMemo](#schemacustodydepositmemo)                   | false    | none         | memo or destination tag used during deposit to help identify account to credit funds to                                                                                               |
| createdAtDateTime    | [CustodyCreatedAtDateTime](#schemacustodycreatedatdatetime)       | false    | none         | time of initial transaction                                                                                                                                                           |
| status               | [CustodyTransactionStatus](#schemacustodytransactionstatus)       | false    | none         | one of 'PENDING', 'COMPLETE', 'CANCELLED', 'FAILED'                                                                                                                                   |
| transactionDetails   | [CustodyTransactionDetails](#schemacustodytransactiondetails)     | false    | none         | none                                                                                                                                                                                  |

<h2 id="tocS_CustodyCryptoDepositInstructions">CustodyCryptoDepositInstructions</h2>
<!-- backwards compatibility -->
<a id="schemacustodycryptodepositinstructions"></a>
<a id="schema_CustodyCryptoDepositInstructions"></a>
<a id="tocScustodycryptodepositinstructions"></a>
<a id="tocscustodycryptodepositinstructions"></a>

```json
{
  "network": "ETH",
  "symbol": "USDC",
  "address": "0xb0a64d976972d87b0783eeb1ff88306cd1891f02"
}
```

### Properties

| Name    | Type                                                  | Required | Restrictions | Description                                                                             |
| ------- | ----------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------- |
| network | [NetworkID](#schemanetworkid)                         | true     | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                             |
| symbol  | [CustodySymbol](#schemacustodysymbol)                 | true     | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                            |
| memo    | [CustodyDepositMemo](#schemacustodydepositmemo)       | false    | none         | memo or destination tag used during deposit to help identify account to credit funds to |
| address | [CustodyNetworkAddress](#schemacustodynetworkaddress) | true     | none         | an address on the given network                                                         |

<h2 id="tocS_CustodyCryptoWithdrawalInstructions">CustodyCryptoWithdrawalInstructions</h2>
<!-- backwards compatibility -->
<a id="schemacustodycryptowithdrawalinstructions"></a>
<a id="schema_CustodyCryptoWithdrawalInstructions"></a>
<a id="tocScustodycryptowithdrawalinstructions"></a>
<a id="tocscustodycryptowithdrawalinstructions"></a>

```json
{
  "type": "object",
  "properties": {
    "network": {
      "example": "ETH",
      "allOf": [
        {
          "type": "string",
          "example": "EOS",
          "description": "the network of the native coin or token, e.g. BTC, ETH, EOS"
        }
      ]
    },
    "symbol": {
      "example": "USDC",
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "address": {
      "allOf": [
        {
          "type": "string",
          "example": "0xb0a64d976972d87b0783eeb1ff88306cd1891f02",
          "description": "an address on the given network"
        }
      ]
    },
    "fee": {
      "allOf": [
        {
          "type": "string",
          "example": "3.00",
          "description": "withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    },
    "memo": {
      "allOf": [
        {
          "type": "string",
          "example": "MZAXEMRXA",
          "description": "memo or destination tag that will be used as a reference on transaction"
        }
      ]
    },
    "label": {
      "allOf": [
        {
          "type": "string",
          "example": "Our cold wallet",
          "description": "descriptive label of destination provided by user"
        }
      ]
    },
    "destinationId": {
      "allOf": [
        {
          "type": "string",
          "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
          "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
        }
      ]
    }
  }
}
```

### Properties

| Name          | Type                                                    | Required | Restrictions | Description                                                                                                 |
| ------------- | ------------------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| network       | [NetworkID](#schemanetworkid)                           | false    | none         | the network of the native coin or token, e.g. BTC, ETH, EOS                                                 |
| symbol        | [CustodySymbol](#schemacustodysymbol)                   | false    | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                |
| address       | [CustodyNetworkAddress](#schemacustodynetworkaddress)   | false    | none         | an address on the given network                                                                             |
| fee           | [CustodyWithdrawalFee](#schemacustodywithdrawalfee)     | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |
| memo          | [CustodyWithdrawalMemo](#schemacustodywithdrawalmemo)   | false    | none         | memo or destination tag that will be used as a reference on transaction                                     |
| label         | [CustodyWithdrawalLabel](#schemacustodywithdrawallabel) | false    | none         | descriptive label of destination provided by user                                                           |
| destinationId | [CustodyDestinationID](#schemacustodydestinationid)     | false    | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account                |

<h2 id="tocS_CustodyBankDetails">CustodyBankDetails</h2>
<!-- backwards compatibility -->
<a id="schemacustodybankdetails"></a>
<a id="schema_CustodyBankDetails"></a>
<a id="tocScustodybankdetails"></a>
<a id="tocscustodybankdetails"></a>

```json
{
  "type": "object",
  "properties": {
    "name": {
      "allOf": [
        {
          "type": "string",
          "example": "Silvergate Bank",
          "description": "name of bank"
        }
      ]
    },
    "physicalAddress": {
      "allOf": [
        {
          "type": "string",
          "description": "physical location of bank",
          "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
        }
      ]
    },
    "routingCode": {
      "allOf": [
        {
          "type": "string",
          "description": "routing code of bank",
          "example": "322286803"
        }
      ]
    }
  }
}
```

### Properties

| Name            | Type                                                            | Required | Restrictions | Description               |
| --------------- | --------------------------------------------------------------- | -------- | ------------ | ------------------------- |
| name            | [CustodyBankName](#schemacustodybankname)                       | false    | none         | name of bank              |
| physicalAddress | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress) | false    | none         | physical location of bank |
| routingCode     | [CustodyBankRoutingCode](#schemacustodybankroutingcode)         | false    | none         | routing code of bank      |

<h2 id="tocS_CustodyBankIntermediateDetails">CustodyBankIntermediateDetails</h2>
<!-- backwards compatibility -->
<a id="schemacustodybankintermediatedetails"></a>
<a id="schema_CustodyBankIntermediateDetails"></a>
<a id="tocScustodybankintermediatedetails"></a>
<a id="tocscustodybankintermediatedetails"></a>

```json
{
  "type": "object",
  "properties": {
    "name": {
      "example": "Middle Bank",
      "allOf": [
        {
          "type": "string",
          "example": "Silvergate Bank",
          "description": "name of bank"
        }
      ]
    },
    "physicalAddress": {
      "example": "523 Exchange Square, Canary Wharf, E14 2WA",
      "allOf": [
        {
          "type": "string",
          "description": "physical location of bank",
          "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
        }
      ]
    },
    "routingCode": {
      "example": "321176234",
      "allOf": [
        {
          "type": "string",
          "description": "routing code of bank",
          "example": "322286803"
        }
      ]
    }
  }
}
```

### Properties

| Name            | Type                                                            | Required | Restrictions | Description               |
| --------------- | --------------------------------------------------------------- | -------- | ------------ | ------------------------- |
| name            | [CustodyBankName](#schemacustodybankname)                       | false    | none         | name of bank              |
| physicalAddress | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress) | false    | none         | physical location of bank |
| routingCode     | [CustodyBankRoutingCode](#schemacustodybankroutingcode)         | false    | none         | routing code of bank      |

<h2 id="tocS_CustodyFiatDepositInstructions">CustodyFiatDepositInstructions</h2>
<!-- backwards compatibility -->
<a id="schemacustodyfiatdepositinstructions"></a>
<a id="schema_CustodyFiatDepositInstructions"></a>
<a id="tocScustodyfiatdepositinstructions"></a>
<a id="tocscustodyfiatdepositinstructions"></a>

```json
{
  "type": "object",
  "properties": {
    "network": {
      "type": "string",
      "example": "SWIFT",
      "description": "the network that the account belongs to and the transaction will be performed on SWIFT, ABA or SEPA"
    },
    "symbol": {
      "type": "string",
      "example": "USD",
      "description": "the currency associated with the account, e.g. USD, EUR"
    },
    "accountNumber": {
      "allOf": [
        {
          "type": "string",
          "description": "bank account number",
          "example": "9873481227"
        }
      ],
      "example": "5090022533",
      "description": "the Bullish account number, varies for SWIFT/ABA and SEPA"
    },
    "name": {
      "type": "string",
      "example": "Bullish (GI) Limited",
      "description": "official Bullish account holder name"
    },
    "physicalAddress": {
      "type": "string",
      "example": "26/F, The Centrium, 60 Wyndham Street, Central, Hong Kong",
      "description": "bullish entity's physical address for the bank account"
    },
    "memo": {
      "type": "string",
      "example": "8VZPKSGPA",
      "description": "client specific reference to identify which account desposits should be allocated to on the exhange"
    },
    "bank": {
      "allOf": [
        {
          "type": "object",
          "properties": {
            "name": {
              "allOf": [
                {
                  "type": "string",
                  "example": "Silvergate Bank",
                  "description": "name of bank"
                }
              ]
            },
            "physicalAddress": {
              "allOf": [
                {
                  "type": "string",
                  "description": "physical location of bank",
                  "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                }
              ]
            },
            "routingCode": {
              "allOf": [
                {
                  "type": "string",
                  "description": "routing code of bank",
                  "example": "322286803"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name            | Type                                                        | Required | Restrictions | Description                                                                                         |
| --------------- | ----------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| network         | string                                                      | false    | none         | the network that the account belongs to and the transaction will be performed on SWIFT, ABA or SEPA |
| symbol          | string                                                      | false    | none         | the currency associated with the account, e.g. USD, EUR                                             |
| accountNumber   | [CustodyBankAccountNumber](#schemacustodybankaccountnumber) | false    | none         | the Bullish account number, varies for SWIFT/ABA and SEPA                                           |
| name            | string                                                      | false    | none         | official Bullish account holder name                                                                |
| physicalAddress | string                                                      | false    | none         | bullish entity's physical address for the bank account                                              |
| memo            | string                                                      | false    | none         | client specific reference to identify which account desposits should be allocated to on the exhange |
| bank            | [CustodyBankDetails](#schemacustodybankdetails)             | false    | none         | none                                                                                                |

<h2 id="tocS_CustodyFiatWithdrawalInstructions">CustodyFiatWithdrawalInstructions</h2>
<!-- backwards compatibility -->
<a id="schemacustodyfiatwithdrawalinstructions"></a>
<a id="schema_CustodyFiatWithdrawalInstructions"></a>
<a id="tocScustodyfiatwithdrawalinstructions"></a>
<a id="tocscustodyfiatwithdrawalinstructions"></a>

```json
{
  "type": "object",
  "properties": {
    "destinationId": {
      "allOf": [
        {
          "type": "string",
          "example": "1560ec0b406c0d909bb9f5f827dd6aa14a1f638884f33a2a3134878102e78038",
          "description": "destination id provided by bullish that uniquely identifies a whitelisted address or account"
        }
      ]
    },
    "accountNumber": {
      "allOf": [
        {
          "type": "string",
          "description": "bank account number",
          "example": "9873481227"
        }
      ]
    },
    "network": {
      "allOf": [
        {
          "type": "string",
          "description": "the fiat network, e.g. SWIFT, ABA or SEPA",
          "example": "SWIFT"
        }
      ]
    },
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "example": "USD",
          "description": "symbol representing fiat currency, e.g. USD, EUR"
        }
      ]
    },
    "name": {
      "allOf": [
        {
          "type": "string",
          "example": "Silvergate Bank",
          "description": "name of bank"
        }
      ]
    },
    "physicalAddress": {
      "allOf": [
        {
          "type": "string",
          "description": "physical location of bank",
          "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
        }
      ]
    },
    "fee": {
      "allOf": [
        {
          "type": "string",
          "example": "3.00",
          "description": "withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    },
    "memo": {
      "allOf": [
        {
          "type": "string",
          "example": "MZAXEMRXA",
          "description": "memo or destination tag that will be used as a reference on transaction"
        }
      ]
    },
    "bank": {
      "allOf": [
        {
          "type": "object",
          "properties": {
            "name": {
              "allOf": [
                {
                  "type": "string",
                  "example": "Silvergate Bank",
                  "description": "name of bank"
                }
              ]
            },
            "physicalAddress": {
              "allOf": [
                {
                  "type": "string",
                  "description": "physical location of bank",
                  "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                }
              ]
            },
            "routingCode": {
              "allOf": [
                {
                  "type": "string",
                  "description": "routing code of bank",
                  "example": "322286803"
                }
              ]
            }
          }
        }
      ]
    },
    "intermediaryBank": {
      "allOf": [
        {
          "type": "object",
          "properties": {
            "name": {
              "example": "Middle Bank",
              "allOf": [
                {
                  "type": "string",
                  "example": "Silvergate Bank",
                  "description": "name of bank"
                }
              ]
            },
            "physicalAddress": {
              "example": "523 Exchange Square, Canary Wharf, E14 2WA",
              "allOf": [
                {
                  "type": "string",
                  "description": "physical location of bank",
                  "example": "4250 Executive Square Suite 300 La Jolla, CA 92037"
                }
              ]
            },
            "routingCode": {
              "example": "321176234",
              "allOf": [
                {
                  "type": "string",
                  "description": "routing code of bank",
                  "example": "322286803"
                }
              ]
            }
          }
        }
      ]
    }
  }
}
```

### Properties

| Name             | Type                                                                    | Required | Restrictions | Description                                                                                                 |
| ---------------- | ----------------------------------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| destinationId    | [CustodyDestinationID](#schemacustodydestinationid)                     | false    | none         | destination id provided by bullish that uniquely identifies a whitelisted address or account                |
| accountNumber    | [CustodyBankAccountNumber](#schemacustodybankaccountnumber)             | false    | none         | bank account number                                                                                         |
| network          | [CustodyBankNetworkID](#schemacustodybanknetworkid)                     | false    | none         | the fiat network, e.g. SWIFT, ABA or SEPA                                                                   |
| symbol           | [CustodyFiatSymbol](#schemacustodyfiatsymbol)                           | false    | none         | symbol representing fiat currency, e.g. USD, EUR                                                            |
| name             | [CustodyBankName](#schemacustodybankname)                               | false    | none         | name of bank                                                                                                |
| physicalAddress  | [CustodyPhysicalBankAddress](#schemacustodyphysicalbankaddress)         | false    | none         | physical location of bank                                                                                   |
| fee              | [CustodyWithdrawalFee](#schemacustodywithdrawalfee)                     | false    | none         | withdrawal fee charged in units of symbol, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |
| memo             | [CustodyWithdrawalMemo](#schemacustodywithdrawalmemo)                   | false    | none         | memo or destination tag that will be used as a reference on transaction                                     |
| bank             | [CustodyBankDetails](#schemacustodybankdetails)                         | false    | none         | none                                                                                                        |
| intermediaryBank | [CustodyBankIntermediateDetails](#schemacustodybankintermediatedetails) | false    | none         | none                                                                                                        |

<h2 id="tocS_CustodyLimits">CustodyLimits</h2>
<!-- backwards compatibility -->
<a id="schemacustodylimits"></a>
<a id="schema_CustodyLimits"></a>
<a id="tocScustodylimits"></a>
<a id="tocscustodylimits"></a>

```json
{
  "type": "object",
  "properties": {
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "example": "USDC",
          "description": "symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB"
        }
      ]
    },
    "available": {
      "allOf": [
        {
          "type": "string",
          "example": "20000.0",
          "description": "remaining limit on amount of coin or token that could be withdrawn now, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    },
    "twentyFourHour": {
      "allOf": [
        {
          "type": "string",
          "example": "1000000.00",
          "description": "limit on amount of coin or token that can be withdrawn over a 24 hour period, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)"
        }
      ]
    }
  }
}
```

### Properties

| Name           | Type                                                                      | Required | Restrictions | Description                                                                                                                                                                   |
| -------------- | ------------------------------------------------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| symbol         | [CustodySymbol](#schemacustodysymbol)                                     | false    | none         | symbol representing coin or token, e.g. USDC, BTC, ETH, SHIB                                                                                                                  |
| available      | [CustodyAvailableWithdrawalLimit](#schemacustodyavailablewithdrawallimit) | false    | none         | remaining limit on amount of coin or token that could be withdrawn now, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei)       |
| twentyFourHour | [Custody24HWithdrawalLimit](#schemacustody24hwithdrawallimit)             | false    | none         | limit on amount of coin or token that can be withdrawn over a 24 hour period, in units of the symbol itself, not in smaller denominations (e.g. BTC not Satoshi, ETH not Wei) |

<h2 id="tocS_Trade">Trade</h2>
<!-- backwards compatibility -->
<a id="schematrade"></a>
<a id="schema_Trade"></a>
<a id="tocStrade"></a>
<a id="tocstrade"></a>

```json
{
  "type": "object",
  "required": [
    "tradeId",
    "orderId",
    "symbol",
    "price",
    "quantity",
    "quoteAmount",
    "baseFee",
    "quoteFee",
    "side",
    "isTaker",
    "tradeRebateAmount",
    "tradeRebateAssetSymbol",
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
    "orderId": {
      "description": "unique order ID",
      "allOf": [
        {
          "type": "string",
          "example": "297735387747975680"
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
      "description": "price, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quantity": {
      "description": "quantity, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteAmount": {
      "description": "quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "baseFee": {
      "description": "base fee, see [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "quoteFee": {
      "description": "quote fee, see [asset value](#overview--price-and-quantity-precision) format",
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
    "tradeRebateAmount": {
      "description": "amount of rebate that is credited to the user as part of the trade.",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "tradeRebateAssetSymbol": {
      "description": "the symbol of the asset in which the rebate is paid",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "USDC"
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
```

### Properties

| Name                   | Type                                          | Required | Restrictions | Description                                                                                                   |
| ---------------------- | --------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------------------- |
| tradeId                | [TradeID](#schematradeid)                     | true     | none         | unique trade ID                                                                                               |
| orderId                | [OrderID](#schemaorderid)                     | true     | none         | unique order ID                                                                                               |
| symbol                 | [MarketSymbol](#schemamarketsymbol)           | true     | none         | market symbol                                                                                                 |
| price                  | [AssetValue](#schemaassetvalue)               | true     | none         | price, see [asset value](#overview--price-and-quantity-precision) format                                      |
| quantity               | [AssetValue](#schemaassetvalue)               | true     | none         | quantity, see [asset value](#overview--price-and-quantity-precision) format                                   |
| quoteAmount            | [AssetValue](#schemaassetvalue)               | true     | none         | quote quantity deducted from asset account, see [asset value](#overview--price-and-quantity-precision) format |
| baseFee                | [AssetValue](#schemaassetvalue)               | true     | none         | base fee, see [asset value](#overview--price-and-quantity-precision) format                                   |
| quoteFee               | [AssetValue](#schemaassetvalue)               | true     | none         | quote fee, see [asset value](#overview--price-and-quantity-precision) format                                  |
| side                   | [OrderSideAsString](#schemaordersideasstring) | true     | none         | order side                                                                                                    |
| isTaker                | [Boolean](#schemaboolean)                     | true     | none         | denotes whether this is a taker's trade                                                                       |
| tradeRebateAmount      | [AssetValue](#schemaassetvalue)               | true     | none         | amount of rebate that is credited to the user as part of the trade.                                           |
| tradeRebateAssetSymbol | [QuoteAssetSymbol](#schemaquoteassetsymbol)   | true     | none         | the symbol of the asset in which the rebate is paid                                                           |
| createdAtDatetime      | [DateTime](#schemadatetime)                   | true     | none         | denotes the time the trade was executed by the exchange, ISO 8601 with millisecond as string                  |
| createdAtTimestamp     | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time the trade was executed by the exchange                                                       |

<h2 id="tocS_Tick">Tick</h2>
<!-- backwards compatibility -->
<a id="schematick"></a>
<a id="schema_Tick"></a>
<a id="tocStick"></a>
<a id="tocstick"></a>

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

### Properties

| Name               | Type                                          | Required | Restrictions | Description                                                                                                                                                                            |
| ------------------ | --------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| createdAtDatetime  | [DateTime](#schemadatetime)                   | true     | none         | denotes the time of the current tick on the exchange, ISO 8601 with millisecond as string                                                                                              |
| createdAtTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | denotes the time of the current tick on the exchange                                                                                                                                   |
| high               | [AssetValue](#schemaassetvalue)               | true     | none         | highest price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                       |
| low                | [AssetValue](#schemaassetvalue)               | true     | none         | lowest price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                        |
| bestBid            | [AssetValue](#schemaassetvalue)               | true     | none         | current best bid (buy) price, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| bidVolume          | [AssetValue](#schemaassetvalue)               | true     | none         | current best bid (buy) quantity (may be missing or undefined), see [asset value](#overview--price-and-quantity-precision) format                                                       |
| bestAsk            | [AssetValue](#schemaassetvalue)               | true     | none         | current best ask (sell) price, see [asset value](#overview--price-and-quantity-precision) format                                                                                       |
| askVolume          | [AssetValue](#schemaassetvalue)               | true     | none         | current best ask (sell) quantity (may be missing or undefined), see [asset value](#overview--price-and-quantity-precision) format                                                      |
| vwap               | [AssetValue](#schemaassetvalue)               | true     | none         | volume weighed average price, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| open               | [AssetValue](#schemaassetvalue)               | true     | none         | opening price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                       |
| close              | [AssetValue](#schemaassetvalue)               | true     | none         | price of last trade (closing price for current period), see [asset value](#overview--price-and-quantity-precision) format                                                              |
| last               | [AssetValue](#schemaassetvalue)               | true     | none         | price of last trade (closing price for current period), see [asset value](#overview--price-and-quantity-precision) format                                                              |
| change             | [AssetValue](#schemaassetvalue)               | true     | none         | absolute change, `last - open`, see [asset value](#overview--price-and-quantity-precision) format                                                                                      |
| percentage         | [AssetValue](#schemaassetvalue)               | true     | none         | relative change, `(change/open) * 100`, see [asset value](#overview--price-and-quantity-precision) format                                                                              |
| average            | [AssetValue](#schemaassetvalue)               | true     | none         | average price, `(last + open) / 2`, see [asset value](#overview--price-and-quantity-precision) format                                                                                  |
| baseVolume         | [AssetValue](#schemaassetvalue)               | true     | none         | volume of base asset traded for last 24 hours, see [asset value](#overview--price-and-quantity-precision) format                                                                       |
| quoteVolume        | [AssetValue](#schemaassetvalue)               | true     | none         | volume of quote asset traded for last 24 hours, see [asset value](#overview--price-and-quantity-precision) format                                                                      |
| bancorPrice        | [AssetValue](#schemaassetvalue)               | true     | none         | current price, see [asset value](#overview--price-and-quantity-precision) format                                                                                                       |
| markPrice          | string                                        | false    | none         | mark price represents the fair value of a contract at the current time.                                                                                                                |
| fundingRate        | string                                        | false    | none         | funding rate is used to calculate funding, which measures the relative difference between the index price and mark price.                                                              |
| openInterest       | string                                        | false    | none         | open interest is the total quantity of open long positions and short positions, see [asset value](#overview--price-and-quantity-precision) format (only applies to derivatives market) |
| lastTradeDatetime  | [DateTime](#schemadatetime)                   | true     | none         | time of the last trade on this symbol, ISO 8601 with millisecond as string                                                                                                             |
| lastTradeTimestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | time of the last trade on this symbol                                                                                                                                                  |
| lastTradeQuantity  | [AssetValue](#schemaassetvalue)               | true     | none         | quantity of the last trade on this symbol, see [asset value](#overview--price-and-quantity-precision) format                                                                           |
| ammData            | [allOf]                                       | true     | none         | AMM data of all available fee tiers.                                                                                                                                                   |

<h2 id="tocS_AmmData">AmmData</h2>
<!-- backwards compatibility -->
<a id="schemaammdata"></a>
<a id="schema_AmmData"></a>
<a id="tocSammdata"></a>
<a id="tocsammdata"></a>

```json
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
```

AMM data

### Properties

| Name                  | Type                          | Required | Restrictions | Description                                                               |
| --------------------- | ----------------------------- | -------- | ------------ | ------------------------------------------------------------------------- |
| feeTierId             | [FeeTierId](#schemafeetierid) | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-) |
| bidSpreadFee          | string                        | true     | none         | bid spread fee                                                            |
| askSpreadFee          | string                        | true     | none         | ask spread fee                                                            |
| baseReservesQuantity  | string                        | true     | none         | base reserves quantity                                                    |
| quoteReservesQuantity | string                        | true     | none         | quote reserves quantity                                                   |
| currentPrice          | string                        | true     | none         | current AMM price                                                         |

<h2 id="tocS_MarketSymbol">MarketSymbol</h2>
<!-- backwards compatibility -->
<a id="schemamarketsymbol"></a>
<a id="schema_MarketSymbol"></a>
<a id="tocSmarketsymbol"></a>
<a id="tocsmarketsymbol"></a>

```json
"BTCUSDC"
```

market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                   |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market |

<h2 id="tocS_DatedFutureMarketSymbol">DatedFutureMarketSymbol</h2>
<!-- backwards compatibility -->
<a id="schemadatedfuturemarketsymbol"></a>
<a id="schema_DatedFutureMarketSymbol"></a>
<a id="tocSdatedfuturemarketsymbol"></a>
<a id="tocsdatedfuturemarketsymbol"></a>

```json
"BTC-USDC-20241201"
```

market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for
DATED FUTURE markets.

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                                       |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for DATED FUTURE markets. |

<h2 id="tocS_PerpMarketSymbol">PerpMarketSymbol</h2>
<!-- backwards compatibility -->
<a id="schemaperpmarketsymbol"></a>
<a id="schema_PerpMarketSymbol"></a>
<a id="tocSperpmarketsymbol"></a>
<a id="tocsperpmarketsymbol"></a>

```json
"BTC-USDC-PERP"
```

market symbol. Eg `BTC-USDC-PERP` for PERPETUAL market

### Properties

| Name        | Type   | Required | Restrictions | Description                                            |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------ |
| _anonymous_ | string | false    | none         | market symbol. Eg `BTC-USDC-PERP` for PERPETUAL market |

<h2 id="tocS_FeeTier">FeeTier</h2>
<!-- backwards compatibility -->
<a id="schemafeetier"></a>
<a id="schema_FeeTier"></a>
<a id="tocSfeetier"></a>
<a id="tocsfeetier"></a>

```json
{
  "type": "object",
  "description": "unique fee tier",
  "required": ["feeTierId", "staticSpreadFee", "isDislocationEnabled"],
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
```

unique fee tier

### Properties

| Name                 | Type                          | Required | Restrictions | Description                                                               |
| -------------------- | ----------------------------- | -------- | ------------ | ------------------------------------------------------------------------- |
| feeTierId            | [FeeTierId](#schemafeetierid) | true     | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-) |
| staticSpreadFee      | string                        | true     | none         | static spread fee                                                         |
| isDislocationEnabled | boolean                       | true     | none         | dislocation enabled/disabled                                              |

<h2 id="tocS_FeeTierId">FeeTierId</h2>
<!-- backwards compatibility -->
<a id="schemafeetierid"></a>
<a id="schema_FeeTierId"></a>
<a id="tocSfeetierid"></a>
<a id="tocsfeetierid"></a>

```json
"1"
```

unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-)

### Properties

| Name        | Type   | Required | Restrictions | Description                                                               |
| ----------- | ------ | -------- | ------------ | ------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | unique fee tier ID, see [Get Market By Symbol](#get-/v1/markets/-symbol-) |

<h2 id="tocS_InstrumentId">InstrumentId</h2>
<!-- backwards compatibility -->
<a id="schemainstrumentid"></a>
<a id="schema_InstrumentId"></a>
<a id="tocSinstrumentid"></a>
<a id="tocsinstrumentid"></a>

```json
"BTC"
```

custody identifier for instrument

### Properties

| Name        | Type   | Required | Restrictions | Description                       |
| ----------- | ------ | -------- | ------------ | --------------------------------- |
| _anonymous_ | string | false    | none         | custody identifier for instrument |

<h2 id="tocS_AssetSymbol">AssetSymbol</h2>
<!-- backwards compatibility -->
<a id="schemaassetsymbol"></a>
<a id="schema_AssetSymbol"></a>
<a id="tocSassetsymbol"></a>
<a id="tocsassetsymbol"></a>

```json
"BTC"
```

asset symbol as denoted in the world

### Properties

| Name        | Type   | Required | Restrictions | Description                          |
| ----------- | ------ | -------- | ------------ | ------------------------------------ |
| _anonymous_ | string | false    | none         | asset symbol as denoted in the world |

<h2 id="tocS_UnderlyingAssetSymbol">UnderlyingAssetSymbol</h2>
<!-- backwards compatibility -->
<a id="schemaunderlyingassetsymbol"></a>
<a id="schema_UnderlyingAssetSymbol"></a>
<a id="tocSunderlyingassetsymbol"></a>
<a id="tocsunderlyingassetsymbol"></a>

```json
"BTC"
```

the underlying asset symbol of the options contract that is eligible for MMP
checks

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                         |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | the underlying asset symbol of the options contract that is eligible for MMP checks |

<h2 id="tocS_QuoteAssetSymbol">QuoteAssetSymbol</h2>
<!-- backwards compatibility -->
<a id="schemaquoteassetsymbol"></a>
<a id="schema_QuoteAssetSymbol"></a>
<a id="tocSquoteassetsymbol"></a>
<a id="tocsquoteassetsymbol"></a>

```json
"USDC"
```

asset symbol as denoted in the world

### Properties

| Name        | Type   | Required | Restrictions | Description                          |
| ----------- | ------ | -------- | ------------ | ------------------------------------ |
| _anonymous_ | string | false    | none         | asset symbol as denoted in the world |

<h2 id="tocS_AssetName">AssetName</h2>
<!-- backwards compatibility -->
<a id="schemaassetname"></a>
<a id="schema_AssetName"></a>
<a id="tocSassetname"></a>
<a id="tocsassetname"></a>

```json
"Bitcoin"
```

asset name

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | asset name  |

<h2 id="tocS_AssetID">AssetID</h2>
<!-- backwards compatibility -->
<a id="schemaassetid"></a>
<a id="schema_AssetID"></a>
<a id="tocSassetid"></a>
<a id="tocsassetid"></a>

```json
"1"
```

unique asset ID

### Properties

| Name        | Type   | Required | Restrictions | Description     |
| ----------- | ------ | -------- | ------------ | --------------- |
| _anonymous_ | string | false    | none         | unique asset ID |

<h2 id="tocS_CollateralBand">CollateralBand</h2>
<!-- backwards compatibility -->
<a id="schemacollateralband"></a>
<a id="schema_CollateralBand"></a>
<a id="tocScollateralband"></a>
<a id="tocscollateralband"></a>

```json
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
```

### Properties

| Name                 | Type   | Required | Restrictions | Description                                                                                                                                 |
| -------------------- | ------ | -------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| collateralPercentage | String | false    | none         | collateral percentage applied to the asset for this band - a value of 90.00 indicates 90% of the asset is eligible to be used as collateral |
| bandLimitUSD         | String | false    | none         | upper limit in USD for this band                                                                                                            |

<h2 id="tocS_UnderlyingAsset">UnderlyingAsset</h2>
<!-- backwards compatibility -->
<a id="schemaunderlyingasset"></a>
<a id="schema_UnderlyingAsset"></a>
<a id="tocSunderlyingasset"></a>
<a id="tocsunderlyingasset"></a>

```json
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
```

### Properties

| Name                       | Type   | Required | Restrictions | Description                                                                                                                   |
| -------------------------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------- |
| symbol                     | String | false    | none         | underlying asset symbol                                                                                                       |
| assetId                    | String | false    | none         | underlying asset ID                                                                                                           |
| bpmMinReturnStart          | String | false    | none         | start of the 1/1000 biggest downward price movement of an underlying asset over 6 hours                                       |
| bpmMinReturnEnd            | String | false    | none         | end of the 1/1000 biggest downward price movement of an underlying asset over 6 hours                                         |
| bpmMaxReturnStart          | String | false    | none         | start of the 1/1000 biggest upward price movement of an underlying asset over 6 hours                                         |
| bpmMaxReturnEnd            | String | false    | none         | end of the 1/1000 biggest upward price movement of an underlying asset over 6 hours                                           |
| marketRiskFloorPctStart    | String | false    | none         | the percentage range of risk reduction allowed for a portfolio                                                                |
| marketRiskFloorPctEnd      | String | false    | none         | the percentage range of risk reduction allowed for a portfolio                                                                |
| bpmTransitionDateTimeStart | String | false    | none         | the start datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset |
| bpmTransitionDateTimeEnd   | String | false    | none         | the end datetime which the values linearly transition from `bpmMinReturnStart` to `bpmMinReturnEnd` for an underlying asset   |

<h2 id="tocS_PriceQuantityTuple">PriceQuantityTuple</h2>
<!-- backwards compatibility -->
<a id="schemapricequantitytuple"></a>
<a id="schema_PriceQuantityTuple"></a>
<a id="tocSpricequantitytuple"></a>
<a id="tocspricequantitytuple"></a>

```json
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
```

### Properties

| Name               | Type                            | Required | Restrictions | Description                                                       |
| ------------------ | ------------------------------- | -------- | ------------ | ----------------------------------------------------------------- |
| price              | [AssetValue](#schemaassetvalue) | false    | none         | see [asset value](#overview--price-and-quantity-precision) format |
| priceLevelQuantity | [AssetValue](#schemaassetvalue) | false    | none         | see [asset value](#overview--price-and-quantity-precision) format |

<h2 id="tocS_OrderBook">OrderBook</h2>
<!-- backwards compatibility -->
<a id="schemaorderbook"></a>
<a id="schema_OrderBook"></a>
<a id="tocSorderbook"></a>
<a id="tocsorderbook"></a>

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

### Properties

| Name           | Type                                          | Required | Restrictions | Description                                                               |
| -------------- | --------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------- |
| bids           | [allOf]                                       | true     | none         | bids                                                                      |
| asks           | [allOf]                                       | true     | none         | asks                                                                      |
| datetime       | [DateTime](#schemadatetime)                   | true     | none         | date and time of order book snapshot, ISO 8601 with millisecond as string |
| timestamp      | [TimeStampAsString](#schematimestampasstring) | true     | none         | timestamp of order book snapshot                                          |
| sequenceNumber | integer                                       | true     | none         | an incremented unique identifier of the order book snapshot               |

<h2 id="tocS_IndexPrice">IndexPrice</h2>
<!-- backwards compatibility -->
<a id="schemaindexprice"></a>
<a id="schema_IndexPrice"></a>
<a id="tocSindexprice"></a>
<a id="tocsindexprice"></a>

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

### Properties

| Name               | Type                              | Required | Restrictions | Description                                   |
| ------------------ | --------------------------------- | -------- | ------------ | --------------------------------------------- |
| assetSymbol        | [AssetSymbol](#schemaassetsymbol) | true     | none         | Asset symbol                                  |
| price              | string                            | true     | none         | Asset price in USD                            |
| updatedAtDatetime  | [DateTime](#schemadatetime)       | true     | none         | Date and time when the index price is updated |
| updatedAtTimestamp | [TimeStamp](#schematimestamp)     | true     | none         | Timestamp when the index price is updated     |

<h2 id="tocS_Market">Market</h2>
<!-- backwards compatibility -->
<a id="schemamarket"></a>
<a id="schema_Market"></a>
<a id="tocSmarket"></a>
<a id="tocsmarket"></a>

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

### Properties

| Name                          | Type                                            | Required | Restrictions | Description                                                                                                                                                                                              |
| ----------------------------- | ----------------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| marketId                      | [MarketID](#schemamarketid)                     | true     | none         | unique market ID                                                                                                                                                                                         |
| symbol                        | [MarketSymbol](#schemamarketsymbol)             | true     | none         | market symbol                                                                                                                                                                                            |
| baseSymbol                    | [AssetSymbol](#schemaassetsymbol)               | true     | none         | base asset symbol (only applies to spot market)                                                                                                                                                          |
| underlyingBaseSymbol          | [AssetSymbol](#schemaassetsymbol)               | false    | none         | underlying base asset symbol (only applies to derivative market)                                                                                                                                         |
| quoteSymbol                   | [AssetSymbol](#schemaassetsymbol)               | true     | none         | quote asset symbol (only applies to spot market)                                                                                                                                                         |
| underlyingQuoteSymbol         | [AssetSymbol](#schemaassetsymbol)               | false    | none         | underlying quote asset symbol (only applies to derivative market)                                                                                                                                        |
| quoteAssetId                  | [AssetID](#schemaassetid)                       | true     | none         | quote asset id                                                                                                                                                                                           |
| baseAssetId                   | [AssetID](#schemaassetid)                       | true     | none         | base asset id                                                                                                                                                                                            |
| quotePrecision                | integer                                         | true     | none         | quote precision                                                                                                                                                                                          |
| basePrecision                 | integer                                         | true     | none         | base precision                                                                                                                                                                                           |
| pricePrecision                | integer                                         | true     | none         | number of decimal digits 'after the dot' for price                                                                                                                                                       |
| quantityPrecision             | integer                                         | true     | none         | number of decimal digits 'after the dot' for quantity                                                                                                                                                    |
| costPrecision                 | integer                                         | true     | none         | number of decimal digits 'after the dot' for cost, `price * quantity`                                                                                                                                    |
| priceBuffer                   | string                                          | true     | none         | buffer range of limit price from the last traded price.                                                                                                                                                  |
| minQuantityLimit              | [AssetValue](#schemaassetvalue)                 | true     | none         | order quantity should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                                        |
| maxQuantityLimit              | [AssetValue](#schemaassetvalue)                 | true     | none         | order quantity should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                                        |
| maxPriceLimit                 | [AssetValue](#schemaassetvalue)                 | true     | none         | order price should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                                           |
| minPriceLimit                 | [AssetValue](#schemaassetvalue)                 | true     | none         | order price should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                                           |
| maxCostLimit                  | [AssetValue](#schemaassetvalue)                 | true     | none         | order cost, `price * quantity` should be < max, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| minCostLimit                  | [AssetValue](#schemaassetvalue)                 | true     | none         | order cost, `price * quantity` should be > min, see [asset value](#overview--price-and-quantity-precision) format                                                                                        |
| timeZone                      | string                                          | true     | none         | time zone                                                                                                                                                                                                |
| tickSize                      | [AssetValue](#schemaassetvalue)                 | true     | none         | tick size, see [asset value](#overview--price-and-quantity-precision) format                                                                                                                             |
| liquidityTickSize             | string                                          | true     | none         | liquidity tick size.                                                                                                                                                                                     |
| liquidityPrecision            | integer                                         | true     | none         | liquidity precision.                                                                                                                                                                                     |
| makerFee                      | integer                                         | false    | none         | Deprecated and no longer accurate. See `feeGroupId`                                                                                                                                                      |
| takerFee                      | integer                                         | false    | none         | Deprecated and no longer accurate. See `feeGroupId`                                                                                                                                                      |
| roundingCorrectionFactor      | string                                          | true     | none         | rounding correction factor for market                                                                                                                                                                    |
| makerMinLiquidityAddition     | string                                          | true     | none         | minimum amount required to invest liquidity to market.                                                                                                                                                   |
| orderTypes                    | [allOf]                                         | false    | none         | none                                                                                                                                                                                                     |
| spotTradingEnabled            | boolean                                         | true     | none         | spot trading enabled (only applies for Spot markets)                                                                                                                                                     |
| marginTradingEnabled          | boolean                                         | true     | none         | margin trading enabled (only applies for Spot markets)                                                                                                                                                   |
| marketEnabled                 | boolean                                         | true     | none         | market enabled                                                                                                                                                                                           |
| createOrderEnabled            | boolean                                         | true     | none         | able to create order                                                                                                                                                                                     |
| cancelOrderEnabled            | boolean                                         | true     | none         | able to cancel order                                                                                                                                                                                     |
| liquidityInvestEnabled        | boolean                                         | true     | none         | able to invest liquidity to market.                                                                                                                                                                      |
| liquidityWithdrawEnabled      | boolean                                         | true     | none         | able to withdraw liquidity from market.                                                                                                                                                                  |
| feeGroupId                    | integer                                         | true     | none         | Identifier to the trade fee assigned to this market. Used with `tradeFeeRate` at [Get Trading Account](#get-/v1/accounts/trading-accounts/-tradingAccountId-)                                            |
| feeTiers                      | [allOf]                                         | true     | none         | all available fee tiers.                                                                                                                                                                                 |
| marketType                    | [MarketTypeAsString](#schemamarkettypeasstring) | true     | none         | market type, e.g. "SPOT" for market like "BTCUSD", "PERPETUAL" for market like "BTC-USDC-PERP", "DATED_FUTURE" for market like "BTC-USDC-20250901", "OPTION" for market like "BTC-USDC-20250901-90000-C" |
| contractMultiplier            | integer                                         | false    | none         | contract multiplier. (only applies to perpetual market)                                                                                                                                                  |
| settlementAssetSymbol         | string                                          | false    | none         | settlement asset symbol. (only applies to perpetual market)                                                                                                                                              |
| openInterestUSD               | string                                          | true     | none         | cumulative notional value of all open interest for a specific derivative contract on the exchange.                                                                                                       |
| concentrationRiskThresholdUSD | string                                          | true     | none         | open interest notional of an account for a specific derivative contract.                                                                                                                                 |
| concentrationRiskPercentage   | string                                          | true     | none         | percentage of the total open interest for a specific derivative contract.                                                                                                                                |
| expiryDatetime                | string                                          | true     | none         | denotes the time when the market expires in ISO 8601 with millisecond format as string                                                                                                                   |

<h2 id="tocS_Asset">Asset</h2>
<!-- backwards compatibility -->
<a id="schemaasset"></a>
<a id="schema_Asset"></a>
<a id="tocSasset"></a>
<a id="tocsasset"></a>

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

### Properties

| Name                     | Type                                      | Required | Restrictions | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ------------------------ | ----------------------------------------- | -------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| assetId                  | [AssetID](#schemaassetid)                 | true     | none         | unique asset ID                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| symbol                   | [AssetSymbol](#schemaassetsymbol)         | true     | none         | asset symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| name                     | [AssetName](#schemaassetname)             | true     | none         | asset name                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| precision                | string                                    | true     | none         | number of decimal digits 'after the dot' for asset amount                                                                                                                                                                                                                                                                                                                                                                                                      |
| minBalanceInterest       | [AssetValue](#schemaassetvalue)           | true     | none         | see [asset value](#overview--price-and-quantity-precision) format                                                                                                                                                                                                                                                                                                                                                                                              |
| minFee                   | [AssetValue](#schemaassetvalue)           | true     | none         | minimum fee                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| apr                      | String                                    | true     | none         | annualized percentage rate                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| collateralRating         | String                                    | true     | none         | collateral rating applied to this asset, a value of 100.00 indicates 100%. `Deprecated in favour of collateral bands`                                                                                                                                                                                                                                                                                                                                          |
| maxBorrow                | String                                    | true     | none         | maximum quantity that can be borrowed for this asset                                                                                                                                                                                                                                                                                                                                                                                                           |
| totalOfferedLoanQuantity | String                                    | true     | none         | quantity of an asset that is across all loan offers on the exchange                                                                                                                                                                                                                                                                                                                                                                                            |
| loanBorrowedQuantity     | String                                    | true     | none         | amount of loans that is currently being borrowed for the asset                                                                                                                                                                                                                                                                                                                                                                                                 |
| collateralBands          | [allOf]                                   | true     | none         | list of collateral bands for the asset. A collateral band holds the upper limit of the USD notional and the corresponding collateral percentage which applies to it. An asset's collateral value will be capped by the highest limit of the collateral bands, any remaining amount greater than this limit will have a collateral percentage of 0. If an asset has an empty list of CollateralBands, this signifies that the asset has a collateralValue of 0. |
| underlyingAsset          | [UnderlyingAsset](#schemaunderlyingasset) | true     | none         | underlying asset for the asset.                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h2 id="tocS_OHLCVCandle">OHLCVCandle</h2>
<!-- backwards compatibility -->
<a id="schemaohlcvcandle"></a>
<a id="schema_OHLCVCandle"></a>
<a id="tocSohlcvcandle"></a>
<a id="tocsohlcvcandle"></a>

```json
{
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
```

### Properties

| Name                 | Type                                          | Required | Restrictions | Description                                                                                       |
| -------------------- | --------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| open                 | [AssetValue](#schemaassetvalue)               | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| high                 | [AssetValue](#schemaassetvalue)               | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| low                  | [AssetValue](#schemaassetvalue)               | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| close                | [AssetValue](#schemaassetvalue)               | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| volume               | [AssetValue](#schemaassetvalue)               | false    | none         | see [asset value](#overview--price-and-quantity-precision) format                                 |
| createdAtTimestamp   | [TimeStampAsString](#schematimestampasstring) | false    | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| createdAtDatetime    | [DateTime](#schemadatetime)                   | false    | none         | date and time of the candle, ISO 8601 with millisecond as string                                  |
| publishedAtTimestamp | [TimeStampAsString](#schematimestampasstring) | false    | none         | date and time of the candle getting published, ISO 8601 with millisecond as string                |

<h2 id="tocS_BorrowInterest">BorrowInterest</h2>
<!-- backwards compatibility -->
<a id="schemaborrowinterest"></a>
<a id="schema_BorrowInterest"></a>
<a id="tocSborrowinterest"></a>
<a id="tocsborrowinterest"></a>

```json
{
  "type": "object",
  "required": [
    "assetId",
    "assetSymbol",
    "borrowedQuantity",
    "totalBorrowedQuantity",
    "createdAtDatetime",
    "createdAtTimestamp"
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
    "assetSymbol": {
      "description": "asset symbol",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "borrowedQuantity": {
      "description": "the principal borrowed quantity",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "totalBorrowedQuantity": {
      "description": "the sum of the principal borrowed quantity and the interest charged",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "createdAtDatetime": {
      "description": "denotes the hour in which the principal quantity was borrowed or when the interest was charged, ISO 8601 with millisecond as string",
      "type": "string",
      "format": "date-time",
      "example": "2020-08-21T08:00:00.000Z"
    },
    "createdAtTimestamp": {
      "description": "denotes the hour in which the principal quantity was borrowed or when the interest was charged",
      "type": "string",
      "format": "string",
      "example": "1621490985000"
    }
  }
}
```

### Properties

| Name                  | Type                              | Required | Restrictions | Description                                                                                                                         |
| --------------------- | --------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------- |
| assetId               | [AssetID](#schemaassetid)         | true     | none         | unique asset ID                                                                                                                     |
| assetSymbol           | [AssetSymbol](#schemaassetsymbol) | true     | none         | asset symbol                                                                                                                        |
| borrowedQuantity      | [AssetValue](#schemaassetvalue)   | true     | none         | the principal borrowed quantity                                                                                                     |
| totalBorrowedQuantity | [AssetValue](#schemaassetvalue)   | true     | none         | the sum of the principal borrowed quantity and the interest charged                                                                 |
| createdAtDatetime     | string(date-time)                 | true     | none         | denotes the hour in which the principal quantity was borrowed or when the interest was charged, ISO 8601 with millisecond as string |
| createdAtTimestamp    | string(string)                    | true     | none         | denotes the hour in which the principal quantity was borrowed or when the interest was charged                                      |

<h2 id="tocS_CurrentExchangeTimeResponse">CurrentExchangeTimeResponse</h2>
<!-- backwards compatibility -->
<a id="schemacurrentexchangetimeresponse"></a>
<a id="schema_CurrentExchangeTimeResponse"></a>
<a id="tocScurrentexchangetimeresponse"></a>
<a id="tocscurrentexchangetimeresponse"></a>

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

### Properties

| Name      | Type                                          | Required | Restrictions | Description                                                                                       |
| --------- | --------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| timestamp | [TimeStampAsString](#schematimestampasstring) | true     | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| datetime  | [DateTime](#schemadatetime)                   | true     | none         | ISO 8601 with millisecond as string                                                               |

<h2 id="tocS_Nonce">Nonce</h2>
<!-- backwards compatibility -->
<a id="schemanonce"></a>
<a id="schema_Nonce"></a>
<a id="tocSnonce"></a>
<a id="tocsnonce"></a>

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

### Properties

| Name       | Type    | Required | Restrictions | Description                |
| ---------- | ------- | -------- | ------------ | -------------------------- |
| lowerBound | integer | true     | none         | lower bound of nonce range |
| upperBound | integer | true     | none         | upper bound of nonce range |

<h2 id="tocS_OrderTimeInForce">OrderTimeInForce</h2>
<!-- backwards compatibility -->
<a id="schemaordertimeinforce"></a>
<a id="schema_OrderTimeInForce"></a>
<a id="tocSordertimeinforce"></a>
<a id="tocsordertimeinforce"></a>

```json
{
  "description": "time in force",
  "type": "string",
  "enum": ["GTC", "FOK", "IOC"]
}
```

time in force

### Properties

| Name        | Type   | Required | Restrictions | Description   |
| ----------- | ------ | -------- | ------------ | ------------- |
| _anonymous_ | string | false    | none         | time in force |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| _anonymous_ | GTC   |
| _anonymous_ | FOK   |
| _anonymous_ | IOC   |

<h2 id="tocS_OrderType">OrderType</h2>
<!-- backwards compatibility -->
<a id="schemaordertype"></a>
<a id="schema_OrderType"></a>
<a id="tocSordertype"></a>
<a id="tocsordertype"></a>

```json
{
  "type": "string",
  "description": "Order Types supported for the market.",
  "enum": ["LMT", "MKT", "STOP_LIMIT"]
}
```

Order Types supported for the market.

### Properties

| Name        | Type   | Required | Restrictions | Description                           |
| ----------- | ------ | -------- | ------------ | ------------------------------------- |
| _anonymous_ | string | false    | none         | Order Types supported for the market. |

#### Enumerated Values

| Property    | Value      |
| ----------- | ---------- |
| _anonymous_ | LMT        |
| _anonymous_ | MKT        |
| _anonymous_ | STOP_LIMIT |

<h2 id="tocS_OrderSide">OrderSide</h2>
<!-- backwards compatibility -->
<a id="schemaorderside"></a>
<a id="schema_OrderSide"></a>
<a id="tocSorderside"></a>
<a id="tocsorderside"></a>

```json
"BUY"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| _anonymous_ | BUY   |
| _anonymous_ | SELL  |

<h2 id="tocS_TimeBucket">TimeBucket</h2>
<!-- backwards compatibility -->
<a id="schematimebucket"></a>
<a id="schema_TimeBucket"></a>
<a id="tocStimebucket"></a>
<a id="tocstimebucket"></a>

```json
"1m"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| _anonymous_ | 1m    |
| _anonymous_ | 5m    |
| _anonymous_ | 30m   |
| _anonymous_ | 1h    |
| _anonymous_ | 6h    |
| _anonymous_ | 12h   |
| _anonymous_ | 1d    |

<h2 id="tocS_OrderStatus">OrderStatus</h2>
<!-- backwards compatibility -->
<a id="schemaorderstatus"></a>
<a id="schema_OrderStatus"></a>
<a id="tocSorderstatus"></a>
<a id="tocsorderstatus"></a>

```json
"OPEN"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value     |
| ----------- | --------- |
| _anonymous_ | OPEN      |
| _anonymous_ | CLOSED    |
| _anonymous_ | CANCELLED |
| _anonymous_ | REJECTED  |

<h2 id="tocS_AMMInstructionStatus">AMMInstructionStatus</h2>
<!-- backwards compatibility -->
<a id="schemaamminstructionstatus"></a>
<a id="schema_AMMInstructionStatus"></a>
<a id="tocSamminstructionstatus"></a>
<a id="tocsamminstructionstatus"></a>

```json
"OPEN"
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| _anonymous_ | OPEN   |
| _anonymous_ | CLOSED |

<h2 id="tocS_LiquidationRisk">LiquidationRisk</h2>
<!-- backwards compatibility -->
<a id="schemaliquidationrisk"></a>
<a id="schema_LiquidationRisk"></a>
<a id="tocSliquidationrisk"></a>
<a id="tocsliquidationrisk"></a>

```json
{
  "type": "string",
  "enum": ["LOW", "MEDIUM", "HIGH"]
}
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value  |
| ----------- | ------ |
| _anonymous_ | LOW    |
| _anonymous_ | MEDIUM |
| _anonymous_ | HIGH   |

<h2 id="tocS_PositionType">PositionType</h2>
<!-- backwards compatibility -->
<a id="schemapositiontype"></a>
<a id="schema_PositionType"></a>
<a id="tocSpositiontype"></a>
<a id="tocspositiontype"></a>

```json
{
  "type": "string",
  "enum": ["LONG", "SHORT"]
}
```

### Properties

| Name        | Type   | Required | Restrictions | Description |
| ----------- | ------ | -------- | ------------ | ----------- |
| _anonymous_ | string | false    | none         | none        |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| _anonymous_ | LONG  |
| _anonymous_ | SHORT |

<h2 id="tocS_DelayBySeconds">DelayBySeconds</h2>
<!-- backwards compatibility -->
<a id="schemadelaybyseconds"></a>
<a id="schema_DelayBySeconds"></a>
<a id="tocSdelaybyseconds"></a>
<a id="tocsdelaybyseconds"></a>

```json
"5"
```

Delay the cancel-all-orders request by (seconds) as a timeout mechanism

### Properties

| Name        | Type   | Required | Restrictions | Description                                                             |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | Delay the cancel-all-orders request by (seconds) as a timeout mechanism |

#### Enumerated Values

| Property    | Value |
| ----------- | ----- |
| _anonymous_ | 5     |
| _anonymous_ | 10    |
| _anonymous_ | 15    |
| _anonymous_ | 20    |
| _anonymous_ | 25    |
| _anonymous_ | 30    |
| _anonymous_ | 40    |
| _anonymous_ | 50    |
| _anonymous_ | 60    |

<h2 id="tocS_CancelId">CancelId</h2>
<!-- backwards compatibility -->
<a id="schemacancelid"></a>
<a id="schema_CancelId"></a>
<a id="tocScancelid"></a>
<a id="tocscancelid"></a>

```json
"123456789"
```

Unique id for this cancel request which is an unsigned 64 bit integer expressed
as string

### Properties

| Name        | Type   | Required | Restrictions | Description                                                                               |
| ----------- | ------ | -------- | ------------ | ----------------------------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | Unique id for this cancel request which is an unsigned 64 bit integer expressed as string |

<h2 id="tocS_DerivativesPositionResponse">DerivativesPositionResponse</h2>
<!-- backwards compatibility -->
<a id="schemaderivativespositionresponse"></a>
<a id="schema_DerivativesPositionResponse"></a>
<a id="tocSderivativespositionresponse"></a>
<a id="tocsderivativespositionresponse"></a>

```json
{
  "description": "Derivatives Position of one market for the trading account",
  "type": "string",
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "symbol": {
      "example": "BTC-USDC-PERP",
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "side": {
      "allOf": [
        {
          "type": "string",
          "example": "BUY",
          "enum": ["BUY", "SELL"]
        }
      ]
    },
    "quantity": {
      "description": "Current size of the  position [asset value](#overview--price-and-quantity-precision) format",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "notional": {
      "description": "Notional value of the current position, calculated using the mark price",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "entryNotional": {
      "description": "Notional value of the position, using the average entry price",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "mtmPnl": {
      "description": "Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "reportedMtmPnl": {
      "description": "The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "reportedFundingPnl": {
      "description": "Sum of all funding payments received  since the position was opened. This is updated every time funding is paid.",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "realizedPnl": {
      "description": "Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "settlementAssetSymbol": {
      "description": "Settlement Asset Symbol",
      "type": "string",
      "example": "USDC"
    },
    "createdAtDatetime": {
      "description": "Denotes the time the position was created by the exchange, ISO 8601 with millisecond as string",
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
      "description": "Denotes the time the position was created by the exchange, number of milliseconds since EPOCH",
      "allOf": [
        {
          "type": "string",
          "format": "string",
          "example": "1621490985000",
          "description": "unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string"
        }
      ]
    },
    "updatedAtDatetime": {
      "description": "Denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string",
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
      "description": "Denotes the time the position was updated by the exchange, number of milliseconds since EPOCH",
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
```

Derivatives Position of one market for the trading account

### Properties

| Name                  | Type                                          | Required | Restrictions | Description                                                                                                                                                                                               |
| --------------------- | --------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| _anonymous_           | string                                        | false    | none         | Derivatives Position of one market for the trading account                                                                                                                                                |
| tradingAccountId      | [TradingAccountId](#schematradingaccountid)   | false    | none         | unique trading account ID                                                                                                                                                                                 |
| symbol                | [MarketSymbol](#schemamarketsymbol)           | false    | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market                                                                                                                             |
| side                  | [OrderSide](#schemaorderside)                 | false    | none         | none                                                                                                                                                                                                      |
| quantity              | [AssetValue](#schemaassetvalue)               | false    | none         | Current size of the position [asset value](#overview--price-and-quantity-precision) format                                                                                                                |
| notional              | [UsdcValue](#schemausdcvalue)                 | false    | none         | Notional value of the current position, calculated using the mark price                                                                                                                                   |
| entryNotional         | [UsdcValue](#schemausdcvalue)                 | false    | none         | Notional value of the position, using the average entry price                                                                                                                                             |
| mtmPnl                | [UsdcValue](#schemausdcvalue)                 | false    | none         | Sum of all mark-to-market profits and losses plus profits and losses realised from trading, accumulated since the last settlement                                                                         |
| reportedMtmPnl        | [UsdcValue](#schemausdcvalue)                 | false    | none         | The profit/losses from the net price change since the last time the absolute quantity decreased. It is updated with every mark to market and is not updated during settlement or a position size increase |
| reportedFundingPnl    | [UsdcValue](#schemausdcvalue)                 | false    | none         | Sum of all funding payments received since the position was opened. This is updated every time funding is paid.                                                                                           |
| realizedPnl           | [UsdcValue](#schemausdcvalue)                 | false    | none         | Total profits realized since the trading account first opened this position. This is only updated every time a position’s absolute quantity (aka size) is reduced.                                        |
| settlementAssetSymbol | string                                        | false    | none         | Settlement Asset Symbol                                                                                                                                                                                   |
| createdAtDatetime     | [DateTime](#schemadatetime)                   | false    | none         | Denotes the time the position was created by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| createdAtTimestamp    | [TimeStampAsString](#schematimestampasstring) | false    | none         | Denotes the time the position was created by the exchange, number of milliseconds since EPOCH                                                                                                             |
| updatedAtDatetime     | [DateTime](#schemadatetime)                   | false    | none         | Denotes the time the position was updated by the exchange, ISO 8601 with millisecond as string                                                                                                            |
| updatedAtTimestamp    | [TimeStampAsString](#schematimestampasstring) | false    | none         | Denotes the time the position was updated by the exchange, number of milliseconds since EPOCH                                                                                                             |

<h2 id="tocS_DerivativesSettlementResponse">DerivativesSettlementResponse</h2>
<!-- backwards compatibility -->
<a id="schemaderivativessettlementresponse"></a>
<a id="schema_DerivativesSettlementResponse"></a>
<a id="tocSderivativessettlementresponse"></a>
<a id="tocsderivativessettlementresponse"></a>

```json
{
  "description": "Derivatives Settlement of one market for the trading account",
  "type": "string",
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "symbol": {
      "example": "BTC-USDC-PERP",
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for DATED FUTURE markets.",
          "example": "BTC-USDC-20241201"
        }
      ]
    },
    "side": {
      "allOf": [
        {
          "type": "string",
          "example": "BUY",
          "enum": ["BUY", "SELL"]
        }
      ]
    },
    "settlementQuantity": {
      "description": "position size at the time of the settlement",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "deltaTradingQuantity": {
      "description": "the change in the position size from the account’s trading activities",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "mtmPnl": {
      "description": "mark to market profit (losses) accumulated since the last settlement",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "fundingPnl": {
      "description": "funding profits (losses) accumulated since the last settlement. Applicable for perpetuals only.",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "eventType": {
      "description": "derivatives position update event types",
      "type": "string",
      "example": "settlementUpdate"
    },
    "settlementMarkPrice": {
      "description": "market price at which the position was settled for this past cycle",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "settlementIndexPrice": {
      "description": "index price at which the position was settled for this past cycle",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.0000"
        }
      ]
    },
    "settlementFundingRate": {
      "description": "funding rate at which the position was settled for this past cycle. Applicable for perpetuals only.",
      "type": "string",
      "example": "10.0"
    },
    "settlementDatetime": {
      "description": "Denotes the time the position was settled by the exchange, ISO 8601 with millisecond as string",
      "allOf": [
        {
          "type": "string",
          "format": "date-time",
          "example": "2025-05-20T01:01:01.000Z",
          "description": "ISO 8601 with millisecond as string"
        }
      ]
    },
    "settlementTimestamp": {
      "description": "Denotes the time the position was settled by the exchange, number of milliseconds since EPOCH",
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
```

Derivatives Settlement of one market for the trading account

### Properties

| Name                  | Type                                                      | Required | Restrictions | Description                                                                                         |
| --------------------- | --------------------------------------------------------- | -------- | ------------ | --------------------------------------------------------------------------------------------------- |
| _anonymous_           | string                                                    | false    | none         | Derivatives Settlement of one market for the trading account                                        |
| tradingAccountId      | [TradingAccountId](#schematradingaccountid)               | false    | none         | unique trading account ID                                                                           |
| symbol                | [DatedFutureMarketSymbol](#schemadatedfuturemarketsymbol) | false    | none         | market symbol. Eg `BTC-USDC-PERP` for PERPETUAL and `BTC-USDC-20241201` for DATED FUTURE markets.   |
| side                  | [OrderSide](#schemaorderside)                             | false    | none         | none                                                                                                |
| settlementQuantity    | [AssetValue](#schemaassetvalue)                           | false    | none         | position size at the time of the settlement                                                         |
| deltaTradingQuantity  | [AssetValue](#schemaassetvalue)                           | false    | none         | the change in the position size from the account’s trading activities                               |
| mtmPnl                | [UsdcValue](#schemausdcvalue)                             | false    | none         | mark to market profit (losses) accumulated since the last settlement                                |
| fundingPnl            | [UsdcValue](#schemausdcvalue)                             | false    | none         | funding profits (losses) accumulated since the last settlement. Applicable for perpetuals only.     |
| eventType             | string                                                    | false    | none         | derivatives position update event types                                                             |
| settlementMarkPrice   | [UsdcValue](#schemausdcvalue)                             | false    | none         | market price at which the position was settled for this past cycle                                  |
| settlementIndexPrice  | [UsdcValue](#schemausdcvalue)                             | false    | none         | index price at which the position was settled for this past cycle                                   |
| settlementFundingRate | string                                                    | false    | none         | funding rate at which the position was settled for this past cycle. Applicable for perpetuals only. |
| settlementDatetime    | [DateTime](#schemadatetime)                               | false    | none         | Denotes the time the position was settled by the exchange, ISO 8601 with millisecond as string      |
| settlementTimestamp   | [TimeStampAsString](#schematimestampasstring)             | false    | none         | Denotes the time the position was settled by the exchange, number of milliseconds since EPOCH       |

<h2 id="tocS_SubAccountTransferResponse">SubAccountTransferResponse</h2>
<!-- backwards compatibility -->
<a id="schemasubaccounttransferresponse"></a>
<a id="schema_SubAccountTransferResponse"></a>
<a id="tocSsubaccounttransferresponse"></a>
<a id="tocssubaccounttransferresponse"></a>

```json
{
  "description": "Get account transfer history",
  "type": "string",
  "properties": {
    "requestId": {
      "description": "unique identifier of the transfer request",
      "type": "string",
      "example": "1"
    },
    "toTradingAccountId": {
      "description": "recipient's trading account",
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "fromTradingAccountId": {
      "description": "sender's trading account",
      "type": "string",
      "example": "121000000000001"
    },
    "assetSymbol": {
      "description": "asset currency of the transfer",
      "allOf": [
        {
          "type": "string",
          "description": "asset symbol as denoted in the world",
          "example": "BTC"
        }
      ]
    },
    "quantity": {
      "description": "transfer quantity",
      "allOf": [
        {
          "description": "see [asset value](#overview--price-and-quantity-precision) format",
          "type": "string",
          "example": "1.00000000"
        }
      ]
    },
    "status": {
      "description": "transfer status [CLOSED/OPEN/REJECTED]",
      "type": "string",
      "example": "CLOSED"
    },
    "statusReasonCode": {
      "description": "status reason code",
      "type": "string",
      "example": "6002"
    },
    "statusReason": {
      "description": "readable status reason",
      "type": "string",
      "example": "Executed"
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

Get account transfer history

### Properties

| Name                 | Type                                          | Required | Restrictions | Description                                                                                       |
| -------------------- | --------------------------------------------- | -------- | ------------ | ------------------------------------------------------------------------------------------------- |
| _anonymous_          | string                                        | false    | none         | Get account transfer history                                                                      |
| requestId            | string                                        | false    | none         | unique identifier of the transfer request                                                         |
| toTradingAccountId   | [TradingAccountId](#schematradingaccountid)   | false    | none         | recipient's trading account                                                                       |
| fromTradingAccountId | string                                        | false    | none         | sender's trading account                                                                          |
| assetSymbol          | [AssetSymbol](#schemaassetsymbol)             | false    | none         | asset currency of the transfer                                                                    |
| quantity             | [AssetValue](#schemaassetvalue)               | false    | none         | transfer quantity                                                                                 |
| status               | string                                        | false    | none         | transfer status [CLOSED/OPEN/REJECTED]                                                            |
| statusReasonCode     | string                                        | false    | none         | status reason code                                                                                |
| statusReason         | string                                        | false    | none         | readable status reason                                                                            |
| createdAtTimestamp   | [TimeStampAsString](#schematimestampasstring) | false    | none         | unsigned 64 bit integer value which is the number of milliseconds since EPOCH expressed as string |
| createdAtDatetime    | [DateTime](#schemadatetime)                   | false    | none         | ISO 8601 with millisecond as string                                                               |

<h2 id="tocS_SimulationPosition">SimulationPosition</h2>
<!-- backwards compatibility -->
<a id="schemasimulationposition"></a>
<a id="schema_SimulationPosition"></a>
<a id="tocSsimulationposition"></a>
<a id="tocssimulationposition"></a>

```json
{
  "type": "string",
  "required": ["symbol", "quantity"],
  "properties": {
    "symbol": {
      "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
      "type": "string",
      "example": "BTC-USDC-PERP"
    },
    "quantity": {
      "description": "size of the position",
      "type": "string",
      "example": "1.0"
    }
  }
}
```

### Properties

| Name        | Type   | Required | Restrictions | Description                                                          |
| ----------- | ------ | -------- | ------------ | -------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | none                                                                 |
| symbol      | string | true     | none         | asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market |
| quantity    | string | true     | none         | size of the position                                                 |

<h2 id="tocS_SimulationOrder">SimulationOrder</h2>
<!-- backwards compatibility -->
<a id="schemasimulationorder"></a>
<a id="schema_SimulationOrder"></a>
<a id="tocSsimulationorder"></a>
<a id="tocssimulationorder"></a>

```json
{
  "type": "string",
  "required": ["symbol", "quantity"],
  "properties": {
    "symbol": {
      "allOf": [
        {
          "type": "string",
          "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
          "example": "BTCUSDC"
        }
      ]
    },
    "quantity": {
      "description": "quantity placed for order",
      "type": "string",
      "example": "1.0"
    },
    "limitPrice": {
      "description": "limit price for order",
      "type": "string",
      "example": "10000.0"
    }
  }
}
```

### Properties

| Name        | Type                                | Required | Restrictions | Description                                                                   |
| ----------- | ----------------------------------- | -------- | ------------ | ----------------------------------------------------------------------------- |
| _anonymous_ | string                              | false    | none         | none                                                                          |
| symbol      | [MarketSymbol](#schemamarketsymbol) | true     | none         | market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market |
| quantity    | string                              | true     | none         | quantity placed for order                                                     |
| limitPrice  | string                              | false    | none         | limit price for order                                                         |

<h2 id="tocS_ReferencePrice">ReferencePrice</h2>
<!-- backwards compatibility -->
<a id="schemareferenceprice"></a>
<a id="schema_ReferencePrice"></a>
<a id="tocSreferenceprice"></a>
<a id="tocsreferenceprice"></a>

```json
{
  "type": "string",
  "properties": {
    "symbol": {
      "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
      "type": "string",
      "example": "BTC"
    },
    "price": {
      "description": "reference price for asset or market",
      "type": "string",
      "example": "12000.0"
    }
  }
}
```

### Properties

| Name        | Type   | Required | Restrictions | Description                                                          |
| ----------- | ------ | -------- | ------------ | -------------------------------------------------------------------- |
| _anonymous_ | string | false    | none         | none                                                                 |
| symbol      | string | false    | none         | asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market |
| price       | string | false    | none         | reference price for asset or market                                  |

<h2 id="tocS_TradeFeeRate">TradeFeeRate</h2>
<!-- backwards compatibility -->
<a id="schematradefeerate"></a>
<a id="schema_TradeFeeRate"></a>
<a id="tocStradefeerate"></a>
<a id="tocstradefeerate"></a>

```json
{
  "type": "object",
  "required": ["feeGroupId", "makerFee", "takerFee"],
  "properties": {
    "feeGroupId": {
      "type": "integer",
      "description": "Identifier for this particular fee tier",
      "example": 1
    },
    "makerFee": {
      "type": "string",
      "description": "Maker Fee in basis points (bps)",
      "example": "10"
    },
    "takerFee": {
      "type": "string",
      "description": "Taker Fee in basis points (bps)",
      "example": "10"
    }
  }
}
```

### Properties

| Name       | Type    | Required | Restrictions | Description                             |
| ---------- | ------- | -------- | ------------ | --------------------------------------- |
| feeGroupId | integer | true     | none         | Identifier for this particular fee tier |
| makerFee   | string  | true     | none         | Maker Fee in basis points (bps)         |
| takerFee   | string  | true     | none         | Taker Fee in basis points (bps)         |

<h2 id="tocS_PortfolioSimulationRequest">PortfolioSimulationRequest</h2>
<!-- backwards compatibility -->
<a id="schemaportfoliosimulationrequest"></a>
<a id="schema_PortfolioSimulationRequest"></a>
<a id="tocSportfoliosimulationrequest"></a>
<a id="tocsportfoliosimulationrequest"></a>

```json
{
  "type": "object",
  "required": ["tradingAccountId"],
  "properties": {
    "tradingAccountId": {
      "allOf": [
        {
          "description": "unique trading account ID",
          "type": "string",
          "example": "111000000000001"
        }
      ]
    },
    "positions": {
      "type": "array",
      "description": "portfolio position to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": ["symbol", "quantity"],
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC-USDC-PERP"
              },
              "quantity": {
                "description": "size of the position",
                "type": "string",
                "example": "1.0"
              }
            }
          }
        ]
      }
    },
    "orders": {
      "type": "array",
      "description": "pending orders to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "required": ["symbol", "quantity"],
            "properties": {
              "symbol": {
                "allOf": [
                  {
                    "type": "string",
                    "description": "market symbol. Eg `BTCUSDC` for SPOT and `BTC-USDC-PERP` for PERPETUAL market",
                    "example": "BTCUSDC"
                  }
                ]
              },
              "quantity": {
                "description": "quantity placed for order",
                "type": "string",
                "example": "1.0"
              },
              "limitPrice": {
                "description": "limit price for order",
                "type": "string",
                "example": "10000.0"
              }
            }
          }
        ]
      }
    },
    "referencePrices": {
      "type": "array",
      "description": "reference price to be used in simulation",
      "items": {
        "allOf": [
          {
            "type": "string",
            "properties": {
              "symbol": {
                "description": "asset or market symbol. Eg `USDC` for asset and `BTCUSDC` for market",
                "type": "string",
                "example": "BTC"
              },
              "price": {
                "description": "reference price for asset or market",
                "type": "string",
                "example": "12000.0"
              }
            }
          }
        ]
      }
    }
  }
}
```

### Properties

| Name             | Type                                        | Required | Restrictions | Description                                 |
| ---------------- | ------------------------------------------- | -------- | ------------ | ------------------------------------------- |
| tradingAccountId | [TradingAccountId](#schematradingaccountid) | true     | none         | unique trading account ID                   |
| positions        | [allOf]                                     | false    | none         | portfolio position to be used in simulation |
| orders           | [allOf]                                     | false    | none         | pending orders to be used in simulation     |
| referencePrices  | [allOf]                                     | false    | none         | reference price to be used in simulation    |

<h2 id="tocS_PortfolioSimulationResponse">PortfolioSimulationResponse</h2>
<!-- backwards compatibility -->
<a id="schemaportfoliosimulationresponse"></a>
<a id="schema_PortfolioSimulationResponse"></a>
<a id="tocSportfoliosimulationresponse"></a>
<a id="tocsportfoliosimulationresponse"></a>

```json
{
  "description": "Simulation result",
  "type": "string",
  "properties": {
    "collateralUSD": {
      "description": "total collateral across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "13000.0000"
    },
    "borrowedUSD": {
      "description": "total borrowed across all assets in this trading account displayed in the reference asset in USD",
      "type": "string",
      "example": "12000.0000"
    },
    "initialMarginUSD": {
      "description": "The minimum margin one must maintain in order to be able to purposefully increase risk",
      "type": "string",
      "example": "14000.0000"
    },
    "warningMarginUSD": {
      "description": "The minimum margin when the customer will receive warning via email/notifications over UI",
      "type": "string",
      "example": "15000.0000"
    },
    "liquidationMarginUSD": {
      "description": "The minimum value of margin one must maintain in order to avoid liquidation",
      "type": "string",
      "example": "16000.0000"
    },
    "fullLiquidationMarginUSD": {
      "description": "The value of margin when full liquidation occurs",
      "type": "string",
      "example": "17000.0000"
    },
    "defaultedMarginUSD": {
      "description": "The value of margin when this trading account will be moved into a Defaulted state",
      "type": "string",
      "example": "18000.0000"
    },
    "liquidityAddonUSD": {
      "description": "expected market impact of unwinding the portfolio in the case of a liquidation event",
      "type": "string",
      "example": "19000.0000"
    },
    "marketRiskUSD": {
      "description": "the worst possible loss on the portfolio based on scenario analysis",
      "type": "string",
      "example": "20000.0000"
    }
  }
}
```

Simulation result

### Properties

| Name                     | Type   | Required | Restrictions | Description                                                                                        |
| ------------------------ | ------ | -------- | ------------ | -------------------------------------------------------------------------------------------------- |
| _anonymous_              | string | false    | none         | Simulation result                                                                                  |
| collateralUSD            | string | false    | none         | total collateral across all assets in this trading account displayed in the reference asset in USD |
| borrowedUSD              | string | false    | none         | total borrowed across all assets in this trading account displayed in the reference asset in USD   |
| initialMarginUSD         | string | false    | none         | The minimum margin one must maintain in order to be able to purposefully increase risk             |
| warningMarginUSD         | string | false    | none         | The minimum margin when the customer will receive warning via email/notifications over UI          |
| liquidationMarginUSD     | string | false    | none         | The minimum value of margin one must maintain in order to avoid liquidation                        |
| fullLiquidationMarginUSD | string | false    | none         | The value of margin when full liquidation occurs                                                   |
| defaultedMarginUSD       | string | false    | none         | The value of margin when this trading account will be moved into a Defaulted state                 |
| liquidityAddonUSD        | string | false    | none         | expected market impact of unwinding the portfolio in the case of a liquidation event               |
| marketRiskUSD            | string | false    | none         | the worst possible loss on the portfolio based on scenario analysis                                |

<h2 id="tocS_FundingRateHistoryResponse">FundingRateHistoryResponse</h2>
<!-- backwards compatibility -->
<a id="schemafundingratehistoryresponse"></a>
<a id="schema_FundingRateHistoryResponse"></a>
<a id="tocSfundingratehistoryresponse"></a>
<a id="tocsfundingratehistoryresponse"></a>

```json
{
  "description": "Hourly Funding Rate History of one market",
  "type": "array",
  "properties": {
    "fundingRate": {
      "description": "funding rate for this hour",
      "type": "string",
      "example": "0.1"
    },
    "updatedAtDatetime": {
      "description": "date time of the last funding rate update for the hour",
      "type": "string",
      "example": "2024-09-16T12:59:59.000Z"
    }
  }
}
```

Hourly Funding Rate History of one market

### Properties

| Name              | Type   | Required | Restrictions | Description                                            |
| ----------------- | ------ | -------- | ------------ | ------------------------------------------------------ |
| fundingRate       | string | false    | none         | funding rate for this hour                             |
| updatedAtDatetime | string | false    | none         | date time of the last funding rate update for the hour |
