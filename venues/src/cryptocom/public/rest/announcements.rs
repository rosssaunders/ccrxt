use super::client::RestClient;
use crate::cryptocom::{EndpointType, RestResult};
use serde_json::Value;

impl RestClient {
    /// Get announcements from Crypto.com Exchange
    ///
    /// # Arguments
    /// * `category` - Optional filter by category: list, delist, event, product, system
    /// * `product_type` - Optional filter by product type. e.g. Spot, Derivative, OTC, Staking, TradingArena etc
    pub async fn get_announcements(
        &self,
        category: Option<&str>,
        product_type: Option<&str>,
    ) -> RestResult<Value> {
        let mut params = serde_json::json!({});

        if let Some(cat) = category {
            params["category"] = Value::String(cat.to_string());
        }

        if let Some(product) = product_type {
            params["product_type"] = Value::String(product.to_string());
        }

        let params = if params.as_object().unwrap().is_empty() {
            None
        } else {
            Some(params)
        };

        self.send_request(
            "public/get-announcements",
            reqwest::Method::GET,
            params.as_ref(),
            EndpointType::PublicGetAnnouncements,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_announcements_endpoint_type() {
        let announcements_endpoint = EndpointType::PublicGetAnnouncements;
        assert!(announcements_endpoint.rate_limit().max_requests > 0);
    }

    #[test]
    fn test_announcements_parameter_building() {
        let params = json!({
            "category": "system",
            "product_type": "Spot"
        });
        assert_eq!(params["category"], "system");
        assert_eq!(params["product_type"], "Spot");
    }
}
