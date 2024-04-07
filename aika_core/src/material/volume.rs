use cgmath::Vector3;
use crate::path_tracing::{ShadingContext, TracingService};
use anyhow::Result;

pub struct VolumeSampleResult<F> {
    /// exit direction
    pub next_direction: Vector3<F>,
    /// the exit point
    pub point: Vector3<F>,
    pub weight: Vector3<F>,
    // maybe a nested volume, or some other objects
    // pub hit_object:
}

pub trait VolumeTrait<F> {
    fn transmittance(&self, p1: Vector3<F>, p2: Vector3<F>) -> Vector3<F>;

    fn emit(&self, p1: Vector3<F>, p2: Vector3<F>) -> Vector3<F>;

    /// Returns (pdf, direction)
    fn sample_ray(
        &self,
        tracing_service: &TracingService<F>,
        shading_context: &ShadingContext<F>,
        current_dir: Vector3<F>
    ) -> Result<VolumeSampleResult<F>>;
}
