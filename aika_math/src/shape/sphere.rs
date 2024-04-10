use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Matrix4, SquareMatrix, Vector3};
use num_traits::Float;
use crate::*;
use crate::utils::{get_2pi, get_4pi};

#[derive(Debug)]
pub struct Sphere<T> {
    pub center: Vector3<T>,
    pub radius: T,
}

impl<T> Sphere<T> where T: BaseFloat {
    pub fn new(center: Vector3<T>, radius: T) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl<F> Bounded<AABB<F>> for Sphere<F> where F: BaseFloat {
    fn get_bv(&self) -> AABB<F> {
        AABB {
            center: self.center,
            extent: Vector3::new(self.radius, self.radius, self.radius)
        }
    }
}

impl<F> Hittable<F, ()> for Sphere<F> where F: BaseFloat {
    fn hit(&self, ray: &Ray<F>, min: F, max: F) -> Option<HitRecord<F, ()>> {
        let temp = ray.origin - self.center;

        let two = F::from(2.0).unwrap();
        let four = F::from(4.0).unwrap();
        let a = F::one();
        let b = ray.direction.dot(temp) * two;
        let c = temp.dot(temp) - self.radius * self.radius;


        let delta = b * b - four * a * c;
        if delta < F::zero() {
            None
        } else {
            let term2 = delta.sqrt() / (two * a);
            let term1 = b / (two * a);

            let x1 = -term1 - term2;
            let x2 = -term1 + term2;

            let interval_min = x1.max(min);
            let interval_max = x2.min(max);

            if interval_min <= interval_max {
                let t;
                if x1 >= interval_min {
                    t = interval_min;
                } else if x2 <= interval_max {
                    t = interval_max;
                } else {
                    return None;
                }
                let hit_point = ray.origin + ray.direction * t;
                let normal = (hit_point - self.center).normalize();

                return Some(HitRecord {
                    t,
                    normal: Some(normal),
                    back_facing: Some(normal.dot(ray.direction) < F::zero()),
                    hit_object: None,
                })
            }

            None
        }
    }
}

impl<F: BaseFloat> HaveCenter<F> for Sphere<F> {
    fn get_center(&self) -> Vector3<F> {
        self.center
    }
}

impl<F> HaveArea<F> for Sphere<F> where F: BaseFloat {
    fn area(&self) -> F {
        let four_pi = F::from(4.0 * PI).unwrap();
        four_pi * self.radius * self.radius
    }
}

impl<F> SampleShape<F> for Sphere<F> where F: BaseFloat {
    fn sample_shape(&self, r1: F, r2: F) -> Option<SampleShapeResult<F>> {
        let pi_2 = get_2pi::<F>();
        let phi: F = pi_2 * r1;
        let cos_theta = F::one() - F::from(2).unwrap() * r2;
        let sin_theta = (F::one() - cos_theta * cos_theta).sqrt();
        let (sin_phi, cos_phi) = phi.sin_cos();

        let x = sin_theta * cos_phi;
        let y = sin_theta * sin_phi;
        let z = cos_theta;
        let offset = Vector3::new(x, y, z) * self.radius;
        let normal = offset.normalize();
        Some(SampleShapeResult {
            pdf: F::one() / self.area(),
            position: self.center + offset,
            normal,
        })
    }
}

impl<F> PrimitiveTrait<F> for Sphere<F> where F: BaseFloat {}

mod test {
    use cgmath::{InnerSpace, Vector3};
    use num_traits::Float;
    use crate::{Hittable, Ray, Sphere};

    #[test]
    fn test_sphere_hit1() {
        let s = Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0f32);
        let ray = Ray {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
        };
        let h = s.hit(&ray, 0.0, f32::infinity());
        assert!(h.is_some());
    }
}
