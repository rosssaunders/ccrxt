// Example: Place a trade on Binance COIN-M using the coinm Rust module
// Loads API credentials from .env
use std::{env, sync::Arc};
use anyhow::{Result};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use secrecy::SecretString;
use venues::binance::coinm::PrivateRestClient;
use venues::binance::coinm::PublicRestClient;
use venues::binance::coinm::{ApiError, RateLimiter, Errors};
use rest::secrets::{SecretValue};

mod commands;
use commands::{handle_account_command, handle_trades_command, handle_batch_order_command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use testnet instead of mainnet
    #[arg(long)]
    prod: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get account information
    Account,
    
    /// Get recent trades for a symbol
    Trades {
        /// Trading pair symbol (e.g., BTCUSD)
        #[arg(required = true)]
        symbol: String,
        /// Maximum number of trades to fetch
        #[arg(short, long, default_value = "100")]
        limit: u32,
    },
    
    /// Place a batch order
    BatchOrder {
        /// Trading pair symbol (e.g., BTCUSD)
        #[arg(required = true)]
        symbol: String,
        
        /// Order side (BUY or SELL)
        #[arg(required = true)]
        side: String,
        
        /// Order type (LIMIT or MARKET)
        #[arg(required = true)]
        order_type: String,
        
        /// Order quantity
        #[arg(required = true)]
        quantity: f64,
        
        /// Order price (required for LIMIT orders)
        #[arg(short, long)]
        price: Option<f64>,
    },

    /// Get exchange information
    ExchangeInfo,
}

