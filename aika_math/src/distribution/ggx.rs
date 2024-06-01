use std::f64::consts::PI;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use crate::utils::{lerp, sample_uniform_disk_polar};

pub struct IsotropicGGXDistribution<F> {
    pub roughness: F
}

impl<F> IsotropicGGXDistribution<F> where F: BaseFloat {
    pub fn new(roughness: F) -> Self {
        Self {
            roughness
        }
    }

    /// The distribution is symmetric, D(wm) = D(-wm)
    pub fn evaluate(&self, wm: Vector3<F>) -> F {
        let pi = F::from(PI).unwrap();
        let temp = wm.x * wm.x / self.roughness + wm.y * wm.y / self.roughness + wm.z * wm.z * self.roughness;
        let result = F::one() / (pi * temp * temp);
        // println!("{:?}", result * wm.z.abs());
        result


        // let cos_theta_2 = wm.z * wm.z;
        // let sin_theta_2 = F::one() - cos_theta_2;
        // let tan_theta_2 = sin_theta_2 / cos_theta_2;
        // if cos_theta_2 == F::zero() || tan_theta_2.is_infinite() {
        //     return F::zero();
        // }
        //
        // let cos_theta_4 = cos_theta_2 * cos_theta_2;
        // let cos_phi =

        // assert!(dir.z > F::zero());
        // let z = wm.z;
        // let a2 = self.roughness * self.roughness;
        // let pi = F::from(PI).unwrap();
        // let one = F::one();
        //
        // let temp = one + z * z * (a2 - one);
        // // println!("{:?}", temp);
        // a2 / (pi * temp * temp)
    }

    pub fn is_effectively_smooth(&self) -> bool {
        self.roughness < F::from(1e-3).unwrap()
    }

    fn lambda(&self, dir: Vector3<F>) -> F {
        let cos_theta = dir.z;
        let cos_theta_2 = cos_theta * cos_theta;
        if cos_theta_2 == F::zero() {
            return F::zero();
        }
        let sin_theta_2 = F::one() - cos_theta_2;
        let tan_theta_2 = sin_theta_2 / cos_theta_2;
        let a2 = self.roughness * self.roughness;

        let one = F::one();
        let h = F::from(0.5).unwrap();
        ((one + a2 * tan_theta_2).sqrt() - one) * h
    }

    pub fn g1(&self, wo: Vector3<F>, _wm: Vector3<F>) -> F {
        let one = F::one();
        one / (one + self.lambda(wo))
    }

    /// Smith height correlated masking-shadowing
    pub fn masking_shadowing(&self, wi: Vector3<F>, wo: Vector3<F>) -> F {
        let one = F::one();
        one / (one + self.lambda(wi) + self.lambda(wo))
    }

    /// wo can be in any hemisphere, wm will always be positive hemisphere
    pub fn distribution_of_visible_normal(&self, wo: Vector3<F>, wm: Vector3<F>) -> F {
        // assert!(wo.z > F::zero());
        // assert!(wo.z * wm.z >= F::zero());
        let cos_theta_o = wo.z.abs();
        let g1 = self.g1(wo, wm);
        // println!("{:?}", self.evaluate(wm));
        g1 / cos_theta_o * self.evaluate(wm) * (wo.dot(wm).abs())
    }

    /// the sampled wm will always be in the same hemisphere of the geometric normal
    pub fn sample_wm(&self, w: Vector3<F>, r1: F, r2: F) -> Vector3<F> {
        // if w.z <= F::zero() {
        //     println!("sampled wm is below hemisphere {:?}", w);
        // }
        // assert!(w.z > F::zero());
        let one = F::one();
        let two = F::from(2).unwrap();
        let backface = w.z < F::zero();

        let mut wh = Vector3::new(self.roughness * w.x, self.roughness * w.y, w.z).normalize();
        if wh.z < F::zero() {
            wh = -wh;
        }
        let t1 = if wh.z < F::from(0.99999).unwrap() {
            let z = Vector3::new(F::zero(), F::zero(), F::one());
            z.cross(wh).normalize()
        } else {
            Vector3::new(F::one(), F::zero(), F::zero())
        };
        let t2 = wh.cross(t1);

        let mut p = sample_uniform_disk_polar(r1, r2);
        let h = (F::one() - p.x * p.x).sqrt();
        p.y = lerp((one + wh.z) / two, h, p.y);

        let length_2 = p.x * p.x + p.y * p.y;
        let pz = (one - length_2).max(F::zero()).sqrt();
        let nh = t1 * p.x + t2 * p.y + wh * pz;

        let n = Vector3::new(self.roughness * nh.x, self.roughness * nh.y, nh.z.max(F::from(1e-6).unwrap())).normalize();
        // if backface {
        //     n = -n;
        // }
        // println!("{:?}", n);
        n

        // nh.z = nh.z.max(F::from(1e-6).unwrap());
        //
        // if backface {
        //     nh = -nh;
        // }
        // nh.normalize()
        // Vector3::new(nh.x)
    }
}

