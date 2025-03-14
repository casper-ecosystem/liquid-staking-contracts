use std::collections::HashMap;

use crate::lst_world::{LSTWorld, StakedCSPRAmount};
use cucumber::{given, then};
use odra::casper_types::U512;
use odra_bdd::types::account::Account;
use odra_bdd::types::cspr::CSPRAmount;

#[given(expr = "{account} has {cspr_amount} CSPR")]
fn set_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    world.env.set_balance(&account, cspr_amount);
}

#[then(expr = "{account} has {cspr_amount} CSPR")]
#[then(expr = "{account}'s CSPR balance is {cspr_amount} CSPR")]
fn get_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert_eq!(cspr_amount, world.env.get_balance(&account));
}

#[then(expr = "{account} has {scspr} sCSPR")]
#[then(expr = "{account}'s sCSPR balance is {scspr}")]
fn get_token_balance(world: &mut LSTWorld, account: Account, scspr: StakedCSPRAmount) {
    let actual = world.token.balance_of(&world.env.get_address(&account));
    assert_eq!(scspr.amount(), actual);
}

#[then(expr = "{account} has more than {cspr_amount} CSPR")]
fn check_more_than_cspr_balance(world: &mut LSTWorld, account: Account, cspr_amount: CSPRAmount) {
    assert!(world.env.get_balance(&account) > cspr_amount);
}

#[then(expr = "{account} has more than {scspr} sCSPR")]
fn check_more_than_scspr_balance(world: &mut LSTWorld, account: Account, scspr: StakedCSPRAmount) {
    assert!(world.token.balance_of(&world.env.get_address(&account)) > scspr.amount());
}

#[then(expr = "{scspr} sCSPR is in the pool")]
fn check_pool_balance(world: &mut LSTWorld, scspr: StakedCSPRAmount) {
    assert_eq!(scspr.amount(), world.token.total_supply());
}

#[then(expr = "staked CSPR is {cspr_amount} CSPR")]
#[then(expr = "{cspr_amount} CSPR is staked")]
fn check_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let staked_cspr = CSPRAmount::new(world.token.staked_cspr(), 9);
    assert_eq!(cspr_amount, staked_cspr);
}

#[then(expr = "more than {cspr_amount} CSPR is staked")]
fn check_more_than_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert!(world.token.staked_cspr() > cspr_amount.amount());
}

#[then(expr = "Validator{int} has {cspr_amount} CSPR in his staking pool")]
fn check_validator_stake(world: &mut LSTWorld, validator_num: u32, cspr_amount: CSPRAmount) {
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let validator_stake = world.token.get_validator_stake(&validator_address);
    assert_eq!(validator_stake, cspr_amount.amount());
}

#[then(expr = "exactly one validator received {cspr_amount} CSPR since previous stake")]
fn check_one_validator_got_stake(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    // Get all "stake" entrypoints from pool_state
    let stake_states: Vec<_> = world
        .pool_state
        .iter()
        .filter(|(entrypoint, _)| *entrypoint == "stake")
        .collect();

    // We need at least 2 stake states to compare
    assert!(
        stake_states.len() >= 2,
        "Need at least 2 stake operations to compare"
    );

    // Get the second-to-last stake state (skip the most recent one)
    let second_last_index = stake_states.len() - 2;
    let state = stake_states[second_last_index].1.clone();

    // Let's collect current validator stakes
    let validators = world.token.get_validators();
    let mut current_stakes = HashMap::new();
    for validator in validators {
        current_stakes.insert(
            validator.clone(),
            world.token.get_validator_stake(&validator),
        );
    }

    // Now we need to make sure that only one validator has more stake than before with the given amount
    let mut upstaked_validators = 0;
    for (validator, stake) in current_stakes {
        if stake == state.get(&validator).unwrap_or(&U512::zero()) + cspr_amount.amount() {
            upstaked_validators += 1;
        }
    }
    assert_eq!(
        upstaked_validators, 1,
        "Expected exactly one validator to receive the stake amount"
    );
}

#[then(expr = "total stakes across all validators is {cspr_amount} CSPR")]
fn check_total_validator_stakes(world: &mut LSTWorld, expected_total: CSPRAmount) {
    let validators = world.token.get_validators();

    // Sum up stakes across all validators
    let total_stake = validators.iter().fold(U512::zero(), |acc, validator| {
        acc + world.token.get_validator_stake(validator)
    });

    // Check that total stake matches expected amount
    assert_eq!(
        total_stake,
        expected_total.amount(),
        "Total stake across validators ({}) doesn't match expected amount ({})",
        total_stake,
        expected_total.amount()
    );
}
