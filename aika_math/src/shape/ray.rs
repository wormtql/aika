use cgmath::{BaseFloat, InnerSpace, Vector3};

#[derive(Debug, Clone)]
pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}

impl<F> Ray<F> where F: BaseFloat {
    pub fn new(origin: Vector3<F>, direction: Vector3<F>) -> Self {
        Self {
            origin,
            direction: direction.normalize()
        }
    }
}
