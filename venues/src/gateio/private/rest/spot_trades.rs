//! Trading statistics and trade history functionality
#![allow(clippy::float_arithmetic)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::arithmetic_side_effects)]

use serde::{Deserialize, Serialize};

use super::RestClient;

/// Request parameters for getting personal trading history
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyTradesRequest {
    /// Currency pair
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_pair: Option<String>,
    /// Limit the number of records
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Start timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// End timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

/// Personal trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyTrade {
    /// Trade ID
    pub id: String,
    /// Trading time
    pub create_time: String,
    /// Trading time in milliseconds
    pub create_time_ms: String,
    /// Currency pair
    pub currency_pair: String,
    /// Order ID
    pub order_id: String,
    /// Trade side
    pub side: String,
    /// Trade role (taker/maker)
    pub role: String,
    /// Trade amount
    pub amount: String,
    /// Trade price
    pub price: String,
    /// Trade fee
    pub fee: String,
    /// Fee currency
    pub fee_currency: String,
    /// Point fee
    pub point_fee: String,
    /// GT fee
    pub gt_fee: String,
    /// Whether GT fee is used
    pub gt_fee_deduction: bool,
    /// Rebated fee
    pub rebated_fee: String,
    /// Rebated fee currency
    pub rebated_fee_currency: String,
    /// Text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl MyTrade {
    /// Calculate the total value of the trade (amount * price)
    pub fn total_value(&self) -> Result<f64, std::num::ParseFloatError> {
        let amount: f64 = self.amount.parse()?;
        let price: f64 = self.price.parse()?;
        Ok(amount * price)
    }

    /// Check if this trade was a maker trade
    pub fn is_maker(&self) -> bool {
        self.role == "maker"
    }

    /// Check if this trade was a taker trade
    pub fn is_taker(&self) -> bool {
        self.role == "taker"
    }

    /// Check if this was a buy trade
    pub fn is_buy(&self) -> bool {
        self.side == "buy"
    }

    /// Check if this was a sell trade
    pub fn is_sell(&self) -> bool {
        self.side == "sell"
    }

    /// Get the timestamp as Unix timestamp in seconds
    pub fn timestamp(&self) -> Result<i64, std::num::ParseIntError> {
        self.create_time.parse()
    }

    /// Get the timestamp in milliseconds
    pub fn timestamp_ms(&self) -> Result<i64, std::num::ParseIntError> {
        self.create_time_ms.parse()
    }
}

/// Implementation for the client
impl RestClient {
    /// Get personal trading history
    /// 
    /// This endpoint returns your personal trading history.
    /// You can filter by currency pair, time range, and other parameters.
    pub async fn get_my_trades(&self, request: GetMyTradesRequest) -> crate::gateio::Result<Vec<MyTrade>> {
        self.get_with_query("/spot/my_trades", &request).await
    }

    /// Get all personal trades for a currency pair
    pub async fn get_my_trades_for_pair(
        &self,
        currency_pair: &str,
        limit: Option<u32>,
    ) -> crate::gateio::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades for a specific order
    pub async fn get_order_trades(
        &self,
        order_id: &str,
        currency_pair: &str,
    ) -> crate::gateio::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: Some(currency_pair.to_string()),
            order_id: Some(order_id.to_string()),
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get trades within a time range
    pub async fn get_my_trades_in_range(
        &self,
        currency_pair: Option<&str>,
        from: i64,
        to: i64,
        limit: Option<u32>,
    ) -> crate::gateio::Result<Vec<MyTrade>> {
        let request = GetMyTradesRequest {
            currency_pair: currency_pair.map(|s| s.to_string()),
            from: Some(from),
            to: Some(to),
            limit,
            ..Default::default()
        };
        self.get_my_trades(request).await
    }

    /// Get recent trades (last 24 hours)
    pub async fn get_recent_my_trades(
        &self,
        currency_pair: Option<&str>,
        limit: Option<u32>,
    ) -> crate::gateio::Result<Vec<MyTrade>> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let yesterday = now - 86400; // 24 hours ago

        self.get_my_trades_in_range(currency_pair, yesterday, now, limit).await
    }

    /// Get trading statistics for a currency pair within a time range
    pub async fn get_trading_stats(
        &self,
        currency_pair: &str,
        from: i64,
        to: i64,
    ) -> crate::gateio::Result<TradingStats> {
        let trades = self.get_my_trades_in_range(Some(currency_pair), from, to, None).await?;
        
        let mut stats = TradingStats::new();
        
        for trade in trades {
            if let Ok(value) = trade.total_value() {
                stats.total_volume += value;
                stats.trade_count += 1;
                
                if trade.is_buy() {
                    stats.buy_volume += value;
                    stats.buy_count += 1;
                } else {
                    stats.sell_volume += value;
                    stats.sell_count += 1;
                }
                
                if trade.is_maker() {
                    stats.maker_volume += value;
                    stats.maker_count += 1;
                } else {
                    stats.taker_volume += value;
                    stats.taker_count += 1;
                }
            }
        }
        
        Ok(stats)
    }
}

/// Trading statistics summary
#[derive(Debug, Clone, Default)]
pub struct TradingStats {
    /// Total trading volume
    pub total_volume: f64,
    /// Total number of trades
    pub trade_count: u32,
    /// Buy volume
    pub buy_volume: f64,
    /// Number of buy trades
    pub buy_count: u32,
    /// Sell volume
    pub sell_volume: f64,
    /// Number of sell trades
    pub sell_count: u32,
    /// Maker volume
    pub maker_volume: f64,
    /// Number of maker trades
    pub maker_count: u32,
    /// Taker volume
    pub taker_volume: f64,
    /// Number of taker trades
    pub taker_count: u32,
}

impl TradingStats {
    fn new() -> Self {
        Self::default()
    }

    /// Calculate average trade size
    pub fn average_trade_size(&self) -> f64 {
        if self.trade_count > 0 {
            self.total_volume / self.trade_count as f64
        } else {
            0.0
        }
    }

    /// Calculate maker ratio (percentage of maker trades)
    pub fn maker_ratio(&self) -> f64 {
        if self.trade_count > 0 {
            (self.maker_count as f64 / self.trade_count as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate buy ratio (percentage of buy trades)
    pub fn buy_ratio(&self) -> f64 {
        if self.trade_count > 0 {
            (self.buy_count as f64 / self.trade_count as f64) * 100.0
        } else {
            0.0
        }
    }
}