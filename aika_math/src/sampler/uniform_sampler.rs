use std::marker::PhantomData;
use cgmath::{BaseFloat, Vector2};
use rand::{Rng, thread_rng};

pub struct UniformSampler<F> {
    _phantom: PhantomData<F>
}

impl<F> UniformSampler<F> where F: BaseFloat {
    pub fn new() -> Self {
        UniformSampler {
            _phantom: PhantomData
        }
    }

    pub fn sample_1d(&self) -> F {
        let r = thread_rng().gen_range(0.0..1.0);
        F::from(r).unwrap()
    }

    pub fn sample_2d(&self) -> Vector2<F> {
        Vector2::new(self.sample_1d(), self.sample_1d())
    }
}
