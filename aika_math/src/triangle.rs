use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::Float;

use crate::{AABB, Bounded, HaveCenter, HitRecord, Hittable, Ray};

pub struct Triangle<T> {
    pub a: Vector3<T>,
    pub b: Vector3<T>,
    pub c: Vector3<T>,
}

impl<T> Triangle<T> where T: BaseFloat {
    pub fn get_normal(&self) -> Vector3<T> {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let n = ab.cross(ac).normalize();
        n
    }
}

impl<F> Bounded<AABB<F>> for Triangle<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        AABB::from_points(&[self.a, self.b, self.c])
    }
}

impl<F> Hittable for Triangle<F> where F: BaseFloat {
    type FloatType = F;

    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F>> {
        let n = self.get_normal();

        let dn = ray.direction.dot(n);
        if dn == F::zero() {
            // ray is orthogonal to triangle normal
            return None;
        }

        let t = (self.a - ray.origin).dot(n) / dn;
        if t < min || t > max {
            return None;
        }
        let p = ray.origin + ray.direction * t;

        let abxbp = (self.b - self.a).cross(p - self.b);
        let bcxcp = (self.c - self.b).cross(p - self.c);
        let caxap = (self.a - self.c).cross(p - self.a);

        // check point is inside the triangle
        let flag1 = abxbp.dot(bcxcp) >= F::zero();
        let flag2 = abxbp.dot(caxap) >= F::zero();
        if flag1 && flag2 {
            Some(HitRecord {
                t,
                normal: Some(n),
                back_facing: Some(n.dot(ray.direction) < F::zero())
            })
        } else {
            None
        }
    }
}

impl<F> HaveCenter for Triangle<F> where F: BaseFloat {
    type FloatType = F;

    fn get_center(&self) -> Vector3<Self::FloatType> {
        let three = F::from(3.0).unwrap();
        (self.a + self.b + self.c) / three
    }
}

#[cfg(test)]
mod test {
    use cgmath::{InnerSpace, Vector3};
    use num_traits::Float;
    use crate::*;

    #[test]
    fn test_triangle_hit1() {
        let triangle = Triangle {
            a: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            b: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            c: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        };
        let ray = Ray {
            origin: Vector3 { x: 0.3, y: 0.3, z: 1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 }.normalize(),
        };

        let hit = triangle.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 1.0);
    }

    #[test]
    fn test_triangle_hit2() {
        let triangle = Triangle {
            a: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            b: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            c: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        };
        let ray = Ray {
            origin: Vector3 { x: 0.3, y: 0.3, z: 1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 }.normalize(),
        };

        let hit = triangle.hit(&ray, 2.0, f32::infinity());
        assert!(hit.is_none());
    }

    #[test]
    fn test_triangle_hit3() {
        let triangle = Triangle {
            a: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            b: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            c: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        };
        let ray = Ray {
            origin: Vector3 { x: -0.3, y: -0.3, z: 1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: -1.0 }.normalize(),
        };

        let hit = triangle.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_none());
    }
}
