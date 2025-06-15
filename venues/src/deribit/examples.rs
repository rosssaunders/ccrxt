//! # Deribit Private API Examples
//!
//! This module demonstrates how to use the Deribit private API,
//! specifically the withdraw functionality.

#[cfg(test)]
mod examples {
    use crate::deribit::{PrivateRestClient, WithdrawRequest};
    use rest::secrets::ExposableSecret;

    /// A simple implementation of ExposableSecret for demonstration
    #[derive(Clone)]
    struct DemoSecret {
        secret: String,
    }

    impl ExposableSecret for DemoSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    #[test]
    fn test_deribit_withdraw_usage_example() {
        // Example 1: Basic withdrawal request structure
        let basic_withdraw = WithdrawRequest {
            currency: "BTC".to_string(),
            address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount: 0.001,
            priority: None, // Uses default priority
        };

        let json = serde_json::to_string_pretty(&basic_withdraw).unwrap();
        println!("Basic withdrawal request JSON:\n{}", json);

        // Example 2: Withdrawal request with high priority
        let priority_withdraw = WithdrawRequest {
            currency: "ETH".to_string(),
            address: "0x742d35Cc6634C0532925a3b8D43C67C3b4BF9B7E".to_string(),
            amount: 0.5,
            priority: Some("high".to_string()),
        };

        let json = serde_json::to_string_pretty(&priority_withdraw).unwrap();
        println!("Priority withdrawal request JSON:\n{}", json);

        // Example 3: USDC withdrawal with very high priority
        let usdc_withdraw = WithdrawRequest {
            currency: "USDC".to_string(),
            address: "0x1234567890123456789012345678901234567890".to_string(),
            amount: 100.0,
            priority: Some("very_high".to_string()),
        };

        let json = serde_json::to_string_pretty(&usdc_withdraw).unwrap();
        println!("USDC withdrawal request JSON:\n{}", json);
    }

    #[test]
    fn test_deribit_client_creation_example() {
        // Example of how to create a Deribit private client
        let client_id = Box::new(DemoSecret {
            secret: "your_client_id".to_string(),
        }) as Box<dyn ExposableSecret>;
        
        let client_secret = Box::new(DemoSecret {
            secret: "your_client_secret".to_string(),
        }) as Box<dyn ExposableSecret>;

        let http_client = reqwest::Client::new();

        // Create the Deribit private REST client
        let _deribit_client = PrivateRestClient::new(
            client_id,
            client_secret,
            "https://www.deribit.com", // Production URL
            http_client,
        );

        println!("Deribit private client created successfully!");
        
        // In a real application, you would:
        // 1. Call client.authenticate().await to get an access token
        // 2. Call client.withdraw("BTC", "address", 0.001, Some("high")).await
    }

    #[test]
    fn test_supported_currencies_and_priorities() {
        // Demonstrate all supported currencies
        let currencies = vec!["BTC", "ETH", "USDC", "USDT", "EURR"];
        println!("Supported currencies:");
        for currency in currencies {
            println!("  - {}", currency);
        }

        // Demonstrate all supported priority levels
        let priorities = vec![
            "insane",
            "extreme_high",
            "very_high", 
            "high",
            "mid",
            "low",
            "very_low"
        ];
        println!("\nSupported priority levels:");
        for priority in priorities {
            println!("  - {}", priority);
        }
    }

    /// Example of a complete withdrawal workflow (without actual API calls)
    #[tokio::test]
    async fn test_withdrawal_workflow_example() {
        println!("=== Deribit Withdrawal Workflow Example ===");
        
        // Step 1: Create client credentials
        let client_id = Box::new(DemoSecret {
            secret: "demo_client_id".to_string(),
        }) as Box<dyn ExposableSecret>;
        
        let client_secret = Box::new(DemoSecret {
            secret: "demo_client_secret".to_string(),
        }) as Box<dyn ExposableSecret>;

        // Step 2: Create HTTP client and Deribit client
        let http_client = reqwest::Client::new();
        let _deribit_client = PrivateRestClient::new(
            client_id,
            client_secret,
            "https://test.deribit.com", // Test environment
            http_client,
        );

        println!("✓ Deribit client created");

        // Step 3: Authentication (would be done in real usage)
        println!("✓ Authentication step (would call authenticate() in real usage)");
        
        // Step 4: Prepare withdrawal parameters
        let currency = "BTC";
        let address = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
        let amount = 0.001;
        let priority = Some("high");

        println!("✓ Withdrawal parameters prepared:");
        println!("  Currency: {}", currency);
        println!("  Address: {}", address);
        println!("  Amount: {}", amount);
        println!("  Priority: {:?}", priority);

        // Step 5: Withdrawal call (would be done in real usage)
        println!("✓ Withdrawal call (would call withdraw() in real usage)");
        
        // In a real application:
        // let result = deribit_client.withdraw(currency, address, amount, priority).await;
        // match result {
        //     Ok(withdraw_result) => {
        //         println!("Withdrawal successful! ID: {}", withdraw_result.id);
        //         println!("State: {}", withdraw_result.state);
        //         println!("Fee: {}", withdraw_result.fee);
        //     }
        //     Err(error) => {
        //         eprintln!("Withdrawal failed: {}", error);
        //     }
        // }

        println!("=== Workflow Example Complete ===");
    }
}