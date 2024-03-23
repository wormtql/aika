pub use aabb::AABB;
pub use bounded::HaveAABB;
pub use hittable::{HitRecord, Hittable};
pub use ray::Ray;
pub use transformable::Transformable;

mod aabb;
mod bounded;
mod transformable;
mod hittable;
// mod triangle;
mod ray;