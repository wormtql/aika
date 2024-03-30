use std::cell::RefCell;
use std::rc::Rc;
use crate::component::Component;
use crate::mesh::DynMesh;

pub struct MeshFilter<F> {
    // pub mesh: Rc<RefCell<DynMesh<F>>>,
    pub mesh: DynMesh<F>,
}
