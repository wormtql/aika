use std::rc::Rc;
use cgmath::BaseFloat;
use aika_math::{Axis, HaveCenter};
use crate::bvh::BVHSplitHeuristic;

pub struct DefaultBVHSplitHeuristic {
    next_axis: Axis
}

impl Default for DefaultBVHSplitHeuristic {
    fn default() -> Self {
        Self {
            next_axis: Axis::X
        }
    }
}

impl BVHSplitHeuristic for DefaultBVHSplitHeuristic {

    fn split<F: BaseFloat, G: HaveCenter<FloatType = F>>(&mut self, objects: &[Rc<G>]) -> (Vec<Rc<G>>, Vec<Rc<G>>) {
        let mut objects_with_positions = objects.iter().map(|obj| {
            (obj.clone(), obj.get_center())
        }).collect::<Vec<_>>();
        objects_with_positions.sort_by(|a, b| {
            let va = self.next_axis.extract_value_vec3(a.1);
            let vb = self.next_axis.extract_value_vec3(b.1);

            va.partial_cmp(&vb).unwrap()
        });

        let mid = objects_with_positions.len() / 2;

        let vec1 = objects_with_positions.iter()
            .take(mid)
            .map(|x| &x.0)
            .cloned()
            .collect::<Vec<_>>();
        let vec2 = objects_with_positions.iter()
            .skip(mid)
            .map(|x| &x.0)
            .cloned()
            .collect::<Vec<_>>();
        (vec1, vec2)
    }
}