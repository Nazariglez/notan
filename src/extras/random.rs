use nae_core::rand;
use nae_core::rand_pcg;
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use rand::seq::{SliceChooseIter, SliceRandom};
use rand::{Rng, SeedableRng};
use rand_pcg::{Lcg64Xsh32, Pcg32};

/// Generate random numbers
/// This is a just a simple wrapper on top of the Pcg32 Algorithm
/// For more advanced use of randomness look at: https://docs.rs/rand/0.7.2/rand/
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

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn gen_range<T, B1, B2>(&mut self, min: B1, max: B2) -> T
    where
        T: SampleUniform,
        B1: SampleBorrow<T> + Sized,
        B2: SampleBorrow<T> + Sized,
    {
        self.rng.gen_range(min, max)
    }

    pub fn shuffle<T: SliceRandom<Item = T>>(&mut self, slice: &mut T) {
        //TODO allow also vecs...
        slice.shuffle(&mut self.rng)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rng: Pcg32::from_entropy(),
        }
    }
}

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
    pub fn new(seed: u64, capacity: usize) -> Self {
        Self {
            rng: Random::new(seed),
            index: 0,
            items: vec![],
            bag: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, item: T, amount: usize) {
        self.items.push(item);
        let index = self.items.len() - 1;
        self.bag.extend_from_slice(&vec![index; amount]);
        self.reset();
    }

    pub fn item(&mut self) -> Option<&T> {
        if self.items.len() == 0 {
            return None;
        }

        if self.index >= self.bag.len() {
            self.reset();
        }

        let item = &self.items[self.bag[self.index]];
        self.index += 1;
        Some(item)
    }
    pub fn reset(&mut self) {
        self.bag.shuffle(&mut self.rng.rng);
        self.index = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_shuffle_bag() {
        let mut bag = ShuffleBag::new(1, 100);
        let to_add = vec![
            (23, 17), // Num 23 - 17 times
            (45, 12), // Num 45 - 12 times
            (76, 30), // NUm 76 - 30 times
        ];
        let (result_expected, iter_num) = to_add.iter().fold((0, 0), |acc, (n, amount)| {
            (acc.0 + n * amount, acc.1 + *amount)
        });

        //add nums to the bag
        to_add.iter().for_each(|(n, amount)| bag.add(*n, *amount));
        let result = (0..iter_num).fold(0, |acc, _| acc + *bag.item().unwrap());
        assert_eq!(result_expected, result);

        //The loop should be reset automatically when the end is reached
        let result = (0..iter_num).fold(0, |acc, _| acc + *bag.item().unwrap());
        assert_eq!(result_expected, result);
    }
}
