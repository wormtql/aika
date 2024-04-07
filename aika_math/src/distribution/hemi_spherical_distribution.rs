use cgmath::Vector3;

pub struct HemiSphericalDistributionSampleResult<F> {
    pub dir: Vector3<F>,
    pub pdf: Vector3<F>,
}

pub trait HemiSphericalDistribution<F> {
    fn generate(&self, r1: F, r2: F) -> HemiSphericalDistributionSampleResult<F>;
}