use std::rc::Rc;
use cgmath::{InnerSpace, Matrix4, Vector3, SquareMatrix};
use crate::bvh::traits::{BVHGeometry, BVHSplit};
use crate::common::types::{PointType, PrecisionType};
use crate::geometry::aabb::AABB;
use crate::geometry::hittable::{PathTracingHitRecordData, HitRecord, Hittable};
use crate::geometry::ray::Ray;
use crate::geometry::traits::{Bounded, Geometry};

pub struct Ball {
    pub center: PointType,
    pub radius: PrecisionType,

    pub transform: Matrix4<PrecisionType>,
}

impl Ball {
    pub fn new(center: PointType, radius: PrecisionType) -> Self {
        Self {
            center,
            radius,
            transform: Matrix4::identity(),
        }
    }
}

impl Bounded for Ball {
    fn bound(&self) -> AABB {
        AABB {
            x_low: self.center.x - self.radius,
            x_high: self.center.x + self.radius,
            y_low: self.center.y - self.radius,
            y_high: self.center.y + self.radius,
            z_low: self.center.z - self.radius,
            z_high: self.center.z + self.radius
        }
    }
}

impl Hittable<PathTracingHitRecordData> for Ball {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<PathTracingHitRecordData>> {
        let temp = ray.origin - self.center;

        let a = 1.0 as PrecisionType;
        let b = ray.direction.dot(temp) * 2.0;
        let c = temp.dot(temp) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else {
            let term2 = delta.sqrt() / (2.0 * a);
            let term1 = b / (2.0 * a);

            let x1 = -term1 - term2;
            let x2 = -term1 + term2;

            let t;
            if x1 < 0.0 && x2 < 0.0 {
                return None;
            } else if x1 < 0.0 && x2 >= 0.0 {
                t = x2;
            } else {
                t = x1;
            }

            let hit_point = ray.origin + t * ray.direction;
            let normal = (hit_point - self.center).normalize();

            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: PathTracingHitRecordData {
                    normal,
                }
            })
        }
    }
}

impl Hittable<()> for Ball {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<()>> {
        let temp = ray.origin - self.center;

        let a = 1.0 as PrecisionType;
        let b = ray.direction.dot(temp) * 2.0;
        let c = temp.dot(temp) - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            None
        } else {
            let term2 = delta.sqrt() / (2.0 * a);
            let term1 = b / (2.0 * a);

            let x1 = -term1 - term2;
            let x2 = -term1 + term2;

            let t;
            if x1 < 0.0 && x2 < 0.0 {
                return None;
            } else if x1 < 0.0 && x2 >= 0.0 {
                t = x2;
            } else {
                t = x1;
            }

            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: ()
            })
        }
    }
}

impl BVHGeometry<()> for Ball {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        self.center
    }
}

impl BVHGeometry<PathTracingHitRecordData> for Ball {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        self.center
    }
}

impl Geometry<()> for Ball {}

impl Geometry<PathTracingHitRecordData> for Ball {}

// impl BVHSplit<()> for Ball {
//     fn split(self: Rc<Self>) -> Vec<Rc<dyn BVHGeometry<()>>> {
//         vec![self.clone()]
//     }
//
//     fn can_split(&self) -> bool {
//         false
//     }
// }
