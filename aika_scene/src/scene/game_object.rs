use cgmath::BaseFloat;
use crate::mesh::{CommonVertex, DynMesh, Mesh, PlaneMesh, VertexBuffer};
use crate::scene::Transform;


pub struct GameObject<F> {
    pub mesh: Option<DynMesh<F>>,
    pub transform: Transform<F>,
    // todo material
}

impl<F> GameObject<F> where F: BaseFloat + 'static {
    pub fn new_plane(width_x: F, width_y: F) -> Self {
        let mesh = PlaneMesh::create_plane_mesh(width_x, width_y);
        let transform = Transform::default();
        Self {
            mesh: Some(mesh),
            transform
        }
    }
}
