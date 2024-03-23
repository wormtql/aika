use std::rc::Rc;
use crate::geometry::traits::Geometry;

pub trait SubGeometry {
    fn get_parent_geometry(&self) -> Rc<dyn Geometry>;
}
