use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use aika_math::Ray;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use anyhow::Result;
use crate::f;
use crate::path_tracing::{ShadingContext, TracingService};

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

impl<F> BSDF<F> for DiffuseBRDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, _dir1: Vector3<F>, _dir2: Vector3<F>) -> Option<Vector3<F>> {
        let pi = F::from(PI).unwrap();
        Some(self.albedo / pi)
    }

    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        if current_dir.z < F::zero() {
            println!("current dir: {:?}", current_dir);
            return None;
        }
        // assert!(current_dir.z >= F::zero());
        let a = service.random_0_1();
        let b = service.random_0_1();

        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * a;
        let cos_theta = F::one() - b;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta).normalize();
        // assert!(dir.z >= F::zero());
        if dir.z < F::zero() {
            println!("sampled diffuse brdf dir is under normal {:?}", dir);
        }
        let pdf = F::one() / pi2;
        let weight = self.albedo * F::from(2).unwrap() * dir.z;

        let result = BSDFSampleResult {
            // pdf: Vector3::new(pdf, pdf, pdf),
            direction: dir,
            weight,
            // value: self.evaluate(current_dir, dir),
            // cos_theta: dir.z,
            next_point: Vector3::new(f!(0), f!(0), f!(1e-5))
        };
        Some(result)
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
