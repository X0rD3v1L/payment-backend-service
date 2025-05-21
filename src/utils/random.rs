use rand::{Rng, rng};

pub fn generate_initial_balance() -> f32 {
    let mut rng = rng();
    rng.random_range(2000.0..=5000.0)
}
