use std::any::TypeId;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::{cast, Float};

use crate::*;
use crate::utils::{abs_vector3, cast_f64, difference_of_products, gamma, length_vector3, max_component_index, max_component_value, permute_vector3};

pub struct Triangle<T> {
    pub a: Vector3<T>,
    pub b: Vector3<T>,
    pub c: Vector3<T>,
}

pub struct TriangleIntersectResult<F> {
    pub barycentric_coordinates: Vector3<F>,
}

impl<F> Triangle<F> where F: BaseFloat {
    pub fn get_normal(&self) -> Vector3<F> {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let n = ab.cross(ac).normalize();
        n
    }

    pub fn get_bary_centric_coordinate(&self, point: Vector3<F>) -> (F, F, F) {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let n = self.get_normal();
        let q = n.cross(e2);
        let s = point - self.a;
        let qdote1 = q.dot(e1);
        let qdots = q.dot(s);
        let r = s.cross(e1);
        let rdotn = r.dot(n);

        let u = qdots / qdote1;
        let v = rdotn / qdote1;
        let w = F::one() - u - v;
        (w, u, v)
    }
}

impl<F> Bounded<AABB<F>> for Triangle<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        AABB::from_points(&[self.a, self.b, self.c])
    }
}

// See https://pbr-book.org/4ed/Shapes/Triangle_Meshes
impl<F> Hittable<F, TriangleIntersectResult<F>> for Triangle<F> where F: BaseFloat + 'static {
    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, TriangleIntersectResult<F>>> {
        let n = self.get_normal();

        let e2 = self.c - self.a;
        let e1 = self.b - self.a;
        let zero = F::zero();

        // if degenerate triangle
        {
            let cross = e1.cross(e2);
            let length_sqr = cross.x * cross.x + cross.y * cross.y + cross.z * cross.z;
            if length_sqr == F::zero() {
                return None;
            }
        }

        let p0t = self.a - ray.origin;
        let p1t = self.b - ray.origin;
        let p2t = self.c - ray.origin;

        let kz = max_component_index(abs_vector3(ray.direction));
        let kx = if kz == 2 { 0 } else { kz + 1 };
        let ky = if kx == 2 { 0 } else { kx + 1 };
        let d = permute_vector3(ray.direction, kx, ky, kz);
        let mut p0t = permute_vector3(p0t, kx, ky, kz);
        let mut p1t = permute_vector3(p1t, kx, ky, kz);
        let mut p2t = permute_vector3(p2t, kx, ky, kz);

        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = F::one() / d.z;
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

        let mut e0 = difference_of_products(p1t.x, p2t.y, p1t.y, p2t.x);
        let mut e1 = difference_of_products(p2t.x, p0t.y, p2t.y, p0t.x);
        let mut e2 = difference_of_products(p0t.x, p1t.y, p0t.y, p1t.x);

        // fallback to double precision edge functions
        if TypeId::of::<F>() == TypeId::of::<f32>() && (e0 == F::zero() || e1 == F::zero() || e2 == F::zero()) {
            let p2txp1ty = cast_f64(p2t.x) * cast_f64(p1t.y);
            let p2typ1tx = cast_f64(p2t.y) * cast_f64(p1t.x);
            e0 = F::from(p2typ1tx - p2txp1ty).unwrap();
            let p0txp2ty = cast_f64(p0t.x) * cast_f64(p2t.y);
            let p0typ2tx = cast_f64(p0t.y) * cast_f64(p2t.x);
            e1 = F::from(p0typ2tx - p0txp2ty).unwrap();
            let p1txp0ty = cast_f64(p1t.x) * cast_f64(p0t.y);
            let p1typ0tx = cast_f64(p1t.y) * cast_f64(p0t.x);
            e2 = F::from(p1typ0tx - p1txp0ty).unwrap();
        }

        if (e0 < zero || e1 < zero || e2 < zero) && (e0 > zero || e1 > zero || e2 > zero) {
            return None;
        }
        let det = e0 + e1 + e2;
        if det == zero {
            return None;
        }

        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;
        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < zero && (t_scaled >= min * det || t_scaled < max * det) {
            return None;
        } else if det > zero && (t_scaled <= min * det || t_scaled > max * det) {
            return None;
        }

        let inv_det = F::one() / det;
        let b0 = e0 * inv_det;
        let b1 = e1 * inv_det;
        let b2 = e2 * inv_det;
        let t = t_scaled * inv_det;

        let max_zt = max_component_value(abs_vector3(Vector3::new(p0t.z, p1t.z, p2t.z)));
        let delta_z = gamma::<F>(3) * max_zt;

        let max_xt = max_component_value(abs_vector3(Vector3::new(p0t.x, p1t.x, p2t.x)));
        let max_yt = max_component_value(abs_vector3(Vector3::new(p0t.y, p1t.y, p2t.y)));
        let delta_x = gamma::<F>(5) * (max_xt + max_zt);
        let delta_y = gamma::<F>(5) * (max_yt + max_zt);

        let delta_e = F::from(2).unwrap() * (gamma::<F>(2) * max_xt * max_yt + delta_y * max_xt + delta_x * max_yt);
        let max_e = max_component_value(abs_vector3(Vector3::new(e0, e1, e2)));
        let delta_t = F::from(3).unwrap() * (gamma::<F>(3) * max_e * max_zt + delta_e * max_zt + delta_z * max_e) * inv_det.abs();
        if t <= delta_t {
            return None;
        }

        Some(HitRecord {
            t,
            normal: Some(n),
            back_facing: Some(n.dot(ray.direction) > F::zero()),
            hit_object: Some(TriangleIntersectResult {
                barycentric_coordinates: Vector3::new(b0, b1, b2),
            }),
            uv: None,
        })
    }
}

