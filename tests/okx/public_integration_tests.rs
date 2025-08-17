//! Integration tests for OKX public REST API endpoints

use venues::okx::{
    Bar, BarSize, ConvertContractCoinRequest, GetDeliveryExerciseHistoryRequest,
    GetDiscountRateInterestFreeQuotaRequest, GetEstimatedPriceRequest,
    GetEstimatedSettlementInfoRequest, GetFundingRateHistoryRequest, GetFundingRateRequest,
    GetHistoryIndexCandlesRequest, GetHistoryMarkPriceCandlesRequest, GetIndexCandlesRequest,
    GetIndexComponentsRequest, GetIndexTickersRequest, GetInstrumentTickBandsRequest,
    GetInstrumentsRequest, GetInsuranceFundRequest, GetMarkPriceCandlesHistoryRequest,
    GetMarkPriceCandlesRequest, GetMarkPriceRequest, GetOpenInterestRequest, GetOptSummaryRequest,
    GetPositionTiersRequest, GetPremiumHistoryRequest, GetPriceLimitRequest,
    GetSettlementHistoryRequest, GetUnderlyingRequest, InstrumentState, InstrumentType,
    PublicRestClient, RateLimiter, TickBandInstrumentType,
};

/// Helper function to create a test client with rate limiting
fn create_public_test_client() -> PublicRestClient {
    let base_url = "https://www.okx.com".to_string();
    let client = reqwest::Client::new();
    let rate_limiter = RateLimiter::new();
    PublicRestClient::new(base_url, client, rate_limiter)
}

#[tokio::test]
async fn test_get_time() {
    let client = create_public_test_client();

    let result = client.get_time().await;
    assert!(
        result.is_ok(),
        "get_time should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!("Current server time: {:?}", first.ts);
    }
}

#[tokio::test]
async fn test_get_instruments() {
    let client = create_public_test_client();
    let request = GetInstrumentsRequest {
        inst_type: InstrumentType::Spot,
        underlying: None,
        inst_family: None,
        inst_id: None,
    };

    let result = client.get_instruments(request).await;
    assert!(
        result.is_ok(),
        "get_instruments should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} spot instruments", response.data.len());
}

#[tokio::test]
async fn test_get_mark_price() {
    let client = create_public_test_client();
    let request = GetMarkPriceRequest {
        inst_type: "SWAP".to_string(),
        uly: None,
        inst_family: None,
        inst_id: Some("BTC-USD-SWAP".to_string()),
    };

    let result = client.get_mark_price(&request).await;
    assert!(
        result.is_ok(),
        "get_mark_price should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!("Mark price for BTC-USD-SWAP: {:?}", first.mark_px);
    }
}

#[tokio::test]
async fn test_get_funding_rate() {
    let client = create_public_test_client();
    let request = GetFundingRateRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
    };

    let result = client.get_funding_rate(&request).await;
    assert!(
        result.is_ok(),
        "get_funding_rate should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!("Funding rate for BTC-USD-SWAP: {:?}", first.funding_rate);
    }
}

#[tokio::test]
async fn test_get_funding_rate_history() {
    let client = create_public_test_client();
    let request = GetFundingRateHistoryRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        before: None,
        after: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_funding_rate_history(&request).await;
    assert!(
        result.is_ok(),
        "get_funding_rate_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} funding rate history records", response.data.len());
}

#[tokio::test]
async fn test_get_open_interest() {
    let client = create_public_test_client();
    let request = GetOpenInterestRequest {
        inst_type: InstrumentType::Swap,
        underlying: None,
        inst_family: None,
        inst_id: Some("BTC-USD-SWAP".to_string()),
    };

    let result = client.get_open_interest(request).await;
    assert!(
        result.is_ok(),
        "get_open_interest should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!("Open interest for BTC-USD-SWAP: {:?}", first.oi);
    }
}

#[tokio::test]
async fn test_get_price_limit() {
    let client = create_public_test_client();
    let request = GetPriceLimitRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
    };

    let result = client.get_price_limit(request).await;
    assert!(
        result.is_ok(),
        "get_price_limit should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!(
            "Price limits for BTC-USD-SWAP - Buy: {:?}, Sell: {:?}",
            first.buy_lmt, first.sell_lmt
        );
    }
}

#[tokio::test]
async fn test_get_opt_summary() {
    let client = create_public_test_client();
    let request = GetOptSummaryRequest {
        underlying: Some("BTC-USD".to_string()),
        exp_time: None,
        inst_family: None,
    };

    let result = client.get_opt_summary(request).await;
    assert!(
        result.is_ok(),
        "get_opt_summary should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} option summaries", response.data.len());
}

