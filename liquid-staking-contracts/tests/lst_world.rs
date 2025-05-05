use std::{collections::HashMap, ops::Deref, str::FromStr};

use cucumber::{Parameter, World};
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef, StakedCSPRInitArgs};
use odra::{
    casper_types::{PublicKey, U256, U512},
    host::Deployer,
};
use odra_bdd::{bdd_env::BDDEnv, types::token_amount::TokenAmount};

pub const WORLD_MIN_STAKE: u128 = 500_000_000_000;

#[derive(World)]
pub struct LSTWorld {
    pub env: BDDEnv,
    pub token: StakedCSPRHostRef,
    // (entrypoint, pool state)
    pub pool_state: Vec<(String, HashMap<PublicKey, U512>)>,
}

impl Default for LSTWorld {
    fn default() -> Self {
        let env = BDDEnv::new(odra_test::env());
        let validator_address = env.env().get_validator(0);
        let token = StakedCSPR::deploy(
            env.env(),
            StakedCSPRInitArgs {
                validator_address,
                claim_time: env.env().auction_delay() * 8,
                fee_percentage: 1000.into(),
                min_stake: U512::from(WORLD_MIN_STAKE),
            },
        );
        Self {
            env,
            token,
            pool_state: vec![],
        }
    }
}

impl LSTWorld {
    pub fn update_pool_state(&mut self, entrypoint: &str) {
        let validators = self.token.get_validators();
        let pool_state = validators
            .iter()
            .map(|validator| {
                (
                    validator.clone(),
                    self.token.get_validator_stake(validator.clone()),
                )
            })
            .collect();
        self.pool_state.push((entrypoint.to_string(), pool_state));
    }

    pub fn current_stakes(&self) -> HashMap<PublicKey, U512> {
        // Let's collect current validator stakes
        let validators = self.token.get_validators();
        let mut current_stakes = HashMap::new();
        for validator in validators {
            current_stakes.insert(validator.clone(), self.token.get_validator_stake(validator));
        }
        current_stakes
    }
}

impl std::fmt::Debug for LSTWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LSTWorld")
    }
}

#[derive(Debug, Clone, Copy, Parameter)]
#[param(regex = r"\d+(\.\d+)?", name = "scspr")]
pub struct StakedCSPRAmount(TokenAmount<9>);

impl Deref for StakedCSPRAmount {
    type Target = TokenAmount<9>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for StakedCSPRAmount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TokenAmount::<9>::from_str(s).map(StakedCSPRAmount)
    }
}

impl From<U256> for StakedCSPRAmount {
    fn from(amount: U256) -> Self {
        StakedCSPRAmount(TokenAmount::<9>::from(amount))
    }
}

impl From<&U256> for StakedCSPRAmount {
    fn from(amount: &U256) -> Self {
        StakedCSPRAmount(TokenAmount::<9>::from(amount))
    }
}

impl PartialEq for StakedCSPRAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
