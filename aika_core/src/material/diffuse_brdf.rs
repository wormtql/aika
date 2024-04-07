use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use rand::{Rng, thread_rng};
use aika_math::Ray;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use anyhow::Result;
use crate::f;
use crate::path_tracing::ShadingContext;

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

    fn sample_ray(&self, current_dir: Vector3<F>) -> Result<BSDFSampleResult<F>> {
        let mut r = thread_rng();
        let a = F::from(r.gen_range(0.0..1.0)).unwrap();
        let b = F::from(r.gen_range(0.0..1.0)).unwrap();

        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * a;
        let cos_theta = F::one() - b;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta).normalize();
        let pdf = F::one() / pi2;
        let weight = self.albedo * F::from(2).unwrap() * dir.z;

        let result = BSDFSampleResult {
            // pdf: Vector3::new(pdf, pdf, pdf),
            direction: dir,
            weight,
            // value: self.evaluate(current_dir, dir),
            // cos_theta: dir.z,
            next_point: Vector3::new(f!(0), f!(0), f!(1e-6))
        };
        Ok(result)
    }
}

pub struct DiffuseBRDFMaterial<F> {
    pub albedo: Vector3<F>,
}

impl<F> DiffuseBRDFMaterial<F> where F: BaseFloat {
    pub fn new(albedo: Vector3<F>) -> Self {
        DiffuseBRDFMaterial {
            albedo
        }
    }
}

impl<F> MaterialTrait<F> for DiffuseBRDFMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        Some(Box::new(DiffuseBRDF::new(self.albedo)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
