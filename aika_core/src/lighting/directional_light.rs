use cgmath::{BaseFloat, Vector3};
use crate::component::ComponentData;

/// A directional light will be pointing at (0, 0, 1) by default
pub struct DirectionalLight<F> {
    pub color: Vector3<F>,
}

impl<F> ComponentData for DirectionalLight<F> where F: BaseFloat + 'static {}

