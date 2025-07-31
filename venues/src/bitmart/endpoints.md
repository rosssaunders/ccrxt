BitMart REST API Endpoints by Category

Below is a comprehensive list of all BitMart REST API endpoints, organized by category (Spot, Margin, Futures, Account, Sub-Account, Broker, and System). Each entry includes the endpoint name/description, relative URL path, category, and a direct link to the official BitMart API documentation section for that endpoint.

System Endpoints

Endpoint	Relative Path	API Category	Documentation
Get System Time – Get the current system time ￼	/system/time	System	https://developer-pro.bitmart.com/en/spot/#get-system-time
Get System Service Status – Get current API service status ￼	/system/service	System	https://developer-pro.bitmart.com/en/spot/#get-system-service-status

Spot Endpoints

Endpoint	Relative Path	API Category	Documentation
Get Currency List (V1) – List all cryptocurrencies on the platform ￼	/spot/v1/currencies	Spot	https://developer-pro.bitmart.com/en/spot/#get-currency-list-v1
Get Trading Pairs List (V1) – List all trading pairs ￼	/spot/v1/symbols	Spot	https://developer-pro.bitmart.com/en/spot/#get-trading-pairs-list-v1
Get Trading Pair Details (V1) – Detailed info for all trading pairs ￼	/spot/v1/symbols/details	Spot	https://developer-pro.bitmart.com/en/spot/#get-trading-pair-details-v1
Get Ticker of All Pairs (V3) – Latest ticker for all pairs ￼ ￼	/spot/quotation/v3/tickers	Spot	https://developer-pro.bitmart.com/en/spot/#get-ticker-of-all-pairs-v3
Get Ticker of a Trading Pair (V3) – Latest ticker for a specific pair ￼	/spot/quotation/v3/ticker	Spot	https://developer-pro.bitmart.com/en/spot/#get-ticker-of-a-trading-pair-v3
Get Latest K-Line (V3) – Latest candle data (up to 1000 records) ￼	/spot/quotation/v3/lite-klines	Spot	https://developer-pro.bitmart.com/en/spot/#get-latest-k-line-v3
Get History K-Line (V3) – Historical candle data for a time range ￼	/spot/quotation/v3/klines	Spot	https://developer-pro.bitmart.com/en/spot/#get-history-k-line-v3
Get Depth (V3) – Order book depth data ￼	/spot/quotation/v3/books	Spot	https://developer-pro.bitmart.com/en/spot/#get-depth-v3
Get Recent Trades (V3) – Latest trades for a trading pair ￼	/spot/quotation/v3/trades	Spot	https://developer-pro.bitmart.com/en/spot/#get-recent-trades-v3
New Order (v2) – Place a new spot order ￼	/spot/v2/submit_order	Spot	https://developer-pro.bitmart.com/en/spot/#new-orderv2-signed
Cancel Order (v3) – Cancel a specific open order ￼	/spot/v3/cancel_order	Spot	https://developer-pro.bitmart.com/en/spot/#cancel-orderv3-signed
New Batch Order (v4) – Place multiple orders in batch ￼	/spot/v4/batch_orders	Spot	https://developer-pro.bitmart.com/en/spot/#new-batch-orderv4-signed
Cancel Batch Order (v4) – Cancel multiple orders (by IDs or criteria) ￼	/spot/v4/cancel_orders	Spot	https://developer-pro.bitmart.com/en/spot/#cancel-batch-orderv4-signed
Cancel All Order (v4) – Cancel all open orders for a trading pair ￼	/spot/v4/cancel_all	Spot	https://developer-pro.bitmart.com/en/spot/#cancel-all-orderv4-signed
Query Order By Id (v4) – Query a single order by its ID ￼	/spot/v4/query/order	Spot	https://developer-pro.bitmart.com/en/spot/#query-order-by-id-v4-signed
Query Order By clientOrderId (v4) – Query a single order by client-defined ID ￼	/spot/v4/query/client-order	Spot	https://developer-pro.bitmart.com/en/spot/#query-order-by-clientorderidv4-signed
Current Open Orders (v4) – Get current open orders (status new/partial) ￼	/spot/v4/query/open-orders	Spot	https://developer-pro.bitmart.com/en/spot/#current-open-ordersv4-signed
Account Orders (v4) – Get account order history (filled/canceled orders) ￼	/spot/v4/query/history-orders	Spot	https://developer-pro.bitmart.com/en/spot/#account-ordersv4-signed
Account Trade List (v4) – Get all trades for the account ￼	/spot/v4/query/trades	Spot	https://developer-pro.bitmart.com/en/spot/#account-trade-listv4-signed
Order Trade List (v4) – Get trade details for a specific order ￼	/spot/v4/query/order-trades	Spot	https://developer-pro.bitmart.com/en/spot/#order-trade-listv4-signed

