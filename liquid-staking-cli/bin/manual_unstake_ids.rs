use liquid_staking_contracts::token::StakedCSPR;
use odra::{host::HostRefLoader, prelude::Address};

fn main() {
    let env = odra_casper_livenet_env::env();
    let contract_addr = "hash-06c2959ffb52388c68e053d8ef792d0db0660f75eee91d7532a8dbfd2d96087d";
    let contract_addr = Address::new(contract_addr).unwrap();
    let contract = StakedCSPR::load(&env, contract_addr);
    let caller = env.caller();

    println!("Caller: {:?}", caller);
    // println!("Unstake Ids: {:?}", contract.get_unstake_ids(&caller));
    println!("Unstake: {:#?}", contract.get_unstake(0));

}