use cgmath::{BaseFloat, Vector3};
use crate::material::{BSDF, BSDFSampleResult, MaterialTrait, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};

pub struct UniformEmit<F> {
    pub radiance: Vector3<F>
}

impl<F> UniformEmit<F> where F: BaseFloat {
    pub fn new(color: Vector3<F>) -> Self {
        UniformEmit {
            radiance: color
        }
    }
}

impl<F> BSDF<F> for UniformEmit<F> where F: BaseFloat + 'static {
    fn evaluate(&self, wi: Vector3<F>, wo: Vector3<F>) -> Option<Vector3<F>> {
        None
    }

    fn sample_ray(&self, service: &mut TracingService<F>, current_dir: Vector3<F>) -> Option<BSDFSampleResult<F>> {
        None
    }

    fn emit(&self, wo: Vector3<F>) -> Option<Vector3<F>> {
        Some(self.radiance)
    }
}

pub struct UniformEmitMaterial<F> {
    pub radiance: Vector3<F>
}

impl<F> UniformEmitMaterial<F> where F: BaseFloat + 'static {
    pub fn new(radiance: Vector3<F>) -> Self {
        Self {
            radiance
        }
    }
}

impl<F> MaterialTrait<F> for UniformEmitMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        false
    }

    fn has_bsdf(&self) -> bool {
        true
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        Some(Box::new(UniformEmit::new(self.radiance)))
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        None
    }
}
