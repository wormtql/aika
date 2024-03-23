use std::cell::RefCell;
use std::rc::Rc;

use cgmath::BaseFloat;

use aika_math::{Bounded, BoundingVolume, HitRecord, Hittable, Ray};

/// B: Bounding volume type
/// G: Geometry type
#[derive(Debug)]
pub struct BVHNode<B, G> {
    pub left: Option<Rc<RefCell<BVHNode<B, G>>>>,
    pub right: Option<Rc<RefCell<BVHNode<B, G>>>>,
    // we don't need to change the objects
    pub objects: Vec<Rc<G>>,

    pub bounding_volume: B,
}

impl<B, G, F> BVHNode<B, G>
where
    F: BaseFloat,
    B: BoundingVolume<FloatType = F>,
{
    pub fn hit_bv(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F>> {
        self.bounding_volume.hit(ray, min, max)
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none()
    }
}

impl<B, G, F> Hittable for BVHNode<B, G>
where
    F: BaseFloat,
    B: BoundingVolume<FloatType = F>,
    G: Hittable<FloatType = F>
{
    type FloatType = F;

    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F>> {
        let mut min = min;
        if self.hit_bv(ray, min, max).is_none() {
            return None;
        }
        println!("{}", 123);

        let mut max = max;
        let mut hr = HitRecord::new();
        if let Some(n) = &self.left {
            let result = n.borrow().hit(ray, min, max);
            if let Some(r) = result {
                max = r.t;
                hr = r.clone();
            }
        }
        if let Some(n) = &self.right {
            let result = n.borrow().hit(ray, min, max);
            if let Some(r) = result {
                max = r.t;
                hr = r.clone();
            }
        }
        for obj in self.objects.iter() {
            let result = obj.hit(ray, min, max);
            if let Some(r) = result {
                max = r.t;
                hr = r.clone();
            }
        }

        if hr.t == F::infinity() {
            None
        } else {
            Some(hr)
        }
    }
}