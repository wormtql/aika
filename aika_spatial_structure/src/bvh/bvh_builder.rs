use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::*;
use crate::bvh::{BVHNode, BVHSplitHeuristic, BVHTree};

pub struct BVHBuilder<F, B, G, GH> {
    pub max_span: usize,

    pub objects: Vec<Rc<G>>,
    _phantom: PhantomData<B>,
    _float_phantom: PhantomData<F>,
    _geometry_hittable_phantom: PhantomData<GH>,
}

impl<F, G, B, GH> BVHBuilder<F, B, G, GH> {
    pub fn new(max_span: usize) -> Self {
        Self {
            max_span,
            objects: Vec::new(),
            _phantom: PhantomData,
            _float_phantom: PhantomData,
            _geometry_hittable_phantom: PhantomData,
        }
    }

    pub fn add_object(&mut self, obj: Rc<G>) {
        self.objects.push(obj);
    }

    pub fn add_objects(&mut self, obj: &[Rc<G>]) {
        for item in obj.iter() {
            self.objects.push(item.clone());
        }
    }
}

impl<B, G, F, GH> BVHBuilder<F, B, G, GH>
where
    F: BaseFloat,
    B: Mergeable<B, Result = B> + Hittable<F, ()>,
    G: Bounded<B> + HaveCenter<F>,
{
    fn build_helper<H>(&self, objects: &[Rc<G>], split_heuristic: &mut H) -> BVHNode<F, B, G, GH>
    where
        H: BVHSplitHeuristic,
    {
        if objects.len() <= self.max_span {
            let mut bv = objects[0].get_bv();
            for i in objects.iter().skip(1) {
                bv = bv.merge(&i.get_bv());
            }
            BVHNode {
                left: None,
                right: None,
                objects: objects.iter().cloned().collect(),
                bounding_volume: bv,
                _float_phantom: PhantomData,
            }
        } else {
            let (vec1, vec2) = split_heuristic.split(objects);
            let left = self.build_helper(&vec1, split_heuristic);
            let right = self.build_helper(&vec2, split_heuristic);
            let bv = left.bounding_volume.merge(&right.bounding_volume);
            BVHNode {
                left: Some(Rc::new(RefCell::new(left))),
                right: Some(Rc::new(RefCell::new(right))),
                objects: Vec::new(),
                bounding_volume: bv,
                _float_phantom: PhantomData,
            }
        }
    }

    pub fn build<H>(&self, split_heuristic: &mut H) -> BVHTree<F, B, G, GH>
    where
        H: BVHSplitHeuristic,
    {
        let root = self.build_helper(&self.objects, split_heuristic);
        BVHTree {
            root: Rc::new(RefCell::new(root))
        }
    }
}
