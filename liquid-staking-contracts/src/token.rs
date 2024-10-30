use odra::{casper_types::{U256, U512}, prelude::*};
use odra_modules::{
    access::{AccessControl, Role, DEFAULT_ADMIN_ROLE},
    cep18::{
        utils::Cep18Modality,
        errors::Error as Cep18Error,
    },
    cep18_token::Cep18,
};

#[odra::module]
pub struct StakedCSPR {
    access_control: SubModule<AccessControl>,
    token: SubModule<Cep18>,
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
            fn approve(&mut self, spender: &Address, amount: &U256);
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
        let amount = self.env().attached_value();
        self.token.raw_mint(&caller, &u512_to_u256(amount));
    }

    pub fn unstake(&mut self, amount: U256) {
        let caller = self.env().caller();
        if self.token.balance_of(&caller) < amount {
            self.env().revert(Cep18Error::InsufficientBalance);
        }
        self.token.raw_burn(&caller, &amount);
        self.env().transfer_tokens(&caller, &u256_to_u512(amount));
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
