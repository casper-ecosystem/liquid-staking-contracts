use std::{ops::Deref, str::FromStr};

use cucumber::{Parameter, World};
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef, StakedCSPRInitArgs};
use odra::{casper_types::U256, host::Deployer};
use odra_bdd::{bdd_env::BDDEnv, types::token_amount::TokenAmount};

#[derive(World)]
pub struct LSTWorld {
    pub(crate) env: BDDEnv,
    pub(crate) token: StakedCSPRHostRef,
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
            },
        );
        Self { env, token }
    }
}

impl LSTWorld {}

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
