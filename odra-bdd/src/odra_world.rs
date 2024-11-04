use cucumber::World;

#[derive(World)]
pub struct OdraWorld {}

impl Default for OdraWorld {
    fn default() -> Self {
        Self {}
    }
}
