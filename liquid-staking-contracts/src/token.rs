use crate::token::Error::*;
use odra::{
    casper_types::{PublicKey, U256, U512},
    prelude::*,
};
use odra_modules::{
    access::{AccessControl, Ownable, Role, DEFAULT_ADMIN_ROLE},
    cep18::{errors::Error as Cep18Error, utils::Cep18Modality},
    cep18_token::Cep18,
};

#[odra::odra_error]
pub enum Error {
    NotYetClaimable = 61401,
    AlreadyClaimed = 61402,
    NotAnOwnerOfAClaim = 61403,
    UnstakeNotFound = 61404,
    NotAnOwner = 61405,
    InsufficientBalance = 61406,
    MisconfiguredValidator = 61407,
}

#[odra::event]
pub struct Staked {
    address: Address,
    cspr_amount: U512,
    scspr_minted: U256,
}

#[odra::event]
pub struct Unstaked {
    address: Address,
    cspr_amount: U512,
    scspr_burned: U256,
    unstake_id: u32,
    claim_time: u64,
}

#[odra::event]
pub struct Claimed {
    address: Address,
    cspr_amount: U512,
    unstake_id: u32,
}

#[odra::module(
    events = [Staked, Unstaked, Claimed],
    errors = Error
)]
pub struct StakedCSPR {
    access_control: SubModule<AccessControl>,
    ownable: SubModule<Ownable>,
    token: SubModule<Cep18>,
    unstake_ids: Mapping<Address, Vec<u32>>,
    unstakes: List<Unstake>,
    unclaimed_cspr: Var<U512>,
    validator_address: Var<PublicKey>,
    claim_time: Var<u64>,
    last_recorded_delegated_amount: Var<U512>,
    fee_percentage: Var<U512>,
}

#[odra::odra_type]
struct Unstake {
    unstake_id: u32,
    owner: Address,
    cspr_amount: U512,
    claimable_from: u64,
    claimed: bool,
}

#[odra::module]
impl StakedCSPR {
    delegate! {
        to self.access_control {
            fn has_role(&self, role: &Role, address: &Address) -> bool;
            fn grant_role(&mut self, role: &Role, address: &Address);
            fn revoke_role(&mut self, role: &Role, address: &Address);
        }

        to self.token {
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn decimals(&self) -> u8;
            fn total_supply(&self) -> U256;
            fn balance_of(&self, account: &Address) -> U256;
            fn allowance(&self, owner: &Address, spender: &Address) -> U256;
            fn transfer(&mut self, recipient: &Address, amount: &U256);
            fn transfer_from(&mut self, owner: &Address, recipient: &Address, amount: &U256);
            fn approve(&mut self, spender: &Address, amount: &U256);
            fn decrease_allowance(&mut self, spender: &Address, decr_by: &U256);
            fn increase_allowance(&mut self, spender: &Address, inc_by: &U256);
            fn change_security(
                &mut self,
                admin_list: Vec<Address>,
                minter_list: Vec<Address>,
                none_list: Vec<Address>
            );
            fn mint(&mut self, owner: &Address, amount: &U256);
            fn burn(&mut self, owner: &Address, amount: &U256);
        }
    }

    pub fn init(&mut self, validator_address: PublicKey, claim_time: u64, fee_percentage: U512) {
        let admin = self.env().caller();

        // Grant the admin role to the deployer.
        self.access_control
            .unchecked_grant_role(&DEFAULT_ADMIN_ROLE, &admin);
        self.ownable.init();

        // Initialize the validator address.
        self.validator_address.set(validator_address);
        // Initialize the claim time.
        self.claim_time.set(claim_time);
        // Initialize the token.
        self.token.init(
            String::from("sCSPR"),
            String::from("Staked CSPR"),
            9,
            U256::zero(),
            vec![],
            vec![],
            Some(Cep18Modality::None),
        );

        self.fee_percentage.set(fee_percentage);
        self.last_recorded_delegated_amount.set(U512::zero());
    }

