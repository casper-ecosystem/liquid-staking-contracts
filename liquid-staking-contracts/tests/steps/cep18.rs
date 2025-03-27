use crate::lst_world::{LSTWorld, StakedCSPRAmount};
use cucumber::{then, when};
use odra_bdd::types::account::Account;

#[then(expr = "name is {string}")]
fn check_name(world: &mut LSTWorld, name: String) {
    assert_eq!(world.token.name(), name);
}

#[then(expr = "symbol is {string}")]
fn check_symbol(world: &mut LSTWorld, symbol: String) {
    assert_eq!(world.token.symbol(), symbol);
}

#[then(expr = "decimals is {int}")]
fn check_decimals(world: &mut LSTWorld, decimals: u8) {
    assert_eq!(world.token.decimals(), decimals);
}

#[then(expr = "total supply is {scspr} sCSPR")]
fn check_total_supply(world: &mut LSTWorld, expected_total_supply: StakedCSPRAmount) {
    let total_supply = StakedCSPRAmount::from(world.token.total_supply());
    assert_eq!(total_supply, expected_total_supply);
}

#[then(expr = "{account} token balance is {scspr} sCSPR")]
#[then(expr = "{account}'s token balance is {scspr} sCSPR")]
fn check_token_balance(world: &mut LSTWorld, account: Account, expected_balance: StakedCSPRAmount) {
    let balance = StakedCSPRAmount::from(world.token.balance_of(&world.env.get_address(&account)));
    assert_eq!(balance, expected_balance);
}

#[when(expr = "{account} transfers {scspr} sCSPR to {account}")]
fn transfer_scspr(
    world: &mut LSTWorld,
    from_account: Account,
    amount: StakedCSPRAmount,
    to_account: Account,
) {
    world.env.set_caller(&from_account);
    let recipient_address = world.env.get_address(&to_account);
    world.token.transfer(&recipient_address, &amount.amount());
    world.update_pool_state("transfer");
}
