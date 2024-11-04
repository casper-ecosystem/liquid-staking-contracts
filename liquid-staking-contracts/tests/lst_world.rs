use cucumber::{then, World};
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRHostRef};
use odra::host::{Deployer, HostEnv, NoArgs};
use odra_bdd::types::u256::U256Param;

#[derive(World)]
pub struct LSTWorld {
    env: HostEnv,
    token: StakedCSPRHostRef
}

impl Default for LSTWorld {
    fn default() -> Self {
        let env = odra_test::env();
        let token = StakedCSPR::deploy(&env, NoArgs);
        Self { env, token }
    }
}

impl std::fmt::Debug for LSTWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LSTWorld")
    }
}

type TokenAmount = U256Param<9>;

// CEP18 steps

#[then(expr = "name is {string}")]
fn check_name(world: &mut LSTWorld, name: String) {
    assert_eq!(world.token.name(), name);
}

#[then(expr = "symbol is {string}")]
fn check_symbol(world: &mut LSTWorld, symbol: String) {
    assert_eq!(world.token.symbol(), symbol);
}

#[then(expr = "decimals is {int}")]
fn check_decimals(world: &mut LSTWorld, decimals: u8) {
    assert_eq!(world.token.decimals(), decimals);
}

#[then(expr = "total supply is {token_amount} sCSPR")]
fn check_total_supply(world: &mut LSTWorld, expected_total_supply: TokenAmount) {
    let total_supply = TokenAmount::from(world.token.total_supply());
    assert_eq!(total_supply, expected_total_supply);
}
