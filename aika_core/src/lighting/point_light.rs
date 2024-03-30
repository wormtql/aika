use cgmath::Vector3;

pub struct PointLight<F> {
    pub color: Vector3<F>,
    pub radius: Option<F>,
}
