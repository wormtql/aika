use std::marker::PhantomData;
use cgmath::{BaseFloat, Vector3};
use aika_math::Complex;
use crate::f;

pub struct MaterialConstants<F> {
    _phantom: PhantomData<F>
}

impl<F> MaterialConstants<F> where F: BaseFloat {
    pub fn gold_ior() -> Vector3<Complex<F>> {
        Vector3::new(
            Complex::new(f!(0.18299), f!(3.4242)),
            Complex::new(f!(0.42108), f!(2.34590)),
            Complex::new(f!(1.37340), f!(1.77040))
        )
    }
}
