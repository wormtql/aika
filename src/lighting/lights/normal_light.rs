use std::any::Any;
use std::rc::Rc;
use crate::bvh::traits::BVHSplit;
use crate::common::types::{PointType, PrecisionType, Vector3Type};
use crate::geometry::hittable::{HitRecord, Hittable, PathTracingHitRecordData};
use crate::geometry::ray::Ray;
use crate::geometry::traits::Geometry;
use crate::lighting::lighting::Lighting;

pub struct NormalLight {
    pub geometry: Rc<dyn Any>,
}

impl Lighting for NormalLight {
    fn get_radiance(&self, point: PointType, direction: Vector3Type) -> PrecisionType {
        1.0
    }
}

impl BVHSplit<PathTracingHitRecordData> for NormalLight {

}