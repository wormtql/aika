use cgmath::BaseFloat;
use aika_math::Triangle;
use crate::mesh::{BoxDynVertexBuffer, SubMesh, VertexBuffer};

pub struct Mesh<V> {
    pub vertices: V,
    pub triangles: Vec<[usize; 3]>,
    pub sub_mesh: Vec<[usize; 2]>,
}

pub type DynMesh<F> = Mesh<BoxDynVertexBuffer<F>>;

impl<V> Mesh<V> where V: VertexBuffer + 'static {
    pub fn to_dyn_mesh(self) -> DynMesh<V::FloatType> {
        Mesh {
            vertices: Box::new(self.vertices),
            triangles: self.triangles,
            sub_mesh: self.sub_mesh
        }
    }
}

impl<V> Mesh<V> where V: VertexBuffer {
    pub fn get_sub_mesh(&self, index: usize) -> SubMesh<V> {
        let [a, b] = self.sub_mesh[index];
        SubMesh {
            vertices: &self.vertices,
            triangles: &self.triangles[a..b],
        }
    }

    pub fn iter_triangles(&self) -> MeshTrianglesIterator<V> {
        MeshTrianglesIterator {
            mesh: &self,
            next: 0
        }
    }

    pub fn iter_triangle_indices(&self) -> MeshTriangleIndicesIterator<V> {
        MeshTriangleIndicesIterator {
            mesh: &self,
            next: 0
        }
    }

    pub fn face_count(&self) -> usize {
        self.triangles.len()
    }
}

pub struct MeshTrianglesIterator<'a, V> {
    mesh: &'a Mesh<V>,
    next: usize,
}

impl<'a, V> Iterator for MeshTrianglesIterator<'a, V> where V: VertexBuffer {
    type Item = Triangle<V::FloatType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.mesh.triangles.len() {
            let [index1, index2, index3] = self.mesh.triangles[self.next];
            let position1 = self.mesh.vertices.get_position(index1);
            let position2 = self.mesh.vertices.get_position(index2);
            let position3 = self.mesh.vertices.get_position(index3);
            self.next += 1;

            Some(Triangle {
                a: position1,
                b: position2,
                c: position3
            })
        } else {
            None
        }
    }
}

pub struct MeshTriangleIndicesIterator<'a, V> {
    mesh: &'a Mesh<V>,
    next: usize
}

impl<'a, V> Iterator for MeshTriangleIndicesIterator<'a, V> where V: VertexBuffer {
    type Item = [usize; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.mesh.triangles.len() {
            let x = Some(self.mesh.triangles[self.next].clone());
            self.next += 1;
            x
        } else {
            None
        }
    }
}
