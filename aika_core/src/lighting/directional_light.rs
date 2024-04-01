use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;

/// A directional light will be pointing at (0, 0, 1) by default
#[derive(Clone)]
pub struct DirectionalLight<F> {
    pub color: Vector3<F>,
}

impl<F> ComponentData for DirectionalLight<F> where F: BaseFloat + 'static {}

impl<F> DirectionalLight<F> where F: BaseFloat {
    pub fn new(color: Vector3<F>) -> DirectionalLight<F> {
        DirectionalLight {
            color
        }
    }
}
