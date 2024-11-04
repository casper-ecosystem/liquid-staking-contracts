use cucumber::{then, when};
use odra::casper_types::U512;
use odra::host::HostRef;
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;
use crate::lst_world::LSTWorld;

#[when(expr = "{account} stakes {cspr_amount} CSPR")]
fn stake_cspr(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert!(!world.get_address(&account).is_contract());
    world.set_caller(&account);
    world.token.with_tokens(*cspr_amount).stake();
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let staked_cspr = U512::from(world.token.staked_cspr());
    assert_eq!(staked_cspr, *cspr_amount);
}