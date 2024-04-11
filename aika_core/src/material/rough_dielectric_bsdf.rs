use cgmath::{BaseFloat, ElementWise, InnerSpace, Vector3};
use num_traits::Zero;
use aika_math::distribution::IsotropicGGXDistribution;
use aika_math::utils::{face_forward, get_z, is_same_hemisphere, is_same_hemisphere_canonical, length_square_vector3, reflect, refract, sqr};
use crate::f;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};
use crate::utils::fresnel_dielectric;

pub struct RoughDielectricBSDF<F> {
    ndf: IsotropicGGXDistribution<F>,
    /// the ior out of the normal / ior inside the object
    relative_ior: Vector3<F>,
    is_single_ior: bool,
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
        }
    }

    pub fn sample_ray_single_ior(&self, service: &mut TracingService<F>, wo: Vector3<F>, ior_index: usize) -> Option<BSDFSampleResult<F>> {
        let eta = self.relative_ior[ior_index];
        let wm = self.ndf.sample_wm(wo, service.random_0_1(), service.random_0_1());
        let distribution_of_visible_normal = self.ndf.distribution_of_visible_normal(wo, wm);
        // println!("{:?}", eta);
        let cos_theta_i = wm.dot(wo);
        let fresnel = fresnel_dielectric(cos_theta_i, F::one(), eta).unwrap_or(F::one());
        let z = Vector3::new(F::zero(), F::zero(), F::one());
        let backface = wo.z < F::zero();

        let reflection_point_bias = if wo.z >= F::zero() {
            f!(1e-3)
        } else {
            f!(-1e-3)
        };

        let transmission = F::one() - fresnel;
        let random = service.random_0_1();
        // let random = F::one();
        // let random = F::zero();
        if random < fresnel {
            let wi = reflect(wo, wm);
            if !is_same_hemisphere_canonical(wi, wo) {
                // since we are not considering multi scattering, the ray ends there
                return None;
            }
            let pdf_reflection = distribution_of_visible_normal / (F::from(4).unwrap() * wo.dot(wm).abs()) * fresnel;
            let ndf = self.ndf.evaluate(wm);
            let smith_g2 = self.ndf.masking_shadowing(wi, wo);
            let brdf_plus_cos = fresnel * ndf * smith_g2 / (f!(4) * wo.z.abs());
            let w = brdf_plus_cos / pdf_reflection;
            // let w = F::zero();
            Some(BSDFSampleResult {
                direction: wi,
                weight: Vector3::new(w, w, w),
                next_point: Vector3::new(F::zero(), F::zero(), reflection_point_bias),
            })
        } else {
            let wi = refract(wo, wm, F::one(), eta);

            if wi.is_none() {
                println!("refract is none");
                let reflect_dir = reflect(wo, wm);
                if !is_same_hemisphere_canonical(reflect_dir, wo) {
                    return None;
                }
                let pdf = distribution_of_visible_normal / (F::from(4).unwrap() * wo.dot(wm).abs()) * fresnel;
                let ndf = self.ndf.evaluate(wm);
                let smith_g2 = self.ndf.masking_shadowing(reflect_dir, wo);
                let brdf_plus_cos = fresnel * ndf * smith_g2 / (f!(4) * wo.z.abs());
                let w = brdf_plus_cos / pdf;
                return Some(BSDFSampleResult {
                    direction: reflect(wo, wm),
                    weight: Vector3::new(w, w, w),
                    next_point: Vector3::new(F::zero(), F::zero(), reflection_point_bias)
                });
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

            let etap = if backface {
                F::one() / eta
            } else {
                eta
            };
            let denom = sqr(wi.dot(wm) + wo.dot(wm) / etap);
            let pdf = distribution_of_visible_normal * (wi.dot(wm).abs() / denom) * transmission;
            let ndf = self.ndf.evaluate(wm);
            let smith_g2 = self.ndf.masking_shadowing(wi, wo);
            let partial_solid_angle = if backface {
                eta * eta
            } else {
                F::one() / (eta * eta)
            };
            let btdf_plus_cos = transmission * ndf * smith_g2 * (wi.dot(wm) * wo.dot(wm) / (wo.z * denom)).abs() * partial_solid_angle;
            let w = btdf_plus_cos / pdf;
            Some(BSDFSampleResult {
                direction: wi,
                weight: Vector3::new(w, w, w),
                next_point: Vector3::new(F::zero(), F::zero(), -reflection_point_bias)
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
        let mut etap = F::one();
        if !reflect {
            etap = if cos_theta_i > F::zero() {
                eta
            } else {
                F::one() / eta
            };
        }
        let mut wm = wi + wo * etap;
        if cos_theta_i == F::zero() || cos_theta_o == F::zero() || length_square_vector3(wm) == F::zero() {
            return None;
        }
        wm = face_forward(wm.normalize(), get_z());
        assert!(wm.z > F::zero());
        if reflect {
            // println!("reflect, {:?}", wm);
        }

        if reflect {
            if !is_same_hemisphere(wi, wo, wm) {
                return None;
            }
        } else {
            if is_same_hemisphere(wi, wo, wm) {
                return None;
            }
        }

        let fresnel = fresnel_dielectric(wi.dot(wm), F::one(), eta).unwrap_or(F::one());
        if reflect {
            let ndf = self.ndf.evaluate(wm);
            let g2 = self.ndf.masking_shadowing(wi, wo);
            let denom = F::from(4).unwrap() * cos_theta_i * cos_theta_o;
            let f = ndf * g2 * fresnel / denom.abs();
            // let f = F::one();
            Some(Vector3::new(f, f, f))
        } else {
            let denom = sqr(wi.dot(wm) + wo.dot(wm) * etap) * cos_theta_i * cos_theta_o;
            let ndf = self.ndf.evaluate(wm);
            let g2 = self.ndf.masking_shadowing(wi, wo);
            let backface = wo.z < F::zero();
            // let partial_solid_angle = if backface {
            //     eta * eta
            // } else {
            //     F::one() / (eta * eta)
            // };
            let f = ndf * (F::one() - fresnel) * g2 * (wi.dot(wm) * wo.dot(wm) / denom).abs();
            let f = f * (etap * etap);
            // let f = F::one();
            Some(Vector3::new(f, f, f))
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