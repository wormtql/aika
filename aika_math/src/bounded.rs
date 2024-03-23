use crate::aabb::AABB;

pub trait HaveAABB<T> {
    fn get_aabb(&self) -> AABB<T>;
}
