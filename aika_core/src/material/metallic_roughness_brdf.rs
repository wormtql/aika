use cgmath::{BaseFloat, ElementWise, InnerSpace, Vector3};
use aika_math::distribution::IsotropicGGXDistribution;
use aika_math::utils::{average_vector3_value, fresnel_schlick_approximate, get_2pi, get_pi, is_same_hemisphere_canonical, lerp_vector3, max_component_value, new_vector3, reflect, reflect_bias, sample_uniform_hemisphere, scalar_sub_vector3, smith_g2_lagarde};
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};

pub struct MetallicRoughnessBRDF<F> {
    pub color: Vector3<F>,
    pub roughness: F,
    pub metallic: F,
}

impl<F> MetallicRoughnessBRDF<F> where F: BaseFloat {
    pub fn new(roughness: F, metallic: F, base_color: Vector3<F>) -> Self {
        Self {
            color: base_color,
            roughness,
            metallic
        }
    }
}

impl<F> BSDF<F> for MetallicRoughnessBRDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>> {
        assert!(wi.z > F::zero());
        assert!(wo.z > F::zero());
        let dist = IsotropicGGXDistribution::new(self.roughness);
        let wm = (wi + wo).normalize();
        let ndf = dist.evaluate(wm);
        // let g2 = dist.masking_shadowing(wi, wo);

        let f0 = lerp_vector3(self.metallic, new_vector3(0.04, 0.04, 0.04), self.color);
        let fresnel = fresnel_schlick_approximate(self.color, wm.dot(wi));
        let specular_reflection = fresnel * ndf * smith_g2_lagarde(wi, wo, self.roughness);

        let local_sss = scalar_sub_vector3(F::one(), fresnel) * (F::one() - self.metallic);
        let local_sss = local_sss.mul_element_wise(self.color) / get_pi();
        // let local_sss = F::zero();

        Some(specular_reflection + local_sss)
    }

    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        assert!(current_dir.z > F::zero());

        let dist = IsotropicGGXDistribution::new(self.roughness);
        let wo = current_dir;
        let wm = dist.sample_wm(wo, service.random_0_1(), service.random_0_1());
        let pdf_wm = dist.distribution_of_visible_normal(wo, wm);
        let wi = reflect(wo, wm);
        let f0 = lerp_vector3(self.metallic, new_vector3(0.04, 0.04, 0.04), self.color);
        let fresnel = fresnel_schlick_approximate(self.color, wi.dot(wm));
        let avg_f = average_vector3_value(fresnel);

        let random = service.random_0_1();
        if random < avg_f {
            // specular reflection

            if wi.z <= F::zero() {
                return None;
            }
            assert!(is_same_hemisphere_canonical(wi, wo));

            let ndf = dist.evaluate(wm);
            let dwm_dwi = F::one() / (f!(4) * wo.dot(wm));
            // let pdf_wi = ndf * dwm_dwi * avg_f;
            let pdf = avg_f * pdf_wm * dwm_dwi;

            let g2 = dist.masking_shadowing(wi, wo);
            let f_mul_cos_theta_i = fresnel * ndf * smith_g2_lagarde(wi, wo, self.roughness) * wi.z;
            let w = f_mul_cos_theta_i / pdf;

            Some(BSDFSampleResult {
                direction: wi,
                weight: w,
                next_point: reflect_bias(wo),
            })
        } else {
            // diffuse

            let wi = sample_uniform_hemisphere(service.random_0_1(), service.random_0_1());
            let pdf = (F::one() - avg_f) / get_2pi();
            let local_sss = scalar_sub_vector3(F::one(), fresnel) * (F::one() - self.metallic);
            let local_sss = local_sss.mul_element_wise(self.color) / get_pi();

            let w = local_sss * wi.z / pdf;
            Some(BSDFSampleResult {
                direction: wi,
                weight: w,
                next_point: reflect_bias(wo)
            })
        }
    }
}

pub struct MetallicRoughnessBRDFMaterial<F> {
    pub f0: Vector3<F>,
    pub roughness: F,
    pub metallic: F,
}

impl<F: BaseFloat> MetallicRoughnessBRDFMaterial<F> {
    pub fn new(roughness: F, metallic: F, f0: Vector3<F>) -> Self {
        MetallicRoughnessBRDFMaterial {
            f0,
            roughness,
            metallic
        }
    }
}

impl<F> MaterialTrait<F> for MetallicRoughnessBRDFMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        Some(Box::new(MetallicRoughnessBRDF::new(self.roughness, self.metallic, self.f0)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
