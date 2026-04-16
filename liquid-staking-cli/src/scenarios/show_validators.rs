use liquid_staking_contracts::token::StakedCSPR;
use odra::host::HostEnv;
use odra_cli::{
    scenario::{Args, Error, Scenario, ScenarioMetadata},
    CommandArg, ContractProvider, DeployedContractsContainer,
};

pub struct ShowValidators;

impl ScenarioMetadata for ShowValidators {
    const NAME: &'static str = "ShowValidators";
    const DESCRIPTION: &'static str = "Queries the contract and prints all configured validators";
}

impl Scenario for ShowValidators {
    fn args(&self) -> Vec<CommandArg> {
        vec![]
    }

    fn run(
        &self,
        env: &HostEnv,
        container: &DeployedContractsContainer,
        _args: Args,
    ) -> Result<(), Error> {
        let token = container.contract_ref::<StakedCSPR>(env)?;
        let validators = token.get_validators();

        if validators.is_empty() {
            odra_cli::log("No validators configured on the contract.");
            return Ok(());
        }

        odra_cli::log(format!("Found {} validator(s):", validators.len()));

        for (index, validator) in validators.iter().enumerate() {
            let stake = token.get_validator_stake(validator.clone());
            odra_cli::log(format!(
                "  {}. {:?} | delegated stake: {}",
                index + 1,
                validator,
                stake
            ));
        }

        odra_cli::log(format!("Total delegated stake: {}", token.get_total_stake()));

        Ok(())
    }
}