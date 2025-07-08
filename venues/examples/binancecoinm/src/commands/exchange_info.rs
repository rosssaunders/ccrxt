use std::sync::Arc;

use anyhow::Result;
use tabled::{settings::Style, Table, Tabled};
use venues::binance::coinm::{ExchangeInfoResponse, Filter, PublicRestClient};

/// Handles the `exchange-info` command: fetches and prints exchange information.
pub async fn handle_exchange_info_command(client: Arc<PublicRestClient>) -> Result<()> {
    let response = client.get_exchange_info().await?;
    print_exchange_info(&response.data);
    Ok(())
}

#[derive(Tabled)]
struct RateLimitRow {
    #[tabled(rename = "Type")]
    rate_limit_type: String,
    #[tabled(rename = "Interval")]
    interval: String,
    #[tabled(rename = "Interval Num")]
    interval_num: u32,
    #[tabled(rename = "Limit")]
    limit: u32,
}

#[derive(Tabled)]
struct SymbolRow {
    #[tabled(rename = "Symbol")]
    symbol: String,
    #[tabled(rename = "Pair")]
    pair: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Contract Type")]
    contract_type: String,
}

fn print_exchange_info(info: &ExchangeInfoResponse) {
    println!("Exchange Timezone: {}", info.timezone);
    println!("Symbols:");

    let mut symbol_rows: Vec<SymbolRow> = info
        .symbols
        .iter()
        .map(|symbol| SymbolRow {
            symbol: symbol.symbol.clone(),
            pair: symbol.pair.clone(),
            status: format!("{:?}", symbol.contract_status),
            contract_type: format!("{:?}", symbol.contract_type),
        })
        .collect();

    //sort symbol_rows by contract type and then pair and then symbol
    symbol_rows.sort_by(|a, b| {
        (a.contract_type.as_str(), a.pair.as_str(), a.symbol.as_str()).cmp(&(
            b.contract_type.as_str(),
            b.pair.as_str(),
            b.symbol.as_str(),
        ))
    });

    let mut symbol_table = Table::new(symbol_rows);
    symbol_table.with(Style::rounded());

    println!("{symbol_table}");
    println!("\nRate Limits:");

    let rows: Vec<RateLimitRow> = info
        .rate_limits
        .iter()
        .map(|rl| RateLimitRow {
            rate_limit_type: format!("{:?}", rl.rate_limit_type),
            interval: format!("{:?}", rl.interval),
            interval_num: rl.interval_num,
            limit: rl.limit,
        })
        .collect();

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");

    println!("\nFilters:");

    // PriceFilter Table
    #[derive(Tabled)]
    struct PriceFilterRow {
        #[tabled(rename = "Symbol")]
        symbol: String,
        #[tabled(rename = "Min Price")]
        min_price: String,
        #[tabled(rename = "Max Price")]
        max_price: String,
        #[tabled(rename = "Tick Size")]
        tick_size: String,
    }
    let mut price_filter_rows = Vec::new();

    // LotSizeFilter Table
    #[derive(Tabled)]
    struct LotSizeFilterRow {
        #[tabled(rename = "Symbol")]
        symbol: String,
        #[tabled(rename = "Min Qty")]
        min_qty: String,
        #[tabled(rename = "Max Qty")]
        max_qty: String,
        #[tabled(rename = "Step Size")]
        step_size: String,
    }
    let mut lot_size_filter_rows = Vec::new();

    // MarketLotSizeFilter Table
    #[derive(Tabled)]
    struct MarketLotSizeFilterRow {
        #[tabled(rename = "Symbol")]
        symbol: String,
        #[tabled(rename = "Min Qty")]
        min_qty: String,
        #[tabled(rename = "Max Qty")]
        max_qty: String,
        #[tabled(rename = "Step Size")]
        step_size: String,
    }
    let mut market_lot_size_filter_rows = Vec::new();

    // MaxNumOrdersFilter Table
    #[derive(Tabled)]
    struct MaxNumOrdersFilterRow {
        #[tabled(rename = "Symbol")]
        symbol: String,
        #[tabled(rename = "Limit")]
        limit: String,
    }
    let mut max_num_orders_filter_rows = Vec::new();

    // PercentPriceFilter Table
    #[derive(Tabled)]
    struct PercentPriceFilterRow {
        #[tabled(rename = "Symbol")]
        symbol: String,
        #[tabled(rename = "Multiplier Down")]
        multiplier_down: String,
        #[tabled(rename = "Multiplier Up")]
        multiplier_up: String,
        #[tabled(rename = "Multiplier Decimal")]
        multiplier_decimal: String,
    }
    let mut percent_price_filter_rows = Vec::new();

    for symbol in &info.symbols {
        for filter in &symbol.filters {
            match filter {
                Filter::PriceFilter(f) => {
                    price_filter_rows.push(PriceFilterRow {
                        symbol: symbol.symbol.clone(),
                        min_price: f.min_price.clone().unwrap_or_default(),
                        max_price: f.max_price.clone().unwrap_or_default(),
                        tick_size: f.tick_size.clone().unwrap_or_default(),
                    });
                }
                Filter::LotSizeFilter(f) => {
                    lot_size_filter_rows.push(LotSizeFilterRow {
                        symbol: symbol.symbol.clone(),
                        min_qty: f.min_qty.clone().unwrap_or_default(),
                        max_qty: f.max_qty.clone().unwrap_or_default(),
                        step_size: f.step_size.clone().unwrap_or_default(),
                    });
                }
                Filter::MarketLotSizeFilter(f) => {
                    market_lot_size_filter_rows.push(MarketLotSizeFilterRow {
                        symbol: symbol.symbol.clone(),
                        min_qty: f.min_qty.clone().unwrap_or_default(),
                        max_qty: f.max_qty.clone().unwrap_or_default(),
                        step_size: f.step_size.clone().unwrap_or_default(),
                    });
                }
                Filter::MaxNumOrdersFilter(f) => {
                    max_num_orders_filter_rows.push(MaxNumOrdersFilterRow {
                        symbol: symbol.symbol.clone(),
                        limit: f.limit.map(|v| v.to_string()).unwrap_or_default(),
                    });
                }
                Filter::PercentPriceFilter(f) => {
                    percent_price_filter_rows.push(PercentPriceFilterRow {
                        symbol: symbol.symbol.clone(),
                        multiplier_down: f.multiplier_down.clone().unwrap_or_default(),
                        multiplier_up: f.multiplier_up.clone().unwrap_or_default(),
                        multiplier_decimal: f
                            .multiplier_decimal
                            .as_ref()
                            .map(|v| v.to_string())
                            .unwrap_or_default(),
                    });
                }
                Filter::MaxNumAlgoOrdersFilter(_) => {
                    // Not displayed in table
                }
                Filter::Unknown => {
                    // Not displayed in table
                }
            }
        }
    }

    if !price_filter_rows.is_empty() {
        println!("\nPRICE_FILTER:");
        let mut table = Table::new(price_filter_rows);
        table.with(Style::rounded());
        println!("{table}");
    }
    if !lot_size_filter_rows.is_empty() {
        println!("\nLOT_SIZE:");
        let mut table = Table::new(lot_size_filter_rows);
        table.with(Style::rounded());
        println!("{table}");
    }
    if !market_lot_size_filter_rows.is_empty() {
        println!("\nMARKET_LOT_SIZE:");
        let mut table = Table::new(market_lot_size_filter_rows);
        table.with(Style::rounded());
        println!("{table}");
    }
    if !max_num_orders_filter_rows.is_empty() {
        println!("\nMAX_NUM_ORDERS:");
        let mut table = Table::new(max_num_orders_filter_rows);
        table.with(Style::rounded());
        println!("{table}");
    }
    if !percent_price_filter_rows.is_empty() {
        println!("\nPERCENT_PRICE:");
        let mut table = Table::new(percent_price_filter_rows);
        table.with(Style::rounded());
        println!("{table}");
    }
}
