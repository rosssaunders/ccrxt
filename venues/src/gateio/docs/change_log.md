# [#](#changelog) Changelog

**v4.103.0**

- Add `code` field in `GET /spot/account_book` query parameter and response to
  filter account book entries by specific code
- Add `text` parameter to `closeAllPositions` operation for order remarks when
  closing all positions
- New comprehensive AccountBook code documentation with detailed explanations
  for over 300 transaction codes

**v4.102.6**

- Enhance staking swap response structure `SwapCoinStruct` with additional
  fields: `pid`, `subtype`, `exchange_amount`, `updateStamp`, `protocol_type`,
  `client_order_id`, `source`

**v4.102.0**

- Add `is_all_collateral` field in `GET /unified/accounts` endpoint response to
  indicate if all currencies are used as collateral
- Add `enabled_collateral` field in balances array of unified accounts to show
  currency collateral status
- New feature `POST /unified/collateral_currencies` endpoint, Set collateral
  currencies for cross-currency margin mode

**v4.101.9**

- New feature `GET /futures/{settle}/risk_limit_table` endpoint, Query risk
  limit tier table by table_id
- Add `enable_tiered_mm` field in futures account model for tiered maintenance
  margin calculation
- Add `risk_limit_table` and `average_maintenance_rate` fields in position model
  for enhanced risk management
- Add `deduction` field in futures limit risk tiers for maintenance margin quick
  calculation
- Introduce new models: `FuturesRiskLimitTier` and `FuturesRiskLimitTierList`
  for risk management
- Enhance `POST /earn/staking/swap` endpoint response structure with improved
  swap order details

**v4.100.0**

- Add alpha account query and account book query functionality
- New feature `GET /earn/staking/coins` endpoint, Query on-chain staking coin
  types
- New feature `POST /earn/staking/swap` endpoint, On-chain staking coin swap
- Add `sub_broker_info` object field in broker commission and transaction APIs

**v4.99.0**

- Add `refresh_time` field in `GET /spot/accounts` endpoint response
- Remove the `PUT /earn/uni/interest_reinvest` endpoint

**v4.98.0**

- New feature `/earn/uni/rate` endpoint, Currency estimate annualized interest
  rate
- Add `delisting_time`、`trade_url` field in
  `GET /spot/currency_pairs`、`GET /spot/currency_pairs/{currency_pair}`
  endpoint

**v4.97.0**

- New feature `GET /unified/batch_borrowable` endpoint, Batch query unified
  account can be borrowed up to a maximum
- `GET /spot/candlesticks` endpoint, `interval` supports `1s` granularity
- New feature `GET /earn/uni/chart` endpoint, UniLoan currency annualized trend
  chart
- New feature `POST /futures/{settle}/positions/cross_mode` endpoint, Switch to
  the full position-by-store mode

**v4.96.0**

- Add `cross_margin_balance`,`cross_mmr`,`cross_imr` field in
  `GET /futures/{settle}/accounts` response

**v4.95.0**

- Add `code` field in `GET /spot/account_book` query & response
- New feature `GET /unified/transferables` endpoint, Batch query unified account
  can be transferred up to a maximum of
- New feature `GET /margin/user/loan_margin_tiers` endpoint, Check the user's
  own leverage lending gradient in the current market
- New feature `GET /margin/loan_margin_tiers` endpoint, Query the current market
  leverage lending gradient
- New feature `POST /margin/leverage/user_market_setting` endpoint, Set the user
  market leverage multiple
- New feature `GET /margin/user/account` endpoint, Query the user's leverage
  account list

**v4.94.0**

- New feature `GET /unified/currencies` endpoint, List of loan currencies
  supported by unified account
- Add `sub_uid` field in `GET /unified/accounts` query

**v4.93.0**

- Add `plan_id` field in `GET /earn/dual/investment_plan` query
- Add `from`、`to`、`page`、`limit` field in `GET /earn/dual/orders` query
- Add `text` field in `GET /earn/dual/orders` response
- Add `text` field in `POST /earn/dual/orders` response
- New feature `GET /earn/staking/eth2/rate_records` endpoint, Query historical
  rate of ETH2

**v4.92.0**

2025-02-24

- Add `name` field in `GET /spot/currencies` query
- Add `base_name`、`quote_name` field in `GET /spot/currency_pairs` response
- Add `unified` field in `GET /spot/price_orders` query
- Add `sub_uid` field in `GET /unified/accounts` query

**v4.91.0**

2025-02-10

`2025-04-01` After that, we will remove the following interface, please migrate
to the new interface as soon as possible

**v4.90.0**

2025-01-20

