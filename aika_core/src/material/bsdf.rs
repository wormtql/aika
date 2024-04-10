use cgmath::{BaseFloat, ElementWise, Vector3};
use anyhow::Result;
use crate::path_tracing::TracingService;

/// all in local frame, and points out
pub struct BSDFSampleResult<F> {
    // pub pdf: Vector3<F>,
    pub direction: Vector3<F>,
    // pub value: Vector3<F>,
    /// cos theta of the sampled dir
    pub weight: Vector3<F>,
    pub next_point: Vector3<F>,
}

impl<F> BSDFSampleResult<F> where F: BaseFloat {
    pub fn get_weight(&self) -> Vector3<F> {
        self.weight
        // let one = Vector3::new(F::one(), F::one(), F::one());
        // one.div_element_wise(self.pdf).mul_element_wise(self.value) * self.cos_theta
    }
}

pub trait BSDF<F> {
    /// All the directions are in tangent space
    /// We follow the convention that all directions points out of the shading point
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>>;

    /// Returns (pdf, direction), in tangent space
    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>>;

    fn emit(&self, wo: Vector3<F>) -> Option<Vector3<F>> {
        None
    }
}
