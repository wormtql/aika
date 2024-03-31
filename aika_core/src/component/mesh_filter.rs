use std::cell::RefCell;
use std::rc::Rc;
use cgmath::BaseFloat;
use crate::component::{ComponentData};
use crate::mesh::DynMesh;

pub struct MeshFilter<F> {
    // pub mesh: Rc<RefCell<DynMesh<F>>>,
    pub mesh: DynMesh<F>,
}

impl<F> MeshFilter<F> where F: BaseFloat {
    pub fn new(mesh: DynMesh<F>) -> Self {
        Self {
            mesh
        }
    }
}

impl<F> ComponentData for MeshFilter<F> where F: BaseFloat + 'static {}
