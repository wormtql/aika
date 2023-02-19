use std::rc::Rc;
use cgmath::Vector3;
use crate::common::types::{PointType, PrecisionType, TransformMatrixType};
use crate::geometry::hittable::{HitRecord, Hittable};
use crate::geometry::ray::Ray;
use crate::geometry::transform::Transformable;

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub x_low: PrecisionType,
    pub x_high: PrecisionType,
    pub y_low: PrecisionType,
    pub y_high: PrecisionType,
    pub z_low: PrecisionType,
    pub z_high: PrecisionType,
}

impl BoundingBox {
    pub fn zero() -> Self {
        BoundingBox {
            x_low: 0.0,
            x_high: 0.0,
            y_low: 0.0,
            y_high: 0.0,
            z_low: 0.0,
            z_high: 0.0
        }
    }

    pub fn unit() -> Self {
        BoundingBox {
            x_low: -0.5,
            x_high: 0.5,
            y_low: -0.5,
            y_high: 0.5,
            z_low: -0.5,
            z_high: 0.5
        }
    }

    pub fn from_points(points: &[PointType]) -> Self {
        let mut result = BoundingBox::zero();

        for p in points.iter() {
            let x = p.x;
            let y = p.y;
            let z = p.z;

            result.x_low = result.x_low.min(x);
            result.x_high = result.x_high.max(x);
            result.y_low = result.y_low.min(y);
            result.y_high = result.y_high.max(y);
            result.z_low = result.z_low.min(z);
            result.z_high = result.z_high.max(z);
        }

        result
    }

    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            x_low: PrecisionType::min(self.x_low, other.x_low),
            x_high: PrecisionType::max(self.x_high, other.x_high),
            y_low: PrecisionType::min(self.y_low, other.y_low),
            y_high: PrecisionType::max(self.y_high, other.y_high),
            z_low: PrecisionType::min(self.z_low, other.z_low),
            z_high: PrecisionType::max(self.z_high, other.z_high),
        }
    }
}

impl Transformable for BoundingBox {
    fn transform(&self, matrix: &TransformMatrixType) -> Self {
        let p1 = Vector3::new(self.x_low, self.y_low, self.z_low).transform(matrix);
        let p2 = Vector3::new(self.x_low, self.y_low, self.z_high).transform(matrix);
        let p3 = Vector3::new(self.x_low, self.y_high, self.z_low).transform(matrix);
        let p4 = Vector3::new(self.x_low, self.y_high, self.z_high).transform(matrix);
        let p5 = Vector3::new(self.x_high, self.y_low, self.z_low).transform(matrix);
        let p6 = Vector3::new(self.x_high, self.y_low, self.z_high).transform(matrix);
        let p7 = Vector3::new(self.x_high, self.y_high, self.z_low).transform(matrix);
        let p8 = Vector3::new(self.x_high, self.y_high, self.z_high).transform(matrix);

        BoundingBox::from_points(&[
            p1, p2, p3, p4, p5, p6, p7, p8,
        ])
    }
}

impl Hittable<()> for BoundingBox {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<()>> {
        let mut t_x_min: PrecisionType;
        let mut t_x_max: PrecisionType;
        if ray.direction.x == 0.0 {
            if ray.origin.x <= self.x_low || ray.origin.x >= self.x_high {
                return None;
            } else {
                t_x_min = -PrecisionType::MAX;
                t_x_max = PrecisionType::MAX;
            }
        } else {
            t_x_min = (self.x_low - ray.origin.x) / ray.direction.x;
            t_x_max = (self.x_high - ray.origin.x) / ray.direction.x;
            if t_x_min > t_x_max {
                let temp = t_x_min;
                t_x_min = t_x_max;
                t_x_max = temp;
            }
        }

        let mut t_y_min: PrecisionType;
        let mut t_y_max: PrecisionType;
        if ray.direction.y == 0.0 {
            if ray.origin.y <= self.y_low || ray.origin.y >= self.y_high {
                return None;
            } else {
                t_y_min = -PrecisionType::MAX;
                t_y_max = PrecisionType::MAX;
            }
        } else {
            t_y_min = (self.y_low - ray.origin.y) / ray.direction.y;
            t_y_max = (self.y_high - ray.origin.y) / ray.direction.y;
            if t_y_min > t_y_max {
                let temp = t_y_min;
                t_y_min = t_y_max;
                t_y_max = temp;
            }
        }

        let mut t_z_min: PrecisionType;
        let mut t_z_max: PrecisionType;
        if ray.direction.z == 0.0 {
            if ray.origin.z <= self.z_low || ray.origin.z >= self.z_high {
                return None;
            } else {
                t_z_min = -PrecisionType::MAX;
                t_z_max = PrecisionType::MAX;
            }
        } else {
            t_z_min = (self.z_low - ray.origin.z) / ray.direction.z;
            t_z_max = (self.z_high - ray.origin.z) / ray.direction.z;
            if t_z_min > t_z_max {
                let temp = t_z_min;
                t_z_min = t_z_max;
                t_z_max = temp;
            }
        }

        let min = t_x_min.max(t_y_min).max(t_z_min);
        let max = t_x_max.min(t_y_max).min(t_z_max);

        if min < max {
            let mut t = PrecisionType::MAX;
            if t_x_min > 0.0 {
                t = t.min(t_x_min);
            }
            if t_x_max > 0.0 {
                t = t.min(t_x_max);
            }
            if t_y_min > 0.0 {
                t = t.min(t_y_min);
            }
            if t_y_max > 0.0 {
                t = t.min(t_y_max);
            }
            if t_z_min > 0.0 {
                t = t.min(t_z_min);
            }
            if t_z_max > 0.0 {
                t = t.min(t_z_max);
            }

            if t == PrecisionType::MAX {
                return None;
            }

            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: ()
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use cgmath::{InnerSpace, Vector3};
    use crate::geometry::bounding_box::BoundingBox;
    use crate::geometry::hittable::Hittable;
    use crate::geometry::ray::Ray;

    #[test]
    fn test_aabb_hit1() {
        let bb = Rc::new(BoundingBox::unit());
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 }.normalize(),
        };

        let hit = bb.hit(&ray);
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 0.5);
    }

    #[test]
    fn test_aabb_hit2() {
        let bb = Rc::new(BoundingBox::unit());
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 1.0, y: 1.0, z: 1.0 }.normalize(),
        };

        let hit = bb.hit(&ray);
        assert!(hit.is_some());
        assert!((hit.unwrap().t - 0.8660254).abs() < 1e-6);
    }

    #[test]
    fn test_aabb_hit3() {
        let bb = Rc::new(BoundingBox::unit());
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: -10.0, y: -7.0, z: 3.0 }.normalize(),
        };

        let hit = bb.hit(&ray);
        assert!(hit.is_some());
    }

    #[test]
    fn test_aabb_hit4() {
        let bb = Rc::new(BoundingBox::unit());
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: 10.0, y: 7.0, z: -3.0 }.normalize(),
        };

        let hit = bb.hit(&ray);
        assert!(hit.is_none());
    }

    #[test]
    fn test_aabb_hit5() {
        let bb = Rc::new(BoundingBox::unit());
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 }.normalize(),
        };

        let hit = bb.hit(&ray);
        assert!(hit.is_none());
    }
}