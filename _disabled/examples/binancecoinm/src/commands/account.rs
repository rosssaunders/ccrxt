use std::sync::Arc;

use anyhow::Result;
use tabled::{settings::Style, Table, Tabled};
use venues::binance::coinm::{AccountRequest, PrivateRestClient};

#[derive(Tabled)]
pub struct AssetRow {
    #[tabled(rename = "Asset")]
    pub asset: String,

    #[tabled(rename = "Wallet Balance")]
    pub wallet_balance: String,

    #[tabled(rename = "Unrealized PNL")]
    pub unrealized_profit: String,

    #[tabled(rename = "Margin Balance")]
    pub margin_balance: String,
}

pub async fn handle_account_command(client: Arc<PrivateRestClient>) -> Result<()> {
    let request = AccountRequest {
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
        recv_window: None,
    };
    let account = client.get_account(request).await?;

    if account.data.assets.is_empty() {
        println!("No assets found in account");
        return Ok(());
    }

    let mut rows: Vec<AssetRow> = account
        .data
        .assets
        .iter()
        .map(|asset| AssetRow {
            asset: asset.asset.clone(),
            wallet_balance: asset.wallet_balance.to_string(),
            unrealized_profit: asset.unrealized_profit.to_string(),
            margin_balance: asset.margin_balance.to_string(),
        })
        .collect();
    rows.sort_by(|a, b| a.asset.cmp(&b.asset));

    let mut table = Table::new(rows);
    table.with(Style::rounded());
    println!("{table}");

    Ok(())
}
