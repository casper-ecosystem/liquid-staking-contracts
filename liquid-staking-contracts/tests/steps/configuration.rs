use crate::lst_world::LSTWorld;
use cucumber::{given, then, when};
use odra_bdd::types::{account::Account, cspr::CSPRAmount};

#[given(expr = "the fee_percentage is {int}")]
#[then(expr = "the fee_percentage is {int}")]
fn fee_percentage(world: &mut LSTWorld, fee: u32) {
    assert_eq!(world.token.get_fee_percentage(), fee.into());
}

#[when(expr = "{account} sets the fee_percentage to {int}")]
fn set_fee_percentage(world: &mut LSTWorld, account: Account, fee: u32) {
    world.env.set_caller(&account);
    world.token.set_fee_percentage(fee.into());
}

#[then(expr = "{account} cannot set the fee_percentage to {int}")]
fn cannot_set_fee_percentage(world: &mut LSTWorld, account: Account, fee: u32) {
    world.env.set_caller(&account);
    assert!(world.token.try_set_fee_percentage(fee.into()).is_err());
}

#[then(expr = "the claim_time is {int} seconds")]
#[given(expr = "the claim_time is {int} seconds")]
#[then(expr = "the claim_time is still {int} seconds")]
fn get_claim_time(world: &mut LSTWorld, claim_time: u64) {
    let claim_time_millis = claim_time * 1000;
    assert_eq!(world.token.get_claim_time(), claim_time_millis);
}

#[when(expr = "{account} changes the claim_time to {int} seconds")]
fn change_claim_time(world: &mut LSTWorld, account: Account, claim_time: u64) {
    world.env.set_caller(&account);
    let claim_time_millis = claim_time * 1000;
    world.token.set_claim_time(claim_time_millis);
}

#[when(expr = "{account} tries to change the claim_time to {int} seconds")]
#[then(expr = "{account} cannot set the claim_time to {int} seconds")]
fn cannot_change_claim_time(world: &mut LSTWorld, account: Account, claim_time: u64) {
    world.env.set_caller(&account);
    let claim_time_millis = claim_time * 1000;
    let result = world.token.try_set_claim_time(claim_time_millis);
    assert!(result.is_err());
}

#[then(expr = "the min_stake is {cspr_amount} CSPR")]
#[given(expr = "the min_stake is {cspr_amount} CSPR")]
fn get_min_stake(world: &mut LSTWorld, min_stake: CSPRAmount) {
    assert_eq!(world.token.get_min_stake(), min_stake.amount());
}

#[when(expr = "{account} changes the min_stake to {cspr_amount} CSPR")]
fn change_min_stake(world: &mut LSTWorld, account: Account, min_stake: CSPRAmount) {
    world.env.set_caller(&account);
    world.token.set_min_stake(min_stake.amount());
}

#[when(expr = "{account} tries to change the min_stake to {cspr_amount} CSPR")]
#[then(expr = "{account} cannot set the min_stake to {cspr_amount} CSPR")]
fn cannot_change_min_stake(world: &mut LSTWorld, account: Account, min_stake: CSPRAmount) {
    world.env.set_caller(&account);
    let result = world.token.try_set_min_stake(min_stake.amount());
    assert!(result.is_err());
}
