use std::any::Any;
use std::rc::Rc;
use cgmath::Vector3;
use crate::common::types::{PointType, PrecisionType};
use crate::geometry::ray::Ray;

pub struct HitRecord<HitData> {
    pub t: PrecisionType,
    pub hit_object: Rc<dyn Hittable<HitData>>,
    pub data: HitData,
}

pub struct GeometryHitRecordData {
    pub normal: Vector3<PrecisionType>,
}

impl<HitData> HitRecord<HitData> {
    pub fn get_closer(self, other: Self) -> Self {
        if self.t > other.t {
            other
        } else {
            self
        }
    }
}

/// T: Hit result data
pub trait Hittable<HitData> {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<HitData>>;

    fn is_hit(self: Rc<Self>, ray: &Ray) -> bool {
        self.hit(ray).is_some()
    }
}