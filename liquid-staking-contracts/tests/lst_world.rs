use cucumber::World;
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef, StakedCSPRInitArgs};
use odra::host::Deployer;
use odra_bdd::bdd_env::BDDEnv;
use odra_bdd::types::u256::U256Param;

pub type TokenAmount = U256Param<9>;

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
