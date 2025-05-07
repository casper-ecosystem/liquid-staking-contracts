use crate::lst_world::LSTWorld;
use cucumber::{given, then, when};
use odra_bdd::types::account::Account;

#[given(expr = "{account} is the contract owner")]
#[then(expr = "{account} is the contract owner")]
fn check_name(world: &mut LSTWorld, account: Account) {
    assert_eq!(world.token.get_owner(), world.env.get_address(&account));
}

#[when(expr = "{account} transfers ownership to {account}")]
fn transfer_ownership(world: &mut LSTWorld, account: Account, new_owner: Account) {
    world.env.set_caller(&account);
    world
        .token
        .transfer_ownership(&world.env.get_address(&new_owner));
}

#[when(expr = "{account} accepts ownership")]
fn accept_ownership(world: &mut LSTWorld, account: Account) {
    world.env.set_caller(&account);
    world.token.accept_ownership();
}

#[then(expr = "{account} is not the contract owner")]
fn check_new_owner(world: &mut LSTWorld, account: Account) {
    assert_ne!(world.token.get_owner(), world.env.get_address(&account));
}
