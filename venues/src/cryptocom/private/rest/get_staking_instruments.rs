use super::client::RestClient;
use crate::cryptocom::RestResult;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Additional reward information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalReward {
    /// Additional reward instrument name
    pub reward_inst_name: String,
}

/// Staking instrument information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInstrument {
    /// Staking instrument name, e.g. SOL.staked
    pub instrument_name: String,
    /// Underlying instrument name, e.g. SOL
    pub underlying_inst_name: String,
    /// Reward instrument name, e.g. SOL.staked
    pub reward_inst_name: String,
    /// Disabled stake - true or false
    pub out_of_stock: bool,
    /// Disabled unstake - true or false
    pub block_unstake: bool,
    /// Estimated rewards
    pub est_rewards: String,
    /// Estimated rewards unit - APR or APY
    pub apr_y: String,
    /// Minimum stake amount
    pub min_stake_amt: String,
    /// Estimated reward frequency (day)
    pub reward_frequency: String,
    /// Estimated lock up period (day)
    pub lock_up_period: String,
    /// Is reward compounded - true or false
    pub is_compound_reward: bool,
    /// Is pre stake charge applied - true or false
    pub pre_stake_charge_enable: bool,
    /// Pre stake charge rate in basis point
    pub pre_stake_charge_rate_in_bps: String,
    /// Is restaked instrument - true or false
    pub is_restaked: bool,
    /// Additional rewards
    pub additional_rewards: Vec<AdditionalReward>,
}

/// Response for get staking instruments endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStakingInstrumentsResponse {
    /// Array of staking instruments data
    pub data: Vec<StakingInstrument>,
}

impl RestClient {
    /// Get staking instruments information
    ///
    /// Returns information about available staking instruments including rates, limits, and conditions.
    ///
    /// See: <https://exchange-docs.crypto.com/exchange/index.html#private-staking-get-staking-instruments>
    ///
    /// Rate limit: 50 requests per second
    ///
    /// # Returns
    /// Staking instruments information including estimated rewards, minimum amounts, and other details
    #[allow(clippy::indexing_slicing)] // Safe: adding optional keys to JSON object
    pub async fn get_staking_instruments(&self) -> RestResult<Value> {
        let nonce = chrono::Utc::now().timestamp_millis() as u64;
        let id = 1;
        let params = json!({});

        let signature = self.sign_request(
            "private/staking/get-staking-instruments",
            id,
            &params,
            nonce,
        )?;

        let request_body = json!({
            "id": id,
            "method": "private/staking/get-staking-instruments",
            "params": params,
            "nonce": nonce,
            "sig": signature,
            "api_key": self.api_key.expose_secret()
        });

        let response = self
            .client
            .post(format!(
                "{}/v1/private/staking/get-staking-instruments",
                self.base_url
            ))
            .json(&request_body)
            .send()
            .await?;

        let result: Value = response.json().await?;
        Ok(result)
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
    fn test_additional_reward_structure() {
        let additional_reward_json = json!({
            "reward_inst_name": "USD_Stable_Coin"
        });

        let additional_reward: AdditionalReward =
            serde_json::from_value(additional_reward_json).unwrap();
        assert_eq!(additional_reward.reward_inst_name, "USD_Stable_Coin");
    }

    #[test]
    fn test_staking_instrument_structure() {
        let instrument_json = json!({
            "instrument_name": "SOL.staked",
            "underlying_inst_name": "SOL",
            "reward_inst_name": "SOL.staked",
            "out_of_stock": false,
            "block_unstake": false,
            "est_rewards": "0.0661",
            "apr_y": "APR",
            "min_stake_amt": "0.00000001",
            "reward_frequency": "2.5",
            "lock_up_period": "5",
            "is_compound_reward": true,
            "pre_stake_charge_enable": false,
            "pre_stake_charge_rate_in_bps": "0",
            "is_restaked": false,
            "additional_rewards": []
        });

        let instrument: StakingInstrument = serde_json::from_value(instrument_json).unwrap();
        assert_eq!(instrument.instrument_name, "SOL.staked");
        assert_eq!(instrument.underlying_inst_name, "SOL");
        assert!(!instrument.out_of_stock);
        assert!(instrument.is_compound_reward);
        assert_eq!(instrument.additional_rewards.len(), 0);
    }

    #[test]
    fn test_staking_instrument_with_additional_rewards() {
        let instrument_json = json!({
            "instrument_name": "DYDX.staked",
            "underlying_inst_name": "DYDX",
            "reward_inst_name": "DYDX",
            "out_of_stock": false,
            "block_unstake": false,
            "est_rewards": "0.05",
            "apr_y": "APR",
            "min_stake_amt": "0.00000001",
            "reward_frequency": "1",
            "lock_up_period": "31",
            "is_compound_reward": false,
            "pre_stake_charge_enable": false,
            "pre_stake_charge_rate_in_bps": "0",
            "is_restaked": false,
            "additional_rewards": [
                {
                    "reward_inst_name": "USD_Stable_Coin"
                }
            ]
        });

        let instrument: StakingInstrument = serde_json::from_value(instrument_json).unwrap();
        assert_eq!(instrument.instrument_name, "DYDX.staked");
        assert_eq!(instrument.underlying_inst_name, "DYDX");
        assert!(!instrument.is_compound_reward);
        assert_eq!(instrument.additional_rewards.len(), 1);
        assert_eq!(
            instrument.additional_rewards[0].reward_inst_name,
            "USD_Stable_Coin"
        );
    }

    #[test]
    fn test_get_staking_instruments_response_structure() {
        let response_json = json!({
            "data": [
                {
                    "instrument_name": "SOL.staked",
                    "underlying_inst_name": "SOL",
                    "reward_inst_name": "SOL.staked",
                    "out_of_stock": false,
                    "block_unstake": false,
                    "est_rewards": "0.0661",
                    "apr_y": "APR",
                    "min_stake_amt": "0.00000001",
                    "reward_frequency": "2.5",
                    "lock_up_period": "5",
                    "is_compound_reward": true,
                    "pre_stake_charge_enable": false,
                    "pre_stake_charge_rate_in_bps": "0",
                    "is_restaked": false,
                    "additional_rewards": []
                },
                {
                    "instrument_name": "DYDX.staked",
                    "underlying_inst_name": "DYDX",
                    "reward_inst_name": "DYDX",
                    "out_of_stock": false,
                    "block_unstake": false,
                    "est_rewards": "0.05",
                    "apr_y": "APR",
                    "min_stake_amt": "0.00000001",
                    "reward_frequency": "1",
                    "lock_up_period": "31",
                    "is_compound_reward": false,
                    "pre_stake_charge_enable": false,
                    "pre_stake_charge_rate_in_bps": "0",
                    "is_restaked": false,
                    "additional_rewards": [
                        {
                            "reward_inst_name": "USD_Stable_Coin"
                        }
                    ]
                }
            ]
        });

        let response: GetStakingInstrumentsResponse =
            serde_json::from_value(response_json).unwrap();
        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].instrument_name, "SOL.staked");
        assert_eq!(response.data[1].instrument_name, "DYDX.staked");
        assert_eq!(response.data[1].additional_rewards.len(), 1);
    }
}
