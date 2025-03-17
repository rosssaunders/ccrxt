use futures::StreamExt;
use std::time::{Duration, Instant};
use std::error::Error;
use venues::binance::{
    spot::{BinanceSpotPublicWebSocket, BinanceSpotPublicRest, WebSocketMessage as SpotWebSocketMessage},
};
use venues::okx::{OkxPublicWebSocket, OkxPublicRest, WebSocketMessage as OkxWebSocketMessage};
use venues::bybit::{
    spot::{BybitSpotPublicWebSocket, BybitSpotPublicRest, WebSocketMessage as BybitSpotMessage},
};
use venues::websockets::WebSocketConnection;
use aggregated_orderbook::orderbook_manager::{OrderBookManager, VenueType};
use aggregated_orderbook::display::{print_metrics_table, print_aggregated_orderbook};

const PRICE_PRECISION: u32 = 8;
const INITIAL_BACKOFF_MS: u64 = 1000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let spot_rest = BinanceSpotPublicRest::new();
    let okx_rest = OkxPublicRest::new();
    let bybit_spot_rest = BybitSpotPublicRest::new();
    
    let spot_symbol = "BTCUSDT";
    let okx_symbol = "BTC-USDT";
    let bybit_spot_symbol = "BTCUSDT";
    
    let mut orderbook_manager = OrderBookManager::new(PRICE_PRECISION);
    
    // Initialize snapshots
    orderbook_manager.initialize_snapshots(
        &spot_rest,
        &okx_rest,
        &bybit_spot_rest,
        spot_symbol,
        okx_symbol,
        bybit_spot_symbol,
    ).await?;
    
    // Initialize WebSocket clients
    let mut spot_ws = BinanceSpotPublicWebSocket::new();
    let mut okx_ws = OkxPublicWebSocket::new();
    let mut bybit_spot_ws = BybitSpotPublicWebSocket::new();
    
    // Connect to WebSockets
    spot_ws.connect().await?;
    okx_ws.connect().await?;
    bybit_spot_ws.connect().await?;
    
    // Subscribe to orderbook channels
    let spot_channel = format!("{}@depth@100ms", spot_symbol.to_lowercase());
    let okx_channel = format!("books:{}", okx_symbol);
    let bybit_spot_channel = format!("orderbook.50.{}", bybit_spot_symbol);
    
    spot_ws.subscribe(vec![spot_channel.clone()]).await?;
    okx_ws.subscribe(vec![okx_channel.clone()]).await?;
    bybit_spot_ws.subscribe(vec![bybit_spot_channel.clone()]).await?;
    
    // Get message streams
    let mut spot_stream = spot_ws.message_stream();
    let mut okx_stream = okx_ws.message_stream();
    let mut bybit_spot_stream = bybit_spot_ws.message_stream();
    
    // Process updates
    let mut last_print_time = Instant::now();
    
    loop {
        tokio::select! {
            Some(spot_result) = spot_stream.next() => {
                match spot_result {
                    Ok(SpotWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.bids {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BinanceSpot, price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.asks {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BinanceSpot, price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::BinanceSpot)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::BinanceSpot, latency, best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Spot message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(okx_result) = okx_stream.next() => {
                match okx_result {
                    Ok(OkxWebSocketMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        for data in &update.data {
                            // Process bids
                            for bid in &data.bids {
                                if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                    orderbook_manager.update_orderbook(&VenueType::OKX, price, size, true);
                                }
                            }
                            
                            // Process asks
                            for ask in &data.asks {
                                if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                    orderbook_manager.update_orderbook(&VenueType::OKX, price, size, false);
                                }
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::OKX)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::OKX, latency, best_bid, best_ask);
                        }
                    },
                    Ok(OkxWebSocketMessage::Response(response)) => {
                        println!("Received OKX response: {:?}", response);
                    },
                    Ok(OkxWebSocketMessage::Raw(value)) => {
                        // Remove the debug print statement
                    },
                    Err(e) => {
                        eprintln!("Error processing OKX message: {}", e);
                        return Err(e);
                    }
                }
            },
            Some(bybit_spot_result) = bybit_spot_stream.next() => {
                match bybit_spot_result {
                    Ok(BybitSpotMessage::OrderBook(update)) => {
                        let start_time = Instant::now();
                        
                        // Process bids
                        for bid in &update.data.b {
                            if let (Ok(price), Ok(size)) = (bid.0.parse::<f64>(), bid.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BybitSpot, price, size, true);
                            }
                        }
                        
                        // Process asks
                        for ask in &update.data.a {
                            if let (Ok(price), Ok(size)) = (ask.0.parse::<f64>(), ask.1.parse::<f64>()) {
                                orderbook_manager.update_orderbook(&VenueType::BybitSpot, price, size, false);
                            }
                        }
                        
                        // Update metrics
                        let latency = start_time.elapsed().as_millis() as u64;
                        if let Some((best_bid, best_ask)) = orderbook_manager.orderbooks.get(&VenueType::BybitSpot)
                            .and_then(|ob| ob.best_bid_ask_prices()) {
                            orderbook_manager.update_metrics(&VenueType::BybitSpot, latency, best_bid, best_ask);
                        }
                    },
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error processing Bybit Spot message: {}", e);
                        return Err(Box::<dyn Error + Send + Sync>::from(e.to_string()));
                    }
                }
            },
            else => break,
        }
        
        // Update aggregated orderbook and print stats every second
        if last_print_time.elapsed() >= Duration::from_secs(1) {
            orderbook_manager.update_aggregated_orderbook().await?;
            
            // Clear screen
            print!("\x1B[2J\x1B[1;1H");
            
            // Print metrics and orderbook
            let metrics_data: Vec<_> = orderbook_manager.get_metrics().iter()
                .map(|(venue, metrics)| (venue.to_string(), metrics))
                .collect();
            let metrics_refs: Vec<_> = metrics_data.iter()
                .map(|(s, m)| (s.as_str(), *m))
                .collect();
            print_metrics_table(&metrics_refs);
            print_aggregated_orderbook(orderbook_manager.get_aggregated_orderbook(), 10);
            
            last_print_time = Instant::now();
        }
    }
    
    Ok(())
} 