- Add `transaction_type` field in `GET /wallet/push` query
- New feature `GET /rebate/user/sub_relation` endpoint, Query whether the
  specified user is in the system
- Add `order_size` field in`GET /futures/{settle}/liq_orders` response
- Add `type` field in `GET /spot/currency_pairs` response

**v4.88.0**

2024-12-24

- New feature `GET /spot/insurance_history` endpoint, Query spot insurance fund
  historical data
- Add
  `cross_balance`、`iso_balance`、`im`、`mm`、`imr`、`mmr`、`margin_balance`、`available_margin`
  field in `GET /unified/accounts` response
- `PUT /unified/unified_mode` endpoint，Added single-currency margin mode

**v4.87.0**

- New feature `GET /unified/history_loan_rate` endpoint, Get historical lending
  rates

**v4.86.0**

2024-12-02

- New feature `GET /wallet/order_status` endpoint. Transfer status query
- Add `update_id` field in `GET /futures/{settle}/positions` response

**v4.85.0**

2024-11-11

- Add `x-gate-exptime` field in
  `POST /futures/{settle}/orders`、`POST /spot/batch_order` header.

Add
`cross_order_margin`、`cross_initial_margin`、`cross_maintenance_margin`、`cross_unrealised_pnl`、`cross_available`、`isolated_position_margin`
field in `POST /futures/{settle}/dual_mode` response.

**v4.84.0**

2024-11-04

- New feature `GET /loan/multi_collateral/current_rate` endpoint, Query the
  current interest rate of the currency
- Add `lowest_size`、`highest_size` field in `GET /spot/tickers` response
- Add `amount` field in `POST /earn/dual/orders` request body

**v4.83.0**

2024-10-28

- New feature `GET /unified/leverage/user_currency_config` endpoint, Query the
  maximum and minimum leverage multiples that users can set for a currency
- New feature `GET /unified/leverage/user_currency_setting` endpoint, Get the
  user's currency leverage
- New feature `POST /unified/leverage/user_currency_setting` endpoint, Set the
  currency leverage ratio
- Add `id` field in `GET /futures/{settle}/account_book` response
- Add `leverage` field in `GET /unified/currency_discount_tiers` response

**v4.82.0**

2024-10-14

