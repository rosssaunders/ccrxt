use dotenv::dotenv;
use std::env;
use venues::gateio::{
    PrivateRestClient,
    private::rest::{
        wallet::{
            TotalBalanceRequest, DepositAddressRequest, DepositsRequest, 
            WithdrawalsRequest, CreateTransferRequest, WithdrawalFeesRequest,
        },
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    let api_key = env::var("GATEIO_API_KEY")
        .expect("GATEIO_API_KEY environment variable not set");
    let api_secret = env::var("GATEIO_API_SECRET")
        .expect("GATEIO_API_SECRET environment variable not set");

    // Initialize private client
    let private_client = PrivateRestClient::new(api_key, api_secret, false)?;

    println!("=== Gate.io Wallet Management Examples ===\n");

    // 1. Total Balance Overview
    println!("1. Getting total balance across all accounts...");
    
    let total_balance = private_client.get_total_balance(TotalBalanceRequest::default()).await?;
    
    println!("   Total Portfolio Value: {} {}", total_balance.total.amount, total_balance.total.currency);
    println!("   Account Details:");
    
    for (currency, balance) in &total_balance.details {
        let total_value = balance.available.parse::<f64>().unwrap_or(0.0) 
                        + balance.unrealised_pnl.parse::<f64>().unwrap_or(0.0);
        
        if total_value > 0.001 { // Only show currencies with meaningful balances
            println!("     {}: Available: {}, PnL: {}, Borrowed: {}", 
                currency, balance.available, balance.unrealised_pnl, balance.borrowed);
        }
    }

    // 2. Deposit Addresses
    println!("\n2. Getting deposit addresses...");
    
    let popular_currencies = vec!["BTC", "ETH", "USDT"];
    
    for currency in &popular_currencies {
        match private_client.get_deposit_address(DepositAddressRequest {
            currency: currency.to_string(),
        }).await {
            Ok(deposit_info) => {
                println!("   {} Deposit Address: {}", currency, deposit_info.address);
                
                if !deposit_info.multichain_addresses.is_empty() {
                    println!("     Multichain addresses:");
                    for chain_addr in &deposit_info.multichain_addresses {
                        println!("       {}: {}", chain_addr.chain, chain_addr.address);
                        if let Some(payment_id) = &chain_addr.payment_id {
                            println!("         Payment ID: {}", payment_id);
                        }
                    }
                }
            }
            Err(e) => {
                println!("   {} Deposit Address: Error - {}", currency, e);
            }
        }
    }

    // 3. Deposit History
    println!("\n3. Recent deposit history...");
    
    let deposits = private_client.get_deposits(DepositsRequest {
        currency: None,
        from: None,
        to: None,
        limit: Some(10),
        offset: None,
    }).await?;
    
    println!("   Recent deposits: {}", deposits.len());
    
    for deposit in &deposits {
        println!("   {} {}: {} {} (Status: {}) - TxID: {}", 
            deposit.currency, deposit.chain, deposit.amount, 
            deposit.currency, deposit.status, deposit.txid);
    }
    
    if deposits.is_empty() {
        println!("   No recent deposits found");
    }

    // 4. Withdrawal History
    println!("\n4. Recent withdrawal history...");
    
    let withdrawals = private_client.get_withdrawals(WithdrawalsRequest {
        currency: None,
        from: None,
        to: None,
        limit: Some(10),
        offset: None,
    }).await?;
    
    println!("   Recent withdrawals: {}", withdrawals.len());
    
    for withdrawal in &withdrawals {
        println!("   {} {}: {} {} (Fee: {} {}) - Status: {}", 
            withdrawal.currency, withdrawal.chain, withdrawal.amount, 
            withdrawal.currency, withdrawal.fee, withdrawal.currency, withdrawal.status);
        
        if let Some(txid) = &withdrawal.txid {
            println!("     TxID: {}", txid);
        }
    }
    
    if withdrawals.is_empty() {
        println!("   No recent withdrawals found");
    }

    // 5. Withdrawal Fees
    println!("\n5. Withdrawal fees for popular currencies...");
    
    let withdrawal_fees = private_client.get_withdrawal_fees(WithdrawalFeesRequest::default()).await?;
    
    println!("   Withdrawal fees:");
    for fee_info in withdrawal_fees.iter().take(10) { // Show first 10
        println!("     {}: {} {} (Min: {}, Max: {})", 
            fee_info.currency, fee_info.fixed, fee_info.currency,
            fee_info.min_amount, fee_info.max_amount);
    }

    // 6. Internal Transfers
    println!("\n6. Internal transfer example (commented out for safety)...");
    
    /*
    // Example: Transfer from spot to futures account
    let transfer_request = CreateTransferRequest {
        currency: "USDT".to_string(),
        from: "spot".to_string(),
        to: "futures".to_string(),
        amount: "10".to_string(), // $10 USDT
        currency_pair: None,
        settle: Some("usdt".to_string()), // For futures account
    };
    
    let transfer_result = private_client.create_transfer(transfer_request).await?;
    println!("   Transfer completed: {} {} from {} to {}", 
        transfer_result.amount, transfer_result.currency, 
        transfer_result.from, transfer_result.to);
    */
    
    println!("   Internal transfers allow moving funds between:");
    println!("     • Spot ↔ Futures");
    println!("     • Spot ↔ Margin");
    println!("     • Spot ↔ Options");
    println!("     • Cross-margin accounts");
    println!("     • Delivery accounts");

    // 7. Account Types Summary
    println!("\n7. Gate.io Account Types:");
    println!("   • Spot: For spot trading and holding cryptocurrencies");
    println!("   • Margin: For leveraged spot trading with borrowed funds");
    println!("   • Cross-margin: Shared margin across multiple pairs");
    println!("   • Futures: For perpetual and quarterly futures contracts");
    println!("   • Delivery: For delivery futures contracts");
    println!("   • Options: For options trading");
    println!("   • Unified: Combined account for cross-asset trading");

    // 8. Security Best Practices
    println!("\n8. Security Best Practices:");
    println!("   • Always verify deposit addresses before sending funds");
    println!("   • Use small test amounts for first-time deposits");
    println!("   • Enable 2FA on your account");
    println!("   • Whitelist withdrawal addresses");
    println!("   • Monitor your account regularly");
    println!("   • Keep API keys secure and limit permissions");

    println!("\n=== Wallet management examples completed successfully! ===");
    
    Ok(())
}

/*
To run this example:

1. Create a .env file in the venues directory with:
   GATEIO_API_KEY=your_api_key_here
   GATEIO_API_SECRET=your_api_secret_here

2. Run with:
   cargo run --example gateio_wallet_management_example

Note: This example only reads wallet data and doesn't perform transfers.
Uncomment transfer examples only if you want to move funds between accounts.

Important: Always double-check addresses and amounts before performing 
real deposits or withdrawals.
*/