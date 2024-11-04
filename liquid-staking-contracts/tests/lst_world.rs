use cucumber::World;
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef};
use odra::host::{Deployer, NoArgs};
use odra_bdd::env::BDDEnv;
use odra_bdd::types::u256::U256Param;

pub type TokenAmount = U256Param<9>;

#[derive(World)]
pub struct LSTWorld {
    pub(crate) env: BDDEnv,
    pub(crate) token: StakedCSPRHostRef
}

impl Default for LSTWorld {
    fn default() -> Self {
        let env = BDDEnv::new(odra_test::env());
        let token = StakedCSPR::deploy(env.env(), NoArgs);
        Self { env, token }
    }
}

impl LSTWorld {

}

impl std::fmt::Debug for LSTWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LSTWorld")
    }
}