// fn handle_api_error(err: &BinanceCoinMAPIError) -> ! {
//     match err {
//         BinanceCoinMAPIError::UnknownApiError { msg } => {
//             eprintln!("API Error: {}", msg);
//         }
//         BinanceCoinMAPIError::Disconnected { msg } => {
//             eprintln!("Disconnected from server: {}", msg);
//         }
//         BinanceCoinMAPIError::Unauthorized { msg } => {
//             eprintln!("Unauthorized: {}", msg);
//         }
//         BinanceCoinMAPIError::TooManyRequests { msg } => {
//             eprintln!("Rate limit exceeded: {}", msg);
//         }
//         BinanceCoinMAPIError::IpBanned { msg } => {
//             eprintln!("IP banned: {}", msg);
//         }
//         BinanceCoinMAPIError::DuplicateIp { msg } => {
//             eprintln!("Duplicate IP: {}", msg);
//         }
//         BinanceCoinMAPIError::NoSuchIp { msg } => {
//             eprintln!("No such IP: {}", msg);
//         }
//         BinanceCoinMAPIError::UnexpectedResponse { msg } => {
//             eprintln!("Unexpected response: {}", msg);
//         }
//         BinanceCoinMAPIError::Timeout { msg } => {
//             eprintln!("Request timeout: {}", msg);
//         }
//         BinanceCoinMAPIError::ErrorMsgReceived { msg } => {
//             eprintln!("Error message received: {}", msg);
//         }
//         BinanceCoinMAPIError::NonWhiteList { msg } => {
//             eprintln!("Non-whitelisted IP: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidMessage { msg } => {
//             eprintln!("Invalid message: {}", msg);
//         }
//         BinanceCoinMAPIError::UnknownOrderComposition { msg } => {
//             eprintln!("Unknown order composition: {}", msg);
//         }
//         BinanceCoinMAPIError::TooManyOrders { msg } => {
//             eprintln!("Too many orders: {}", msg);
//         }
//         BinanceCoinMAPIError::ServiceShuttingDown { msg } => {
//             eprintln!("Service shutting down: {}", msg);
//         }
//         BinanceCoinMAPIError::UnsupportedOperation { msg } => {
//             eprintln!("Unsupported operation: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidTimestamp { msg } => {
//             eprintln!("Invalid timestamp: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidSignature { msg } => {
//             eprintln!("Invalid signature: {}", msg);
//         }
//         BinanceCoinMAPIError::StartTimeGreaterThanEndTime { msg } => {
//             eprintln!("Start time greater than end time: {}", msg);
//         }
//         BinanceCoinMAPIError::IllegalChars { msg } => {
//             eprintln!("Illegal characters: {}", msg);
//         }
//         BinanceCoinMAPIError::TooManyParameters { msg } => {
//             eprintln!("Too many parameters: {}", msg);
//         }
//         BinanceCoinMAPIError::MandatoryParamEmptyOrMalformed { msg } => {
//             eprintln!("Mandatory parameter empty or malformed: {}", msg);
//         }
//         BinanceCoinMAPIError::UnknownParam { msg } => {
//             eprintln!("Unknown parameter: {}", msg);
//         }
//         BinanceCoinMAPIError::UnreadParameters { msg } => {
//             eprintln!("Unread parameters: {}", msg);
//         }
//         BinanceCoinMAPIError::ParamEmpty { msg } => {
//             eprintln!("Empty parameter: {}", msg);
//         }
//         BinanceCoinMAPIError::ParamNotRequired { msg } => {
//             eprintln!("Parameter not required: {}", msg);
//         }
//         BinanceCoinMAPIError::BadAsset { msg } => {
//             eprintln!("Bad asset: {}", msg);
//         }
//         BinanceCoinMAPIError::BadAccount { msg } => {
//             eprintln!("Bad account: {}", msg);
//         }
//         BinanceCoinMAPIError::BadInstrumentType { msg } => {
//             eprintln!("Bad instrument type: {}", msg);
//         }
//         BinanceCoinMAPIError::BadPrecision { msg } => {
//             eprintln!("Bad precision: {}", msg);
//         }
//         BinanceCoinMAPIError::NoDepth { msg } => {
//             eprintln!("No depth: {}", msg);
//         }
//         BinanceCoinMAPIError::WithdrawNotNegative { msg } => {
//             eprintln!("Withdraw not negative: {}", msg);
//         }
//         BinanceCoinMAPIError::TifNotRequired { msg } => {
//             eprintln!("Time in force not required: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidTif { msg } => {
//             eprintln!("Invalid time in force: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidOrderType { msg } => {
//             eprintln!("Invalid order type: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidSide { msg } => {
//             eprintln!("Invalid side: {}", msg);
//         }
//         BinanceCoinMAPIError::EmptyNewClOrdId { msg } => {
//             eprintln!("Empty new client order ID: {}", msg);
//         }
//         BinanceCoinMAPIError::EmptyOrgClOrdId { msg } => {
//             eprintln!("Empty original client order ID: {}", msg);
//         }
//         BinanceCoinMAPIError::BadInterval { msg } => {
//             eprintln!("Bad interval: {}", msg);
//         }
//         BinanceCoinMAPIError::BadSymbol { msg } => {
//             eprintln!("Bad symbol: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidListenKey { msg } => {
//             eprintln!("Invalid listen key: {}", msg);
//         }
//         BinanceCoinMAPIError::MoreThanXxHours { msg } => {
//             eprintln!("More than XX hours: {}", msg);
//         }
//         BinanceCoinMAPIError::OptionalParamsBadCombo { msg } => {
//             eprintln!("Optional parameters bad combination: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidParameter { msg } => {
//             eprintln!("Invalid parameter: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidNewOrderRespType { msg } => {
//             eprintln!("Invalid new order response type: {}", msg);
//         }
//         BinanceCoinMAPIError::NewOrderRejected { msg } => {
//             eprintln!("New order rejected: {}", msg);
//         }
//         BinanceCoinMAPIError::CancelRejected { msg } => {
//             eprintln!("Cancel rejected: {}", msg);
//         }
//         BinanceCoinMAPIError::NoSuchOrder { msg } => {
//             eprintln!("No such order: {}", msg);
//         }
//         BinanceCoinMAPIError::BadApiKeyFmt { msg } => {
//             eprintln!("Bad API key format: {}", msg);
//         }
//         BinanceCoinMAPIError::RejectedMbxKey { msg } => {
//             eprintln!("Rejected MBX key: {}", msg);
//         }
//         BinanceCoinMAPIError::NoTradingWindow { msg } => {
//             eprintln!("No trading window: {}", msg);
//         }
//         BinanceCoinMAPIError::BalanceNotSufficient { msg } => {
//             eprintln!("Balance not sufficient: {}", msg);
//         }
//         BinanceCoinMAPIError::MarginNotSufficient { msg } => {
//             eprintln!("Margin not sufficient: {}", msg);
//         }
//         BinanceCoinMAPIError::UnableToFill { msg } => {
//             eprintln!("Unable to fill: {}", msg);
//         }
//         BinanceCoinMAPIError::OrderWouldImmediatelyTrigger { msg } => {
//             eprintln!("Order would immediately trigger: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyReject { msg } => {
//             eprintln!("Reduce only reject: {}", msg);
//         }
//         BinanceCoinMAPIError::UserInLiquidation { msg } => {
//             eprintln!("User in liquidation: {}", msg);
//         }
//         BinanceCoinMAPIError::PositionNotSufficient { msg } => {
//             eprintln!("Position not sufficient: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxOpenOrderExceeded { msg } => {
//             eprintln!("Max open order exceeded: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyOrderTypeNotSupported { msg } => {
//             eprintln!("Reduce only order type not supported: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxLeverageRatio { msg } => {
//             eprintln!("Max leverage ratio: {}", msg);
//         }
//         BinanceCoinMAPIError::MinLeverageRatio { msg } => {
//             eprintln!("Min leverage ratio: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidOrderStatus { msg } => {
//             eprintln!("Invalid order status: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceLessThanZero { msg } => {
//             eprintln!("Price less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceGreaterThanMaxPrice { msg } => {
//             eprintln!("Price greater than max price: {}", msg);
//         }
//         BinanceCoinMAPIError::QtyLessThanZero { msg } => {
//             eprintln!("Quantity less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::QtyLessThanMinQty { msg } => {
//             eprintln!("Quantity less than min quantity: {}", msg);
//         }
//         BinanceCoinMAPIError::QtyGreaterThanMaxQty { msg } => {
//             eprintln!("Quantity greater than max quantity: {}", msg);
//         }
//         BinanceCoinMAPIError::StopPriceLessThanZero { msg } => {
//             eprintln!("Stop price less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::StopPriceGreaterThanMaxPrice { msg } => {
//             eprintln!("Stop price greater than max price: {}", msg);
//         }
//         BinanceCoinMAPIError::TickSizeLessThanZero { msg } => {
//             eprintln!("Tick size less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxPriceLessThanMinPrice { msg } => {
//             eprintln!("Max price less than min price: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxQtyLessThanMinQty { msg } => {
//             eprintln!("Max quantity less than min quantity: {}", msg);
//         }
//         BinanceCoinMAPIError::StepSizeLessThanZero { msg } => {
//             eprintln!("Step size less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxNumOrdersLessThanZero { msg } => {
//             eprintln!("Max number of orders less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceLessThanMinPrice { msg } => {
//             eprintln!("Price less than min price: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceNotIncreasedByTickSize { msg } => {
//             eprintln!("Price not increased by tick size: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidClOrdIdLen { msg } => {
//             eprintln!("Invalid client order ID length: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceHighterThanMultiplierUp { msg } => {
//             eprintln!("Price higher than multiplier up: {}", msg);
//         }
//         BinanceCoinMAPIError::MultiplierUpLessThanZero { msg } => {
//             eprintln!("Multiplier up less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::MultiplierDownLessThanZero { msg } => {
//             eprintln!("Multiplier down less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::CompositeScaleOverflow { msg } => {
//             eprintln!("Composite scale overflow: {}", msg);
//         }
//         BinanceCoinMAPIError::TargetStrategyInvalid { msg } => {
//             eprintln!("Target strategy invalid: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidDepthLimit { msg } => {
//             eprintln!("Invalid depth limit: {}", msg);
//         }
//         BinanceCoinMAPIError::WrongMarketStatus { msg } => {
//             eprintln!("Wrong market status: {}", msg);
//         }
//         BinanceCoinMAPIError::QtyNotIncreasedByStepSize { msg } => {
//             eprintln!("Quantity not increased by step size: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceLowerThanMultiplierDown { msg } => {
//             eprintln!("Price lower than multiplier down: {}", msg);
//         }
//         BinanceCoinMAPIError::MultiplierDecimalLessThanZero { msg } => {
//             eprintln!("Multiplier decimal less than zero: {}", msg);
//         }
//         BinanceCoinMAPIError::CommissionInvalid { msg } => {
//             eprintln!("Commission invalid: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidAccountType { msg } => {
//             eprintln!("Invalid account type: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidLeverage { msg } => {
//             eprintln!("Invalid leverage: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidTickSizePrecision { msg } => {
//             eprintln!("Invalid tick size precision: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidStepSizePrecision { msg } => {
//             eprintln!("Invalid step size precision: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidWorkingType { msg } => {
//             eprintln!("Invalid working type: {}", msg);
//         }
//         BinanceCoinMAPIError::ExceedMaxCancelOrderSize { msg } => {
//             eprintln!("Exceed max cancel order size: {}", msg);
//         }
//         BinanceCoinMAPIError::InsuranceAccountNotFound { msg } => {
//             eprintln!("Insurance account not found: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidBalanceType { msg } => {
//             eprintln!("Invalid balance type: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxStopOrderExceeded { msg } => {
//             eprintln!("Max stop order exceeded: {}", msg);
//         }
//         BinanceCoinMAPIError::NoNeedToChangeMarginType { msg } => {
//             eprintln!("No need to change margin type: {}", msg);
//         }
//         BinanceCoinMAPIError::ThereExistsOpenOrders { msg } => {
//             eprintln!("There exists open orders: {}", msg);
//         }
//         BinanceCoinMAPIError::ThereExistsQuantity { msg } => {
//             eprintln!("There exists quantity: {}", msg);
//         }
//         BinanceCoinMAPIError::AddIsolatedMarginReject { msg } => {
//             eprintln!("Add isolated margin reject: {}", msg);
//         }
//         BinanceCoinMAPIError::CrossBalanceInsufficient { msg } => {
//             eprintln!("Cross balance insufficient: {}", msg);
//         }
//         BinanceCoinMAPIError::IsolatedBalanceInsufficient { msg } => {
//             eprintln!("Isolated balance insufficient: {}", msg);
//         }
//         BinanceCoinMAPIError::NoNeedToChangeAutoAddMargin { msg } => {
//             eprintln!("No need to change auto add margin: {}", msg);
//         }
//         BinanceCoinMAPIError::AutoAddCrossedMarginReject { msg } => {
//             eprintln!("Auto add crossed margin reject: {}", msg);
//         }
//         BinanceCoinMAPIError::AddIsolatedMarginNoPositionReject { msg } => {
//             eprintln!("Add isolated margin no position reject: {}", msg);
//         }
//         BinanceCoinMAPIError::AmountMustBePositive { msg } => {
//             eprintln!("Amount must be positive: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidApiKeyType { msg } => {
//             eprintln!("Invalid API key type: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidRsaPublicKey { msg } => {
//             eprintln!("Invalid RSA public key: {}", msg);
//         }
//         BinanceCoinMAPIError::MaxPriceTooLarge { msg } => {
//             eprintln!("Max price too large: {}", msg);
//         }
//         BinanceCoinMAPIError::NoNeedToChangePositionSide { msg } => {
//             eprintln!("No need to change position side: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidPositionSide { msg } => {
//             eprintln!("Invalid position side: {}", msg);
//         }
//         BinanceCoinMAPIError::PositionSideNotMatch { msg } => {
//             eprintln!("Position side not match: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyConflict { msg } => {
//             eprintln!("Reduce only conflict: {}", msg);
//         }
//         BinanceCoinMAPIError::PositionSideChangeExistsOpenOrders { msg } => {
//             eprintln!("Position side change exists open orders: {}", msg);
//         }
//         BinanceCoinMAPIError::PositionSideChangeExistsQuantity { msg } => {
//             eprintln!("Position side change exists quantity: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidBatchPlaceOrderSize { msg } => {
//             eprintln!("Invalid batch place order size: {}", msg);
//         }
//         BinanceCoinMAPIError::PlaceBatchOrdersFail { msg } => {
//             eprintln!("Place batch orders fail: {}", msg);
//         }
//         BinanceCoinMAPIError::UpcomingMethod { msg } => {
//             eprintln!("Upcoming method: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidPriceSpreadThreshold { msg } => {
//             eprintln!("Invalid price spread threshold: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidPair { msg } => {
//             eprintln!("Invalid pair: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidTimeInterval { msg } => {
//             eprintln!("Invalid time interval: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyOrderPermission { msg } => {
//             eprintln!("Reduce only order permission: {}", msg);
//         }
//         BinanceCoinMAPIError::NoPlaceOrderPermission { msg } => {
//             eprintln!("No place order permission: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidContractType { msg } => {
//             eprintln!("Invalid contract type: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidClientTranIdLen { msg } => {
//             eprintln!("Invalid client transaction ID length: {}", msg);
//         }
//         BinanceCoinMAPIError::DuplicatedClientTranId { msg } => {
//             eprintln!("Duplicated client transaction ID: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyMarginCheckFailed { msg } => {
//             eprintln!("Reduce only margin check failed: {}", msg);
//         }
//         BinanceCoinMAPIError::MarketOrderReject { msg } => {
//             eprintln!("Market order reject: {}", msg);
//         }
//         BinanceCoinMAPIError::InvalidActivationPrice { msg } => {
//             eprintln!("Invalid activation price: {}", msg);
//         }
//         BinanceCoinMAPIError::QuantityExistsWithClosePosition { msg } => {
//             eprintln!("Quantity exists with close position: {}", msg);
//         }
//         BinanceCoinMAPIError::ReduceOnlyMustBeTrue { msg } => {
//             eprintln!("Reduce only must be true: {}", msg);
//         }
//         BinanceCoinMAPIError::OrderTypeCannotBeMkt { msg } => {
//             eprintln!("Order type cannot be market: {}", msg);
//         }
//         BinanceCoinMAPIError::StrategyInvalidTriggerPrice { msg } => {
//             eprintln!("Strategy invalid trigger price: {}", msg);
//         }
//         BinanceCoinMAPIError::IsolatedLeverageRejectWithPosition { msg } => {
//             eprintln!("Isolated leverage reject with position: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceHighterThanStopMultiplierUp { msg } => {
//             eprintln!("Price higher than stop multiplier up: {}", msg);
//         }
//         BinanceCoinMAPIError::PriceLowerThanStopMultiplierDown { msg } => {
//             eprintln!("Price lower than stop multiplier down: {}", msg);
//         }
//         BinanceCoinMAPIError::StopPriceHigherThanPriceMultiplierLimit { msg } => {
//             eprintln!("Stop price higher than price multiplier limit: {}", msg);
//         }
//         BinanceCoinMAPIError::StopPriceLowerThanPriceMultiplierLimit { msg } => {
//             eprintln!("Stop price lower than price multiplier limit: {}", msg);
//         }
//         BinanceCoinMAPIError::MinNotional { msg } => {
//             eprintln!("Min notional: {}", msg);
//         }
//         BinanceCoinMAPIError::CoolingOffPeriod { msg } => {
//             eprintln!("Cooling off period: {}", msg);
//         }
//         BinanceCoinMAPIError::AdjustLeverageKycFailed { msg } => {
//             eprintln!("Adjust leverage KYC failed: {}", msg);
//         }
//         BinanceCoinMAPIError::AdjustLeverageOneMonthFailed { msg } => {
//             eprintln!("Adjust leverage one month failed: {}", msg);
//         }
//         BinanceCoinMAPIError::LimitOrderOnly { msg } => {
//             eprintln!("Limit order only: {}", msg);
//         }
//         BinanceCoinMAPIError::SameOrder { msg } => {
//             eprintln!("Same order: {}", msg);
//         }
//         BinanceCoinMAPIError::ExceedMaxModifyOrderLimit { msg } => {
//             eprintln!("Exceed max modify order limit: {}", msg);
//         }
//         BinanceCoinMAPIError::MoveOrderNotAllowedSymbolReason { msg } => {
//             eprintln!("Move order not allowed symbol reason: {}", msg);
//         }
//         BinanceCoinMAPIError::AdjustLeverageXDaysFailed { msg } => {
//             eprintln!("Adjust leverage X days failed: {}", msg);
//         }
//         BinanceCoinMAPIError::AdjustLeverageKycLimit { msg } => {
//             eprintln!("Adjust leverage KYC limit: {}", msg);
//         }
//         BinanceCoinMAPIError::AdjustLeverageAccountSymbolFailed { msg } => {
//             eprintln!("Adjust leverage account symbol failed: {}", msg);
//         }
//         BinanceCoinMAPIError::MeInvalidTimestamp { msg } => {
//             eprintln!("ME invalid timestamp: {}", msg);
//         }
//         BinanceCoinMAPIError::UnmappedApiError { code, msg } => {
//             eprintln!("Unmapped API error (code: {}): {}", code, msg);
//         }
//     }
//     std::process::exit(1);
// }

