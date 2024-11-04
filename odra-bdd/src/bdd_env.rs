use odra::host::HostEnv;
use odra::prelude::Address;
use crate::types::account::Account;

pub struct BDDEnv {
    env: HostEnv
}

impl BDDEnv {
    pub fn new(env: HostEnv) -> Self {
        Self { env }
    }

    pub fn get_address(&self, account: &Account) -> Address {
        self.env.get_account(account.account_id())
    }

    pub fn set_caller(&mut self, account: &Account) {
        self.env.set_caller(self.get_address(account));
    }

    pub fn env(&self) -> &HostEnv {
        &self.env
    }
}