    #[odra(payable)]
    pub fn stake(&mut self) {
        self.collect_fee();
        let caller = self.env().caller();
        let cspr_amount = self.env().attached_value();
        let scspr_amount = self.cspr_to_scspr(cspr_amount);
        self.env().delegate(
            self.validator_address
                .get()
                .unwrap_or_revert_with(self, MisconfiguredValidator),
            cspr_amount,
        );
        self.token.raw_mint(&caller, &scspr_amount);

        self.env().emit_event(Staked {
            address: caller,
            cspr_amount,
            scspr_minted: scspr_amount,
        });
        self.last_recorded_delegated_amount.set(self.staked_cspr());
    }

    pub fn unstake(&mut self, scspr_amount: U256) -> u32 {
        self.collect_fee();
        let caller = self.env().caller();
        if self.token.balance_of(&caller) < scspr_amount {
            self.env().revert(Cep18Error::InsufficientBalance);
        }

        let cspr_amount = self.scspr_to_cspr(scspr_amount);
        self.token.raw_burn(&caller, &scspr_amount);
        self.env().undelegate(
            self.validator_address
                .get()
                .unwrap_or_revert_with(self, MisconfiguredValidator),
            cspr_amount,
        );

        let mut account_unstake_ids = self.unstake_ids.get(&caller).unwrap_or_default();
        let new_unstake_id = self.unstakes.len();
        account_unstake_ids.push(new_unstake_id);

        self.unstakes.push(Unstake {
            unstake_id: new_unstake_id,
            owner: caller,
            cspr_amount,
            claimable_from: self.next_claim_time(),
            claimed: false,
        });

        // FIXME: unstake_ids per user can potentially overflow.
        self.unstake_ids.set(&caller, account_unstake_ids);
        self.unclaimed_cspr
            .set(self.unclaimed_cspr.get().unwrap_or_default() + cspr_amount);

        self.env().emit_event(Unstaked {
            address: caller,
            cspr_amount,
            scspr_burned: scspr_amount,
            unstake_id: new_unstake_id,
            claim_time: self.next_claim_time(),
        });

        self.last_recorded_delegated_amount.set(self.staked_cspr());

        new_unstake_id
    }

    pub fn claim(&mut self, unstake_id: u32) {
        let mut unstake = self
            .unstakes
            .get(unstake_id)
            .unwrap_or_revert_with(self, UnstakeNotFound);
        if unstake.claimable_from > self.env().get_block_time() {
            self.env().revert(NotYetClaimable);
        }
        if unstake.claimed {
            self.env().revert(AlreadyClaimed);
        }
        let caller = self.env().caller();
        if unstake.owner != caller {
            self.env().revert(NotAnOwnerOfAClaim);
        }

        let cspr_amount = unstake.cspr_amount;
        self.env().transfer_tokens(&unstake.owner, &cspr_amount);
        unstake.claimed = true;
        self.unclaimed_cspr
            .set(self.unclaimed_cspr.get().unwrap_or_default() - cspr_amount);
        self.unstakes.replace(unstake_id, unstake);

        self.env().emit_event(Claimed {
            address: caller,
            cspr_amount,
            unstake_id,
        });
    }

    pub fn staked_cspr(&self) -> U512 {
        self.env().delegated_amount(
            self.validator_address
                .get()
                .unwrap_or_revert_with(self, MisconfiguredValidator),
        )
    }

    #[odra(payable)]
    pub fn add_to_the_pool(&mut self) {
        self.collect_fee();
        self.env().delegate(
            self.validator_address
                .get()
                .unwrap_or_revert_with(self, MisconfiguredValidator),
            self.env().attached_value(),
        );
        self.last_recorded_delegated_amount.set(self.staked_cspr());
    }

