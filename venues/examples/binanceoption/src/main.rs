use clap::{Arg, Command};
use anyhow::Result;
use venues::binance::option::{
    PublicRestClient, 
    Filter, 
    PriceFilter, 
    LotSizeFilter
};

/// Example demonstrating Binance Options trading rules
#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("binanceoption")
        .about("Binance Options Trading Rules Example")
        .arg(
            Arg::new("testnet")
                .long("testnet")
                .help("Use testnet instead of mainnet")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("exchange-info")
                .about("Get exchange information and trading rules")
        )
        .get_matches();

    let client = if matches.get_flag("testnet") {
        PublicRestClient::new_testnet()
    } else {
        PublicRestClient::new()
    };

    match matches.subcommand() {
        Some(("exchange-info", _)) => {
            handle_exchange_info(&client).await?;
        }
        _ => {
            println!("Use --help for available commands");
        }
    }

    Ok(())
}

async fn handle_exchange_info(client: &PublicRestClient) -> Result<()> {
    println!("Fetching Binance Options exchange information...");
    
    let response = client.get_exchange_info().await
        .map_err(|e| anyhow::anyhow!("Failed to get exchange info: {}", e))?;

    println!("Exchange timezone: {}", response.data.timezone);
    println!("Number of rate limits: {}", response.data.rate_limits.len());
    println!("Number of option symbols: {}", response.data.symbols.len());

    if !response.data.symbols.is_empty() {
        println!("\nSample Option Symbol Trading Rules:");
        let symbol = &response.data.symbols[0];
        
        println!("Symbol: {}", symbol.symbol);
        println!("Underlying: {}", symbol.underlying);
        println!("Option Type: {:?}", symbol.option_type);
        println!("Strike Price: {}", symbol.strike_price);
        println!("Status: {:?}", symbol.status);
        
        println!("\nTrading Filters:");
        for filter in &symbol.filters {
            match filter {
                Filter::PriceFilter(price_filter) => {
                    print_price_filter_rules(price_filter);
                }
                Filter::LotSizeFilter(lot_filter) => {
                    print_lot_size_filter_rules(lot_filter);
                }
                Filter::Unknown => {
                    println!("  - Unknown filter type");
                }
            }
        }
    }

    println!("\nRequest completed in: {:?}", response.request_duration);

    Ok(())
}

fn print_price_filter_rules(filter: &PriceFilter) {
    println!("  PRICE_FILTER:");
    
    if let Some(min_price) = &filter.min_price {
        if min_price != "0" {
            println!("    - Minimum price: {}", min_price);
        } else {
            println!("    - Minimum price: DISABLED (value is 0)");
        }
    }
    
    if let Some(max_price) = &filter.max_price {
        if max_price != "0" {
            println!("    - Maximum price: {}", max_price);
        } else {
            println!("    - Maximum price: DISABLED (value is 0)");
        }
    }
    
    if let Some(tick_size) = &filter.tick_size {
        if tick_size != "0" {
            println!("    - Price tick size: {}", tick_size);
        } else {
            println!("    - Price tick size: DISABLED (value is 0)");
        }
    }
}

fn print_lot_size_filter_rules(filter: &LotSizeFilter) {
    println!("  LOT_SIZE:");
    
    if let Some(min_qty) = &filter.min_qty {
        if min_qty != "0" {
            println!("    - Minimum quantity: {}", min_qty);
        } else {
            println!("    - Minimum quantity: DISABLED (value is 0)");
        }
    }
    
    if let Some(max_qty) = &filter.max_qty {
        if max_qty != "0" {
            println!("    - Maximum quantity: {}", max_qty);
        } else {
            println!("    - Maximum quantity: DISABLED (value is 0)");
        }
    }
    
    if let Some(step_size) = &filter.step_size {
        if step_size != "0" {
            println!("    - Quantity step size: {}", step_size);
        } else {
            println!("    - Quantity step size: DISABLED (value is 0)");
        }
    }
}