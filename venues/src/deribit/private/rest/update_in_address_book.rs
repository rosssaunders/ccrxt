use super::client::RestClient;
use crate::deribit::{RestResult, EndpointType};
use serde::{Deserialize, Serialize};

/// Request parameters for updating address book beneficiary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInAddressBookRequest {
    /// The currency symbol
    pub currency: Currency,
    /// Address book type
    #[serde(rename = "type")]
    pub address_type: AddressType,
    /// Address in currency format, it must be in address book
    pub address: String,
    /// Name of beneficiary VASP
    pub beneficiary_vasp_name: String,
    /// DID of beneficiary VASP
    pub beneficiary_vasp_did: String,
    /// Website of the beneficiary VASP (required if VASP not in known list)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_vasp_website: Option<String>,
    /// First name of beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_first_name: Option<String>,
    /// Last name of beneficiary (if beneficiary is a person)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_last_name: Option<String>,
    /// Beneficiary company name (if beneficiary is a company)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_company_name: Option<String>,
    /// Beneficiary address
    pub beneficiary_address: String,
    /// Indicates that the user agreed to share provided information with 3rd parties
    pub agreed: bool,
    /// The user confirms that he provided address belongs to him and he has access to it via an un-hosted wallet software
    pub personal: bool,
    /// Label of the address book entry
    pub label: String,
}

/// Currency types supported by Deribit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Btc,
    Eth,
    Steth,
    Ethw,
    Usdc,
    Usdt,
    Eurr,
    Matic,
    Sol,
    Xrp,
    Usyc,
    Paxg,
    Bnb,
    Usde,
}

/// Address book type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Transfer,
    Withdrawal,
    DepositSource,
}

/// Response for update in address book operation
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateInAddressBookResponse {
    /// The JSON-RPC version (2.0)
    pub jsonrpc: String,
    /// The id that was sent in the request
    pub id: i32,
    /// The result - should be "ok" for success
    pub result: String,
}