    pub fn remove_from_the_pool(&mut self, amount: U512) {
        self.collect_fee();
        self.env().undelegate(
            self.validator_address
                .get()
                .unwrap_or_revert_with(self, MisconfiguredValidator),
            amount,
        );
        self.last_recorded_delegated_amount.set(self.staked_cspr());
    }

    pub fn withdraw_from_the_pool(&mut self, amount: U512) {
        if !self
            .access_control
            .has_role(&DEFAULT_ADMIN_ROLE, &self.env().caller())
        {
            self.env().revert(NotAnOwner);
        }

        if !self.env().self_balance() < amount {
            self.env().revert(InsufficientBalance);
        }

        self.env().transfer_tokens(&self.env().caller(), &amount);
    }

    pub fn state_report(&self) -> String {
        format!(
            "Staked CSPR: {} Unclaimed CSPR: {}, Admin sCSPR: {}",
            self.staked_cspr(),
            self.unclaimed_cspr.get().unwrap_or_default(),
            self.token.balance_of(&self.ownable.get_owner())
        )
    }

    pub fn collect(&mut self) {
        self.collect_fee();
    }
}

impl StakedCSPR {
    pub fn next_claim_time(&self) -> u64 {
        let now = self.env().get_block_time();
        let claim_time = self.claim_time.get().unwrap_or_default();
        now + claim_time
    }

    // TODO: Cover with tests at U256 and U512 boundaries.
    pub fn cspr_to_scspr(&self, cspr_stake: U512) -> U256 {
        let staked_cspr = self.staked_cspr();
        if staked_cspr.is_zero() {
            return u512_to_u256(cspr_stake);
        }
        let scspr_total_supply = u256_to_u512(self.token.total_supply());
        u512_to_u256(cspr_stake * scspr_total_supply / staked_cspr)
    }

    pub fn scspr_to_cspr(&self, scspr: U256) -> U512 {
        if scspr.is_zero() {
            return U512::zero();
        }
        let scspr_total_supply = u256_to_u512(self.token.total_supply());
        if scspr_total_supply.is_zero() {
            return U512::zero();
        }
        let staked_cspr = self.staked_cspr();

        u256_to_u512(scspr) * staked_cspr / scspr_total_supply
    }

    /// Calculates the rewards since last collection and mints the fee in the form of
    /// sCSPR to the admin.
    fn collect_fee(&mut self) {
        let current_delegated = self.staked_cspr();
        let last_recorded = self
            .last_recorded_delegated_amount
            .get()
            .unwrap_or_default();

        // First time delegation, set the last recorded delegation and bail, as there is no reward
        if last_recorded.is_zero() {
            self.last_recorded_delegated_amount.set(current_delegated);
            return;
        }

        // Only calculate rewards if delegation has increased.
        if current_delegated > last_recorded {
            let reward = current_delegated - last_recorded;
            let fee_percent = self.fee_percentage.get().unwrap_or_default();
            // Fee calculation: fee = reward * fee_percentage / 10000 (basis points)
            let fee = reward * fee_percent / U512::from(10000u64);
            // Calculate fee_scspr based on total staked amount of cspr and total liquidity of scspr
            let total_staked_cspr = self.staked_cspr();
            let total_scspr_liquidity = u256_to_u512(self.token.total_supply());
            let fee_scspr = if total_staked_cspr.is_zero() || total_scspr_liquidity.is_zero() {
                U256::zero()
            } else {
                u512_to_u256(fee * total_scspr_liquidity / total_staked_cspr)
            };
            // Mint sCSPR to admin (using the deployer/admin stored in DEFAULT_ADMIN_ROLE)
            let admin = self.ownable.get_owner();
            self.token.raw_mint(&admin, &fee_scspr);
            // Update the last recorded delegation.
            self.last_recorded_delegated_amount.set(current_delegated);
        }
    }
}

fn u512_to_u256(value: U512) -> U256 {
    U256::from(value.as_u128())
}

fn u256_to_u512(value: U256) -> U512 {
    U512::from(value.as_u128())
}