impl<F> HaveCenter<F> for Triangle<F> where F: BaseFloat {
    fn get_center(&self) -> Vector3<F> {
        let three = F::from(3.0).unwrap();
        (self.a + self.b + self.c) / three
    }
}

impl<F> HaveArea<F> for Triangle<F> where F: BaseFloat {
    fn area(&self) -> F {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let cross = ab.cross(ac);

        F::from(0.5).unwrap() * length_vector3(cross)
    }
}

impl<F> SampleShape<F> for Triangle<F> where F: BaseFloat {
    fn sample_shape(&self, r1: F, r2: F) -> Option<SampleShapeResult<F>> {
        let mut r1 = r1;
        let mut r2 = r2;
        let one = F::one();
        if r1 + r2 >= one {
            r1 = one - r1;
            r2 = one - r2;
        }
        let r3 = one - r1 - r2;
        let point = self.a * r1 + self.b * r2 + self.c * r3;
        let area = self.area();
        Some(SampleShapeResult {
            pdf: one / area,
            position: point,
            normal: self.get_normal(),
        })
    }
}

impl<F> PrimitiveTrait<F> for Triangle<F> where F: BaseFloat {}

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

    #[test]
    fn test_triangle_hit4() {
        let triangle = Triangle {
            a: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            b: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            c: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
        };
        let ray = Ray {
            origin: Vector3 { x: 0.3, y: 0.3, z: -1.0 },
            direction: Vector3 { x: 0.0, y: 0.0, z: 1.0 }.normalize(),
        };

        let hit = triangle.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().t, 1.0);
    }

    #[test]
    fn test_triangle_hit5() {
        let triangle = Triangle {
            a: Vector3 { x: 0.692910015_f32, y: 0.202948838, z: -2.20294881 },
            b: Vector3 { x: 0.649519026, y: 0.265165031, z: -2.26516509 },
            c: Vector3 { x: 0.678524971, y: 0.18861863, z: -2.36539531 },
        };
        let ray = Ray {
            origin: Vector3 { x: -0.636595785_f32, y: -2.13779736, z: -0.960788309 },
            direction: Vector3 { x: 0.437592089, y: 0.773531199, z: -0.458435059 },
        };

        let hit = triangle.hit(&ray, 0.0, f32::infinity());
        assert!(hit.is_some());
        // assert_eq!(hit.unwrap().t, 1.0);
    }
}
