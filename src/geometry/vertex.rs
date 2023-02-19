use cgmath::{Vector2, Vector3};
use crate::common::types::{PointType, PrecisionType};

pub struct Vertex<VertexData> {
    pub position: PointType,
    pub data: VertexData,
}

pub struct GeometricVertexData {
    pub normal: Option<Vector3<PrecisionType>>,
    pub uv: Option<Vector2<PrecisionType>>,

}