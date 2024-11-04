pub mod types;

use std::fmt::Debug;
use cucumber::{codegen::WorldInventory, World};

pub fn run<W: World + Debug + WorldInventory>(path: &str) {
    futures::executor::block_on(W::run(path));
}