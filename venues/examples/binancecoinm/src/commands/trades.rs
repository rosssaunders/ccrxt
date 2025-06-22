use std::sync::Arc;

use chrono::Utc;
use tabled::{settings::Style, Table, Tabled};
use venues::binance::coinm::AccountTradeListRequest;
use venues::binance::coinm::Errors;
use venues::binance::coinm::PrivateRestClient;

#[derive(Tabled)]
pub struct TradeRow {
    #[tabled(rename = "Trade ID")]
    pub id: u64,

    #[tabled(rename = "Side")]
    pub side: String,

    #[tabled(rename = "Price")]
    pub price: String,

    #[tabled(rename = "Quantity")]
    pub quantity: String,

    #[tabled(rename = "Time")]
    pub time: String,
}

pub async fn handle_trades_command(client: Arc<PrivateRestClient>, symbol: String, limit: u32) -> Result<(), Errors> {
    let mut trades = Vec::new();
    let mut from_id = 0_u64;
    let mut page_count = 0;
    const MAX_PAGES: u32 = 1000;

    loop {
        if page_count >= MAX_PAGES {
            break;
        }

        // Create request with current pagination state
        let now = Utc::now().timestamp_millis() as u64;
        let trade_req = AccountTradeListRequest {
            symbol: Some(symbol.clone()),
            from_id: Some(from_id),
            limit: Some(limit),
            timestamp: now,
            ..Default::default()
        };

        // Fetch page of trades
        let resp = client.get_account_trades(trade_req).await?;
        if resp.data.is_empty() {
            break;
        }

        // Update pagination state
        if let Some(last_trade) = resp.data.iter().max_by_key(|t| t.id) {
            from_id = last_trade.id + 1;
        }

        trades.extend(resp.data);
        page_count += 1;
    }

    let mut rows: Vec<TradeRow> = trades
        .iter()
        .map(|trade| TradeRow {
            id: trade.id,
            side: format!("{:?}", trade.side),
            price: trade.price.to_string(),
            quantity: trade.quantity.to_string(),
            time: trade.time.to_string(),
        })
        .collect();

    rows.sort_by(|a, b| a.id.cmp(&b.id));

    println!("Trades for {} ({} total):", symbol, rows.len());
    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");

    Ok(())
}
