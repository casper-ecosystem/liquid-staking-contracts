use crate::lst_world::{LSTWorld, WORLD_MIN_STAKE};
use cucumber::given;
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRInitArgs};
use odra::{casper_types::U512, host::Deployer};

#[given(expr = "the contract is deployed with {int} basis points fee")]
fn deploy_with_fee(world: &mut LSTWorld, fee: u32) {
    let validator_address = world.env.env().get_validator(0);
    let _token = StakedCSPR::deploy(
        world.env.env(),
        StakedCSPRInitArgs {
            validator_address,
            claim_time: world.env.env().auction_delay() * 8,
            fee_percentage: fee.into(),
            min_stake: U512::from(WORLD_MIN_STAKE),
        },
    );
}

#[given(expr = "the contract cannot be deployed with {int} basis points fee")]
fn try_deploy_with_fee(world: &mut LSTWorld, fee: u32) {
    let validator_address = world.env.env().get_validator(0);
    let token = StakedCSPR::try_deploy(
        world.env.env(),
        StakedCSPRInitArgs {
            validator_address,
            claim_time: world.env.env().auction_delay() * 8,
            fee_percentage: fee.into(),
            min_stake: U512::from(WORLD_MIN_STAKE),
        },
    );
    assert!(token.is_err());
}

#[given(expr = "the contract is deployed with {int} min_stake")]
fn deploy_with_min_stake(world: &mut LSTWorld, min_stake: u128) {
    let validator_address = world.env.env().get_validator(0);
    let _token = StakedCSPR::deploy(
        world.env.env(),
        StakedCSPRInitArgs {
            validator_address,
            claim_time: world.env.env().auction_delay() * 8,
            fee_percentage: 1000.into(),
            min_stake: U512::from(min_stake),
        },
    );
}

#[given(expr = "the contract cannot be deployed with {int} min_stake")]
fn try_deploy_with_min_stake(world: &mut LSTWorld, min_stake: u128) {
    let validator_address = world.env.env().get_validator(0);
    let token = StakedCSPR::try_deploy(
        world.env.env(),
        StakedCSPRInitArgs {
            validator_address,
            claim_time: world.env.env().auction_delay() * 8,
            fee_percentage: 1000.into(),
            min_stake: U512::from(min_stake),
        },
    );
    assert!(token.is_err());
}
