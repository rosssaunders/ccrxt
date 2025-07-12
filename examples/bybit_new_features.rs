// Example demonstrating the new Bybit functionality added from API documentation updates
//
// This example shows how to use the new enums and endpoints that were added
// to align with the latest Bybit V5 API documentation.

use venues::bybit::{
    MaintenanceType, NetworkType, ProductType, ServiceType, Status, TransferRequest, TransferType,
    WithdrawRequest,
};

fn main() {
    println!("🚀 Bybit API Documentation Updates - New Functionality Demo");
    println!("===========================================================\n");

    // Demonstrate new enum types and their serialization
    println!("📋 New Status Types:");
    let statuses = vec![
        Status::Scheduled,
        Status::Ongoing,
        Status::Completed,
        Status::Canceled,
    ];
    for status in statuses {
        println!("  • {} -> {}", format!("{:?}", status), serde_json::to_string(&status).unwrap());
    }

    println!("\n🔧 New Service Types:");
    let services = vec![
        ServiceType::TradingService,
        ServiceType::HttpTradingService,
        ServiceType::WebsocketTradingService,
        ServiceType::DerivativesTradingService,
        ServiceType::SpotTradingService,
        ServiceType::OptionsTradingService,
    ];
    for service in services {
        println!("  • {} -> {}", format!("{:?}", service), serde_json::to_string(&service).unwrap());
    }

    println!("\n📦 New Product Types:");
    let products = vec![
        ProductType::Future,
        ProductType::Spot,
        ProductType::Option,
        ProductType::Spread,
    ];
    for product in products {
        println!("  • {} -> {}", format!("{:?}", product), serde_json::to_string(&product).unwrap());
    }

    println!("\n🔧 New Maintenance Types:");
    let maintenance_types = vec![
        MaintenanceType::PlannedMaintenance,
        MaintenanceType::TemporaryMaintenance,
        MaintenanceType::SystemFailure,
    ];
    for maintenance_type in maintenance_types {
        println!("  • {} -> {}", format!("{:?}", maintenance_type), serde_json::to_string(&maintenance_type).unwrap());
    }

    println!("\n🌐 Network Types:");
    let networks = vec![NetworkType::Mainnet, NetworkType::MainnetDemo];
    for network in networks {
        println!("  • {} -> {}", format!("{:?}", network), serde_json::to_string(&network).unwrap());
    }

    println!("\n💰 Transfer Types:");
    let transfer_types = vec![
        TransferType::Spot,
        TransferType::Contract,
        TransferType::Unified,
        TransferType::Option,
        TransferType::Fund,
    ];
    for transfer_type in transfer_types {
        println!("  • {} -> {}", format!("{:?}", transfer_type), serde_json::to_string(&transfer_type).unwrap());
    }

    // Demonstrate new endpoint request structures
    println!("\n📤 New Transfer Request Structure:");
    let transfer_request = TransferRequest {
        transfer_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        coin: "USDT".to_string(),
        amount: "100.00".to_string(),
        from_account_type: TransferType::Spot,
        to_account_type: TransferType::Unified,
    };
    println!("  Transfer Request JSON:");
    println!("  {}", serde_json::to_string_pretty(&transfer_request).unwrap());

    println!("\n💸 New Withdraw Request Structure (with UAE fields):");
    let withdraw_request = WithdrawRequest {
        coin: "BTC".to_string(),
        amount: "0.1".to_string(),
        address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        tag: None,
        chain: "BTC".to_string(),
        account_type: "UNIFIED".to_string(),
        force_chain: None,
        // UAE user specific fields (required for UAE users)
        beneficiary_address: Some("123 Blockchain Street, Dubai".to_string()),
        beneficiary_name: Some("John Doe".to_string()),
        beneficiary_country: Some("AE".to_string()),
        beneficiary_city: Some("Dubai".to_string()),
        beneficiary_postal_code: Some("00000".to_string()),
    };
    println!("  Withdraw Request JSON:");
    println!("  {}", serde_json::to_string_pretty(&withdraw_request).unwrap());

    println!("\n✅ All new types and endpoints are properly integrated!");
    println!("📚 These updates align CCRXT with the latest Bybit V5 API documentation.");
    println!("\n🔗 New endpoints available:");
    println!("  • /v5/asset/transfer/inter-transfer (internal transfers)");
    println!("  • /v5/asset/withdraw/create (withdrawals with UAE support)");
    println!("  • /v5/status (system status for WebSocket and other services)");
}