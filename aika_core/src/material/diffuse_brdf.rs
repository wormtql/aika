use std::f64::consts::PI;
use cgmath::{BaseFloat, Vector3};
use rand::{Rng, thread_rng};
use aika_math::Ray;
use crate::material::surface_material::SurfaceMaterial;

pub struct DiffuseBRDF<F> {
    pub albedo: Vector3<F>,
}

impl<F> SurfaceMaterial<F> for DiffuseBRDF<F> where F: BaseFloat {
    fn bsdf(&self, _light_dir: Vector3<F>, _view_dir: Vector3<F>) -> Vector3<F> {
        let pi = F::from(PI).unwrap();
        self.albedo / pi
    }

    fn sample_ray(&self, _current_dir: Vector3<F>) -> (F, Vector3<F>) {
        let mut r = thread_rng();
        let a = F::from(r.gen_range(0.0..1.0)).unwrap();
        let b = F::from(r.gen_range(0.0..1.0)).unwrap();

        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * a;
        let cos_theta = F::one() - b;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_phi);

        (F::one() / pi2, dir)
    }
}
