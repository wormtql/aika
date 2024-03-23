use crate::aabb::AABB;
use crate::Hittable;

pub trait Bounded<B> {
    fn get_bv(&self) -> B;
}

pub trait BoundingVolume: Hittable {
    fn merge(&self, other: &Self) -> Self;
}
