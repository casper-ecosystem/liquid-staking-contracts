use crate::lst_world::{LSTWorld, StakedCSPRAmount};
use cucumber::{given, then};
use odra::casper_types::U512;
use odra::Addressable;
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

#[then(expr = "removed validator stake is {cspr_amount} CSPR")]
fn removed_validator_stake(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert_eq!(cspr_amount.amount(), world.token.removed_validator_stake());
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

#[then(expr = "the contract has {cspr_amount} CSPR")]
fn check_contract_cspr_balance(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    assert_eq!(
        cspr_amount,
        CSPRAmount::new(world.env.env().balance_of(world.token.address()), 9)
    );
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

#[then(expr = "{cspr_amount} CSPR is really staked")]
fn check_really_staked_cspr(world: &mut LSTWorld, cspr_amount: CSPRAmount) {
    let validators = world.token.get_validators();
    let total_stake = validators.iter().fold(U512::zero(), |acc, validator| {
        acc + world
            .env
            .env()
            .delegated_amount(*world.token.address(), validator.clone())
    });
    assert_eq!(cspr_amount.amount(), total_stake);
}

#[then(expr = "Validator{int} has {cspr_amount} CSPR in his staking pool")]
fn check_validator_stake(world: &mut LSTWorld, validator_num: u32, cspr_amount: CSPRAmount) {
    let validator_address = world.env.env().get_validator(validator_num as usize - 1);
    let validator_stake = world.token.get_validator_stake(validator_address);
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
    let current_stakes = world.current_stakes();

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
        acc + world.token.get_validator_stake(validator.clone())
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

#[then(expr = "{cspr_amount} CSPR is loose")]
fn check_loose_tokens(world: &mut LSTWorld, cspr: CSPRAmount) {
    // Get the loose tokens from the contract
    let total_loose = world.token.get_loose_tokens();

    // Assert that the total loose tokens match the expected amount
    assert_eq!(
        total_loose,
        cspr.amount(),
        "Total loose tokens ({}) doesn't match expected amount ({})",
        total_loose,
        cspr.amount()
    );

    // Also the balance of the token should be greater than the total loose tokens
    assert!(
        world.env.env().balance_of(world.token.address()) >= total_loose,
        "Token balance ({}) is less than the total loose tokens ({})",
        world.env.env().balance_of(world.token.address()),
        total_loose
    );
}

#[then(
    expr = "{int} validators received total {cspr_amount} CSPR since last remove in equal amounts"
)]
fn random_validators_check(world: &mut LSTWorld, validators_count: u32, cspr: CSPRAmount) {
    let remove_state = world
        .pool_state
        .iter()
        .filter(|(entrypoint, _)| *entrypoint == "remove_validator")
        .last()
        .unwrap()
        .1
        .clone();
    let current_stakes = world.current_stakes();

    let mut upstaked_validators = 0;
    let mut total = U512::zero();
    for (validator, stake) in current_stakes {
        let previous_stake = remove_state
            .get(&validator)
            .unwrap_or(&U512::zero())
            .clone();
        if stake != previous_stake {
            upstaked_validators += 1;
            total += stake;
        }
    }

    assert_eq!(
        upstaked_validators, validators_count,
        "Expected exactly {} validators to receive the stake amount",
        validators_count
    );

    let expected = cspr.amount() / validators_count;
    assert_eq!(
        total,
        expected * validators_count,
        "Total stake across validators ({}) doesn't match expected amount ({})",
        total,
        expected
    );
}
