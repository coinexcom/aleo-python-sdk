use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn os_rng() -> StdRng {
    StdRng::from_rng(&mut rand::rng())
}
