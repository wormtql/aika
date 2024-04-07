use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use cgmath::BaseFloat;
use crate::component::{ComponentData, MeshFilter, Transform, Component};
use crate::mesh::{CommonVertex, DynMesh, Mesh, PlaneMesh, VertexBuffer};
use anyhow::Result;

pub struct GameObjectInternal<F> {
    pub components: HashMap<TypeId, Component<F>>,
    pub name: String,
}

pub struct GameObject<F> {
    pub go: Rc<RefCell<GameObjectInternal<F>>>,
}

impl<F> GameObject<F> where F: BaseFloat {
    pub fn add_component<C: ComponentData>(&mut self, component: Component<F>) {
        let type_id = TypeId::of::<C>();
        self.go.borrow_mut().components.insert(type_id, component);
    }

    pub fn set_name(&mut self, name: &str) {
        self.go.borrow_mut().name = String::from(name);
    }
}

impl<F> GameObject<F> where F: BaseFloat + 'static {
    /// create a game object without any component
    pub fn new_empty(name: String) -> GameObject<F> {
        let go = Rc::new(RefCell::new(GameObjectInternal {
            components: HashMap::new(),
            name,
            // _float_phantom: PhantomData
        }));

        GameObject {
            go
        }
    }

    /// create a game object with only transform component
    pub fn new_with_transform(name: String) -> GameObject<F> {
        let mut go = GameObject::new_empty(name);
        // let x = (*go)
        // GameObjectInternal::add_component_owned(go.clone(), Transform::default());
        go.add_component_owned::<Transform<F>>(Transform::default());
        go
    }

    pub fn new_plane(name: String, width_x: F, width_y: F) -> GameObject<F> {
        let mesh = PlaneMesh::create_plane_mesh(width_x, width_y);
        let mut go = GameObject::new_empty(name);
        let mesh_filter = MeshFilter {
            mesh
        };

        go.add_component_owned(mesh_filter);
        go
    }
}

impl<F> GameObject<F> where F: BaseFloat + 'static {
    pub fn add_component_owned<C: ComponentData>(&mut self, component: C) {
        let component = Component::new_owned(self.clone(), component);
        self.add_component::<C>(component);
    }

    pub fn get_component<C: ComponentData>(&self) -> Result<Component<F>> {
        let type_id = TypeId::of::<C>();
        let borrow = self.go.borrow();
        if borrow.components.contains_key(&type_id) {
            let c = borrow.components.get(&type_id).unwrap();
            return Ok(c.clone());
        }

        Err(anyhow::anyhow!("Component `{:?}` found", type_id))
    }

    pub fn has_component<C: ComponentData>(&self) -> bool {
        let type_id = TypeId::of::<C>();
        self.go.borrow().components.contains_key(&type_id)
    }

    pub fn get_transform(&self) -> Option<Transform<F>> {
        let component = self.get_component::<Transform<F>>();
        if let Ok(c) = component {
            let borrow = c.downcast::<Transform<F>>();
            Some(borrow.clone())
        } else {
            None
        }
    }
}

impl<F> Clone for GameObject<F> {
    fn clone(&self) -> Self {
        GameObject {
            go: self.go.clone()
        }
    }
}
