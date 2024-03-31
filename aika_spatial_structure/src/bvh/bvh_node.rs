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
    pub fn hit_bv(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, ()>> {
        let temp = self.bounding_volume.hit(ray, min, max);
        let mut result = HitRecord::new();
        if let Some(r) = temp {
            r.copy_except_hit_object(&mut result);
            Some(result)
        } else {
            None
        }
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
    type HitObjectType = Rc<G>;

    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, Rc<G>>> {
        let hit_bv_result = self.hit_bv(ray, min, max);
        if hit_bv_result.is_none() {
            return None;
        }

        let mut max = max;
        let mut hr: HitRecord<F, Rc<G>> = HitRecord::new();
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
                r.copy_except_hit_object(&mut hr);
                hr.hit_object = Some(obj.clone());
            }
        }

        if hr.t == F::infinity() {
            None
        } else {
            Some(hr)
        }
    }
}