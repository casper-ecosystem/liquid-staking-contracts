// TODO: CSPRParam

use cucumber::Parameter;
use odra::casper_types::U512;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Parameter, Debug, PartialEq, Clone, Copy, PartialOrd)]
#[param(regex = r"\d+", name = "cspr_amount")]
pub struct CSPRAmount {
    amount: U512,
}

impl CSPRAmount {
    pub fn new(amount: U512) -> Self {
        CSPRAmount { amount }
    }

    pub fn amount(&self) -> U512 {
        self.amount
    }
}

impl Deref for CSPRAmount {
    type Target = U512;

    fn deref(&self) -> &Self::Target {
        &self.amount
    }
}
impl FromStr for CSPRAmount {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount = U512::from_dec_str(s).map_err(|e| e.to_string())?;
        Ok(CSPRAmount { amount })
    }
}
