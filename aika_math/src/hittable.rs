use cgmath::{BaseFloat, Vector3};

use crate::Ray;

pub struct HitRecord<T> {
    pub t: T,
    pub normal: Option<Vector3<T>>,
    pub back_facing: Option<bool>,
}

impl<T> HitRecord<T> where T: BaseFloat {
    pub fn get_hit_point(&self, ray: &Ray<T>) -> Vector3<T> {
        ray.origin + ray.direction * self.t
    }
}

pub trait Hittable<T> {
    fn hit(&self, ray: &Ray<T>, min: T, max: T) -> Option<HitRecord<T>>;

    fn is_hit(&self, ray: &Ray<T>, min: T, max: T) -> bool {
        self.hit(ray, min, max).is_some()
    }
}
