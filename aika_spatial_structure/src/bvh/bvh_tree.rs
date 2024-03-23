use std::cell::RefCell;
use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::{Bounded, BoundingVolume, HitRecord, Hittable, Ray};
use crate::bvh::BVHNode;

#[derive(Debug)]
pub struct BVHTree<B, G> {
    pub root: Rc<RefCell<BVHNode<B, G>>>,
}

impl<B, G, F> Hittable for BVHTree<B, G>
where
    F: BaseFloat,
    G: Hittable<FloatType = F>,
    B: BoundingVolume<FloatType = F>,
{
    type FloatType = F;

    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F>> {
        self.root.borrow().hit(ray, min, max)
    }
}