impl RestClient {
    /// Updates beneficiary information for an address in the address book
    ///
    /// This endpoint allows to provide beneficiary information for the address.
    /// The address must already exist in the address book.
    ///
    /// # Arguments
    /// * `request` - The update in address book parameters
    ///
    /// # Returns
    /// A confirmation response indicating success
    pub async fn update_in_address_book(
        &self,
        request: UpdateInAddressBookRequest,
    ) -> RestResult<UpdateInAddressBookResponse> {
        let params = serde_json::to_value(&request)
            .map_err(|e| crate::deribit::Errors::Error(format!("Serialization error: {}", e)))?;

        let response = self
            .send_request("private/update_in_address_book", params, EndpointType::NonMatchingEngine)
            .await?;

        // Extract the result from the JSON-RPC response
        let update_response: UpdateInAddressBookResponse = serde_json::from_value(response)
            .map_err(|e| crate::deribit::Errors::Error(format!("Deserialization error: {}", e)))?;

        Ok(update_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rest::secrets::ExposableSecret;
    use serde_json::json;

    /// A plain text implementation of ExposableSecret for testing purposes.
    #[derive(Clone)]
    struct PlainTextSecret {
        secret: String,
    }

    impl ExposableSecret for PlainTextSecret {
        fn expose_secret(&self) -> String {
            self.secret.clone()
        }
    }

    impl PlainTextSecret {
        fn new(secret: String) -> Self {
            Self { secret }
        }
    }

    #[test]
    fn test_update_in_address_book_request_structure() {
        let request = UpdateInAddressBookRequest {
            currency: Currency::Btc,
            address_type: AddressType::Withdrawal,
            address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            beneficiary_vasp_name: "Test VASP".to_string(),
            beneficiary_vasp_did: "did:example:123456789".to_string(),
            beneficiary_vasp_website: Some("https://example.com".to_string()),
            beneficiary_first_name: Some("John".to_string()),
            beneficiary_last_name: Some("Doe".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "123 Main St, City, Country".to_string(),
            agreed: true,
            personal: true,
            label: "Test Address".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        
        assert_eq!(serialized["currency"], "BTC");
        assert_eq!(serialized["type"], "withdrawal");
        assert_eq!(serialized["address"], "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh");
        assert_eq!(serialized["beneficiary_vasp_name"], "Test VASP");
        assert_eq!(serialized["beneficiary_vasp_did"], "did:example:123456789");
        assert_eq!(serialized["beneficiary_vasp_website"], "https://example.com");
        assert_eq!(serialized["beneficiary_first_name"], "John");
        assert_eq!(serialized["beneficiary_last_name"], "Doe");
        assert_eq!(serialized["beneficiary_address"], "123 Main St, City, Country");
        assert_eq!(serialized["agreed"], true);
        assert_eq!(serialized["personal"], true);
        assert_eq!(serialized["label"], "Test Address");
        
        // Ensure company name is not serialized when None
        assert!(serialized.get("beneficiary_company_name").is_none());
    }

    #[test]
    fn test_update_in_address_book_response_structure() {
        let response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": "ok"
        });

        let response: UpdateInAddressBookResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert_eq!(response.result, "ok");
    }

    #[test]
    fn test_currency_serialization() {
        assert_eq!(serde_json::to_string(&Currency::Btc).unwrap(), "\"BTC\"");
        assert_eq!(serde_json::to_string(&Currency::Eth).unwrap(), "\"ETH\"");
        assert_eq!(serde_json::to_string(&Currency::Usdc).unwrap(), "\"USDC\"");
        assert_eq!(serde_json::to_string(&Currency::Sol).unwrap(), "\"SOL\"");
    }

    #[test]
    fn test_address_type_serialization() {
        assert_eq!(serde_json::to_string(&AddressType::Transfer).unwrap(), "\"transfer\"");
        assert_eq!(serde_json::to_string(&AddressType::Withdrawal).unwrap(), "\"withdrawal\"");
        assert_eq!(serde_json::to_string(&AddressType::DepositSource).unwrap(), "\"deposit_source\"");
    }

    #[test]
    fn test_minimal_request() {
        let request = UpdateInAddressBookRequest {
            currency: Currency::Usdt,
            address_type: AddressType::Transfer,
            address: "TUEbNxaxqTR8BQvGqWjCjUeDnXgoDYMUd7".to_string(),
            beneficiary_vasp_name: "Minimal VASP".to_string(),
            beneficiary_vasp_did: "did:minimal:987654321".to_string(),
            beneficiary_vasp_website: None,
            beneficiary_first_name: None,
            beneficiary_last_name: None,
            beneficiary_company_name: Some("ACME Corp".to_string()),
            beneficiary_address: "456 Business Ave, Tech City".to_string(),
            agreed: true,
            personal: false,
            label: "Corporate Transfer".to_string(),
        };

        let serialized = serde_json::to_value(&request).unwrap();
        
        // Check required fields
        assert_eq!(serialized["currency"], "USDT");
        assert_eq!(serialized["type"], "transfer");
        assert_eq!(serialized["beneficiary_company_name"], "ACME Corp");
        assert_eq!(serialized["personal"], false);
        
        // Check optional fields are not present when None
        assert!(serialized.get("beneficiary_vasp_website").is_none());
        assert!(serialized.get("beneficiary_first_name").is_none());
        assert!(serialized.get("beneficiary_last_name").is_none());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original_request = UpdateInAddressBookRequest {
            currency: Currency::Eth,
            address_type: AddressType::DepositSource,
            address: "0x742d35cc6639C0532fEb62B8F94A4bBb3b0dD17f".to_string(),
            beneficiary_vasp_name: "Test Exchange".to_string(),
            beneficiary_vasp_did: "did:test:roundtrip".to_string(),
            beneficiary_vasp_website: Some("https://test-exchange.com".to_string()),
            beneficiary_first_name: Some("Alice".to_string()),
            beneficiary_last_name: Some("Smith".to_string()),
            beneficiary_company_name: None,
            beneficiary_address: "789 Crypto Lane, DeFi District".to_string(),
            agreed: true,
            personal: true,
            label: "Roundtrip Test".to_string(),
        };

        let serialized = serde_json::to_value(&original_request).unwrap();
        let deserialized: UpdateInAddressBookRequest = serde_json::from_value(serialized).unwrap();

        // Compare key fields
        assert_eq!(format!("{:?}", original_request.currency), format!("{:?}", deserialized.currency));
        assert_eq!(format!("{:?}", original_request.address_type), format!("{:?}", deserialized.address_type));
        assert_eq!(original_request.address, deserialized.address);
        assert_eq!(original_request.beneficiary_vasp_name, deserialized.beneficiary_vasp_name);
        assert_eq!(original_request.agreed, deserialized.agreed);
        assert_eq!(original_request.personal, deserialized.personal);
    }
}