- New feature `GET /account/rate_limit` endpoint, Get user flow limit
  information. For details, please refer to
  [Trade Ratio Rate Limiting](#trade-ratio-rate-limiting)
- `GET /account/detail` 接口, 返回值增加 `copy_trading_role` field

**v4.81.0**

2024-09-30

- New feature `POST /options/countdown_cancel_all` endpoint, Countdown to cancel
  order
- Add `message` field in `GET /wallet/push` response
- Add `from`、`to` in `GET /futures/{settle}/funding_rate` query
- Add `is_max` field in `POST /earn/dual/orders` response

**v4.80.0**

2024-09-09

- New feature `GET /options/mmp` endpoint, MMP Query
- New feature `POST /options/mmp` endpoint, MMP Settings
- New feature `POST /options/mmp/reset` endpoint, MMP Reset
- Add `block_number` field in `GET /wallet/withdrawals` response

**v4.79.0**

2024-09-02

- Add `from`、`to` field in `GET /unified/interest_records` query
- Add `options` field in `GET /unified/unified_mode` response
- Add `options` field in `PUT /unified/unified_mode` request body

**v4.78.0**

2024-08-19

- New feature `GET /wallet/push` endpoint, Get Records
- New feature `POST /withdrawals/push` endpoint, Transfer between spot main
  accounts. Both parties cannot be sub-accounts.
- New feature `GET /futures/{settle}/batch_amend_orders` endpoint, Batch modify
  orders with specified IDs
- Add `close_size` field in `GET /futures/{settle}/my_trades` response
- Add `tx_id` field in `POST /wallet/transfers` response

**v4.77.0**

2024-08-05

- New feature: add `GET /sub_accounts/unified_mode` endpoint，Get sub-account
  mode
- Add `from`、`to` field in `GET /rebate/broker/commission_history` query
- Add `from`、`to` field in `GET /rebate/broker/transaction_history` query

**v4.76.0**

2024-07-22

- New feature: add `GET /rebate/partner/sub_list` endpoint，Partner subordinate
  list
- Add `page`、`limit` field in `GET /flash_swap/currency_pairs` query
- Add `order_id`、`currency_pair`、`account` field in
  `PATCH /spot/orders/{order_id}`
- Add `order_id`、`currency_pair`、`account` field in
  `DELETE /spot/orders/{order_id}`

**v4.75.1**

2024-07-08

- New feature: add `GET /delivery/{settle}/risk_limit_tiers` endpoint，querying
  risk limit levels
- New feature: add `GET /rebate/partner/transaction_history` endpoint，partners
  to get the transaction history of recommended users
- Add `borrow_type` field in `GET /unified/loan_records` response
- Add `accum_size` field in `GET /futures/{settle}/position_close` response

**v4.75.0**

2024-06-24

- New feature: add `GET /account/debit_fee` endpoint，query GT deduction
  configuration.
- New feature: add `POST /account/debit_fee` endpoint, to enable or disable GT
  deduction for the current account.

**v4.74.1**

2024-06-11

- Optimization of DOM for the visible area on mobile devices

**v4.74.0**

2024-05-29

- New feature: add `GET /unified/loan_margin_tiers` endpoint, list loan margin
  tiers

**v4.73.0**

2024-05-27

- Add `is_all` parameter in `POST /wallet/small_balance` endpoint
- Add `text` field in `POST /spot/cancel_batch_orders` response
- Add `funding`、`funding_version`、`use_funding` field in
  `GET /unified/accounts` response

**v4.72.0**

2024-05-13

- Add `last_access` field in `GET /sub_accounts/{user_id}/keys` response
- Add `contract` field in `GET /futures/{settle}/risk_limit_tiers` response

**v4.71.0**

2024-04-23

- Add `page` parameter in `GET /wallet/saved_address` endpoint
- New feature: add `GET /api/v4/rebate/user/info` endpoint, retrieve user rebate
  information
- New feature: add `POST /unified/portfolio_calculator` endpoint, portfolio
  margin calculator
- New feature: add `GET /unified/risk_units` endpoint, retrieve user risk unit
- New feature: add `PUT /unified/unified_mode` endpoint, set unified account
  mode
- New feature: add `GET /unified/unified_mode` endpoint, retrieve unified
  account mode

**v4.70.0**

2024-04-08

- Add `pnl_pnl`、`pnl_fund`、`pnl_fee` field in
  `GET /futures/{settle}/positions` response
- Add `pnl_pnl`、`pnl_fund`、`pnl_fee` field in
  `GET /futures/{settle}/position_close` response

**v4.69.0**

2024-03-25

- Add `text` field in `POST /delivery/{settle}/price_orders` response

**v4.68.0**

2024-03-18

- New feature: add `GET /unified/currency_discount_tiers` endpoint, list
  currency discount tiers
- Add `type` parameter in `GET /unified/loans` endpoint
- Add `type` parameter in `GET /unified/interest_records` endpoint

**v4.67.0**

2024-03-11

- Add `filled_amount` field in `POST /spot/orders`,`POST /spot/batch_orders`
  response
- In frequency limit rule for the wallet withdrawal interface, the speed limit
  description has been corrected from `10r/10s` to `1r/3s`(No modification to
  the original rate limiting behavior)

**v4.66.1**

2024-02-19

- New feature: add `GET /wallet/small_balance` endpoint, list small balance.
- New feature: add `GET /wallet/small_balance_history` endpoint, list small
  balance history.
- New feature: add `GET /unified/estimate_rate` endpoint, get unified estimate
  rate.

**v4.65.0**

2024-01-29

- Add `debit_fee` field in `GET /spot/batch_fee` response
- Add `user_id` parameter in `DELETE /account/stp_groups/{stp_id}/users`
  endpoint
- Spot API introduces asynchronous support modes for create orders: `ACK`,
  `RESULT`, `FULL`. For details, please refer to [SPOT API](#create-an-order)

**v4.64.0**

2024-01-22

- Add `order_type` parameter in `GET /loan/multi_collateral/orders` endpoint
- Add
  `order_type`,`fixed_type`,`fixed_rate`,`expire_time`,`auto_renew`,`auto_repay`
  field in `GET /loan/multi_collateral/orders` response
- Add `before_ltv`,`after_ltv` field in `GET /loan/multi_collateral/repay`
  response
- New feature: add `GET /loan/multi_collateral/fixed_rate` endpoint, query
  multi-collateral fix rate.
- Add `unrealised_pnl`,`borrowed` field in `GET /wallet/total_balance` response

**v4.63.0**

2024-01-15

- Add `decimal` field in `GET /wallet/currency_chains` response
- New feature: add `GET /futures/{settle}/risk_limit_tiers` endpoint, list risk
  limit tiers.

**v4.62.0**

2024-01-02

- New feature: add `POST /futures/{settle}/batch_cancel_orders` endpoint, users
  have the ability to batch cancel orders.
- New feature: add multi-collateral-loan api. (`/loan/multi_collateral/**`)

**v4.61.0**

2023-12-18

- New features: The broker obtains the user's commission rebate records in
  `GET /rebate/broker/commission_history` and
  `GET /rebate/broker/commission_history` endpoints

**v4.60.0**

2023-12-01

- Breaking change: New [Unified API](#unified-account) is online. The old
  `/portfoli/*` endpoints are deprecated.
- New features: add earn product api. (`/earn/**`)
- Add `trade_id` field in `GET /futures/{settle}/account_book` response

**v4.59.0**

2023-11-22

- Add `funding_cap_ratio` field in `GET /futures/{settle}/contracts` response
- Add `contract` field in `GET /delivery/{settle}/account_book` response
- Add `withdraw_percent_on_chains` field in `GET /wallet/withdraw_status`
  response
- Add `leverage` field in `GET /portfolio/accounts` response

**v4.58.0**

2023-11-03

- Add `tier` field in `GET /account/detail` response
- Add `max_base_amount`、`max_quote_amount` field in `GET /spot/currency_pairs`
  response

**v4.57.0**

2023-10-20

- New feature: API Gateway inbound & outbound time, For more details, please
  refer to the [API Gateway in/out time](#api-gateway-in-out-time)
- New feature: support portfolio account in `POST /spot/orders` endpoint
- New feature: add `PUT /earn/uni/interest_reinvest` endpoint, users have the
  option to enable or disable interest reinvestment.
- New feature: add `POST /spot/amend_batch_orders` endpoint, users have the
  ability to batch modify orders.
- Add `sequence_id` field in `GET /spot/trades` response
- Add `text` field in `GET /spot/account_book` response
- Add `text` field in `GET /spot/my_trades` response
- `GET /portfolio/spot/orders`、
  `GET /portfolio/spot/orders`、`GET /portfolio/spot/orders/{order_id}`、`DELETE /portfolio/spot/orders/{order_id}`
  and `PATCH /portfolio/spot/orders/{order_id}` have been deprecated. We will
  remove the endpoints by the end of October 2023. Please use `/spot/orders`
  instead.

**v4.56.0**

2023-09-25

- Add `repayment_type` field in `GET /portfolio/loan_records` endpoint.
- Add request parameter `holding` in `GET /futures/{settle}/positions` endpoint
- Add request parameter `role` in `GET /futures/{settle}/my_trades_timerange`
  endpoint
- Add request parameter `side` and `pnl` in
  `GET /futures/{settle}/position_close` endpoint

**v4.55.0**

2023-09-12

- Add new `POST /portfolio/account_mode` endpoint, allow to change the mode.

**v4.54.0**

2023-08-28

- Add `contract_address` field in `GET /wallet/currency_chains` endpoint.
- Add `GET /portfolio/spot/currency_pairs` and
  `GET /portfolio/spot/currency_pairs/{currency_pair}` endpoints, list portfolio
  spot's currency pairs.

**v4.53.0**

2023-08-14

- New feature: delete user in STP group in
  `DELETE /account/stp_groups/{stp_id}/users` endpoint

**v4.52.0**

2023-08-07

- New feature: add collateral loan api

**v4.51.0**

2023-07-29

- Adjusted and optimized the [account book types](#accountbook-type)
- Add `mode` field in `GET /account/detail` edpoint.

**v4.50.0**

2023-07-14

- New feature: New [Portfolio API](#portfolio-account). Currently, these
  services are only available to whitelisted users. If you are interested in
  accessing these APIs, please contact our institutional department for further
  information.
- Add new endpoint `GET /flash_swap/currency_pairs`, list all flash swap
  currency pair

**v4.49.0**

2023-07-03

- Add new [frequency limit rule](#frequency-limit-rule)，the new rule is
  expected to take effect on 2023-07-10 (UTC+8)
- In the `GET /futures/{settle}/orders` API endpoint, the request field
  `contract` has been modified to be optional instead of mandatory.

**v4.48.0**

2023-06-16

- Add `client_order_id` fields in `GET /wallet/sub_account_transfers` edpoint.

**v4.47.0**

2023-05-23

- New feature: add STP group admin api
- New feature: query estimated interest rates of margin in
  `GET /margin/uni/estimate_rate` endpoint.
- New feature: list futures order by time range in
  `GET /futures/{settle}/orders_timerange` endpoint
- Add
  `underlying`、`underlying_price`、`mark_iv`、`delta`、`gamma`、`vega`、`theta`
  fields in `GET /options/positions/{contract}` endpoint.

**v4.46.0**

2023-05-08

- New feature: query spot account book in `GET /spot/account_book` endpoint
- New feature: query user futures trading fee in `GET /futures/{settle}/fee`
  endpoint

**v4.45.0**

2023-04-21

- The margin loan has been migrated to the `Lend & Earn`. For more information,
  please refer to the
  [Margin Migration Instructions](#margin-migration-instructions)
- New feature: Add `Self-Trade Prevention` feature in the
  `POST /futures/{settle}/batch_orders` endpoint.

**v4.44.0**

2023-04-07

- Add `ORDER_BOOK_NOT_FOUND` and `FAILED_RETRIEVE_ASSETS` error messages.

**v4.43.0**

2023-03-27

- New feature: Add `Self-Trade Prevention` feature in the `POST /spot/orders`
  endpoint. Fore more detail, please refer to
  [STP overview](#self-trade-prevention-stp)
- New feature: Get API key's ip whitelist in `GET /account/detail` endpoint.
- Add `amend_text` in `PATCH /spot/orders/{order_id}` endpoint.

**v4.42.0**

2023-03-13

- New feature: add `Lend & Earn` API
- New feature: Add `Self-Trade Prevention` feature in the
  `POST /futures/{settle}/orders` endpoint. Fore more detail, please refer to
  [STP overview](#self-trade-prevention-stp)
- Add `delivery` account type in `POST /wallet/sub_account_transfers` endpoint

**v4.41.0**

2023-03-03

**v4.40.0**

2023-02-24

- New feature: List Auto-Deleveraging history endpoint
  `Get /futures/{settle}/auto_deleverages`
- Add `sum` field in `GET /futures/{settle}/candlesticks` endpoint

**v4.39.0**

2023-02-09

- New feature: Query a batch of user trading fee rate endpoint
  `GET /spot/batch_fee`
- Add `enable_bonus`、`enable_credit` fields in
  `GET /futures/{settle}/contracts` endpoint

**v4.38.0**

2023-02-04

- New feature: time range query for my futures trade endpoint
  `GET /futures/{settle}/my_trades_timerange`
- Add `withdraw_order_id` field in `POST /withdrawals` endpoint

**v4.37.0**

2023-01-20

- Add new rebate API endpoints.

**v4.36.0**

2022-12-23

- Hiding all amount is not supported any more when using `iceberg` in
  `POST /spot/orders` and `POST /spot/batch_orders` endpoints

**v4.35.0**

2022-12-09

- New feature: amend order endpoint `/spot/orders/{order_id}`
- Add `avg_deal_price` field in `GET /spot/orders` response
- Support market order in `POST /spot/batch_orders` endpoint

**v4.34.0**

2022-11-25

- Support market order in `POST /spot/orders` endpoint

**v4.33.0**

2022-11-11

- New feature: Futures Premium Index endpoint
  `GET /futures/{settle}/premium_index`
- Allow to specify password and email when creating a sub-account.

**v4.32.0**

2022-10-28

- Improve options api document

**v4.31.0**

2022-10-14

- Allow to transfer futures and cross_margin funds between two sub-accounts in
  `POST /wallet/sub_account_to_sub_account` endpoint

**v4.30.0**

2022-09-23

- New feature: manage sub-accounts API Key
- New feature: lock and unlock sub-account endpoint
- Allow to transfer between two sub-accounts in
  `POST /wallet/sub_account_to_sub_account` endpoint

**v4.29.0**

2022-09-09

- New feature: create and list sub-accounts
- Add `settle` parameter in `GET /wallet/fee` endpoint
- Add `refr` field in option order
- The maximum number of API Keys changes to 20

**v4.28.0**

2022-08-12

- Add `offset` parameter in `GET /futures/{settle}/trades`
- new countdown cancel orders endpoint for spot and futures.

**v4.27.0**

2022-07-29

- Add `X-Client-Request-Id` http header for tracking request
- new create a batch of futures order endpoint
  `POST /futures/{settle}/batch_orders`
- new `FOK` tif type for futures order

**v4.26.0**

2022-07-15

- Spot Price-Trigger order supports portfolio margin account
- Add `GET /wallet/saved_address` to list saved address
- `POST /wallet/transfers` returns `tx_id` field
- Add `GET /wallet/sub_account_cross_margin_balances` to query subaccount's
  `cross_margin` account
- Add `status` field in `GET /margin/currency_pairs` response

**v4.25.1**

2022-07-06

- New `GET /spot/time` endpoint which get system's time info.
- New `GET /options/my_settlements` endpoint which list my selttlements.
- Add `change_utc0`, `change_utc8` fields in `GET /spot/tickers` endpoint

**v4.25.0**

2022-06-24

- Support portfolio margin account API
- Cross-margin add more fields. Please refer to endpoint document for more
  details.
- New `POST /spot/cross_liquidate_orders` spot trading endpoint that close
  position when the cross-currency is disabled
- Add `bouns` and `history` fields in `GET /futures/{settle}/accounts` endpoint
- Add `text`、`fee` and `point_fee` fields in `GET /futures/{settle}/my_trades`
  endpoint
- Fix typo for `cancel a price-triggered order` endpoints
- `POST /wallet/sub_account_transfers` supports transferring to `cross_margin`

**v4.24.0**

2022-05-20

- Support flash swap operations with new API group `/flash_swap`. Spot operation
  permission is required.
- New wallet APIs `GET /wallet/sub_account_margin_balances` and
  `GET /wallet/sub_account_futures_balances` to help main account retrieving sub
  accounts' margin and perpetual contract balances
- New perpetual contract API `GET /futures/{settle}/index_constituents/{index}`
  to retrieve index price constituents
- Fix missing fields like `order_type` in `FuturesPriceTriggeredOrder`

**v4.23.4**

2022-04-25

- Spot candlesticks supports `30d` interval

**v4.23.3**

2022-04-01

1.  Spot candlestick API returns base currency amount
2.  Spot currency detail add `chain` field.
3.  Add withdrawal and deposit status in `GET /wallet/currency_chains` response
4.  Add missing `cross_leverage_limit` in perpetual contract's dual mode
    position leverage update API
5.  Support more intervals in perpetual and delivery contract candlesticks

**v4.23.2**

2022-01-21

1.  Add `fee` in withdrawal and deposit history
2.  Add fix fee rate in spot `Currency`

**v4.23.1**

2021-12-23

1.  Spot orders support new `time_in_force` `FOK`
2.  New `FOK_NOT_FILL` error label

**v4.23.0**

2021-12-09

1.  Add options API
2.  Add detailed rate limiting rules
3.  Add `GET /wallet/currency_chains` to retrieve chains supported by currency
4.  Add additional status for deposit and withdrawal history

**v4.22.4**

2021-11-01

1.  Data type of `ctime` and `ftime` in `SpotPriceTriggeredOrder` should be
    `int64`

**v4.22.3**

2021-10-27

1.  `GET /spot/trades` supports time range based query using `from` and `to`.

**v4.22.2**

2021-09-29

1.  Add more status in withdrawal or deposit record model
2.  Add new write only field `auto_size` in `FuturesOrder` to support closing
    dual mode position.

**v4.22.1**

2021-09-07

1.  New wallet API `GET /wallet/total_balance` to retrieve all user's estimate
    balance.
2.  Add `locked` and `risk` in margin account response
3.  Margin and cross margin loans support custom text input.

**v4.22.0**

2021-08-13

1.  Delivery contract API supports BTC settled
2.  Spot API `GET /spot/orders` and `GET /spot/my_trades` supports query by time
    range
3.  Add margin and cross margin max borrowable API
4.  Multiple document enhancements.

**v4.21.6**

2021-08-12

1.  Fix incorrect address field name in `GET /wallet/deposit_address`

**v4.21.5**

2021-06-30

- `GET /spot/orders`, `GET /spot/orders/{order_id}` and `GET /spot/my_trades`
  allow empty `currency_pair` if operated against finished orders
- Add fixed withdrawal fee on multiple chains in `GET /wallet/withdraw_status`
  response
- Add `GET /margin/transferable` to retrieve maximum transferable amount from
  margin account
- Add `from` and `to` parameter to specify time range for futures position
  closes history API

**v4.21.4**

2021-06-23

- Add millisecond timestamp in `GET /margin/account_book` response

**v4.21.3**

2021-06-17

- Add order book timestamp for both spot and futures trading

**v4.21.2**

2021-06-07

- Futures API support cross margin leverage modification
- Add new spot cross margin API `/margin/cross`
- Add spot order operations using spot cross margin account
- Add unpaid interests in spot margin account query
- Add new millisecond fields `create_time_ms` and `update_time_ms` in spot
  orders.
- Add `DELETE /withdrawals/{withdrawal_id}` to cancel withdrawal operation

**v4.20.1**

2021-04-14

- Update document links

**v4.20.0**

2021-03-25

- Support spot auto orders with API group `/spot/price_orders`

**v4.19.6**

2021-03-22

- Add trading timestamp in spot currency pair

**v4.19.5**

2021-03-18

- Spot and Futures operations based on order ID also accept user custom ID(but
  only for 30 minutes since creation)

**v4.19.4**

2021-03-10

- `/wallet/sub_account_transfers` supports transferals with sub user's perpetual
  contract account

**v4.19.3**

2021-03-04

- Add margin loans auto repay API `/margin/auto_repay`
- Add `multichain_address` in `/wallet/deposit_address` for currencies with
  multiple deposit addresses
- Optimize documentation

**v4.19.2**

2021-03-01

- Add `/wallet/fee` API to retrieve trading fee. Previous `/spot/fee` is
  deprecated in favour of this one.
- Add new field `chain` in withdrawal operation.
- Add new field `with_id` in `/futures/{settle}/order_book` API and `id` field
  in its response
- Add new `offset` in API `/futures/{settle}/position_close` to retrieve
  position close history with pagination.
- Add contract value calculation. Refer to `Contract` model for details.
- Fix incorrect field type in futures stats API

**v4.18.4**

2021-01-22

- Add field `create_time_ms` in spot `Trade` model
- ETF currency pairs' ticker add net value related info

**v4.18.1**

2021-01-07

- Add iceberg order support for spot orders
- Fix incorrect field types in `/futures/{settle}/contract_stats`

**v4.18.0**

2020-12-21

- Add new spot API`/spot/currencies` and `/spot/currencies/{currency}` to
  retrieve currency info
- Add more fields, e.g., `top_lsr_account`, `top_lsr_size`, in futures
  `ContractStat` model.

**v4.17.1**

2020-12-16

- Increase maximum of `limit` in `/spot/order_book` to 100

**v4.17.0**

2020-12-15

- Add `/wallet/sub_account_balances` to retrieve sub accounts' balances.

**v4.16.1**

2020-12-10

- Fix mistaken field name `dual_mode` in futures position model which should be
  `mode` instead.

**v4.16.0**

2020-12-09

_Spot_

- Increase order number limit each currency pair to 10 in
  `POST /spot/batch_orders`
- Add new query parameter `reverse` in `GET /spot/trades` to trace back trading
  history

_Futures_

- Add perpetual contract dual mode position support. Use
  `/futures/{settle}/dual_mode` to set position's dual mode. For dual mode
  position operations, refer to `/futures/{settle}/dual_comp/positions` API
  group
- Add perpetual contract new field `in_dual_mode` in futures account response
  model; `dual_mode` in position response model.
- Add new perpetual contract public API `/futures/{settle}/liq_orders` to query
  liquidated orders in markets

**v4.15.5**

2020-11-04

- Add `/futures/{settle}/contract_stats` API to retrieve contract stats
- Add `/margin/{currency_pair}` to retrieve single margin currency pair detail

**v4.15.4**

2020-09-01

- Add `point_type` in `GET /spot/fee` response
- Add `GET /wallet/withdraw_status` API
- Add C# SDK entry

**v4.15.2**

2020-08-12

- Add `GET /spot/fee` to retrieve spot order trading fee rates

**v4.15.1**

2020-08-04

- Add `GET /spot/open_orders` to retrieve all open orders in spot trading
- Add `GET /margin/account_book` to retrieve margin account balance history

**v4.14.1**

2020-07-08

- maximum length of `text` field in order extends to 28(prefix excluded)

**v4.14.0**

2020-07-06

- New Delivery contract APIs `/delivery`

**v4.13.1**

2020-06-28

- Add `GET /wallet/sub_account_transfers` to list sub account transfer records

**v4.13.0**

2020-05-20

- APIv4 now supports withdraw API. Refer to `POST /withdrawals` and
  "Authentication" section for details.
- `POST /wallet/transfers` supports transferring between spot and futures
  account
- Wallet API supports retrieving deposits and withdrawals history
- Futures orders and personal trades retrieving now supports `offset` field
- Futures `Contract` model add new field `in_delisting`

**v4.12.0**

2020-04-08

- APIv4 Key management improved. Keys are no longer separated with different
  trading types. Every key can now have multiple operation permissions. Refer to
  _"About APIv4 key improvement"_ for details.
- Add `POST /wallet/sub_account_transfers` to support transferring between main
  and sub account
- `GET /spot/candlesticks` adds query parameters `from` and `to` to support
  retrieving history data points

**v4.11.2**

2020-03-29

- Add `filled_total` in `Order` to replace `fill_price` (the latter is badly
  named)
- Add new error label `POC_FILL_IMMEDIATELY`

**v4.11.1**

2020-03-23

- Add `role` in `GET /spot/my_trades` response
- Fix missing currency account in `GET /margin/funding_accounts`

**v4.11.0**

2020-03-20

- Spot order supports GT fee discount
- Spot order time in force supports `poc`

**v4.10.1**

2020-02-24

- Add `trade_status` in spot currency pair

**v4.10.0**

2020-02-17

- Margin order creation adds new field `auto_borrow`(write only) to borrow the
  insufficient part by the system if balance is not enough
- Add new API `POST /spot/cancel_batch_orders` to support batch cancellation
  with specified order IDs
- Add new document section "Error handling" and "Which one to choose, APIv4 or
  APIv2?"

**v4.9.1**

2020-01-07

- Add fee and recent modification time in `Order` and `BatchOrder`
- Add fee in `GET /spot/my_trades` response

**v4.9.0**

2019-12-17

- `last_id` in `GET /futures/{settle}/trades` is deprecated. Use `from` and `to`
  to retrieve trading history

**v4.8.2**

2019-12-02

- Add `/spot/batch_orders` to support creating a bundle of spot or margin orders
- Fee rate of margin loan repayment enjoys VIP discount
- `Loan` add new fields `fee_rate`(fee rate of lending loan) and
  `orig_id`(original loan ID if loan is auto renewed)

**v4.8.1**

2019-11-27

- Fix missing `settle` in `GET /futures/{settle}/positions` docs and code
  snippet

**v4.8.0**

2019-11-07

- Futures API now supports settling in USDT.
- Change `/futures` to `/futures/{settle}` in ALL futures API to support futures
  operations in different settle currency.
- `currency` field in `/futures/{settle}/accounts` response adds new value:
  `USDT` response to replace `volume_24h_btc` and `volume_24h_usd`. The latter
  two are still preserved for compatibility usage, but are NOT recommended for
  any futures operations.

To use USDT futures, just replace `/futures` with `/futures/usdt`, e.g. use
`GET /futures/usdt/accounts` to retrieve futures accounts settled in USDT, while
`GET /futures/btc/accounts` returns accounts in BTC.

For compatibility, `GET /futures/xxx` defaults to `GET /futures/btc/xxx`, e.g.
`GET /futures/accounts` will be treated as `GET /futures/btc/accounts`

**v4.7.3**

2019-07-18

- Add `text` in `/spot/orders` and `/futures/orders` to support user defined
  order information

**v4.6.3**

2019-06-11

- Add point information in Futures account and position

**v4.7.2**

2019-05-29

- Change `rate` in `Loan` as non-required for lending side.

**v4.7.1**

2019-04-17

- Add wallet v4 API. Support transfers between spot and margin account for now.
- `GET /margin/loans` can sort by `rate` and support an optional parameter
  `currency_pair`
- Fix miscellaneous document issues

**v4.6.2**

2019-04-24

- Fix price-triggered futures order's docs incorrect override docs for
  `GET /futures/orders/{order_id}` and `DELETE /futures/orders/{order_id}`

**v4.6.1**

2019-04-02

- Add `high_24h`, `low_24h` and `funding_rate_indicative` in futures ticker

**v4.6.0**

2019-03-21

_SDK related only_

- Rename futures order related function name in SDKs to avoid duplication with
  spot order API in Go
- Fix query parameter not decoded while generating authentication signature

**v4.5.2**

2019-03-14

- `currency_pair` in `/spot/order_book` should be a required parameter
- Optimize document code samples

**v4.5.1**

2019-03-11

- Fix missing URL parameter description

**v4.5.0**

2019-03-05

To avoid version confusion, all versions in APIv4 (documents and SDKs are both
included) will start with `4` from now on

- Add Spot v4 API to provide improved API capability
- Add Margin v4 API to provide support for margin loans and trading
- Add Futures price triggered auto order API support. Refer to
  `/futures/price_orders` for details
- Base URL of all Gate API v4 real trading changed to
  `https://api.gateio.ws/api/v4`

**v1.3.0**

2019-02-13

_Important update_

- Domain of base URLs are changed to `fx-api.gateio.ws` and
  `fx-api-testnet.gateio.ws` respectively, `*.gateio.io` is deprecated and will
  soon be out of service.

**v1.2.1**

2019-02-13

- Add `volumn_24h_usd` and `volume_24h_btc` in `GET /futures/tickers` response

**v1.2.0**

2019-01-17

- Add `GET /futures/contracts/{contract}` to get one single contract
- Add `GET /futures/positions/{contract}` to get one single position
- Add `GET /futures/account_book` to retrieve user account balance history
- Add `config_change_time` in `Contract` model
- fix miscellaneous document issues

**v1.1.0**

2019-01-08

- Add more fields to `Contract`, `Position`, `FuturesOrder`
- Add API `GET /futures/position_close` to retrieve position close history
- Add optional `order_id` support for API `GET /futures/my_trades`
- Change the status code of `DELETE /futures/orders` and
  `DELETE /futures/orders/{order_id}` from 204 to 200, with cancelled order
  details returned on success.
- Request `DELETE /futures/orders/{order_id}` with invalid order ID or order
  that has been finished will return 404 instead of ignoring the error
- `POST /futures/orders` now supports POC, iceberg

**v1.0.0**

2018-12-30

- Initial release