fn create_client(prod: bool) -> Result<Arc<PrivateRestClient>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not set");

    let client = if !prod {
        PrivateRestClient::new(
            Box::new(SecretValue::new(SecretString::from(api_key))),
            Box::new(SecretValue::new(SecretString::from(api_secret))),
            "https://testnet.binancefuture.com".to_string(),
            RateLimiter::new(),
            reqwest::Client::new()
        )
    } else {
        PrivateRestClient::new(
            Box::new(SecretValue::new(SecretString::from(api_key))),
            Box::new(SecretValue::new(SecretString::from(api_secret))),
            "https://dapi.binance.com".to_string(),
            RateLimiter::new(),
            reqwest::Client::new()
        )
    };

    Ok(Arc::new(client))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = create_client(cli.prod)?;

    match cli.command {
        Commands::Account => {
            if let Err(e) = handle_account_command(client.clone()).await {
                if let Some(api_err) = e.downcast_ref::<ApiError>() {
                    match api_err {
                        // handle_api_error(api_err);
                        ApiError::RateLimitExceeded { .. } => eprintln!("Rate limit exceeded"),
                        _ => eprintln!("API Error: {}", api_err),
                    }
                }
                return Err(e.into());
            }
        }
        Commands::Trades { symbol, limit } => {
            if let Err(e) = handle_trades_command(client.clone(), symbol, limit).await {
                match &e {
                    Errors::ApiError(api_err) => {
                        match api_err {
                            ApiError::RateLimitExceeded { .. } => eprintln!("Rate limit exceeded"),
                            ApiError::BadSymbol { msg } => eprintln!("Bad symbol dsfsffdf: {}", msg),
                            _ => eprintln!("API Error: {}", api_err),
                        }
                    },
                    _ => eprintln!("Unexpected error: {}", e),
                }
                return Err(e.into());
            }
        }
        Commands::BatchOrder { symbol, side, order_type, quantity, price } => {
            if let Err(e) = handle_batch_order_command(client.clone(), symbol, side, order_type, quantity, price).await {
                if let Some(api_err) = e.downcast_ref::<ApiError>() {
                    //handle_api_error(api_err);
                }
                return Err(e);
            }
        }
        Commands::ExchangeInfo => {
            let public_client = PublicRestClient::new(
                "https://dapi.binance.com".to_string(),
                reqwest::Client::new(),
                RateLimiter::new(),
            );
            let public_client = Arc::new(public_client);
            if let Err(e) = commands::handle_exchange_info_command(public_client.clone()).await {
                if let Some(api_err) = e.downcast_ref::<ApiError>() {
                    //handle_api_error(api_err);
                }
                return Err(e);
            }
        }
    }

    Ok(())
} 