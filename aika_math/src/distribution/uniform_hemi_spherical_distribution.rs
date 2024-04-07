use std::f32::consts::PI;
use std::marker::PhantomData;
use cgmath::{BaseFloat, Vector3};
use crate::distribution::{HemiSphericalDistribution, HemiSphericalDistributionSampleResult};

pub struct UniformHemiSphericalDistribution<F> {
    _float_phantom: PhantomData<F>
}

impl<F> HemiSphericalDistribution<F> for UniformHemiSphericalDistribution<F> where F: BaseFloat {
    fn generate(&self, r1: F, r2: F) -> HemiSphericalDistributionSampleResult<F> {
        let pi2 = F::from(PI * 2.0).unwrap();
        let phi = pi2 * r1;
        let cos_theta = F::one() - r2;
        let sin_theta = (F::one() - cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let dir = Vector3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
        let pdf = F::one() / pi2;
        HemiSphericalDistributionSampleResult {
            dir,
            pdf: Vector3::new(pdf, pdf, pdf)
        }
    }
}
