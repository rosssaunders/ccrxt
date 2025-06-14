use crate::cryptocom::private::RestClient;
use rest::secrets::ExposableSecret;
use serde_json::json;

/// A plain text implementation of ExposableSecret for testing purposes.
#[derive(Clone)]
#[allow(dead_code)]
struct PlainTextSecret {
    secret: String,
}

impl ExposableSecret for PlainTextSecret {
    fn expose_secret(&self) -> String {
        self.secret.clone()
    }
}

impl PlainTextSecret {
    #[allow(dead_code)]
    fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[tokio::test]
async fn test_private_endpoints_compile() {
    // Test that all the new private endpoints compile and are accessible
    let api_key =
        Box::new(PlainTextSecret::new("test_key".to_string())) as Box<dyn ExposableSecret>;
    let api_secret =
        Box::new(PlainTextSecret::new("test_secret".to_string())) as Box<dyn ExposableSecret>;
    let client = reqwest::Client::new();

    let rest_client = RestClient::new(api_key, api_secret, "https://api.crypto.com", client);

    // Test that methods exist by verifying we can get function references to them
    // This proves they compile and are accessible without needing to call them
    let _ = RestClient::get_user_balance;
    let _ = RestClient::get_user_balance_history;
    let _ = RestClient::get_accounts;
    let _ = RestClient::create_subaccount_transfer;
    let _ = RestClient::get_subaccount_balances;
    let _ = RestClient::get_positions;
    let _ = RestClient::get_order_history;
    let _ = RestClient::get_trades;
    let _ = RestClient::get_transactions;

    // Wallet API methods
    let _ = RestClient::create_withdrawal;
    let _ = RestClient::get_currency_networks;
    let _ = RestClient::get_deposit_address;
    let _ = RestClient::get_deposit_history;
    let _ = RestClient::get_withdrawal_history;

    // Verify RestClient itself compiles
    let _ = &rest_client;

    println!("All private endpoint methods including Wallet API are accessible and properly typed");
}

#[test]
fn test_request_parameters_serialization() {
    // Test that all parameter structures serialize correctly
    use crate::cryptocom::private::rest::create_subaccount_transfer::CreateSubaccountTransferRequest;
    use crate::cryptocom::private::rest::get_accounts::GetAccountsRequest;
    use crate::cryptocom::private::rest::get_order_history::GetOrderHistoryRequest;
    use crate::cryptocom::private::rest::get_positions::GetPositionsRequest;
    use crate::cryptocom::private::rest::get_trades::GetTradesRequest;
    use crate::cryptocom::private::rest::get_transactions::GetTransactionsRequest;
    use crate::cryptocom::private::rest::user_balance_history::UserBalanceHistoryRequest;

    // Balance history params
    let balance_params = UserBalanceHistoryRequest {
        timeframe: Some("H1".to_string()),
        end_time: Some(1629478800000),
        limit: Some(10),
    };
    let json_value = serde_json::to_value(balance_params).unwrap();
    assert_eq!(json_value.get("timeframe").unwrap(), "H1");

    // Get accounts params
    let accounts_params = GetAccountsRequest {
        page_size: Some(30),
        page: Some(2),
    };
    let json_value = serde_json::to_value(accounts_params).unwrap();
    assert_eq!(json_value.get("page_size").unwrap(), 30);

    // Create subaccount transfer params
    let transfer_params = CreateSubaccountTransferRequest {
        from: "uuid1".to_string(),
        to: "uuid2".to_string(),
        currency: "USD".to_string(),
        amount: "100.00".to_string(),
    };
    let json_value = serde_json::to_value(transfer_params).unwrap();
    assert_eq!(json_value.get("currency").unwrap(), "USD");

    // Get positions params
    let position_params = GetPositionsRequest {
        instrument_name: Some("BTCUSD-PERP".to_string()),
    };
    let json_value = serde_json::to_value(position_params).unwrap();
    assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");

    // Get order history params
    let order_history_params = GetOrderHistoryRequest {
        instrument_name: Some("BTCUSD-PERP".to_string()),
        start_time: Some("1610905028000081486".to_string()),
        end_time: Some("1613570791058211357".to_string()),
        limit: Some(20),
    };
    let json_value = serde_json::to_value(order_history_params).unwrap();
    assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
    assert_eq!(json_value.get("limit").unwrap(), 20);

    // Get trades params
    let trades_params = GetTradesRequest {
        instrument_name: Some("BTCUSD-PERP".to_string()),
        start_time: Some("1619089031996081486".to_string()),
        end_time: Some("1619200052124211357".to_string()),
        limit: Some(20),
    };
    let json_value = serde_json::to_value(trades_params).unwrap();
    assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
    assert_eq!(json_value.get("limit").unwrap(), 20);

    // Get transactions params
    let transactions_params = GetTransactionsRequest {
        instrument_name: Some("BTCUSD-PERP".to_string()),
        journal_type: Some("TRADING".to_string()),
        start_time: Some("1619089031996081486".to_string()),
        end_time: Some("1619200052124211357".to_string()),
        limit: Some(20),
    };
    let json_value = serde_json::to_value(transactions_params).unwrap();
    assert_eq!(json_value.get("instrument_name").unwrap(), "BTCUSD-PERP");
    assert_eq!(json_value.get("journal_type").unwrap(), "TRADING");
    assert_eq!(json_value.get("limit").unwrap(), 20);
}

#[test]
fn test_response_structures_deserialization() {
    // Test that all response structures deserialize correctly from JSON
    use crate::cryptocom::private::rest::get_accounts::Account;
    use crate::cryptocom::private::rest::get_positions::Position;
    use crate::cryptocom::private::rest::user_balance::UserBalance;

    // Test UserBalance deserialization
    let balance_json = json!({
        "total_available_balance": "4721.05898582",
        "total_margin_balance": "7595.42571782",
        "total_initial_margin": "2874.36673202",
        "total_position_im": "486.31273202",
        "total_haircut": "2388.054",
        "total_maintenance_margin": "1437.18336601",
        "total_position_cost": "14517.54641301",
        "total_cash_balance": "7890.00320721",
        "total_collateral_value": "7651.18811483",
        "total_session_unrealized_pnl": "-55.76239701",
        "instrument_name": "USD",
        "total_session_realized_pnl": "0.00000000",
        "is_liquidating": false,
        "total_effective_leverage": "1.90401230",
        "position_limit": "3000000.00000000",
        "used_position_limit": "40674.69622001",
        "position_balances": []
    });
    let _balance: UserBalance = serde_json::from_value(balance_json).unwrap();

    // Test Account deserialization
    let account_json = json!({
        "uuid": "243d3f39-b193-4eb9-1d60-e98f2fc17707",
        "master_account_uuid": "291879ae-b769-4eb3-4d75-3366ebee7dd6",
        "enabled": true,
        "tradable": true,
        "name": "Test Account",
        "email": "test@crypto.com",
        "mobile_number": "",
        "country_code": "US",
        "address": "",
        "margin_access": "DEFAULT",
        "derivatives_access": "DISABLED",
        "create_time": 1620962543792_u64,
        "update_time": 1622019525960_u64,
        "two_fa_enabled": true,
        "kyc_level": "ADVANCED",
        "suspended": false,
        "terminated": false
    });
    let _account: Account = serde_json::from_value(account_json).unwrap();

    // Test Position deserialization
    let position_json = json!({
        "account_id": "858dbc8b-22fd-49fa-bff4-d342d98a8acb",
        "quantity": "-0.1984",
        "cost": "-10159.573500",
        "open_position_pnl": "-497.743736",
        "open_pos_cost": "-10159.352200",
        "session_pnl": "2.236145",
        "update_timestamp_ms": 1613552240770_u64,
        "instrument_name": "BTCUSD-PERP",
        "type": "PERPETUAL_SWAP"
    });
    let _position: Position = serde_json::from_value(position_json).unwrap();
}
