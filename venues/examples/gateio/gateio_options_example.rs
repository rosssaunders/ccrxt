use tokio;
use tracing_subscriber;
use venues::gateio::public::{
    RestClient as PublicClient,
    rest::{
        OptionsCandlesticksRequest, OptionsContractsRequest, OptionsOrderBookRequest,
        OptionsSettlementsRequest, OptionsTickersRequest, OptionsTradesRequest,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Gate.io Options API Example ===\n");

    // Create public client
    let public_client = PublicClient::new(false)?; // false = use live trading, true = use testnet

    // 1. Get all available underlying assets
    println!("1. Fetching all underlying assets...");
    match public_client.get_options_underlyings().await {
        Ok(underlyings) => {
            println!("Found {} underlying assets:", underlyings.len());
            for (i, underlying) in underlyings.iter().take(5).enumerate() {
                println!(
                    "  {}. {} - Index Price: {}",
                    i + 1,
                    underlying.name,
                    underlying.index_price
                );
            }
            if underlyings.len() > 5 {
                println!("  ... and {} more", underlyings.len() - 5);
            }
        }
        Err(e) => println!("Error fetching underlyings: {}", e),
    }
    println!();

    // 2. Get all expiration times
    println!("2. Fetching expiration times...");
    match public_client.get_options_expirations().await {
        Ok(expirations) => {
            println!("Found {} expiration times:", expirations.len());
            for (i, expiration) in expirations.iter().take(5).enumerate() {
                let datetime = chrono::DateTime::from_timestamp(*expiration, 0).unwrap_or_default();
                println!("  {}. {} ({})", i + 1, expiration, datetime);
            }
            if expirations.len() > 5 {
                println!("  ... and {} more", expirations.len() - 5);
            }
        }
        Err(e) => println!("Error fetching expirations: {}", e),
    }
    println!();

    // 3. Get options contracts for BTC
    println!("3. Fetching BTC options contracts...");
    let contracts_request = OptionsContractsRequest {
        underlying: Some("BTC_USDT".to_string()),
        expiration: None,
    };

    match public_client.get_options_contracts(contracts_request).await {
        Ok(contracts) => {
            println!("Found {} BTC options contracts:", contracts.len());
            for (i, contract) in contracts.iter().take(3).enumerate() {
                println!(
                    "  {}. {} - Type: {}, Strike: {}, Mark Price: {}",
                    i + 1,
                    contract.name,
                    contract.option_type,
                    contract.strike_price,
                    contract.mark_price
                );
            }
            if contracts.len() > 3 {
                println!("  ... and {} more", contracts.len() - 3);
            }
        }
        Err(e) => println!("Error fetching contracts: {}", e),
    }
    println!();

    // 4. Get options tickers
    println!("4. Fetching options tickers...");
    let tickers_request = OptionsTickersRequest {
        underlying: Some("BTC_USDT".to_string()),
    };

    match public_client.get_options_tickers(tickers_request).await {
        Ok(tickers) => {
            println!("Found {} options tickers:", tickers.len());
            for (i, ticker) in tickers.iter().take(3).enumerate() {
                println!(
                    "  {}. {} - Last: {}, Mark Price: {}, Delta: {}",
                    i + 1,
                    ticker.name,
                    ticker.last,
                    ticker.mark_price,
                    ticker.delta
                );
            }
            if tickers.len() > 3 {
                println!("  ... and {} more", tickers.len() - 3);
            }

            // 5. Get order book for first contract
            if let Some(first_ticker) = tickers.first() {
                println!("\\n5. Fetching order book for {}...", first_ticker.name);
                let orderbook_request = OptionsOrderBookRequest {
                    contract: first_ticker.name.clone(),
                    interval: Some("0.1".to_string()),
                    limit: Some(5),
                    with_id: Some(true),
                };

                match public_client
                    .get_options_order_book(orderbook_request)
                    .await
                {
                    Ok(orderbook) => {
                        println!("Order book for {}:", first_ticker.name);
                        println!("  Update time: {}", orderbook.update);
                        println!("  Asks (selling):");
                        for (i, ask) in orderbook.asks.iter().take(3).enumerate() {
                            println!("    {}. Price: {}, Size: {}", i + 1, ask.p, ask.s);
                        }
                        println!("  Bids (buying):");
                        for (i, bid) in orderbook.bids.iter().take(3).enumerate() {
                            println!("    {}. Price: {}, Size: {}", i + 1, bid.p, bid.s);
                        }
                    }
                    Err(e) => println!("Error fetching order book: {}", e),
                }

                // 6. Get recent trades for the contract
                println!("\\n6. Fetching recent trades for {}...", first_ticker.name);
                let trades_request = OptionsTradesRequest {
                    contract: first_ticker.name.clone(),
                    last_id: None,
                    limit: Some(5),
                };

                match public_client.get_options_trades(trades_request).await {
                    Ok(trades) => {
                        println!("Recent trades for {}:", first_ticker.name);
                        for (i, trade) in trades.iter().enumerate() {
                            let datetime =
                                chrono::DateTime::from_timestamp(trade.create_time as i64, 0)
                                    .unwrap_or_default();
                            println!(
                                "  {}. ID: {}, Size: {}, Price: {}, Time: {}",
                                i + 1,
                                trade.id,
                                trade.size,
                                trade.price,
                                datetime
                            );
                        }
                    }
                    Err(e) => println!("Error fetching trades: {}", e),
                }

                // 7. Get candlestick data
                println!(
                    "\\n7. Fetching candlestick data for {}...",
                    first_ticker.name
                );
                let candlesticks_request = OptionsCandlesticksRequest {
                    contract: first_ticker.name.clone(),
                    from: None,
                    to: None,
                    limit: Some(5),
                    interval: Some("1h".to_string()),
                };

                match public_client
                    .get_options_candlesticks(candlesticks_request)
                    .await
                {
                    Ok(candlesticks) => {
                        println!("Candlestick data for {}:", first_ticker.name);
                        for (i, candle) in candlesticks.iter().enumerate() {
                            let datetime =
                                chrono::DateTime::from_timestamp(candle.t, 0).unwrap_or_default();
                            println!(
                                "  {}. Time: {}, O: {}, H: {}, L: {}, C: {}, V: {}",
                                i + 1,
                                datetime,
                                candle.o,
                                candle.h,
                                candle.l,
                                candle.c,
                                candle.v
                            );
                        }
                    }
                    Err(e) => println!("Error fetching candlesticks: {}", e),
                }
            }
        }
        Err(e) => println!("Error fetching tickers: {}", e),
    }
    println!();

    // 8. Get underlying ticker
    println!("8. Fetching underlying ticker for BTC_USDT...");
    match public_client.get_underlying_ticker("BTC_USDT").await {
        Ok(underlying_ticker) => {
            println!("BTC_USDT underlying ticker:");
            println!("  Trading enabled: {}", underlying_ticker.trade_enabled);
            println!("  Index price: {}", underlying_ticker.index_price);
            println!("  Put trades (24h): {}", underlying_ticker.trade_put);
            println!("  Call trades (24h): {}", underlying_ticker.trade_call);
        }
        Err(e) => println!("Error fetching underlying ticker: {}", e),
    }
    println!();

    // 9. Get settlement history
    println!("9. Fetching settlement history...");
    let settlements_request = OptionsSettlementsRequest {
        underlying: Some("BTC_USDT".to_string()),
        limit: Some(5),
    };

    match public_client
        .get_options_settlements(settlements_request)
        .await
    {
        Ok(settlements) => {
            println!("Found {} settlement records:", settlements.len());
            for (i, settlement) in settlements.iter().enumerate() {
                let datetime =
                    chrono::DateTime::from_timestamp(settlement.time, 0).unwrap_or_default();
                println!(
                    "  {}. Contract: {}, Strike: {}, Settle Price: {}, Time: {}",
                    i + 1,
                    settlement.contract,
                    settlement.strike_price,
                    settlement.settle_price,
                    datetime
                );
            }
        }
        Err(e) => println!("Error fetching settlements: {}", e),
    }
    println!();

    // For authenticated endpoints, you would need API credentials
    println!("=== Authenticated Endpoints (Requires API Keys) ===");
    println!("Note: The following examples require valid API credentials:");
    println!("- Get options account balance");
    println!("- Get options positions");
    println!("- Create/cancel options orders");
    println!("- Get trading history");
    println!("- Configure Market Maker Protection (MMP)");
    println!("- Get account book and position close history");

    // Example of how you would use authenticated endpoints:
    /*
    if let (Ok(key), Ok(secret)) = (std::env::var("GATEIO_API_KEY"), std::env::var("GATEIO_SECRET_KEY")) {
        let private_client = PrivateClient::new(&key, &secret);

        // Get options account
        match private_client.get_options_accounts().await {
            Ok(account) => {
                println!("Options account balance:");
                println!("  Total: {}", account.total);
                println!("  Available: {}", account.available);
                println!("  Unrealized PnL: {}", account.unrealised_pnl);
            }
            Err(e) => println!("Error fetching account: {}", e),
        }

        // Get options positions
        let positions_request = OptionsPositionsRequest {
            underlying: Some("BTC_USDT".to_string()),
            limit: Some(10),
            offset: None,
        };

        match private_client.get_options_positions(positions_request).await {
            Ok(positions) => {
                println!("Found {} positions:", positions.len());
                for position in positions {
                    println!("  Contract: {}, Size: {}, PnL: {}",
                        position.contract, position.size, position.unrealised_pnl);
                }
            }
            Err(e) => println!("Error fetching positions: {}", e),
        }
    }
    */

    println!("\\n=== Options API Example Complete ===");
    Ok(())
}
