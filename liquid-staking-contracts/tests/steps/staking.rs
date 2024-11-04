use cucumber::{then, when};
use odra::casper_types::U512;
use odra::host::HostRef;
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;
use crate::lst_world::{LSTWorld, TokenAmount};

#[when(expr = "{account} stakes {cspr_amount} CSPR")]
fn stake_cspr(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.with_tokens(*cspr_amount).stake();
}

#[when(expr = "{account} unstakes {token_amount} sCSPR")]
fn unstake_cspr(world: &mut LSTWorld, account: Account, token_amount: TokenAmount) {
    world.env.set_caller(&account);
    world.token.unstake(*token_amount);
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let staked_cspr = U512::from(world.token.staked_cspr());
    assert_eq!(staked_cspr, *cspr_amount);
}