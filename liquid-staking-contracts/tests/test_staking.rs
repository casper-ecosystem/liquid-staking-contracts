mod lst_world;

pub fn main() {
    odra_bdd::run::<lst_world::LSTWorld>("./tests/features");
}