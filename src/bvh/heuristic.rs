use crate::bvh::traits::{BVHSplitHeuristic, GeometryListType};
use crate::common::axis::Axis;
use crate::common::types::PointType;

pub struct BVHNaiveHeuristic {
    pub axis: Axis
}

impl<T> BVHSplitHeuristic<T> for BVHNaiveHeuristic {
    fn split(&self, objects: GeometryListType<T>) -> (GeometryListType<T>, GeometryListType<T>) {
        let mut objects2 = objects.iter().collect::<Vec<_>>();

        let axis = self.axis;
        let get_coord = move |x: &PointType| {
            match axis {
                Axis::X => x.x,
                Axis::Y => x.y,
                Axis::Z => x.z,
            }
        };
        objects2.sort_by(|a, b| {
            let coord_a = get_coord(&a.1);
            let coord_b = get_coord(&b.1);
            coord_a.partial_cmp(&coord_b).unwrap()
        });

        let mid = objects.len() / 2;

        let left = &objects2[..mid];
        let right = &objects2[mid..];

        (left.iter().cloned().cloned().collect(), right.iter().cloned().cloned().collect())
    }

    fn next(&self) -> Box<dyn BVHSplitHeuristic<T>> {
        Box::new(Self {
            axis: self.axis.next()
        })
    }
}
