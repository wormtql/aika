use cgmath::{BaseFloat, ElementWise, Vector3};
use num_traits::Zero;
use rand::Rng;
use aika_math::math_utils::refract;
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{RayObjectStatus, ShadingContext};
use crate::utils::fresnel_dielectric;

#[derive(Clone)]
pub struct DielectricBSDF<F> {
    pub relative_ior: F
}

impl<F> DielectricBSDF<F> where F: BaseFloat {
    pub fn new(relative_ior: F) -> Self {
        DielectricBSDF {
            relative_ior
        }
    }
}

impl<F> BSDF<F> for DielectricBSDF<F> where F: BaseFloat {
    fn evaluate(&self, dir1: Vector3<F>, dir2: Vector3<F>) -> Vector3<F> {
        Vector3::zero()
    }

    fn sample_ray(&self, current_dir: Vector3<F>) -> anyhow::Result<BSDFSampleResult<F>> {
        let cos_theta = current_dir.z;
        let fresnel = fresnel_dielectric(cos_theta, F::one(), self.relative_ior);
        let vector_one = Vector3::new(F::one(), F::one(), F::one());
        let reflection_point_bias = if current_dir.z >= F::zero() {
            f!(1e-6)
        } else {
            f!(-1e-6)
        };
        // println!("{:?}", self.relative_ior);
        if fresnel.is_none() {
            // total internal reflection
            println!("total internal reflection");
            return Ok(BSDFSampleResult {
                direction: Vector3::new(-current_dir.x, -current_dir.y, current_dir.z),
                weight: vector_one,
                next_point: Vector3::new(F::zero(), F::zero(), reflection_point_bias),
            });
        }
        let fresnel = fresnel.unwrap();
        let transmittance = F::one() - fresnel;

        // println!("fresnel: {:?}", fresnel);
        let random = f!(rand::thread_rng().gen_range(0.0..1.0));
        // let random = F::one();
        if random < fresnel {
            // sample reflect
            // println!("reflect");
            let w = F::one();
            Ok(BSDFSampleResult {
                direction: Vector3::new(-current_dir.x, -current_dir.y, current_dir.z),
                weight: Vector3::new(w, w, w),
                next_point: Vector3::new(F::zero(), F::zero(), reflection_point_bias)
            })
        } else {
            // sample transmit
            // println!("transmit");
            let z = Vector3::new(F::zero(), F::zero(), F::one());
            let refract_dir = refract(current_dir, z, F::one(), self.relative_ior).unwrap();

            // {
            //     let a = refract_dir.z;
            //     let b = current_dir.z;
            //     println!("{:?}", a * b);
            // }

            // let w = F::one() / transmittance;
            let w = F::one() / (self.relative_ior * self.relative_ior);
            // println!("wi: {:?}, refract: {:?}", current_dir, refract_dir);

            Ok(BSDFSampleResult {
                direction: refract_dir,
                weight: Vector3::new(w, w, w),
                next_point: Vector3::new(F::zero(), F::zero(), -reflection_point_bias),
            })
        }
    }
}

pub struct DielectricMaterial<F> {
    pub ior: Vector3<F>
}

impl<F> DielectricMaterial<F> where F: BaseFloat {
    pub fn new(ior: Vector3<F>) -> Self {
        Self {
            ior
        }
    }
}

impl<F> MaterialTrait<F> for DielectricMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        let current_ior = context.get_current_ior();
        if context.ray_status == RayObjectStatus::Exiting {
            println!("{:?}", context.ray_status);
        }

        let relative_ior = if !context.back_face {
            self.ior.div_element_wise(current_ior)
        } else {
            let next_ior = context.get_next_top_ior();
            next_ior.div_element_wise(self.ior)
        };
        Some(Box::new(DielectricBSDF::new(relative_ior.x)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }

    fn get_ior(&self) -> Option<Vector3<F>> {
        Some(self.ior)
    }
}
