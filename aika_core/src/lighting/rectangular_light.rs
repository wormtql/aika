use cgmath::{BaseFloat, InnerSpace, Matrix4, Quaternion, Vector3};
use aika_math::{Rectangle, SampleShape};
use aika_math::utils::length_vector3;
use crate::component::ComponentData;
use crate::lighting::{Light, LightSampleContext, LightSampleResult};
use crate::path_tracing::TracingService;

pub struct RectangularLightComponent<F> {
    pub x_width: F,
    pub y_width: F,
    pub color: Vector3<F>,
    pub two_sided: bool,
}

impl<F> ComponentData for RectangularLightComponent<F> where F: BaseFloat + 'static {}

impl<F> RectangularLightComponent<F> where F: BaseFloat {
    pub fn new(x_width: F, y_width: F, color: Vector3<F>, two_sided: bool) -> Self {
        Self {
            x_width,
            y_width,
            color,
            two_sided
        }
    }
}

pub struct RectangularLight<F> {
    pub x_width: F,
    pub y_width: F,
    pub color: Vector3<F>,
    pub two_sided: bool,
    pub position: Vector3<F>,
    pub rotation: Quaternion<F>,
}

impl<F> Light<F> for RectangularLight<F> where F: BaseFloat + 'static {
    fn get_radiance(&self, position: Vector3<F>, wi: Vector3<F>) -> Option<Vector3<F>> {
        Some(self.color)
    }

    fn sample_light(&self, service: &TracingService<F>, context: &LightSampleContext<F>) -> Option<LightSampleResult<F>> {
        let rect = Rectangle::new(self.x_width, self.y_width, self.position, self.rotation);
        let rect_sample_result = rect.sample_shape(service.random_0_1(), service.random_0_1())?;

        let dir = rect_sample_result.position - context.position;
        let wi = dir.normalize();
        let dis = length_vector3(dir);
        let dis2 = dis * dis;

        if !self.two_sided && rect_sample_result.normal.dot(wi) >= F::zero() {
            return None;
        }

        let w = rect_sample_result.normal.dot(wi).abs() / dis2 / rect_sample_result.pdf;

        Some(LightSampleResult {
            wi,
            weight: Vector3::new(w, w, w),
            radiance: self.color,
            distance: dis,
            point: Some(rect_sample_result.position),
        })
    }

    fn get_total_power(&self) -> F {
        todo!()
    }
}