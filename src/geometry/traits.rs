use std::rc::Rc;
use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::hittable::Hittable;
use crate::geometry::ray::Ray;

pub trait Geometry<T>: Bounded + Hittable<T> {
}

pub trait Bounded {
    fn bound(&self) -> BoundingBox;
}


