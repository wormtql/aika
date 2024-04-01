use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;

#[derive(Clone)]
pub struct PointLight<F> {
    pub color: Vector3<F>,
    pub radius: Option<F>,
}

impl<F> ComponentData for PointLight<F> where F: BaseFloat + 'static {}
