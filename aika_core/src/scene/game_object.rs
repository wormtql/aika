use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use cgmath::BaseFloat;
use crate::component::{Component, MeshFilter, Transform};
use crate::mesh::{CommonVertex, DynMesh, Mesh, PlaneMesh, VertexBuffer};
use anyhow::Result;

pub struct GameObject<F> {
    pub components: HashMap<TypeId, Rc<RefCell<Component<F>>>>,
    // pub _float_phantom: PhantomData<F>,
    // pub mesh: Option<DynMesh<F>>,
    // pub transform: Transform<F>,
    // todo material
}

// constructors
impl<F> GameObject<F> where F: BaseFloat + 'static {
    /// create a game object without any component
    pub fn new_empty() -> Rc<RefCell<GameObject<F>>> {
        Rc::new(RefCell::new(GameObject {
            components: HashMap::new(),
            // _float_phantom: PhantomData
        }))
    }

    /// create a game object with only transform component
    pub fn new_with_transform() -> Rc<RefCell<GameObject<F>>> {
        let mut go = GameObject::new_empty();
        GameObject::add_component_owned(go.clone(), Transform::default());
        go
    }

    pub fn new_plane(width_x: F, width_y: F) -> Rc<RefCell<GameObject<F>>> {
        let mesh = PlaneMesh::create_plane_mesh(width_x, width_y);
        let mut go = GameObject::new_with_transform();
        let mesh_filter = MeshFilter {
            mesh
        };
        GameObject::add_component_owned(go.clone(), mesh_filter);
        go
    }
}

// Component CRUD
impl<F> GameObject<F> where F: BaseFloat + 'static {
    pub fn add_component<C: Any>(self: Rc<RefCell<GameObject<F>>>, component: Rc<RefCell<Component<F>>>) {
        let type_id = (*component.borrow()).type_id();
        self.borrow_mut().components.insert(type_id, component);
    }

    pub fn add_component_owned<C: Any + Sized>(self: Rc<RefCell<GameObject<F>>>, component: C) {
        let c: Rc<RefCell<dyn Any>> = Rc::new(RefCell::new(component));
        GameObject::add_component(self, c);
    }

    pub fn get_component<C: Any>(self: Rc<RefCell<GameObject<F>>>) -> Result<Rc<RefCell<Component<F>>>> {
        let type_id = TypeId::of::<C>();
        if self.borrow().components.contains_key(&type_id) {
            let c = self.borrow().components.get(&type_id).unwrap();
            return Ok(c.clone());
        }

        Err(anyhow::anyhow!("Component `{:?}` found", type_id))
    }

    pub fn has_component<C: Any>(self: Rc<RefCell<GameObject<F>>>) -> bool {
        let type_id = TypeId::of::<C>();
        self.borrow().components.contains_key(&type_id)
    }
}
