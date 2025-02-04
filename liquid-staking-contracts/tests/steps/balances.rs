use crate::lst_world::{LSTWorld, TokenAmount};
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

#[then(expr = "{account} has {token_amount} sCSPR")]
#[then(expr = "{account}'s sCSPR balance is {token_amount}")]
fn get_token_balance(world: &mut LSTWorld, account: Account, token_amount: TokenAmount) {
    let actual = world.token.balance_of(&world.env.get_address(&account));
    assert_eq!(token_amount, TokenAmount::from(actual));
}

#[then(expr = "{account} has more than {cspr_amount} CSPR")]
fn check_more_than_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert!(world.env.get_balance(&account) > cspr_amount);
}

#[then(expr = "{account} has more than {token_amount} sCSPR")]
fn check_more_than_scspr_balance(
    world: &mut LSTWorld,
    account: Account,
    scspr_amount: TokenAmount,
) {
    assert!(world.token.balance_of(&world.env.get_address(&account)) > scspr_amount.as_u256());
}

#[then(expr = "{token_amount} sCSPR is in the pool")]
fn check_pool_balance(world: &mut LSTWorld, scspr_amount: TokenAmount) {
    assert_eq!(scspr_amount, TokenAmount::from(world.token.total_supply()));
}
