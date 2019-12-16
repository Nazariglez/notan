use std::num::Wrapping;

const MAX_I64_AS_U64:u64 = std::i64::MAX as u64;
const MAX_I32_AS_U32:u32 = std::i32::MAX as u32;

/// This is a port of PCG32 random generator, but it's not exactly a 1:1 port.
/// Some functions are added to get u64 numbers and use as a seed just one u64.
/// This maybe could broke some purpose of the original algorithm but could be acceptable for games.
/// https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.c
struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    const MAGIC: Wrapping<u64> = Wrapping(6364136223846793005);
    const DEFAULT_STATE: u64 = 0x853c49e6748fea9b;
    const DEFAULT_SEQ: u64 = 0xda3e39cb94b95bdb;

    pub fn new(state: u64, inc: u64) -> Self {
        let mut rnd = Self { state: 0, inc: 0 };
        rnd.reseed(state, inc);
        rnd
    }

    pub fn reseed(&mut self, init_state: u64, init_seq: u64) {
        self.state = 0;
        self.inc = (init_seq << 1) | 1;
        self.next_u32();
        self.state = (Wrapping(self.state) + Wrapping(init_state)).0;
        self.next_u32();
    }

    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = (Wrapping(old_state) * Self::MAGIC + Wrapping(self.inc)).0;
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        return (xorshifted >> rot) | (xorshifted << ((!rot) & 31));
    }

    //Ported from https://github.com/rust-random/rand/blob/684aa8f8a6de2a077dc3be2bd779bf4d91cc5167/rand_core/src/impls.rs#L28
    pub fn next_u64(&mut self) -> u64 {
        let x = u64::from(self.next_u32());
        let y = u64::from(self.next_u32());
        (y << 32) | x
    }

    pub fn from_u64_seed(seed: u64) -> Self {
        let (state, seq) = get_state_seq_from_seed(seed);
        Self::new(state, seq)
    }

    pub fn reseed_u64(&mut self, seed: u64) {
        let (state, seq) = get_state_seq_from_seed(seed);
        self.reseed(state, seq);
    }
}

impl Default for PCG32 {
    fn default() -> Self {
        Self::new(Self::DEFAULT_STATE, Self::DEFAULT_SEQ)
    }
}

fn get_state_seq_from_seed(seed: u64) -> (u64, u64) {
    let state:u64 = (Wrapping(seed) + Wrapping(PCG32::DEFAULT_STATE)).0;
    let seq:u64 = (Wrapping(seed) + Wrapping(PCG32::DEFAULT_SEQ)).0;
    (state, seq)
}

pub struct Random {
    rnd: PCG32,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        Self {
            rnd: PCG32::from_u64_seed(seed)
        }
    }

    pub fn reseed(&mut self, seed: u64) {
        self.rnd.reseed_u64(seed);
    }

    pub fn next<T: RngType>(&mut self) -> T {
        T::random_pcg(&mut self.rnd)
    }

    pub fn next_range<T: RngRandomType>(&mut self, min: T, max: T) -> T {
        T::random_range_pcg(&mut self.rnd, min, max)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rnd: PCG32::default()
        }
    }
}

pub trait RngType {
    fn random_pcg(rnd: &mut PCG32) -> Self;
}

pub trait RngRandomType: RngType {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self;
}

impl RngType for f32 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        rnd.next_u32() as f32 / std::u32::MAX as f32
    }
}

impl RngRandomType for f32 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        min + f32::random_pcg(rnd) * (max - min)
    }
}

impl RngType for f64 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        rnd.next_u64() as f64 / std::u64::MAX as f64
    }
}

impl RngRandomType for f64 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        min + f64::random_pcg(rnd) * (max - min)
    }
}

impl RngType for i64 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        (rnd.next_u64() % MAX_I64_AS_U64) as i64
    }
}

impl RngRandomType for i64 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        i64::random_pcg(rnd) % (max + 1 - min) + min
    }
}

impl RngType for i32 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        (rnd.next_u32() % MAX_I32_AS_U32) as i32
    }
}

impl RngRandomType for i32 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        i32::random_pcg(rnd) % (max + 1 - min) + min //TODO overflow...
    }
}

impl RngType for u64 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        rnd.next_u64()
    }
}

impl RngRandomType for u64 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        u64::random_pcg(rnd) % (max + 1 - min) + min
    }
}

impl RngType for u32 {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        rnd.next_u32()
    }
}

impl RngRandomType for u32 {
    fn random_range_pcg(rnd: &mut PCG32, min: Self, max: Self) -> Self {
        u32::random_pcg(rnd) % (max + 1 - min) + min
    }
}

impl RngType for bool {
    fn random_pcg(rnd: &mut PCG32) -> Self {
        f32::random_pcg(rnd) < 0.5
    }
}

#[cfg(test)]
mod test {
    use super::PCG32;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_default_u32() {
        let mut rnd = PCG32::default();
        assert_eq!(rnd.next_u32(), 232741498);
        assert_eq!(rnd.next_u32(), 4095165673);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_seed_u32() {
        let mut rnd = PCG32::new(123456789, 987654321);
        assert_eq!(rnd.next_u32(), 3092585893);
        assert_eq!(rnd.next_u32(), 2549368267);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_reseed_u32() {
        let mut rnd = PCG32::new(1, 1);
        assert_eq!(rnd.next_u32(), 3837955985);
        assert_eq!(rnd.next_u32(), 180974196);
        rnd.reseed(123456789, 987654321);
        assert_eq!(rnd.next_u32(), 3092585893);
        assert_eq!(rnd.next_u32(), 2549368267);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_default_u64() {
        let mut rnd = PCG32::default();
        assert_eq!(rnd.next_u64(), 17588602637469571706);
        assert_eq!(rnd.next_u64(), 8072651164842098155);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_seed_u64() {
        let mut rnd = PCG32::new(123456789, 987654321);
        assert_eq!(rnd.next_u64(), 10949453335317781925);
        assert_eq!(rnd.next_u64(), 9470389433207719862);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_pcg32_reseed_u64() {
        let mut rnd = PCG32::new(1, 1);
        assert_eq!(rnd.next_u64(), 777278257077850001);
        assert_eq!(rnd.next_u64(), 15195909113815377495);
        rnd.reseed(123456789, 987654321);
        assert_eq!(rnd.next_u64(), 10949453335317781925);
        assert_eq!(rnd.next_u64(), 9470389433207719862);
    }
}
