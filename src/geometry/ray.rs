use cgmath::Vector3;
use crate::common::types::PrecisionType;
// use cgm

pub struct Ray {
    pub origin: Vector3<PrecisionType>,
    pub direction: Vector3<PrecisionType>,
}