#[cfg(test)]
mod tests {
    use odra::host::{Deployer, HostRef};

    use super::*;

    #[test]
    fn test_initialization() {
        let env = odra_test::env();
        let token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 1000.into(),
            },
        );
        assert!(token.has_role(&DEFAULT_ADMIN_ROLE, &env.caller()));
    }

    #[test]
    fn test_fee_collection() {
        let env = odra_test::env();
        let auction_delay = env.auction_delay();
        let unbonding_delay = auction_delay * 8;
        let mut token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 100.into(),
            },
        );
        // Given Alice and Bob.
        let alice = env.get_account(1);
        let bob = env.get_account(2);
        let admin = env.get_account(0);
        let alice_initial_cspr_balance = env.balance_of(&alice);
        let bob_initial_cspr_balance = env.balance_of(&bob);

        // When Alice stakes 10 CSPR.
        let deposit_amount_u512 = U512::from(10_000_000_000u64);
        let deposit_amount_u256 = U256::from(10_000_000_000u64);
        env.set_caller(alice);
        token.with_tokens(deposit_amount_u512).stake();

        // And some time passes
        env.advance_with_auctions(auction_delay * 8);

        // And Bob stakes 10 CSPR.
        env.set_caller(bob);
        token.with_tokens(deposit_amount_u512).stake();

        // 8 Eras generated 3999 CSPR rewards
        // 3999 * 100 / 10000 = 39 CSPR fee
        // (10_000_000_000 / 10_000_003_999) * 39 = 38 sCSPR

        // Then Admin has 38 sCSPR
        assert_eq!(token.balance_of(&admin), U256::from(38u64));
    }

    #[test]
    fn test_staking() {
        // Given a deployed StakedCSPR contract.
        let env = odra_test::env();
        let auction_delay = env.auction_delay();
        let unbonding_delay = auction_delay * 8;
        let mut token = StakedCSPR::deploy(
            &env,
            StakedCSPRInitArgs {
                validator_address: env.get_validator(),
                claim_time: env.auction_delay() * 8,
                fee_percentage: 1000.into(),
            },
        );

        // Given Alice and Bob.
        let alice = env.get_account(1);
        let bob = env.get_account(2);
        let alice_initial_cspr_balance = env.balance_of(&alice);
        let bob_initial_cspr_balance = env.balance_of(&bob);

        // When Alice stakes 10 CSPR.
        let deposit_amount_u512 = U512::from(10_000_000_000u64);
        let deposit_amount_u256 = U256::from(10_000_000_000u64);
        env.set_caller(alice);
        token.with_tokens(deposit_amount_u512).stake();

        // Then staked CSPR should be 10 CSPR.
        assert_eq!(token.staked_cspr(), deposit_amount_u512);

        // Then Alice's balance should be less by 10 CSPR.
        let expected_amount = alice_initial_cspr_balance - deposit_amount_u512;
        assert_eq!(env.balance_of(&alice), expected_amount);

        // Then Alice's balance should be 10 sCSPR.
        assert_eq!(token.balance_of(&alice), deposit_amount_u256);

        // When Alice transfers 10 sCSPR to Bob.
        env.set_caller(alice);
        token.transfer(&bob, &deposit_amount_u256);

        // When Bob unstakes 10 sCSPR.
        env.set_caller(bob);
        token.unstake(deposit_amount_u256);

        // Then Bob's balance should be 0 sCSPR.
        assert_eq!(token.balance_of(&bob), U256::zero());

        // When time passes.
        env.advance_with_auctions(unbonding_delay);

        // And bob claims his unstake.
        env.set_caller(bob);
        token.claim(0);

        // // Then Bob's CSPR balance should be 10 CSPR more.
        // let expected_amount = bob_initial_cspr_balance + deposit_amount_u512;
        // assert_eq!(env.balance_of(&bob), expected_amount);
    }
}
