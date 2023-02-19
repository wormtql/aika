pub trait VertexCount {
    fn get_vertex_count(&self) -> usize;
}

pub trait FaceCount {
    fn get_face_count(&self) -> usize;
}

pub trait EdgeCount {
    fn get_edge_count(&self) -> usize;
}

pub trait CommonMesh {
    fn cube() -> Self;
}