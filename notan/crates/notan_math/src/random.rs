use rand;
use rand_pcg::Pcg32;
use rand::{SeedableRng};
use std::ops::{Deref, DerefMut};
pub use rand::Rng;

pub struct Random {
    rng: Pcg32
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed)
        }
    }

    pub fn reseed(&mut self, seed: u64) {
        self.rng = Pcg32::seed_from_u64(seed);
    }
}

impl Deref for Random {
    type Target = Pcg32;
    fn deref(&self) -> &Self::Target {
        &self.rng
    }
}

impl DerefMut for Random {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rng
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rng: Pcg32::from_entropy(),
        }
    }
}