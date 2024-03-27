use cgmath::{BaseFloat, Vector3};

use crate::Ray;

#[derive(Clone, Debug)]
pub struct HitRecord<T> {
    pub t: T,
    pub normal: Option<Vector3<T>>,
    pub back_facing: Option<bool>,
}

impl<T> HitRecord<T> where T: BaseFloat {
    pub fn get_hit_point(&self, ray: &Ray<T>) -> Vector3<T> {
        ray.origin + ray.direction * self.t
    }

    pub fn new() -> Self {
        Self {
            t: T::infinity(),
            normal: None,
            back_facing: None,
        }
    }
}

pub trait Hittable {
    type FloatType;

    fn hit(&self, ray: &Ray<Self::FloatType>, min: Self::FloatType, max: Self::FloatType) -> Option<HitRecord<Self::FloatType>>;

    fn is_hit(&self, ray: &Ray<Self::FloatType>, min: Self::FloatType, max: Self::FloatType) -> bool {
        self.hit(ray, min, max).is_some()
    }
}
