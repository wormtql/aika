use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use cgmath::BaseFloat;

use aika_math::*;

/// F: Float type
/// B: Bounding volume type
/// G: Geometry type
/// GH: Geometry Hittable data
#[derive(Debug)]
pub struct BVHNode<F, B, G, GH> {
    pub left: Option<Rc<RefCell<BVHNode<F, B, G, GH>>>>,
    pub right: Option<Rc<RefCell<BVHNode<F, B, G, GH>>>>,
    // we don't need to change the objects
    pub objects: Vec<Rc<G>>,

    pub bounding_volume: B,

    pub _float_phantom: PhantomData<F>,
}

impl<B, G, F, GH> BVHNode<F, B, G, GH>
where
    F: BaseFloat,
    B: Mergeable<B, Result = B> + Hittable<F, ()>,
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

impl<B, G, F, GH> Hittable<F, Rc<G>> for BVHNode<F, B, G, GH>
where
    F: BaseFloat,
    B: Mergeable<B, Result = B> + Hittable<F, ()>,
    G: Hittable<F, GH>
{
    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, Rc<G>>> {
        let hit_bv_result = self.hit_bv(ray, min, max);
        // println!("{:?}", hit_bv_result);
        if hit_bv_result.is_none() {
            return None;
        }

        let mut max = max;
        let mut hr: HitRecord<F, Rc<G>> = HitRecord::new();
        hr.t = F::infinity();
        if let Some(n) = &self.right {
            let result = n.borrow().hit(ray, min, max);
            if let Some(r) = result {
                // max = r.t;
                if r.t < hr.t {
                    hr = r.clone();
                }
                // hr = r.clone();
            }
        }
        if let Some(n) = &self.left {
            let result = n.borrow().hit(ray, min, max);
            if let Some(r) = result {
                // max = r.t;
                if r.t < hr.t {
                    hr = r.clone();
                }
                // hr = r.clone();
            }
        }
        for obj in self.objects.iter() {
            let result = obj.hit(ray, min, max);
            if let Some(r) = result {
                max = r.t;
                // if r.t < hr.t {
                //     r.copy_except_hit_object(&mut hr);
                //     hr.hit_object = Some(obj.clone());
                // }
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