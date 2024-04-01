use std::f64::consts::PI;
use cgmath::{BaseFloat, Vector3};
use rand::{Rng, thread_rng};
use aika_math::Ray;
use crate::material::{BSDF, MaterialTrait, VolumeTrait};

#[derive(Clone)]
pub struct DiffuseBRDF<F> {
    pub albedo: Vector3<F>,
}

impl<F> DiffuseBRDF<F> where F: BaseFloat {
    pub fn new(albedo: Vector3<F>) -> DiffuseBRDF<F> {
        DiffuseBRDF {
            albedo
        }
    }
}

impl<F> BSDF<F> for DiffuseBRDF<F> where F: BaseFloat {
    fn evaluate(&self, _dir1: Vector3<F>, _dir2: Vector3<F>) -> Vector3<F> {
        let pi = F::from(PI).unwrap();
        self.albedo / pi
    }

    fn sample_ray(&self, _current_dir: Vector3<F>) -> (Vector3<F>, Vector3<F>) {
        let mut r = thread_rng();
        let a = F::from(r.gen_range(0.0..1.0)).unwrap();
        let b = F::from(r.gen_range(0.0..1.0)).unwrap();

        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * a;
        let cos_theta = F::one() - b;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        let weight = F::one() / pi2;

        (Vector3::new(weight, weight, weight), dir)
    }
}

impl<F> MaterialTrait<F> for DiffuseBRDF<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn get_bsdf(&self) -> Box<dyn BSDF<F>> {
        Box::new(self.clone())
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
