# Binance API Endpoints Reference

This document provides a complete overview of **Binance API base URLs**, including **production**, **testnet**, and **WebSocket** endpoints.  
Useful for configuring multi-exchange or cross-product trading systems.

---

## 🌍 Binance Global – Production

| Service / Product                 | Base URL                                                   | Path Prefix Examples              |
| --------------------------------- | ---------------------------------------------------------- | --------------------------------- |
| **Spot / Wallet / Margin (SAPI)** | `https://api.binance.com` <br> (also `api1-4.binance.com`) | `/api/v3/...` <br> `/sapi/v1/...` |
| **USDT-M Futures (FAPI)**         | `https://fapi.binance.com`                                 | `/fapi/v1/...`                    |
| **Coin-M Futures (DAPI)**         | `https://dapi.binance.com`                                 | `/dapi/v1/...`                    |
| **Portfolio Margin (PAPI)**       | `https://papi.binance.com`                                 | `/papi/v1/...`                    |
| **Options (EAPI)**                | `https://eapi.binance.com`                                 | `/eapi/v1/...`                    |

---

## 🧪 Binance Global – Testnet

| Service / Product                   | Base URL                            | Notes                           |
| ----------------------------------- | ----------------------------------- | ------------------------------- |
| **Spot Testnet**                    | `https://testnet.binance.vision`    | For spot trading simulation.    |
| **USDT-M Futures Testnet**          | `https://testnet.binancefuture.com` | Use `/fapi/...` paths.          |
| **Coin-M Futures Testnet**          | `https://testnet.binancefuture.com` | Use `/dapi/...` paths.          |
| **Options (EAPI) Testnet**          | `https://testnet.binanceops.com`    | For options testing.            |
| **Portfolio Margin (PAPI) Testnet** | Not generally public                | Usually by request/invite only. |

---

## 🇺🇸 Binance US – Production

| Service / Product        | Base URL                 | Path Prefix Examples              |
| ------------------------ | ------------------------ | --------------------------------- |
| **Spot / Wallet (SAPI)** | `https://api.binance.us` | `/api/v3/...` <br> `/sapi/v1/...` |
| **Futures**              | ❌ Not available yet     | —                                 |

> Binance.US currently has no official futures/options testnet.

---

## 🔒 WebSocket Endpoints

| Service / Product              | Base URL                             |
| ------------------------------ | ------------------------------------ |
| **Spot WS**                    | `wss://stream.binance.com:9443/ws`   |
| **USDT-M Futures WS**          | `wss://fstream.binance.com/ws`       |
| **Coin-M Futures WS**          | `wss://dstream.binance.com/ws`       |
| **Options (EAPI) WS**          | `wss://eapi.binance.com/ws`          |
| **Portfolio Margin (PAPI) WS** | `wss://papi.binance.com/ws`          |
| **Spot Testnet WS**            | `wss://testnet.binance.vision/ws`    |
| **USDT-M Futures Testnet WS**  | `wss://stream.binancefuture.com/ws`  |
| **Coin-M Futures Testnet WS**  | `wss://dstream.binancefuture.com/ws` |
| **Options Testnet WS**         | `wss://testnetws.binanceops.com/ws`  |

---

## ✅ Summary

- **Different product families use different subdomains** (not just path prefixes).
- Path prefixes (`/sapi`, `/fapi`, `/dapi`, `/papi`, `/eapi`) mirror the subdomain.
- **Testnet domains differ** from production — you must swap both base URL and path.
- **Portfolio Margin (PAPI) testnet** requires special access.

---

📌 Keep this handy as a config reference when integrating with multiple Binance product lines!
