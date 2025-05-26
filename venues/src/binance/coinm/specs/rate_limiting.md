Below is a mental model that most COIN-M-Futures users find helpful when they start bumping into  HTTP 429/418 errors.  (Everything comes straight from the public docs; citations follow each paragraph.)

⸻

1.  Three independent “buckets”

Limiter ( rateLimitType )	What it measures	Default window & cap*	Where you see the meter
raw_request (a.k.a. raw_requests)	Every single REST call, weight = 1, no exceptions	5 min ⋅ 61 000 requests (same number Binance publishes for Spot & Margin; COIN-M docs say the limiter exists but do not publish the figure)  ￼ ￼	No header – you must count client-side
request_weight	The weighted sum shown in each endpoint’s “Request Weight” line	1 min ⋅ 6 000 weight units per-IP  ￼ ￼	X-MBX-USED-WEIGHT-1M (and 3M, 1H, 1D)
orders	Number of orders created / canceled / amended	1 min ⋅ 1 200   •  10 s ⋅ 100 (plus 24 h cap)  ￼ ￼	X-MBX-ORDER-COUNT-10S and -1M

* The published caps are “hard-limits”.  VIP tiers and negotiated uplifts can raise them, while abuse can temporarily lower them.

⸻

2.  How a request is tallied

Example endpoint	Adds to raw_request	Adds to request_weight	Adds to orders
get /dapi/v1/time (weight = 1)  ￼	+1	+1	+0
get /dapi/v1/markPriceKlines?limit=1000 (weight = 10)  ￼	+1	+10	+0
post /dapi/v1/order (place 1 order, weight = 1)	+1	+1	+1
post /dapi/v1/batchOrders with 5 orders (weight = 5)  ￼	+1	+5	+5
delete /dapi/v1/order (cancel)	+1	usually +1	+1

Key takeaway: most order-related endpoints hit all three buckets; market-data and utility endpoints hit only the first two.

⸻

3.  Reading—and trusting—the server headers
	•	Request-weight headers (X-MBX-USED-WEIGHT-1M, etc.) and order-count headers (X-MBX-ORDER-COUNT-10S/1M) are authoritative – reset your own counters to them whenever you receive a response.  ￼
	•	There is no header for raw_request, so you must maintain that counter locally (or just assume each request adds 1 and stop well before ~61 k/5 min).

⸻

4.  Choosing which limiter to watch first
	1.	Trading bots usually hit the 10 s / 1 min order caps first.
	•	Tip: batch endpoints let you send up to five orders in a single request, which saves request-weight and raw-requests but not order quota.
	2.	Data scrapers trip the request-weight cap; switch heavy polling to WebSockets or spread requests across multiple IPs.
	3.	Stress / latency tests are the typical way to reach the rarely-seen raw-request ceiling.

⸻

5.  What happens when you go over
	•	Exceed any of the three → immediate 429.
	•	Continue without backing off → temporary 418 (IP ban escalates from 2 minutes to 3 days).  ￼
	•	The response may include Retry-After (weight/order) but never for raw_request, because the server does not track that per-response.

⸻

6.  Practical checklist for a COIN-M client
	1.	Parse the “Request Weight” line for every endpoint you call and hard-code it.
	2.	Wrap HTTP calls with a small middleware that:
	•	reads the two X-MBX-ORDER-COUNT-* headers and updates per-UID counters,
	•	reads X-MBX-USED-WEIGHT-* and updates per-IP counters,
	•	increments a local raw_request counter, and
	•	sleeps/retries when any counter is about to exceed its cap.
	3.	For market data, switch to the COIN-M WebSocket streams wherever possible—they do not count toward any REST limiter.  ￼

Put these guards in place once and you will almost never see another unexpected 429/418, even during volatile markets.

⸻

Cited sources
	•	COIN-M General-Info (limits section, lists RAW_REQUEST/REQUEST_WEIGHT/ORDER, 429 & 418 behaviour)  ￼
	•	COIN-M Common-Definition (6000 request-weight / 1200 orders caps)  ￼
	•	Spot ENUM example showing RAW_REQUESTS object with 61 000/5 min (same numbers apply)  ￼
	•	Endpoint docs (/dapi/v1/time, /dapi/v1/markPriceKlines) for weight examples  ￼ ￼
	•	POST /dapi/v1/batchOrders showing mixed weight vs. order counting  ￼