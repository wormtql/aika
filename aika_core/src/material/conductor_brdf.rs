use cgmath::{BaseFloat, Vector3};
use num_traits::Zero;
use aika_math::Complex;
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::utils::{fresnel_complex};
use crate::f;
use crate::path_tracing::{ShadingContext, TracingService};

#[derive(Clone)]
pub struct ConductorBRDF<F> {
    /// Because metal will not transmit light, so the relative IOR always equals outer IOR / metal IOR
    pub relative_ior: Vector3<Complex<F>>,
}

impl<F> ConductorBRDF<F> where F: BaseFloat {
    pub fn new(relative_ior: Vector3<Complex<F>>) -> Self {
        ConductorBRDF {
            relative_ior
        }
    }
}

impl<F> BSDF<F> for ConductorBRDF<F> where F: BaseFloat {
    fn evaluate(&self, dir1: Vector3<F>, dir2: Vector3<F>) -> Option<Vector3<F>> {
        // let dir1_r = reflect(dir1, Vector3::new(F::zero(), F::zero(), F::one()));
        // let vector_one = Vector3::new(F::one(), F::one(), F::one());
        // if dir1_r == dir2 {
        //     let f1 = fresnel_complex(dir1.z, vector_one, self.relative_ior.x);
        //     let f2 = fresnel_complex(dir1.z, vector_one, self.relative_ior.y);
        //     let f3 = fresnel_complex(dir1.z, vector_one, self.relative_ior.z);
        //     Vector3::new(f1, f2, f3)
        // } else {
        //     Vector3::zero()
        // }
        Some(Vector3::zero())
    }

    fn sample_ray(&self, _service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        let reflect_dir = Vector3::new(-current_dir.x, -current_dir.y, current_dir.z);

        let vector_one = Vector3::new(F::one(), F::one(), F::one());
        let f1 = fresnel_complex(current_dir.z, F::one(), self.relative_ior.x);
        let f2 = fresnel_complex(current_dir.z, F::one(), self.relative_ior.y);
        let f3 = fresnel_complex(current_dir.z, F::one(), self.relative_ior.z);
        let value = Vector3::new(f1, f2, f3);
        // if f1 < F::zero() || f2 < F::zero() || f3 < F::zero() {
        //     println!("{:?}", value);
        // }

        Some(BSDFSampleResult {
            // pdf: vector_one,
            direction: reflect_dir,
            // value,
            // cos_theta: current_dir.z,
            weight: Vector3::new(f1, f2, f3),
            next_point: Vector3::new(f!(0), f!(0), f!(1e-6)),
        })
    }
}

pub struct ConductorBRDFMaterial<F> {
    pub relative_ior: Vector3<Complex<F>>,
}

impl<F> ConductorBRDFMaterial<F> where F: BaseFloat {
    pub fn gold() -> Self {
        ConductorBRDFMaterial {
            relative_ior: Vector3::new(
                Complex::new(f!(0.18299), f!(3.4242)),
                Complex::new(f!(0.42108), f!(2.34590)),
                Complex::new(f!(1.37340), f!(1.77040))
            )
        }
    }
}

impl<F> MaterialTrait<F> for ConductorBRDFMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        Some(Box::new(ConductorBRDF::new(self.relative_ior)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
