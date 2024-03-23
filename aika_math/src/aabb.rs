use std::fmt::Display;
use cgmath::{BaseFloat, Matrix4, Vector3, Vector4};
use num_traits::{Float, Num, One, Zero};

use crate::{HitRecord, Hittable, Ray, Transformable};

#[derive(Clone, Debug, PartialEq)]
pub struct AABB<T> {
    /// half diagonal
    pub extent: Vector3<T>,
    pub center: Vector3<T>,
}

impl<T> AABB<T> where T: Num + Copy {
    pub fn zero() -> Self {
        let z = T::zero();
        AABB {
            extent: Vector3::new(z, z, z),
            center: Vector3::new(z, z, z),
        }
    }
}

impl<T> AABB<T> where T: BaseFloat {
    pub fn unit() -> AABB<T> {
        let half = T::from(0.5).unwrap();
        let zero = T::zero();
        Self {
            extent: Vector3::new(half, half, half),
            center: Vector3::new(zero, zero, zero),
        }
    }

    pub fn from_min_max(min: Vector3<T>, max: Vector3<T>) -> Self {
        let center = (min + max) / T::from(2.0).unwrap();
        let extent = (max - min) / T::from(2.0).unwrap();
        Self {
            center,
            extent
        }
    }

    pub fn min(&self) -> Vector3<T> {
        self.center - self.extent
    }

    pub fn max(&self) -> Vector3<T> {
        self.center + self.extent
    }

    pub fn from_points(points: &[Vector3<T>]) -> Self {
        let mut max = points[0];
        let mut min = points[0];

        for p in points.iter().skip(1) {
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z > max.z {
                max.z = p.z;
            }
            if p.x < min.x {
                min.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.z < min.z {
                min.z = p.z;
            }
        }

        Self::from_min_max(min, max)
    }

    pub fn union(&self, other: &Self) -> Self {
        let max1 = self.max();
        let min1 = self.min();
        let max2 = other.max();
        let min2 = other.min();
        let max = Vector3::new(
            T::max(max1.x, max2.x),
            T::max(max1.y, max2.y),
            T::max(max1.z, max2.z),
        );
        let min = Vector3::new(
            T::min(min1.x, min2.x),
            T::min(min1.y, min2.y),
            T::min(min1.z, min2.z),
        );

        Self::from_min_max(min, max)
    }

    pub fn get_vertices(&self) -> [Vector3<T>; 8] {
        let mut result = [Vector3::zero(); 8];
        let c = self.center;
        let e = self.extent;
        result[0] = Vector3::new(c.x + e.x, c.y + e.y, c.z + e.z);
        result[1] = Vector3::new(c.x + e.x, c.y + e.y, c.z - e.z);
        result[2] = Vector3::new(c.x + e.x, c.y - e.y, c.z + e.z);
        result[3] = Vector3::new(c.x + e.x, c.y - e.y, c.z - e.z);
        result[4] = Vector3::new(c.x - e.x, c.y + e.y, c.z + e.z);
        result[5] = Vector3::new(c.x - e.x, c.y + e.y, c.z - e.z);
        result[6] = Vector3::new(c.x - e.x, c.y - e.y, c.z + e.z);
        result[7] = Vector3::new(c.x - e.x, c.y - e.y, c.z - e.z);

        result
    }
}

impl<T> Transformable<T> for AABB<T> where T: BaseFloat {
    fn transform(&self, matrix: &Matrix4<T>) -> Self {
        let vertices = self.get_vertices();
        let mut transformed_vertices = [Vector3::zero(); 8];
        for (i, v) in vertices.iter().enumerate() {
            let vec4 = Vector4::new(v.x, v.y, v.z, T::one());
            let transformed_vec4 = vec4.transform(matrix);
            transformed_vertices[i] = Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z);
        }

        AABB::from_points(&transformed_vertices)
    }
}

impl<T> Hittable<T> for AABB<T> where T: BaseFloat + Display {
    fn hit(&self, ray: &Ray<T>, min: T, max: T) -> Option<HitRecord<T>> {
        let mut t_x_min: T;
        let mut t_x_max: T;

        let bb_max = self.max();
        let bb_min = self.min();

        if ray.direction.x == T::zero() {
            t_x_min = T::neg_infinity();
            t_x_max = T::infinity();
        } else {
            t_x_min = (bb_min.x - ray.origin.x) / ray.direction.x;
            t_x_max = (bb_max.x - ray.origin.x) / ray.direction.x;
            if t_x_min > t_x_max {
                let temp = t_x_min;
                t_x_min = t_x_max;
                t_x_max = temp;
            }
        }

        let mut t_y_min: T;
        let mut t_y_max: T;
        if ray.direction.y == T::zero() {
            t_y_min = T::neg_infinity();
            t_y_max = T::infinity();
        } else {
            t_y_min = (bb_min.y - ray.origin.y) / ray.direction.y;
            t_y_max = (bb_max.y - ray.origin.y) / ray.direction.y;
            if t_y_min > t_y_max {
                let temp = t_y_min;
                t_y_min = t_y_max;
                t_y_max = temp;
            }
        }

        let mut t_z_min: T;
        let mut t_z_max: T;
        if ray.direction.z == T::zero() {
            t_z_min = T::neg_infinity();
            t_z_max = T::infinity();
        } else {
            t_z_min = (bb_min.z - ray.origin.z) / ray.direction.z;
            t_z_max = (bb_max.z - ray.origin.z) / ray.direction.z;
            if t_z_min > t_z_max {
                let temp = t_z_min;
                t_z_min = t_z_max;
                t_z_max = temp;
            }
        }

        let interval_min = t_x_min.max(t_y_min).max(t_z_min);
        let interval_max = t_x_max.min(t_y_max).min(t_z_max);
        let interval_min2 = interval_min.max(min);
        let interval_max2 = interval_max.min(max);

        if interval_min2 < interval_max2 {
            let t;
            if interval_min >= interval_min2 {
                t = interval_min;
            } else if interval_max <= interval_max2 {
                t = interval_max;
            } else {
                return None;
            }
            Some(HitRecord {
                t,
                normal: None,
                back_facing: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use cgmath::{InnerSpace, Vector3};
    use num_traits::Float;

    use crate::*;

    #[test]
    fn test_aabb_hit1() {
        let bb: AABB<f32> = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, <f32 as Float>::infinity());
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 0.5);
    }

    #[test]
    fn test_aabb_hit2() {
        let bb = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 1.0, y: 1.0, z: 1.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_some());
        assert!((hit.unwrap().t - 0.8660254).abs() < 1e-6);
    }

    #[test]
    fn test_aabb_hit3() {
        let bb = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: -10.0, y: -7.0, z: 3.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_some());
    }

    #[test]
    fn test_aabb_hit4() {
        let bb = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: 10.0, y: 7.0, z: -3.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_none());
    }

    #[test]
    fn test_aabb_hit5() {
        let bb = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_none());
    }

    #[test]
    fn test_aabb_hit6() {
        let bb = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 10.0, y: 7.0, z: -3.0 },
            direction: Vector3 { x: 1.0, y: 0.0, z: 0.0 }.normalize(),
        };

        let hit = bb.hit(&ray, -f32::infinity(), 0.0);
        assert!(hit.is_some());
    }

    #[test]
    fn test_aabb_hit7() {
        let bb: AABB<f32> = AABB::unit();
        let ray = Ray {
            origin: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 }.normalize(),
        };

        let hit = bb.hit(&ray, 0.0, 0.25);
        assert!(hit.is_none());
    }
}