#[tokio::test]
async fn test_get_estimated_price() {
    let client = create_public_test_client();
    // Fetch a live instrument ID for a futures contract
    let instruments_req = GetInstrumentsRequest {
        inst_type: InstrumentType::Futures,
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        inst_id: None,
    };
    let instruments_resp = client
        .get_instruments(instruments_req)
        .await
        .expect("get_instruments failed");
    assert_eq!(instruments_resp.code, "0");
    // find a live instrument
    let inst_id = instruments_resp
        .data
        .iter()
        .find(|inst| inst.state == InstrumentState::Live)
        .map(|inst| inst.inst_id.clone())
        .expect("No live futures instrument found");

    let request = GetEstimatedPriceRequest { inst_id };

    let result = client.get_estimated_price(request).await;

    // This endpoint might return empty data if the instrument is not near settlement
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, "0");
        if let Some(first) = response.data.first() {
            println!("Estimated price: {:?}", first.settle_px);
        } else {
            println!("No estimated price data available for this instrument");
        }
    } else {
        println!(
            "Estimated price endpoint returned error: {:?}",
            result.err()
        );
    }
}

#[tokio::test]
async fn test_get_estimated_settlement_info() {
    let client = create_public_test_client();
    // Fetch a live instrument ID for a futures contract
    let instruments_req = GetInstrumentsRequest {
        inst_type: InstrumentType::Futures,
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        inst_id: None,
    };
    let instruments_resp = client
        .get_instruments(instruments_req)
        .await
        .expect("get_instruments failed");
    assert_eq!(instruments_resp.code, "0");
    let inst_id = instruments_resp
        .data
        .iter()
        .find(|inst| inst.state == InstrumentState::Live)
        .map(|inst| inst.inst_id.clone())
        .expect("No live futures instrument found");

    let request = GetEstimatedSettlementInfoRequest { inst_id };

    let result = client.get_estimated_settlement_info(&request).await;

    // This endpoint might return empty data if the instrument is not near settlement
    // (only has data one hour before settlement)
    if result.is_ok() {
        let response = result.unwrap();
        assert_eq!(response.code, "0");
        if let Some(first) = response.data.first() {
            println!(
                "Estimated settlement info: inst_id={}, est_settle_px={}, next_settle_time= {}",
                first.inst_id, first.est_settle_px, first.next_settle_time
            );
        } else {
            println!(
                "No estimated settlement info available for this instrument (likely not near settlement time)"
            );
        }
    } else {
        println!(
            "Estimated settlement info endpoint returned error: {:?}",
            result.err()
        );
    }
}

#[tokio::test]
async fn test_exchange_rate() {
    let client = create_public_test_client();

    let result = client.get_exchange_rate().await;
    assert!(
        result.is_ok(),
        "exchange_rate should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} exchange rates", response.data.len());
    for rate in &response.data {
        println!("USD/CNY exchange rate: {}", rate.usd_cny);
    }
}

#[tokio::test]
async fn test_get_index_tickers() {
    let client = create_public_test_client();
    let request = GetIndexTickersRequest {
        quote_ccy: None,
        inst_id: Some("BTC-USD".to_string()),
    };

    let result = client.get_index_tickers(Some(request)).await;
    assert!(
        result.is_ok(),
        "get_index_tickers should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    if let Some(first) = response.data.first() {
        println!("Index ticker for BTC-USD: {:?}", first.idx_px);
    }
}

#[tokio::test]
async fn test_get_index_candles() {
    let client = create_public_test_client();
    let request = GetIndexCandlesRequest {
        inst_id: "BTC-USD".to_string(),
        after: None,
        before: None,
        bar: Some(Bar::M1),
        limit: Some("10".to_string()),
    };

    let result = client.get_index_candles(request).await;
    assert!(
        result.is_ok(),
        "get_index_candles should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} index candles", response.data.len());
}

#[tokio::test]
async fn test_get_history_index_candles() {
    let client = create_public_test_client();
    let request = GetHistoryIndexCandlesRequest {
        inst_id: "BTC-USD".to_string(),
        after: None,
        before: None,
        bar: Some(Bar::H1),
        limit: Some("10".to_string()),
    };

    let result = client.get_history_index_candles(request).await;
    assert!(
        result.is_ok(),
        "get_history_index_candles should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} historical index candles", response.data.len());
}

#[tokio::test]
async fn test_get_mark_price_candles() {
    let client = create_public_test_client();
    let request = GetMarkPriceCandlesRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: None,
        before: None,
        bar: Some("1m".to_string()),
        limit: Some("10".to_string()),
    };

    let result = client.get_mark_price_candles(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_candles should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} mark price candles", response.data.len());
}

#[tokio::test]
async fn test_get_history_mark_price_candles() {
    let client = create_public_test_client();
    let request = GetHistoryMarkPriceCandlesRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: None,
        before: None,
        bar: Some(BarSize::OneHour),
        limit: Some("10".to_string()),
    };

    let result = client.get_history_mark_price_candles(request).await;
    assert!(
        result.is_ok(),
        "get_history_mark_price_candles should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!(
        "Found {} historical mark price candles",
        response.data.len()
    );
}

#[tokio::test]
async fn test_get_mark_price_candles_history() {
    let client = create_public_test_client();
    let request = GetMarkPriceCandlesHistoryRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: None,
        before: None,
        bar: Some("1D".to_string()),
        limit: Some("10".to_string()),
    };

    let result = client.get_mark_price_candles_history(request).await;
    assert!(
        result.is_ok(),
        "get_mark_price_candles_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} mark price candles history", response.data.len());
}

