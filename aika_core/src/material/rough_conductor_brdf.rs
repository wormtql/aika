use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::Zero;
use aika_math::Complex;
use aika_math::distribution::IsotropicGGXDistribution;
use aika_math::utils::{is_same_hemisphere, reflect, reflect_bias, smith_g2_lagarde};
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};
use crate::utils::fresnel_complex;

pub struct RoughConductorBRDF<F> {
    pub distribution: IsotropicGGXDistribution<F>,
    pub relative_ior: Vector3<Complex<F>>,
    pub roughness: F,
}

impl<F> RoughConductorBRDF<F> where F: BaseFloat {
    pub fn new(roughness: F, relative_ior: Vector3<Complex<F>>) -> Self {
        RoughConductorBRDF {
            distribution: IsotropicGGXDistribution::new(roughness),
            relative_ior,
            roughness
        }
    }

    pub fn get_fresnel(&self, wi: Vector3<F>, wm: Vector3<F>) -> Vector3<F> {
        let fresnel1 = fresnel_complex(wm.dot(wi).abs(), F::one(), self.relative_ior[0]);
        let fresnel2 = fresnel_complex(wm.dot(wi).abs(), F::one(), self.relative_ior[1]);
        let fresnel3 = fresnel_complex(wm.dot(wi).abs(), F::one(), self.relative_ior[2]);
        Vector3::new(fresnel1, fresnel2, fresnel3)
    }
}

impl<F> BSDF<F> for RoughConductorBRDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>> {
        if wi.z <= F::zero() || wo.z <= F::zero() {
            // println!("this should not happen");
            return None;
        }
        let cos_theta_o = wo.z.abs();
        let cos_theta_i = wi.z.abs();

        let wm = (wi + wo).normalize();
        let fresnel = self.get_fresnel(wi, wm);

        let shadowing_masking = smith_g2_lagarde(wi, wo, self.roughness);
        Some(
            fresnel * self.distribution.evaluate(wm) * shadowing_masking
        )
    }

    fn sample_ray(&self, service: &mut TracingService<F>,  current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        let wo = current_dir;
        let z = Vector3::new(F::zero(), F::zero(), F::one());

        let wm = self.distribution.sample_wm(wo, service.random_0_1(), service.random_0_1());
        let wi = reflect(wo, wm);
        let pdf_wm = self.distribution.distribution_of_visible_normal(wo, wm);
        if wi.z <= F::zero() {
            return None;
        }

        let ndf = self.distribution.evaluate(wm);
        let fresnel = self.get_fresnel(wi, wm);
        let lar = smith_g2_lagarde(wi, wo, self.roughness);
        let weight = lar * f!(4) * wi.z.abs() * ndf / pdf_wm;
        let weight = fresnel * weight;

        Some(BSDFSampleResult {
            direction: wi,
            weight,
            next_point: reflect_bias(wo),
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
