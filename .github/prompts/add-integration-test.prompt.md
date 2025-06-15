---
mode: 'agent'
tools: ['githubRepo', 'codebase']
description: 'Add a new integration test for a stable coin buy order flow in the venues crate (Rust)'
---

Write a rust integration tests for ${venue} that

1. Get the instruments
2. Finds a stable coin pair
3. Gets the current balance
4. Get the current market price
5. Places a buy order at the ask price.
6. Get the order status.
7. If it hasn't executed, replace the order (using amend or cancel replace) with a higher price.
8. Get the updated order status.
9. Keep iternating until the order is filled or a maximum number of iterations is reached.
10. Run the get historical trades endpoint to verify the order was filled.

---

**Prompt variables:**
- `${venue}`: The trading venue to test against (e.g., binance, binancecoinm)

---