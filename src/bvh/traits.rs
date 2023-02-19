use std::rc::Rc;
use cgmath::Vector3;
use smallvec::SmallVec;
use crate::bvh::bvh::BVHNode;
use crate::common::types::{PointType, PrecisionType};
use crate::geometry::hittable::Hittable;
use crate::geometry::traits::{Bounded, Geometry};

// a struct can be contents of a BVH node
pub trait BVHGeometry<HitData>: Hittable<HitData> + Bounded {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType>;
}

pub trait BVHSplit<HitData> {
    // split object into smaller pieces
    fn split(self: Rc<Self>) -> Vec<Rc<dyn BVHGeometry<HitData>>> {
        vec![]
    }
}

pub type GeometryListType<HitData> = Vec<(Rc<dyn BVHGeometry<HitData>>, PointType)>;

pub trait BVHSplitHeuristic<HitData> {
    fn split(&self, objects: GeometryListType<HitData>) -> (GeometryListType<HitData>, GeometryListType<HitData>);

    fn next(&self) -> Box<dyn BVHSplitHeuristic<HitData>>;
}
