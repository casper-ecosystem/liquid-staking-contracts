use crate::lst_world::{LSTWorld, StakedCSPRAmount};
use cucumber::{given, then};
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;

#[given(expr = "{account} has {cspr_amount} CSPR")]
fn set_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_balance(&account, cspr_amount);
}

#[then(expr = "{account} has {cspr_amount} CSPR")]
#[then(expr = "{account}'s CSPR balance is {cspr_amount} CSPR")]
fn get_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert_eq!(cspr_amount, world.env.get_balance(&account));
}

#[then(expr = "{account} has {scspr} sCSPR")]
#[then(expr = "{account}'s sCSPR balance is {scspr}")]
fn get_token_balance(world: &mut LSTWorld, account: Account, scspr: StakedCSPRAmount) {
    let actual = world.token.balance_of(&world.env.get_address(&account));
    assert_eq!(scspr.amount(), actual);
}

#[then(expr = "{account} has more than {cspr_amount} CSPR")]
fn check_more_than_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert!(world.env.get_balance(&account) > cspr_amount);
}

#[then(expr = "{account} has more than {scspr} sCSPR")]
fn check_more_than_scspr_balance(world: &mut LSTWorld, account: Account, scspr: StakedCSPRAmount) {
    assert!(world.token.balance_of(&world.env.get_address(&account)) > scspr.amount());
}

#[then(expr = "{scspr} sCSPR is in the pool")]
fn check_pool_balance(world: &mut LSTWorld, scspr: StakedCSPRAmount) {
    assert_eq!(scspr.amount(), world.token.total_supply());
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
#[then(expr = "{cspr_amount} CSPR is staked")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let staked_cspr = CSPRAmount::new(world.token.staked_cspr(), 9);
    assert_eq!(cspr_amount, staked_cspr);
}

#[then(expr = "more than {cspr_amount} CSPR is staked")]
fn check_more_than_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert!(world.token.staked_cspr() > cspr_amount.amount());
}
