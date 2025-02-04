use crate::lst_world::{LSTWorld, TokenAmount};
use cucumber::{then, when};
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

#[when(expr = "{account} unstakes everything")]
fn unstake_everything(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    let total_stake = world.token.balance_of(&world.env.get_address(&account));
    world.token.unstake(total_stake);
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

#[when(expr = "{account} removes {cspr_amount} CSPR from the pool")]
fn remove_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.remove_from_the_pool(*cspr_amount);
}

#[when(expr = "{account} withdraws {cspr_amount} CSPR from the contract")]
fn withdraw_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.withdraw_from_the_pool(*cspr_amount);
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
#[then(expr = "{cspr_amount} CSPR is staked")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert_eq!(world.token.staked_cspr(), *cspr_amount);
}

///     Then more than 20 CSPR is staked
#[then(expr = "more than {cspr_amount} CSPR is staked")]
fn check_more_than_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert!(world.token.staked_cspr() > *cspr_amount);
}
