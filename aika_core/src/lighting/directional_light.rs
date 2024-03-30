use cgmath::Vector3;

/// A directional light will be pointing at (0, 0, 1) by default
pub struct DirectionalLight<F> {
    pub color: Vector3<F>,
}
