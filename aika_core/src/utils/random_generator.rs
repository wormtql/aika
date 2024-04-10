use std::marker::PhantomData;
use std::ptr::slice_from_raw_parts;
use cgmath::BaseFloat;
use rand::{Rng, SeedableRng, thread_rng};
use rand_chacha::ChaCha20Rng;

pub struct RandomGenerator<F> {
    generator: ChaCha20Rng,
    _phantom: PhantomData<F>
}

impl<F> RandomGenerator<F> where F: BaseFloat {
    pub fn new(seed: usize) -> RandomGenerator<F> {
        let mut s: [u8; 32] = [0; 32];

        let seed_bytes = unsafe {
            let ptr = &seed as *const usize as *const u8;
            &*slice_from_raw_parts(ptr, 4)
        };
        s[0] = seed_bytes[0];
        s[1] = seed_bytes[1];
        s[2] = seed_bytes[2];
        s[3] = seed_bytes[3];

        RandomGenerator {
            generator: ChaCha20Rng::from_seed(s),
            _phantom: PhantomData
        }
    }

    pub fn random(&mut self) -> F {
        let r = self.generator.gen_range(0.0..1.0);
        F::from(r).unwrap()
    }

    pub fn random_range(&mut self, left: i32, right: i32) -> i32 {
        self.generator.gen_range(left..right)
    }
}
