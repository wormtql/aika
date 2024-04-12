use cgmath::{BaseFloat, InnerSpace, Vector3};
use aika_math::utils::length_square_vector3;
use crate::component::ComponentData;
use crate::lighting::{Light, LightSampleContext, LightSampleResult};
use crate::path_tracing::TracingService;

#[derive(Clone)]
pub struct PointLightComponent<F> {
    pub color: Vector3<F>,
    pub radius: Option<F>,
}

impl<F> ComponentData for PointLightComponent<F> where F: BaseFloat + 'static {}

pub struct PointLight<F> {
    pub position: Vector3<F>,
    pub color: Vector3<F>,
}

impl<F> Light<F> for PointLight<F> where F: BaseFloat {
    fn get_radiance(&self, position: Vector3<F>, wi: Vector3<F>) -> Option<Vector3<F>> {
        None
    }

    fn sample_light(&self, service: &TracingService<F>, context: &LightSampleContext<F>) -> Option<LightSampleResult<F>> {
        let wi = (self.position - context.position).normalize();
        let r2 = length_square_vector3(self.position - context.position);
        Some(LightSampleResult {
            wi,
            weight: Vector3::new(F::one(), F::one(), F::one()),
            radiance: self.color / r2,
            distance: r2.sqrt(),
            point: Some(self.position),
        })
    }

    fn get_total_power(&self) -> F {
        todo!()
    }
}