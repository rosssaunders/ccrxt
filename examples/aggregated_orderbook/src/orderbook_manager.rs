use std::collections::HashMap;
use std::error::Error;
use orderbook::OrderBook;
use orderbook::aggregated::AggregatedOrderBook;
use venues::price_feed::UsdConverter;
use venues::venue::{BinanceSpot, Okx, BybitSpot};
use crate::metrics::VenueMetrics;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum VenueType {
    BinanceSpot,
    OKX,
    BybitSpot,
}

impl std::fmt::Display for VenueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VenueType::BinanceSpot => write!(f, "Binance Spot"),
            VenueType::OKX => write!(f, "OKX"),
            VenueType::BybitSpot => write!(f, "Bybit Spot"),
        }
    }
}

pub struct OrderBookManager {
    pub orderbooks: HashMap<VenueType, OrderBook>,
    metrics: HashMap<VenueType, VenueMetrics>,
    aggregated_ob: AggregatedOrderBook,
    usd_converter: UsdConverter,
}

impl OrderBookManager {
    pub fn new(price_precision: u32) -> Self {
        Self {
            orderbooks: HashMap::new(),
            metrics: HashMap::new(),
            aggregated_ob: AggregatedOrderBook::new(price_precision),
            usd_converter: UsdConverter::new(std::time::Duration::from_secs(60)),
        }
    }

    pub fn add_venue(&mut self, venue: VenueType, price_precision: u32) {
        self.orderbooks.insert(venue.clone(), OrderBook::new(price_precision));
        self.metrics.insert(venue, VenueMetrics::default());
    }

    pub async fn initialize_snapshots(
        &mut self,
        spot_rest: &venues::binance::spot::BinanceSpotPublicRest,
        okx_rest: &venues::okx::OkxPublicRest,
        bybit_spot_rest: &venues::bybit::spot::BybitSpotPublicRest,
        spot_symbol: &str,
        okx_symbol: &str,
        bybit_spot_symbol: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Initialize venues
        self.add_venue(VenueType::BinanceSpot, 8);
        self.add_venue(VenueType::OKX, 8);
        self.add_venue(VenueType::BybitSpot, 8);

        // Get initial snapshots
        let spot_snapshot = spot_rest.get_orderbook_snapshot(spot_symbol, Some(1000)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        let okx_snapshot = okx_rest.get_orderbook_snapshot(okx_symbol, Some(400)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        let bybit_spot_snapshot = bybit_spot_rest.get_orderbook_snapshot(bybit_spot_symbol, Some(50)).await
            .map_err(|e| Box::<dyn Error + Send + Sync>::from(e.to_string()))?;
        
        // Process and apply snapshots
        if let Some(spot_ob) = self.orderbooks.get_mut(&VenueType::BinanceSpot) {
            let spot_bids: Vec<(f64, f64)> = spot_snapshot.bids.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            let spot_asks: Vec<(f64, f64)> = spot_snapshot.asks.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            spot_ob.apply_snapshot(spot_bids, spot_asks);
        }

        if let Some(okx_ob) = self.orderbooks.get_mut(&VenueType::OKX) {
            let okx_bids: Vec<(f64, f64)> = okx_snapshot.bids.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            let okx_asks: Vec<(f64, f64)> = okx_snapshot.asks.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            okx_ob.apply_snapshot(okx_bids, okx_asks);
        }

        if let Some(bybit_spot_ob) = self.orderbooks.get_mut(&VenueType::BybitSpot) {
            let bybit_spot_bids: Vec<(f64, f64)> = bybit_spot_snapshot.result.b.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            let bybit_spot_asks: Vec<(f64, f64)> = bybit_spot_snapshot.result.a.iter()
                .filter_map(|level| {
                    let price = level.0.parse::<f64>().ok()?;
                    let size = level.1.parse::<f64>().ok()?;
                    Some((price, size))
                })
                .collect();
            
            bybit_spot_ob.apply_snapshot(bybit_spot_bids, bybit_spot_asks);
        }
        
        Ok(())
    }

    pub async fn update_aggregated_orderbook(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Clear all venues from the aggregated orderbook
        for venue in self.orderbooks.keys() {
            match venue {
                VenueType::BinanceSpot => self.aggregated_ob.clear_venue(&BinanceSpot::Spot.to_string()),
                VenueType::OKX => self.aggregated_ob.clear_venue(&Okx::Spot.to_string()),
                VenueType::BybitSpot => self.aggregated_ob.clear_venue(&BybitSpot::Spot.to_string()),
            }
        }
        
        // Update aggregated orderbook with each venue's data
        for (venue, orderbook) in &self.orderbooks {
            let (bids, asks) = orderbook.get_depth_with_prices(50);
            
            // Add bids
            for (price, bid) in bids {
                match venue {
                    VenueType::BinanceSpot => self.aggregated_ob.update(price, bid.size, true, &BinanceSpot::Spot),
                    VenueType::OKX => self.aggregated_ob.update(price, bid.size, true, &Okx::Spot),
                    VenueType::BybitSpot => self.aggregated_ob.update(price, bid.size, true, &BybitSpot::Spot),
                }
            }
            
            // Add asks
            for (price, ask) in asks {
                match venue {
                    VenueType::BinanceSpot => self.aggregated_ob.update(price, ask.size, false, &BinanceSpot::Spot),
                    VenueType::OKX => self.aggregated_ob.update(price, ask.size, false, &Okx::Spot),
                    VenueType::BybitSpot => self.aggregated_ob.update(price, ask.size, false, &BybitSpot::Spot),
                }
            }
        }
        
        Ok(())
    }

    pub fn update_orderbook(&mut self, venue: &VenueType, price: f64, size: f64, is_bid: bool) {
        if let Some(orderbook) = self.orderbooks.get_mut(venue) {
            orderbook.update(price, size, is_bid);
            
            // Update aggregated orderbook immediately
            match venue {
                VenueType::BinanceSpot => self.aggregated_ob.update(price, size, is_bid, &BinanceSpot::Spot),
                VenueType::OKX => self.aggregated_ob.update(price, size, is_bid, &Okx::Spot),
                VenueType::BybitSpot => self.aggregated_ob.update(price, size, is_bid, &BybitSpot::Spot),
            }
        }
    }

    pub fn update_metrics(&mut self, venue: &VenueType, latency_ms: u64, best_bid: f64, best_ask: f64) {
        if let Some(metrics) = self.metrics.get_mut(venue) {
            metrics.update_latency(latency_ms);
            metrics.update_prices(best_bid, best_ask);
        }
    }

    pub fn get_metrics(&self) -> &HashMap<VenueType, VenueMetrics> {
        &self.metrics
    }

    pub fn get_aggregated_orderbook(&self) -> &AggregatedOrderBook {
        &self.aggregated_ob
    }
} 