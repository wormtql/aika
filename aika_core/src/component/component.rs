use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::scene::GameObject;

pub struct Component<F> {
    pub game_object: Weak<RefCell<GameObject<F>>>,
    pub data: RefCell<dyn Any>,
}
