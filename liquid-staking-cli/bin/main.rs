use liquid_staking_cli::{DeployScript, UpgradeStakedCSPR};
use liquid_staking_contracts::token::StakedCSPR;

pub fn main() {
    odra_cli::OdraCli::new()
        .about("Liquid Staking for CSPR. The CLI.")
        .deploy(DeployScript)
        .scenario(UpgradeStakedCSPR)
        .contract::<StakedCSPR>()
        .build()
        .run();
}