Margin Endpoints

Endpoint	Relative Path	API Category	Documentation
New Margin Order (v1) – Place a new isolated margin order ￼	/spot/v1/margin/submit_order	Margin	https://developer-pro.bitmart.com/en/spot/#new-margin-orderv1-signed
Margin Borrow (Isolated) – Borrow assets on isolated margin ￼	/spot/v1/margin/isolated/borrow	Margin	https://developer-pro.bitmart.com/en/spot/#margin-borrow-isolated-signed
Margin Repay (Isolated) – Repay borrowed assets on isolated margin ￼	/spot/v1/margin/isolated/repay	Margin	https://developer-pro.bitmart.com/en/spot/#margin-repay-isolated-signed
Get Margin Account Details (Isolated) – Query isolated margin account info ￼	/spot/v1/margin/isolated/account	Margin	https://developer-pro.bitmart.com/en/spot/#get-margin-account-detailsisolated-keyed
Margin Asset Transfer – Transfer assets between spot and isolated margin ￼	/spot/v1/margin/isolated/transfer	Margin	https://developer-pro.bitmart.com/en/spot/#margin-asset-transfer-signed
Get Borrow Record (Isolated) – Borrowing records for isolated margin ￼	/spot/v1/margin/isolated/borrow_record	Margin	https://developer-pro.bitmart.com/en/spot/#get-borrow-recordisolated-keyed
Get Repayment Record (Isolated) – Repayment records for isolated margin ￼	/spot/v1/margin/isolated/repay_record	Margin	https://developer-pro.bitmart.com/en/spot/#get-repayment-recordisolated-keyed
Get Trading Pair Borrowing Rate and Amount – Interest rates and limits for margin pairs ￼	/spot/v1/margin/isolated/pairs	Margin	https://developer-pro.bitmart.com/en/spot/#get-trading-pair-borrowing-rate-and-amount-keyed

Account (Funding) Endpoints

Endpoint	Relative Path	API Category	Documentation
Get Account Balance – Get main account balances (all wallets) ￼	/account/v1/wallet	Account	https://developer-pro.bitmart.com/en/spot/#get-account-balance-keyed
Get Spot Wallet Balance – Get user’s spot wallet balances ￼	/spot/v1/wallet	Account	https://developer-pro.bitmart.com/en/spot/#get-spot-wallet-balance-keyed
Get Currencies – Get info on all supported currencies ￼	/account/v1/currencies	Account	https://developer-pro.bitmart.com/en/spot/#get-currencies
Deposit Address – Get deposit address for each currency ￼	/account/v1/deposit/address	Account	https://developer-pro.bitmart.com/en/spot/#deposit-address-keyed
Withdraw Quota – Query withdrawal limit/quota for a currency ￼	/account/v1/withdraw/charge	Account	https://developer-pro.bitmart.com/en/spot/#withdraw-quota-keyed
Withdraw – Submit a withdrawal request ￼	/account/v1/withdraw/apply	Account	https://developer-pro.bitmart.com/en/spot/#withdraw-signed
Withdraw Address – Get the list of withdrawal addresses ￼	/account/v1/withdraw/address/list	Account	https://developer-pro.bitmart.com/en/spot/#withdraw-address-keyed
Get Deposit And Withdraw History – Query past deposit/withdraw records ￼	/account/v2/deposit-withdraw/history	Account	https://developer-pro.bitmart.com/en/spot/#get-deposit-and-withdraw-history-keyed
Get A Deposit Or Withdraw Detail – Query details of a specific deposit or withdrawal ￼	/account/v1/deposit-withdraw/detail	Account	https://developer-pro.bitmart.com/en/spot/#get-a-deposit-or-withdraw-detail-keyed
Get Basic Fee Rate – Get current user’s base fee rate ￼	/spot/v1/user_fee	Account	https://developer-pro.bitmart.com/en/spot/#get-basic-fee-rate-keyed
Get Actual Trade Fee Rate – Get actual fee rate for specific trading pair ￼	/spot/v1/trade_fee	Account	https://developer-pro.bitmart.com/en/spot/#get-actual-trade-fee-rate-keyed

