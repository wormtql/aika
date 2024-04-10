use std::cell::RefCell;
use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::*;
use crate::bvh::BVHNode;

#[derive(Debug)]
pub struct BVHTree<F, B, G, GH> {
    pub root: Rc<RefCell<BVHNode<F, B, G, GH>>>,
}

impl<B, G, F, GH> Hittable<F, Rc<G>> for BVHTree<F, B, G, GH>
where
    F: BaseFloat,
    B: Mergeable<B, Result = B> + Hittable<F, ()>,
    G: Hittable<F, GH>,
{
    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, Rc<G>>> {
        self.root.borrow().hit(ray, min, max)
    }
}
