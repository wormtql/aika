/// V: Vertex buffer type
pub struct SubMesh<'a, V> {
    /// a slice of original mesh
    pub vertices: &'a V,

    pub triangles: &'a [[usize; 3]],
}