#[tokio::test]
async fn test_get_underlying() {
    let client = create_public_test_client();
    let request = GetUnderlyingRequest {
        inst_type: InstrumentType::Futures,
    };

    let result = client.get_underlying(request).await;
    assert!(
        result.is_ok(),
        "get_underlying should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} underlying assets", response.data.len());
}

#[tokio::test]
async fn test_get_insurance_fund() {
    let client = create_public_test_client();
    let request = GetInsuranceFundRequest {
        inst_type: InstrumentType::Swap,
        fund_type: None,
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        currency: Some("BTC".to_string()),
        before: None,
        after: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_insurance_fund(request).await;
    assert!(
        result.is_ok(),
        "get_insurance_fund should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} insurance fund records", response.data.len());

    for (i, record) in response.data.iter().enumerate() {
        println!("Insurance fund record {}: {:#?}", i + 1, record);
    }
}

#[tokio::test]
async fn test_convert_contract_coin() {
    let client = create_public_test_client();
    let request = ConvertContractCoinRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        px: Some("50000".to_string()),
        sz: "1".to_string(),
        convert_type: Some("1".to_string()),
        op_type: None,
        unit: None,
    };

    let result = client.convert_contract_coin(request).await;
    assert!(
        result.is_ok(),
        "convert_contract_coin should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Converted amount: {:?}", response.data[0].sz);
}

#[tokio::test]
async fn test_get_index_components() {
    let client = create_public_test_client();
    let request = GetIndexComponentsRequest {
        index: "BTC-USD".to_string(),
    };

    let result = client.get_index_components(&request).await;
    assert!(
        result.is_ok(),
        "get_index_components should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    // data is Vec<IndexComponentData>
    println!(
        "Found {} index components",
        response
            .data
            .first()
            .map(|d| d.components.len())
            .unwrap_or(0)
    );
}

#[tokio::test]
async fn test_get_premium_history() {
    let client = create_public_test_client();
    let request = GetPremiumHistoryRequest {
        inst_id: "BTC-USD-SWAP".to_string(),
        after: None,
        before: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_premium_history(&request).await;
    assert!(
        result.is_ok(),
        "get_premium_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} premium history records", response.data.len());
}

#[tokio::test]
async fn test_get_instrument_tick_bands() {
    let client = create_public_test_client();
    let request = GetInstrumentTickBandsRequest {
        inst_type: TickBandInstrumentType::Option,
        inst_family: None,
    };

    let result = client.get_instrument_tick_bands(request).await;
    assert!(
        result.is_ok(),
        "get_instrument_tick_bands should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} instrument tick bands", response.data.len());
}

#[tokio::test]
async fn test_get_delivery_exercise_history() {
    let client = create_public_test_client();
    let request = GetDeliveryExerciseHistoryRequest {
        inst_type: InstrumentType::Option,
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        after: None,
        before: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_delivery_exercise_history(&request).await;
    assert!(
        result.is_ok(),
        "get_delivery_exercise_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!(
        "Found {} delivery/exercise history records",
        response.data.len()
    );
}

#[tokio::test]
async fn test_get_position_tiers() {
    let client = create_public_test_client();
    let request = GetPositionTiersRequest {
        inst_type: InstrumentType::Swap,
        td_mode: "cross".to_string(),
        underlying: Some("BTC-USD".to_string()),
        inst_family: None,
        inst_id: Some("BTC-USD-SWAP".to_string()),
        ccy: None,
        tier: None,
    };

    let result = client.get_position_tiers(request).await;
    assert!(
        result.is_ok(),
        "get_position_tiers should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} position tiers", response.data.len());
}

#[tokio::test]
async fn test_get_interest_rate_loan_quota() {
    let client = create_public_test_client();
    let result = client.get_interest_rate_loan_quota().await;
    assert!(
        result.is_ok(),
        "get_interest_rate_loan_quota should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!(
        "Found {} interest rate loan quota records",
        response.data.len()
    );
}

#[tokio::test]
async fn test_get_settlement_history() {
    let client = create_public_test_client();
    let request = GetSettlementHistoryRequest {
        inst_family: "BTC-USD".to_string(),
        after: None,
        before: None,
        limit: Some("10".to_string()),
    };

    let result = client.get_settlement_history(&request).await;
    assert!(
        result.is_ok(),
        "get_settlement_history should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    println!("Found {} settlement history records", response.data.len());
}

#[tokio::test]
async fn test_get_discount_rate_interest_free_quota() {
    let client = create_public_test_client();
    let request = GetDiscountRateInterestFreeQuotaRequest {
        ccy: None,
        discount_lv: None,
    };

    let result = client.get_discount_rate_interest_free_quota(&request).await;
    assert!(
        result.is_ok(),
        "get_discount_rate_interest_free_quota should succeed: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.code, "0");
    assert!(!response.data.is_empty());
    println!("Found {} discount rate records", response.data.len());
}
