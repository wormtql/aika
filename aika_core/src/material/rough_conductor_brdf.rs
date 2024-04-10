use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::Zero;
use aika_math::Complex;
use aika_math::distribution::IsotropicGGXDistribution;
use aika_math::math_utils::reflect;
use aika_math::utils::is_same_hemisphere;
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};
use crate::utils::fresnel_complex;

pub struct RoughConductorBRDF<F> {
    pub distribution: IsotropicGGXDistribution<F>,
    pub relative_ior: Vector3<Complex<F>>,
}

impl<F> RoughConductorBRDF<F> where F: BaseFloat {
    pub fn new(roughness: F, relative_ior: Vector3<Complex<F>>) -> Self {
        RoughConductorBRDF {
            distribution: IsotropicGGXDistribution::new(roughness),
            relative_ior
        }
    }
}

impl<F> BSDF<F> for RoughConductorBRDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>> {
        let cos_theta_o = wo.z.abs();
        let cos_theta_i = wi.z.abs();
        if cos_theta_i == F::zero() || cos_theta_o == F::zero() {
            return None;
        }

        let half = (wi + wo).normalize();
        let fresnel1 = fresnel_complex(half.dot(wo).abs(), F::one(), self.relative_ior[0]);
        let fresnel2 = fresnel_complex(half.dot(wo).abs(), F::one(), self.relative_ior[1]);
        let fresnel3 = fresnel_complex(half.dot(wo).abs(), F::one(), self.relative_ior[2]);
        let fresnel = Vector3::new(fresnel1, fresnel2, fresnel3);

        Some(
            fresnel * self.distribution.evaluate(half) * self.distribution.masking_shadowing(wi, wo) /
                (F::from(4).unwrap() * cos_theta_o * cos_theta_i)
        )
    }

    fn sample_ray(&self, service: &mut TracingService<F>,  current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        let wo = current_dir;
        let z = Vector3::new(F::zero(), F::zero(), F::one());

        let (wm, wi) = {
            let mut m = Vector3::zero();
            let mut i = Vector3::zero();
            let mut finish = false;
            while !finish {
                m = self.distribution.sample_wm(wo, service.random_0_1(), service.random_0_1());
                i = reflect(wo, m);
                if is_same_hemisphere(i, wo, z) {
                    finish = true;
                }
            }
            (m, i)
        };

        if !is_same_hemisphere(wi, wo, z) {
            return None;
        }

        let pdf = self.distribution.distribution_of_visible_normal(wo, wm) / (f!(4) * wo.dot(wm).abs());

        // let fresnel1 = fresnel_complex(wm.dot(wo).abs(), F::one(), self.relative_ior[0]);
        // let fresnel2 = fresnel_complex(wm.dot(wo).abs(), F::one(), self.relative_ior[1]);
        // let fresnel3 = fresnel_complex(wm.dot(wo).abs(), F::one(), self.relative_ior[2]);
        // let fresnel = Vector3::new(fresnel1, fresnel2, fresnel3);

        let next_point_offset = if wo.z > F::zero() {
            f!(1e-3)
        } else {
            f!(-1e-3)
        };

        let brdf = self.evaluate(wi, wo)?;
        Some(BSDFSampleResult {
            direction: wi,
            weight: brdf / pdf,
            next_point: Vector3::new(F::zero(), F::zero(), next_point_offset),
        })
    }
}

pub struct RoughConductorBRDFMaterial<F> {
    pub ior: Vector3<Complex<F>>,
    pub roughness: F
}

impl<F> RoughConductorBRDFMaterial<F> where F: BaseFloat {
    pub fn new(roughness: F, ior: Vector3<Complex<F>>) -> Self {
        Self {
            roughness,
            ior
        }
    }
}

impl<F> MaterialTrait<F> for RoughConductorBRDFMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        let current_ior = context.get_current_ior();
        let relative_ior_r = self.ior[0] / current_ior[0];
        let relative_ior_g = self.ior[1] / current_ior[1];
        let relative_ior_b = self.ior[2] / current_ior[2];
        let relative_ior = Vector3::new(relative_ior_r, relative_ior_g, relative_ior_b);

        Some(Box::new(RoughConductorBRDF::new(self.roughness, relative_ior)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
