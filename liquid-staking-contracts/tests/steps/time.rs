use crate::lst_world::LSTWorld;
use cucumber::{then, when};
use odra_bdd::types::account::Account;

#[when(expr = "{int} eras pass")]
fn advance_eras(world: &mut LSTWorld, eras: u32) {
    // Era is 2h, blocktime is in ms
    let advance = eras * 2 * 60 * 60 * 1000;
    world.env.env().advance_block_time(advance.into())
}

#[when(expr = "{int} auctions pass")]
#[when(expr = "{int} auction passes")]
fn advance_auctions(world: &mut LSTWorld, auctions: u32) {
    let auction_delay = world.env.env().auction_delay() as u32;
    let advance = auctions * auction_delay;
    world.env.env().advance_with_auctions(advance.into());
}

#[when(expr = "unbonding period passes")]
fn advance_unbonding_period(world: &mut LSTWorld) {
    let auction_delay = world.env.env().auction_delay() as u32;
    let advance = auction_delay * 8;
    world.env.env().advance_with_auctions(advance.into());
}

#[then(expr = "the claim_time is {int} seconds")]
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
fn cannot_change_claim_time(world: &mut LSTWorld, account: Account, claim_time: u64) {
    world.env.set_caller(&account);
    let claim_time_millis = claim_time * 1000;
    let result = world.token.try_set_claim_time(claim_time_millis);
    assert!(result.is_err());
}
