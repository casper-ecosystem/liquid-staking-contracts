use crate::lst_world::{LSTWorld, StakedCSPRAmount};
use cucumber::{then, when};
use odra::host::HostRef;
use odra::prelude::OdraError;
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;

#[when(expr = "{account} stakes {cspr_amount} CSPR")]
fn stake_cspr(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.with_tokens(*cspr_amount).stake();
    world.update_pool_state("stake");
}

#[when(expr = "{account} unstakes {scspr} sCSPR")]
fn unstake_cspr(world: &mut LSTWorld, account: Account, scspr: StakedCSPRAmount) {
    world.env.set_caller(&account);
    world.token.unstake(scspr.amount());
    world.update_pool_state("unstake");
}

#[when(expr = "{account} unstakes everything")]
fn unstake_everything(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    let total_stake = world.token.balance_of(&world.env.get_address(&account));
    world.token.unstake(total_stake);
    world.update_pool_state("unstake_everything");
}

#[when(expr = "{account} claims unstakes")]
fn claim_unstake(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    world.token.claim();
    world.update_pool_state("claim_unstake");
}

#[when(expr = "{account} adds {cspr_amount} CSPR to the pool")]
fn add_to_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.with_tokens(*cspr_amount).add_to_the_pool_without_staking();
    world.update_pool_state("add_to_the_pool");
}

#[when(expr = "{account} removes {cspr_amount} CSPR from the pool")]
fn remove_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.remove_from_the_pool(*cspr_amount);
    world.update_pool_state("remove_from_the_pool");
}

#[when(expr = "{account} removes everything from the pool")]
fn remove_everything_from_the_pool(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    let cspr_amount = world.token.get_total_stake();
    world.token.remove_from_the_pool(cspr_amount);
    world.update_pool_state("remove_from_the_pool");
}

#[when(expr = "{account} withdraws {cspr_amount} CSPR from the contract")]
fn withdraw_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.withdraw_from_the_contract(*cspr_amount);
    world.update_pool_state("withdraw_from_the_contract");
}

#[when(expr = "{account} tries to withdraw {cspr_amount} CSPR")]
fn try_withdraw_from_the_pool(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_caller(&account);
    let _ = world.token.try_withdraw_from_the_contract(*cspr_amount);
    world.update_pool_state("withdraw_from_the_contract");
}

#[when(expr = "{account} restakes loose tokens")]
fn restake_loose_tokens(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    world.token.restake_loose_tokens();
}

#[then(expr = "{account} cannot unstake {scspr} sCSPR because there's no backing for redemption")]
fn cannot_unstake_because_no_backing(
    world: &mut LSTWorld,
    account: Account,
    scspr: StakedCSPRAmount,
) {
    world.env.set_caller(&account);
    let result = world.token.try_unstake(scspr.amount());
    assert_eq!(result.err().unwrap(), OdraError::user(61416));
    world.update_pool_state("cannot_unstake_because_no_backing");
}
