use cgmath::BaseFloat;
use crate::mesh::{BoxDynVertexBuffer, SubMesh, VertexBuffer};

pub struct Mesh<V> {
    pub vertices: V,
    pub triangles: Vec<[usize; 3]>,
    pub sub_mesh: Vec<[usize; 2]>,
}

pub type DynMesh<F> = Mesh<BoxDynVertexBuffer<F>>;

impl<V> Mesh<V> where V: VertexBuffer {
    pub fn get_sub_mesh(&self, index: usize) -> SubMesh<V> {
        let [a, b] = self.sub_mesh[index];
        SubMesh {
            vertices: &self.vertices,
            triangles: &self.triangles[a..b],
        }
    }
}
