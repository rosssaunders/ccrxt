use serde::Serialize;

use super::{RestClient, RestResult};

const EARN_UNI_LENDS_ENDPOINT: &str = "/earn/uni/lends";

/// Request to amend user lending information.
///
/// Matches Gate.io API: PATCH /earn/uni/lends
#[derive(Debug, Clone, Serialize, Default)]
pub struct ModifyLoanRequest {
    /// Currency name (e.g., "AE"). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Minimum interest rate (hourly). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_rate: Option<String>,
}

impl RestClient {
    /// Amend user lending information (PATCH /earn/uni/lends)
    ///
    /// [docs](https://www.gate.io/docs/developers/apiv4/en/#amend-user-lending-information)
    ///
    /// Currently only supports amending minimum interest rate (hourly).
    ///
    /// Rate limit: per venue config
    ///
    /// # Arguments
    /// * `request` - The amend request parameters
    ///
    /// # Returns
    /// Empty response - success indicated by HTTP status 204
    pub async fn amend_user_lending_information(
        &self,
        request: ModifyLoanRequest,
    ) -> RestResult<()> {
        // verb-specific function (PATCH)
        self.send_patch_request::<(), _>(EARN_UNI_LENDS_ENDPOINT, Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify_loan_serializes_optional_fields() {
        let r = ModifyLoanRequest {
            currency: Some("AE".into()),
            min_rate: Some("0.0001".into()),
        };
        let js = serde_json::to_string(&r).expect("serialize");
        assert!(js.contains("AE"));
        assert!(js.contains("0.0001"));
    }
}
