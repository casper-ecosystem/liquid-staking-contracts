use crate::lst_world::LSTWorld;
use cucumber::given;
use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRInitArgs, MIN_STAKE};
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
            min_stake: U512::from(MIN_STAKE),
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
            min_stake: U512::from(MIN_STAKE),
        },
    );
    assert!(token.is_err());
}
