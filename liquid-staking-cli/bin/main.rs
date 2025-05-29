use liquid_staking_contracts::token::{StakedCSPR, StakedCSPRInitArgs};
use odra::{
    casper_types::{bytesrepr::FromBytes, PublicKey, U512},
    host::{Deployer, HostEnv},
};
use odra_cli::OdraCli;

pub fn validator() -> PublicKey {
    let bytes = "0106ca7c39cd272dbf21a86eeb3b36b7c26e2e9b94af64292419f7862936bca2ca";
    let bytes = hex::decode(bytes).expect("Failed to decode hex");
    let (pk, _) = PublicKey::from_bytes(&bytes).unwrap();
    pk
}

const ONE_HOUR: u64 = 60 * 60 * 1000;
const NET_MIN_STAKE: u128 = 500_000_000_000;

pub struct DeployScript;
impl odra_cli::deploy::DeployScript for DeployScript {
    fn deploy(
        &self,
        env: &HostEnv,
        container: &mut odra_cli::DeployedContractsContainer,
    ) -> Result<(), odra_cli::deploy::Error> {
        env.set_gas(500_000_000_000);
        let token = StakedCSPR::try_deploy(
            env,
            StakedCSPRInitArgs {
                validator_address: validator(),
                claim_time: 2 * ONE_HOUR * 8,
                fee_percentage: 1000.into(),
                min_stake: U512::from(NET_MIN_STAKE),
            },
        )?;
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
