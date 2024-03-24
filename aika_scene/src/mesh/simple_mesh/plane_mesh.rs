use cgmath::{BaseFloat, Vector2, Vector3};
use crate::mesh::{DynMesh, Mesh};
use crate::mesh::CommonVertex;

pub struct PlaneMesh;

impl PlaneMesh {
    pub fn create_plane_mesh<F>(edge_x: F, edge_y: F) -> DynMesh<F> where F: BaseFloat + 'static {
        let mut vertices = Vec::new();
        let two = F::from(2.0).unwrap();
        let x2 = edge_x / two;
        let y2 = edge_y / two;
        let z = F::zero();
        let o = F::one();
        {
            let mut v: CommonVertex<F> = CommonVertex::new();
            v.position = Vector3::new(x2, y2, z);
            v.normal = Some(Vector3::new(z, z, o));
            v.uv0 = Some(Vector2::new(o, o));
            vertices.push(v);
        }
        {
            let mut v: CommonVertex<F> = CommonVertex::new();
            v.position = Vector3::new(-x2, y2, z);
            v.normal = Some(Vector3::new(z, z, o));
            v.uv0 = Some(Vector2::new(z, o));
            vertices.push(v);
        }
        {
            let mut v: CommonVertex<F> = CommonVertex::new();
            v.position = Vector3::new(-x2, -y2, z);
            v.normal = Some(Vector3::new(z, z, o));
            v.uv0 = Some(Vector2::new(z, z));
            vertices.push(v);
        }
        {
            let mut v: CommonVertex<F> = CommonVertex::new();
            v.position = Vector3::new(x2, -y2, z);
            v.normal = Some(Vector3::new(z, z, o));
            v.uv0 = Some(Vector2::new(o, z));
            vertices.push(v);
        }

        let triangles = vec![
            [0, 1, 2],
            [1, 2, 3],
        ];

        Mesh {
            vertices: Box::new(vertices),
            sub_mesh: vec![[0, 4]],
            triangles
        }
    }
}