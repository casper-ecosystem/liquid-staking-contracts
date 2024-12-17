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
    let era_length = world.env.env().era_length() as u32;
    let advance = auctions * 8 * era_length;
    world.env.env().advance_with_rewards(advance.into());
}
