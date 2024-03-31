use std::any::Any;
use std::cell::{Ref, RefCell};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use cgmath::BaseFloat;
use crate::scene::{GameObject, GameObjectInternal};
use anyhow::{anyhow, Result};

struct ComponentInternal<F, C> {
    pub game_object: Weak<RefCell<GameObjectInternal<F>>>,
    pub data: C,
}

impl<F, C> Deref for ComponentInternal<F, C> where F: BaseFloat, C: ComponentData {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<F, C> DerefMut for ComponentInternal<F, C> where F: BaseFloat, C: ComponentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// A marker trait
pub trait ComponentData: Any + 'static {}

type DynComponent<F> = ComponentInternal<F, Box<dyn Any>>;

pub struct Component<F> {
    c: Rc<RefCell<DynComponent<F>>>,
}

pub struct ComponentDowncastRef<'a, F, C> where F: 'a, C: ComponentData {
    r: Ref<'a, DynComponent<F>>,
    _phantom: PhantomData<C>
}

impl<'a, F, C> Deref for ComponentDowncastRef<'a, F, C> where F: 'a, C: ComponentData {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        let downcast = self.r.data.downcast_ref::<C>();
        downcast.unwrap()
    }
}

impl<'a, F, C> DerefMut for ComponentDowncastRef<'a, F, C> where F: 'a, C: ComponentData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let downcast = self.r.data.downcast_mut::<C>();
        downcast.unwrap()
    }
}

impl<F> Component<F> where F: BaseFloat {
    pub fn game_object(&self) -> Option<GameObject<F>> {
        let go_internal = self.c.borrow().game_object.upgrade()?;
        Some(GameObject {
            go: go_internal
        })
    }

    pub fn downcast<C: ComponentData>(&self) -> ComponentDowncastRef<'_, F, C> {
        let borrow = self.c.borrow();
        ComponentDowncastRef {
            r: borrow,
            _phantom: PhantomData
        }
    }

    pub fn new_owned<C: ComponentData>(go: GameObject<F>, data: C) -> Component<F> {
        let c: Box<dyn Any> = Box::new(data);
        let internal_component = ComponentInternal {
            game_object: Rc::downgrade(&go.go),
            data: c
        };
        Component {
            c: Rc::new(RefCell::new(internal_component))
        }
    }
}

impl<F> Clone for Component<F> {
    fn clone(&self) -> Self {
        Component {
            c: self.c.clone()
        }
    }
}