Sub-Account Endpoints

Endpoint	Relative Path	API Category	Documentation
Sub-Account to Main-Account (For Main Account) – Transfer assets from a sub-account to the main account (initiated by Main) ￼	/account/sub-account/main/v1/sub-to-main	Sub-Account	https://developer-pro.bitmart.com/en/spot/#sub-account-to-main-account-for-main-account-signed
Sub-Account to Main-Account (For Sub-Account) – Transfer assets from a sub-account to main (initiated by Sub-Account) ￼	/account/sub-account/sub/v1/sub-to-main	Sub-Account	https://developer-pro.bitmart.com/en/spot/#sub-account-to-main-account-for-sub-account-signed
Main-Account to Sub-Account (For Main Account) – Transfer assets from main account to a sub-account ￼	/account/sub-account/main/v1/main-to-sub	Sub-Account	https://developer-pro.bitmart.com/en/spot/#main-account-to-sub-account-for-main-account-signed
Sub-Account to Sub-Account (For Main Account) – Transfer assets between two sub-accounts (initiated by Main) ￼	/account/sub-account/main/v1/sub-to-sub	Sub-Account	https://developer-pro.bitmart.com/en/spot/#sub-account-to-sub-account-for-main-account-signed
Get Sub-Account Transfer History (For Main Account) – History of spot asset transfers by the main account ￼	/account/sub-account/main/v1/transfer-list	Sub-Account	https://developer-pro.bitmart.com/en/spot/#get-sub-account-transfer-history-for-main-account-keyed
Get Account Spot Asset Transfer History (For Main/Sub Account) – Transfer history for a given account (spot assets) ￼	/account/sub-account/v1/transfer-history	Sub-Account	https://developer-pro.bitmart.com/en/spot/#get-account-spot-asset-transfer-history-for-mainsub-account-keyed
Get Sub-Account Spot Wallet Balance (For Main Account) – Query a sub-account’s spot wallet balances (by main) ￼	/account/sub-account/main/v1/wallet	Sub-Account	https://developer-pro.bitmart.com/en/spot/#get-sub-account-spot-wallet-balance-for-main-account-keyed
Get Sub-Account List (For Main Account) – List all sub-accounts under the main account ￼	/account/sub-account/main/v1/subaccount-list	Sub-Account	https://developer-pro.bitmart.com/en/spot/#get-sub-account-list-for-main-account-keyed
Sub-Account to Main-Account (For Main Account) (Futures) – Transfer futures assets from sub-account to main (main initiates) ￼	/account/contract/sub-account/main/v1/sub-to-main	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#sub-account-to-main-account-for-main-account-signed
Main-Account to Sub-Account (For Main Account) (Futures) – Transfer futures assets from main to sub-account ￼	/account/contract/sub-account/main/v1/main-to-sub	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#main-account-to-sub-account-for-main-account-signed
Sub-Account to Main-Account (For Sub-Account) (Futures) – Transfer futures assets from sub-account to main (sub initiates)	/account/contract/sub-account/sub/v1/sub-to-main	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#sub-account-to-main-account-for-sub-account-signed
Get Sub-Account Futures Wallet Balance (For Main Account) – Query a sub-account’s futures wallet balances ￼	/account/contract/sub-account/main/v1/wallet	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#get-sub-account-futures-wallet-balance-for-main-account-keyed
Get Sub-Account Transfer History (For Main Account) (Futures) – History of futures asset transfers by main account ￼	/account/contract/sub-account/main/v1/transfer-list	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#get-sub-account-transfer-history-for-main-account-keyed
Get Account Futures Asset Transfer History (For Main/Sub Account) – Futures asset transfer history for a given account ￼	/account/contract/sub-account/v1/transfer-history	Sub-Account	https://developer-pro.bitmart.com/en/futuresv2/#get-account-futures-asset-transfer-history-for-mainsub-account-keyed

