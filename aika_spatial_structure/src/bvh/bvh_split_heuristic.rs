use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::HaveCenter;

pub trait BVHSplitHeuristic {
    fn split<F: BaseFloat, G: HaveCenter<F>>(&mut self, objects: &[Rc<G>]) -> (Vec<Rc<G>>, Vec<Rc<G>>);
}
