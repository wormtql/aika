use cgmath::{BaseFloat, ElementWise, InnerSpace, Vector3};
use num_traits::Zero;
use aika_math::distribution::IsotropicGGXDistribution;
use aika_math::utils::{face_forward, get_generalized_half, get_z, is_same_hemisphere, is_same_hemisphere_canonical, length_square_vector3, reflect, reflect_bias, refract, smith_g2_lagarde, sqr};
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};
use crate::utils::fresnel_dielectric;

pub struct RoughDielectricBSDF<F> {
    ndf: IsotropicGGXDistribution<F>,
    /// the ior out of the normal / ior inside the object
    relative_ior: Vector3<F>,
    is_single_ior: bool,
    roughness: F,
}

impl<F> RoughDielectricBSDF<F> where F: BaseFloat + 'static {
    pub fn new(roughness: F, ior: Vector3<F>) -> Self {
        let is_single_ior = if ior.x == ior.y && ior.x == ior.z {
            true
        } else {
            false
        };
        Self {
            ndf: IsotropicGGXDistribution::new(roughness),
            relative_ior: ior,
            is_single_ior,
            roughness
        }
    }

    pub fn sample_ray_single_ior(&self, service: &mut TracingService<F>, wo: Vector3<F>, ior_index: usize) -> Option<BSDFSampleResult<F>> {
        let eta = self.relative_ior[ior_index];
        let wm = self.ndf.sample_wm(wo, service.random_0_1(), service.random_0_1());
        assert!(wm.z > F::zero());
        let pdf_wm = self.ndf.distribution_of_visible_normal(wo, wm);
        let cos_theta_o = wm.dot(wo);
        let cos_theta_o_abs = cos_theta_o.abs();
        let fresnel = fresnel_dielectric(cos_theta_o, F::one(), eta).unwrap_or(F::one());
        let backface = wo.z < F::zero();
        let transmission = F::one() - fresnel;
        let random = service.random_0_1();
        // let random = F::one();
        // let random = F::zero();
        if random < fresnel {
            // reflect

            let wi = reflect(wo, wm);
            if !is_same_hemisphere_canonical(wi, wo) {
                // since we are not considering multi scattering, the ray ends there
                return None;
            }

            let ndf = self.ndf.evaluate(wm);
            assert!(ndf > F::zero());
            let lar = smith_g2_lagarde(wi, wo, self.roughness);
            let weight = lar * f!(4) * cos_theta_o_abs * ndf / pdf_wm;

            Some(BSDFSampleResult {
                direction: wi,
                weight: Vector3::new(weight, weight, weight),
                next_point: reflect_bias(wo),
            })
        } else {
            let wi = refract(wo, wm, F::one(), eta);

            if wi.is_none() {
                println!("refract is none");
                return None;
            }
            let wi = wi.unwrap();

            // check correctness, which can be excluded in release build
            {
                let sin_theta_o = (F::one() - wo.dot(wm).powi(2)).sqrt();
                let sin_theta_i = (F::one() - wi.dot(wm).powi(2)).sqrt();
                let e2 = if backface {
                    F::one() / eta
                    // eta
                } else { eta };
                if (sin_theta_o - sin_theta_i * e2).abs() >= F::from(1e-3).unwrap() {
                    println!("wi: {:?}, wo: {:?}, relative ior: {:?}", wi, wo, e2);
                    println!("{:?}, {:?}, backface: {}", sin_theta_o, sin_theta_i * e2, backface);
                }
            }

            if is_same_hemisphere_canonical(wi, wo) || wi.z == F::zero() {
                return None;
            }

            let ndf = self.ndf.evaluate(wm);
            let lar = smith_g2_lagarde(wi, wo, self.roughness);
            let cos_theta_i_abs = wi.z.abs();
            let mut weight = f!(4) * lar * cos_theta_i_abs * wm.dot(wo).abs() * ndf / pdf_wm;
            // if backface {
            //     weight = weight / (eta * eta);
            // } else {
            //     weight = weight * (eta * eta);
            // }
            Some(BSDFSampleResult {
                direction: wi,
                weight: Vector3::new(weight, weight, weight),
                next_point: -reflect_bias(wo)
            })
        }
    }
}

