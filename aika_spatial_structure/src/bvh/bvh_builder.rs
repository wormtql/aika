use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::{Bounded, BoundingVolume, HaveCenter};
use crate::bvh::{BVHNode, BVHSplitHeuristic, BVHTree};

pub struct BVHBuilder<B, G> {
    pub max_span: usize,

    pub objects: Vec<Rc<G>>,
    _phantom: PhantomData<B>,
}

impl<G, B> BVHBuilder<B, G> {
    pub fn new(max_span: usize) -> Self {
        Self {
            max_span,
            objects: Vec::new(),
            _phantom: PhantomData,
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

impl<B, G, F> BVHBuilder<B, G>
where
    F: BaseFloat,
    B: BoundingVolume<FloatType = F>,
    G: Bounded<B> + HaveCenter<FloatType = F>,
{
    fn build_helper<H>(&self, objects: &[Rc<G>], split_heuristic: &mut H) -> BVHNode<B, G>
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
            }
        }
    }

    pub fn build<H>(&self, split_heuristic: &mut H) -> BVHTree<B, G>
    where
        H: BVHSplitHeuristic,
    {
        let root = self.build_helper(&self.objects, split_heuristic);
        BVHTree {
            root: Rc::new(RefCell::new(root))
        }
    }
}
