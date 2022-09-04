use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::ops::{Deref, DerefMut};

/// Wrapper around a random generator based on Pcg32.
pub struct Random {
    rng: Pcg32,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
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

/// Returns a random number for a predefined bag of them
pub struct ShuffleBag<T>
where
    T: Sized + Clone,
{
    rng: Random,
    index: usize,
    items: Vec<T>,
    bag: Vec<usize>,
}

impl<T> ShuffleBag<T>
where
    T: Sized + Clone,
{
    /// Create a new ShuffleBag using a random seed
    pub fn new(capacity: usize) -> Self {
        Self::new_with_random(Random::default(), capacity)
    }

    pub fn new_with_random(rng: Random, capacity: usize) -> Self {
        Self {
            rng,
            index: 0,
            items: vec![],
            bag: Vec::with_capacity(capacity),
        }
    }

    /// Adds a new value to the bag
    pub fn add(&mut self, item: T, amount: usize) {
        self.items.push(item);
        let index = self.items.len() - 1;
        self.bag.extend_from_slice(&vec![index; amount]);
        self.reset();
    }

    /// Returns the next value from the bag
    pub fn item(&mut self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }

        if self.index >= self.bag.len() {
            self.reset();
        }

        let item = &self.items[self.bag[self.index]];
        self.index += 1;
        Some(item)
    }

    /// Reset the bag to the initial state
    pub fn reset(&mut self) {
        self.bag.shuffle(&mut self.rng.rng);
        self.index = 0;
    }
}
