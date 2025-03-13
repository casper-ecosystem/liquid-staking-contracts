use crate::lst_world::LSTWorld;
use cucumber::{given, then, when};
use odra_bdd::types::account::Account;

#[given(expr = "Owner deploys a contract with Validator{int}")]
fn deploy_with_validator(world: &mut LSTWorld, validator_num: u32) {
    // The contract is already deployed with one validator in the LSTWorld::default
    // We'll validate that Validator1 corresponds to the first validator in the test environment
    let validator1_address = world.env.env().get_validator(validator_num as usize - 1);
    let initial_validator = world.token.get_validators().get(0).unwrap().clone();
    assert_eq!(validator1_address, initial_validator);
}

#[then(expr = "Validator{int} is a validator")]
fn is_validator(world: &mut LSTWorld, validator_num: u32) {
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let validators = world.token.get_validators();
    assert!(validators.contains(&validator_address));
}

#[then(expr = "Validator{int} is not a validator")]
fn is_not_validator(world: &mut LSTWorld, validator_num: u32) {
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let validators = world.token.get_validators();
    assert!(!validators.contains(&validator_address));
}

#[then(expr = "(the )contract has {int} validator(s)")]
fn count_validators(world: &mut LSTWorld, count: u32) {
    let validators = world.token.get_validators();
    assert_eq!(validators.len(), count as usize);
}

#[when(expr = "{account} adds Validator{int}")]
fn add_validator(world: &mut LSTWorld, account: Account, validator_num: u32) {
    world.env.set_caller(&account);
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    world.token.add_validator(validator_address);
}

#[when(expr = "{account} removes Validator{int}")]
fn remove_validator(world: &mut LSTWorld, account: Account, validator_num: u32) {
    world.env.set_caller(&account);
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    world.token.remove_validator(validator_address);
}
