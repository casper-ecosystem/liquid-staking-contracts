use cucumber::World;
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef};
use odra::host::{Deployer, HostEnv, NoArgs};
use odra::prelude::Address;
use odra_bdd::types::account::Account;
use odra_bdd::types::u256::U256Param;

pub type TokenAmount = U256Param<9>;

#[derive(World)]
pub struct LSTWorld {
    env: HostEnv,
    pub(crate) token: StakedCSPRHostRef
}

impl Default for LSTWorld {
    fn default() -> Self {
        let env = odra_test::env();
        let token = StakedCSPR::deploy(&env, NoArgs);
        Self { env, token }
    }
}

impl LSTWorld {
    pub fn get_address(&self, account: &Account) -> Address {
        self.env.get_account(account.account_id())
    }

    pub fn set_caller(&mut self, account: &Account) {
        self.env.set_caller(self.get_address(account));
    }
}

impl std::fmt::Debug for LSTWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LSTWorld")
    }
}

