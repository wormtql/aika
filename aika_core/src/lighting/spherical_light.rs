use cgmath::{BaseFloat, InnerSpace, Vector2, Vector3};
use aika_math::{HaveArea, SampleShape, Sphere};
use aika_math::utils::{length_square_vector3, length_vector3};
use crate::component::ComponentData;
use crate::f;
use crate::lighting::{Light, LightSampleContext, LightSampleResult};
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

    fn sample_light(&self, service: &TracingService<F>, context: &LightSampleContext<F>) -> Option<LightSampleResult<F>> {
        let sphere = Sphere::new(self.position, self.radius);
        let sample_result = sphere.sample_shape_solid_angle(
            Vector2::new(service.random_0_1(), service.random_0_1()),
            context.position,
            context.normal
        )?;
        // let sample_result = sphere.sample_shape(service.random_0_1(), service.random_0_1())?;

        let dir = sample_result.position - context.position;
        let length2 = length_square_vector3(dir);
        if length2 == F::zero() {
            return None;
        }
        let wi = dir.normalize();

        // if sample_point.normal.dot(wi) > F::zero() {
        //     return None;
        // }

        // let w = sample_point.normal.dot(wi).abs() / (length * length) * area * f!(0.5);
        // let w = area * f!(0.5);
        let w = F::one() / sample_result.pdf;
        Some(LightSampleResult {
            wi,
            weight: Vector3::new(w, w, w),
            radiance: self.color,
            distance: length2.sqrt(),
            point: Some(sample_result.position),
        })
    }

    fn get_total_power(&self) -> F {
        todo!()
    }
}