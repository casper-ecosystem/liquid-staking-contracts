use odra::host::{Deployer, HostEnv, NoArgs};
use odra_cli::OdraCli;
use liquid_staking_contracts::token::StakedCSPR;

pub struct DeployScript;
impl odra_cli::deploy::DeployScript for DeployScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut odra_cli::DeployedContractsContainer,
    ) -> Result<(), odra_cli::deploy::Error> {
        env.set_gas(250_000_000_000);
        let token = StakedCSPR::try_deploy(&env, NoArgs)?;
        container.add_contract(&token)?;
        Ok(())
    }
}

pub fn main() {
    OdraCli::new()
        .about("Liquid Staking for CSPR. The CLI.")
        .deploy(DeployScript)
        .contract::<StakedCSPR>()
        .build()
        .run();
}