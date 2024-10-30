use odra::prelude::*;

#[odra::module]
struct StakedCSPR {

}

#[odra::module]
impl StakedCSPR {
    pub fn init(&mut self) {
        // Initialize the contract
    }
}

#[cfg(test)]
mod tests {
    use odra::host::{Deployer, NoArgs};

    use super::*;

    #[test]
    fn test_init() {
        let env = odra_test::env();
        let _token = StakedCSPR::deploy(&env, NoArgs);
    }
}