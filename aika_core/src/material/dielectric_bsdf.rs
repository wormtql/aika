use cgmath::{BaseFloat, ElementWise, Vector3};
use num_traits::Zero;
use aika_math::math_utils::refract;
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{RayObjectStatus, ShadingContext, TracingService};
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

impl<F> BSDF<F> for DielectricBSDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, dir1: Vector3<F>, dir2: Vector3<F>) -> Option<Vector3<F>> {
        Some(Vector3::zero())
    }

    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        let cos_theta = current_dir.z;
        let fresnel = fresnel_dielectric(cos_theta, F::one(), self.relative_ior);
        let vector_one = Vector3::new(F::one(), F::one(), F::one());
        let reflection_point_bias = if current_dir.z >= F::zero() {
            f!(1e-3)
        } else {
            f!(-1e-3)
        };
        // println!("{:?}", self.relative_ior);
        if fresnel.is_none() {
            // total internal reflection
            // println!("total internal reflection");
            return Some(BSDFSampleResult {
                direction: Vector3::new(-current_dir.x, -current_dir.y, current_dir.z),
                weight: vector_one,
                next_point: Vector3::new(F::zero(), F::zero(), reflection_point_bias),
            });
        }
        let fresnel = fresnel.unwrap();
        let transmittance = F::one() - fresnel;

        // println!("fresnel: {:?}", fresnel);
        let random = service.random_0_1();
        // let random = F::one();
        if random < fresnel {
            // sample reflect
            // println!("reflect");
            let w = F::one();
            Some(BSDFSampleResult {
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
            //     let backface = current_dir.z < F::zero();
            //     let wo = current_dir;
            //     let wi = refract_dir;
            //     let eta = self.relative_ior;
            //     let sin_theta_o = (F::one() - wo.z * wo.z).sqrt();
            //     let sin_theta_i = (F::one() - wi.z * wi.z).sqrt();
            //     let e2 = if backface {
            //         F::one() / eta
            //     } else { eta };
            //     if (sin_theta_o - sin_theta_i * e2).abs() >= F::from(1e-3).unwrap() {
            //         println!("wi: {:?}, wo: {:?}, relative ior: {:?}", wi, wo, e2);
            //         println!("{:?}, {:?}, backface: {}", sin_theta_o, sin_theta_i * e2, backface);
            //     }
            // }

            // {
            //     let a = refract_dir.z;
            //     let b = current_dir.z;
            //     println!("{:?}", a * b);
            // }

            // let w = F::one() / transmittance;
            let partial_solid_angle = if cos_theta < F::zero() {
                self.relative_ior * self.relative_ior
            } else {
                F::one() / (self.relative_ior * self.relative_ior)
            };
            let w = partial_solid_angle;
            // println!("wi: {:?}, refract: {:?}", current_dir, refract_dir);

            Some(BSDFSampleResult {
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
        // if context.ray_status == RayObjectStatus::Exiting {
        //     println!("{:?}", context.ray_status);
        // }

        let relative_ior = if !context.back_face {
            self.ior.div_element_wise(current_ior)
        } else {
            let next_ior = context.get_next_top_ior();
            // next_ior.div_element_wise(self.ior)
            self.ior.div_element_wise(next_ior)
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
