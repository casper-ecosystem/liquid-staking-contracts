mod lst_world;
mod steps;

pub fn main() {
    odra_bdd::run::<lst_world::LSTWorld>("./tests/features/fee.feature");
}
