use std::rc::Rc;
use crate::bvh::traits::{BVHGeometry, BVHSplit};
use crate::common::types::PointType;
use crate::geometry::geometries::triangle::Triangle;
use crate::geometry::hittable::GeometryHitRecordData;
use crate::geometry::mesh_structure::mesh_structre_traits::{CommonMesh, FaceCount, VertexCount};
use crate::geometry::vertex::Vertex;

pub struct MeshStructureTriangleNeighbor<VertexData> {
    // vertex indices
    triangles: Vec<[usize; 3]>,
    // indices of neighbor triangles
    neighbor_triangles: Vec<[Option<usize>; 3]>,
    // index of any adjacent triangle
    vertex_adjacent_triangle: Vec<Option<usize>>,
    vertex_data: Vec<VertexData>,
    vertex_position: Vec<PointType>,
}

impl<VertexData> VertexCount for MeshStructureTriangleNeighbor<VertexData> {
    fn get_vertex_count(&self) -> usize {
        self.vertex_data.len()
    }
}

impl<VertexData> FaceCount for MeshStructureTriangleNeighbor<VertexData> {
    fn get_face_count(&self) -> usize {
        self.triangles.len()
    }
}

impl<VertexData: Clone + 'static> BVHSplit<()> for MeshStructureTriangleNeighbor<VertexData> {
    fn split(self: Rc<Self>) -> Vec<Rc<dyn BVHGeometry<()>>> {
        let mut result: Vec<Rc<dyn BVHGeometry<()>>> = Vec::new();

        for tri in self.triangles.iter() {
            let a_index = tri[0];
            let b_index = tri[1];
            let c_index = tri[2];

            let triangle = Triangle {
                a: Vertex {
                    position: self.vertex_position[a_index],
                    data: self.vertex_data[a_index].clone(),
                },
                b: Vertex {
                    position: self.vertex_position[b_index],
                    data: self.vertex_data[b_index].clone(),
                },
                c: Vertex {
                    position: self.vertex_position[c_index],
                    data: self.vertex_data[c_index].clone(),
                }
            };

            result.push(Rc::new(triangle));
        }

        result
    }
}

impl<VertexData: Clone + 'static> BVHSplit<GeometryHitRecordData> for MeshStructureTriangleNeighbor<VertexData> {
    fn split(self: Rc<Self>) -> Vec<Rc<dyn BVHGeometry<GeometryHitRecordData>>> {
        let mut result: Vec<Rc<dyn BVHGeometry<GeometryHitRecordData>>> = Vec::new();

        for tri in self.triangles.iter() {
            let a_index = tri[0];
            let b_index = tri[1];
            let c_index = tri[2];

            let triangle = Triangle {
                a: Vertex {
                    position: self.vertex_position[a_index],
                    data: self.vertex_data[a_index].clone(),
                },
                b: Vertex {
                    position: self.vertex_position[b_index],
                    data: self.vertex_data[b_index].clone(),
                },
                c: Vertex {
                    position: self.vertex_position[c_index],
                    data: self.vertex_data[c_index].clone(),
                }
            };

            result.push(Rc::new(triangle));
        }

        result
    }
}

impl<VertexData> CommonMesh for  MeshStructureTriangleNeighbor<VertexData> {
    fn cube() -> Self {
        let
    }
}