Futures Endpoints

Endpoint	Relative Path	API Category	Documentation
Get Contract Details – Detailed info for a futures contract ￼	/contract/public/details	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-contract-details
Get Market Depth – Full order book depth for a contract ￼	/contract/public/depth	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-market-depth
Get Market Trade – Latest trades for a contract ￼	/contract/public/market-trade	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-market-trade
Get Futures Openinterest – Open interest data for a contract ￼	/contract/public/open-interest	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-futures-openinterest
Get Current Funding Rate – Current funding rate for a contract ￼	/contract/public/funding-rate	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-current-funding-rate
Get K-line – K-line (candlestick) data for a contract ￼	/contract/public/kline	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-k-line
Get MarkPrice K-line – Mark price K-line data for a contract ￼	/contract/public/markprice-kline	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-markprice-k-line
Get Funding Rate History – Historical funding rates for a contract ￼	/contract/public/funding-rate-history	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-funding-rate-history
Get Current Leverage Risk Limit – Current leverage brackets (risk limits) ￼ ￼	/contract/public/leverage-bracket	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-current-leverage-risk-limit
Get Contract Assets (KEYED) – User’s futures account asset details ￼	/contract/private/assets-detail	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-contract-assets-keyed
Get Trade Fee Rate (KEYED) – User’s futures trading fee rate ￼	/contract/private/trade-fee-rate	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-trade-fee-rate-keyed
Get Order Detail (KEYED) – Details of a specific futures order ￼	/contract/private/order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-order-detail-keyed
Get Order History (KEYED) – Historical list of past futures orders ￼	/contract/private/order-history	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-order-history-keyed
Get All Open Orders (KEYED) – All currently open futures orders ￼	/contract/private/get-open-orders	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-all-open-orders-keyed
Get All Current Plan Orders (KEYED) – All open plan orders (stop/trigger orders)	/contract/private/current-plan-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-all-current-plan-orders-keyed
Get Current Position (KEYED) – Current position details for a contract ￼	/contract/private/position	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-current-position-keyed
Get Current Position V2 (KEYED) – Extended position details (v2) ￼	/contract/private/position-v2	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-current-position-v2-keyed
Get Current Position Risk Details (KEYED) – Risk details for current position ￼ ￼	/contract/private/position-risk	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-current-position-risk-details-keyed
Get Order Trade (KEYED) – Trade details of a specific order ￼	/contract/private/trades	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-order-trade-keyed
Get Transaction History (KEYED) – Account transaction history (PNL, funding, etc.) ￼	/contract/private/transaction-history	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-transaction-history-keyed
Get Transfer List (SIGNED) – History of transfers between spot and futures ￼ ￼	/account/v1/transfer-contract-list	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-transfer-list-signed
Transfer (SIGNED) – Transfer assets between spot and futures accounts ￼ ￼	/account/v1/transfer-contract	Futures	https://developer-pro.bitmart.com/en/futuresv2/#transfer-signed
Submit Order (SIGNED) – Place a new futures order ￼	/contract/private/submit-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#submit-order-signed
Modify Limit Order (SIGNED) – Amend an existing limit order’s parameters ￼ ￼	/contract/private/modify-limit-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#modify-limit-order-signed
Cancel Order (SIGNED) – Cancel a specific futures order ￼	/contract/private/cancel-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#cancel-order-signed
Cancel All Orders (SIGNED) – Cancel all open orders for a contract (batch cancel) ￼	/contract/private/cancel-orders	Futures	https://developer-pro.bitmart.com/en/futuresv2/#cancel-all-orders-signed
Timed Cancel All Orders (SIGNED) – Schedule a cancel-all at a specific time ￼	/contract/private/cancel-all-after	Futures	https://developer-pro.bitmart.com/en/futuresv2/#timed-cancel-all-orders-signed
Submit Plan Order (SIGNED) – Place a new plan (trigger) order ￼	/contract/private/submit-plan-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#submit-plan-order-signed
Cancel Plan Order (SIGNED) – Cancel a specific plan order ￼	/contract/private/cancel-plan-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#cancel-plan-order-signed
Submit Leverage (SIGNED) – Adjust leverage for a position ￼	/contract/private/submit-leverage	Futures	https://developer-pro.bitmart.com/en/futuresv2/#submit-leverage-signed
Submit TP/SL Order (SIGNED) – Place a Take-Profit/Stop-Loss order ￼ ￼	/contract/private/submit-tp-sl-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#submit-tp-sl-order-signed
Modify Plan Order (SIGNED) – Modify an existing plan order ￼	/contract/private/modify-plan-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#modify-plan-order-signed
Modify Preset Plan Order (SIGNED) – Modify a preset (untriggered) plan order ￼	/contract/private/modify-preset-plan-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#modify-preset-plan-order-signed
Modify TP/SL Order (SIGNED) – Modify an existing TP/SL order ￼	/contract/private/modify-tp-sl-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#modify-tp-sl-order-signed
Submit Trail Order (SIGNED) – Place a trailing stop order ￼	/contract/private/submit-trail-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#submit-trail-order-signed
Cancel Trail Order (SIGNED) – Cancel a specific trailing stop order ￼	/contract/private/cancel-trail-order	Futures	https://developer-pro.bitmart.com/en/futuresv2/#cancel-trail-order-signed
Set Position Mode (SIGNED) – Set one-way or hedge mode for positions ￼	/contract/private/set-position-mode	Futures	https://developer-pro.bitmart.com/en/futuresv2/#set-position-mode-signed
Get Position Mode (KEYED) – Get current position mode (single or hedge) ￼	/contract/private/get-position-mode	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-position-mode-keyed
Get Futures Rebate List (KEYED) – Affiliate rebate records for futures ￼	/contract/private/affiliate/rebate-list	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-futures-rebate-list-keyed
Get Futures Trade List (KEYED) – Affiliate trade records for futures ￼	/contract/private/affiliate/trade-list	Futures	https://developer-pro.bitmart.com/en/futuresv2/#get-futures-trade-list-keyed

API Broker Endpoints

Endpoint	Relative Path	API Category	Documentation
Get Rebate Records (KEYED) – Query API Broker rebate details ￼	/spot/v1/broker/rebate	API Broker	https://developer-pro.bitmart.com/en/broker/#get-rebate-records-keyed

Each endpoint and its details above are sourced from BitMart’s official API documentation ￼ ￼. You can refer to the “Documentation” links for the official descriptions, parameters, and response formats for each endpoint in the BitMart Developer site ￼ ￼. All endpoints are under the base URL https://api-cloud.bitmart.com (for Spot/Account) or https://api-cloud-v2.bitmart.com (for Futures V2) as noted in the official docs ￼ ￼. The above table covers all REST API endpoints across Spot, Margin, Futures, Account, Sub-Account, Broker, and System categories as per the latest BitMart API reference.