use std::ops::Mul;
use cgmath::{Matrix4, Vector3, Vector4};
use crate::common::types::{PrecisionType, TransformMatrixType};

pub trait Transformable {
    fn transform(&self, matrix: &TransformMatrixType) -> Self;
}

impl Transformable for Vector3<PrecisionType> {
    fn transform(&self, matrix: &TransformMatrixType) -> Self {
        let coord = Vector4::new(self.x, self.y, self.z, 1.0);
        let temp = matrix * coord;
        temp.xyz()
    }
}