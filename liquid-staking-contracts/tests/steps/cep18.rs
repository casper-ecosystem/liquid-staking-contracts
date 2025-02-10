use crate::lst_world::LSTWorld;
use cucumber::then;
use odra_bdd::types::{account::Account, token_amount::TokenAmount};

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

#[then(expr = "total supply is {token_amount} sCSPR")]
fn check_total_supply(world: &mut LSTWorld, expected_total_supply: TokenAmount) {
    let total_supply = TokenAmount::from(world.token.total_supply());
    assert_eq!(total_supply, expected_total_supply);
}

#[then(expr = "{account} token balance is {token_amount} sCSPR")]
#[then(expr = "{account}'s token balance is {token_amount} sCSPR")]
fn check_token_balance(world: &mut LSTWorld, account: Account, expected_balance: TokenAmount) {
    let balance = TokenAmount::from(world.token.balance_of(&world.env.get_address(&account)));
    assert_eq!(balance, expected_balance);
}
