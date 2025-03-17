use std::time::SystemTime;
use crate::metrics::VenueMetrics;
use orderbook::aggregated::AggregatedOrderBook;

pub fn print_metrics_table(metrics: &[(&str, &VenueMetrics)]) {
    // Calculate column widths
    let venue_width = 15;
    let updates_width = 12;
    let reconnects_width = 10;
    let last_latency_width = 12;
    let avg_latency_width = 12;
    let best_bid_width = 12;
    let best_ask_width = 12;
    let last_update_width = 15;
    let total_width = venue_width + updates_width + reconnects_width + last_latency_width + 
                     avg_latency_width + best_bid_width + best_ask_width + last_update_width + 16; // +16 for separators

    // Print header
    println!("\n{}", "=".repeat(total_width));
    println!("{:^total_width$}", "Exchange Metrics");
    println!("{}", "=".repeat(total_width));
    
    // Print column headers
    println!("{:<venue_width$} | {:>updates_width$} | {:>reconnects_width$} | {:>last_latency_width$} | {:>avg_latency_width$} | {:>best_bid_width$} | {:>best_ask_width$} | {:>last_update_width$}",
        "Venue", "Updates", "Reconnects", "Last Lat", "Avg Lat", "Best Bid", "Best Ask", "Age");
    println!("{}", "-".repeat(total_width));

    // Print each venue's metrics
    let now = SystemTime::now();
    for (venue, metric) in metrics {
        let age = now.duration_since(metric.last_update_time)
            .map(|d| {
                if d.as_secs() >= 60 {
                    format!("{:.1}m", d.as_secs_f64() / 60.0)
                } else if d.as_secs() > 0 {
                    format!("{:.1}s", d.as_secs_f64())
                } else {
                    format!("{}ms", d.as_millis())
                }
            })
            .unwrap_or_else(|_| "N/A".to_string());

        // Format the bid and ask with colors
        let colored_bid = format!("\x1b[32m{:.2}\x1b[0m", metric.best_bid);  // Green for bids
        let colored_ask = format!("\x1b[31m{:.2}\x1b[0m", metric.best_ask);  // Red for asks

        println!("{:<venue_width$} | {:>updates_width$} | {:>reconnects_width$} | {:>last_latency_width$} | {:>avg_latency_width$} | {:>best_bid_width$} | {:>best_ask_width$} | {:>last_update_width$}",
            venue,
            metric.updates_processed,
            metric.reconnects,
            format!("{}ms", metric.last_update_latency_ms),
            format!("{:.2}ms", metric.avg_update_latency_ms),
            colored_bid,
            colored_ask,
            age);
    }

    println!("{}", "=".repeat(total_width));
    println!(); // Add a blank line after the table
}

pub fn print_aggregated_orderbook(orderbook: &AggregatedOrderBook, depth: usize) {
    // Get top levels for display
    let (bids, asks) = orderbook.get_depth(depth);
    
    // Calculate column widths
    let price_width = 12;
    let size_width = 12;
    let sources_width = 40;
    let total_width = price_width + size_width + sources_width + 4; // +4 for separators
    
    // Print header
    println!("\n{}", "=".repeat(total_width));
    println!("{:^total_width$}", "Aggregated Orderbook");
    println!("{}", "=".repeat(total_width));
    println!("{:<price_width$} | {:<size_width$} | {:<sources_width$}", 
        "Price", "Size", "Sources");
    println!("{}", "-".repeat(total_width));
    
    // Print asks in reverse order (highest to lowest)
    let mut asks_reversed: Vec<_> = asks.into_iter().collect();
    asks_reversed.reverse();
    for (level, price) in asks_reversed {
        let sources = level.sources.iter()
            .map(|(source, size)| format!("{}: {:.3}", source, size))
            .collect::<Vec<_>>()
            .join(", ");
        println!("\x1b[31m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
            price, level.size, sources);
    }
    
    // Print spread if available
    if let Some((best_bid_price, best_ask_price)) = orderbook.best_bid_ask_prices() {
        let spread = best_ask_price - best_bid_price;
        let spread_pct = (spread / best_bid_price) * 100.0;
        println!("{}", "-".repeat(total_width));
        println!("\x1b[33m{:<total_width$}\x1b[0m",
            format!("Spread: {:.8} ({:.4}%)", spread, spread_pct));
        println!("{}", "-".repeat(total_width));
    }
    
    // Print bids
    for (level, price) in bids {
        let sources = level.sources.iter()
            .map(|(source, size)| format!("{}: {:.3}", source, size))
            .collect::<Vec<_>>()
            .join(", ");
        println!("\x1b[32m{:<price_width$.8} | {:<size_width$.3} | {:<sources_width$}\x1b[0m",
            price, level.size, sources);
    }
    
    println!("{}", "=".repeat(total_width));
} 