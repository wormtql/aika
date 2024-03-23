use cgmath::Vector3;

pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}
