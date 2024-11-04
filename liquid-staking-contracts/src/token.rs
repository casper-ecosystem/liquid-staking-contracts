use odra::{casper_types::{U256, U512}, prelude::*};
use odra_modules::{
    access::{AccessControl, Role, DEFAULT_ADMIN_ROLE},
    cep18::{
        utils::Cep18Modality,
        errors::Error as Cep18Error,
    },
    cep18_token::Cep18,
};
use crate::token::Error::{AlreadyClaimed, NotAnOwnerOfAClaim, NotYetClaimable, UnstakeNotFound};

#[odra::odra_error]
pub enum Error {
    NotYetClaimable = 61401,
    AlreadyClaimed = 61402,
    NotAnOwnerOfAClaim = 61403,
    UnstakeNotFound = 61404,
}

#[odra::module(
    errors = Error
)]
pub struct StakedCSPR {
    access_control: SubModule<AccessControl>,
    token: SubModule<Cep18>,
    unstake_ids: Mapping<Address, Vec<u32>>,
    unstakes: List<Unstake>,
    unclaimed_cspr: Var<U512>,
}

#[odra::odra_type]
struct Unstake {
    unstake_id: u32,
    owner: Address,
    cspr_amount: U512,
    claimable_from: u64,
    claimed: bool
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

    pub fn init(&mut self) {
        let admin = self.env().caller();

        // Grant the admin role to the deployer.
        self.access_control
            .unchecked_grant_role(&DEFAULT_ADMIN_ROLE, &admin);

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
    }

    #[odra(payable)]
    pub fn stake(&mut self) {
        let caller = self.env().caller();
        let cspr_amount = self.env().attached_value();
        let scspr_amount = self.cspr_to_scspr(cspr_amount);
        self.ffi_stake(cspr_amount);
        self.token.raw_mint(&caller, &scspr_amount);
    }

    pub fn unstake(&mut self, scspr_amount: U256) -> u32 {
        let caller = self.env().caller();
        if self.token.balance_of(&caller) < scspr_amount {
            self.env().revert(Cep18Error::InsufficientBalance);
        }

        let cspr_amount = self.scspr_to_cspr(scspr_amount);
        self.token.raw_burn(&caller, &scspr_amount);
        self.ffi_unstake(cspr_amount);

        let mut account_unstake_ids = self.unstake_ids.get(&caller).unwrap_or_default();
        let new_unstake_id = self.unstakes.len();
        account_unstake_ids.push(new_unstake_id);

        self.unstakes.push(Unstake {
            unstake_id: new_unstake_id,
            owner: caller,
            cspr_amount,
            claimable_from: self.claim_time(),
            claimed: false,
        });

        self.unstake_ids.set(&caller, account_unstake_ids);
        self.unclaimed_cspr.set(self.unclaimed_cspr.get().unwrap_or_default() + cspr_amount);
        new_unstake_id
    }

    pub fn claim(&mut self, receipt_id: u32) {
        let mut unstake = self.unstakes.get(receipt_id).unwrap_or_revert_with(self, UnstakeNotFound);
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

        self.env().transfer_tokens(&unstake.owner, &unstake.cspr_amount);
        unstake.claimed = true;
        self.unclaimed_cspr.set(self.unclaimed_cspr.get().unwrap_or_default() - unstake.cspr_amount);
        self.unstakes.replace(receipt_id, unstake);
    }

    pub fn staked_cspr(&self) -> U512 {
        self.env().self_balance() - self.unclaimed_cspr.get().unwrap_or_default()
    }
}

impl StakedCSPR {
    pub fn claim_time(&self) -> u64 {
        // TODO: confirm the era duration and unstake time.
        let seven_eras = 7 * 2 * 60 * 60 * 1000;
        self.env().get_block_time() + seven_eras
    }

    pub fn cspr_to_scspr(&self, amount: U512) -> U256 {
        // TODO: Implement correct conversion.
        u512_to_u256(amount)
    }

    pub fn scspr_to_cspr(&self, amount: U256) -> U512 {
        // TODO: Implement correct conversion.
        u256_to_u512(amount)
    }

    pub fn ffi_stake(&self, _amount: U512) {
        // TODO: Implement when available.
    }

    pub fn ffi_unstake(&self, _amount: U512) {
        // TODO: Implement when available.
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
    use odra::host::{Deployer, HostRef, NoArgs};

    use super::*;

    #[test]
    fn test_initialization() {
        let env = odra_test::env();
        let token = StakedCSPR::deploy(&env, NoArgs);
        assert!(token.has_role(&DEFAULT_ADMIN_ROLE, &env.caller()));
    }

    #[test]
    fn test_staking() {
        // Given a deployed StakedCSPR contract.
        let env = odra_test::env();
        let mut token = StakedCSPR::deploy(&env, NoArgs);

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

        // Then Bob's CSPR balance should be 10 CSPR more.
        let expected_amount = bob_initial_cspr_balance + deposit_amount_u512;
        assert_eq!(env.balance_of(&bob), expected_amount);
    }
}
