use crate::common::types::{PointType, PrecisionType, Vector3Type};

pub trait Lighting {
    fn get_radiance(&self, point: PointType, direction: Vector3Type) -> PrecisionType;
}