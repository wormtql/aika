use cgmath::{BaseFloat, InnerSpace, Vector3};
use aika_math::{SampleShape, Sphere};
use aika_math::utils::length_vector3;
use crate::component::ComponentData;
use crate::lighting::{Light, LightSampleResult};
use crate::path_tracing::TracingService;

pub struct SphericalLightComponent<F> {
    pub radius: F,
    pub color: Vector3<F>,
}

impl<F> SphericalLightComponent<F> where F: BaseFloat {
    pub fn new(radius: F, color: Vector3<F>) -> Self {
        SphericalLightComponent {
            radius, color
        }
    }
}

impl<F> ComponentData for SphericalLightComponent<F> where F: BaseFloat + 'static {}

pub struct SphericalLight<F> {
    pub position: Vector3<F>,
    pub radius: F,
    pub color: Vector3<F>,
}

impl<F> Light<F> for SphericalLight<F> where F: BaseFloat + 'static {
    fn get_radiance(&self, position: Vector3<F>, wi: Vector3<F>) -> Option<Vector3<F>> {
        Some(self.color)
    }

    fn sample_light(&self, service: &TracingService<F>, position: Vector3<F>) -> Option<LightSampleResult<F>> {
        let sphere = Sphere::new(self.position, self.radius);
        let sample_point = sphere.sample_shape(service.random_0_1(), service.random_0_1())?;

        let dir = sample_point.position - position;
        let wi = dir.normalize();
        let length = length_vector3(dir);

        // if sample_point.normal.dot(wi) > F::zero() {
        //     return None;
        // }

        let pdf = sample_point.pdf * (length * length) / (sample_point.normal.dot(wi).abs()) * F::from(2).unwrap();
        Some(LightSampleResult {
            wi,
            pdf: Vector3::new(pdf, pdf, pdf),
            radiance: self.color,
            distance: length,
        })
    }

    fn get_total_power(&self) -> F {
        todo!()
    }
}