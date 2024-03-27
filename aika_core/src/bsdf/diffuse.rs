use std::f64::consts::PI;
use cgmath::{BaseFloat, Vector3};
use rand::{Rng, thread_rng};
use crate::bsdf::BSDF;

pub struct DiffuseBRDF<F> {
    pub albedo: Vector3<F>,
}

impl<F> BSDF<F> for DiffuseBRDF<F> where F: BaseFloat {
    fn evaluate(&self, _in_dir: Vector3<F>, _out_dir: Vector3<F>) -> Vector3<F> {
        let pi = F::from(PI).unwrap();
        self.albedo / pi
    }

    fn importance_sample(&self, _in_dir: Vector3<F>) -> (Vector3<F>, F) where F: BaseFloat {
        let mut r = thread_rng();
        let a = F::from(r.gen_range(0.0..1.0)).unwrap();
        let b = F::from(r.gen_range(0.0..1.0)).unwrap();

        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * a;
        let cos_theta = F::one() - b;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_phi);

        (dir, F::one() / pi2)
    }
}
