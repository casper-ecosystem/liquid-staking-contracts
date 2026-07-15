use liquid_staking_contracts::token::StakedCSPR;
use odra::host::{Deployer, HostEnv, NoArgs};
use odra::prelude::Addressable;
use odra_cli::{
    cspr,
    scenario::{Args, Error, Scenario, ScenarioMetadata},
    CommandArg, ContractProvider, DeployedContractsContainer,
};

pub struct UpgradeStakedCSPR;

impl ScenarioMetadata for UpgradeStakedCSPR {
    const NAME: &'static str = "UpgradeStakedCSPR";
    const DESCRIPTION: &'static str = "Upgrade Position Tokens to the newest version";
}

impl Scenario for UpgradeStakedCSPR {
    fn args(&self) -> Vec<CommandArg> {
        vec![]
    }

    fn run(
        &self,
        env: &HostEnv,
        container: &DeployedContractsContainer,
        _args: Args,
    ) -> Result<(), Error> {
        let stcspr = container.contract_ref::<StakedCSPR>(env)?;
        env.set_gas(cspr!(500));
        odra_cli::log(format!(
            "Upgrading StakedCSPR at address: {:?}",
            stcspr.address()
        ));

        let _ = StakedCSPR::try_upgrade(env, stcspr.address(), NoArgs).unwrap();
        odra_cli::log("Upgraded StakedCSPR successfully.");
        Ok(())
    }
}