impl<F> BSDF<F> for RoughDielectricBSDF<F> where F: BaseFloat + 'static {
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>> {
        if self.ndf.is_effectively_smooth() {
            return Some(Vector3::zero());
        }

        let eta = self.relative_ior[0];

        let cos_theta_o = wo.z;
        let cos_theta_i = wi.z;
        let reflect = cos_theta_i * cos_theta_o > F::zero();
        let fresnel = fresnel_dielectric(cos_theta_i.abs(), F::one(), eta);
        let wm = get_generalized_half(wi, wo, eta)?;

        if reflect {
            if fresnel.is_none() {
                println!("fresnel is none but is reflect");
                return None;
            }
            let fresnel = fresnel.unwrap();
            assert!(is_same_hemisphere(wi, wo, wm));
            let wm = if wm.z < F::zero() { -wm } else { wm };
            let ndf = self.ndf.evaluate(wm);
            let lar = smith_g2_lagarde(wi, wo, self.roughness);
            let brdf = fresnel * ndf * lar;
            Some(Vector3::new(brdf, brdf, brdf))
            // Some(Vector3::zero())
        } else {
            if wm.z <= F::zero() {
                // println!("wm.z < 0");
                return None;
            }
            assert!(wm.z > F::zero());
            assert!(fresnel.is_some());
            let fresnel = fresnel.unwrap();
            let lar = smith_g2_lagarde(wi, wo, self.roughness);
            // let etap = if wi.z > F::zero() { F::one() / eta } else { eta };
            let etap = if wi.z > F::zero() { eta } else { F::one() / eta };
            let wi_dot_wm = wi.dot(wm);
            let wo_dot_wm = wo.dot(wm);
            let vertical_component_sqr = sqr(wi_dot_wm + etap * wo_dot_wm);
            let transmit = F::one() - fresnel;
            let ndf = self.ndf.evaluate(wm);
            let mut btdf = transmit * f!(4) * lar * ndf * wi_dot_wm.abs() * wo_dot_wm.abs() * etap * etap / vertical_component_sqr;
            // if wi.z > F::zero() {
            //     btdf = btdf * (eta * eta);
            // } else {
            //     btdf = btdf / (eta * eta);
            // }
            Some(Vector3::new(btdf, btdf, btdf))
            // Some(Vector3::zero())
        }
    }

    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        if self.is_single_ior {
            self.sample_ray_single_ior(service, current_dir, 0)
        } else {
            // sample rgb independently
            let component = service.random_range(0, 3);
            let mut mask = Vector3::zero();
            mask[component as usize] = F::one();

            let result = self.sample_ray_single_ior(service, current_dir, component as usize);
            if let Some(r) = result {
                Some(BSDFSampleResult {
                    direction: r.direction,
                    weight: r.weight.mul_element_wise(mask) * f!(3),
                    next_point: r.next_point,
                })
            } else {
                None
            }
        }
    }
}

pub struct RoughDielectricBSDFMaterial<F> {
    pub roughness: F,
    pub ior: Vector3<F>,
}

impl<F> RoughDielectricBSDFMaterial<F> where F: BaseFloat {
    pub fn new(roughness: F, ior: Vector3<F>) -> Self {
        RoughDielectricBSDFMaterial {
            roughness, ior
        }
    }

    pub fn new_single_ior(roughness: F, ior: F) -> Self {
        RoughDielectricBSDFMaterial {
            roughness,
            ior: Vector3::new(ior, ior, ior)
        }
    }
}

impl<F> MaterialTrait<F> for RoughDielectricBSDFMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        let current_ior = context.get_current_ior();
        let relative_ior = if !context.back_face {
            self.ior.div_element_wise(current_ior)
        } else {
            let next_ior = context.get_next_top_ior();
            // next_ior.div_element_wise(self.ior)
            self.ior.div_element_wise(next_ior)
        };
        Some(Box::new(RoughDielectricBSDF::new(self.roughness, relative_ior)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }

    fn get_ior(&self) -> Option<Vector3<F>> {
        Some(self.ior)
    }
}