use cgmath::{BaseFloat, Vector2, Vector3};

use crate::Ray;

#[derive(Clone, Debug)]
pub struct HitRecord<T, H> {
    pub t: T,
    pub normal: Option<Vector3<T>>,
    pub uv: Option<Vector2<T>>,
    pub back_facing: Option<bool>,

    pub hit_object: Option<H>
}

impl<T, H> HitRecord<T, H> where T: BaseFloat {
    pub fn get_hit_point(&self, ray: &Ray<T>) -> Vector3<T> {
        ray.origin + ray.direction * self.t
    }

    pub fn new() -> Self {
        Self {
            t: T::infinity(),
            normal: None,
            back_facing: None,
            hit_object: None,
            uv: None,
        }
    }

    pub fn copy_except_hit_object<H2>(&self, target: &mut HitRecord<T, H2>) {
        target.t = self.t;
        target.normal = self.normal.clone();
        target.back_facing = self.back_facing.clone();
        target.uv = self.uv.clone();
    }
}

/// F: Float type
/// H: Hit data
pub trait Hittable<F, H> {
    fn hit(&self, ray: &Ray<F>, min: F, max: F)
        -> Option<HitRecord<F, H>>;

    fn is_hit(&self, ray: &Ray<F>, min: F, max: F) -> bool {
        self.hit(ray, min, max).is_some()
    }
}
