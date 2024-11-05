use crate::lst_world::{LSTWorld, TokenAmount};
use cucumber::{then, when};
use odra::casper_types::U512;
use odra::host::HostRef;
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;

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

#[when(expr = "{account} claims unstake with id {int}")]
fn claim_unstake(world: &mut LSTWorld, account: Account, unstake_id: u32) {
    world.env.set_caller(&account);
    world.token.claim(unstake_id);
}

#[when(expr = "{account} adds {cspr_amount} CSPR to the pool")]
fn add_to_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.with_tokens(*cspr_amount).add_to_the_pool();
}

#[when(expr = "{account} withdraws {cspr_amount} CSPR from the pool")]
#[when(expr = "{account} removes {cspr_amount} CSPR from the pool")]
fn withdraw_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.withdraw_from_the_pool(*cspr_amount);
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
#[then(expr = "{cspr_amount} CSPR is staked")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let staked_cspr = U512::from(world.token.staked_cspr());
    assert_eq!(staked_cspr, *cspr_amount);
}
