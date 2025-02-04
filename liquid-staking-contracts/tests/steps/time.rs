use crate::lst_world::LSTWorld;
use cucumber::when;

#[when(expr = "{int} eras pass")]
fn advance_eras(world: &mut LSTWorld, eras: u32) {
    // Era is 2h, blocktime is in ms
    let advance = eras * 2 * 60 * 60 * 1000;
    world.env.env().advance_block_time(advance.into())
}

#[when(expr = "{int} auctions pass")]
#[when(expr = "{int} auction passes")]
fn advance_auctions(world: &mut LSTWorld, auctions: u32) {
    let era_time = world.env.env().auction_delay() as u32;
    let advance = auctions * era_time * 8;
    world.env.env().advance_with_auctions(advance.into());
}

#[when(expr = "unbonding period passes")]
fn advance_unbonding_period(world: &mut LSTWorld) {
    let auction_delay = world.env.env().auction_delay() as u32;
    let advance = auction_delay * 8;
    world.env.env().advance_with_auctions(advance.into());
}
