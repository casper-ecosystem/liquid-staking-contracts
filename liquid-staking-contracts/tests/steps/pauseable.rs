use crate::lst_world::LSTWorld;
use cucumber::{then, when};
use odra::casper_types::{U256, U512};
use odra::host::HostRef;
use odra_bdd::types::account::Account;
use odra_modules::security::errors::Error::UnpausedRequired;

#[when(expr = "the contract is paused")]
fn pause_contract(world: &mut LSTWorld) {
    match world.token.is_paused() {
        true => (),
        false => world.token.pause(),
    }
}

#[when(expr = "the contract is unpaused")]
fn unpause_contract(world: &mut LSTWorld) {
    match world.token.is_paused() {
        true => world.token.unpause(),
        false => (),
    }
}

#[then(expr = "the contract is paused")]
fn assert_contract_paused(world: &mut LSTWorld) {
    assert!(world.token.is_paused());
}

#[then(expr = "the contract is unpaused")]
#[then(expr = "the contract is not paused")]
fn assert_contract_unpaused(world: &mut LSTWorld) {
    assert!(!world.token.is_paused());
}

#[then(expr = "transfer fails because the contract is paused")]
fn assert_transfer_fails(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let transfer_result = world.token.try_transfer(&alice, &U256::one());
    assert_eq!(transfer_result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "transfer_from fails because the contract is paused")]
fn assert_transfer_from_fails(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let bob = world.env.get_address(&Account::BOB);
    let transfer_result = world.token.try_transfer_from(&alice, &bob, &U256::one());
    assert_eq!(transfer_result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "increase_allowance fails because the contract is paused")]
fn assert_increase_allowance_fails(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let result = world.token.try_increase_allowance(&alice, &U256::one());
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "decrease_allowance fails because the contract is paused")]
fn assert_decrease_allowance_fails(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let result = world.token.try_decrease_allowance(&alice, &U256::one());
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "approve fails because the contract is paused")]
fn assert_approve_fails(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let result = world.token.try_approve(&alice, &U256::one());
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "claim fails because the contract is paused")]
fn assert_claim_fails(world: &mut LSTWorld) {
    let result = world.token.try_claim();
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "stake fails because the contract is paused")]
fn assert_stake_fails(world: &mut LSTWorld) {
    let result = world.token.try_stake();
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "unstake fails because the contract is paused")]
fn assert_unstake_fails(world: &mut LSTWorld) {
    let amount = U256::from(100);
    let result = world.token.try_unstake(amount);
    assert_eq!(result.unwrap_err(), UnpausedRequired.into());
}

#[then(expr = "transfer succeeds because the contract is unpaused")]
fn assert_transfer_fails_unpaused(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let transfer_result = world.token.try_transfer(&alice, &U256::one());
    assert!(transfer_result.is_ok());
}

#[then(expr = "transfer_from succeeds because the contract is unpaused")]
fn assert_transfer_from_succeeds(world: &mut LSTWorld) {
    // Setup: The owner needs to approve the transfer
    let alice = world.env.get_address(&Account::ALICE);
    let owner = world.env.get_address(&Account::OWNER);

    world.token.approve(&alice, &U256::from(5));

    let original_caller = world.env.env().caller();
    world.env.env().set_caller(alice);
    let result = world
        .token
        .try_transfer_from(&owner, &alice, &U256::from(1));
    assert!(result.is_ok());
    world.env.env().set_caller(original_caller);
}

#[then(expr = "increase_allowance succeeds because the contract is unpaused")]
fn assert_increase_allowance_succeeds(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let result = world.token.try_increase_allowance(&alice, &U256::one());
    assert!(result.is_ok());
}

#[then(expr = "decrease_allowance succeeds because the contract is unpaused")]
fn assert_decrease_allowance_succeeds(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);

    // First increase allowance so we can decrease it
    world.token.increase_allowance(&alice, &U256::from(10));

    let result = world.token.try_decrease_allowance(&alice, &U256::one());
    assert!(result.is_ok());
}

#[then(expr = "approve succeeds because the contract is unpaused")]
fn assert_approve_succeeds(world: &mut LSTWorld) {
    let alice = world.env.get_address(&Account::ALICE);
    let result = world.token.try_approve(&alice, &U256::one());
    assert!(result.is_ok());
}

#[then(expr = "stake succeeds because the contract is unpaused")]
fn assert_stake_succeeds(world: &mut LSTWorld) {
    let result = world
        .token
        .with_tokens(world.token.get_min_stake())
        .try_stake();
    assert!(result.is_ok());
}

#[then(expr = "unstake succeeds because the contract is unpaused")]
fn assert_unstake_succeeds(world: &mut LSTWorld) {
    let amount = U256::from(10);
    let result = world.token.try_unstake(amount);
    assert!(result.is_ok());
}

#[then(expr = "claim succeeds because the contract is unpaused")]
fn assert_claim_succeeds(world: &mut LSTWorld) {
    let result = world.token.try_claim();
    assert!(result.is_ok());
}

#[then(expr = "add_validator succeeds despite the contract being paused")]
fn assert_add_validator_succeeds_when_paused(world: &mut LSTWorld) {
    let validator_num = 2; // Using validator 2 for testing
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let result = world.token.try_add_validator(validator_address);
    assert!(result.is_ok());
}

#[then(expr = "remove_validator succeeds despite the contract being paused")]
fn assert_remove_validator_succeeds_when_paused(world: &mut LSTWorld) {
    let validator_num = 1; // Using validator 1 for testing
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let result = world.token.try_remove_validator(validator_address);
    assert!(result.is_ok());
}

#[then(expr = "set_min_stake succeeds despite the contract being paused")]
fn assert_set_min_stake_succeeds_when_paused(world: &mut LSTWorld) {
    let min_stake = U512::from(1000);
    let result = world.token.try_set_min_stake(min_stake);
    assert!(result.is_ok());
}

#[then(expr = "set_claim_time succeeds despite the contract being paused")]
fn assert_set_claim_time_succeeds_when_paused(world: &mut LSTWorld) {
    let claim_time: u64 = 86400; // 1 day in seconds
    let result = world.token.try_set_claim_time(claim_time);
    assert!(result.is_ok());
}

#[then(expr = "set_fee_percentage succeeds despite the contract being paused")]
fn assert_set_fee_percentage_succeeds_when_paused(world: &mut LSTWorld) {
    let fee_percentage: u32 = 5; // 5%
    let result = world.token.try_set_fee_percentage(fee_percentage.into());
    assert!(result.is_ok());
}

#[then(expr = "add_loose_tokens succeeds despite the contract being paused")]
fn assert_add_loose_tokens_succeeds_when_paused(world: &mut LSTWorld) {
    let result = world
        .token
        .with_tokens(U512::from(100))
        .try_add_loose_tokens();
    assert!(result.is_ok());
}

#[then(expr = "restake_loose_tokens succeeds despite the contract being paused")]
fn assert_restake_loose_tokens_succeeds_when_paused(world: &mut LSTWorld) {
    // First add some loose tokens
    world
        .token
        .with_tokens(world.token.get_min_stake())
        .add_loose_tokens();

    let result = world.token.try_restake_loose_tokens();
    assert!(result.is_ok());
}
