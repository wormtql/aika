use cgmath::Vector3;
use num_traits::Float;

use crate::{AABB, HaveAABB, HitRecord, Hittable, Ray};

pub struct Triangle<T> {
    pub a: Vector3<T>,
    pub b: Vector3<T>,
    pub c: Vector3<T>,
}

impl<T> Triangle<T> where T: Float {
    pub fn get_normal<T>(&self) -> Vector3<T> {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let n = ab.cross(ac).normalize();
        n
    }
}

impl<T> HaveAABB<T> for Triangle<T> where T: Float {
    fn get_aabb(&self) -> AABB<T> {
        AABB::from_points(&[self.a, self.b, self.c])
    }
}

impl<T> Hittable<T> for Triangle<T> where T: Float {
    fn hit(&self, ray: &Ray<T>, min: T, max: T) -> Option<HitRecord<T>> {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let n = ab.cross(ac).normalize();

        let dn = ray.direction.dot(n);
        if dn == 0.0 {
            return None;
        }

        let t = (self.a - ray.origin).dot(n) / dn;
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + t * ray.direction;

        let abxbp = (self.b - self.a).cross(p - self.b);
        let bcxcp = (self.c - self.b).cross(p - self.c);
        let caxap = (self.a - self.c).cross(p - self.a);

        let flag1 = abxbp.dot(bcxcp) > 0.0;
        let flag2 = abxbp.dot(caxap) > 0.0;
        if flag1 && flag2 {
            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: (),
            })
        } else {
            None
        }
    }
}

impl Hittable<PathTracingHitRecordData> for Triangle {
    fn hit(self: Rc<Self>, ray: &Ray) -> Option<HitRecord<PathTracingHitRecordData>> {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let n = ab.cross(ac).normalize();

        let dn = ray.direction.dot(n);
        if dn == 0.0 {
            return None;
        }

        let t = (self.a - ray.origin).dot(n) / dn;
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + t * ray.direction;

        let abxbp = (self.b - self.a).cross(p - self.b);
        let bcxcp = (self.c - self.b).cross(p - self.c);
        let caxap = (self.a - self.c).cross(p - self.a);

        let flag1 = abxbp.dot(bcxcp) > 0.0;
        let flag2 = abxbp.dot(caxap) > 0.0;
        if flag1 && flag2 {
            Some(HitRecord {
                t,
                hit_object: self.clone(),
                data: PathTracingHitRecordData {
                    normal: n
                },
            })
        } else {
            None
        }
    }
}

impl BVHGeometry<()> for Triangle {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        (self.a + self.b + self.c) / 3.0
    }
}

impl BVHGeometry<PathTracingHitRecordData> for Triangle {
    fn get_center_heuristic(&self) -> Vector3<PrecisionType> {
        (self.a + self.b + self.c) / 3.0
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use cgmath::{InnerSpace, Vector3};

    use crate::geometry::geometries::triangle::Triangle;
    use crate::geometry::hittable::{HitRecord, Hittable};
    use crate::geometry::ray::Ray;
    use crate::geometry::vertex::Vertex;

    #[test]
    fn test_triangle_hit1() {
        let triangle = Rc::new(Triangle {
            a: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            b: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            c: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        });
        let ray = Ray {
            origin: Vector3 { x: 0.3, y: 0.3, z: 1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 }.normalize(),
        };

        let hit: Option<HitRecord<()>> = triangle.hit(&ray);
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 1.0);
    }
}