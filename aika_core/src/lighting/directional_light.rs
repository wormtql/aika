use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;
use crate::lighting::{Light, LightSampleContext, LightSampleResult};
use crate::path_tracing::TracingService;

/// A directional light will be pointing at (0, 0, 1) by default
#[derive(Clone)]
pub struct DirectionalLightComponent<F> {
    pub color: Vector3<F>,
}

impl<F> ComponentData for DirectionalLightComponent<F> where F: BaseFloat + 'static {}

impl<F> DirectionalLightComponent<F> where F: BaseFloat {
    pub fn new(color: Vector3<F>) -> DirectionalLightComponent<F> {
        DirectionalLightComponent {
            color
        }
    }
}

pub struct DirectionalLight<F> {
    pub dir: Vector3<F>,
    pub color: Vector3<F>,
}

impl<F> Light<F> for DirectionalLight<F> where F: BaseFloat {
    fn get_radiance(&self, position: Vector3<F>, wi: Vector3<F>) -> Option<Vector3<F>> {
        None
    }

    fn sample_light(&self, service: &TracingService<F>, context: &LightSampleContext<F>) -> Option<LightSampleResult<F>> {
        Some(LightSampleResult {
            wi: -self.dir,
            weight: Vector3::new(F::one(), F::one(), F::one()),
            radiance: self.color,
            distance: F::infinity(),
            point: None,
        })
    }

    fn get_total_power(&self) -> F {
        todo!()
    }
}