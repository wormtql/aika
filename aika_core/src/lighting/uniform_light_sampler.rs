use cgmath::BaseFloat;
use crate::lighting::{Light, LightSampleResult};
use crate::path_tracing::{ShadingContext, TracingService};

pub struct UniformLightSampler<F> {
    lights: Vec<Box<dyn Light<F>>>,
}

impl<F> UniformLightSampler<F> where F: BaseFloat + 'static {
    pub fn new() -> Self {
        UniformLightSampler {
            lights: Vec::new()
        }
    }

    pub fn add_light(&mut self, light: Box<dyn Light<F>>) {
        self.lights.push(light);
    }

    pub fn sample_light(&self, service: &TracingService<F>, context: &ShadingContext<F>) -> Option<LightSampleResult<F>> {
        if self.lights.len() == 0 {
            return None;
        }

        let len = self.lights.len();
        let random_index = service.random_range(0, len as i32) as usize;
        let light = &self.lights[random_index];
        let mut sample_result = light.sample_light(service, context.point)?;
        sample_result.pdf *= F::one() / F::from(len).unwrap();
        Some(sample_result)
    }
}
