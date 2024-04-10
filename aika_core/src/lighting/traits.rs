use cgmath::Vector3;
use crate::path_tracing::TracingService;

pub struct LightSampleResult<F> {
    pub wi: Vector3<F>,
    pub pdf: Vector3<F>,
    pub radiance: Vector3<F>,
    pub distance: F,
}

pub trait Light<F> {
    fn get_radiance(&self, position: Vector3<F>, wi: Vector3<F>) -> Option<Vector3<F>>;

    fn sample_light(&self, service: &TracingService<F>, position: Vector3<F>) -> Option<LightSampleResult<F>>;

    fn get_total_power(&self) -> F;
}
