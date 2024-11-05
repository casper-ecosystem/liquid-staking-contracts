use crate::lst_world::LSTWorld;
use cucumber::when;

#[when(expr = "{int} eras pass")]
fn advance_eras(world: &mut LSTWorld, eras: u32) {
    // Era is 2h, blocktime is in ms
    let advance = eras * 2 * 60 * 60 * 1000;
    world.env.env().advance_block_time(advance.into())
}
